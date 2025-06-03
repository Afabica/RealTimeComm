use std::{
    collections::HashMap,
    net::{Ipv4Addr, SocketAddr, UdpSocket},
    sync::{Arc, Mutex},
    thread,
};
mod components;
use components::simple_nat::simple_nat;
use components::entities::NatParams;


fn main() -> std::io::Result<()> {
    // NAT "public" IP and port (the external side)
    let nat_ip: Ipv4Addr = Ipv4Addr::new(127,0,0,1);
    let nat_port = 40000;
    let nat_addr = format!("{}:{}", nat_ip, nat_port);

    // STUN server IP and port (assumed running)
    let stun_ip: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
    let stun_port = 3478;
    let stun_addr: SocketAddr = format!("{}:{}", stun_ip, stun_port).parse().unwrap();
    let nat_params: NatParams = NatParams {
        nat_ip: nat_ip,
        nat_port: nat_port,
        nat_addr: nat_addr,
        stun_ip: stun_ip,
        stun_port: stun_port,
        stun_addr: stun_addr,
    };

    let _ = simple_nat(nat_params);

    Ok(())
}

