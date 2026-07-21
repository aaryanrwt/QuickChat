# QuickChat Relay Dockerfile (V3)
# Used for decentralized self-hosting

FROM rust:1.96-slim AS builder

WORKDIR /usr/src/quickchat
COPY . .

# Build the relay daemon
RUN cargo build --release -p quickchat_relay

# Final minimal image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/quickchat/target/release/quickchat_relay /usr/local/bin/quickchat_relay

# Default DHT Port
EXPOSE 4001

ENTRYPOINT ["quickchat_relay"]
