use quickchat_types::proto::{Envelope, envelope::Payload, ChatMessage};
use serde::{Deserialize, Serialize};

/// Foundational structure for an incoming Matrix room event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatrixEvent {
    pub room_id: String,
    pub sender: String,
    pub content: MatrixContent,
    pub origin_server_ts: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatrixContent {
    pub msgtype: String,
    pub body: String,
}

/// Foundational structure for an incoming ActivityPub object
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityPubPayload {
    pub id: String,
    #[serde(rename = "type")]
    pub object_type: String,
    pub actor: String,
    pub content: String,
}

/// A core trait for bridging alien network payloads into QuickChat's native protobuf format
pub trait FederationTranslator {
    fn translate_to_native(&self) -> anyhow::Result<Envelope>;
}

impl FederationTranslator for MatrixEvent {
    fn translate_to_native(&self) -> anyhow::Result<Envelope> {
        let chat = ChatMessage {
            id: format!("matrix-{}-{}", self.room_id, self.origin_server_ts),
            content: format!("[Matrix: {}] {}", self.sender, self.content.body),
            timestamp: self.origin_server_ts,
            group_id: None,
        };

        Ok(Envelope {
            payload: Some(Payload::ChatMessage(chat)),
        })
    }
}

impl FederationTranslator for ActivityPubPayload {
    fn translate_to_native(&self) -> anyhow::Result<Envelope> {
        let chat = ChatMessage {
            id: self.id.clone(),
            content: format!("[ActivityPub: {}] {}", self.actor, self.content),
            timestamp: 0,
            group_id: None,
        };

        Ok(Envelope {
            payload: Some(Payload::ChatMessage(chat)),
        })
    }
}
