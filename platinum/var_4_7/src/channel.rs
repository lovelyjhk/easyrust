//채널(Channel)을 사용한 스레드 간 통신:

use std::sync::mpsc;
use std::thread;

pub fn channel() {
    let (tx, rx) = mpsc::channel();

    let handle = thread::spawn(move || {
        match rx.recv() {
            Ok(_) => println!("Thread received stop signal and will terminate."),
            Err(_) => println!("Thread error while receiving message."),
        }
    });

    // 일정 시간 후 스레드 종료를 위해 메시지 전송
    thread::sleep(std::time::Duration::from_secs(1));
    tx.send(()).expect("Failed to send stop signal to thread");

    // 스레드의 실행이 완료될 때까지 대기합니다.
    handle.join().expect("Thread panicked");
    
}
