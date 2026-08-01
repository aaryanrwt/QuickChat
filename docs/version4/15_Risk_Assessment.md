# QuickChat V4: Risk Assessment

Transitioning to a 100% open-source, decentralized model introduces new challenges that must be mitigated.

## 1. Network Reliability
*   **Risk:** P2P networks can be flaky, especially behind restrictive corporate firewalls or symmetric NATs.
*   **Mitigation:** The community-hosted Relay Nodes are crucial. We must ensure the relay software is trivial to deploy (Docker) to encourage a robust volunteer network.

## 2. Security and Abuse
*   **Risk:** Decentralized networks can be targets for spam or malicious files.
*   **Mitigation:** Strict TOFU (Trust on First Use) and manual contact verification. No global "search for user" directory. File transfers require explicit acceptance. The WASM sandbox protects against malicious plugins.

## 3. Fragmentation
*   **Risk:** The removal of a central authority might lead to incompatible forks of the protocol.
*   **Mitigation:** Maintain strict protocol versioning and a strong RFC process. The core team must foster a welcoming environment so developers contribute upstream rather than forking.

## 4. Sustainability
*   **Risk:** Without enterprise revenue, funding core development and infrastructure is challenging.
*   **Mitigation:** Lean infrastructure (DHT + volunteer relays) minimizes costs. Implement robust sponsorship models and seek grants for open-source communication infrastructure.
