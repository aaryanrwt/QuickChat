use quickchat_dht::DhtNode;
use std::error::Error;
use tokio::time::{Duration, sleep};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Starting QuickChat Relay Server (V3)...");

    // Initialize the DHT Node for the relay
    let mut dht_node = DhtNode::new()?;
    dht_node.start_listening()?;

    println!("QuickChat Relay is running and listening on the DHT network.");
    println!("Node PeerID: {}", dht_node.swarm.local_peer_id());

    // Keep the relay alive indefinitely
    loop {
        sleep(Duration::from_secs(60)).await;
        println!("Relay node heartbeat...");
    }
}
