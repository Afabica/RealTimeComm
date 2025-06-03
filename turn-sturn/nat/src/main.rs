use std::{
    collections::HashMap,
    net::{SocketAddr, UdpSocket},
    sync::{Arc, Mutex},
    thread,
};

fn main() -> std::io::Result<()> {
    // NAT "public" IP and port (the external side)
    let nat_ip = "127.0.0.1";
    let nat_port = 40000;
    let nat_addr = format!("{}:{}", nat_ip, nat_port);

    // STUN server IP and port (assumed running)
    let stun_ip = "127.0.0.1";
    let stun_port = 3478;
    let stun_addr: SocketAddr = format!("{}:{}", stun_ip, stun_port).parse().unwrap();

    // Shared NAT mapping table: private_addr -> public_port
    let nat_table: Arc<Mutex<HashMap<SocketAddr, u16>>> = Arc::new(Mutex::new(HashMap::new()));
    let next_public_port = Arc::new(Mutex::new(nat_port + 1));

    // Bind UDP socket on NAT public IP/port (simulate public interface)
    let socket = UdpSocket::bind(&nat_addr)?;
    println!("NAT simulator listening on {}", nat_addr);

    let socket = Arc::new(socket);

    loop {
        let mut buf = [0u8; 1500];
    let (amt, src_addr) = socket.recv_from(&mut buf)?;

    // Copy the received data into a Vec<u8> so it can live inside the thread
    let data = buf[..amt].to_vec();

    println!(
        "NAT received {} bytes from {}: {:02x?}",
        amt, src_addr, &data
    );

    let socket = Arc::clone(&socket);
    let nat_table = Arc::clone(&nat_table);
    let next_public_port = Arc::clone(&next_public_port);
        // Spawn a thread to handle each packet so NAT can process multiple packets
        thread::spawn(move || {
            if src_addr.ip().to_string() == stun_ip && src_addr.port() == stun_port {
                // Packet from STUN server -> forward to private client by reverse mapping
                let nat_table = nat_table.lock().unwrap();

                // Find private address that maps to this port (public port in src_addr)
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
                // Packet from private client -> assign public port & forward to STUN server
                let mut nat_table = nat_table.lock().unwrap();
                let mut next_port = next_public_port.lock().unwrap();

                if !nat_table.contains_key(&src_addr) {
                    nat_table.insert(src_addr, *next_port);
                    println!("Creating NAT mapping {} -> {}:{}", src_addr, nat_ip, *next_port);
                    *next_port += 1;
                }

                let public_port = nat_table[&src_addr];
                drop(nat_table);
                drop(next_port);

                // To simulate NAT translation, create a new socket bound to public port
                let bind_addr = format!("{}:{}", nat_ip, public_port);
                match UdpSocket::bind(&bind_addr) {
                    Ok(nat_socket) => {
                        println!(
                            "NAT forwarding from private {} to STUN server from public {}",
                            src_addr, bind_addr
                        );
                        let _ = nat_socket.send_to(&data, stun_addr);
                        // No need to keep socket alive, it will close after scope
                    }
                    Err(e) => {
                        eprintln!("Failed to bind socket on {}: {}", bind_addr, e);
                    }
                }
            }
        });
    }
}

