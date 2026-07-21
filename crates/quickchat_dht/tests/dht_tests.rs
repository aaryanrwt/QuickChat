use quickchat_dht::DhtNode;
use tokio::runtime::Runtime;

#[test]
fn test_dht_node_startup() {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        // Instantiate a node
        let mut node = DhtNode::new().expect("Failed to initialize DhtNode");

        // Listen on random port
        assert!(node.start_listening().is_ok());
    });
}
