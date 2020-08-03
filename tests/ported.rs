//! Tests ported from https://github.com/Amanieu/parking_lot/blob/master/src/rwlock.rs

use pflock::PFLock;
use rand::Rng;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::thread;

#[cfg(feature = "serde")]
use bincode::{deserialize, serialize};

#[derive(Eq, PartialEq, Debug)]
struct NonCopy(i32);

#[test]
fn smoke() {
    let l = PFLock::new(());
    drop(l.read());
    drop(l.write());
    drop((l.read(), l.read()));
    drop(l.write());
}

#[test]
fn frob() {
    const N: u32 = 10;
    const M: u32 = 1000;

    let r = Arc::new(PFLock::new(()));

    let (tx, rx) = channel::<()>();
    for _ in 0..N {
        let tx = tx.clone();
        let r = r.clone();
        thread::spawn(move || {
            let mut rng = rand::thread_rng();
            for _ in 0..M {
                if rng.gen_bool(1.0 / N as f64) {
                    drop(r.write());
                } else {
                    drop(r.read());
                }
            }
            drop(tx);
        });
    }
    drop(tx);
    let _ = rx.recv();
}

#[test]
fn test_rw_arc_no_poison_wr() {
    let arc = Arc::new(PFLock::new(1));
    let arc2 = arc.clone();
    let _: Result<(), _> = thread::spawn(move || {
        let _lock = arc2.write();
        panic!();
    })
    .join();
    let lock = arc.read();
    assert_eq!(*lock, 1);
}

#[test]
fn test_rw_arc_no_poison_ww() {
    let arc = Arc::new(PFLock::new(1));
    let arc2 = arc.clone();
    let _: Result<(), _> = thread::spawn(move || {
        let _lock = arc2.write();
        panic!();
    })
    .join();
    let lock = arc.write();
    assert_eq!(*lock, 1);
}

#[test]
fn test_rw_arc_no_poison_rr() {
    let arc = Arc::new(PFLock::new(1));
    let arc2 = arc.clone();
    let _: Result<(), _> = thread::spawn(move || {
        let _lock = arc2.read();
        panic!();
    })
    .join();
    let lock = arc.read();
    assert_eq!(*lock, 1);
}

#[test]
fn test_rw_arc_no_poison_rw() {
    let arc = Arc::new(PFLock::new(1));
    let arc2 = arc.clone();
    let _: Result<(), _> = thread::spawn(move || {
        let _lock = arc2.read();
        panic!()
    })
    .join();
    let lock = arc.write();
    assert_eq!(*lock, 1);
}

#[test]
fn test_rw_arc() {
    let arc = Arc::new(PFLock::new(0));
    let arc2 = arc.clone();
    let (tx, rx) = channel();

    thread::spawn(move || {
        let mut lock = arc2.write();
        for _ in 0..10 {
            let tmp = *lock;
            *lock = -1;
            thread::yield_now();
            *lock = tmp + 1;
        }
        tx.send(()).unwrap();
    });

    // Readers try to catch the writer in the act
    let mut children = Vec::new();
    for _ in 0..5 {
        let arc3 = arc.clone();
        children.push(thread::spawn(move || {
            let lock = arc3.read();
            assert!(*lock >= 0);
        }));
    }

    // Wait for children to pass their asserts
    for r in children {
        assert!(r.join().is_ok());
    }

    // Wait for writer to finish
    rx.recv().unwrap();
    let lock = arc.read();
    assert_eq!(*lock, 10);
}

#[test]
fn test_rw_arc_access_in_unwind() {
    let arc = Arc::new(PFLock::new(1));
    let arc2 = arc.clone();
    let _ = thread::spawn(move || {
        struct Unwinder {
            i: Arc<PFLock<isize>>,
        }
        impl Drop for Unwinder {
            fn drop(&mut self) {
                let mut lock = self.i.write();
                *lock += 1;
            }
        }
        let _u = Unwinder { i: arc2 };
        panic!();
    })
    .join();
    let lock = arc.read();
    assert_eq!(*lock, 2);
}

