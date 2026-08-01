# QuickChat V4: Security Model

QuickChat V4 is built on a zero-trust, privacy-first security model. No centralized authority is trusted with unencrypted data.

## 1. End-to-End Encryption (E2EE)
*   **Protocol:** The Noise Protocol Framework (specifically the Noise_XX pattern) is used for mutual authentication and secure key exchange.
*   **Primitives:** X25519 for Elliptic Curve Diffie-Hellman (ECDH), ChaCha20-Poly1305 for Authenticated Encryption with Associated Data (AEAD).
*   **Scope:** ALL data is encrypted in transit. This includes text messages, file transfers, and typing indicators. Relay servers only route encrypted blobs.

## 2. Identity & Key Management
*   **Keypairs:** Each user's identity is defined by a locally generated cryptographic keypair.
*   **Storage:** Private keys are stored locally on the user's filesystem, protected by OS-level permissions.
*   **Trust:** V4 utilizes a Trust on First Use (TOFU) model, supplemented by out-of-band verification (e.g., verifying a fingerprint over a voice call) or an optional self-hosted identity directory.

## 3. Threat Model
*   **Network Eavesdropper:** Defeated by E2EE (Noise Protocol).
*   **Compromised Relay Server:** Relay servers cannot read message contents or forge messages, as they lack the private keys. They can only observe traffic patterns (metadata).
*   **Malicious Plugin:** Defeated by the WASM sandbox. Plugins have heavily restricted network and filesystem access, enforced by the Rust runtime.

## 4. No Telemetry
*   V4 strictly prohibits any form of phone-home telemetry, tracking pixels, or usage analytics. Privacy is the default and only state.
