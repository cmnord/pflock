use pflock::PFLock_C;
use std::cell::{Ref, RefCell, RefMut};
use std::sync::Arc;
use std::thread;

#[derive(Debug, PartialEq)]
pub struct DummyStruct<T>(RefCell<T>);

unsafe impl<T> Send for DummyStruct<T> {}
unsafe impl<T> Sync for DummyStruct<T> {}

impl<T> DummyStruct<T> {
    pub fn new(inner: T) -> DummyStruct<T> {
        DummyStruct(RefCell::new(inner))
    }

    pub fn borrow(&self) -> Ref<T> {
        self.0.borrow()
    }

    pub fn borrow_mut(&self) -> RefMut<T> {
        self.0.borrow_mut()
    }
}

#[test]
fn simple() {
    let obj = Arc::new(DummyStruct::new(0));
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
