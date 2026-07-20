# QuickChat Version 2: Enterprise Features

## 1. Executive Summary

To achieve sustainability and fulfill the "Open Core" monetization strategy, QuickChat V2 must deliver features tailored to the stringent security, compliance, and management requirements of enterprise IT environments, without compromising the decentralized ethos of the open-source core.

## 2. Identity Management & SSO Integration

### 2.1. The Enterprise Requirement
Enterprises cannot rely on individual users managing private key files on their local machines. They require centralized identity management, onboarding, and offboarding via Single Sign-On (SSO).

### 2.2. V2 Enterprise Architecture
- **Federated Identity Bridge:** QuickChat Enterprise will ship with a "Directory Bridge" server.
- **OIDC/SAML Integration:** This server integrates with Azure AD, Okta, or Google Workspace.
- **Key Escrow/Derivation:** When a user logs in via SSO, the QuickChat client receives a JWT. The client uses the JWT material (or a password derived from it) to securely fetch or derive their X25519/Ed25519 keys from a managed Key Management Service (KMS) or Enterprise Escrow.
- **Public Key Directory:** The Directory Bridge maintains a verifiable registry of internal employee public keys, bypassing the need for manual TOFU (Trust on First Use) within the organization.

## 3. Compliance and Audit Logging

### 3.1. The Enterprise Requirement
Regulated industries (Finance, Healthcare) require tamper-evident audit trails of communication metadata (and sometimes content).

### 3.2. V2 Enterprise Architecture
- **Local Tamper-Evident Ledger:** QuickChat clients will log all metadata (who spoke to whom, when, file transfer hashes) to a local, append-only SQLite database.
- **Centralized Log Aggregation:** In an Enterprise deployment, clients will securely stream these logs to a central SIEM (Security Information and Event Management) system via the Managed Relay.
- **Data Loss Prevention (DLP):** Enterprise policies can enforce that all file transfers must be hashed and checked against a central DLP server before the QUIC transfer is allowed to initiate.

## 4. Policy Enforcement & Device Management

### 4.1. The Enterprise Requirement
IT admins must be able to restrict features (e.g., disabling external file transfers, enforcing minimum encryption standards) across all company devices.

### 4.2. V2 Enterprise Architecture
- **Configuration Profiles:** QuickChat V2 will support reading configurations from OS-level Mobile Device Management (MDM) payloads (e.g., macOS Configuration Profiles, Windows Group Policy).
- **Hardcoded Restrictions:** The Enterprise binary can be compiled to strictly enforce MDM policies, disabling specific plugins or preventing connections to non-federated external public keys.

## 5. Managed Relays & Hybrid Cloud

Enterprises often have strict firewall rules blocking standard UDP P2P traffic.
- QuickChat will offer Enterprise customers a dedicated, SLA-backed, high-throughput TURN relay network.
- Alternatively, enterprises can deploy the QuickChat Relay Docker image inside their own DMZ, ensuring all internal communication stays on-premise while allowing external contractor communication through the controlled relay.
