# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - Initial Public Release

### Added
- **Core Engine:** Cryptographic identity generation and secure local SQLite storage.
- **LAN Discovery:** Zero-configuration `mdns-sd` discovery for local network peers.
- **WAN Bootstrapping:** Magic Wormhole PAKE short-codes (`/host`, `/join`) for Internet NAT-traversal and connection brokering.
- **Secure Networking:** End-to-end encrypted QUIC connections using `quinn`.
- **Messaging:** Protobuf framing with robust SQLite message persistence across restarts.
- **File Transfers:** Out-of-band `zstd` compressed streaming of large files via QUIC unistreams (`/file <path>`).
- **Terminal UI:** Fully asynchronous, beautiful terminal UI using `ratatui` with markdown support.
- **Developer Tools:** Background UDP IPC allowing CLI to pipe stdout (`quickchat send --stdin`) and code pointers (`quickchat share pointer`) to the active daemon.

### Security
- Integrated `x25519-dalek` and `ed25519-dalek` for robust key generation.
- Custom TLS certificate validation tied directly to peer's known public key.
