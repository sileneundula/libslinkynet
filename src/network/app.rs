use crate::network::network::SlinkyL1Behaviour;
use crate::network::swarm::*;
use crate::network::transport::SlinkyConnectionTCP;
use crate::network::swarm::SlinkyL1Swarm;
use futures::StreamExt;
use libp2p::identity;
use libp2p::PeerId;
use libp2p::Swarm;
use pretty_env_logger;
use log::*;
use tokio::io::AsyncBufReadExt;

use crate::network::events::EventType;

use crate::network::network::SlinkyL1Topic;
use crate::network::network::SlinkyL1Topics;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();


    println!("Initializing SlinkyL1");
    println!("TOPIC: {:?}",SlinkyL1Topic::new(SlinkyL1Topics::SlinkyL1Alpha).get_topic());
    println!("");
    println!("[INFO]Keypair-Algorithm: Ed25519");

    let local_key = identity::Keypair::generate_ed25519();
    let peer_id = PeerId::from_public_key(&local_key.public());

    println!("[INFO]Peer-ID: {}", &peer_id);

    println!("");

    println!("Starting Swarm with SlinkyL1Behaviour...");
    // Create the swarm
    let mut swarm = SlinkyL1Swarm::new(local_key);

    println!("Subscribing to Topic using FloodSub");
    
    // Behaviour For Floodsub Using Topic
    swarm.behaviour_mut().floodsub.subscribe(SlinkyL1Topic::new(SlinkyL1Topics::SlinkyL1Alpha).get_topic());



    println!("Listening");

    // Listen on a multiaddress
    let listen_addr = "/ip4/0.0.0.0/tcp/0".parse()?;


    let mut stdin = tokio::io::BufReader::new(tokio::io::stdin()).lines();


    Swarm::listen_on(&mut swarm, listen_addr)?;

    // Event loop
    loop {
        let slinkyevents = {
            tokio::select! {
                line = stdin.next_line() => Some(EventType::Input(line.expect("can get line").expect("can read line from stdin"))),
            
                // Events
                event = swarm.next() => {
                    println!("Swarm event: {:?}", event);
                    None
                }
            }
        };
    }
}