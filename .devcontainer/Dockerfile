FROM mcr.microsoft.com/devcontainers/rust:1-1-bullseye

# Install extra tools for WebAssembly and Security auditing
RUN cargo install cargo-audit
RUN cargo install wasm-pack
RUN rustup target add wasm32-wasi

# Install system dependencies if required for OpenSSL or Networking
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends libssl-dev pkg-config cmake \
    && apt-get clean -y && rm -rf /var/lib/apt/lists/*
