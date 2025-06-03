use std::net::{UdpSocket, SocketAddr, IpAddr, Ipv4Addr, Ipv6Addr};
use rand::Rng;
use byteorder::{BigEndian, WriteBytesExt};
use crate::components::stun1::{StunHeader, StunAttribute, XorMappedAddress};


pub fn start_udp_listener() -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:8080")?;
    println!("Listening on 127.0.0.1:8080...");
    let mut buf = [0u8; 1024];

    loop {
        let (amt, src) = socket.recv_from(&mut buf)?;

        let received: &str = std::str::from_utf8(&buf[..amt]).unwrap_or("<Invalid UTF-8>"); 
        println!("Received from {}: {}", src, received);
        let u8_val: u8 = u8::from_str_radix(received, 16).expect("Invalid hex string");
        let data_slice: &[u8] = &[u8_val];
        let parseddata = parse_stun_attributes(data_slice);
        println!("Received from {}: {:?}", src, &parseddata);
        let response = b"ACK";
        socket.send_to(response,src)?;
    }
}

pub fn receive_packet(socket: &UdpSocket) -> Option<(Vec<u8>, std::net::SocketAddr)> {
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
            buf.push(0); // First byte is 0 per RFC
            buf.push(0x01);

            // XOR port: port XORed with the upper 16 bits of Magic Cookie
            let port = client_addr.port();
            let x_port = port ^ ((0x2112A442u32 >> 16) as u16);
            buf.write_u16::<BigEndian>(x_port).unwrap();

            // XOR IPv4 address: each byte XORed with Magic Cookie bytes
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

            // XOR port
            let port = client_addr.port();
            let x_port = port ^ ((0x2112A442u32 >> 16) as u16);
            buf.write_u16::<BigEndian>(x_port).unwrap();

            // XOR IPv6 address: each byte XORed with Magic Cookie + Transaction ID bytes
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



pub fn generate_transaction_id() -> [u8; 12] {
    let mut rng = rand::thread_rng();
    let mut id = [0u8; 12];
    rng.fill(&mut id);
    id
}

pub fn build_stun_packet(msg_type: StunHeader, attribute: Vec<StunAttribute>) -> Vec<u8> {
    let mut buf: Vec<u8> = Vec::new();

    buf.write_u16::<BigEndian>(msg_type.MessageType as u16).unwrap();
    buf.write_u16::<BigEndian>(0).unwrap();
    buf.write_u32::<BigEndian>(0x2112A442).unwrap();

    let mut transaction_id = [0u8; 12];
    rand::thread_rng().fill(&mut transaction_id);
    buf.extend_from_slice(&transaction_id);

    let mut attr_bytes = Vec::new();
    for attr in attribute {
        attr_bytes.write_u16::<BigEndian>(attr.ATTR_Type).unwrap();
        attr_bytes
            .write_u16::<BigEndian>(attr.Value.len() as u16)
            .unwrap();
        attr_bytes.extend_from_slice(&attr.Value);

        let pad = (4 - (attr.Value.len() % 4)) % 4;
        attr_bytes.extend(std::iter::repeat(0).take(pad));
    }

    let msg_len = attr_bytes.len() as u16;
    buf[2..4].copy_from_slice(&msg_len.to_be_bytes());

    buf.extend(attr_bytes);
    buf
}

pub fn parse_stun_attributes(buf: &[u8]) -> Result<Vec<StunAttribute>, String> {
    let mut attrs = Vec::new();
    let mut pos = 0;

    while pos + 4 <= buf.len() {
        let attr_type = u16::from_be_bytes([buf[pos], buf[pos + 1]]);
        let attr_len = u16::from_be_bytes([buf[pos + 2], buf[pos + 3]]) as usize;
        pos += 4;

        if pos + attr_len > buf.len() {
            return Err("Attribute length exceeds buffer".to_string());
        }

        let attr_value = &buf[pos..pos + attr_len];
        pos += attr_len;

        // Move pos to next multiple of 4 (padding)
        let padding = (4 - (attr_len % 4)) % 4;
        pos += padding;

        attrs.push(StunAttribute {
            ATTR_Type: attr_type,
            Value: attr_value.to_vec(),
            Length: attr_len as u16,
        });
    }

    Ok(attrs)
}

