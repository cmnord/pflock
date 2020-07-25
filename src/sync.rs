#[cfg(loom)]
pub(crate) use loom::sync::atomic::{spin_loop_hint, AtomicUsize, Ordering};

#[cfg(not(loom))]
pub(crate) use std::sync::atomic::{spin_loop_hint, AtomicUsize, Ordering};
