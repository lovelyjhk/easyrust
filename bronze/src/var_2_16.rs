use std::io::{self, Write};
use std::fs::File;
use std::thread;
use std::sync::{Arc, Mutex};

pub fn var_2_16() {
    // 1.std::io: 표준 입력 및 출력과 관련된 함수 및 트레이트
    println!("Enter your name:");
    io::stdout().flush().expect("Failed to flush stdout");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    println!("Hello, {}", input.trim());

    // 2.std::fs: 파일 시스템과 상호작용하는 함수
    let mut file = File::create("output.txt").expect("Failed to create file");
    file.write_all(b"Hello, world!").expect("Failed to write to file");

    // 3.std::thread: 스레드를 생성하고 조작하는 함수
    let handle = thread::spawn(|| {
        println!("Thread started");
    });
    handle.join().expect("Failed to join thread");

    // 4.std::sync: 동기화 원자 연산 및 동기화 기능
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().expect("Failed to join thread");
    }
    println!("Final count: {}", *counter.lock().unwrap());

    // 6.std::str: 문자열 조작과 관련된 함수들을 제공합니다.
    let s1 = "hello";
    let s2 = "world";
    let s3 = format!("{} {}", s1, s2);
    println!("Concatenated string: {}", s3);

    // 7.std::string: 표준 문자열 타입을 제공합니다.
    let mut s = String::from("Hello, ");
    s.push_str("world!");
    println!("String: {}", s);
}
