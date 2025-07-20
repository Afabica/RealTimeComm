use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::time::{Duration, Instant};

pub struct NatMapping {
    internal_addr: SocketAddr,
    external_addr: SocketAddr,
    last_active: Instant,
}


#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct InternalEndpoint {
    ip: Ipv4Addr,
    port: u16,
}

#[derive(Debug, Clone)]
struct ExternalEndpoint {
    ip: Ipv4Addr,
    port: u16,
}

pub struct NatTable {
    mappings: HashMap<InternalEndpoint, ExternalEndpoint>,
    reverse_mappings: HashMap<ExternalEndpoint, InternalEndpoint>,
}

impl NatTable {
fn new() -> Self {
        NatTable {
            mappings: HashMap::new(),
            reverse_mappings: HashMap::new(),
        }
    }
    
    fn add_mapping(&mut self, internal: InternalEndpoint, external: ExternalEndpoint) {
        self.mappings.insert(internal.clone(), external.clone());
        self.reverse_mappings.insert(external,internal);
    }

    fn get_external(&self, internal: &InternalEndpoint) -> Option<&ExternalEndpoint> {
        self.mappings.get(internal)
    }

    fn get_internal(&self, external: &ExternalExnpoint) -> Option<&InternalEndpoint> {
        self.reverse_mappings.get(external)
    }
}