#[test]
fn test_rwlock_unsized() {
    let rw: &PFLock<[i32]> = &PFLock::new([1, 2, 3]);
    {
        let b = &mut *rw.write();
        b[0] = 4;
        b[2] = 5;
    }
    let comp: &[i32] = &[4, 2, 5];
    assert_eq!(&*rw.read(), comp);
}

#[test]
fn test_rwlock_try_read() {
    let lock = PFLock::new(0isize);
    {
        let read_guard = lock.read();

        let read_result = lock.try_read();
        assert!(
            read_result.is_some(),
            "try_read should succeed while read_guard is in scope"
        );

        drop(read_guard);
    }
    {
        let write_guard = lock.write();

        let read_result = lock.try_read();
        assert!(
            read_result.is_none(),
            "try_read should fail while write_guard is in scope"
        );

        drop(write_guard);
    }
}

#[test]
fn test_rwlock_try_write() {
    let lock = PFLock::new(0isize);
    {
        let read_guard = lock.read();

        let write_result = lock.try_write();
        assert!(
            write_result.is_none(),
            "try_write should fail while read_guard is in scope"
        );

        drop(read_guard);
    }
    {
        let write_guard = lock.write();

        let write_result = lock.try_write();
        assert!(
            write_result.is_none(),
            "try_write should fail while write_guard is in scope"
        );

        drop(write_guard);
    }
}

#[test]
fn test_into_inner() {
    let m = PFLock::new(NonCopy(10));
    assert_eq!(m.into_inner(), NonCopy(10));
}

#[test]
fn test_into_inner_drop() {
    struct Foo(Arc<AtomicUsize>);
    impl Drop for Foo {
        fn drop(&mut self) {
            self.0.fetch_add(1, Ordering::SeqCst);
        }
    }
    let num_drops = Arc::new(AtomicUsize::new(0));
    let m = PFLock::new(Foo(num_drops.clone()));
    assert_eq!(num_drops.load(Ordering::SeqCst), 0);
    {
        let _inner = m.into_inner();
        assert_eq!(num_drops.load(Ordering::SeqCst), 0);
    }
    assert_eq!(num_drops.load(Ordering::SeqCst), 1);
}

#[test]
fn test_get_mut() {
    let mut m = PFLock::new(NonCopy(10));
    *m.get_mut() = NonCopy(20);
    assert_eq!(m.into_inner(), NonCopy(20));
}

#[test]
fn test_rwlockguard_sync() {
    fn sync<T: Sync>(_: T) {}

    let pflock = PFLock::new(());
    sync(pflock.read());
    sync(pflock.write());
}

#[test]
fn test_rwlock_debug() {
    let x = PFLock::new(vec![0u8, 10]);

    assert_eq!(format!("{:?}", x), "RwLock { data: [0, 10] }");
    let _lock = x.write();
    assert_eq!(format!("{:?}", x), "RwLock { data: <locked> }");
}

#[test]
fn test_clone() {
    let pflock = PFLock::new(Arc::new(1));
    let a = pflock.read();
    let b = a.clone();
    assert_eq!(Arc::strong_count(&b), 2);
}

#[cfg(feature = "serde")]
#[test]
fn test_serde() {
    let contents: Vec<u8> = vec![0, 1, 2];
    let mutex = PFLock::new(contents.clone());

    let serialized = serialize(&mutex).unwrap();
    let deserialized: PFLock<Vec<u8>> = deserialize(&serialized).unwrap();

    assert_eq!(*(mutex.read()), *(deserialized.read()));
    assert_eq!(contents, *(deserialized.read()));
}

#[test]
fn test_issue_203() {
    struct Bar(PFLock<()>);

    impl Drop for Bar {
        fn drop(&mut self) {
            let _n = self.0.write();
        }
    }

    thread_local! {
        static B: Bar = Bar(PFLock::new(()));
    }

    thread::spawn(|| {
        B.with(|_| ());

        let a = PFLock::new(());
        let _a = a.read();
    })
    .join()
    .unwrap();
}
