//! Project constants.

/// FLASH Latency
//
// 3 for Vcc = 3.3v and 90 MHz <= HCLK <= 100 MHz
// Power on default is 0.
pub const FLASH_LATENCY: u32 = 3;

/// HSE crystal frequency.
pub const HSE_FREQ: u32 = 25_000_000;

/// HSI crystal frequency.
pub const HSI_FREQ: u32 = 16_000_000;

// VCO-freq = HSE * (PLLN / PLLM) = 192 MHz
// PLL general clock = VCO-freq / PLLP = 96 MHz
// USB/SDIO/RG freq = VCO-freq / PLLQ = 48 MHz

/// PLLM - Division factor for the main PLL (PLL) input clock
pub const PLLM: u32 = 25;

/// PLLN - Main PLL multiplication factor for VCO
pub const PLLN: u32 = 192;

/// PLLP - Main PLL division factor for main system clock
pub const PLLP: u32 = 0b00; // 0b00 = Divide by 2

/// PLLQ - Main PLL division factor for USB OTG FS, and SDIO clocks
pub const PLLQ: u32 = 0b0100; // 0b0100 - Divide by 4

/// System clock frequency (should be 96 MHz so we have a good USB frequency)
pub const SYS_CLK: u32 = ((HSE_FREQ / PLLM) * PLLN) / ((PLLP + 1) * 2);

/// SysTick clock frequency.
pub const SYS_TICK_FREQ: u32 = SYS_CLK / 8;
