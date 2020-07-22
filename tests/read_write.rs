use pflock::PFLock;
use std::sync::Arc;
use std::thread;
use std::time::Instant;

const NUM_ITERATIONS: usize = 1_000;

#[test]
fn read_write() {
    let lock = Arc::new(PFLock::new());

    let mut threads = vec![];

    let now = Instant::now();
    for i in 0..NUM_ITERATIONS {
        let arc_clone = Arc::clone(&lock);
        threads.push(thread::spawn(move || {
            arc_clone.read_lock();
            println!("r{}", i);
            arc_clone.read_unlock();
        }));
    }

    // wait for all threads to finish
    for t in threads {
        let _ = t.join();
    }
    let elapsed = now.elapsed();
    println!("read elapsed = {:?}", elapsed);

    let mut threads = vec![];

    let now = Instant::now();
    for i in 0..NUM_ITERATIONS {
        let arc_clone = Arc::clone(&lock);
        threads.push(thread::spawn(move || {
            arc_clone.write_lock();
            println!("w{}", i);
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

#[test]
fn writes() {}