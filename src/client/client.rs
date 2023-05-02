use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};
use std::thread;
pub fn main() {
    // 클라이언트가 접속할 서버의 IP 주소와 포트 번호
    let server_addr = "127.0.0.1:8080";
    let num_tunnels = 3;
    // let mut tunnel_vec = create_tunnels(num_tunnels, server_addr);

    for i in 0..num_tunnels {
        // 클라이언트는 서버로의 연결을 시도
        let mut stream = TcpStream::connect(server_addr).expect("connection failed");

        // 서버에게 터널 ID를 보냄
        let tunnel_id = format!("Tunnel-{}", i);
        stream.write(tunnel_id.as_bytes()).unwrap();

        // 새로운 스레드에서 터널로부터 메시지 수신
        let mut tunnel = stream.try_clone().unwrap();
        thread::spawn(move || {
            loop {
                let mut buffer = [0; 512];
                match tunnel.read(&mut buffer) {
                    Ok(n) if n > 0 => {
                        println!(
                            "Received from tunnel {}: {}",
                            tunnel_id,
                            String::from_utf8_lossy(&buffer[..n])
                        );
                    }
                    Ok(_) | Err(_) => break,
                }
            }
            println!("Tunnel {} closed", tunnel_id);
        });
    }
    //메인 스레드에서 사용자 입력을 받아서 터널로 전송
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        if input.trim() == "exit" {
            println!("connect out!");
            break;
        }

        // 모든 터널로 메시지 전송
        for i in 0..num_tunnels {
            let tunnel_id = format!("Tunnel-{}", i);
            let msg = format!("[{}] {}", tunnel_id, input.trim());
            let mut tunnel = TcpStream::connect(server_addr).unwrap(); // 새로운 연결 생성
            tunnel.write(msg.as_bytes()).unwrap();
        }
    }
    // 연결 종료
    println!("Closing tunnels...");
    for i in 0..num_tunnels {
        let tunnel_id = format!("Tunnel-{}", i);
        let mut tunnel = TcpStream::connect(server_addr).unwrap(); // 새로운 연결 생성
        tunnel
            .write(format!("{} exit()", tunnel_id).as_bytes())
            .unwrap();
        tunnel.shutdown(Shutdown::Both).unwrap();
    }
}
