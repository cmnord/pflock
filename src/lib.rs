use std::sync::atomic::{spin_loop_hint, AtomicUsize, Ordering};

pub struct PFLock {
    pub rin: AtomicUsize,
    pub rout: AtomicUsize,
    pub win: AtomicUsize,
    pub wout: AtomicUsize,
}

const RINC: usize = 0x100; // reader increment
const WBITS: usize = 0x3; // writer bits in rin
const PRES: usize = 0x2; // writer present bit
const PHID: usize = 0x1; // phase ID bit

const ZERO_MASK: usize = !255usize;

impl PFLock {
    pub const fn new() -> PFLock {
        PFLock {
            rin: AtomicUsize::new(0),
            rout: AtomicUsize::new(0),
            win: AtomicUsize::new(0),
            wout: AtomicUsize::new(0),
        }
    }

    pub fn read_lock(&self) {
        let w: usize;

        // Increment the rin count and read the writer bits
        w = self.rin.fetch_add(RINC, Ordering::SeqCst) & WBITS;

        // Spin (wait) if there is a writer present (w != 0), until either PRES
        // and/or PHID flips
        while !(w == 0 || w != (self.rin.load(Ordering::SeqCst) & WBITS)) {
            spin_loop_hint();
        }
    }

    pub fn read_unlock(&self) {
        // Increment rout to mark the read-lock returned
        self.rout.fetch_add(RINC, Ordering::SeqCst);
    }

    pub fn write_lock(&self) {
        let mut ticket: usize;
        let w: usize;

        // Wait until it is my turn to write-lock the resource
        ticket = self.win.fetch_add(1, Ordering::SeqCst);
        while !(ticket == self.wout.load(Ordering::SeqCst)) {
            spin_loop_hint();
        }

        // Set the write-bits of rin to indicate this writer is here
        w = PRES | (ticket & PHID);
        ticket = self.rin.fetch_add(w, Ordering::SeqCst);

        // Wait until all current readers have finished (i.e. rout catches up)
        while !(ticket == self.rout.load(Ordering::SeqCst)) {
            spin_loop_hint();
        }
    }

    pub fn write_unlock(&self) {
        // Clear the least-significant byte of rin
        self.rin.fetch_and(ZERO_MASK, Ordering::SeqCst);

        // Increment wout to indicate this write has released the lock
        // Only one writer should ever be here
        self.wout.fetch_add(1, Ordering::SeqCst);
    }
}
