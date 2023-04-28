use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

pub fn main() {
    // 서버의 IP 주소와 포트 번호
    let server_addr = "127.0.0.1:8080";

    // 서버는 클라이언트의 연결을 대기
    let listener = TcpListener::bind(server_addr).expect("bind failed");

    // 연결이 생길 때까지 대기
    println!("Waiting for connection...");
    let (mut stream, addr) = listener.accept().expect("accept failed");
    println!("Connected to {}", addr);

    // 클라이언트로부터의 메시지 수신 및 출력
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    println!("{}", String::from_utf8_lossy(&buffer));

    // 메시지에 대한 응답을 클라이언트에게 전송
    let response = "Hello from server!";
    stream.write(response.as_bytes()).unwrap();

    // 연결 종료
    stream.shutdown(std::net::Shutdown::Both).unwrap();
}
