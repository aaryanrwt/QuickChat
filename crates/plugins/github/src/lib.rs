use quickchat_plugin_sdk::{QuickChatPlugin, export_plugin};
use quickchat_types::proto::ChatMessage;

#[derive(Default)]
pub struct Plugin;

impl QuickChatPlugin for Plugin {
    fn on_command(&mut self, command: &str, _args: &[&str]) {
        if command == "github" {
            // Placeholder logic for github
        }
    }
}

export_plugin!(Plugin);
