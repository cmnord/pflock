use std::sync::atomic::{AtomicUsize, Ordering};

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
        w = self.rin.fetch_add(RINC, Ordering::SeqCst) & WBITS;
        while !(w == 0 || w != self.rin.load(Ordering::SeqCst) & WBITS) {}
    }

    pub fn read_unlock(&self) {
        self.rout.fetch_add(RINC, Ordering::SeqCst);
    }

    pub fn write_lock(&self) {
        let mut ticket: usize;
        let w: usize;
        ticket = self.win.fetch_add(1, Ordering::SeqCst);
        while !(ticket == self.wout.load(Ordering::SeqCst)) {}
        w = PRES | (ticket & PHID);
        ticket = self.rin.fetch_add(w, Ordering::SeqCst);
        while !(ticket == self.rout.load(Ordering::SeqCst)) {}
    }

    pub fn write_unlock(&self) {
        // zero the least-significant byte of rin
        let mask = !255usize;
        self.rin.fetch_and(mask, Ordering::SeqCst);
        self.wout.fetch_add(1, Ordering::SeqCst);
    }
}
