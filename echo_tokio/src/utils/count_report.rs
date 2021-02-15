use std::sync::atomic;
use std::time::Duration;

use once_cell::sync::Lazy;
use tokio::task;
use tokio::time::sleep;

static COUNT: Lazy<atomic::AtomicUsize> = Lazy::new(|| atomic::AtomicUsize::new(0));

pub fn start() {
    let _ = task::spawn(async {
        loop {
            sleep(Duration::from_secs(1)).await;
            let count = COUNT.swap(0, atomic::Ordering::Acquire);
            println!("one second report count:{}", count);
        }
    });
}

pub fn add_count() {
    COUNT.fetch_add(1, atomic::Ordering::Acquire);
}
