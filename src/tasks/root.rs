//! The root task.

use crate::{clock::WeActSystemClock, thr, thr::ThrsInit, Regs};
use drone_cortexm::{fib, reg::prelude::*, thr::prelude::*};
use drone_cortexm::{fib::ThrFiberFuture, thr::ThrNvic};
use drone_stm32_map::periph::{
    gpio::periph_gpio_c13,
    sys_tick::{periph_sys_tick, SysTickPeriph},
};
use drone_stm32f4_utils::{clock::SystemClockRegs, gpioled::GpioLedActiveLow, led::Led};
use futures::prelude::*;

/// An error returned when a receiver has missed too many ticks.
#[derive(Debug)]
pub struct TickOverflow;

/// The root task handler.
#[inline(never)]
pub fn handler(reg: Regs, thr_init: ThrsInit) {
    let thr = thr::init(thr_init);
    let sys_tick = periph_sys_tick!(reg);
    let sysclk = WeActSystemClock::init();

    thr.hard_fault.add_once(|| panic!("Hard Fault"));

    let sysclk_regs = SystemClockRegs::init(
        reg.flash_acr,
        reg.pwr_cr,
        reg.rcc_cr,
        reg.rcc_pllcfgr,
        reg.rcc_cfgr,
        reg.rcc_cir.into_copy(),
        reg.rcc_apb1enr,
        thr.rcc,
        &sysclk,
    );

    sysclk_regs.raise_system_frequency().root_wait();

    // Enable power for GPIOC
    reg.rcc_ahb1enr.gpiocen.set_bit();

    let led = GpioLedActiveLow::init(periph_gpio_c13!(reg));

    beacon(&led, &sysclk_regs, sys_tick, thr.sys_tick)
        .root_wait()
        .expect("beacon fail");

    // Enter a sleep state on ISR exit.
    reg.scb_scr.sleeponexit.set_bit();
}

async fn beacon<'a, THR: ThrNvic + ThrFiberFuture>(
    led: &dyn Led,
    sysclk_regs: &'a SystemClockRegs<'a, THR>,
    sys_tick: SysTickPeriph,
    thr_sys_tick: thr::SysTick,
) -> Result<(), TickOverflow> {
    // Attach a listener that will notify us on each interrupt trigger.
    let mut tick_stream = thr_sys_tick.add_stream_pulse(
        // This closure will be called when a receiver no longer can store the
        // number of ticks since the last stream poll. If this happens, a
        // `TickOverflow` error will be sent over the stream as is final value.
        || Err(TickOverflow),
        // A fiber that will be called on each interrupt trigger. It sends a
        // single tick over the stream.
        fib::new_fn(|| fib::Yielded(Some(1))),
    );
    // Clear the current value of the timer.
    sys_tick.stk_val.store(|r| r.write_current(0));
    // Set the value to load into the `stk_val` register when the counter
    // reaches 0. We set it to the count of SysTick clocks per second divided by
    // 8, so the reload will be triggered each 125 ms.
    sys_tick
        .stk_load
        .store(|r| r.write_reload(sysclk_regs.systick_frequency() / 8));
    sys_tick.stk_ctrl.store(|r| {
        r.set_tickint() // Counting down to 0 triggers the SysTick interrupt
            .set_enable() // Start the counter in a multi-shot way
    });

    // A value cycling from 0 to 7. Full cycle represents a full second.
    let mut counter: u32 = 0;
    while let Some(tick) = tick_stream.next().await {
        for _ in 0..tick?.get() {
            // Each full second print a message.
            if counter == 0 {
                println!("sec");
            }
            match counter {
                // On 0's and 250's millisecond pull the pin low.
                0 | 2 => {
                    led.on();
                }
                // On 125's, 375's, 500's, 625's, 750's, and 875's millisecond
                // pull the pin high.
                _ => {
                    led.off();
                }
            }
            counter = (counter + 1) % 8;
        }
    }

    Ok(())
}
