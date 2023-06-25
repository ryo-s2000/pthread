use std::sync::Arc;
use std::sync::atomic::{AtomicIsize, Ordering};
use std::thread;

fn main() {
    let ai = Arc::new(AtomicIsize::new(0));

    let ai1 = ai.clone();
    let th = thread::spawn(move || {
        for _ in 0..100 {
            ai1.fetch_add(1, Ordering::SeqCst);
        }
    });
    th.join().unwrap();

    println!("{}", ai.load(Ordering::SeqCst));
}
