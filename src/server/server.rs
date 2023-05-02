use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_client(mut client_stream: TcpStream) {
    let mut buffer = [0; 512];
    loop {
        // 클라이언트로부터의 요청 수신
        let bytes_read = client_stream.read(&mut buffer).unwrap();
        if bytes_read == 0 {
            // 클라이언트 연결 종료
            break;
        }

        // 수신한 데이터 처리 (여기서는 간단히 출력)
        let message = String::from_utf8_lossy(&buffer[..bytes_read]);

        println!("Received message from client: {}", message);

        // 클라이언트로 응답 전송
        let response = format!("Server received message: {}", message);
        client_stream.write(response.as_bytes()).unwrap();
    }
}

pub fn main() {
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
