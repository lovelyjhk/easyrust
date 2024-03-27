use std::sync::{Mutex, Arc};
use std::thread;

//1. 데드락 발생 예제 
pub fn var_3_37_1() {
    let mutex_a = Arc::new(Mutex::new(()));
    let mutex_b = Arc::new(Mutex::new(()));
    let mutex_a_clone = Arc::clone(&mutex_a);
    let mutex_b_clone = Arc::clone(&mutex_b);
    let thread1 = thread::spawn(move || {
        let _lock_a = mutex_a_clone.lock().unwrap();
        thread::sleep(std::time::Duration::from_secs(1));
        let _lock_b = mutex_b_clone.lock().unwrap();
        println!("Thread 1 acquired mutex A and B");
    });
    let thread2 = thread::spawn(move || {
        let _lock_b = mutex_b.lock().unwrap();
        thread::sleep(std::time::Duration::from_secs(1));
        let _lock_a = mutex_a.lock().unwrap();
        println!("Thread 2 acquired mutex B and A");
    });
    thread1.join().unwrap();
    thread2.join().unwrap();
}

//2. 데드락 해결 

pub fn var_3_37_2() {
	let mutex_a = Arc::new(Mutex::new(()));
	let mutex_b = Arc::new(Mutex::new(()));
	let mutex_a_clone = Arc::clone(&mutex_a);
	let mutex_b_clone = Arc::clone(&mutex_b);
	let thread1 = thread::spawn(move || {
    	let _lock_a = mutex_a_clone.lock().unwrap();
    	thread::sleep(std::time::Duration::from_secs(1));
    	let _lock_b = mutex_b_clone.lock().unwrap();
    	println!("Thread 1 acquired mutex A and B");
	});
	let thread2 = thread::spawn(move || {
    	let _lock_a = mutex_a.lock().unwrap();
    	thread::sleep(std::time::Duration::from_secs(1));
    	let _lock_b = mutex_b.lock().unwrap();
    	println!("Thread 2 acquired mutex A and B");
	});
	thread1.join().unwrap();
	thread2.join().unwrap();
}
