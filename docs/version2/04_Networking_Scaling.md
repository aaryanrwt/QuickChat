# QuickChat Version 2: Networking & Scaling

## 1. Executive Summary

QuickChat V1 utilizes QUIC and mDNS for robust local networking. However, wide-area network (WAN) connectivity relies on cumbersome out-of-band key exchanges (STUN). To achieve the V2 vision of seamless, global collaboration and persistent group chats, the networking layer must evolve to support decentralized discovery and managed relays for strict NAT traversal.

## 2. Global Peer Discovery (DHT)

### 2.1. The Limitation of V1
In V1, connecting to a peer across the internet requires manually sharing a public IP/port (discovered via STUN) and a public key. This is a severe UX bottleneck.

### 2.2. V2 Solution: Kademlia DHT
V2 will integrate a Distributed Hash Table (DHT), specifically a Kademlia-based protocol (likely via `libp2p` or a custom lightweight implementation).
- **Node IDs:** A peer's Node ID is a hash of its X25519 public key.
- **Routing:** Peers maintain routing tables of their closest neighbors in the network.
- **Provider Records:** When a peer comes online, it publishes a provider record to the DHT: "I am Public Key X, and my current IP/Port is Y."
- **Discovery Flow:** When User A wants to message User B, QuickChat queries the DHT for User B's public key to retrieve their current IP/Port, enabling automatic connection establishment.

## 3. Strict NAT Traversal & Managed Relays

### 3.1. The Symmetric NAT Problem
STUN fails when both peers are behind symmetric NATs or strict corporate firewalls. Direct P2P connection is impossible in these scenarios.

### 3.2. V2 Solution: QuickChat Managed Relays (TURN-like)
QuickChat must introduce an optional Relay architecture.
- **Protocol:** A custom, lightweight UDP relay protocol operating over QUIC (or standard TURN).
- **Operation:** If direct P2P fails, both peers connect to a QuickChat Managed Relay Server. The server blindly forwards encrypted QUIC packets between the two peers.
- **Security:** The relay server *cannot* decrypt the traffic because the Noise XX handshake provides End-to-End Encryption (E2EE). The relay only sees opaque encrypted streams.
- **Enterprise Alignment:** The Managed Relay service forms a core component of the Enterprise monetization strategy (SLA-backed, high-bandwidth relays).

## 4. Asynchronous Messaging (Offline Delivery)

### 4.1. The P2P Offline Problem
In pure P2P, if User B is offline, User A cannot send them a message. 

### 4.2. V2 Solution: Relay Store-and-Forward
- **Opt-in Asynchrony:** Users can opt-in to use the Managed Relay for offline delivery.
- **Mechanism:** User A encrypts the message with User B's public key (using Noise/ChaCha20-Poly1305) and sends the ciphertext to the Relay.
- **Storage:** The Relay stores the encrypted blob.
- **Retrieval:** When User B comes online, they authenticate with the Relay and download pending encrypted blobs.
- **Zero-Knowledge:** The Relay server learns nothing about the message content or metadata, preserving privacy.

## 5. Persistent Group Chats

Group chats in V2 require a hybrid consensus model.
- **Small Groups:** Full mesh networking (every peer connects to every other peer). Highly secure but doesn't scale well past ~10 users.
- **Large/Enterprise Groups:** A designated "Group Server" (which can be a user's machine acting as a host, or an Enterprise deployment) handles distributing messages to the group, acting as a pub-sub broker. All messages remain E2EE using a group ratcheting protocol (e.g., Sender Keys/Signal Protocol).
