use  libp2p:{identity,PeerOd,Swarm,mdns,noise,tcp,Transport};

pub fn getp2pConnection() { 
    let keypair = identity::Keypair::generate_ed25519();
    let peer_id = PeerId::from(keypair.public());
    println!("Node ID: {:?}",peer_id);
}
