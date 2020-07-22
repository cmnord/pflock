use pflock::PFLock;
use std::sync::Arc;
use std::thread;
use std::time::Instant;

const N: usize = 3;
const X: usize = 300;

#[test]
fn reads() {
    let mut locks = Vec::new();
    for _ in 0..N {
        locks.push(Arc::new(PFLock::new()));
    }

    let mut threads = vec![];

    let now = Instant::now();
    for i in 0..N {
        for _ in 0..X {
            let arc_clone = Arc::clone(&locks[i]);
            threads.push(thread::spawn(move || {
                arc_clone.read_lock();
                arc_clone.read_unlock();
            }));
        }
    }

    // wait for all threads to finish
    for t in threads {
        let _ = t.join();
    }
    let elapsed = now.elapsed();
    println!("read elapsed = {:?}", elapsed);
}

#[test]
fn writes() {
    let mut locks = Vec::new();
    for _ in 0..N {
        locks.push(Arc::new(PFLock::new()));
    }

    let mut threads = vec![];

    let now = Instant::now();
    for i in 0..N {
        for _ in 0..X {
            let arc_clone = Arc::clone(&locks[i]);
            threads.push(thread::spawn(move || {
                arc_clone.write_lock();
                arc_clone.write_unlock();
            }));
        }
    }

    // wait for all threads to finish
    for t in threads {
        let _ = t.join();
    }
    let elapsed = now.elapsed();
    println!("write elapsed = {:?}", elapsed);
}
