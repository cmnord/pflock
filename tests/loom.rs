#![cfg(loom)]
use loom::sync::Arc;
use loom::thread;
use pflock::PFLock;
use std::cell::Cell;

pub struct DummyStruct<T>(Cell<T>);

unsafe impl<T> Send for DummyStruct<T> {}
unsafe impl<T> Sync for DummyStruct<T> {}

impl<T: Copy> DummyStruct<T> {
    pub fn new(inner: T) -> DummyStruct<T> {
        DummyStruct(Cell::new(inner))
    }

    pub fn set(&self, val: T) {
        self.0.set(val)
    }

    pub fn get(&self) -> T {
        self.0.get()
    }
}

#[test]
#[should_panic]
fn loom() {
    loom::model(|| {
        let obj = Arc::new(DummyStruct::new(0));
        let lock = Arc::new(PFLock::new());

        let num_threads = 3;
        let num_repeats = 100;

        let ths: Vec<_> = (0..num_threads)
            .map(|_| {
                let obj_clone = Arc::clone(&obj);
                let lock_clone = Arc::clone(&lock);

                thread::spawn(move || {
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
                })
            })
            .collect();

        for th in ths {
            th.join().unwrap();
        }

        assert_eq!((num_threads * num_repeats) / 2, obj.get());
    });
}
