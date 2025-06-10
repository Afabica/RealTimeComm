use std::net::{UdpSocket, SocketAddr, IpAddr, Ipv4Addr};
use std::time::Duration;
use rand::Rng;
mod components;
use components::functionality::{build_binding_request, parse_xor_mapped_address, generate_transaction_id, discover_public_ip};
use components::functionality::udp_connector;
use components::stun1::{StunMessage, StunHeader, StunAttribute};

fn main() -> std::io::Result<()> {
    udp_connector()

//    let stun_server = "127.0.0.1:8080";
//    let socket = UdpSocket::bind("0.0.0.0:0")?;
//    socket.set_read_timeout(Some(Duration::from_secs(2)))?;
//    let address_pub = discover_public_ip();
//
//    let transaction_id = generate_transaction_id();
//    let hex_string = transaction_id.iter().map(|b| format!("{:02x}", b)).collect::<String>();
// 
//    let mut request_msg =  build_binding_request();
//
//    let request_bytes = request_msg.to_bytes();
//
//    // Send request
//    socket.send_to(request_bytes, stun_server)?;
//    println!("STUN Binding Request sent.");
//
//    // Receive response
//    let mut buf = [0u8; 1500];
//    let (len, _src) = socket.recv_from(&mut buf)?;
//    println!("Received response ({} bytes)", len);
//
//    // Parse response header
//    let header = match StunHeader::parse(&buf[..len]) {
//        Some(h) => h,
//        None => {
//            eprintln!("Failed to parse STUN header");
//            return Ok(());
//        }
//    };
//
//    // Parse attributes
//    let attributes = match StunAttribute::parse(&buf[20..len]) {
//        Ok(attrs) => attrs,
//        Err(e) => {
//            eprintln!("Failed to parse STUN attributes: {}", e);
//            return Ok(());
//        }
//    };
//
//    let response_msg = StunMessage {
//        header,
//        attributes,
//        raw: buf[..len].to_vec(),
//    };
//
//    // Find XOR-MAPPED-ADDRESS attribute
//    for attr in &response_msg.attributes {
//        if attr.ATTR_Type == 0x0020 {
//            if let Some(public_addr) = parse_xor_mapped_address(attr) {
//                println!("Public IP and Port: {}", public_addr);
//                return Ok(());
//            }
//        }
//    }
//
//    println!("XOR-MAPPED-ADDRESS attribute not found.");
//    Ok(())
}

