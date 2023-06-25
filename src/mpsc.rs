use std::sync::mpsc::channel;
use std::thread;

fn main() {
    let (sender, receiver) = channel();
    let sender2 = sender.clone();

    thread::spawn(move ||{
        sender.send(1).unwrap();
    });

    thread::spawn(move || {
        sender2.send(2).unwrap();
    });

    while let Ok(i) = receiver.recv() {
        println!("{}", i);
    }
}
