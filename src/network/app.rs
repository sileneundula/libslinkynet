#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the transport
    let transport = build_transport()?;

    // Create the swarm
    let mut swarm = create_swarm(transport);

    // Listen on a multiaddress
    let listen_addr = "/ip4/0.0.0.0/tcp/0".parse()?;
    Swarm::listen_on(&mut swarm, listen_addr)?;

    // Event loop
    loop {
        tokio::select! {
            event = swarm.next() => {
                if let Some(event) = event {
                    println!("Swarm event: {:?}", event);
                }
            }
        }
    }
}