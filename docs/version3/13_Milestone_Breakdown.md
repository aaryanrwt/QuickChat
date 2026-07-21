# 13: Milestone Breakdown

## V3 Engineering Milestones

To ensure steady progress and continuous integration, V3 development is divided into four strictly scoped milestones.

### Milestone 1: The Foundation (Networking & Crypto)
*Focus: Ripping out the V2 1-on-1 limitations and laying the groundwork for decentralized groups.*
- [ ] Implement the `quickchat_dht` crate (Kademlia routing).
- [ ] Integrate Messaging Layer Security (MLS) for group E2EE.
- [ ] Develop the `quickchat_relay` daemon.
- [ ] **Deliverable:** A headless CLI tool capable of sending encrypted messages to a group via the DHT and relay.

### Milestone 2: The Extensibility Layer (Plugins & APIs)
*Focus: Hardening the WASM host and expanding the developer API.*
- [ ] Finalize `PluginRuntime` WASI capabilities (network/fs sandboxing).
- [ ] Implement the IPC socket for editor integrations.
- [ ] Build the official "Live Code Pointers" plugin.
- [ ] **Deliverable:** A stable Plugin SDK and a working integration between QuickChat and a local Neovim/VSCode instance.

### Milestone 3: The User Experience (TUI & Persistence)
*Focus: Bringing the new backend capabilities to the terminal interface.*
- [ ] Refactor the Ratatui state engine.
- [ ] Integrate `rusqlite` for local, encrypted message history.
- [ ] Build the UI for navigating persistent group chats and viewing plugin outputs.
- [ ] **Deliverable:** A fully functional, feature-complete TUI client.

### Milestone 4: Polish & Open Source Launch
*Focus: Documentation, testing, and community onboarding.*
- [ ] Comprehensive unit and integration test coverage for the DHT and Relay crates.
- [ ] Finalize the `docs/version3/` documentation suite for public consumption.
- [ ] Publish the `quickchat_relay` Docker images for easy self-hosting.
- [ ] **Deliverable:** The official QuickChat V3.0.0 Release.
