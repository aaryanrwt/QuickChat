# QuickChat V4: Open Source Architecture

QuickChat V4 is architected as a decentralized, peer-to-peer system built entirely in Rust, maximizing performance and memory safety while ensuring no single point of failure.

## High-Level Components

1.  **Client Application (Rust):**
    *   **TUI Layer (Ratatui):** Handles all user interactions, rendering, and keyboard events.
    *   **Application Core:** Manages business logic, file I/O, local state, and plugin lifecycle.
    *   **Networking Stack:** Manages P2P connections, encryption, and data serialization.

2.  **Networking & Security (The P2P Foundation):**
    *   **Transport:** `s2n-quic` (QUIC over UDP) for multiplexed, low-latency streams.
    *   **Encryption:** Noise Protocol Framework (XX Handshake) with X25519 (ECDH) and ChaCha20-Poly1305. E2EE is non-negotiable.
    *   **Discovery:** mDNS for local networks; a Kademlia-based DHT (Distributed Hash Table) for global decentralized discovery.

3.  **Self-Hosted Infrastructure (Optional):**
    *   **Open Relay Nodes:** Open-source Rust server for NAT traversal (STUN/TURN) and asynchronous message delivery.
    *   **Identity Directory:** Optional self-hosted directory for teams using OIDC/SAML.

## Extensibility Architecture
*   **WASM Sandbox:** Plugins run in a secure WebAssembly environment. They communicate with the Application Core via a well-defined RPC protocol, ensuring plugins cannot crash the main application or access unauthorized data.

## Open Source Shift
Previously closed modules (e.g., the Enterprise relay and proprietary SSO connectors) have been entirely removed and replaced with open-standard, community-maintainable Rust crates.
