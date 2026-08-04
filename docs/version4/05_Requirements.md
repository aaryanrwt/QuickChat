# QuickChat V4: Requirements

## Functional Requirements
*   **REQ-F1:** The system must compile to a single, statically linked binary for Linux, macOS, and Windows.
*   **REQ-F2:** All messaging and file transfers must be end-to-end encrypted automatically.
*   **REQ-F3:** The UI must be navigable 100% via keyboard shortcuts.
*   **REQ-F4:** The system must support self-hosted relay servers for offline message queuing.
*   **REQ-F5:** The system must integrate a WASM runtime for executing community plugins.
*   **REQ-F6:** The system must provide generic OpenID/SAML hooks for team identity, without proprietary locks.

## Non-Functional Requirements
*   **Performance:** Message delivery latency on LAN must be < 50ms. TUI render loops must maintain 60 FPS equivalent.
*   **Security:** Cryptographic keys must be generated and stored securely on the local file system, never transmitted unencrypted.
*   **Usability:** Zero mandatory configuration. A user must be able to launch the binary and immediately discover LAN peers.
*   **Open Source Compliance:** All dependencies must have permissive licenses (MIT, Apache 2.0). No GPL/AGPL dependencies that would restrict integration.

## Deprecated Requirements (From V3/Community)
*   *REMOVED:* License key validation mechanisms.
*   *REMOVED:* Telemetry and phone-home analytics for billing.
*   *REMOVED:* Hardcoded restrictions on group chat sizes or plugin installations.

