use std::sync::atomic;
use std::time::Duration;

use async_std::task::{sleep, Builder};
use once_cell::sync::Lazy;

static COUNT: Lazy<atomic::AtomicUsize> = Lazy::new(|| atomic::AtomicUsize::new(0));

pub fn start() {
    Builder::new()
        .name("CountReport".into())
        .spawn(async {
            loop {
                sleep(Duration::from_secs(1)).await;
                let count = COUNT.swap(0, atomic::Ordering::Acquire);
                println!("one second report count:{}", count);
            }
        })
        .unwrap();
}

pub fn add_count() {
    COUNT.fetch_add(1, atomic::Ordering::Acquire);
}
