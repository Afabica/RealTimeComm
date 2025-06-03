use std::net::UdpSocket;
mod components;
use components::functionality::start_udp_listener;
use components::functionality::parse_stun_attributes;

fn main() -> std::io::Result<()> {
    start_udp_listener()?;
    Ok(())
}
//
//

