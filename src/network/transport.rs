use libp2p::{core, identity, noise, tcp, websocket, yamux, PeerId, Transport};


pub struct SlinkyConnection(core::transport::Boxed<(PeerId,core::muxing::StreamMuxerBox)>);
pub struct SlinkyConnectionTCP(core::transport::Boxed<(PeerId, core::muxing::StreamMuxerBox)>);
pub struct SlinkyConnectionWSS(core::transport::Boxed<(PeerId,core::muxing::StreamMuxerBox)>);


pub fn create_secure_transport_tcp(keypair: identity::Keypair) -> core::transport::Boxed<(PeerId, core::muxing::StreamMuxerBox)> {
    let auth_config = noise::Config::new(&keypair).unwrap();
    let transport: core::transport::Boxed<(PeerId, core::muxing::StreamMuxerBox)> = tcp::tokio::Transport::default()
        .upgrade(core::upgrade::Version::V1)  // Use protocol upgrade version 1
        .authenticate(auth_config)    // Add Noise encryption
        .multiplex(yamux::Config::default()) // Multiplexing
        .boxed();

    return transport
}

pub fn create_secure_transport_wss(keypair: identity::Keypair) -> core::transport::Boxed<(PeerId, core::muxing::StreamMuxerBox)> {
    // Noise Config
    let auth_config = noise::Config::new(&keypair).unwrap();

    // WebSocket Secure Protocol
    let transport = websocket::WsConfig::new(tcp::tokio::Transport::default())
        .upgrade(core::upgrade::Version::V1)
        .authenticate(auth_config)
        .multiplex(yamux::Config::default())
        .boxed();

    return transport
}

