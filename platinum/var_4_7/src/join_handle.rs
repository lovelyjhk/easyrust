//var_4_8
//JoinHandle 사용하여 스레드 대기 및 종료
use std::thread;

pub fn join_handle() {
    let handle = thread::spawn(|| {
        // 스레드 작업 수행
        for i in 1..=5 {
            println!("Thread working: {}", i);
            thread::sleep(std::time::Duration::from_secs(1));
        }
    });

    // 스레드의 실행이 완료될 때까지 대기합니다.
    handle.join().expect("Thread panicked");

    println!("Thread finished working.");
  
}
