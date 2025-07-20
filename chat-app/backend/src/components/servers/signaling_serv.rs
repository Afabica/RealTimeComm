// ws_server.rs

use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use futures_util::StreamExt;
use std::collections::HashMap;
use tokio::sync::Mutex;
use std::sync::Arc;
use uuid::Uuid;
use tokio_tungstenite::tungstenite::Message;

/// Type alias for peer map storage
type PeerMap = Arc<Mutex<HashMap<String, ()>>>;

/// Starts the WebSocket signaling server (run this in main)
pub async fn ws_handler() {
    let listener = TcpListener::bind("127.0.0.1:9001")
        .await
        .expect("Failed to bind WebSocket server");

    let peers: PeerMap = Arc::new(Mutex::new(HashMap::new()));
    println!("WebSocket Signaling Server running at ws://127.0.0.1:9001");

    while let Ok((stream, _)) = listener.accept().await {
        let peers = peers.clone();

        tokio::spawn(async move {
            match accept_async(stream).await {
                Ok(ws_stream) => {
                    let (mut ws_write, mut ws_read) = ws_stream.split();
                    let id = Uuid::new_v4().to_string();

                    println!("New connection: {}", id);
                    peers.lock().await.insert(id.clone(), ());

                    while let Some(message) = ws_read.next().await {
                        match message {
                            Ok(Message::Text(text)) => {
                                println!("Received from {}: {}", id, text);
                                // Echo back or broadcast logic could go here
                            }
                            Ok(Message::Binary(bin)) => {
                                println!("Binary from {}: {:?}", id, bin);
                            }
                            Ok(Message::Close(reason)) => {
                                println!("Client {} disconnected: {:?}", id, reason);
                                break;
                            }
                            Err(e) => {
                                eprintln!("Error from {}: {}", id, e);
                                break;
                            }
                            _ => {}
                        }
                    }

                    peers.lock().await.remove(&id);
                }
                Err(e) => eprintln!("WebSocket upgrade error: {}", e),
            }
        });
    }
}

