use std::net::{UdpSocket, TcpStream, SocketAddr, IpAddr, Ipv4Addr};
use std::time::Duration;
use std::io::{Write, Read};
use stunclient::StunClient;
use rand::Rng;

use crate::components::stun1::{StunHeader, StunMessage, StunAttribute};
use crate::components::attributes::{ATTR_USERNAME, ATTR_MESSAGE_INTEGRITY, ATTR_FINGERPRINT, ATTR_SOFTWARE};

pub fn tcp_connector(address: &str) -> std::io::Result<()> {
    let mut stream = TcpStream::connect(address)?;
    println!("Connected to server!");

    let msg = b"Hello TCP Server!";
    stream.write_all(msg)?;
    println!("Sent message: {}", String::from_utf8_lossy(msg));

    let mut buffer = [0; 512];
    let n = stream.read(&mut buffer)?;

    println!("Received: {}", String::from_utf8_lossy(&buffer[..n]));
    Ok(())
}

pub fn discover_public_ip() -> Option<SocketAddr> {
    let stun_server = "127.0.0.1:19032";
    let socket = UdpSocket::bind("0.0.0.0:0").ok()?;
    let client = StunClient::new(stun_server.parse().expect("Unable to parse socket address"));
    client.query_external_address(&socket).ok()
}

pub fn udp_connector() -> std::io::Result<()> {
    let stun_server = "127.0.0.1:8080";
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    
    loop {
        let mut request = build_binding_request();
        socket.send_to(&request.to_bytes(), stun_server)?;
        println!("Sent STUN Binding Request");

        let mut buf = [0u8; 1024];
        match socket.recv_from(&mut buf) {
            Ok((size, src)) => {
                println!("Received {} bytes from {}", size, src);
            }
            Err(e) => {
                println!("No response: {}", e);
            }
        }

        std::thread::sleep(Duration::from_secs(5));
    }
}


pub fn generate_transaction_id() -> [u8; 12] {
    let mut rng = rand::thread_rng();
    let mut id = [0u8; 12];
    rng.fill(&mut id);
    id
}

pub fn build_binding_request() -> StunMessage {
    StunMessage {
        header: StunHeader {
            message_type: 0x0001,
            message_length: 0,
            magic_cookie: 0x2112A442,
            transaction_id: generate_transaction_id(),
        },
        attributes: Vec::new(),
        raw: Vec::new(),
    }
}

pub fn build_request() -> StunMessage {
    let stun_package: StunMessage = StunMessage {
        header: StunHeader {
            message_type: 0x0000,
            message_length: 0,
            magic_cookie: 0,
            transaction_id: generate_transaction_id()
        },
        attributes: Vec::new(),
        raw: Vec::new(),
    };

    

    stun_package
}

pub fn parse_xor_mapped_address(attr: &StunAttribute) -> Option<SocketAddr> {
    if attr.value.len() < 8 {
        return None;
    }

    println!("Parsing attribute type: {}", attr.attr_type);
    println!("Raw bytes: {:?}", attr.value);

    let family = attr.value[1];
    let port_xored = u16::from_be_bytes([attr.value[2], attr.value[3]])
        ^ ((0x2112A442u32 >> 16) as u16);

    match family {
        0x01 => {
            let ip_bytes: Vec<u8> = attr.value[4..8]
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

pub fn parse_stun_header(buf: &[u8]) -> Option<StunHeader> {
    if buf.len() < 20 {
        return None;
    }

    let message_type = u16::from_be_bytes([buf[0], buf[1]]);
    let message_length = u16::from_be_bytes([buf[2], buf[3]]);
    let magic_cookie = u32::from_be_bytes([buf[4], buf[5], buf[6], buf[7]]);

    if magic_cookie != 0x2112A442 {
        return None;
    }

    let mut transaction_id = [0u8; 12];
    transaction_id.copy_from_slice(&buf[8..20]);

    Some(StunHeader {
        message_type,
        message_length,
        magic_cookie,
        transaction_id,
    })
}

pub fn parse_stun_attributes(buf: &[u8]) -> Vec<StunAttribute> {
    let mut attrs = Vec::new();
    let mut pos = 0;

    while pos + 4 <= buf.len() {
        let attr_type = u16::from_be_bytes([buf[pos], buf[pos + 1]]);
        let attr_len = u16::from_be_bytes([buf[pos + 2], buf[pos + 3]]) as usize;
        pos += 4;

        if pos + attr_len > buf.len() {
            break;
        }

        let attr_value = &buf[pos..pos + attr_len];
        pos += attr_len;

        let padding = (4 - (attr_len % 4)) % 4;
        pos += padding;

        attrs.push(StunAttribute {
            attr_type,
            value: attr_value.to_vec(),
            length: attr_len as u16,
        });
    }
    attrs
}

