use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    while let Ok((mut stream, _)) = listener.accept().await {
        let (mut rd, mut wr) = stream.split();
        let (stdin_rd, mut stdin_wr) = tokio::io::split(tokio::io::stdin());

        tokio::spawn(async move {
            let mut buf = [0u8; 4096];
            loop {
                match rd.read(&mut buf).await {
                    Ok(0) => break,
                    Ok(n) => {
                        stdin_wr.write_all(&buf[..n]).await.unwrap();
                        stdin_wr.flush().await.unwrap();
                    }
                    Err(e) => {
                        eprintln!("error while reading from stream; err = {:?}", e);
                        break;
                    }
                }
            }
        });

        let mut buf = [0u8; 4096];
        loop {
            match tokio::io::stdin().read(&mut buf).await {
                Ok(0) => break,
                Ok(n) => {
                    wr.write_all(&buf[..n]).await.unwrap();
                    wr.flush().await.unwrap();
                }
                Err(e) => {
                    eprintln!("error while reading from stdin; err = {:?}", e);
                    break;
                }
            }
        }
    }

    Ok(())
}
