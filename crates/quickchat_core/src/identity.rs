use ed25519_dalek::{SigningKey, VerifyingKey};
use getrandom::fill;
use rusqlite::{Connection, OptionalExtension};
use thiserror::Error;
use x25519_dalek::{PublicKey as X25519PublicKey, StaticSecret};

#[derive(Error, Debug)]
pub enum IdentityError {
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),
    #[error("Invalid key length")]
    InvalidKeyLength,
}

pub struct Identity {
    pub ed25519_signing_key: SigningKey,
    pub x25519_static_secret: StaticSecret,
}

impl Identity {
    pub fn generate() -> Self {
        let mut ed_bytes = [0u8; 32];
        fill(&mut ed_bytes).expect("Failed to get random bytes");
        let ed25519_signing_key = SigningKey::from_bytes(&ed_bytes);

        let mut x_bytes = [0u8; 32];
        fill(&mut x_bytes).expect("Failed to get random bytes");
        let x25519_static_secret = StaticSecret::from(x_bytes);

        Self {
            ed25519_signing_key,
            x25519_static_secret,
        }
    }

    pub fn ed25519_public_key(&self) -> VerifyingKey {
        self.ed25519_signing_key.verifying_key()
    }

    pub fn x25519_public_key(&self) -> X25519PublicKey {
        X25519PublicKey::from(&self.x25519_static_secret)
    }

    pub fn load_or_create(conn: &Connection) -> Result<Self, IdentityError> {
        let mut stmt = conn
            .prepare("SELECT ed25519_private_key, x25519_private_key FROM identity WHERE id = 1")?;
        let existing = stmt
            .query_row([], |row| {
                let ed_blob: Vec<u8> = row.get(0)?;
                let x_blob: Vec<u8> = row.get(1)?;
                Ok((ed_blob, x_blob))
            })
            .optional()?;

        match existing {
            Some((ed_blob, x_blob)) => {
                if ed_blob.len() != 32 || x_blob.len() != 32 {
                    return Err(IdentityError::InvalidKeyLength);
                }
                let mut ed_arr = [0u8; 32];
                ed_arr.copy_from_slice(&ed_blob);
                let ed25519_signing_key = SigningKey::from_bytes(&ed_arr);

                let mut x_arr = [0u8; 32];
                x_arr.copy_from_slice(&x_blob);
                let x25519_static_secret = StaticSecret::from(x_arr);

                Ok(Self {
                    ed25519_signing_key,
                    x25519_static_secret,
                })
            }
            None => {
                let new_identity = Self::generate();
                conn.execute(
                    "INSERT INTO identity (id, ed25519_private_key, x25519_private_key) VALUES (1, ?1, ?2)",
                    (
                        new_identity.ed25519_signing_key.to_bytes(),
                        new_identity.x25519_static_secret.to_bytes(),
                    ),
                )?;
                Ok(new_identity)
            }
        }
    }
}
