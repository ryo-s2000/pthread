use std::thread;
use std::sync::Arc;
// use std::rc::Rc;

fn main() {
    // let rc1 = Rc::new("rc".to_string());
    let rc1 = Arc::new("arc".to_string());
    let rc2 = rc1.clone();
    thread::spawn(move || {
        println!("{}", rc2);
    });

    println!("main!")
}
