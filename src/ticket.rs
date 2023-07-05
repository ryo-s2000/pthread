use std::cell::UnsafeCell;
use std::ops::{Deref, DerefMut};
use std::sync::atomic::{fence, AtomicUsize, Ordering};

pub struct TicketLock<T> {
    ticket: AtomicUsize,
    turn: AtomicUsize,
    data: UnsafeCell<T>,
}

impl<T> TicketLock<T> {
    pub fn new(v: T) -> Self {
        TicketLock {
            ticket: AtomicUsize::new(0),
            turn: AtomicUsize::new(0),
            data: UnsafeCell::new(v),
        }
    }

    pub fn lock(&self) -> TicketLockGuard<T> {
        // チケットの獲得にスピンを使わない
        let t = self.ticket.fetch_add(1, Ordering::Relaxed);
        while self.turn.load(Ordering::Relaxed) != t {}
        fence(Ordering::Acquire);

        TicketLockGuard { ticket_lock: self }
    }
}

pub struct TicketLockGuard<'a, T> {
    ticket_lock: & 'a TicketLock<T>,
}

impl<'a, T> Drop for TicketLockGuard<'a, T>  {
    fn drop(&mut self) {
        // ロックを解除してチケットの順番毎に割り当てられる
        self.ticket_lock.turn.fetch_add(1, Ordering::Release);
    }
}

fn main() {
    println!("main!")
}
