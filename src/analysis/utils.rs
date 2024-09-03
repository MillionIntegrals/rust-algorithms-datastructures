use std::time::{Duration, Instant};
use rand::Rng;


pub fn measure_execution<F>(f: F) -> Duration
where
    F: FnOnce(),
{
    let start = Instant::now();
    f();
    start.elapsed()
}


pub fn generate_random_vector(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let mut vec = Vec::new();

    for _ in 0 .. n {
        vec.push(rng.gen_range(0..100_000_000));
    }
    
    vec
}