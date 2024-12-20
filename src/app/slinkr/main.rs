use crate::app::slinkr::slinkrdefs::definitions::SlinkrVersion;
use crate::internals::time::chrono::Utc;
use std::io;

pub struct InitialSetup;

impl InitialSetup {
    pub fn new() {

    }
    pub fn interactive() {
        println!("Initializing Slinkr Interactive Setup For SlinkyL1 Network.");
        println!("Slinkr Version: {}",SlinkrVersion::new(0u32).get_version());
        println!("");
        println!("# Timestamp");
        println!("Current UTC Timestamp: {}", Utc::now());
        println!("Current Confirmed Bitcoin Height (-8 Blocks): {}", String::from("None"));
        println!("Current Confirmed Ethereum Height: {}", String::from("None"));
        println!("");


        println!("Description: SlinkyL1 is an open, decentralized, peer-to-peer network useful for transmitting data and running certain protocols. It uses a Directed Acyclic Graph (DAG) for transactions and logic using CREEDNV. It uses a standardized language, known as SLI, for interacting with peers.");
        println!("Philosphy: Minimalistic with Extendability.");
        println!("");
        println!("");
        println!("Please Follow The Directions Carefully For Initialization.");
        println!("[y/n] Do you want to continue: ");
        
        let mut buffer = String::new();
        let stdin = io::stdin(); // We get `Stdin` here.
        stdin.read_line(&mut buffer).expect("Failed To Read Input");
    }
}