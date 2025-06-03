use rstun::{StunServer, UdpSocket};
use tokio::net::UdpSocket as TokioUdpSocket;

pub async fn start_stun_server() {
    let socket = TokioUdpSocket::bind("0.0.0.0:3478").await.unwrap(); 
    let socket = UdpSocket::from_std(socket.into_std().unwrap()).unwrap();
    let server = StunServer::new(socket);
    println!("STUN server is running on port 3478...");

    server.serve().await.unwrap();
}
