//! Project constants.

use drone_stm32f4_utils::clock::*;

pub struct WeActSystemClock {}

impl WeActSystemClock {
    pub fn init() -> Self {
        Self {}
    }
}

impl SystemClock for WeActSystemClock {
    fn flash_latency(&self) -> u32 {
        3
    }

    fn hse_freq(&self) -> u32 {
        25_000_000
    }

    /// HSI crystal frequency.
    fn hsi_freq(&self) -> u32 {
        16_000_000
    }

    // VCO-freq = HSE * (PLLN / PLLM)
    // PLL general clock = VCO-freq / PLLP
    // USB/SDIO/RG freq = VCO-freq / PLLQ

    /// PLLM - Division factor for the main PLL (PLL) input clock
    fn pllm(&self) -> u32 {
        25
    }

    /// PLLN - Main PLL multiplication factor for VCO
    fn plln(&self) -> u32 {
        192
    }

    /// PLLP - Main PLL division factor for main system clock
    fn pllp(&self) -> u32 {
        PLLP_DIV2
    }

    /// PLLQ - Main PLL division factor for USB OTG FS, and SDIO clocks
    fn pllq(&self) -> u32 {
        4
    }
}
