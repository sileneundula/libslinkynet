use libp2p::core::upgrade::{ProtocolName, UpgradeInfo};
use libp2p::bytes::Bytes;


/// # Yack Protocol
/// 
/// A protocol for retrieving packages, extensions, scripts, and downloads.
#[derive(Debug, Clone)]
pub struct YackProtocol;

impl ProtocolName for YackProtocol {
    fn protocol_name(&self) -> &[u8] {
        b"/yack/1.0.0"
    }
}