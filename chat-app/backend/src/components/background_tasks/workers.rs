use tokio::time::{sleep, Duration};
use warp::Filter;
use tokio::sync::mpsc;
use watp::ws::{Message, WebSocket};
use futures_util::{StremExt, SinkExt};

pub async fn start_worker() {
    loop {
        println!("Running background task...");
        sleep(Duration::from_secs(60)).await;
    }
}

pub async fn handle_connection(ws: WebSocket, sender: mpsc::Sender<String>, mut receiver: mpsc::Receiver<String>) {
    let (mut tx, mut rx) = ws.split();

    tokio::spawn(async move {
        while let Some(Ok(msg)) = rx.next().await {
            if let Ok(text) = msg.to_str() {
                println!("Received WebRtc Signal: {}", text);
                sender.send(text.to_string()).await.unwrap();
            }
        }
    });

    tokio::spawn(async move {
        while let Some(Ok(msg)) = rx.next().await {
            if  let Ok(text) = msg.to_str() {
                println!("Received WebRTC Signal: {}", text);
                sender.send(text.to_string()).await.unwrap();
            }
        }
    });
}

pub async fn start_signaling_server() {
    let (tx, rx) = mpsc::channel(32);

    let route = warp::path("ws")
        .and(warp::ws())
        .map(move |ws: warp::ws::Ws| {
            let tx = tx.clone();
            let rx = rx.clone();
            ws.on_upgrade(move |socket| handle_connection(socket, tx, rx))
        });

    warp::serve(route).run(([0, 0, 0, 0], 9000)).await;
}
