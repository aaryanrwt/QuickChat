# QuickChat V4: Networking

QuickChat V4 relies on a robust, highly optimized networking stack designed for peer-to-peer communication.

## 1. Transport Layer (QUIC)
V4 uses QUIC (`s2n-quic` in Rust) over UDP instead of TCP.
*   **Multiplexing:** Prevents head-of-line blocking. A large file transfer will not delay a small text message on the same connection.
*   **Low Latency:** Faster handshake connection setup compared to TCP+TLS.
*   **Connection Migration:** Handles IP changes gracefully (e.g., switching from WiFi to Cellular) without dropping the connection.

## 2. Peer Discovery
*   **LAN (mDNS):** Zero-configuration discovery on local networks via Multicast DNS.
*   **WAN (DHT):** A Kademlia-based Distributed Hash Table allows peers to find each other globally without a central server.

## 3. NAT Traversal & Relays
*   **STUN/TURN:** Basic NAT traversal is achieved using standard STUN techniques.
*   **Community Relays:** For restrictive symmetric NATs or offline messaging, V4 introduces open-source Relay Nodes. These nodes act as dumb, blind forwarders of encrypted QUIC packets. Anyone can host a relay node to support the network.

## 4. Bandwidth Optimization
*   **Zstd Compression:** All file transfers and large message payloads are compressed using Zstandard before encryption to minimize bandwidth usage.
