use std::sync::{Arc, Mutex};
use std::thread;

fn add_until(lock: Arc<Mutex<u64>>, i: u64, name: &str) {
    loop {
        let mut val = lock.lock().unwrap();
        *val += 1;
        if *val > i { break; };
        println!("{}:{}", name, val);
    }
}

fn main() {
    let mu_value = Arc::new(Mutex::new(0));

    let lock0 = mu_value.clone();
    let th0 = thread::spawn(move || {
        add_until(lock0, 5000, "th0");
    });

    let lock1 = mu_value.clone();
    let th1 = thread::spawn(move || {
        add_until(lock1, 5001, "th1");
    });

    th0.join().unwrap();
    th1.join().unwrap();
}
