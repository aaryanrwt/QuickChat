use libp2p::futures::StreamExt;
use quickchat_dht::DhtNode;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Starting QuickChat Community Relay Node (V4)...");

    // Initialize the DHT Node for the relay (with relay behavior enabled)
    let mut dht_node = DhtNode::new()?;
    dht_node.start_listening()?;

    println!("QuickChat Relay is running and listening on the DHT network.");
    println!("Node PeerID: {}", dht_node.swarm.local_peer_id());

    // Keep the relay alive and process network events
    loop {
        tokio::select! {
            event = dht_node.swarm.select_next_some() => {
                match event {
                    libp2p::swarm::SwarmEvent::NewListenAddr { address, .. } => {
                        println!("Relay listening on: {}", address);
                    }
                    _ => {}
                }
            }
        }
    }
}
