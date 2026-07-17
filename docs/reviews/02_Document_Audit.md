# Phase 2: Document Audit & Cross-Document Validation

After a rigorous, word-by-word analysis of the QuickChat documentation suite, several critical inconsistencies, contradictions, and gaps have been identified. 

## 1. Contradictions

**Contradiction 1: Persistent vs. Ephemeral Messaging**
- *Competitive Analysis (Sec 4.2):* "Lack of Persistent Terminal Chat: Existing terminal file transfer tools are ephemeral. QuickChat provides persistent, secure messaging."
- *Engineering Design Document (Sec 4.2.1):* "Manages message history (local, non-persistent for V1)."
- *Resolution Needed:* If V1 is non-persistent, it is no better than an encrypted netcat session. Users expect chat apps to retain history when they restart the app. V1 *must* include local SQLite persistence for message history, or the marketing copy must be changed.

**Contradiction 2: Zero-Configuration vs. Out-of-Band Key Exchange**
- *PRD (Sec 4.5):* "REQ-5.1: Zero-Configuration Discovery... REQ-5.2: NAT Traversal without manual port forwarding."
- *Discovery Protocol (Sec 3.2.2):* "Users will need to exchange their public keys and associated public endpoint information (obtained via STUN) through an out-of-band channel."
- *Resolution Needed:* Out-of-band exchange of IP addresses and public keys is the *definition* of manual configuration. It is terrible UX. Magic Wormhole solves this with PAKE and a short relay code. QuickChat needs a lightweight signaling server for WAN connections to actually achieve "Zero-Configuration".

**Contradiction 3: STUN vs. Real-World NAT**
- *Networking Protocol (Sec 5.2):* "STUN... This information is crucial for peers behind NATs to initiate... connections. ICE/Relay (Future): ...out of scope for V1."
- *Resolution Needed:* STUN fails entirely if *both* peers are behind Symmetric NATs (common in enterprise/cellular networks). Without a TURN (relay) fallback, a significant percentage of WAN connection attempts in V1 will silently fail, leading to churn.

## 2. Inconsistencies

**Inconsistency 1: Serialization Formats**
- *Technical Requirements Document (TRD):* Mentions choosing between Protocol Buffers or MessagePack.
- *Networking Protocol / Engineering Design:* Explicitly states Protocol Buffers will be used.
- *Resolution Needed:* Standardize on Protocol Buffers. It provides strong schema evolution which is vital for a P2P protocol where peers might run different versions.

**Inconsistency 2: Compression & Encryption Order**
- *File Transfer Protocol (Sec 5.2):* States chunks are compressed using Zstd, *then* encrypted using ChaCha20-Poly1305. 
- *Note:* While Compress-then-Encrypt is correct to prevent attackers from manipulating the ciphertext to exploit compression, it exposes the system to traffic analysis (e.g., CRIME/BREACH style attacks). An attacker observing the ciphertext length can infer the compressibility of the plaintext. For a secure developer tool, we must ensure padding is applied *after* compression and *before* encryption to obscure file metadata.

## 3. Missing Requirements

- **Local Database/Storage Schema:** None of the documents define *how* Contacts, Trust state (TOFU), and configurations are stored. A robust embedded database (SQLite) is required.
- **Crash Reporting & Telemetry:** For a V1 targeting 10k stars rapidly, telemetry (opt-in) and crash reporting are essential.
- **Rate Limiting / Anti-Spam:** Even in P2P, a malicious peer can flood a connection. The QUIC stream management needs application-level rate limiting.
- **Offline Queuing:** If Peer B is offline, what happens to Peer A's message? If it simply fails to send, the UX is poor. If it queues locally until Peer B is online, that requires local persistence (which contradicts the EDD).
