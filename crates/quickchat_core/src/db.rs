pub use rusqlite::Connection;
use rusqlite::Result;
use std::path::Path;

pub fn init_db(db_path: &Path) -> Result<Connection> {
    let conn = Connection::open(db_path)?;

    // Create tables
    conn.execute(
        "CREATE TABLE IF NOT EXISTS identity (
            id INTEGER PRIMARY KEY CHECK (id = 1),
            ed25519_private_key BLOB NOT NULL,
            x25519_private_key BLOB NOT NULL
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS contacts (
            public_key BLOB PRIMARY KEY,
            alias TEXT NOT NULL,
            is_trusted BOOLEAN NOT NULL DEFAULT 0,
            last_seen DATETIME
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS messages (
            id TEXT PRIMARY KEY,
            sender_id BLOB NOT NULL,
            recipient_id BLOB NOT NULL,
            timestamp DATETIME NOT NULL,
            content BLOB NOT NULL,
            status TEXT NOT NULL
        )",
        [],
    )?;

    Ok(conn)
}

pub struct Contact {
    pub public_key: Vec<u8>,
    pub alias: String,
    pub is_trusted: bool,
    pub last_seen: Option<String>,
}

pub struct ChatMessageDb {
    pub id: String,
    pub sender_id: Vec<u8>,
    pub recipient_id: Vec<u8>,
    pub timestamp: i64,
    pub content: String,
    pub status: String,
}

pub fn upsert_contact(conn: &Connection, public_key: &[u8], alias: &str) -> Result<()> {
    conn.execute(
        "INSERT INTO contacts (public_key, alias, is_trusted, last_seen)
         VALUES (?1, ?2, 0, datetime('now'))
         ON CONFLICT(public_key) DO UPDATE SET alias = excluded.alias, last_seen = datetime('now')",
        rusqlite::params![public_key, alias],
    )?;
    Ok(())
}

pub fn get_contacts(conn: &Connection) -> Result<Vec<Contact>> {
    let mut stmt = conn.prepare(
        "SELECT public_key, alias, is_trusted, last_seen FROM contacts ORDER BY last_seen DESC",
    )?;
    let contact_iter = stmt.query_map([], |row| {
        Ok(Contact {
            public_key: row.get(0)?,
            alias: row.get(1)?,
            is_trusted: row.get(2)?,
            last_seen: row.get(3)?,
        })
    })?;

    let mut contacts = Vec::new();
    for contact in contact_iter {
        contacts.push(contact?);
    }
    Ok(contacts)
}

pub fn insert_message(conn: &Connection, msg: &ChatMessageDb) -> Result<()> {
    conn.execute(
        "INSERT INTO messages (id, sender_id, recipient_id, timestamp, content, status)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        rusqlite::params![
            msg.id,
            msg.sender_id,
            msg.recipient_id,
            msg.timestamp,
            msg.content.as_bytes(),
            msg.status
        ],
    )?;
    Ok(())
}

pub fn get_messages_for_contact(
    conn: &Connection,
    contact_pubkey: &[u8],
) -> Result<Vec<ChatMessageDb>> {
    let mut stmt = conn.prepare(
        "SELECT id, sender_id, recipient_id, timestamp, content, status 
         FROM messages 
         WHERE sender_id = ?1 OR recipient_id = ?1 
         ORDER BY timestamp ASC",
    )?;
    let msg_iter = stmt.query_map(rusqlite::params![contact_pubkey], |row| {
        let content_blob: Vec<u8> = row.get(4)?;
        Ok(ChatMessageDb {
            id: row.get(0)?,
            sender_id: row.get(1)?,
            recipient_id: row.get(2)?,
            timestamp: row.get(3)?,
            content: String::from_utf8_lossy(&content_blob).into_owned(),
            status: row.get(5)?,
        })
    })?;

    let mut msgs = Vec::new();
    for msg in msg_iter {
        msgs.push(msg?);
    }
    Ok(msgs)
}
