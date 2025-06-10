use std::net::{UdpSocket, SocketAddr, IpAddr, Ipv4Addr, Ipv6Addr, SocketAddrV4};

use rand::Rng;
use byteorder::{BigEndian, WriteBytesExt};
use crate::components::stun1::{StunMessage, StunHeader, StunAttribute, XorMappedAddress};
use crate::components::attributes::{StunMessageTypes, StunAttributeType};
use crate::components::stun1::ResponseHandling;


pub fn start_udp_listener() -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:8080")?;
    println!("Listening on 127.0.0.1:8080...");
    let mut buf = [0u8; 1024];

    loop {
        let (size, src) = socket.recv_from(&mut buf)?;

        let server: SocketAddr = src.to_string().parse().expect("Unable to parse socket address.");
        let src_str = src.to_string();
        let client_ip: Ipv4Addr = Ipv4Addr::parse_ascii(src_str)?;
        //        let client_ip: IpAddr = src.to_string().parse().unwrap();
//        let client_ip: Ipv4Addr = src.to_string().parse().expect("Unable to parse ip address");
        println!("\nReceived {} bytes from port {}, ip {}", size, server, client_ip);

        if size < 20 {
            println!("Received too few bytes for STUN header");
            continue;
        }

        // Parse header
        let header = match parse_stun_header(&buf[..20]) {
            Some(h) => h,
            None => {
                println!("Invalid or non-STUN packet received");
                continue;
            }
        };

        println!("Parsed STUN Header: {:?}", header);

        let expected_len = 20 + header.message_length as usize;
        if size < expected_len {
            println!("Packet does not contain full attribute data");
            continue;
        }

        // Parse attributes
        let data_attrs = parse_stun_attributes(&buf[20..expected_len]);

        if data_attrs.is_empty() {
            println!("No STUN attributes parsed."); 
            continue;
        }

        for (index, attr) in data_attrs.iter().enumerate() {
            println!("Attr {}: {:?}", index, attr);
        } 
    }
}

pub fn receive_packet(socket: &UdpSocket) -> Option<(Vec<u8>, SocketAddr)> {
    let mut buf = [0u8; 1500];
    match socket.recv_from(&mut buf) {
        Ok((size, addr)) => Some((buf[..size].to_vec(), addr)),
        Err(_) => None,
    }
}

pub fn build_xor_mapped_address(client_addr: SocketAddr, transaction_id: &[u8; 12]) -> Vec<u8> {
    let mut buf = Vec::new();

    match client_addr.ip() {
        IpAddr::V4(ipv4) => {
            // Address family: 0x01 for IPv4
            buf.push(0); // First byte zero per RFC
            buf.push(0x01);

            // XOR port: port XORed with upper 16 bits of Magic Cookie
            let port = client_addr.port();
            let x_port = port ^ ((0x2112A442u32 >> 16) as u16);
            buf.write_u16::<BigEndian>(x_port).unwrap();

            // XOR IPv4 address bytes
            let ip_bytes = ipv4.octets();
            let magic_cookie_bytes = 0x2112A442u32.to_be_bytes();
            for i in 0..4 {
                buf.push(ip_bytes[i] ^ magic_cookie_bytes[i]);
            }
        }
        IpAddr::V6(ipv6) => {
            // Address family: 0x02 for IPv6
            buf.push(0);
            buf.push(0x02);

            let port = client_addr.port();
            let x_port = port ^ ((0x2112A442u32 >> 16) as u16);
            buf.write_u16::<BigEndian>(x_port).unwrap();

            // XOR IPv6 address with Magic Cookie + Transaction ID
            let ip_bytes = ipv6.octets();
            let mut xor_mask = [0u8; 16];
            xor_mask[..4].copy_from_slice(&0x2112A442u32.to_be_bytes());
            xor_mask[4..].copy_from_slice(transaction_id);

            for i in 0..16 {
                buf.push(ip_bytes[i] ^ xor_mask[i]);
            }
        }
    }

    buf
}

