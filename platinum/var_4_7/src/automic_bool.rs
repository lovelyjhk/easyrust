use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::thread;
use std::io::Write; // stdout을 플러시하기 위해 추가

// AtomicBool 및 스레드 간 플래그 사용:
pub fn automic_bool() {
    let running = Arc::new(AtomicBool::new(true));

    let running_clone = Arc::clone(&running);
    let handle = thread::spawn( move|| {
        while running_clone.load(Ordering::Relaxed) {
            println!("Thread working...");
            std::io::stdout().flush().expect("Failed to flush stdout"); // stdout을 즉시 플러시
            thread::sleep(std::time::Duration::from_secs(1));
        }
        println!("Thread stopped.");
    });

    // 일정 시간 후 스레드 종료
    thread::sleep(std::time::Duration::from_secs(1));

    // 스레드 종료를 위해 플래그 설정
    running.store(false, Ordering::Relaxed);

    // 스레드의 실행이 완료될 때까지 대기합니다.
    handle.join().expect("Thread panicked");
}
