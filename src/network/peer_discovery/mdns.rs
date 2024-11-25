use libp2p::mdns;

pub fn create_discovery_mdns() -> mdns::tokio::Behaviour {
    mdns::tokio::Behaviour::new(mdns::Config::default(), Default::default()).unwrap()
}