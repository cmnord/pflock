use std::cell::{Cell, UnsafeCell};
use std::sync::Arc;
use std::thread;

#[cfg(not(c_reference))]
use pflock::PFLock;
#[cfg(c_reference)]
use pflock::PFLock_C as PFLock;

pub struct MockUnsafeCell<T>(UnsafeCell<T>);

unsafe impl<T> Send for MockUnsafeCell<T> {}
unsafe impl<T> Sync for MockUnsafeCell<T> {}

impl<T> MockUnsafeCell<T> {
    pub fn new(inner: T) -> MockUnsafeCell<T> {
        MockUnsafeCell(UnsafeCell::new(inner))
    }

    pub fn borrow(&self) -> &T {
        unsafe { &*self.0.get() }
    }

    pub fn borrow_mut(&self) -> &mut T {
        unsafe { &mut *self.0.get() }
    }
}

pub struct MockCell<T>(Cell<T>);

unsafe impl<T> Send for MockCell<T> {}
unsafe impl<T> Sync for MockCell<T> {}

impl<T: Copy> MockCell<T> {
    pub fn new(inner: T) -> MockCell<T> {
        MockCell(Cell::new(inner))
    }

    pub fn get(&self) -> T {
        self.0.get()
    }

    pub fn set(&self, val: T) {
        self.0.set(val)
    }
}

#[test]
fn simple() {
    let obj = Arc::new(MockUnsafeCell::new(0));
    let lock = Arc::new(PFLock::new());

    let num_threads = 3;
    let num_repeats = 100;

    let mut handles = vec![];

    for _ in 0..num_threads {
        let obj_clone = Arc::clone(&obj);
        let lock_clone = Arc::clone(&lock);

        handles.push(thread::spawn(move || {
            for i in 0..num_repeats {
                if i % 2 == 0 {
                    lock_clone.write_lock();
                    {
                        *obj_clone.borrow_mut() += 1;
                    }
                    lock_clone.write_unlock();
                } else {
                    lock_clone.read_lock();
                    {
                        let _ = *obj_clone.borrow();
                    }
                    lock_clone.read_unlock();
                }
            }
        }));
    }

    for handle in handles {
        let _ = handle.join().unwrap();
    }

    assert_eq!((num_threads * num_repeats) / 2, *obj.borrow());
}

#[test]
fn cell() {
    let obj = Arc::new(MockCell::new(0));
    let lock = Arc::new(PFLock::new());

    let num_threads = 3;
    let num_repeats = 2000;

    let mut handles = vec![];

    for _ in 0..num_threads {
        let obj_clone = Arc::clone(&obj);
        let lock_clone = Arc::clone(&lock);

        handles.push(thread::spawn(move || {
            for i in 0..num_repeats {
                if i % 2 == 0 {
                    lock_clone.write_lock();
                    {
                        obj_clone.set(obj_clone.get() + 1);
                    }
                    lock_clone.write_unlock();
                } else {
                    lock_clone.read_lock();
                    {
                        let _ = obj_clone.get();
                    }
                    lock_clone.read_unlock();
                }
            }
        }));
    }

    for handle in handles {
        let _ = handle.join().unwrap();
    }

    assert_eq!((num_threads * num_repeats) / 2, obj.get());
}
