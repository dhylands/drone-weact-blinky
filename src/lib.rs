#![feature(allocator_api)]
#![feature(const_fn)]
#![feature(prelude_import)]
#![feature(proc_macro_hygiene)]
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod consts;
pub mod tasks;
pub mod thr;

#[prelude_import]
#[allow(unused_imports)]
use drone_core::prelude::*;

use drone_core::heap;
use drone_stm32_map::stm32_reg_tokens;

drone_cortexm::swo::set_log!();

stm32_reg_tokens! {
    /// A set of tokens for all memory-mapped registers.
    pub struct Regs;

    !dwt_cyccnt;
    !itm_tpr; !itm_tcr; !itm_lar;
    !tpiu_acpr; !tpiu_sppr; !tpiu_ffcr;

    !scb_ccr;
    !mpu_type; !mpu_ctrl; !mpu_rnr; !mpu_rbar; !mpu_rasr;
}

heap! {
    /// A heap allocator generated from the `Drone.toml`.
    pub struct Heap;
}

/// The global allocator.
#[cfg_attr(not(feature = "std"), global_allocator)]
pub static HEAP: Heap = Heap::new();
