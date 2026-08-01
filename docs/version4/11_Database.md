# QuickChat V4: Database

QuickChat V4 is designed to minimize local state, but requires efficient, embedded databases for persistent data.

## 1. Embedded Storage (SQLite/Sled)
*   For the V4 client, an embedded, zero-configuration database (like SQLite or a pure-Rust alternative like `sled` or `redb`) is used to store:
    *   **Contacts:** Known public keys, aliases, and trust states.
    *   **Settings:** User preferences and TUI configurations.
    *   **Message History (Optional):** Users can opt-in to logging message history locally.

## 2. Security at Rest
*   All sensitive data stored on disk (particularly message history and contact lists) can be optionally encrypted using a key derived from a user passphrase or OS-level keystore.

## 3. Relay Node Storage
*   Relay servers require temporary storage for offline message queuing (Store-and-Forward).
*   This storage is ephemeral. Relays use an embedded database to hold encrypted message blobs until the recipient comes online, at which point the blobs are delivered and immediately deleted from the relay.
*   Relays do NOT use heavy relational databases (PostgreSQL/MySQL), keeping deployment lightweight and trivial.
