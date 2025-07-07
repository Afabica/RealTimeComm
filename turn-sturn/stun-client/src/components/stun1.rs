use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr};
use std::collections::HashMap;
use std::sync::atomic::AtomicU64;
use std::io::{Cursor, Write};
use byteorder::{BigEndian, WriteBytesExt, ReadBytesExt};

#[derive(Debug, Clone)]
pub struct StunHeader {
    pub message_type: u16,
    pub message_length: u16,
    pub magic_cookie: u32,
    pub transaction_id: [u8; 12],
}

impl StunHeader {
    pub fn parse(buf: &[u8]) -> Option<Self> {
        if buf.len() < 20 { return None; }
        Some(Self {
            message_type: u16::from_be_bytes([buf[0], buf[1]]),
            message_length: u16::from_be_bytes([buf[2], buf[3]]),
            magic_cookie: u32::from_be_bytes([buf[4], buf[5], buf[6], buf[7]]),
            transaction_id: buf[8..20].try_into().ok()?,
        })
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(4 + self.message_length as usize + 4);
        buf.extend(&self.message_type.to_be_bytes());
        buf.extend(&self.message_length.to_be_bytes());
        buf.extend(&self.magic_cookie.to_be_bytes());
        buf.extend(&self.transaction_id.to_vec());
        buf
    }
}

#[derive(Debug, Clone)]
pub struct StunAttribute {
    pub attr_type: u16,
    pub length: u16,
    pub value: Vec<u8>,
}

impl StunAttribute {
    pub fn parse(bytes: &[u8]) -> Result<(Self, usize), String> {
        if bytes.len() < 4 {
            return Err("Not enough bytes for attribute header".to_string());
        }

        let attr_type = u16::from_be_bytes([bytes[0], bytes[1]]);
        let length = u16::from_be_bytes([bytes[2], bytes[3]]) as usize;

        if bytes.len() < 4 + length {
            return Err("Not enough bytes for attribute value".to_string());
        }

        let value = bytes[4..4 + length].to_vec();

        // Calculate padding to 4-byte boundary
        let padding = (4 - (length % 4)) % 4;
        let total_len = 4 + length + padding;

        if bytes.len() < total_len {
            return Err("Not enough bytes for padding".to_string());
        }

        Ok((
            StunAttribute {
                attr_type,
                length: length as u16,
                value,
            },
            total_len,
        ))
    }

    /// Serialize attribute to bytes (with padding)
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(4 + self.length as usize + 4); // extra padding space
        buf.extend(&self.attr_type.to_be_bytes());
        buf.extend(&self.length.to_be_bytes());
        buf.extend(&self.value);

        // Pad to 4-byte boundary
        let pad = (4 - (self.length as usize % 4)) % 4;
        buf.extend(vec![0u8; pad]);

        buf
    }
}

#[derive(Debug, Clone)]
pub struct StunMessage {
    pub header: StunHeader,
    pub attributes: Vec<StunAttribute>,
    pub raw: Vec<u8>,
}

impl StunMessage {
    /// Serialize to bytes
    pub fn to_bytes(&mut self) -> &[u8] {
        let mut buf = Cursor::new(Vec::new());

        // Write header (message length will be fixed later)
        buf.write_u16::<BigEndian>(self.header.message_type).unwrap();
        buf.write_u16::<BigEndian>(0).unwrap(); // placeholder for message length
        buf.write_u32::<BigEndian>(self.header.magic_cookie).unwrap();
        buf.write_all(&self.header.transaction_id).unwrap();

        // Write attributes
        for attr in &self.attributes {
            buf.write_all(&attr.to_bytes()).unwrap();
        }

        let mut final_buf = buf.into_inner();
        // Calculate attribute length (excluding 20-byte header)
        let attr_len = (final_buf.len() - 20) as u16;

        // Insert actual message length into header bytes [2..4]
        final_buf[2..4].copy_from_slice(&attr_len.to_be_bytes());

        // Update header length field
        self.header.message_length = attr_len;
        self.raw = final_buf;
        &self.raw
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, String> {
        if bytes.len() < 20 {
            return Err("Invalid STUN message: too short".to_string());
        }

        let header = StunHeader::parse(&bytes[0..20])
            .ok_or_else(|| "Failed to parse STUN header".to_string())?;

        let mut attributes = Vec::new();
        let mut offset = 20;
        while offset < bytes.len() {
            let (attr, consumed) = StunAttribute::parse(&bytes[offset..])?;
            attributes.push(attr);
            offset += consumed;
        }

        Ok(Self {
            header,
            attributes,
            raw: bytes.to_vec(),
        })
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
    },
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
    },
}

#[derive(Debug, Clone)]
pub struct MessageIntegrity {
    pub hmac: [u8; 20],
}

#[derive(Debug, Clone)]
pub struct Fingerprint {
    pub crc32: u32,
}

#[derive(Debug)]
pub struct ParsedRequestContext {
    pub source_addr: SocketAddr,
    pub stun_message: StunMessage,
    pub username: Option<String>,
    pub us_authenticated: bool,
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

