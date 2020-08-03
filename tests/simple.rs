use pflock::PFLock;
use std::cell::{Cell, UnsafeCell};
use std::sync::Arc;
use std::thread;

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
    let obj = MockUnsafeCell::new(0);
    let lock = Arc::new(PFLock::new(obj));

    let num_threads = 3;
    let num_repeats = 100;

    let mut handles = vec![];

    for _ in 0..num_threads {
        let lock_clone = Arc::clone(&lock);

        handles.push(thread::spawn(move || {
            for i in 0..num_repeats {
                if i % 2 == 0 {
                    let guard = lock_clone.write();
                    *guard.borrow_mut() += 1;
                } else {
                    let guard = lock_clone.read();
                    let _ = *guard.borrow();
                }
            }
        }));
    }

    for handle in handles {
        let _ = handle.join().unwrap();
    }

    assert_eq!((num_threads * num_repeats) / 2, *lock.read().borrow());
}

#[test]
fn cell() {
    let obj = MockCell::new(0);
    let lock = Arc::new(PFLock::new(obj));

    let num_threads = 3;
    let num_repeats = 2000;

    let mut handles = vec![];

    for _ in 0..num_threads {
        let lock_clone = Arc::clone(&lock);

        handles.push(thread::spawn(move || {
            for i in 0..num_repeats {
                if i % 2 == 0 {
                    let guard = lock_clone.write();
                    guard.set(guard.get() + 1);
                } else {
                    let guard = lock_clone.read();
                    let _ = guard.get();
                }
            }
        }));
    }

    for handle in handles {
        let _ = handle.join().unwrap();
    }

    assert_eq!((num_threads * num_repeats) / 2, lock.read().get());
}
