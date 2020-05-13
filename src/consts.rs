//! Project constants.

use crate::clock::*;

impl SystemClockConsts for SystemClock {
    const FLASH_LATENCY: u32 = 3;

    /// HSE crystal frequency.
    const HSE_FREQ: u32 = 25_000_000;

    /// HSI crystal frequency.
    const HSI_FREQ: u32 = 16_000_000;

    // VCO-freq = HSE * (PLLN / PLLM) = 192 MHz
    // PLL general clock = VCO-freq / PLLP = 96 MHz
    // USB/SDIO/RG freq = VCO-freq / PLLQ = 48 MHz

    /// PLLM - Division factor for the main PLL (PLL) input clock
    const PLLM: u32 = 25;

    /// PLLN - Main PLL multiplication factor for VCO
    const PLLN: u32 = 192;

    /// PLLP - Main PLL division factor for main system clock
    const PLLP: u32 = PLLP_DIV2;

    /// PLLQ - Main PLL division factor for USB OTG FS, and SDIO clocks
    const PLLQ: u32 = 4;
}
