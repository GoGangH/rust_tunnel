mod client;
mod server;

fn main() {
    // 새로운 thread에서 서버 실행
    let server_handle = std::thread::spawn(|| {
        server::server::main();
    });

    // 클라이언트 실행
    client::client::main();

    // 서버 thread 종료 대기
    server_handle.join().unwrap();
}
