# Phase 4: Version 1 Audit

The proposed V1 scope is **Too Ambitious and Slightly Misdirected**. It prioritizes complex, edge-case features (like inline images and WASM plugin groundwork) over foundational reliability (NAT traversal and persistence).

## 1. Feature Trimming (What to Cut from V1)
To ensure V1 is "beautiful, stable, memorable, and enjoyable," we must rigorously cut features that add complexity without proportional value.

*   **Cut: Inline Image Rendering.** Terminal support is a nightmare. Relegate this to a V2 plugin.
*   **Cut: Voice Notes.** Compressing and streaming audio in the terminal is cool, but a massive distraction from the core textual/code developer experience.
*   **Cut: Temporary Rooms.** Focus 100% on 1-on-1 direct messaging and file transfer first. Multi-party P2P state synchronization is a remarkably hard distributed systems problem (requires CRDTs or consensus).

## 2. Feature Additions (What to Add to V1)
*   **Add: PAKE-based Contact Exchange.** As mentioned in the Architecture review, out-of-band copy-pasting of STUN IP addresses and Public Keys is unacceptable. Implement a short-code exchange (like Magic Wormhole) to bootstrap the long-term trust relationship.
*   **Add: SQLite Persistence.** Messages and contacts must survive an app restart. 
*   **Add: Local Signaling/Relay Server.** Provide a lightweight, open-source signaling server that QuickChat instances connect to by default. This solves the WAN discovery and Symmetric NAT problems instantly, providing the "Zero-Config" experience promised in the PRD.

## 3. CLI vs. TUI Experience
The `quickchat share output` command is brilliant and uniquely caters to developers. 
However, the CLI documentation proposes: `quickchat send rahul "message"`.
*Improvement:* Integrate this deeply with standard Unix pipes. 
`cat error.log | quickchat send rahul --code-block="log"`
This should be the primary marketing hook: native integration into existing developer workflows.

## 4. Observability and Developer Experience
If V1 fails to connect, users need to know why.
*   Implement a `quickchat doctor` command that tests network connectivity, STUN/TURN resolution, and database health.
*   Use the `tracing` crate for structured logging, allowing users to easily share debug logs when opening GitHub issues.
