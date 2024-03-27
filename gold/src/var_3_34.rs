use std::time::{Instant};

fn loop_creation() {
    let start_time = Instant::now();
    let mut vec = Vec::new();
    for i in 0..100_000_000 {
        vec.push(i as i64 * i as i64);
    }
    let end_time = Instant::now();
    let elapsed_time = end_time - start_time;
    println!("Loop creation elapsed time: {:?}", elapsed_time);
}

fn iterator_creation() {
    let start_time = Instant::now();
    let vec: Vec<i64> = (0..100_000_000).map(|i| i as i64 * i as i64).collect();
    let end_time = Instant::now();
    let elapsed_time = end_time - start_time;
    println!("Iterator creation elapsed time: {:?}", elapsed_time);
}

pub fn var_3_34() {
    loop_creation();
    iterator_creation();
}


