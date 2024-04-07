use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::{BufRead, BufReader, Write};
use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};


#[derive(Debug)]
struct WrappedTcpStream {
    stream: TcpStream,
}


impl PartialEq for WrappedTcpStream {
    fn eq(&self, other: &Self) -> bool {
        self.stream.peer_addr().map(|addr1| addr1 == other.stream.peer_addr().unwrap_or(addr1)).unwrap_or(false)
    }
}


impl Eq for WrappedTcpStream {}


impl std::hash::Hash for WrappedTcpStream {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.stream.peer_addr().map(|addr| addr.hash(state));
    }
}


fn handle_client(stream: TcpStream, clients: Arc<Mutex<Vec<WrappedTcpStream>>>, sender: Sender<String>) {
    let mut reader = BufReader::new(&stream);
    let mut buffer = String::new();


    loop {
        match reader.read_line(&mut buffer) {
            Ok(0) => {
                println!("클라이언트 연결이 종료되었습니다.");
                let mut clients = clients.lock().unwrap();
                if let Some(pos) = clients.iter().position(|client| *client == WrappedTcpStream { stream: stream.try_clone().unwrap() }) {
                    clients.remove(pos);
                }
                return;
            },
            Ok(_) => {
                println!("클라이언트로부터 받은 메시지: {}", buffer.trim());
                buffer.clear();
            },
            Err(_) => {
                println!("클라이언트로부터 메시지를 받는 중 오류가 발생했습니다.");
                return;
            }
        }
    }
}


pub fn server() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind");
    println!("서버가 127.0.0.1:8080 에서 대기중입니다...");


    let clients: Arc<Mutex<Vec<WrappedTcpStream>>> = Arc::new(Mutex::new(Vec::new()));


    let (sender, receiver): (Sender<String>, Receiver<String>) = mpsc::channel();


    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("새로운 연결을 받았습니다.");
                let cloned_clients = clients.clone();
                let cloned_sender = sender.clone();
                let mut clients = clients.lock().unwrap();
                clients.push(WrappedTcpStream { stream: stream.try_clone().expect("Failed to clone stream") });
                drop(clients); // Release the lock before passing to the thread
                let clients = cloned_clients.clone();
                thread::spawn(move || {
                    handle_client(stream, clients, cloned_sender);
                });
            }
            Err(e) => {
                println!("연결 요청을 수락하는 중 오류가 발생했습니다: {:?}", e);
            }
        }
    }


    // Send messages to clients
    for msg in receiver {
        let mut clients = clients.lock().unwrap();
        for client in &mut *clients {
            let _ = client.stream.write_all(msg.as_bytes());
        }
    }
}


