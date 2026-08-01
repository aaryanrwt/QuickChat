# QuickChat V4: Developer Experience (DX)

QuickChat V4 is built *by* developers, *for* developers. The DX extends both to the users of the application and the contributors building it.

## 1. User DX (Features for Devs)
*   **Live Code Pointers:** Instead of copy-pasting code, send a pointer (e.g., `src/main.rs:45-50`). When the recipient clicks or activates it, it opens in their configured editor (Neovim, VS Code).
*   **Terminal Output Sharing:** Native commands to pipe `stdout` and `stderr` directly into a chat. `cargo build 2>&1 | quickchat send @alice`.
*   **Ephemeral Rooms:** Quickly spin up a temporary chat room for a quick debugging session, which self-destructs when everyone leaves.
*   **Clipboard Sync:** Securely push the local clipboard contents to a trusted peer's clipboard.

## 2. Contributor DX (Building QuickChat)
*   **Single Command Setup:** Contributing should require only `cargo run`. No complex databases or microservices to configure locally.
*   **Monorepo Structure:** The Rust codebase is organized as a Cargo workspace, keeping the core, networking, and CLI modules logically separated but easily accessible.
*   **Clear Plugin SDK:** A well-documented, type-safe Rust SDK for building WASM plugins, complete with templates and examples.
