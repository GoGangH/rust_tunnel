use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};

pub fn main() {
    // 클라이언트가 접속할 서버의 IP 주소와 포트 번호
    let server_addr = "127.0.0.1:8080";

    // 클라이언트는 서버로의 연결을 시도
    let mut stream = TcpStream::connect(server_addr).expect("connection failed");

    // 표준 입력에서 읽어들이고, 서버로 전송
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    stream.write(input.as_bytes()).unwrap();

    // 서버로부터의 응답 출력
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    println!("{}", String::from_utf8_lossy(&buffer));

    // 연결 종료
    stream.shutdown(Shutdown::Both).unwrap();
}
