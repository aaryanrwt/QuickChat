# QuickChat V4: Implementation Strategy

The execution of V4 requires a disciplined, phased approach to migrate away from legacy V3 concepts.

## 1. Codebase Audit & Cleansing
*   Create a clean `v4-dev` branch.
*   Aggressively delete all code related to license checks, free tier enforcement, telemetry, and proprietary SSO integrations.
*   Refactor the core to ensure it compiles without these dependencies.

## 2. Infrastructure Democratization
*   Extract the formerly proprietary "Community Relay" code.
*   Audit it for security, remove any hardcoded dependencies, and publish it as a separate open-source repository (`quickchat-relay`).
*   Publish Dockerfiles and Terraform scripts for easy community deployment.

## 3. Feature Porting & Open Standardization
*   Convert legacy Community Audit Logging into a local, encrypted SQLite log available to all users.
*   Implement generic OpenID Connect (OIDC) and SAML support as an optional module, allowing teams to use Keycloak, Authelia, or any standard provider, replacing the proprietary identity system.

## 4. WASM Plugin Rollout
*   Stabilize the `quickchat-sdk` crate.
*   Rewrite core integrations (e.g., the GitHub integration) as WASM plugins to dogfood the SDK and prove the architecture.

## 5. Community Launch
*   Draft a clear migration guide for V3 users.
*   Publish a manifesto detailing the pivot to a 100% open-source, community-driven model to generate momentum.

