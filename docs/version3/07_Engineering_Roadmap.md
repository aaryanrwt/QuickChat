# 07: Engineering Roadmap

## High-Level Engineering Strategy for V3

This roadmap outlines the technical progression of QuickChat V3 over a projected 9-month development cycle. The focus is exclusively on open-source community needs, prioritizing decentralization, extensibility, and stability.

### Phase 1: Core Networking & Protocol Maturation (Months 1-3)
- **Objective:** Evolve the QUIC and Noise protocol stack to support multi-party encryption and asynchronous messaging.
- **Key Deliverables:**
  - Transition from 1-on-1 Noise XX handshakes to group-key ratcheting (e.g., implementing an adaptation of Signal's Sender Key protocol or MLS - Messaging Layer Security).
  - Develop the `quickchat-relay` reference implementation for store-and-forward message delivery.
  - Initial integration of Kademlia DHT for decentralized peer discovery, eliminating the need for strict LAN/STUN assumptions.

### Phase 2: Plugin Ecosystem & `PluginRuntime` (Months 4-6)
- **Objective:** Solidify the WASM plugin infrastructure to allow the community to build arbitrary integrations safely.
- **Key Deliverables:**
  - Expand the `PluginRuntime` trait to include strict, capability-based filesystem and network access controls (WASI Preview 2 alignment).
  - Launch an open-source decentralized plugin registry protocol.
  - Develop reference plugins for Neovim/VSCode (Live Code Pointers) and Local LLMs (Ollama integration).

### Phase 3: TUI Modernization & Workflows (Months 7-8)
- **Objective:** Overhaul the Terminal UI to support complex workflows like persistent group chats and shared terminal sessions.
- **Key Deliverables:**
  - Implement a highly responsive, multi-pane Ratatui layout engine with dynamic resizing and theme support.
  - Native integration for streaming `stdout`/`stderr` from subprocesses into specific chat channels (Shared Terminal Sessions).
  - Advanced message history search utilizing a local embedded database (e.g., SQLite or Sled).

### Phase 4: Stabilization & Community Beta (Month 9)
- **Objective:** Finalize APIs, harden security, and prepare for a stable V3 release.
- **Key Deliverables:**
  - Comprehensive security audit of the DHT and group messaging protocols.
  - Finalize the Plugin SDK stable API (v1.0).
  - Launch the public community beta test.
