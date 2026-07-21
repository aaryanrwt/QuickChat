# Migration Guide: QuickChat V2 to V3

Welcome to QuickChat Version 3! 🚀 

Version 3 represents a massive architectural leap forward, transforming QuickChat from a local-network 1-on-1 tool into a globally routable, multi-party secure communication platform.

This guide outlines the significant changes, what they mean for your workflow, and how plugin developers should adapt.

## 1. Network Discovery: mDNS → Kademlia DHT

**What Changed?**
In V2, QuickChat relied exclusively on Multicast DNS (mDNS) to discover peers. This meant you could only talk to developers on your immediate local Wi-Fi or subnet. 
In V3, we have implemented a **Kademlia Distributed Hash Table (DHT)** (`quickchat_dht`).

**What does this mean for you?**
You can now securely discover and route messages to peers globally over the internet without needing manual IP configurations or port-forwarding. mDNS is still supported as a fallback for offline LAN environments, but the DHT is the new primary routing engine.

## 2. Cryptography: Noise_XX → OpenMLS

**What Changed?**
V2 utilized the Noise Protocol framework (specifically the `Noise_XX` handshake) for 1-on-1 End-to-End Encryption.
V3 introduces **Messaging Layer Security (OpenMLS)** (`quickchat_core::mls`).

**What does this mean for you?**
You can now create persistent, secure **Group Chats**. OpenMLS provides industry-standard Continuous Group Key Agreement (CGKA), meaning encryption keys ratchet securely every time a team member joins or leaves a room, ensuring perfect forward secrecy for multi-party conversations.

## 3. Message Persistence: Ephemeral → SQLite

**What Changed?**
In V2, all chat history was ephemeral and lived only in RAM. If you closed the terminal, your history vanished.
V3 integrates an embedded **SQLite Database** (`rusqlite`) directly into the TUI application state.

**What does this mean for you?**
Your chat histories are now persistently saved to your local machine (typically `~/.local/share/quickchat/quickchat.db` on Linux/macOS). Your encrypted messages are stored locally, meaning you never lose track of a conversation across reboots. QuickChat still has no central database servers.

## 4. The New `quickchat_relay` Daemon

**What Changed?**
V3 introduces an optional, headless binary: `quickchat_relay`.

**What does this mean for you?**
In P2P systems, if your peer is offline, you cannot send them a message. The new open-source Relay daemon acts as an asynchronous store-and-forward queue. You (or your organization) can self-host this lightweight daemon. When you send an encrypted message to an offline peer, it waits at the relay until they come online. The relay cannot decrypt the messages.

## 5. Plugin SDK: WASI & IPC Enhancements

**What Changed?**
The WebAssembly plugin sandbox (`quickchat_plugin_host`) has been hardened. Furthermore, plugins can now issue Inter-Process Communication (IPC) commands to the host OS.

**What does this mean for developers?**
V3 introduces the **Live Code Pointers** plugin (`code://file.rs:42`). When a peer clicks this link in the UI, the plugin parses it and triggers the host OS to instantly open the specified file and line number in your local editor (VS Code, Neovim, etc.).

---

### Upgrading

To upgrade from V2 to V3:
1. Ensure your local Rust toolchain is updated (`rustup update`).
2. Run `cargo install --path .` or `cargo build --release` from the project root.
3. The new SQLite database will automatically initialize on your first launch.

*Note: Because the cryptography engine completely changed from Noise to MLS, V2 identities and active sessions are not backwards compatible with V3 nodes.*
