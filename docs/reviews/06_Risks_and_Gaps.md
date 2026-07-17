# Phase 2 & 5: Implementation Risks and Gaps

A frank assessment of the technical debt, implementation risks, and gaps present in the current documentation suite.

## 1. The NAT Traversal Reality Gap (High Risk)
The biggest risk to the project's success is the assumption that STUN + Out-of-band key exchange equals a working WAN product.
*   **The Reality:** Symmetric NATs alter the external port for every new destination IP. If Peer A is behind a Symmetric NAT, STUN will report `IP:Port_X`. When Peer B tries to connect to `IP:Port_X`, the router drops it.
*   **The Consequence:** Users will try QuickChat, it will fail to connect over the internet, and they will abandon it, labeling it "broken".
*   **The Fix:** QuickChat V1 *must* be accompanied by a lightweight, open-source relay server (TURN-equivalent). The client should attempt direct P2P, but seamlessly fall back to the relay if it fails.

## 2. State Management & Persistence Gap (High Risk)
The EDD states message history is non-persistent for V1, but the PRD positions this against "ephemeral" tools like Wormhole.
*   **The Reality:** If a user accidentally closes their terminal, they lose their chat history and potentially active file transfers.
*   **The Consequence:** Complete loss of user trust.
*   **The Fix:** Integrate `rusqlite`. Define a schema for `Contacts`, `Messages`, and `Transfers`. All incoming/outgoing state must hit the local disk immediately.

## 3. Cryptographic Complexity Risk (Medium Risk)
Implementing Noise (XX) *over* QUIC (which already implements TLS 1.3).
*   **The Reality:** Double encryption adds CPU overhead, battery drain, and implementation complexity.
*   **The Consequence:** Slower file transfers and increased chance of cryptographic implementation bugs.
*   **The Fix:** Use QUIC's native TLS 1.3 with custom certificate validation (mapping certificates to the long-term Ed25519 identity keys). This achieves mutual authentication and E2EE natively.

## 4. Developer Experience (UX) Risk (Medium Risk)
Terminal emulators vary wildly in their capabilities (colors, font rendering, graphics protocols).
*   **The Reality:** Building a "stunning" TUI that works flawlessly across default Windows CMD, Alacritty, iTerm2, and tmux is incredibly difficult.
*   **The Consequence:** Glitchy rendering on older setups.
*   **The Fix:** Stick to robust, widely supported ANSI sequences. Provide a `--basic-ui` flag that strips out complex borders and animations for maximum compatibility.

## 5. Security: Encrypt-Then-Compress vs Compress-Then-Encrypt
*   **The Issue:** The File Transfer protocol specifies compressing, then encrypting. This opens the door to side-channel length-extension attacks (CRIME/BREACH) if the attacker can influence the plaintext.
*   **The Fix:** While standard for file transfers, the documentation should explicitly mandate padding *after* compression to obscure the exact size of the compressed payload.
