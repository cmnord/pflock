use pflock::PFLock;
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

const NUM_ITERATIONS: usize = 100;

fn read_serial(sleep_duration: Duration) {
    let lock = Arc::new(PFLock::new());

    let now = Instant::now();
    for i in 0..NUM_ITERATIONS {
        let arc_clone = Arc::clone(&lock);
        thread::spawn(move || {
            arc_clone.read_lock();
            thread::sleep(sleep_duration);
            println!("rs{}", i);
            arc_clone.read_unlock();
        })
        .join()
        .unwrap();
    }

    let elapsed = now.elapsed();
    println!("read serial    = {:?}", elapsed);
}

fn read_parallel(sleep_duration: Duration) {
    let lock = Arc::new(PFLock::new());

    let mut threads = vec![];

    let now = Instant::now();
    for i in 0..NUM_ITERATIONS {
        let arc_clone = Arc::clone(&lock);
        threads.push(thread::spawn(move || {
            arc_clone.read_lock();
            thread::sleep(sleep_duration);
            println!("rp{}", i);
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

fn write_serial(sleep_duration: Duration) {
    let lock = Arc::new(PFLock::new());

    let now = Instant::now();
    for i in 0..NUM_ITERATIONS {
        let arc_clone = Arc::clone(&lock);
        thread::spawn(move || {
            arc_clone.write_lock();
            thread::sleep(sleep_duration);
            println!("ws{}", i);
            arc_clone.write_unlock();
        })
        .join()
        .unwrap();
    }

    let elapsed = now.elapsed();
    println!("write serial   = {:?}", elapsed);
}

fn write_parallel(sleep_duration: Duration) {
    let lock = Arc::new(PFLock::new());

    let mut threads = vec![];

    let now = Instant::now();
    for i in 0..NUM_ITERATIONS {
        let arc_clone = Arc::clone(&lock);
        threads.push(thread::spawn(move || {
            arc_clone.write_lock();
            thread::sleep(sleep_duration);
            println!("wp{}", i);
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
    let sleep_duration = Duration::from_millis(100);
    println!(
        "sleep {:?} in each thread, {} iterations",
        sleep_duration, NUM_ITERATIONS
    );
    read_serial(sleep_duration);
    read_parallel(sleep_duration);
    write_serial(sleep_duration);
    write_parallel(sleep_duration);
}
