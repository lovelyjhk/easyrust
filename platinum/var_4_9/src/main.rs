use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

type ClientList = Arc<Mutex<Vec<Arc<Mutex<TcpStream>>>>>;

fn handle_client(mut stream: TcpStream, clients: ClientList) {
    let mut reader = BufReader::new(&stream);
    let mut client_id = String::new();
    match reader.read_line(&mut client_id) {
        Ok(_) => {
            println!("입장했습니다. ID: {}", client_id.trim());
        }
        Err(_) => {
            println!("Error reading client ID");
            return;
        }
    }

    let cloned_stream = stream.try_clone().expect("Failed to clone stream");

    {
        let mut clients = clients.lock().unwrap();
        clients.push(Arc::new(Mutex::new(cloned_stream)));
    }

    loop {
        let mut msg = String::new();
        match reader.read_line(&mut msg) {
            Ok(0) => {
                println!("Client {} disconnected", client_id.trim());
                let mut clients = clients.lock().unwrap();
                clients.retain(|c| {
                    let c = c.lock().unwrap();
                    !c.peer_addr().unwrap().ip().is_unspecified()
                });
                break;
            }
            Ok(_) => {
                if msg.trim().is_empty() {
                    continue;
                }
                println!("Received message from client {} : {}", client_id.trim(), msg);
                // 메시지를 다른 클라이언트들에게 전달
                let clients = clients.lock().unwrap();
                for client in clients.iter() {
                    let mut client = client.lock().unwrap();
                    if let Err(_) = client.write_all(msg.as_bytes()) {
                        //println!("Error sending message to client");
                    }
                }
            }
            Err(_) => {
                //println!("Error reading from socket");
                break;
            }
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind");
    let clients: ClientList = Arc::new(Mutex::new(Vec::new()));

    println!("Server listening on port 8080...");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let clients_clone = clients.clone();
                thread::spawn(move || {
                    handle_client(stream, clients_clone);
                });
            }
            Err(e) => {
                //println!("Error accepting connection: {}", e);
            }
        }
    }
}
