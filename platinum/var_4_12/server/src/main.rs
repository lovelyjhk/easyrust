use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // 서버 접속
    let serverip = "127.0.0.1:8081";
    // mpsc 메인 스레드 
    let (tx, rx) = mpsc::channel::<String>();
    // 접속중인 client 변수 
    let mut clients: Vec<TcpStream> = Vec::new();

    // 서버를 바인딩 
    let server = TcpListener::bind(serverip).expect("서버 실행 실패");
    server.set_nonblocking(true).expect("알 수 없는 에러");
    println!("{}에서 서버가 동작중입니다.", serverip);

    // 메인 스레드 loop 무한루프
    loop {
        // 클라이언트 접속 메세지 출력 
        if let Ok((client, addr)) = server.accept() {
            println!("Client 가 접속되었습니다: {}", addr);
            clients.push(client.try_clone().unwrap());
            start_thread(client, tx.clone());
        }
        // 스레드 통신을 대기시켜서, 전원에게 메세지 브로드캐스팅
        if let Ok(msg) = rx.try_recv() {
            println!("메세지 전송(전체) : {}", msg.trim());
            clients = send_all(clients, &msg);
        }
        thread::sleep(Duration::from_millis(100));
    }
}
// 클라이언트가 보내는 메세지를 수신 
fn start_thread(client: TcpStream, tx: mpsc::Sender<String>) {
    let mut reader = BufReader::new(client);
    thread::spawn(move || loop {
        // 메세지 수신을 대기 함.
        let mut msg = String::new();
        if let Ok(n) = reader.read_line(&mut msg) {
            // 수신한 메시지를 메인 스레드 전달중..
            if n > 0 {
                tx.send(msg).unwrap();
            }
        }
        thread::sleep(Duration::from_millis(100));
    });
}
// 모든 클라이언트들에게 메세지를 전송하는 함수 
fn send_all(clients: Vec<TcpStream>, s: &str) -> Vec<TcpStream> {
    let mut collector = vec![];
    for mut socket in clients.into_iter() {
        // 문자열을 바이트열로 변환해 전송한다!
        let bytes = String::from(s).into_bytes();
        if let Err(e) = socket.write_all(&bytes) {
            println!("전송 에러 : {}", e);
            continue;
        }
        collector.push(socket); // 소유권 push
    }
    collector // 소유권 return
}