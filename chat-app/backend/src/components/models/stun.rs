use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr};
use std::collections::HashMap;
use std::sync::atomic::AtomicU64;
use std::io::{Cursor, Write};
use byteorder::{BigEndian, WriteBytesExt};
//use crate::components::attributes::StunMessageTypes;
use crate::components::attributes::attributes::StunMessageTypes;

#[derive(Debug, Clone)]
pub struct StunHeader {
    pub MessageType: u16,
    pub MessageLength: u16,
    pub MagicCookie: u32,
    pub TransactionID: [u8; 12]
}

impl StunHeader {
    pub fn parse(buf: &[u8]) -> Option<Self> {
        if buf.len() < 20 { return None; }
        Some(Self {
            MessageType: u16::from_be_bytes([buf[0],buf[1]]),
            MessageLength: u16::from_be_bytes([buf[2], buf[3]]),
            MagicCookie: u32::from_be_bytes([buf[4], buf[5], buf[6], buf[7]]),
            TransactionID: buf[8..20].try_into().ok()?,
        })
    }

    pub fn new() -> Option<Self> {
        Some(Self {
            MessageType: '0' as u16,
            MessageLength: '0'  as u16,
            MagicCookie: '0' as u32,
            TransactionID: [0; 12],
        })
    }
}



#[derive(Debug, Clone)]
pub struct StunAttribute {
    pub ATTR_Type: u16,
    pub Length: u16,
    pub Value: Vec<u8>,
}

impl StunAttribute {
    pub fn parse(mut bytes: &[u8]) -> Result<Vec<StunAttribute>, String> {
        let mut attrs = Vec::new();

        while !bytes.is_empty() {
            if bytes.len() < 4 {
                return Err("Not enough bytes for attribute header".to_string());
            }

            let attr_type = u16::from_be_bytes([bytes[0], bytes[1]]);
            let length = u16::from_be_bytes([bytes[2], bytes[3]]);
            bytes = &bytes[4..];
            if bytes.len() < length as usize {
                return Err("Not enough bytes for attribute value".to_string());
            }

            let value = bytes[..length as usize].to_vec();

            attrs.push(StunAttribute {
                ATTR_Type: attr_type,
                Length: length,
                Value: value,
            });

            bytes = &bytes[length as usize..];

            let padding = (4 - (length as usize % 4)) % 4;

            if bytes.len() < padding {
                return Err("Not enough bytes for padding".to_string());
            }

            bytes = &bytes[padding..];
        }
        Ok(attrs)
    }
}

#[derive(Debug, Clone)]
pub struct StunMessage {
    pub header: StunHeader,
    pub attributes: Vec<StunAttribute>,
    pub raw: Vec<u8>,
}

impl StunMessage {
    pub fn to_bytes(&mut self) -> &[u8] {
        let mut buf = Cursor::new(Vec::new());

        buf.write_u16::<byteorder::BigEndian>(self.header.MessageType).unwrap();
        buf.write_u16::<byteorder::BigEndian>(0).unwrap();
        buf.write_u32::<byteorder::BigEndian>(self.header.MagicCookie).unwrap();
        buf.write_all(&self.header.TransactionID).unwrap();

        for attr in &self.attributes {
            buf.write_u16::<byteorder::BigEndian>(attr.ATTR_Type).unwrap();
            buf.write_u16::<byteorder::BigEndian>(attr.Length).unwrap();
            buf.write_all(&attr.Value).unwrap();
            let pad = (4 - (attr.Length as usize % 4)) % 4;
            buf.write_all(&vec![0u8; pad]).unwrap();
        }
        
        let final_buf = buf.into_inner();
        let attr_len = (final_buf.len() - 20) as u16;

        let mut final_buf = final_buf;
        final_buf[2..4].copy_from_slice(&attr_len.to_be_bytes());

        self.header.MessageLength = attr_len;
        self.raw = final_buf;
        &self.raw
    }
}

#[derive(Debug, Clone)]
pub enum XorMappedAddress {
    V4 {
        family: u8,
        port: u16,
        ip: Ipv4Addr,
    },
    V6 {
        family: u8,
        port: u16,
        ip: Ipv6Addr,
    }
}

#[derive(Debug, Clone)] 
pub enum ResponseHandling {
    SuccResponse {
        transaction_id: [u8; 12],
        mapped_address: SocketAddr,
        message_integrity: Option<[u8; 20]>,
        fingerprint: Option<u32>,
    },
    ErrorResponse {
        transaction_id: [u8; 12],
        error_code: u16,
        reason: String,
        unknown_attributes: Option<Vec<u16>>,
        realm: Option<String>,
        nonce: Option<String>,
    }
}

#[derive(Debug, Clone)]
pub struct MessageIntegrity {
    pub hmac: [u8; 20],
}

#[derive(Debug, Clone)]
pub struct Fingerprint {
    pub crc32: u32,
}

#[derive(Debug, Clone)]
pub struct ParsedRequestContext {
    pub source_addr: SocketAddr,
    pub stun_message: StunMessage,
    pub username: Option<String>,
    pub us_authenticationted: bool,
    pub integrity_valid: bool,
}

pub struct StunServerConfig {
    pub listen_ip: String,
    pub port: u16,
    pub users: HashMap<String, String>,
    pub realm: Option<String>,
    pub enable_auth: bool,
    pub enable_tls: bool,
}

pub struct StunStats {
    pub total_requests: AtomicU64,
    pub successful_responses: AtomicU64,
    pub auth_failures: AtomicU64,
    pub malformed_packets: AtomicU64,
}

impl StunStats {
    pub fn new() -> Self {
        Self {
            total_requests: AtomicU64::new(0),
            successful_responses: AtomicU64::new(0),
            auth_failures: AtomicU64::new(0),
            malformed_packets: AtomicU64::new(0),
        }
    }
}

pub struct MessageParams {
   pub ATTR_Type: u16,
   pub USERNAME: String,
   pub MESSAGE_INTEGRITY: String,
   pub SOFTWARE: String,
   pub REALM: String,
   pub NONCE: String,
   pub FINGERPRINT: String,
   pub MESS_TYPE: StunMessageTypes,
}

