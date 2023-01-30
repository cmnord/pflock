//! This library provides a phase-fair reader-writer lock, as described in the
//! paper ["Reader-Writer Synchronization for Shared-Memory Multiprocessor
//! Real-Time Systems"](https://www.cs.unc.edu/~anderson/papers/ecrts09b.pdf)
//! by Brandenburg et. al.
//!
//! > Reader preference, writer preference, and task-fair reader-writer locks are
//! > shown to cause undue blocking in multiprocessor real-time systems. A new
//! > phase-fair reader-writer lock is proposed as an alternative that
//! > significantly reduces worst-case blocking for readers.
//!
//! # Example
//!
//! ```
//! use pflock::PFLock;
//!
//! let lock = PFLock::new(5);
//!
//! // many reader locks can be held at once
//! {
//!     let r1 = lock.read();
//!     let r2 = lock.read();
//!     assert_eq!(*r1, 5);
//!     assert_eq!(*r2, 5);
//! } // read locks are dropped at this point
//!
//! // only one write lock may be held, however
//! {
//!     let mut w = lock.write();
//!     *w += 1;
//!     assert_eq!(*w, 6);
//! } // write lock is dropped here
//! ```
//!
//! # Spin vs. suspend
//!
//! `PFLock` is a spinlock specifically targeted at **short critical sections** and
//! does not suspend threads while blocking. Section 3 of the paper addresses this:
//!
//! > The terms “short” and “long” arise because (intuitively) spinning is
//! > appropriate only for short critical sections, since spinning wastes processor
//! > time. However, two recent studies have shown that, in terms of
//! > schedulability, spinning is usually preferable to suspending when overheads
//! > are considered [11, 15]. Based on these trends (and due to space
//! > constraints), we restrict our focus to short resources in this paper and
//! > delegate RW synchronization of long resources to future work.

#![no_std]

use core::hint::spin_loop;
use core::sync::atomic::{AtomicUsize, Ordering};
use lock_api::{GuardSend, RawRwLock, RwLock};

pub struct RawPFLock {
    rin: AtomicUsize,
    rout: AtomicUsize,
    win: AtomicUsize,
    wout: AtomicUsize,
}

const RINC: usize = 0x100; // reader increment
const WBITS: usize = 0x3; // writer bits in rin
const PRES: usize = 0x2; // writer present bit
const PHID: usize = 0x1; // phase ID bit

const ZERO_MASK: usize = !255usize;

unsafe impl RawRwLock for RawPFLock {
    const INIT: RawPFLock = RawPFLock {
        rin: AtomicUsize::new(0),
        rout: AtomicUsize::new(0),
        win: AtomicUsize::new(0),
        wout: AtomicUsize::new(0),
    };

    type GuardMarker = GuardSend;

    fn lock_shared(&self) {
        // Increment the rin count and read the writer bits
        let w = self.rin.fetch_add(RINC, Ordering::Relaxed) & WBITS;

        // Spin (wait) if there is a writer present (w != 0), until either PRES
        // and/or PHID flips
        while (w != 0) && (w == (self.rin.load(Ordering::Relaxed) & WBITS)) {
            spin_loop();
        }
    }

    unsafe fn unlock_shared(&self) {
        // Increment rout to mark the read-lock returned
        self.rout.fetch_add(RINC, Ordering::Relaxed);
    }

    fn try_lock_shared(&self) -> bool {
        let w = self.rin.fetch_add(RINC, Ordering::Relaxed) & WBITS;

        if w == 0 || w != (self.rin.load(Ordering::Relaxed) & WBITS) {
            true
        } else {
            self.rout.fetch_add(RINC, Ordering::Relaxed);
            false
        }
    }

    fn lock_exclusive(&self) {
        // Wait until it is my turn to write-lock the resource
        let wticket = self.win.fetch_add(1, Ordering::Relaxed);
        while wticket != self.wout.load(Ordering::Relaxed) {
            spin_loop();
        }

        // Set the write-bits of rin to indicate this writer is here
        let w = PRES | (wticket & PHID);
        let rticket = self.rin.fetch_add(w, Ordering::Relaxed);

        // Wait until all current readers have finished (i.e. rout catches up)
        while rticket != self.rout.load(Ordering::Relaxed) {
            spin_loop();
        }
    }

    unsafe fn unlock_exclusive(&self) {
        // Clear the least-significant byte of rin
        self.rin.fetch_and(ZERO_MASK, Ordering::Relaxed);

        // Increment wout to indicate this write has released the lock
        // Only one writer should ever be here
        self.wout.fetch_add(1, Ordering::Relaxed);
    }

    fn try_lock_exclusive(&self) -> bool {
        let wticket = self.win.fetch_add(1, Ordering::Relaxed);
        if wticket != self.wout.load(Ordering::Relaxed) {
            self.wout.fetch_add(1, Ordering::Relaxed);
            return false;
        }
        let w = PRES | (wticket & PHID);
        let rticket = self.rin.fetch_add(w, Ordering::Relaxed);

        if rticket != self.rout.load(Ordering::Relaxed) {
            self.rin.fetch_and(ZERO_MASK, Ordering::Relaxed);
            self.wout.fetch_add(1, Ordering::Relaxed);
            return false;
        }

        true
    }
}

/// A phase-fair reader-writer lock.
pub type PFLock<T> = RwLock<RawPFLock, T>;
pub type PFLockGuard<'a, T> = lock_api::MutexGuard<'a, RawPFLock, T>;
