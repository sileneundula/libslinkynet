use libp2p::{PeerId, Swarm};
use crate::network::transport::*;
use crate::network::network::SlinkyL1Behaviour;

pub fn create_swarm(transport: libp2p::core::transport::Boxed<(PeerId, libp2p::core::muxing::StreamMuxerBox)>) -> Swarm<()> {
    // Generate a random PeerId
    let local_key = libp2p::identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());

    println!("Local peer id: {:?}", local_peer_id);

    // Create a basic Swarm
    let swarm = Swarm::new(transport, SlinkyL1Behaviour, local_peer_id);
    return swarm
}