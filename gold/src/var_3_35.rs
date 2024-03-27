/* 15-2. Rust의 스레드(Thread) 모델 */
use std::thread;
use std::time::Duration;

//메인 스레드에서 무언가를 출력하는 동안 다른 것을 출력하는 새로운 스레드 생성하기
pub fn var_3_35_1() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}


//spawn으로 새로운 스레드 생성하고 생성된 스레드가 종료될 때 까지 기다리기.
pub fn var_3_35_2(){
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();

}

//메인 스레드에서 생성된 벡터에 대한 다른 스레드에서의 사용 시도
pub fn var_3_35_3() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move|| {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}


// move 키워드를 사용하여 메인 스레드에서 생성된 벡터를 새로운 스레드로 소유권을 이전합니다.
pub fn var_3_35_4(){
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();

}

// v를 버리는 메인 스레드로부터 v에 대한 참조자를 캡처하려 하는 클로저를 갖는 스레드
pub fn var_3_35_5(){

    let v = vec![1, 2, 3];

    let handle = thread::spawn(move|| {
        println!("Here's a vector: {:?}", v);
    });

    //drop(v); // 오, 이런!

    handle.join().unwrap();
}

pub fn var_3_35_6(){
    // 현재 스레드의 ID를 가져오는 예제
    let current_thread_id = thread::current().id();
    println!("Current thread ID: {:?}", current_thread_id);

    // 현재 실행 중인 스레드에 대한 핸들을 가져오는 예제
    let current_thread = thread::current();
    println!("Current thread: {:?}", current_thread);

    // 스레드 생성을 위한 Builder를 사용하는 예제
    let builder = thread::Builder::new().name("custom-thread".into());
    let handle = builder.spawn(|| {
        println!("Custom thread started!");
    }).unwrap();
    handle.join().unwrap();

    // 스레드를 지정된 시간만큼 잠들게 하는 예제
    println!("Thread sleeping for 2 seconds...");
    thread::sleep(Duration::from_secs(2));
    println!("Thread woke up after sleeping!");

}