use libp2p::relay;
use libp2p::*;

#[derive(libp2p::NetworkBehaviour)]
struct MyBehaviour {
    relay: relay::Behaviour,
    discovery: 
    // Other behaviours like discovery
}

impl MyBehaviour {
    fn new() -> Self {
        Self {
            relay: relay::Behaviour::new(),
        }
    }
}
