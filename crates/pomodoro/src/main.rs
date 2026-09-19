use std::{
    thread,
    time::{Duration, Instant},
};
use tokio::time::{Duration, interval, sleep, sleep_until};

pub struct time_track {
    // time remaining
    time: u64,
    // percent time elapsed
    percent: u64,
}

// a function to countdown the time
pub async fn countdown(seconds: u64) {
    let started = Instant::now();

    let mut info = time_track {
        time: seconds,
        percent: 0,
    };

    let mut elapsed = started.elapsed().as_secs();
    loop {
        elapsed = started.elapsed().as_secs();

        info.time = seconds.saturating_sub(elapsed);

        info.percent = elapsed.saturating_mul(100).saturating_div(seconds);

        if (info.time <= 0) {
            break;
        }

        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}

pub fn secs_to_min(seconds: u64) -> (u64, u64) {
    let minutes = seconds / 60;
    let secs = seconds % 60;

    (minutes, secs)
}

fn main() {
    println!("Hello, world!");
}
