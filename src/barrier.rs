use std::sync::{Arc, Barrier};
use std::thread;

fn main() {
    let mut v = Vec::new();

    let barrier = Arc::new(Barrier::new(10));

    for id in 0..100 {
        let b = barrier.clone();
        let th = thread::spawn(move || {
            println!("load thread {}", id);
            b.wait();
            println!("finished barrier {}", id);
        });
        v.push(th);
    }

    for th in v {
        th.join().unwrap();
    }
}
