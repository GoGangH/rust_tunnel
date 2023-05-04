use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

struct tunnel {
    id: u,
}

fn handle_client(mut client_stream: TcpStream) {
    let mut buffer = [0; 8];
    client_stream.read(&mut buffer).unwrap();
    let id = usize::from_be_bytes(buffer);
    println!("Received id from client: {}", id);
    let response = format!("Server received message: {}", id);
    client_stream.write(response.as_bytes()).unwrap();

    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        if input.trim() == "exit" {
            println!("connect out!");
            break;
        }
    }
}

pub fn main() {
    let mut route = HashMap::new();

    route.insert(String::from("127.0.0.1:8000"), 0);
    route.insert(String::from("127.0.0.1:8001"), 1);
    route.insert(String::from("127.0.0.1:8002"), 2);

    // 서버는 특정 포트에서 클라이언트의 연결을 받음
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("Server listening on port 8080");

    for stream in listener.incoming() {
        // 클라이언트로부터의 연결 수신
        let client_stream = stream.unwrap();
        println!("Client connected");

        // 새로운 스레드에서 클라이언트 요청 처리
        thread::spawn(|| {
            handle_client(client_stream);
        });
    }
}
