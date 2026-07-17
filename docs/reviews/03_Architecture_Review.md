# Phase 5: Architecture, Networking, & Security Review

This review challenges the proposed technical architecture of QuickChat V1.

## 1. Core Architecture Review

**Should Rust remain the language?** 
Yes. Rust is the perfect choice for a CLI/TUI tool requiring high performance, memory safety, and cross-platform static binaries.

**Should the async runtime be Tokio?**
Yes. Tokio is the industry standard for async Rust. However, care must be taken with the TUI thread. The Ratatui rendering loop should ideally run on a dedicated thread separated from the Tokio networking runtime to prevent I/O blocking the UI refresh rate (60fps).

**Storage: Where is SQLite?**
The EDD ignores local storage, implying in-memory state. This is a fatal flaw for a chat app. QuickChat *must* use an embedded database. 
*Recommendation:* Use `sqlite` (via `rusqlite` or `sqlx`). It is battle-tested, handles concurrent reads well, and provides a structured schema for Contacts, Messages, and Keys.

## 2. Networking Protocol Review

**Should networking use QUIC?**
Yes. QUIC is superior to TCP for this use case because it prevents Head-of-Line blocking across multiple streams (e.g., streaming a file while simultaneously sending text messages).
*Critique on `s2n-quic`:* While Amazon's `s2n-quic` is great, `quinn` is much more widely adopted in the open-source Rust P2P ecosystem (e.g., libp2p, Iroh). Using `quinn` might provide better community support and easier integration.

**Discovery & NAT Traversal (The Biggest Flaw)**
Relying purely on STUN for WAN in V1 is an engineering mistake.
* If Peer A and Peer B are both behind Symmetric NATs, STUN cannot establish a connection.
* The out-of-band exchange of STUN IPs is terrible UX.
*Recommendation:* Implement a lightweight Signaling Server for V1. Peers connect to the signaling server to exchange ICE candidates (SDP). This allows true "Zero-Config" connection initiation. If direct P2P fails, V1 *must* implement a minimal TURN relay fallback, or users will complain it "doesn't work".

## 3. Security & Cryptography Review

**Should Noise Protocol replace TLS?**
The architecture proposes running Noise Protocol (XX) *inside* QUIC (TLS 1.3). 
*Critique:* QUIC already enforces TLS 1.3. Running Noise inside TLS 1.3 is double-encryption. While this achieves the goal of application-level identity separate from transport, it adds overhead and complexity.
*Alternative:* Use QUIC with custom TLS certificate validation. Both peers generate self-signed certificates based on their Ed25519 identity keys. The QUIC TLS handshake can be configured to verify these custom certificates. This provides mutual authentication and E2EE natively at the QUIC layer, avoiding the need for a separate Noise handshake stream.

**Trust On First Use (TOFU)**
TOFU is acceptable for V1, but visually verifying base64 public keys is error-prone.
*Recommendation:* Implement "Magic Wormhole" style PAKE (Password-Authenticated Key Exchange) for initial contact establishment. User A generates a short, memorable word-code (e.g., `7-purple-dragon`). User B types it. The PAKE protocol establishes a secure channel to exchange the long-term public keys, entirely preventing MitM attacks without requiring out-of-band base64 copying.

## 4. UI/UX Architecture

**Terminal Emojis & Images**
The PRD mentions inline image rendering (Sixel/Kitty). 
*Recommendation:* Drop this from V1. Terminal emulators have wildly inconsistent support for images. It will lead to rendering artifacts and crashes. Focus purely on text, markdown, and code blocks for V1. Provide an "Open in default image viewer" command instead.
