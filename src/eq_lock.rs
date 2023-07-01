use std::cell::UnsafeCell;
use std::ops::{Deref, DerefMut};
use std::sync::atomic::{fence, AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;

pub const NUM_LOCK : usize = 8;
const MASK: usize = NUM_LOCK - 1;

pub struct FairLock<T> {
    waiting: Vec<AtomicBool>,
    lock: AtomicBool,
    turn: AtomicUsize,
    data: UnsafeCell<T>,
}

pub struct FairLockGuard<'a, T> {
    fair_lock: &'a FairLock<T>,
    idx: usize,
}

impl<T> FairLock<T> {
    pub fn new(v: T) -> Self {
        let mut vec = Vec::new();
        for _ in 0..NUM_LOCK {
            vec.push(AtomicBool::new(false));
        }

        FairLock {
            waiting: vec,
            lock: AtomicBool::new(false),
            turn: AtomicUsize::new(0),
            data: UnsafeCell::new(v),
        }
    }

    pub fn lock(&self, idx: usize) -> FairLockGuard<T> {
        assert!(idx < NUM_LOCK);

        self.waiting[idx].store(true, Ordering::Relaxed);
        loop {
            if !self.waiting[idx].load(Ordering::Relaxed) {
                break;
            }

            if !self.lock.load(Ordering::Relaxed) {
                if let Ok(_) = self.lock.compare_exchange_weak(
                    false, true, Ordering::Relaxed, Ordering::Relaxed
                ) {
                    break;
                }
            }
        }
        fence(Ordering::Acquire);

        FairLockGuard { fair_lock: self, idx: idx }
    }
}

impl<'a, T> Drop for FairLockGuard<'a, T> {
    fn drop(&mut self) {
        let fl = self.fair_lock;
        fl.waiting[self.idx].store(false, Ordering::Relaxed);

        let turn = fl.turn.load(Ordering::Relaxed);
        let next = if turn == self.idx {
            (turn + 1) & MASK
        } else {
            turn
        };

        if fl.waiting[next].load(Ordering::Relaxed) {
            fl.turn.store(next, Ordering::Release);
            fl.waiting[next].store(false, Ordering::Release);
        } else {
            fl.turn.store((next + 1) & MASK, Ordering::Release);
            fl.lock.store(false, Ordering::Release);
        }
    }
}

unsafe impl<T> Sync for FairLock<T> {}
unsafe impl<T> Send for FairLock<T> {}

impl<'a, T> Deref for FairLockGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.fair_lock.data.get() }
    }
}

impl<'a, T> DerefMut for FairLockGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.fair_lock.data.get() }
    }
}

use std::time::Instant;
const NUM_LOOP: usize = 5_000_000;

macro_rules! measure {
    ( $x:expr) => {
        {
            let start = Instant::now();
            let result = $x;
            let end = start.elapsed();
            println!("計測開始から{}.{:03}秒経過しました。", end.as_secs(), end.subsec_nanos() / 1_000_000);
            result
        }
    };
}

fn main() {
    measure!({
        exec(1);exec(1);exec(1);exec(1);
        exec(1);exec(1);exec(1);exec(1);
    });

    measure!({
        exec(2);exec(2);
        exec(2);exec(2);
    });

    // ロックを取り合うので性能はむしろ悪くなる
    measure!(
        exec(4)
    )
}

fn exec(num_threads: usize) {
    let lock = Arc::new(FairLock::new(0));
    let mut v = Vec::new();

    for i in 0..num_threads {
        let lock0 = lock.clone();
        let t = std::thread::spawn(move || {
            for _ in 0..NUM_LOOP {
                let mut data = lock0.lock(i);
                *data += 1;
            }
        });
        v.push(t);
    }

    for t in v {
        t.join().unwrap();
    }

    println!(
        "COUNT = {} (expected = {})",
        *lock.lock(0),
        NUM_LOOP * num_threads,
    );
}
