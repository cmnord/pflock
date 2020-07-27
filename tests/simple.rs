use pflock::PFLock_C;
use std::cell::{Cell, Ref, RefCell, RefMut};
use std::sync::Arc;
use std::thread;

pub struct DummyRefCell<T>(RefCell<T>);

unsafe impl<T> Send for DummyRefCell<T> {}
unsafe impl<T> Sync for DummyRefCell<T> {}

impl<T> DummyRefCell<T> {
    pub fn new(inner: T) -> DummyRefCell<T> {
        DummyRefCell(RefCell::new(inner))
    }

    pub fn borrow(&self) -> Ref<T> {
        self.0.borrow()
    }

    pub fn borrow_mut(&self) -> RefMut<T> {
        self.0.borrow_mut()
    }
}

pub struct DummyCell<T>(Cell<T>);

unsafe impl<T> Send for DummyCell<T> {}
unsafe impl<T> Sync for DummyCell<T> {}

impl<T: Copy> DummyCell<T> {
    pub fn new(inner: T) -> DummyCell<T> {
        DummyCell(Cell::new(inner))
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
    let obj = Arc::new(DummyRefCell::new(0));
    let lock = Arc::new(PFLock_C::new());

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
    let obj = Arc::new(DummyCell::new(0));
    let lock = Arc::new(PFLock_C::new());

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
