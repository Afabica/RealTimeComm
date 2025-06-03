use tokio::sync::mpsc;
use warp::ws::{Message, WebSocket};
use warp::Filter;

pub async fn start_signaling_server() {
    let route = warp::path("ws")
        .and(warp::ws())
        .map(|ws: warp::ws::ws| ws.on_upgrade(handle_ws_connection));

    warp::serve(route).run(([0, 0, 0, 0], 9000)).await;

}

pub async fn handle_ws_connection(ws: WebSocket) {
    let (mut tx, mut rx) = ws.split();
    while let Some(Ok(msg)) = rx.next().await {
        println!("Received WebRTC Signal: {}", text);
        tx.send(Message::text(text)).await.unwrap();
    }
}
