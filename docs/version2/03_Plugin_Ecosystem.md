# QuickChat Version 2: Plugin Ecosystem Specification

## 1. Executive Summary

The most transformative feature of QuickChat V2 is the WebAssembly (WASM) Plugin Ecosystem. This allows the community and enterprise users to extend QuickChat's capabilities (e.g., GitHub PR reviews, CI/CD monitoring, Jira ticketing) without modifying the core Rust binary. This document details the technical implementation of the SDK and Host environment.

## 2. WASM Runtime Selection

QuickChat will utilize **Wasmtime** (developed by the Bytecode Alliance) as the embedded runtime within the `quickchat_plugin_host` crate.
- **Why Wasmtime?** It is written in Rust, highly optimized, secure, and actively developed. It strongly supports the WebAssembly System Interface (WASI), which is crucial for managing plugin capabilities.

## 3. Host-Plugin Communication Model

### 3.1. Linear Shared Memory
WASM instances cannot directly access the host's memory. Communication relies on a shared linear memory buffer.
- The Host allocates memory inside the Plugin's memory space using a plugin-exported `_allocate(size)` function.
- The Host writes serialized data to this pointer, then calls the Plugin's event handler, passing the pointer and length.
- The Plugin reads the data, deserializes it, processes it, and frees the memory using `_deallocate(ptr, size)`.

### 3.2. Serialization: Protocol Buffers
To ensure ABI stability and cross-language plugin support (Rust, Go, AssemblyScript), all complex data structures passed across the WASM boundary will be serialized using Protocol Buffers (`quickchat_types` will generate the `.proto` files).

## 4. The Plugin SDK (`quickchat_plugin_sdk`)

The SDK provides ergonomic Rust macros to abstract away the FFI (Foreign Function Interface) complexity for plugin developers.

### 4.1. Core Traits

Plugin developers will implement a primary trait:

```rust
pub trait QuickChatPlugin {
    fn on_initialize(&mut self, context: &mut PluginContext);
    fn on_message_received(&mut self, message: Message);
    fn on_command(&mut self, command: &str, args: Vec<String>) -> Result<CommandResponse, PluginError>;
}
```

### 4.2. Host APIs (Imports)

The Host exposes specific capabilities to the plugins via WASM imports.
- `host_send_message(pubkey_ptr, msg_ptr)`
- `host_register_command(name_ptr, desc_ptr)`
- `host_log(level, msg_ptr)`
- `host_get_config(key_ptr)`

## 5. Security & Sandboxing

Executing third-party code requires strict security boundaries.

### 5.1. Capability-Based Security (WASI)
Using WASI, the plugin host will heavily restrict the plugin's access to the system.
- **File System:** Denied by default. Plugins can only access a specific sandboxed directory (e.g., `~/.config/quickchat/plugin_data/<plugin_name>/`).
- **Network:** Denied by default. Plugins cannot open raw TCP/UDP sockets. They must use the `host_http_request` API provided by the host, which can be audited and rate-limited.

### 5.2. Granular Permissions Model
Upon installation, plugins must declare a manifest (`plugin.toml`):
```toml
name = "github-integration"
version = "1.0.0"

[permissions]
network = ["api.github.com"]
commands = ["/pr", "/issue"]
read_messages = false
```
The user (or Enterprise Admin) must explicitly grant these permissions. If a plugin attempts an action outside its manifest, the host immediately terminates the WASM instance.

## 6. Development Workflow

1. Developer creates a new Rust lib crate.
2. Adds `quickchat_plugin_sdk` as a dependency.
3. Implements the `QuickChatPlugin` trait.
4. Compiles using `cargo build --target wasm32-wasi`.
5. Places the output `.wasm` file into the local QuickChat plugins directory for testing.
