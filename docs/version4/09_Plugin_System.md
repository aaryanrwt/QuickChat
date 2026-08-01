# QuickChat V4: Plugin System

The V4 Plugin System is the engine for infinite, community-driven extensibility, democratizing integrations previously locked behind enterprise paywalls.

## 1. Architecture
*   **WASM Runtime:** Plugins are compiled to WebAssembly (WASM). This provides a secure, sandboxed execution environment that is platform-independent. QuickChat uses `wasmtime` (or similar) as the host runtime.
*   **Language Agnostic:** While Rust is recommended, developers can write plugins in any language that compiles to WASM (Go, AssemblyScript, C++).
*   **RPC Interface:** Plugins communicate with the QuickChat core via a strict RPC interface (e.g., using Protocol Buffers). 

## 2. Capabilities & Permissions
Plugins must request explicit permissions upon installation. 
*   `ui:read` / `ui:write`: Modify the TUI layout or inject custom widgets.
*   `message:read` / `message:send`: Read incoming messages or send messages on behalf of the user.
*   `network:request`: Make external HTTP requests (e.g., to the GitHub API).
*   `fs:read`: Read specific local files (e.g., logs).

## 3. Decentralized Registry
The central plugin marketplace has been eliminated. V4 introduces a decentralized plugin registry based on Git repositories. Users install plugins directly via a GitHub/GitLab URL or a local path.

## 4. Example Use Cases
*   **GitHub/GitLab:** Inline PR reviews, issue unfurling.
*   **Docker:** Managing local containers from the chat TUI.
*   **Local AI:** Piping messages to a local Ollama instance for summarization.