pub fn build_stun_packet(msg_type: StunHeader, attributes: Vec<StunAttribute>) -> Vec<u8> {
    let mut buf: Vec<u8> = Vec::new();

    buf.write_u16::<BigEndian>(msg_type.message_type).unwrap();
    buf.write_u16::<BigEndian>(0).unwrap(); // placeholder for length
    buf.write_u32::<BigEndian>(0x2112A442).unwrap();

    let mut transaction_id = [0u8; 12];
    rand::thread_rng().fill(&mut transaction_id);
    buf.extend_from_slice(&transaction_id);

    let mut attr_bytes = Vec::new();
    for attr in attributes {
        attr_bytes.write_u16::<BigEndian>(attr.attr_type).unwrap();
        attr_bytes.write_u16::<BigEndian>(attr.value.len() as u16).unwrap();
        attr_bytes.extend_from_slice(&attr.value);

        // padding to 4-byte boundary
        let pad = (4 - (attr.value.len() % 4)) % 4;
        attr_bytes.extend(std::iter::repeat(0).take(pad));
    }

    let msg_len = attr_bytes.len() as u16;
    buf[2..4].copy_from_slice(&msg_len.to_be_bytes());

    buf.extend(attr_bytes);
    buf
}

pub fn constructing_response(header: &StunHeader, client_addr: SocketAddr) -> StunMessage { 
    // Build XOR-MAPPED-ADDRESS attribute
    let xor_mapped_value = build_xor_mapped_address(client_addr, &header.transaction_id);

    let attr = StunAttribute {
        attr_type: 0x0020, // XOR-MAPPED-ADDRESS type
        length: xor_mapped_value.len() as u16,
        value: xor_mapped_value,
    };

    StunMessage {
        header: StunHeader {
            message_type: 0x0101, // Binding Success Response
            message_length: 0, // will be recalculated in to_bytes()
            magic_cookie: 0x2112A442,
            transaction_id: header.transaction_id,
        },
        attributes: vec![attr],
        raw: Vec::new(),
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
        message_type: message_type,
        message_length: message_length,
        magic_cookie: magic_cookie,
        transaction_id: transaction_id,
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
            break; // malformed attribute, exit parsing
        }

        let attr_value = &buf[pos..pos + attr_len];
        pos += attr_len;

        // skip padding bytes
        let padding = (4 - (attr_len % 4)) % 4;
        pos += padding;

        attrs.push(StunAttribute {
            attr_type: attr_type,
            length: attr_len as u16,
            value: attr_value.to_vec(),
        });
    }

    attrs
}

pub fn parse_xor_mapped_address(attr: &StunAttribute) -> Option<SocketAddr> {
    if attr.value.len() < 8 {
        return None;
    }

    let family = attr.value[1];
    let port_xored = u16::from_be_bytes([attr.value[2], attr.value[3]])
        ^ ((0x2112A442u32 >> 16) as u16);

    match family {
        0x01 => {
            if attr.value.len() < 8 {
                return None;
            }
            let ip_bytes: Vec<u8> = attr.value[4..8]
                .iter()
                .zip(&0x2112A442u32.to_be_bytes())
                .map(|(b, m)| b ^ m)
                .collect();

            let ip = Ipv4Addr::new(ip_bytes[0], ip_bytes[1], ip_bytes[2], ip_bytes[3]);
            Some(SocketAddr::new(IpAddr::V4(ip), port_xored))
        }
        0x02 => {
            if attr.value.len() < 20 {
                return None;
            }
            let mut ip_bytes = [0u8; 16];
            for i in 0..16 {
                ip_bytes[i] = attr.value[4 + i]
                    ^ if i < 4 {
                        0x2112A442u32.to_be_bytes()[i]
                    } else {
                        attr.value[8 + i - 4]
                    };
            }
            let ip = Ipv6Addr::from(ip_bytes);
            Some(SocketAddr::new(IpAddr::V6(ip), port_xored))
        }
        _ => None,
    }
}

