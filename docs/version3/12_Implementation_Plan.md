# 12: Implementation Plan

## QuickChat V3 Technical Execution Strategy

This document serves as the tactical blueprint for engineers contributing to QuickChat V3. It translates the high-level roadmap into concrete technical tasks, architectural shifts, and repository management guidelines.

### 1. Repository Restructuring
The current monorepo will be slightly restructured to accommodate the new components:
- `crates/quickchat_core`: Continues to house the primary TUI and business logic.
- `crates/quickchat_net`: Will undergo a major refactor to abstract QUIC and introduce the DHT and MLS (Messaging Layer Security) submodules.
- `crates/quickchat_plugin_sdk`: Will be locked to a stable API to guarantee backward compatibility for WASM plugins.
- **[NEW]** `crates/quickchat_relay`: A lightweight, self-hostable relay server binary for asynchronous delivery.
- **[NEW]** `crates/quickchat_dht`: The Kademlia implementation for peer routing and discovery.

### 2. Networking Implementation Steps
1. **Deprecate STUN reliance:** Replace the rudimentary V2 STUN logic with a robust `libp2p`-inspired Kademlia DHT implementation.
2. **Implement MLS:** Integrate an open-source MLS library to handle cryptographic group management, discarding the 1-on-1 Noise limitation for group chats.
3. **Relay Server (Opt-In):** Build `quickchat_relay` using `tokio` and `s2n-quic`. It will act as a blind store-and-forward queue. Messages remain end-to-end encrypted; the relay cannot read them.

### 3. Plugin Implementation Steps
1. **WASI Upgrades:** Upgrade the `wasmtime` runtime configuration in `quickchat_plugin_host` to rigidly enforce WASI capability restrictions (e.g., allowing a GitHub plugin network access to `api.github.com` but denying it filesystem access).
2. **IPC Bridge:** Implement an Inter-Process Communication (IPC) bridge or local socket server allowing external editors (Neovim/VSCode) to communicate with the `quickchat_core` for the "Live Code Pointers" feature.

### 4. TUI Implementation Steps
1. **State Management:** Refactor the Ratatui state management to use a robust unidirectional data flow (e.g., Elm architecture) to handle the complexity of concurrent group chats, file transfers, and plugin outputs.
2. **Local Storage:** Integrate `rusqlite` or a pure-Rust KV store to persist chat history and DHT routing tables locally across sessions.

### 5. Migration Strategy from V2
- V3 will introduce breaking changes to the networking protocol.
- V2 clients will not be able to connect directly to V3 clients.
- We will maintain V2 in maintenance mode (security patches only) for 6 months post-V3 release to allow teams to transition their self-hosted configurations.
