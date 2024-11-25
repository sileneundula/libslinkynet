use libp2p::relay;

#[derive(libp2p::NetworkBehaviour)]
struct MyBehaviour {
    relay: relay::Behaviour,
    // Other behaviours like discovery
}

impl MyBehaviour {
    fn new() -> Self {
        Self {
            relay: relay::Behaviour::new(),
        }
    }
}
