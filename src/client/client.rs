use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};
use std::sync::{Arc, Mutex};

struct Tunnel {
    id: usize,
    stream: TcpStream,
}

impl Tunnel {
    fn new(id: usize, stream: TcpStream) -> Tunnel {
        Tunnel { id, stream }
    }

    fn run(&mut self) {
        loop {
            let mut buffer = [0; 1024];
            match self.stream.read(&mut buffer) {
                Ok(n) => {
                    if n == 0 {
                        println!("Connection closed for tunnel {}", self.id);
                        break;
                    }
                    let received_message = std::str::from_utf8(&buffer[0..n]).unwrap();
                    println!(
                        "Received message in tunnel {}: {}",
                        self.id, received_message
                    );
                }
                Err(err) => {
                    println!("Failed to read from tunnel {}: {}", self.id, err);
                    break;
                }
            }
        }
    }
}

fn create_tunnels(num_tunnels: usize, server_addr: &str) -> Vec<Arc<Mutex<Tunnel>>> {
    let mut tunnels = Vec::new();
    for i in 0..num_tunnels {
        let stream = TcpStream::connect(server_addr).expect("connection failed");
        stream
            .set_nonblocking(true)
            .expect("failed to set non-blocking");
        let tunnel = Arc::new(Mutex::new(Tunnel::new(i, stream)));
        let tunnel_clone = tunnel.clone();
        std::thread::spawn(move || {
            let mut locked_tunnel = tunnel_clone.lock().unwrap();
            locked_tunnel.run();
        });
        tunnels.push(tunnel);
    }
    tunnels
}

fn send_message(tunnel: &Arc<Mutex<Tunnel>>, message: &str) {
    let mut locked_tunnel_guard = tunnel.lock().unwrap();
    let locked_tunnel = &mut *locked_tunnel_guard;
    if locked_tunnel.stream.peer_addr().is_ok() {
        println!("Sending message through tunnel {}", locked_tunnel.id);
        locked_tunnel
            .stream
            .write_all(message.as_bytes())
            .expect("failed to send message");
        return;
    }
    println!("All tunnels are busy");
}

pub fn main() {
    // 클라이언트가 접속할 서버의 IP 주소와 포트 번호
    let server_addr = "127.0.0.1:8080"; //server 포트
    let num_tunnels = 3;
    let tunnel_vec = create_tunnels(num_tunnels, server_addr);
    // db의 주소에 따라 tunnel 라우팅
    let mut route = HashMap::new();
    route.insert(String::from("127.0.0.1:8000"), 0);
    route.insert(String::from("127.0.0.1:8001"), 1);
    route.insert(String::from("127.0.0.1:8002"), 2);

    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        if input.trim() == "exit" {
            println!("connect out!");
            break;
        }
        let tunnel_num = route.get(&input.trim()[0..14]).unwrap();

        let message = format!("{} {}", tunnel_num.to_string(), input[14..].trim());

        send_message(&tunnel_vec[*tunnel_num as usize], &message);
    }

    // 연결 종료
    println!("Closing tunnels...");
    for tunnel in tunnel_vec {
        let mut locked_tunnel_guard = tunnel.lock().unwrap();
        let locked_tunnel = &mut *locked_tunnel_guard;
        let tunnel_id = format!("Tunnel-{}", locked_tunnel.id);
        locked_tunnel
            .stream
            .write(format!("{} exit()", tunnel_id).as_bytes())
            .unwrap();
        locked_tunnel.stream.shutdown(Shutdown::Both).unwrap();
    }
}
