use std::intrinsics::atomic_cxchg_release_acquire;
use std::net::{UdpSocket, TcpStream, SocketAddr, IpAddr, Ipv4Addr};
use std::io::{Write, Read};
use std::time::Duration;
use rand::Rng;
use crate::components::stun1::{StunHeader, StunStats, StunMessage, StunServerConfig, StunAttribute, MessageParams};
use crate::components::attributes::{ATTR_USERNAME, ATTR_MESSAGE_INTEGRITY, ATTR_FINGERPRINT};

use super::attributes::ATTR_SOFTWARE;

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

pub fn discover_public_ip() -> Option<std::net::SocketAddr> {
    let stun_server = "127.0.0.1:19032";
    let socket = UdpSocket::bind("0.0.0.0:0").ok()?;
    let client = StunClient::new(stun_server);
    client.query_external_address(&socket).ok()
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

pub fn build_binding_request(user: &String) -> StunMessage {
    StunMessage {
        header: StunHeader {
            MessageType: 0x0001,
            MessageLength: 0,
            MagicCookie: 0x2112A442,
            TransactionID: generate_transaction_id(),
        },
        attributes: StunAttribute {
            ATTR_Type: 0x0001,
            Length: 0,
            Value: user.to_string(),
        },
        raw: Vec::new(),
    }
}

pub fn build_indication_request(user_identities: &MessageParams) -> StunMessage {
    StunMessage {
        header: StunHeader {
            MessageType: 0x0011,
            MessageLength: 0,
            MagicCookie: 0x2112A442,
            TransactionID: generate_transaction_id(),
        },
        attributes: Vec::new(),
        raw: Vec::new(),
    }
}

pub fn build_vec_of_attributes(user_identities: MessageParams) -> Vec<StunAttribute> {
    let mut attributes: Vec<StunAttribute> = Vec::new();
    match user_identities.MESS_TYPE {
        0x0001 => {
           attributes.push(StunAttribute {
           ATTR_Type: ATTR_USERNAME,
           Length: user_identities.USERNAME.to_string().len(),
           Value: user_identities.USERNAME.to_string(),
        });
        attributes.push(StunAttribute {
           ATTR_Type: ATTR_MESSAGE_INTEGRITY,
           Length: user_identities.MESSAGE_INTEGRITY.to_string().len(),
           Value: user_identities.MESSAGE_INTEGRITY.to_string(),
        });
        attributes.push(StunAttibute {
           ATTR_Type: ATTR_SOFTWARE,
           Length: user_identities.SOFTWARE.to_string().len(),
           Value: user_identities.to_string(),
        });
        }
        0x0011 => {
            attributes.push(StunAttribute {
                ATTR_Type: ATTR_SOFTWARE,
                Length: user_identities.SOFTWARE.to_string().len(),
                Value: user_identities.SOFTWARE.to_string(),
            });
            attributes.push(StunAttribute {
                ATTR_Type: ATTR_USERNAME,
                Length: user_identities.USERNAME.to_string().len(),
                Value: user_identities.USERNAME.to_string(),
            });
            attributes.push(StunAttribute {
                ATTR_Type: ATTR_MESSAGE_INTEGRITY,
                Length: user_identities.MESSAGE_INTEGRITY.to_string().len(),
                Value: user_identities.MESSAGE_INTEGRITY.to_string(),
            });
            attributes.push(StunAttribute {
                ATTR_Type: ATTR_FINGERPRINT,
                Length: user_identities.FINGERPRINT.to_string().len(),
                Value: user_identities.MESSAGE_INTEGRITY.to_string(),
            })
        }
        _ => None,
    } 
    
    
    attributes
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
