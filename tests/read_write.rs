#![feature(div_duration)]

use pflock::PFLock;
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

const NUM_ITERATIONS: usize = 40;

fn read_serial(sleep_duration: Duration) -> Duration {
    let lock = Arc::new(PFLock::new());

    let now = Instant::now();
    for _ in 0..NUM_ITERATIONS {
        let arc_clone = Arc::clone(&lock);
        thread::spawn(move || {
            arc_clone.read_lock();
            thread::sleep(sleep_duration);
            arc_clone.read_unlock();
        })
        .join()
        .unwrap();
    }

    now.elapsed()
}

fn read_parallel(sleep_duration: Duration) -> Duration {
    let lock = Arc::new(PFLock::new());

    let mut threads = vec![];

    let now = Instant::now();
    for _ in 0..NUM_ITERATIONS {
        let arc_clone = Arc::clone(&lock);
        threads.push(thread::spawn(move || {
            arc_clone.read_lock();
            thread::sleep(sleep_duration);
            arc_clone.read_unlock();
        }));
    }

    // wait for all threads to finish
    for t in threads {
        let _ = t.join();
    }

    now.elapsed()
}

fn write_serial(sleep_duration: Duration) -> Duration {
    let lock = Arc::new(PFLock::new());

    let now = Instant::now();
    for _ in 0..NUM_ITERATIONS {
        let arc_clone = Arc::clone(&lock);
        thread::spawn(move || {
            arc_clone.write_lock();
            thread::sleep(sleep_duration);
            arc_clone.write_unlock();
        })
        .join()
        .unwrap();
    }

    now.elapsed()
}

fn write_parallel(sleep_duration: Duration) -> Duration {
    let lock = Arc::new(PFLock::new());

    let mut threads = vec![];

    let now = Instant::now();
    for _ in 0..NUM_ITERATIONS {
        let arc_clone = Arc::clone(&lock);
        threads.push(thread::spawn(move || {
            arc_clone.write_lock();
            thread::sleep(sleep_duration);
            arc_clone.write_unlock();
        }));
    }

    // wait for all threads to finish
    for t in threads {
        let _ = t.join();
    }

    now.elapsed()
}

#[test]
fn read_write() {
    let sleep_duration = Duration::from_millis(50);
    println!(
        "sleep {:?} in each thread, {} iterations",
        sleep_duration, NUM_ITERATIONS
    );

    let rs = read_serial(sleep_duration);
    println!("read serial    = {:?}", rs);

    let rp = read_parallel(sleep_duration);
    println!("read parallel  = {:?}", rp);

    println!(
        "read parallelism: -{:?} change, {:.1}x faster",
        rs - rp,
        rs.div_duration_f64(rp)
    );

    let ws = write_serial(sleep_duration);
    println!("write serial   = {:?}", ws);

    let wp = write_parallel(sleep_duration);
    println!("write parallel = {:?}", wp);

    println!(
        "write parallelism: +{:?} change, {:.3}% as fast",
        wp - ws,
        ws.div_duration_f64(wp)
    );
}
