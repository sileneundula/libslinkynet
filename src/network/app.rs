use crate::network::network::SlinkyL1Behaviour;
use crate::network::swarm::*;
use crate::network::transport::SlinkyConnectionTCP;
use crate::network::swarm::SlinkyL1Swarm;
use futures::StreamExt;
use libp2p::identity;
use libp2p::PeerId;
use libp2p::Swarm;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    


    let local_key = identity::Keypair::generate_ed25519();
    let peer_id = PeerId::from_public_key(&local_key.public());

    println!("Peer ID: {:?}", peer_id);

    // Create the swarm
    let mut swarm = SlinkyL1Swarm::new(local_key);

    // Listen on a multiaddress
    let listen_addr = "/ip4/0.0.0.0/tcp/0".parse()?;
    Swarm::listen_on(&mut swarm, listen_addr)?;

    // Event loop
    loop {
        tokio::select! {
            event = swarm.next() => {
                if let Some(event) = event {
                    println!("Swarm event: {:?}", event);
                }
            }
        }
    }
}