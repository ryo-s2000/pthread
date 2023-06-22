use std::sync::{Arc, Mutex, Condvar};
use std::{thread, time};

fn child(id: u64, p: Arc<(Mutex<bool>, Condvar)>) {
    let &(ref lock, ref cvar) = &*p;

    let mut started = lock.lock().unwrap();
    while !*started {
        println!("child while !*started {} at once even if sleep 1 sec", id);
        started = cvar.wait(started).unwrap();
    }

    println!("child {}", id);
}

fn parenmt(p: Arc<(Mutex<bool>, Condvar)>) {
    let &(ref lock, ref cvar) = &*p;

    let mut started = lock.lock().unwrap();
    *started = true;
    cvar.notify_all();
    println!("parent!");
}

fn main() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));

    let pair0 = pair.clone();
    let pair1 = pair.clone();
    let pair2 = pair.clone();

    let c0 = thread::spawn(move || { child(0, pair0) });
    let c1 = thread::spawn(move || { child(1, pair1) });
    let c2 = thread::spawn(move || { child(2, pair.clone()) });

    let ten_millis = time::Duration::from_millis(1000);
    thread::sleep(ten_millis);

    let p = thread::spawn(move || { parenmt(pair2) });

    c0.join().unwrap();
    c1.join().unwrap();
    c2.join().unwrap();
    p.join().unwrap();
}
