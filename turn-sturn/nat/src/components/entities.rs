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


#[derive(Clone, Debug)]
pub enum NATParam {
    FullCone,
    RestictedCone,
    PortRestrictedCone,
    Symmetric,
}

#[derive(Debug, Clone Copy, PartialEq, Eq)]
pub enum FilteringBehaviour {
    EndpointIndependent,
    AddressDependent,
    AddressAndPortDependent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NatCapabilities {
    pub support_hairpinning: bool,
    pub preserves_port: bool,
    pub is_cgnat: bool
}

#[derive(Debug, Clone, Copy)]
pub struct NatTimeouts {
    pub udp_timeout_secs: u64,
    pub tcp_timeout_secs: u64,
}

#[derive(Debug, Clone)]
pub struct NatProfile {
    pub mapping_behaviour: MappingBehavior,
    pub filtering_behaviour: FilteringBehaviour,
    pub timeoouts: NatTimeouts,
    pub capabilities: NatCapabilities,
}
