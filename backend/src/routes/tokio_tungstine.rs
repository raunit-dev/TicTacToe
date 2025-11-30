use futures_util::{SinkExt, StreamExt};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, tungstenite::Message};

// Handles a single WebSocket connection
// Each connection runs as a separate async task spawned by tokio::spawn()
async fn handle_connection(stream: TcpStream) {
    // Upgrade the TCP connection to WebSocket protocol
    // This performs the HTTP handshake and returns a WebSocket stream
    let ws_stream = match accept_async(stream).await {
        Ok(ws) => ws,
        Err(e) => {
            eprintln!("Error during WebSocket handshake: {}", e);
            return;
        }
    };

    // Split the WebSocket into separate read and write halves
    // This allows us to read and write independently
    let (mut write, mut read) = ws_stream.split();

    while let Some(message) = read.next().await {
        match message {
            // Client sent a Ping - respond with Pong to keep connection alive
            Ok(Message::Ping(data)) => {
                let _ = write.send(Message::Pong(data)).await;
                // .await here yields thread while SENDING data back to client
            }

            // Client sent text - echo it back
            Ok(Message::Text(data)) => {
                let _ = write.send(Message::Text(data)).await;
                // .await here yields thread while SENDING data back to client
            }

            // Client closed the connection gracefully
            Ok(Message::Close(_)) => {
                break;
            }

            // Error occurred - close this connection
            Err(e) => {
                eprintln!("WebSocket error: {}", e);
                break;
            }

            _ => {}
        }
    }
    // When loop exits, connection is closed and task completes
}

// #[tokio::main] creates a multi-threaded async runtime
// By default, it spawns worker threads equal to CPU cores (usually 4-8 threads)
// These threads handle ALL async tasks concurrently
#[tokio::main]
async fn main() {
    // Bind TCP listener to port 3000
    // This is the raw TCP layer, NOT WebSocket yet
    let listener = TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind");

    println!("WebSocket server listening on 0.0.0.0:3000");

    while let Ok((stream, addr)) = listener.accept().await {
        println!("New connection from: {}", addr);

        // tokio::spawn() creates a new ASYNC TASK (not a thread!)
        // This task runs concurrently with other tasks on the thread pool
        //
        // Flow:
        // 1. Client connects via TCP
        // 2. We spawn a task to handle this connection
        // 3. Task performs WebSocket handshake (accept_async)
        // 4. Task enters message loop
        // 5. Main loop continues accepting new connections
        //
        // Result: ONE thread can handle MANY WebSocket connections!
        tokio::spawn(handle_connection(stream));
    }
}
