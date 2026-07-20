# QuickChat

<p align="center">
  <strong>A secure, peer-to-peer terminal messenger built in Rust.</strong>
</p>

<p align="center">
  <a href="https://github.com/aaryanrwt/QuickChat/actions/workflows/ci.yml">
    <img src="https://github.com/aaryanrwt/QuickChat/actions/workflows/ci.yml/badge.svg" alt="CI">
  </a>
  <a href="https://opensource.org/licenses/MIT">
    <img src="https://img.shields.io/badge/License-MIT-yellow.svg" alt="License: MIT">
  </a>
</p>

QuickChat enables developers to communicate securely over local networks without centralized servers, accounts, or cloud infrastructure. It solves the problem of untrusted networks and third-party data mining by keeping your conversations purely peer-to-peer, encrypted, and inside the environment developers love most: the terminal.



## ✨ Features

- **Peer-to-Peer Encrypted Messaging**: Connect directly to other users without a middleman server. Every connection is secured via the Noise Protocol framework, ensuring complete privacy.
- **Terminal-Native UI**: Built with `ratatui`, the interface is entirely keyboard-driven, fast, and familiar to anyone who uses tools like `vim` or `htop`.
- **WASM Plugin Ecosystem**: Extend QuickChat using WebAssembly plugins (`wasm32-wasi`). This allows the community to build custom commands (like `/ping`) in a safe, sandboxed environment without altering the core Rust binary.
- **SQLite Message Persistence**: Your chat history is saved locally in an embedded SQLite database, meaning you never lose track of a conversation, and your data never leaves your machine.
- **Local Peer Discovery**: Automatically discover other QuickChat users on your local network using mDNS, bypassing the need for manual IP configuration.

---

## 📥 Installation

### Build from Source
1. Clone the repository:
   ```bash
   git clone https://github.com/aaryanrwt/QuickChat.git
   cd QuickChat
   ```
2. Build the workspace:
   ```bash
   cargo build --release
   ```
3. The executable will be available at `./target/release/quickchat_cli`.
4. Run the executable:
   ```bash
   ./target/release/quickchat_cli
   ```

---

## 🚀 Quick Start

1. **Launch**: Start the application by running the CLI.
   ```bash
   quickchat_cli
   ```
2. **Discover**: Check the left "Contacts" pane for locally discovered peers on your network.
3. **Connect**: Use the `/connect` command with a peer's public key (displayed in their UI) to initiate a secure handshake.
4. **Message**: Type your message in the input bar and press `Enter` to send it instantly over the P2P QUIC stream.
5. **Exit**: Press `Ctrl+C` or type `/quit` to close the application safely.

---

## ⌨️ Commands

QuickChat utilizes a slash-command syntax for all actions.

### `/help`
- **Purpose**: Displays the integrated help menu and lists all available loaded commands.
- **Syntax**: `/help`
- **Example**: `/help`
- **Expected Output**: Opens a modal in the center of the terminal displaying all shortcuts and available plugin commands.

### `/connect`
- **Purpose**: Initiates a Noise handshake to establish a secure connection with a peer.
- **Syntax**: `/connect <public_key>`
- **Example**: `/connect 8a2f...3c`
- **Expected Output**: The connection details pane updates to "Connected", and you can begin sending messages.

### `/clear`
- **Purpose**: Clears the current terminal workspace pane to improve readability.
- **Syntax**: `/clear`
- **Example**: `/clear`
- **Expected Output**: The chat history UI clears (the SQLite database is unaffected).

### `/ping`
- **Purpose**: An example command implemented via the bundled WASM plugin.
- **Syntax**: `/ping`
- **Example**: `/ping`
- **Expected Output**: The plugin intercepts the command and the host responds with `Pong!`.

---

## 🏗️ Architecture

QuickChat utilizes an event-driven, highly modular architecture to separate networking, UI, and plugin execution.

- **CLI**: The entry point that initializes the async runtime and configuration.
- **Event Bus**: The nervous system of the application (`tokio::sync::broadcast`). All components talk to each other by emitting and listening for events (e.g., `MessageReceived`, `InputSubmit`).
- **Networking**: Handles the QUIC streams and cryptographic Noise handshakes.
- **UI**: Renders the terminal interface and translates keystrokes into Event Bus events.
- **Plugins**: A WebAssembly sandbox (`wasmtime`) that listens to the event bus and executes third-party code securely.

