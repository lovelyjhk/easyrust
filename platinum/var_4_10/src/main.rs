use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::thread;

fn read_server_response(stream: TcpStream) {
    let mut reader = BufReader::new(stream.try_clone().expect("Failed to clone stream"));
    thread::spawn(move || {
        loop {
            let mut msg = String::new();
            match reader.read_line(&mut msg) {
                Ok(_) => {
                    println!("Server: {}", msg);
                }
                Err(_) => {
                    println!("Server disconnected");
                    return;
                }
            }
        }
    });
}

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:8080").expect("Failed to connect to server");

    println!("Connected to server.");

    read_server_response(stream.try_clone().expect("Failed to clone stream"));

    loop {
        let mut msg = String::new();
        std::io::stdin().read_line(&mut msg).expect("Failed to read line");
        stream.write_all(msg.as_bytes()).expect("Failed to send message");
    }
}
