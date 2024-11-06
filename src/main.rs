use std::net::SocketAddr;
use log::{error, info};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, WebSocketStream};
use tokio_tungstenite::tungstenite::Error;

async fn handle_connection(stream : TcpStream) -> () {
    // Accept the websocket connection
    let ws_stream = match accept_async(stream).await {
        Ok(ws) => ws,
        Err(err) => {
            error!("Error during the websocket handshake : {}", err);
            return;
        }
    };

}

#[tokio::main]
async fn main() -> () {
    let listener =
        TcpListener::bind("0.0.0.0:8000".parse::<SocketAddr>().unwrap())
            .await
            .expect("Couldn't bind.");
    info!("Listening on : http://localhost:8000");
    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(handle_connection(stream))
    }
}
