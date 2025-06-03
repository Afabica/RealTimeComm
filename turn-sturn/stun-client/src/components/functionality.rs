use std::net::{UdpSocket, TcpStream, SocketAddr, IpAddr, Ipv4Addr};
use std::io::{Write, Read};
use std::time::Duration;
use rand::Rng;
use crate::components::stun1::{StunHeader, StunStats, StunMessage, StunServerConfig, StunAttribute};

pub fn tcp_connector(address: &str) -> std::io::Result<()> {
    // Connect to the TCP server, unwrap Result or propagate error with `?`
    let mut stream = TcpStream::connect(address)?;
    println!("Connected to server!");

    let msg = b"Hello TCP Server!";
    stream.write_all(msg)?;  // send message
    println!("Sent message: {}", String::from_utf8_lossy(msg));

    let mut buffer = [0; 512];
    let n = stream.read(&mut buffer)?;  // read response, get number of bytes read

    println!("Received: {}", String::from_utf8_lossy(&buffer[..n]));
    Ok(())
}

pub fn udp_connector() -> std::io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    let server_addr = "127.0.0.1:8080";

    let msg = b"Hello UDP server!";
    socket.send_to(msg, server_addr)?;
    println!("Sent: {}", String::from_utf8_lossy(msg));

    let mut buf = [0u8; 1024];
    let (amt, src) = socket.recv_from(&mut buf)?;
    let received = String::from_utf8_lossy(&buf[..amt]);

    println!("Received from {}: {}", src, received);
    Ok(())
}

pub fn generate_transaction_id() -> [u8; 12] {
    let mut rng = rand::thread_rng();
    let mut id = [0u8; 12];
    rng.fill(&mut id);
    id
}

pub fn build_binding_request(transaction_id: [u8; 12]) -> StunMessage {
    StunMessage {
        header: StunHeader {
            MessageType: 0x0001,
            MessageLength: 0,
            MagicCookie: 0x2112A442,
            TransactionID: transaction_id,
        },
        attributes: Vec::new(),
        raw: Vec::new(),
    }
}


pub fn parse_xor_mapped_address(attr: &StunAttribute) -> Option<SocketAddr> {
    if attr.Value.len() < 8 {
        return None;
    }

    let family = attr.Value[1];
    let port_xored = u16::from_be_bytes([attr.Value[2], attr.Value[3]]) ^ ((0x2112A442u32 >> 16) as u16);

    match family {
        0x01 => { // IPv4
            let ip_bytes: Vec<u8> = attr.Value[4..8]
                .iter()
                .zip(&0x2112A442u32.to_be_bytes())
                .map(|(b, m)| b ^ m)
                .collect();
            Some(SocketAddr::new(
                IpAddr::V4(Ipv4Addr::new(ip_bytes[0], ip_bytes[1], ip_bytes[2], ip_bytes[3])),
                port_xored,
            ))
        }
        _ => None,
    }
}
