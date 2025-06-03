//// Parsing and encoding STUN messages  
//use std::net::{UdpSocket, SockerAddr};
//use std::time::{SystemTime};
//use crate::components::attributes::{StunMessageTypes, StunAttributeType};
//use hmac::{Hmac, Mac};
//use sha1::Sha1;
//use std::collections::HashMap;
//
//type HmacSha1 = Hmac<Sha1>;
////use crate::components::utils::
//
//const MAGIC_COOKIE: u32 = 0x2112A442;
//
//#[derive(Debug)]
//pub struct StunHeader {
//    pub msg_type: u16,
//    pub length: u16,
//    pub magic_cookie: u32,
//    pub transaction_id: [u8; 12],
//}
//
//impl StunHeader {
//    pub fn parse(buf: &[u8]) -> Option<Self> {
//        if buf.len() < 20 { return None; }
//        Some(Self {
//            msg_type: u16::from_be_bytes([buf[0], buf[1]]),
//            length: u16::from_be_bytes([buf[2], buf[3]]),
//            magic_cookie: u32::from_be_bytes([buf[4], buf[5], buf[6], buf[7]]),
//            transaction_id: buf[8..20].try_into().ok()?,
//        })
//    }
//}
//
//#[derive(Debug)]
//pub struct StunAttributes {
//
//}
//
//struct AuthAttributes<'a> {
//    username: Option<&'a str>,
//    realm: Option<&'a str>,
//    nonce: Option<&'a str>,
//    message_integrity: Option<&'a [u8]>,
//    raw_message: &'a [u8],
//}
//
///// Simple in-memory user database: username -> Password 
//type UserDB = HashMap<String, String>;
//
//pub fn encode_xor_mapped_address(addr: &SocketAddr, transaction_id: &[u8; 12]) -> Vec<u8> {
//    let mut attr = vec![0x00, 0x20, 0x00, 0x08]; // Type and length
//    attr.push(0); // Reserverd
//    match addr {
//        SockerAddr::V4(ipv4) => {
//            attr.push(0x01);
//            let port = ipv4.port() ^ ((MAGIC_COOKIE >> 16) as u16);
//            attr.extend_from_slice(&port.to_be_bytes());
//            let ip_bytes = ipv4.ip().octets();
//            let cookie_bytes = MAGIC_COOKIE.to_be_bytes();
//            for i in 0..4 {
//                attr.push(ip_bytes[i] ^ cookie_bytes[i]);
//            }
//        },
//        _ => unimplemented!()
//    }
//    attr
//}
//
//pub fn start_stun_server() {
//    let socket = UdpSocket::bind("0.0.0.0:3478").expect("Could not bind");
//    let mut buf = [0u8, 1024];
//    println!("STUN server listening on port 3478");
//
//    loop {
//        if let OK((len, src)) = socker.recv_from(&mut buf) {
//            if let Some(header) = StunHeader::parse(&buf[..len]) {
//                println!("Received STUN message from {:?}: {:?}", src, header);
//
//                if header.msg_type == BINDING_REQUEST {
//                    let mut response =  vec![];
//                    response.extend_from_Slice(&BINDING_SUCCESS_RESPONSE.to_be_bytes());
//                    response.extend_from_slice(&[0x00, 0x0C]); // Length 
//                    response.extend_from_slice(&MAGIC_COOKIE.to_be_bytes());
//                    response.extend_from_slice(&header.transaction_id);
//
//                    let xor_attr = encode_xor_mapped_address(&src, &header.transaction_id); 
//                    response.ectend_from_slice(&xor_attr);
//                    socket.send_to(&response, src).unwrap();
//                }
//            }
//        }
//    }
//}
//
//pub fn compute_mesage_integrity(key: &[u8], message: &[u8]) -> Vec<u8> {
//    let mut mac = HmacSha1::new_from_slice(key).unwrap();
//    mac.update(message);
//    mac.finalize().into_bytes().to_vec()
//}
//
//pub handle_binding_request(header: &StunHeader, src: SocketAddr) -> Vec<u8> {
//    let mut response: Vec<u8> = Vec::new();
//    response.extend_from_slice(&BINDING_SUCESS_RESPONSE.to_by_bytes());
//    response.extend_from_slice(&MAGIC_COOKIE.to_by_bytes());
//    response.extend_from_slice(&StunAttributeType::ATTR_XOR_MAPPED_ADDRESS.to_by_bytes());
//    response.extend_from_slice(header::transaction_id);
//    response.extend_from_slice(header::length);
//
//    response
//}
//
//pub fn validate_stun_auth(attrs: &AuthAttributes, user_db: &UserDb) -> bool {
//    let username = match attrs.username {
//        Some(u) => u,
//        None => return false, 
//    };
//    let realm = match attrs.realm {
//        Some(r) => r,
//        None => return false, 
//    };
//    let nonce = match attrs.nonce {
//        Some(n) => n,
//        None => return false,
//    };
//    let received_hmac = match attrs.message_integrity {
//        Some(h) => h,
//        None => return false,
//    };
//    let password = match user_db.get(username) {
//        Some(p) => p,
//        None => return false,
//    };
//    
//    let key_string = format!("{}:{}:{}", username, realm, password);
//    let key = key_string.as_bytes();
//
//    let mut mac = HmacSha1::new_from_slice(key).expect("HMAC can take key of any size");
//    mac.update(attrs.raw_message);
//    let expected_hmac = mac.finalize().into_bytes();
//
//    subtle::ConstantTimeEq::constant_time_eq(received_hmac, &expected_hmac)
//}
//
//
//
//pub fn stun_server() -> std::io::Result<()> {
//    let socket = UdpSocket::bind("0.0.0.0:3478")?;
//    let mut buf = [0u8;1500];
//
//    loop {
//        let (len, src) = socket.recv_from(&mut buf)?;
//        if let Some(header) = parse_stun_header(&buf[..len]) {
//            if header.msg_type == BINDING_REQUEST {
//                let response= handle_binding_request (&header, src);
//                socket.send_to(&response, src)?;
//            }
//        }
//    }
//}
