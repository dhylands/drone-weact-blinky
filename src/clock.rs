#![cfg_attr(feature = "cargo-clippy", allow(clippy::too_many_arguments))]

use crate::{consts::*, thr};
use drone_cortexm::{fib, reg::prelude::*, thr::prelude::*};
use drone_stm32_map::reg;

pub async fn raise_system_frequency(
    flash_acr: reg::flash::Acr<Srt>,
    pwr_cr: reg::pwr::Cr<Srt>,
    rcc_pllcfgr: reg::rcc::Pllcfgr<Srt>,
    rcc_cfgr: reg::rcc::Cfgr<Srt>,
    rcc_cir: reg::rcc::Cir<Srt>,
    rcc_cr: reg::rcc::Cr<Srt>,
    rcc_apb1enr: reg::rcc::Apb1Enr<Srt>,
    thr_rcc: thr::Rcc,
) {
    // Enable Power Control Clock.
    rcc_apb1enr.pwren.set_bit();

    // Set VOS field in PWR_CR register to Scale 1 (0b11) (HCLK <= 100 MHz)
    pwr_cr.modify(|r| r.write_vos(0b11));

    thr_rcc.enable_int();
    rcc_cir.modify(|r| r.set_hserdyie().set_pllrdyie());

    // We need to move ownership of `hserdyc` and `hserdyf` into the fiber.
    let reg::rcc::Cir {
        hserdyc, hserdyf, ..
    } = rcc_cir;
    // Attach a listener that will notify us when RCC_CIR_HSERDYF is asserted.
    let hserdy = thr_rcc.add_future(fib::new_fn(move || {
        if hserdyf.read_bit() {
            hserdyc.set_bit();
            fib::Complete(())
        } else {
            fib::Yielded(())
        }
    }));

    // Turn on the HSE and wait for it to become ready
    rcc_cr.hseon.set_bit();
    // Sleep until RCC_CIR_HSERDYF is asserted.
    hserdy.await;

    // We need to move ownership of `pllrdyc` and `pllrdyf` into the fiber.
    let reg::rcc::Cir {
        pllrdyc, pllrdyf, ..
    } = rcc_cir;
    // Attach a listener that will notify us when RCC_CIR_PLLRDYF is asserted.
    let pllrdy = thr_rcc.add_future(fib::new_fn(move || {
        if pllrdyf.read_bit() {
            pllrdyc.set_bit();
            fib::Complete(())
        } else {
            fib::Yielded(())
        }
    }));

    rcc_pllcfgr.modify(|r| {
        r.set_pllsrc() // HSE oscillator clock selected as PLL input clock
            .write_pllm(PLLM)
            .write_plln(PLLN)
            .write_pllp(PLLP)
            .write_pllq(PLLQ)
    });
    // Enable the PLL.
    rcc_cr.modify(|r| r.set_pllon());
    // Sleep until RCC_CIR_PLLRDYF is asserted.
    pllrdy.await;

    // The power-on reset latency is set to zero. Since we're going to be increasing
    // the clock speed, increase the latency before increasing the clock speed.
    flash_acr.modify(|r| r.write_latency(FLASH_LATENCY));
    if flash_acr.load().latency() != FLASH_LATENCY {
        panic!("LATENCY");
    }

    rcc_cfgr.modify(|r| {
        r.write_ppre1(0b101) // 0b101 = Divide by 4 - APB1 - 24 MHz (must be < 45 MHz)
            .write_ppre2(0b100) // 0b100 = Divide by 2 - APB2 - 48 MHz (must be < 90 MHz)
            .write_hpre(0b0000) // 0b0000 = Divide by 1 - AHB - 96 MHz
            .write_sw(0b10) // 0b10 = Use PLL for System Clock - 96 MHz
    });

    if rcc_cfgr.load().sws() != 0b10 {
        panic!("SYSCLK not set to HSE");
    }
}
