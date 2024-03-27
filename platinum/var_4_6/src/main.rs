use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    // TCP Listener를 포트 8080에서 생성합니다.
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("Server listening on port 8080...");

    // 클라이언트의 요청을 대기하고 수락합니다.
    for stream in listener.incoming() {
        // 클라이언트의 요청을 수락한 후, 해당 스트림을 처리하는 함수를 호출합니다.
        handle_connection(stream.unwrap());
    }
}

// 클라이언트의 요청을 처리하는 함수입니다.
fn handle_connection(mut stream: TcpStream) {
    // 클라이언트로부터 받은 요청을 저장할 버퍼를 생성합니다.
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap(); // 요청을 버퍼에 읽어옵니다.

    // 클라이언트로부터 받은 요청을 출력합니다.
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    // HTTP 응답을 작성합니다.
    let response = "HTTP/1.1 200 OK\r\nContent-Length: 13\r\n\r\nHello, World!";

    // 클라이언트에게 HTTP 응답을 보냅니다.
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap(); // 버퍼를 비워 응답을 전송합니다.
}

