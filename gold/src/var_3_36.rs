use std::sync::{Mutex, Arc};
use std::thread;

pub fn var_3_36() {
    // 공유 데이터 생성
    let counter = Arc::new(Mutex::new(0));

    // 여러 스레드 생성
    let mut handles = vec![];
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            // 뮤텍스 락 획득
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    // 모든 스레드 완료 대기
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
