use std::thread;

pub fn sleep() {
    let handle = thread::spawn(|| {
        loop {
            println!("Thread working...");
            thread::sleep(std::time::Duration::from_secs(1));
        }
    });

    // 일정 시간 후 스레드 종료
    thread::sleep(std::time::Duration::from_secs(1));
    handle.join().expect("Thread panicked");
   
}
