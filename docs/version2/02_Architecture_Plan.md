# QuickChat Version 2: Architecture Plan

## 1. Introduction

The QuickChat V2 architecture must evolve to support extreme extensibility (WASM Plugins), enterprise-scale deployment (Managed Relays, SSO), and asynchronous multi-party collaboration. This document outlines the proposed architectural changes to the existing Rust monorepo.

## 2. Monorepo Evolution

The V1 monorepo strategy (crates: `quickchat_core`, `quickchat_tui`, `quickchat_net`, `quickchat_cli`, `quickchat_types`) remains valid but requires expansion and strict boundary enforcement.

### 2.1. New Crates
- **`quickchat_plugin_host`**: A new crate dedicated to managing the WASM runtime (e.g., using `wasmtime` or `wasmer`). It isolates the complex logic of memory management, sandboxing, and host-plugin bindings from the core business logic.
- **`quickchat_plugin_sdk`**: A library crate published to `crates.io`. Plugin developers will depend on this crate to build QuickChat plugins in Rust. It provides the high-level macros and FFI bindings.
- **`quickchat_enterprise` (Proprietary/Closed Source)**: An optional crate containing proprietary enterprise features (SSO, Audit Logging, LDAP). This aligns with the "Open Core" monetization strategy. The open-source `quickchat_core` will define trait boundaries that this crate implements.

## 3. Core Architecture Paradigms

### 3.1. Event-Driven Architecture (EDA)
The internal API must double down on event-driven principles. In V2, the `quickchat_core` becomes an event bus.
- **Event Producers:** `quickchat_net` (incoming messages), `quickchat_tui` (user input), and `quickchat_plugin_host` (plugin actions).
- **Event Consumers:** `quickchat_tui` (UI updates), `quickchat_plugin_host` (dispatching events to subscribed plugins).

### 3.2. Trait-Based Dependency Inversion
To support the "Open Core" model seamlessly, enterprise features must be injected.
```rust
pub trait IdentityProvider: Send + Sync {
    fn authenticate(&self, credentials: AuthContext) -> Result<UserContext, AuthError>;
}
```
The open-source build injects a `LocalIdentityProvider` (using local keys). The enterprise build injects an `OIDCIdentityProvider`.

## 4. Plugin Architecture Lifecycle

The `quickchat_plugin_host` acts as the bridge:
1. **Discovery:** Scans the `~/.config/quickchat/plugins` directory for `.wasm` files.
2. **Validation:** Checks signatures (if enterprise policy requires) and parses manifest files.
3. **Instantiation:** Spawns a new WASM instance (linear memory, sandboxed) for each plugin.
4. **Binding:** Injects Host APIs (`host_send_message`, `host_log`) into the WASM instance.
5. **Event Routing:** Subscribes to the core event bus and routes relevant events (e.g., `MessageReceived`) to the specific plugins that registered for them.

## 5. Network Architecture Overhaul

V2 introduces a hybrid P2P/Relay architecture.
- **Primary Transport:** QUIC + Noise XX (Maintained from V1).
- **Discovery Layer 1 (LAN):** mDNS (Maintained from V1).
- **Discovery Layer 2 (WAN - New):** Kademlia-based DHT integrated into `quickchat_net`.
- **Fallback Transport (New):** If STUN-based direct connection fails, the connection transparently falls back to a TURN-like QuickChat Managed Relay over TLS.

## 6. Data Persistence

V2 requires robust local storage.
- **Current State:** In-memory or basic file-based.
- **V2 State:** Embedded SQLite (via `rusqlite` or `sqlx`) or an embedded key-value store (e.g., `sled`) for persistent message history, contact books, and plugin state. SQLite provides the necessary relational querying capabilities needed for complex chat history searches.
