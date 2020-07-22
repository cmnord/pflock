use pflock::PFLock;
use std::sync::Arc;
use std::thread;
use std::time::Instant;

const NUM_ITERATIONS: usize = 1_000;

fn read_serial() {
    let lock = Arc::new(PFLock::new());

    let now = Instant::now();
    for i in 0..NUM_ITERATIONS {
        let arc_clone = Arc::clone(&lock);
        arc_clone.read_lock();
        arc_clone.read_unlock();
    }

    let elapsed = now.elapsed();
    println!("read serial    = {:?}", elapsed);
}

fn read_parallel() {
    let lock = Arc::new(PFLock::new());

    let mut threads = vec![];

    let now = Instant::now();
    for i in 0..NUM_ITERATIONS {
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
    println!("read parallel  = {:?}", elapsed);
}

fn write_serial() {
    let lock = Arc::new(PFLock::new());

    let now = Instant::now();
    for i in 0..NUM_ITERATIONS {
        let arc_clone = Arc::clone(&lock);
        arc_clone.write_lock();
        arc_clone.write_unlock();
    }

    let elapsed = now.elapsed();
    println!("write serial   = {:?}", elapsed);
}

fn write_parallel() {
    let lock = Arc::new(PFLock::new());

    let mut threads = vec![];

    let now = Instant::now();
    for i in 0..NUM_ITERATIONS {
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
    println!("write parallel = {:?}", elapsed);
}

#[test]
fn read_write() {
    read_serial();
    read_parallel();
    write_serial();
    write_parallel();
}
