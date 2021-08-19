#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![feature(const_fn)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

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
        // Increment the rin count and read the writer bits
        let w = self.rin.fetch_add(RINC, Ordering::Relaxed) & WBITS;

        // Spin (wait) if there is a writer present (w != 0), until either PRES
        // and/or PHID flips
        while (w != 0) && (w == (self.rin.load(Ordering::Relaxed) & WBITS)) {
            spin_loop_hint();
        }
    }

    pub fn read_unlock(&self) {
        // Increment rout to mark the read-lock returned
        self.rout.fetch_add(RINC, Ordering::Relaxed);
    }

    pub fn write_lock(&self) {
        // Wait until it is my turn to write-lock the resource
        let wticket = self.win.fetch_add(1, Ordering::Relaxed);
        while wticket != self.wout.load(Ordering::Relaxed) {
            spin_loop_hint();
        }

        // Set the write-bits of rin to indicate this writer is here
        let w = PRES | (wticket & PHID);
        let rticket = self.rin.fetch_add(w, Ordering::Relaxed);

        // Wait until all current readers have finished (i.e. rout catches up)
        while rticket != self.rout.load(Ordering::Relaxed) {
            spin_loop_hint();
        }
    }

    pub fn write_unlock(&self) {
        // Clear the least-significant byte of rin
        self.rin.fetch_and(ZERO_MASK, Ordering::Relaxed);

        // Increment wout to indicate this write has released the lock
        // Only one writer should ever be here
        self.wout.fetch_add(1, Ordering::Relaxed);
    }
}

pub struct PFLock_C(pft_lock_struct);

impl PFLock_C {
    pub const fn new() -> PFLock_C {
        let mut lock = pft_lock_struct {
            rin: 0,
            _b1 : 0,
            _buf1: [0;15],
            wout: 0,
            _b2 : 0,
            _buf2: [0;15],
            win: 0,
            _b3: 0,
            _buf3: [0;15],
            rout: 0,
            _b4 : 0,
            _buf4: [0;15],
            
        };
        /*unsafe {
            pft_lock_init(&mut lock);
        }*/
        PFLock_C(lock)
    }

    pub fn read_lock(&self) {
        unsafe {
            let const_ptr = self as *const PFLock_C;
            let mut_ptr = const_ptr as *mut PFLock_C;
            pft_read_lock(&mut (*mut_ptr).0);
        }
    }

    pub fn read_unlock(&self) {
        unsafe {
            let const_ptr = self as *const PFLock_C;
            let mut_ptr = const_ptr as *mut PFLock_C;
            pft_read_unlock(&mut (*mut_ptr).0);
        }
    }

    pub fn write_lock(&self) {
        unsafe {
            let const_ptr = self as *const PFLock_C;
            let mut_ptr = const_ptr as *mut PFLock_C;
            pft_write_lock(&mut (*mut_ptr).0);
        }
    }

    pub fn write_unlock(&self) {
        unsafe {
            let const_ptr = self as *const PFLock_C;
            let mut_ptr = const_ptr as *mut PFLock_C;
            pft_write_unlock(&mut (*mut_ptr).0);
        }
    }
}

pub struct PFLockc_C(pftc_lock_struct);

impl PFLockc_C {
    pub fn new() -> PFLockc_C {
        let mut lock = pftc_lock_struct {
            rin: 0,
            wout: 0,
            win: 0,
            rout: 0,
            _buf1: [0;14],

        };
        unsafe {
            pftc_lock_init(&mut lock);
        }
        PFLockc_C(lock)
    }

    pub fn read_lock(&self) {
        unsafe {
            let const_ptr = self as *const PFLockc_C;
            let mut_ptr = const_ptr as *mut PFLockc_C;
            pftc_read_lock(&mut (*mut_ptr).0);
        }
    }

    pub fn read_unlock(&self) {
        unsafe {
            let const_ptr = self as *const PFLockc_C;
            let mut_ptr = const_ptr as *mut PFLockc_C;
            pftc_read_unlock(&mut (*mut_ptr).0);
        }
    }

    pub fn write_lock(&self) {
        unsafe {
            let const_ptr = self as *const PFLockc_C;
            let mut_ptr = const_ptr as *mut PFLockc_C;
            pftc_write_lock(&mut (*mut_ptr).0);
        }
    }

    pub fn write_unlock(&self) {
        unsafe {
            let const_ptr = self as *const PFLockc_C;
            let mut_ptr = const_ptr as *mut PFLockc_C;
            pftc_write_unlock(&mut (*mut_ptr).0);
        }
    }
}

