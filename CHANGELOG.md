# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [2.0.0] - 2026-07-20

### Added
- **Peer-to-Peer Encrypted Messaging**: Built directly on the Noise Protocol framework, QuickChat now utilizes `Noise_XX` handshakes over QUIC streams for fully local, secure peer-to-peer messaging.
- **WASM Plugin Ecosystem**: Support for WebAssembly plugins (`wasm32-wasi`) securely sandboxed via `wasmtime`. Users can develop plugins (e.g., `/ping`) utilizing the new `quickchat_plugin_sdk`.
- **SQLite Persistence**: Complete migration to an embedded `rusqlite` database ensuring chat histories are stored securely and persistently on the host machine.
- **Local Peer Discovery**: Integrated mDNS-based peer discovery, eliminating the need for manual IP configuration on local networks.
- **Modular Monorepo Architecture**: Refactored the core binary into a scalable workspace containing `quickchat_core`, `quickchat_cli`, `quickchat_tui`, `quickchat_net`, and plugin infrastructure.
- **Event Bus System**: Replaced tight coupling with a flexible `tokio::sync::broadcast` event-driven architecture, enabling headless operations and isolated TUI rendering.

### Changed
- Complete rewrite of the terminal user interface utilizing `ratatui` for enhanced performance and visual clarity.
- Removed all central server logic in favor of a pure P2P discovery mechanism.
- Upgraded default cryptographic primitives to X25519 (Key Exchange) and ChaCha20-Poly1305 (AEAD).

### Security
- Created explicit audit overrides for transitive `wasmtime` dependencies that are unmaintained (`mach`, `paste`) but isolated within the host execution environment.
