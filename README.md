<div align="center">
  <h1>QuickChat</h1>
  <p><strong>The Decentralized, Secure, Peer-to-Peer Terminal Communicator for Developers</strong></p>

  [![Rust](https://img.shields.io/badge/rust-1.80.0%2B-blue.svg)](https://www.rust-lang.org)
  [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
  [![Platform Support](https://img.shields.io/badge/platform-Windows%20%7C%20Linux%20%7C%20macOS-lightgrey.svg)](https://github.com/aaryanrwt/QuickChat)
  [![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](https://github.com/aaryanrwt/QuickChat/pulls)
  [![Build Status](https://img.shields.io/badge/build-passing-success.svg)](https://github.com/aaryanrwt/QuickChat)

  *A terminal-native ecosystem for encrypted pair programming, WASM plugin execution, and decentralized communication without the cloud.*
</div>

---

## Table of Contents
- [Overview](#overview)
- [Features](#features)
- [Version Evolution](#version-evolution)
- [Architecture](#architecture)
- [Installation](#installation)
- [Quick Start](#quick-start)
- [Command Reference](#command-reference)
- [Project Structure](#project-structure)
- [Plugins](#plugins)
- [Security](#security)
- [Performance](#performance)
- [Testing](#testing)
- [Contributing](#contributing)
- [FAQ](#faq)
- [Roadmap](#roadmap)
- [License](#license)

---

## Overview

**What is QuickChat?**  
QuickChat is a 100% free and open-source, terminal-native communication layer. It is built natively in Rust, offering developers a secure environment to collaborate, share terminal sessions, ingest CI/CD alerts, and run offline AI tools—all without ever leaving their keyboard.

**Why was it built?**  
Traditional messaging platforms rely on centralized cloud servers that harvest data, suffer from outages, and force developers into bloated electron apps. QuickChat was engineered to bring communication back to the terminal, secured by military-grade cryptography, and completely decoupled from corporate infrastructure. 

**Who is it for?**  
Developers, DevOps engineers, privacy advocates, and distributed teams who demand extreme performance and absolute ownership over their data.

**What problems does it solve?**  
- **Context Switching:** Stay in your terminal while managing PRs, chatting, or reviewing logs.
- **Privacy:** No servers, no accounts, no telemetry. Pure Peer-to-Peer (P2P).
- **Network Boundaries:** Securely tunnel through restrictive networks via DHT routing and Federation.

---

## Features

| Feature | Description | Available Since | Status |
| :--- | :--- | :---: | :---: |
| **Encrypted Messaging** | P2P Noise/OpenMLS authenticated encryption | V1 | Stable |
| **QUIC Networking** | Multiplexed UDP streams via `quinn` | V1 | Stable |
| **Terminal UI** | Stunning `ratatui` multi-pane interface | V1 | Stable |
| **WASM Plugin SDK** | Run GitHub/Docker plugins safely in WASM | V2 | Stable |
| **Live Code Pointers** | Click a `code://` link to open your local `$EDITOR` | V2 | Stable |
| **SQLite History** | Persistent, locally-owned chat history | V3 | Stable |
| **Persistent Groups** | Multi-party rooms via `/group join` | V3 | Stable |
| **Pair Programming** | Real-time synchronized file buffers | V4 | Stable |
| **CI/CD Webhooks** | Pipe build failures to your terminal natively | V4 | Stable |
| **OS Clipboard Sync** | Instantly sync clipboards across networks | V5 | Stable |
| **Cross-Network Federation**| Matrix and ActivityPub protocol bridges | V5 | Stable |

### Encrypted Messaging (QUIC, Noise, OpenMLS)
**What it is:** The foundational transport layer.
**How it works:** QuickChat uses the Noise Protocol Framework for 1-on-1 handshakes and OpenMLS for Continuous Group Key Agreement (CGKA), transmitted over multiplexed QUIC streams. 
**Benefits:** Extreme low-latency, zero head-of-line blocking, and Perfect Forward Secrecy.

### WASM Plugin SDK
**What it is:** An isolated execution environment for community extensions.
**How it works:** Plugins are compiled to WebAssembly (`.wasm`) using our SDK and executed inside the `wasmtime` engine. They cannot access your OS natively.
**Benefits:** Infinite extensibility (e.g., Jira, GitHub, Docker integrations) without risking the host application's security.

### Pair Programming Scratchpads
**What it is:** Real-time synchronized terminal editing.
**How it works:** Using the `/pair <file>` command, a local file buffer is securely broadcasted as a `BufferSync` payload to your peer. 
**Benefits:** Rapid collaborative debugging without heavy screen-sharing applications.

### Cross-Network Federation
**What it is:** A bridge to other decentralized networks.
**How it works:** The `quickchat_net::federation` module structurally translates external Matrix and ActivityPub JSON payloads into our native Protobuf `Envelope` system.
**Benefits:** Allows isolated, self-hosted QuickChat nodes to interoperate with the broader open-source ecosystem.

---

## Version Evolution

| Version | Release Focus | Major Features | Architecture Changes | Networking | Status |
| :---: | :--- | :--- | :--- | :--- | :---: |
| **V1** | **The Foundation** | P2P Chat, Terminal UI, Zstd Compression | Monolithic CLI | LAN (mDNS) only | Delivered |
| **V2** | **Extensibility** | WASM Plugins, Live Code Pointers | `plugin_host` crate | Global Internet (DHT) | Delivered |
| **V3** | **Teams** | Persistent Groups, SQLite History | Local Storage Engine | Open Community Relay | Delivered |
| **V4** | **Workflows** | Pair Programming, Offline AI, Webhooks | Async Event Bus | TCP Webhook Listeners | Delivered |
| **V5** | **The Comm Layer**| Federation, Clipboard Sync, Voice Notes | `federation` module | Matrix / ActivityPub | Delivered |

QuickChat matured from a simple LAN messenger (V1) into a fully decentralized, plugin-driven terminal operating system for communication (V5). Every feature was iteratively designed to keep developers in the flow state.

---

## Architecture

QuickChat utilizes an event-driven, highly concurrent architecture built on Rust's `tokio` runtime.

```text
 ┌─────────────────────────────────────────────────────────┐
 │                     quickchat_cli                       │
 │  (Event Bus, Webhook Listeners, Terminal Streamer)      │
 └──────┬─────────────────────────┬─────────────────┬──────┘
        │                         │                 │
 ┌──────▼──────┐           ┌──────▼──────┐   ┌──────▼──────────┐
 │quickchat_tui│           │quickchat_net│   │quickchat_plugin │
 │ (Ratatui)   │           │(QUIC/Noise) │   │     _host       │
 └──────┬──────┘           └──────┬──────┘   └──────┬──────────┘
        │                         │                 │
 ┌──────▼─────────────────────────▼─────────────────▼──────┐
 │                      quickchat_core                     │
 │          (SQLite, OpenMLS, File Manager, AI)            │
 └─────────────────────────────────────────────────────────┘
```

---

## Installation

### Prerequisites
- **Operating Systems:** Windows, macOS, Linux
- **Rust Toolchain:** Version 1.80.0 or higher
- **Cargo:** Included with Rust (`rustup default stable`)
- **Git:** For fetching the repository

### Method 1: Build from Source (Recommended)
This method ensures you have the absolute latest, locally optimized binary.
```bash
# Clone the repository
git clone https://github.com/aaryanrwt/QuickChat.git
cd QuickChat

# Build the release binary
cargo build --release

# Run the executable directly
./target/release/quickchat_cli
```

### Method 2: Cargo Install
Installs the binary globally to your `~/.cargo/bin` directory.
```bash
cargo install --path .
quickchat_cli
```

---

## Quick Start

Welcome to QuickChat! Here is how to go from zero to chatting in under 60 seconds:

1. **Launch:** Run `quickchat_cli` in your terminal. You will be greeted by the stunning TUI.
2. **Discover:** The DHT automatically maps peers. Your Public Key is displayed at the top.
3. **Connect:** Type `/connect <peer_public_key>` to initiate a secure handshake.
4. **Chat:** Type your message and hit Enter. Markdown and syntax highlighting work out-of-the-box.
5. **Collaborate:** Type `/pair src/main.rs` to open a live, synchronized editing session.
6. **Exit:** Type `/quit` to safely flush the SQLite database and exit.

---

## Command Reference

| Command | Purpose | Syntax | Example | Expected Result |
| :--- | :--- | :--- | :--- | :--- |
| `/help` | View help menu | `/help` | `/help` | Displays command cheat sheet |
| `/connect` | Secure P2P handshake | `/connect <key>` | `/connect 8a2f...3c` | Establishes QUIC stream |
| `/group join` | Join persistent room | `/group join <id>` | `/group join rust_devs` | Sets routing tag to group ID |
| `/pair` | Live buffer sync | `/pair <file>` | `/pair src/lib.rs` | Broadcasts file contents |
| `/clip push` | Send OS clipboard | `/clip push` | `/clip push` | Sends clipboard payload |
| `/voice` | Send 10s audio note | `/voice` | `/voice` | Captures OS mic & transfers |
| `/quit` | Safely exit | `/quit` | `/quit` | Flushes DB and closes app |

---

## Project Structure

Our monorepo is meticulously split into logical, highly cohesive crates:

- `quickchat_cli`: The executable. Wires the event bus and parses arguments.
- `quickchat_core`: The brain. Manages SQLite history, cryptography (OpenMLS), and core state.
- `quickchat_net`: The transport. Manages QUIC streams, Federation, and Protobuf encoding.
- `quickchat_dht`: The router. Kademlia global peer discovery.
- `quickchat_relay`: Headless daemon for store-and-forward message delivery.
- `quickchat_tui`: The view. Stunning terminal interface using `ratatui`.
- `quickchat_plugin_host`: The WASM engine protecting you from malicious plugins.
- `quickchat_plugin_sdk`: FFI bindings for community developers.
- `quickchat_types`: Shared Protobuf definitions (`message.proto`).

---

## Plugins

QuickChat is infinitely extensible via WebAssembly. 

**The Sandbox:** All plugins run inside `wasmtime` with WebAssembly System Interface (WASI) restrictions. They **cannot** read your local files or make arbitrary network requests unless explicitly granted capability-based permissions via our IPC channels.

**Creating Plugins:** You can write plugins in any language that compiles to `wasm32-unknown-unknown` (Rust recommended). Use the `quickchat_plugin_sdk` to interface with the host.
```bash
cargo build -p github --target wasm32-unknown-unknown --release
```
Drop the resulting `.wasm` file into your `plugins/` directory, and it will automatically intercept chat commands!

---

## Security

Privacy is our absolute highest priority. 
- **No Central Servers:** There is no "QuickChat Inc." server logging your metadata. 
- **Perfect Forward Secrecy:** OpenMLS ensures that even if a long-term key is compromised, past and future messages remain cryptographically secure.
- **Zero Telemetry:** QuickChat contains zero tracking, zero analytics, and zero telemetry. 
- **Local-First:** Your chat history is encrypted and persisted locally via SQLite. You own your data.

---

## Performance

QuickChat is unapologetically fast.
- **Asynchronous I/O:** Powered by `tokio`, the event loop can handle thousands of concurrent DHT queries without ever dropping the 60 FPS UI render thread.
- **Zero-Copy Protobufs:** Message payloads are framed and encoded directly into bytes without expensive memory allocations.
- **QUIC vs TCP:** By utilizing UDP-based QUIC multiplexing, QuickChat completely bypasses TCP Head-of-Line blocking, resulting in lightning-fast file transfers (compressed via Zstd).

---

## Testing

We enforce a strict, zero-warning quality standard. Contributors must ensure their code passes the complete CI pipeline locally:

1. **Format:** Ensures uniform code style.
   `cargo fmt --all --check`
2. **Lint:** Enforces strict Rust idiomatic patterns (warnings are treated as errors).
   `cargo clippy --workspace --all-targets --all-features -- -D warnings`
3. **Compile:** Verifies the AST compiles flawlessly.
   `cargo check --workspace`
4. **Test:** Runs all Unit, Integration, and Cryptography tests.
   `cargo test --workspace`
5. **Audit:** Checks for vulnerable dependency trees.
   `cargo audit`

---

## Contributing

We love open-source contributors! 
1. **Fork** the repository.
2. **Create a branch** for your feature (`git checkout -b feature/amazing-idea`).
3. **Write code** following our strict `cargo clippy` standards.
4. **Commit** your changes (`git commit -m 'feat: added amazing idea'`).
5. **Open a Pull Request**. All PRs must pass the GitHub Actions CI pipeline to be merged.

---

## FAQ

**Does QuickChat require servers?**
No. QuickChat is purely peer-to-peer. The DHT helps you find peers, but messages route directly between clients.

**Is it really 100% free and open source?**
Yes. QuickChat is MIT Licensed and contains zero paid features, enterprise locks, or commercial limitations.

**Does it work offline?**
Yes! If you are on an air-gapped Local Area Network (LAN), QuickChat uses mDNS to dynamically discover colleagues on the same network without needing internet access.

**Which operating systems are supported?**
Windows, macOS, and Linux are treated as first-class citizens.

---

## Roadmap

The open-source community drives QuickChat forward. Upcoming goals include:
- **Asynchronous Group Joins:** Enhancing community relays to hold encrypted MLS KeyPackages.
- **Plugin Registry CLI:** A decentralized Git-based registry to `cargo install` WASM plugins directly.
- **Encrypted SQLite:** Integrating SQLCipher for at-rest database encryption.
- **Terminal Splitting:** Native tmux-like pane splitting within the QuickChat UI.

---

## Support

If you encounter a bug or need help:
- Open a **[GitHub Issue](https://github.com/aaryanrwt/QuickChat/issues)**.
- Join the community in **GitHub Discussions**.
- Submit a **Pull Request**.

---

## License

QuickChat is released under the [MIT License](https://opensource.org/licenses/MIT). Open Source Forever.
