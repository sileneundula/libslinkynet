use libp2p::relay;
use libp2p::*;
use libp2p::swarm::*;
use libp2p::kad::store::MemoryStore;
use libp2p::identity;
use libp2p::identity::PeerId;
use libp2p::floodsub::*;

pub struct SlinkyL1Topic(floodsub::Topic);

pub enum SlinkyL1Topics {
    SlinkyL1Alpha,
}

impl SlinkyL1Topic {
    pub fn new(s: SlinkyL1Topics) -> Self {
        let topic = match s {
            SlinkyL1Topics::SlinkyL1Alpha => Topic::new("SLINKYL1ALPHA")
        };
        return Self(topic)
    }
    pub fn get_topic(self) -> Topic {
        return self.0
    }
}

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
/// 2. 
#[derive(NetworkBehaviour)]
pub struct SlinkyL1Behaviour {
    // Basics
    pub relay: relay::Behaviour,
    pub ping: ping::Behaviour,
    pub discovery: kad::Behaviour<MemoryStore>,
    pub identify: identify::Behaviour,

    pub floodsub: floodsub::Floodsub,
    // # SlinkyL1
    // Lists the core functionalities
    

    // Serve
    
    // Repositories

}

impl SlinkyL1Behaviour {
    pub fn new(kp: identity::Keypair) -> Self {
        
        let pk: identity::PublicKey = kp.public();
        let id: PeerId = PeerId::from(pk);

        
        Self {
            relay: relay::Behaviour::new(id,relay::Config::default()),
            ping: ping::Behaviour::new(ping::Config::default()),
            discovery: kad::Behaviour::new(id,kad::store::MemoryStore::new(id)),
            identify: identify::Behaviour::new(identify::Config::new(String::from("SlinkyL1Alpha"),kp.public())),
            
            floodsub: Floodsub::new(id),
        }
    }
}
