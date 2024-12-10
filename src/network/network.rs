use libp2p::relay;
use libp2p::*;
use libp2p::swarm::*;

use crate::network::peer_discovery::kad::SlinkyKAD;

/// # SlinkyL1Behaviour
/// 
/// The SlinkyL1Behaviour defines the protocols and behaviour used on the swarm.
/// 
/// These include:
/// 
/// ## Basics
/// 
/// 1. Relay using WSS
/// 2. Ping using WSS
/// 3. SlinkyDiscovery using KAD
/// 4. Identify To KAD
/// 
/// ## Advanced
/// 
/// 1. SlinkyBridge
#[derive(NetworkBehaviour)]
pub struct SlinkyL1Behaviour {
    relay: relay::Behaviour,
    ping: ping::Behaviour,
    discovery: kad::Behaviour<MemoryStore>, // KAD-CORE 
    identify: identify::Behaviour,
    // Other behaviours like discovery
}

impl SlinkyL1Behaviour {
    fn new() -> Self {
        Self {
            relay: relay::Behaviour::new(),
            ping: ping::Behaviour::new(),
        }
    }
}
