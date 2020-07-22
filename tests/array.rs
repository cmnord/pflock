use pflock::PFLock;
use std::sync::Arc;
use std::thread;
use std::time::Instant;

const NUM_ITERATIONS: usize = 3_000;

#[test]
fn reads() {
    let lock = Arc::new(PFLock::new());

    let mut threads = vec![];

    let now = Instant::now();
    for _ in 0..NUM_ITERATIONS {
        let arc_clone = Arc::clone(&lock);
        threads.push(thread::spawn(move || {
            arc_clone.read_lock();
            arc_clone.read_unlock();
        }));
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
    let lock = Arc::new(PFLock::new());

    let mut threads = vec![];

    let now = Instant::now();
    for _ in 0..NUM_ITERATIONS {
        let arc_clone = Arc::clone(&lock);
        threads.push(thread::spawn(move || {
            arc_clone.write_lock();
            arc_clone.write_unlock();
        }));
    }

    // wait for all threads to finish
    for t in threads {
        let _ = t.join();
    }
    let elapsed = now.elapsed();
    println!("write elapsed = {:?}", elapsed);
}
