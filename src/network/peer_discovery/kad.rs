use libp2p::kad::*;
use libp2p::identity;

pub struct SlinkyKAD;

impl SlinkyKAD {
    pub fn new(local_key: identity::Keypair) -> libp2p::kad::Behaviour<_> {
        let local_peer_id = identity::PeerId::from(local_key.public());
        
        let store = store::MemoryStore::new(local_peer_id);
        
        let mut kad: Behaviour<_> = Config::with_config(
            local_peer_id,
            store,
            Config::default(),
        );

        return kad
    }

}