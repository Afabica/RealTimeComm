use std::{net::{Ipv4Addr, SocketAddr}};
 
#[derive(Clone, Debug)]
pub struct NatParams {
    pub nat_ip: Ipv4Addr,
    pub nat_port: u16,
    pub nat_addr: String,
    pub stun_ip: Ipv4Addr,
    pub stun_port: u16,
    pub stun_addr: SocketAddr,
}

impl NatParams {
    pub fn new(nat_ip: Ipv4Addr, nat_port: u16, nat_addr: String, stun_ip: Ipv4Addr,
stun_port: u16, stun_addr: SocketAddr) -> Self {
        Self {
            nat_ip: nat_ip,
            nat_port: nat_port,
            nat_addr: nat_addr,
            stun_ip: stun_ip,
            stun_port: stun_port,
            stun_addr: stun_addr,
        }
    }
}

