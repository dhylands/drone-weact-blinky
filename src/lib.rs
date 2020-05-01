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
use drone_cortex_m::prelude::*;

use drone_core::heap;
use drone_stm32_map::stm32_reg_tokens;

stm32_reg_tokens! {
    /// A set of tokens for all memory-mapped registers.
    pub struct Regs;
}

heap! {
    /// A heap allocator generated from the `Drone.toml`.
    pub struct Heap;

    #[cfg(feature = "heaptrace")] use drone_cortex_m::itm::trace_alloc;
    #[cfg(feature = "heaptrace")] use drone_cortex_m::itm::trace_dealloc;
    #[cfg(feature = "heaptrace")] use drone_cortex_m::itm::trace_grow_in_place;
    #[cfg(feature = "heaptrace")] use drone_cortex_m::itm::trace_shrink_in_place;
}

/// The global allocator.
#[cfg_attr(not(feature = "std"), global_allocator)]
pub static HEAP: Heap = Heap::new();