**How Messages Travel:**
User types in UI -> UI emits `InputSubmit` -> Event Bus -> Network catches event -> Encrypts & Sends via QUIC -> Receiver Network catches packet -> Decrypts -> Emits `MessageReceived` -> UI & SQLite DB catch event -> UI renders message & DB saves it.

```text
  +-----------------------------------------------------------+
  |                   QuickChat Event Bus                     |
  |                (tokio::sync::broadcast)                   |
  +----+------------------------+------------------------+----+
       |                        |                        |
+------v-------+        +-------v--------+       +-------v--------+
|              |        |                |       |                |
| quickchat_tui|        | quickchat_net  |       | Plugin Host    |
| (Ratatui)    |        | (QUIC / Noise) |       | (Wasmtime)     |
|              |        |                |       |                |
+------+-------+        +-------+--------+       +-------+--------+
```

---

## 📂 Project Structure

The monorepo is split into logical crates to maximize compile times and separation of concerns:

- `quickchat_cli`: The main executable and CLI argument parser.
- `quickchat_core`: The central event bus, SQLite database logic (`rusqlite`), and shared application state.
- `quickchat_net`: The P2P networking layer handling QUIC streams, mDNS discovery, and Noise encryption.
- `quickchat_tui`: The terminal user interface built with `ratatui` and `crossterm`.
- `quickchat_plugin_host`: The `wasmtime` runtime that securely loads and executes `.wasm` plugins.
- `quickchat_plugin_sdk`: A library containing the macros (`export_plugin!`) and FFI bindings that plugin developers use to build QuickChat extensions.
- `quickchat_types`: The shared Protocol Buffer definitions used to pass data across the network and the WASM boundaries.

---

## 🛡️ Security

QuickChat is built on modern cryptographic standards.

- **Encryption**: Every message is End-to-End Encrypted (E2EE) using the Noise Protocol framework (specifically, the `Noise_XX` handshake pattern utilizing X25519 for key exchange and ChaCha20-Poly1305 for AEAD).
- **Identity**: Identities are tied to locally generated Ed25519 key pairs. There are no central servers holding your private keys.
- **Plugin Sandbox**: Third-party plugins execute inside a restricted WebAssembly System Interface (WASI). By default, plugins have zero access to your local filesystem or network sockets.
- **Peer Discovery**: Discovery happens purely over local multicast DNS (mDNS) or via explicit public key exchange. 
- **Privacy**: The application does not collect telemetry, analytics, or usage data.

---

## 🗺️ Roadmap

QuickChat is actively evolving. Our upcoming priorities focus on improving the core developer experience:

- **Crates.io Publishing**: Publish all QuickChat crates to crates.io for easier installation.
- **Better File Transfer**: Optimized chunking and streaming for large binary files.
- **Themes**: Full TOML-based dynamic theming support for terminal colors.
- **Plugin Ecosystem**: A public registry for discovering and installing community-built WASM plugins.
- **Workspace Improvements**: Support for multiple tabs and collapsible sidebars.
- **Performance**: Optimizing Ratatui render loops and reducing memory allocations during high-throughput networking.
- **Documentation**: Extensive tutorials for building your first WASM plugin.

---

## 🤝 Contributing

We welcome contributions from the community! 

- **Coding Standards**: All Rust code must pass `cargo fmt` and `cargo clippy -- -D warnings`.
- **Testing**: Ensure all unit tests pass (`cargo test`) before submitting a PR.
- **Branch Naming**: Use descriptive prefixes (e.g., `feat/`, `fix/`, `docs/`).
- **Commit Style**: We follow Conventional Commits (e.g., `feat: add new CLI flag`).
- **CI Expectations**: Pull requests will not be merged unless the GitHub Actions CI pipeline is green.

Please read `CONTRIBUTING.md` for more detailed information.

---

## ❤️ Support

QuickChat is an independent, open-source project. Building a secure, cross-platform terminal ecosystem takes time and dedication. 

If this tool improves your workflow or saves you time, consider supporting its continued development. Your contributions help maintain the infrastructure, fund CI runners, and support the creation of core plugins.

---

## 📜 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
