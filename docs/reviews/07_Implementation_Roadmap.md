# Phase 8: Engineering Roadmap

To ensure successful delivery of a polished, robust product, Version 1 is broken down into progressive milestones. Never jump randomly between modules; everything must build upon the previous foundation.

## Milestone 1: The Core Foundation
**Goal:** Establish local storage, cryptographic identity, and basic CLI architecture.
*   **Modules:** `quickchat_types`, `quickchat_core` (Identity, SQLite DB).
*   **Dependencies:** `rusqlite`, `ed25519-dalek`, `x25519-dalek`, `clap`.
*   **Deliverable:** A CLI command `quickchat id` that generates and securely stores a keypair, displaying the public key.
*   **Definition of Done:** Keys are generated, stored in local SQLite, and unit tests verify persistence.

## Milestone 2: Networking & Discovery
**Goal:** Enable two instances to find each other on a LAN and establish a secure connection.
*   **Modules:** `quickchat_net`.
*   **Dependencies:** `quinn` (or `s2n-quic`), `mdns-sd`.
*   **Deliverable:** Two instances on the same network can discover each other, perform mutual authentication via QUIC/TLS (with custom cert validation), and maintain an open connection.
*   **Definition of Done:** Automated integration test spins up two instances, they discover via mDNS, connect, and authenticate.

## Milestone 3: Messaging Engine
**Goal:** Send and receive text messages persistently.
*   **Modules:** `quickchat_core` (MessageProcessor).
*   **Dependencies:** `prost` (Protocol Buffers).
*   **Deliverable:** A CLI command `quickchat send <alias> "hello"` successfully transmits a message over the secure connection. The message is saved to the SQLite DB on both ends.
*   **Definition of Done:** Messages survive application restarts.

## Milestone 4: Terminal UI (The "Wow" Factor)
**Goal:** Build the interactive interface on top of the working core.
*   **Modules:** `quickchat_tui`.
*   **Dependencies:** `ratatui`, `crossterm`.
*   **Deliverable:** The full TUI as described in the Design System. Users can select contacts, type messages, and see history.
*   **Definition of Done:** Keyboard navigation works flawlessly. Rendering handles resizing correctly.

## Milestone 5: File Transfer & Compression
**Goal:** Enable secure, compressed file sharing.
*   **Modules:** `quickchat_core` (FileManager).
*   **Dependencies:** `zstd`.
*   **Deliverable:** Users can initiate file transfers. Data is chunked, compressed, encrypted, and reassembled on the receiver's end with SHA256 verification.
*   **Definition of Done:** A 1GB file transfers successfully and checksums match.

## Milestone 6: Developer Tools (The Differentiator)
**Goal:** Implement the features that make QuickChat special for developers.
*   **Modules:** `quickchat_core` (DeveloperTools).
*   **Deliverable:** `quickchat share pointer file:line` and `quickchat share output <cmd>` are fully functional within the TUI and CLI.
*   **Definition of Done:** Piping stdout to the CLI (`make test | quickchat send alias --stdin`) sends correctly formatted code blocks.

## Milestone 7: PAKE & WAN Bootstrapping (Crucial Fix)
**Goal:** Fix the UX of WAN discovery.
*   **Modules:** `quickchat_net`, `quickchat_core`.
*   **Deliverable:** Implement Magic Wormhole-style short codes (PAKE) for initial key exchange and signaling over a default fallback relay server.
*   **Definition of Done:** Users can connect across different internet connections using only a 3-word phrase.
