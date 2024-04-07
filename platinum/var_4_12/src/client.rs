
use std::net::TcpStream;
use std::io::{BufReader, BufRead, Write};
use std::thread;
use std::sync::mpsc;


fn handle_server_response(mut stream: TcpStream, sender: mpsc::Sender<String>) {
    let mut reader = BufReader::new(&stream);
    let mut buffer = String::new();


    loop {
        match reader.read_line(&mut buffer) {
            Ok(0) => {
                println!("서버 연결이 종료되었습니다.");
                return;
            },
            Ok(_) => {
                println!("서버로부터 받은 메시지: {}", buffer.trim());
                // 서버로부터 받은 메시지를 메인 스레드에 전달
                sender.send(buffer.clone()).expect("메시지를 보낼 수 없습니다.");
                buffer.clear();
            },
            Err(_) => {
                println!("서버로부터 메시지를 받는 중 오류가 발생했습니다.");
                return;
            }
        }
    }
}


pub fn client() {
    let mut stream = TcpStream::connect("127.0.0.1:8080").expect("서버에 연결할 수 없습니다.");
    println!("서버에 연결되었습니다.");


    // 메인 스레드와 서버 응답 처리 스레드 간의 통신을 위한 채널 생성
    let (sender, receiver) = mpsc::channel();


    // 서버 응답을 처리할 새로운 스레드 시작
    let cloned_stream = stream.try_clone().expect("스트림 복제 실패");
    thread::spawn(move || {
        handle_server_response(cloned_stream, sender);
    });


    // 메시지를 입력하고 서버에게 전송
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("입력을 읽을 수 없습니다.");
        stream.write_all(input.as_bytes()).expect("메시지를 서버에게 보낼 수 없습니다.");


        // 서버로부터 받은 메시지 처리
        if let Ok(message) = receiver.try_recv() {
            println!("서버 응답: {}", message);
        }
    }
}



