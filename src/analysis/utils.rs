use std::time::{Duration, Instant};


pub fn measure_execution<F>(f: F) -> Duration
where
    F: FnOnce(),
{
    let start = Instant::now();
    f();
    start.elapsed()
}