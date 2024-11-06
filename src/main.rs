use std::net::SocketAddr;  // Import `SocketAddr` for handling IP address and port
use futures::{SinkExt, StreamExt};  // Import `SinkExt` and `StreamExt` for working with asynchronous streams and sinks
use log::{error, info};  // Import logging macros for error and info messages
use tokio::net::{TcpListener, TcpStream};  // Import `TcpListener` and `TcpStream` for handling TCP connections
use tokio_tungstenite::{accept_async, WebSocketStream};  // Import `accept_async` to upgrade TCP to WebSocket, and `WebSocketStream` for the WebSocket stream
use tokio_tungstenite::tungstenite::{Error, Message};  // Import `Error` for WebSocket error handling and `Message` for WebSocket messages

// Define an async function to handle each WebSocket connection, taking in a `TcpStream` as the argument
async fn handle_connection(stream: TcpStream) -> () {
    // Try to accept the WebSocket handshake on the incoming TCP connection
    let ws_stream = match accept_async(stream).await {
        Ok(ws) => ws,  // If successful, `ws` is a WebSocket stream
        Err(err) => {  // If there’s an error during the handshake
            error!("Error during the websocket handshake: {}", err);  // Log the error
            return;  // Exit the function early
        }
    };

    // Split the WebSocket stream into `sender` and `receiver` to handle sending and receiving separately
    let (mut sender, mut receiver) = ws_stream.split();

    // Enter a loop to read messages from the WebSocket `receiver`
    while let Some(message) = receiver.next().await {
        match message {
            // If the message is a text message, reverse the text and send it back
            Ok(Message::Text(text)) => {
                let reversed = text.chars().rev().collect::<String>();  // Reverse the text content
                // Attempt to send the reversed text back to the client
                if let Err(err) = sender.send(Message::Text(reversed)).await {
                    error!("Error sending message: {}", err);  // Log any errors during sending
                }
            }
            // If the message is a close message, break out of the loop to end the connection
            Ok(Message::Close(_)) => break,
            // If there is an error in receiving a message, log it and break the loop
            Err(err) => {
                error!("Error processing message: {}", err);
                break;
            }
            _ => {}  // Ignore other message types (e.g., binary messages)
        }
    }
}

// The main function, annotated with `#[tokio::main]` to run the Tokio async runtime
#[tokio::main]
async fn main() -> () {
    // Create a TCP listener bound to `0.0.0.0:8000`, meaning it listens on all available IPs on port 8000
    let listener =
        TcpListener::bind("0.0.0.0:8000".parse::<SocketAddr>().unwrap())  // Parse and unwrap the socket address
            .await
            .expect("Couldn't bind.");  // Panic if the listener could not be created

    // Log that the server is listening on the specified address
    info!("Listening on: http://localhost:8000");

    // Enter a loop to accept incoming TCP connections
    while let Ok((stream, _)) = listener.accept().await {
        // For each incoming TCP connection, spawn a new async task to handle it as a WebSocket connection
        tokio::spawn(handle_connection(stream));
    }
}