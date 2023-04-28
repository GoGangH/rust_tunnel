use std::net::{Shutdown, TcpStream};
use std::io::{Read, Write};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;

    let (mut rd, mut wr) = stream.split();
    let (mut stdin_rd, mut stdin_wr) = tokio::io::split(tokio::io::stdin());

    tokio::select! {
        res = tokio::io::copy(&mut stdin_rd, &mut wr) => {
            res?;
            stream.shutdown(Shutdown::Write)?;
            Ok(())
        }
        res = tokio::io::copy(&mut rd, &mut stdout) => {
            res?;
            stream.shutdown(Shutdown::Read)?;
            Ok(())
        }
    }
}
