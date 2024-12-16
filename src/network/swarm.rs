use libp2p::swarm::Config;
use libp2p::{PeerId, Swarm};
use crate::network::transport::*;
use crate::network::network::SlinkyL1Behaviour;
use libp2p::identity;


/// # SlinkyL1Swarm
/// 
/// This swarm includes creating the basic SlinkyL1Swarm
pub struct SlinkyL1Swarm;

impl SlinkyL1Swarm {
    pub fn new(keypair: identity::Keypair) -> Swarm<SlinkyL1Behaviour> {
        // Transport TCP
        let transport = SlinkyConnectionTCP::new(keypair);

        let pk = keypair.public();
        let peer_id = PeerId::from_public_key(&pk);


        let swarm = Swarm::new(transport.get_transport(),SlinkyL1Behaviour::new(keypair),PeerId::from_public_key(&pk), Config::with_tokio_executor());

        return swarm

    }
}




pub fn create_swarm(transport: SlinkyConnectionTCP) -> Swarm<SlinkyL1Behaviour> {
    // Generate a random PeerId
    let local_key: libp2p::identity::Keypair = libp2p::identity::Keypair::generate_ed25519();
    let local_peer_id: PeerId = PeerId::from(local_key.public());

    println!("Local peer id: {:?}", local_peer_id);

    // Create a basic Swarm
    let swarm = Swarm::new(transport.get_transport(), SlinkyL1Behaviour::new(local_key), local_peer_id, Config::with_tokio_executor());
    return swarm
}