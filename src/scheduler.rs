use std::thread;
use std::time::{Duration, Instant};

pub fn run_on_interval(
    function: fn() -> Result<(), String>,
    interval: Duration,
) -> Result<(), String> {
    let mut next_time = Instant::now() + interval;

    loop {
        let now = Instant::now();
        if now < next_time {
            thread::sleep(next_time - now);
        }

        if let Err(error) = function() {
            return Err(error);
        }

        next_time += interval;
    }
}
