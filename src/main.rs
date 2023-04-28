mod client;
mod server;

use tokio::join;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let server_handle = tokio::spawn(async move {
        if let Err(e) = server::main().await {
            eprintln!("server error: {:?}", e);
        }
    });
    let client_handle = tokio::spawn(async move {
        if let Err(e) = client::main().await {
            eprintln!("client error: {:?}", e);
        }
    });

    join!(server_handle, client_handle);

    Ok(())
}
