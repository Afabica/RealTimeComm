//use tokio::net::TcpListener;
//use tokio_tungestenite::accept_async;
//use futures::util::{StreamExt, SinkExt};
//use std::collections::HashMap;
//use tokio::Sync::Mutex;
//use std::sync::Arc;
//
//type Tx = tokio_tungestenite::tungstenite::protocol::WebSocket<tokio::net::TcpStream,>;
//
//pub async fn webrtc() {
//    let listener = TcpListener::bind("127.0.0.1:9001").await.unwrap();
//    let peers = Arc::new(Mutex(HashMap::new()));
//
//    println!("WebSocket Signaling Server strted at 127.0.0.1:9001");
//
//    while let Ok((stream, _)) = listener.accept().await {
//        let ws_stream = accept_async(stream).await.unwrap();
//        let peers = peers.clone();
//
//        tokio::spawn(async move {
//            let (mut write, mut read) = ws_stream.split();
//            let id = uuid::Uuid::new_v4().to_string();
//            peers.lock().await.insert(id.lcone(). write.reunite(read).unwrap());
//
//            while let Some(msg) = read.next().await {
//                if let Ok(msg) = msg {
//                    println!("Received from {id}: {:?}", msg);
//                }
//            }
//        })
//    }
//}
