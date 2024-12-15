use libp2p::swarm::Config;
use libp2p::{PeerId, Swarm};
use crate::network::transport::*;
use crate::network::network::SlinkyL1Behaviour;

pub fn create_swarm(transport: SlinkyConnectionTCP) -> Swarm<SlinkyL1Behaviour> {
    // Generate a random PeerId
    let local_key: libp2p::identity::Keypair = libp2p::identity::Keypair::generate_ed25519();
    let local_peer_id: PeerId = PeerId::from(local_key.public());

    println!("Local peer id: {:?}", local_peer_id);

    // Create a basic Swarm
    let swarm = Swarm::new(transport.get_transport(), SlinkyL1Behaviour::new(local_key), local_peer_id, Config::with_tokio_executor());
    return swarm
}