//! Database Module for persistent message history
//!
//! Uses `rusqlite` to store E2EE decrypted messages locally.

use rusqlite::{Connection, Result};

pub struct Contact {
    pub public_key: Vec<u8>,
    pub alias: String,
}

pub struct ChatDatabase {
    conn: Connection,
}

impl ChatDatabase {
    pub fn new(db_path: &str) -> Result<Self> {
        let conn = Connection::open(db_path)?;

        // Initialize schema for persistent group chats (V3)
        conn.execute(
            "CREATE TABLE IF NOT EXISTS messages (
                id INTEGER PRIMARY KEY,
                group_id TEXT NOT NULL,
                sender_id TEXT NOT NULL,
                content TEXT NOT NULL,
                timestamp DATETIME DEFAULT CURRENT_TIMESTAMP
            )",
            (),
        )?;

        Ok(Self { conn })
    }

    pub fn insert_message(&self, group_id: &str, sender_id: &str, content: &str) -> Result<()> {
        self.conn.execute(
            "INSERT INTO messages (group_id, sender_id, content) VALUES (?1, ?2, ?3)",
            (group_id, sender_id, content),
        )?;
        Ok(())
    }

    pub fn get_messages(&self, group_id: &str) -> Result<Vec<String>> {
        let mut stmt = self.conn.prepare(
            "SELECT sender_id, content FROM messages WHERE group_id = ?1 ORDER BY timestamp ASC",
        )?;
        let msg_iter = stmt.query_map([group_id], |row| {
            let sender: String = row.get(0)?;
            let content: String = row.get(1)?;
            Ok(format!("{}: {}", sender, content))
        })?;

        let mut messages = Vec::new();
        for msg in msg_iter {
            messages.push(msg?);
        }
        Ok(messages)
    }
}
