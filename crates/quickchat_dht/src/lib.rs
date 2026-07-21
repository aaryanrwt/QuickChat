use libp2p::{
    kad, mdns, noise, swarm::NetworkBehaviour, tcp, yamux, Swarm, SwarmBuilder,
};
use std::error::Error;
use std::time::Duration;

#[derive(NetworkBehaviour)]
pub struct QuickChatBehavior {
    pub kademlia: kad::Behaviour<kad::store::MemoryStore>,
    pub mdns: mdns::tokio::Behaviour,
}

pub struct DhtNode {
    pub swarm: Swarm<QuickChatBehavior>,
}

impl DhtNode {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let local_key = libp2p::identity::Keypair::generate_ed25519();
        let local_peer_id = libp2p::PeerId::from(local_key.public());

        let mut cfg = kad::Config::default();
        cfg.set_query_timeout(Duration::from_secs(5 * 60));
        let store = kad::store::MemoryStore::new(local_peer_id);
        let kademlia = kad::Behaviour::with_config(local_peer_id, store, cfg);

        let mdns = mdns::tokio::Behaviour::new(mdns::Config::default(), local_peer_id)?;

        let behavior = QuickChatBehavior { kademlia, mdns };

        let swarm = SwarmBuilder::with_existing_identity(local_key)
            .with_tokio()
            .with_tcp(
                tcp::Config::default(),
                noise::Config::new,
                yamux::Config::default,
            )?
            .with_behaviour(|_| behavior)?
            .with_swarm_config(|c| c.with_idle_connection_timeout(Duration::from_secs(60)))
            .build();

        Ok(Self { swarm })
    }

    pub fn start_listening(&mut self) -> Result<(), Box<dyn Error>> {
        self.swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;
        Ok(())
    }
}
