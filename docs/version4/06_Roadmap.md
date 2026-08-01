# QuickChat V4: Roadmap

The V4 roadmap represents the transition to a purely community-driven open-source model.

## Phase 1: The Great Open-Sourcing
*   Audit the V3 codebase to strip all license checks, telemetry, and paid-tier gates.
*   Open-source the relay server repository and DHT implementation.
*   Establish the V4 governance model and community contribution guidelines.

## Phase 2: Decentralization & Reliability
*   Deploy the global DHT for peer discovery, reducing reliance on manual IP exchange.
*   Release the official Docker image for self-hosting relay nodes.
*   Implement asynchronous messaging (queueing encrypted messages on relays when peers are offline).

## Phase 3: The Plugin Ecosystem
*   Finalize the WASM Plugin SDK.
*   Launch the decentralized plugin registry.
*   Release official open-source plugins for Git, Docker, and Jira.

## Phase 4: Local AI Integration
*   Native integration with local LLM providers (Ollama, Llama.cpp).
*   Context-aware AI for summarizing terminal outputs and code blocks directly in chat, with zero data leaving the local machine.

## Phase 5: Federation
*   Enable distinct self-hosted instances (e.g., a company's internal relay network) to federate securely with the global public network, similar to Matrix.
