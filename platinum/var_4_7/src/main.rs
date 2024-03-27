use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

mod join_handle;
mod channel;
mod sleep;
mod automic_bool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8081").expect("Failed to bind");
    println!("Server listening on port 8081...");

    let mut handles = vec![];

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let handle = thread::spawn(move || {
                    handle_connection(stream);
                });
                handles.push(handle);
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }

    // 모든 스레드가 종료될 때까지 기다림
    for handle in handles {
        handle.join().expect("Failed to join thread");
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    match stream.read(&mut buffer) {
        Ok(_) => {
            let request = String::from_utf8_lossy(&buffer[..]);
            println!("Received request: {}", request);
            let response = "HTTP/1.1 200 OK\r\n\r\nHello, World!";
            if let Err(e) = stream.write_all(response.as_bytes()) {
                eprintln!("Error writing response: {}", e);
            }
        }
        Err(e) => {
            eprintln!("Error reading request: {}", e);
        }
    }

    //스레드 종료
    println!("Thread 종료중...");
    //thread::sleep(Duration::from_secs(20));
   // join_handle::join_handle();
   // automic_bool::automic_bool();
   channel::channel();
}
