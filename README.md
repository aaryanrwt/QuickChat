<div align="center">
  <h1>QuickChat ⚡</h1>
  <p>
    <strong>A blazing fast, serverless, end-to-end encrypted Terminal Chat & File Transfer CLI built in Rust.</strong>
  </p>

  <p>
    <a href="https://github.com/aaryanrwt/QuickChat/actions"><img src="https://img.shields.io/github/actions/workflow/status/aaryanrwt/QuickChat/ci.yml?branch=main" alt="Build Status"></a>
    <a href="https://github.com/aaryanrwt/QuickChat/blob/main/LICENSE"><img src="https://img.shields.io/badge/license-MIT%20%2F%20Apache--2.0-blue.svg" alt="License"></a>
    <a href="https://crates.io/crates/quickchat_cli"><img src="https://img.shields.io/crates/v/quickchat_cli.svg" alt="Crates.io"></a>
  </p>
</div>

---

QuickChat is an extremely lightweight, secure, peer-to-peer terminal chat application built for developers. It requires zero configuration, no central servers, and works seamlessly over LAN and WAN. 

## ✨ Features

- **Serverless LAN Discovery:** Nodes find each other instantly on local networks using `mDNS`.
- **WAN Bootstrapping:** Connect over the internet effortlessly using Magic Wormhole short-codes.
- **End-to-End Encrypted:** Mutual authentication and encryption powered by QUIC (`quinn`) and X25519/Ed25519.
- **Blazing Fast File Transfers:** Send files of any size out-of-band via QUIC unistreams, with on-the-fly `zstd` compression.
- **Developer First IPC:** Pipe terminal output directly into chats (`make test | quickchat send peer --stdin`) and share code pointers (`quickchat share pointer src/main.rs:42`).
- **Beautiful TUI:** Smooth, asynchronous Terminal UI built with `ratatui`.



## 🚀 Installation

### Prerequisites
- [Rust](https://rustup.rs/) (1.75+)

### Build from Source
```bash
git clone https://github.com/aaryanrwt/QuickChat.git
cd QuickChat
cargo build --release -p quickchat_cli
```
The binary will be located at `target/release/quickchat_cli`.

## 💻 Usage

Start QuickChat in TUI mode:
```bash
quickchat_cli
```

### QuickChat CLI Commands
QuickChat provides a suite of CLI tools that allow you to interact with your running QuickChat TUI daemon directly from your standard terminal:

- **`quickchat_cli`**
  Starts the main Terminal UI (TUI) and background daemon. Run this first to start the application.

- **`quickchat_cli id`**
  Generates and securely stores a new cryptographic identity (Ed25519/X25519) in the local SQLite database, displaying your public key.

- **`quickchat_cli send <alias> --stdin`**
  Reads from standard input (`stdin`) and sends the content as a formatted code block to the specified contact.
  *Example:* `cat error.log | quickchat_cli send my_peer --stdin`

- **`quickchat_cli share pointer <file:line>`**
  Highlights and sends a specific line of code from a local file to the running chat instance, beautifully formatted as a markdown pointer.
  *Example:* `quickchat_cli share pointer src/main.rs:180`

### TUI Slash Commands
While using the QuickChat Terminal UI, you can use the following commands directly in the chat input bar:

- **`/file <absolute/path/to/file>`**
  Initiates a blazing fast, out-of-band file transfer. The file is compressed using `zstd` on the fly and sent securely over a dedicated QUIC stream to the active contact.

- **`/host`**
  Generates a secure 3-word phrase (e.g. `4-purple-sausages`) using Magic Wormhole. Share this phrase out-of-band with a peer over the internet to establish a WAN connection.

- **`/join <3-word-phrase>`**
  Connects to a peer over the internet using their generated short-code. This securely exchanges IPs and public keys to punch a hole and bootstrap the QUIC connection!

## 🏗️ Architecture overview

QuickChat is built on a highly concurrent, multi-threaded Rust architecture:
- **`quickchat_cli`**: The main executable and UDP IPC client.
- **`quickchat_tui`**: The asynchronous `ratatui` UI layer.
- **`quickchat_net`**: Manages mDNS discovery, Magic Wormhole PAKE signaling, and QUIC (`quinn`) encrypted streams.
- **`quickchat_core`**: The messaging engine, SQLite identity/history persistence, and file transfer logic.
- **`quickchat_types`**: Protobuf definitions (`prost`).

## 🤝 Support Development

If QuickChat has been useful to you or your team, and you'd like to support its continued development, you can contribute using UPI.

Your support helps improve QuickChat, maintain the project, fix bugs, and build new open-source features.

UPI ID:
```text
aaryanrawat909@oksbi
```

See [SUPPORT.md](SUPPORT.md) for other ways to support the project.

## 🤝 Contributing

Contributions are what make the open-source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

Please see our [CONTRIBUTING.md](CONTRIBUTING.md) for details.

## 🛡️ Security

Please report vulnerabilities following the process outlined in our [SECURITY.md](SECURITY.md).

## 📄 License

This project is dual-licensed under either the [MIT License](LICENSE-MIT) or the [Apache License 2.0](LICENSE-APACHE).
