//! The threads.

pub use drone_cortexm::thr::{init, init_extended};
pub use drone_stm32_map::thr::*;

use drone_cortexm::thr;

thr::vtable! {
    use Thr;

    /// The vector table type.
    pub struct Vtable;

    /// Explicit vector table handlers.
    pub struct Handlers;

    /// A set of thread tokens.
    pub struct Thrs;

    /// Threads initialization token.
    pub struct ThrsInit;

    /// The array of thread data.
    static THREADS;

    // --- Allocated threads ---

    /// All classes of faults.
    pub HARD_FAULT;

    /// System tick timer.
    pub SYS_TICK;

    /// RCC global interrupt.
    pub 5: RCC;
}

thr! {
    use THREADS;

    /// The thread data.
    pub struct Thr {}

    /// The thread-local storage.
    pub struct ThrLocal {}
}
