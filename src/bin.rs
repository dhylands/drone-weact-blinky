#![feature(naked_functions)]
#![no_main]
#![no_std]

use drone_core::{mem, token::Token};
use drone_cortexm::processor;
use weact_blinky::{
    tasks,
    thr::{Handlers, ThrsInit, Vtable},
    Regs,
};

/// The vector table.
#[no_mangle]
pub static VTABLE: Vtable = Vtable::new(Handlers { reset });

/// The entry point.
///
/// # Safety
///
/// This function should not be called by software.
#[no_mangle]
#[naked]
pub unsafe extern "C" fn reset() -> ! {
    mem::bss_init();
    mem::data_init();
    tasks::root(Regs::take(), ThrsInit::take());
    loop {
        processor::wait_for_int();
    }
}
