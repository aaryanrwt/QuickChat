# 12: Risk Assessment

## Critical Risks
1. **Cryptographic Liability:** `mls.rs` is mocking encryption (`b"MLS_ENCRYPTED_v1:"`). Deploying this would result in plaintext data transmission over the network.
2. **Missing Enterprise Features:** A failure to deliver the Enterprise Edition and SSO integration completely voids the V3 business strategy outlined in the Research documents.
3. **IPC Vulnerability:** The local TCP IPC socket in `ipc.rs` does not verify the origin of incoming commands.
