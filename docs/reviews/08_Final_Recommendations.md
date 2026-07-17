# Phase 9: Final Recommendations & Codebase Standards

Before writing any code, the engineering team must align on the following standards and philosophies to ensure QuickChat becomes a legendary 100k+ star open-source project.

## 1. Codebase Standards

*   **Error Handling:** Use `thiserror` for library crates (`quickchat_net`, `quickchat_types`) to provide specific, matchable errors. Use `anyhow` in the binary crate (`quickchat_cli`, `quickchat_tui`) for ergonomic error propagation and context. *Never* use `.unwrap()` or `panic!()` in production code; all errors must be handled or logged gracefully.
*   **Logging:** Use the `tracing` crate. It provides structured, async-aware logging. This is critical for debugging complex P2P networking issues.
*   **Testing Philosophy:** 
    *   *Unit Tests:* Core business logic, cryptography, and state machines must have 90%+ coverage.
    *   *Integration Tests:* Crucial for the `quickchat_net` module. Spin up multiple local instances to test discovery and connection edge cases.
*   **Formatting & Linting:** Enforce `rustfmt` and strict `clippy` rules in the CI pipeline. Code that does not pass `cargo clippy -- -D warnings` cannot be merged.
*   **Architecture Principle:** UI is a pure function of State. The `quickchat_core` should manage the definitive state (backed by SQLite). The `quickchat_tui` simply renders that state and sends user actions back to the core. They must remain decoupled.

## 2. Final Architectural Decisions

Based on the deep audit, here are the final mandates before implementation begins:

1.  **Mandatory SQLite Persistence:** Reject the "non-persistent V1" idea. Chat history and contacts must be persisted to a local SQLite database immediately.
2.  **Ditch Double Encryption:** Do not run Noise Protocol *inside* QUIC's TLS 1.3. Use QUIC's native TLS 1.3 with custom self-signed certificates tied to the user's long-term identity keys. This achieves the same security goals with half the overhead.
3.  **Implement PAKE:** Out-of-band base64 key exchange is dead on arrival for user adoption. Implement a short-code exchange (like Magic Wormhole) to bootstrap trust and exchange STUN/Relay IPs.
4.  **Acknowledge the Need for a Relay:** P2P is great, but Symmetric NATs break it. V1 must ship with a lightweight signaling/relay server that clients fall back to when direct connection fails.
5.  **Trim the Fat:** Cut voice notes, temporary rooms, and inline image rendering from V1. Focus entirely on rock-solid text, Markdown, code pointers, and file transfers.

## 3. Conclusion

QuickChat has the potential to be a category-defining tool. By pivoting slightly away from overly complex distributed systems edge-cases and focusing ruthlessly on reliability, persistence, and developer UX (Unix pipes, short-codes), V1 will lay a solid, scalable foundation for massive open-source growth. 

*Approval of this analysis is required before proceeding to Milestone 1.*
