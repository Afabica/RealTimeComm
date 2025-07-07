use std::{
    collections::HashMap,
    net::{SocketAddr, UdpSocket},
    sync::{Arc, Mutex},
    thread,
};
use std::time::{Duration, Instant};
use crate::components::entities::*;

pub fn simple_nat(nat_params: NatParams) -> std::io::Result<()> {
    let nat_table: Arc<Mutex<HashMap<SocketAddr, u16>>> = Arc::new(Mutex::new(HashMap::new()));
    let next_public_port = Arc::new(Mutex::new(nat_params.nat_port + 1));

    let socket = UdpSocket::bind(&nat_params.nat_addr)?;
    println!("NAT simulator listening on {}", &nat_params.nat_addr);

    let socket = Arc::new(socket);

    loop {
        let mut buf = [0u8; 1500];
        let (amt, src_addr) = socket.recv_from(&mut buf)?;

        let data = buf[..amt].to_vec();

        println!(
            "NAT received {} bytes from {}: {:02x?}",
            amt, src_addr, &data
        );

        let socket = Arc::clone(&socket);
        let nat_table = Arc::clone(&nat_table);
        let next_public_port = Arc::clone(&next_public_port);
        let nat_params = nat_params.clone();

        thread::spawn(move || {
            let is_response_from_stun = 
                src_addr.ip().to_string() == nat_params.nat_ip.to_string() &&
                src_addr.port() == nat_params.nat_port;

            if is_response_from_stun {
                let nat_table = nat_table.lock().unwrap();

                let private_addr = nat_table
                    .iter()
                    .find_map(|(priv_addr, &pub_port)| {
                        if src_addr.port() == pub_port {
                            Some(*priv_addr)
                        } else {
                            None
                        }
                    });

                match private_addr {
                    Some(priv_addr) => {
                        println!("NAT forwarding response to private client {}", priv_addr);
                        let _ = socket.send_to(&data, priv_addr);
                    }
                    None => {
                        println!("NAT has no mapping for this STUN server response");
                    }
                }
            } else {
                let mut nat_table = nat_table.lock().unwrap();
                let mut next_port = next_public_port.lock().unwrap();

                if !nat_table.contains_key(&src_addr) {
                    nat_table.insert(src_addr, *next_port);
                    println!(
                        "Creating NAT mapping {} -> {}:{}",
                        src_addr, nat_params.nat_ip, *next_port
                    );
                    *next_port += 1;
                }

                let public_port = nat_table[&src_addr];

                let bind_addr = format!("{}:{}", nat_params.nat_ip, public_port);

                match UdpSocket::bind(&bind_addr) {
                    Ok(nat_socket) => {
                        println!(
                            "NAT forwarding from private {} to STUN server from public {}",
                            src_addr, bind_addr
                        );
                        let _ = nat_socket.send_to(&data, nat_params.nat_addr);
                    }
                    Err(e) => {
                        eprintln!("Failed to bind socket on {}: {}", bind_addr, e);
                    }
                }
            }
        });
    }
}


pub fn define_address_range() -> Result {
    
}



