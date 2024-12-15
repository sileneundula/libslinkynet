use libp2p::request_response::{ProtocolName, RequestResponseCodec, RequestResponseEvent};

#[derive(Clone)]
struct MyProtocol;

impl ProtocolName for MyProtocol {
    fn protocol_name(&self) -> &[u8] {
        PROTOCOL_NAME
    }
}