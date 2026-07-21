# 02: Version 3 Vision

## The Open-Source Developer's Communication Layer

The vision for QuickChat V3 is to become the definitive open-source communication layer for developers, entirely free of commercial gating, telemetry, or proprietary lock-in. V3 will transition QuickChat from a standalone P2P chat tool into a persistent, deeply integrated collaborative environment.

## Core Pillars for V3

### 1. Pure Open-Source Ecosystem
We reject the notion of an "Enterprise Edition". Every feature, from relay servers to role-based access control in group chats, will be 100% open source and self-hostable. 

### 2. Decentralized Persistence
V1 and V2 were ephemeral. V3 will introduce **Persistent Group Chats** using a hybrid DHT (Distributed Hash Table) and opt-in open-source relay servers. This ensures teams can maintain asynchronous communication without relying on a central corporate server.

### 3. Hyper-Integrated Workflows
QuickChat V3 will natively integrate with the developer's environment:
- **Live Code Pointers:** Instantly jump to a teammate's exact line of code in Neovim/VSCode from the terminal chat.
- **Shared Terminal Sessions:** Native `tmux`-style read-only session broadcasting over QUIC.
- **Local AI:** Seamless integration with local LLMs (e.g., Ollama) via WASM plugins to summarize logs and troubleshoot errors without sending data to the cloud.

### 4. Zero-Config Global Discovery
V2 relied on mDNS (LAN) and basic STUN. V3 will fully implement a secure DHT network, allowing developers across the globe to discover and connect with peers using only a public key, completely abstracting away NAT traversal complexity.
