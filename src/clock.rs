#![cfg_attr(feature = "cargo-clippy", allow(clippy::too_many_arguments))]

use crate::thr;
use drone_cortexm::{fib, reg::prelude::*, thr::prelude::*};
use drone_stm32_map::reg;

pub const PLLP_DIV2: u32 = 0b00;

pub trait SystemClockConsts {
    /// Specifies the number of wait states to set for FLASH_ACR register.
    const FLASH_LATENCY: u32;

    /// HSE crystal frequency.
    const HSE_FREQ: u32;

    /// HSI crystal frequency.
    const HSI_FREQ: u32;

    // VCO-freq = HSE * (PLLN / PLLM)
    // PLL general clock = VCO-freq / PLLP
    // USB/SDIO/RG freq = VCO-freq / PLLQ

    /// PLLM - Division factor for the main PLL (PLL) input clock
    const PLLM: u32;

    /// PLLN - Main PLL multiplication factor for VCO
    const PLLN: u32;

    /// PLLP - Main PLL division factor for main system clock
    const PLLP: u32;

    /// PLLQ - Main PLL division factor for USB OTG FS, and SDIO clocks
    const PLLQ: u32;
}

pub struct SystemClock {
    flash_acr: reg::flash::Acr<Srt>,
    pwr_cr: reg::pwr::Cr<Srt>,
    rcc_pllcfgr: reg::rcc::Pllcfgr<Srt>,
    rcc_cfgr: reg::rcc::Cfgr<Srt>,
    rcc_cr: reg::rcc::Cr<Srt>,
    rcc_cir: reg::rcc::Cir<Srt>,
    rcc_apb1enr: reg::rcc::Apb1Enr<Srt>,
    thr_rcc: thr::Rcc,
}

impl SystemClock {
    pub fn init(
        flash_acr: reg::flash::Acr<Srt>,
        pwr_cr: reg::pwr::Cr<Srt>,
        rcc_pllcfgr: reg::rcc::Pllcfgr<Srt>,
        rcc_cfgr: reg::rcc::Cfgr<Srt>,
        rcc_cr: reg::rcc::Cr<Srt>,
        rcc_cir: reg::rcc::Cir<Srt>,
        rcc_apb1enr: reg::rcc::Apb1Enr<Srt>,
        thr_rcc: thr::Rcc,
    ) -> Self {
        Self {
            flash_acr,
            pwr_cr,
            rcc_pllcfgr,
            rcc_cfgr,
            rcc_cr,
            rcc_cir,
            rcc_apb1enr,
            thr_rcc,
        }
    }

    pub fn clock(&self) -> u32 {
        ((SystemClock::HSE_FREQ / self.rcc_pllcfgr.pllm.read_bits())
            * self.rcc_pllcfgr.plln.read_bits())
            / ((self.rcc_pllcfgr.pllp.read_bits() + 1) * 2)
    }

    pub fn systick_frequency(&self) -> u32 {
        self.clock() / 8
    }

    pub async fn raise_system_frequency(&self) {
        // Enable Power Control Clock.
        self.rcc_apb1enr.pwren.set_bit();

        // Set VOS field in PWR_CR register to Scale 1 (0b11) (HCLK <= 100 MHz)
        self.pwr_cr.modify(|r| r.write_vos(0b11));

        self.thr_rcc.enable_int();
        self.rcc_cir.modify(|r| r.set_hserdyie().set_pllrdyie());

        // We need to move ownership of `hserdyc` and `hserdyf` into the fiber.
        let reg::rcc::Cir {
            hserdyc, hserdyf, ..
        } = self.rcc_cir;
        // Attach a listener that will notify us when RCC_CIR_HSERDYF is asserted.
        let hserdy = self.thr_rcc.add_future(fib::new_fn(move || {
            if hserdyf.read_bit() {
                hserdyc.set_bit();
                fib::Complete(())
            } else {
                fib::Yielded(())
            }
        }));

        // Turn on the HSE and wait for it to become ready
        self.rcc_cr.hseon.set_bit();
        // Sleep until RCC_CIR_HSERDYF is asserted.
        hserdy.await;

        // We need to move ownership of `pllrdyc` and `pllrdyf` into the fiber.
        let reg::rcc::Cir {
            pllrdyc, pllrdyf, ..
        } = self.rcc_cir;
        // Attach a listener that will notify us when RCC_CIR_PLLRDYF is asserted.
        let pllrdy = self.thr_rcc.add_future(fib::new_fn(move || {
            if pllrdyf.read_bit() {
                pllrdyc.set_bit();
                fib::Complete(())
            } else {
                fib::Yielded(())
            }
        }));

        self.rcc_pllcfgr.modify(|r| {
            r.set_pllsrc() // HSE oscillator clock selected as PLL input clock
                .write_pllm(SystemClock::PLLM)
                .write_plln(SystemClock::PLLN)
                .write_pllp(SystemClock::PLLP)
                .write_pllq(SystemClock::PLLQ)
        });
        // Enable the PLL.
        self.rcc_cr.modify(|r| r.set_pllon());
        // Sleep until RCC_CIR_PLLRDYF is asserted.
        pllrdy.await;

        // The power-on reset latency is set to zero. Since we're going to be increasing
        // the clock speed, increase the latency before increasing the clock speed.
        self.flash_acr
            .modify(|r| r.write_latency(SystemClock::FLASH_LATENCY));
        if self.flash_acr.load().latency() != SystemClock::FLASH_LATENCY {
            panic!("LATENCY");
        }

        self.rcc_cfgr.modify(|r| {
            r.write_ppre1(0b101) // 0b101 = Divide by 4 - APB1 - 24 MHz (must be < 45 MHz)
                .write_ppre2(0b100) // 0b100 = Divide by 2 - APB2 - 48 MHz (must be < 90 MHz)
                .write_hpre(0b0000) // 0b0000 = Divide by 1 - AHB - 96 MHz
                .write_sw(0b10) // 0b10 = Use PLL for System Clock - 96 MHz
        });

        if self.rcc_cfgr.load().sws() != 0b10 {
            panic!("SYSCLK not set to HSE");
        }
    }
}
