use quickchat_types::proto::ChatMessage;
use tokio::sync::broadcast;

#[derive(Clone, Debug)]
pub enum CoreEvent {
    MessageReceived(ChatMessage),
    MessageStatusUpdated(String, String),
    ContactPresenceUpdated(String, String),
    PluginEvent(String, Vec<u8>),
}

pub struct EventBus {
    sender: broadcast::Sender<CoreEvent>,
}

impl EventBus {
    pub fn new(capacity: usize) -> Self {
        let (sender, _) = broadcast::channel(capacity);
        Self { sender }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<CoreEvent> {
        self.sender.subscribe()
    }

    pub fn publish(
        &self,
        event: CoreEvent,
    ) -> Result<usize, broadcast::error::SendError<CoreEvent>> {
        self.sender.send(event)
    }
}
