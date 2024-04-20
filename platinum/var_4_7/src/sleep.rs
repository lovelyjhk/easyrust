use std::thread;

pub fn sleep(seconds: u64) {
    let handle = thread::spawn(move || {
         {
            println!("Thread stoping...");
            thread::sleep(std::time::Duration::from_secs(seconds));
            
        }
    });

    // 입력된 초 후에 스레드 종료
    //thread::sleep(std::time::Duration::from_secs(seconds));
    panic!("ddd");
    //handle.join().expect("Thread panicked");
}
