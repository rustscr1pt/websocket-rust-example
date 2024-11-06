use std::net::SocketAddr;
use log::info;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let listener =
        TcpListener::bind("0.0.0.0:8000".parse::<SocketAddr>().unwrap())
            .await
            .expect("Couldn't bind.");
    info!("Listening on : http://localhost:8000");
    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(handle_connection(stream))
    }
}
