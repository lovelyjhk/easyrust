/*
 14-3. 반복자 vs clone 성능 비교하기 
Q. 1부터 100,000,000까지의 수를 반복하는 방법
*/
use std::time::{Duration, Instant};

fn iterator_creation() -> Duration {
    let start_time = Instant::now();
    let vec: Vec<_> = (0..100_000_000).map(|i| i as i64 * i as i64).collect();
    let end_time = Instant::now();
    end_time - start_time
}

fn clone_creation() -> Duration {
    let start_time = Instant::now();
    let mut vec = Vec::new();
    for i in 0..100_000_000 {
        vec.push(i as i64 * i as i64);
    }
    let end_time = Instant::now();
    end_time - start_time
}

pub fn var_3_33() {
    let iterator_time = iterator_creation();
    let clone_time = clone_creation();

    println!("Iterator creation elapsed time: {:?}", iterator_time);
    println!("Clone creation elapsed time: {:?}", clone_time);
}


