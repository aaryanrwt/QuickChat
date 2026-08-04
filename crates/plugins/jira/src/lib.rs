use quickchat_plugin_sdk::{QuickChatPlugin, export_plugin};

#[derive(Default)]
pub struct Plugin;

impl QuickChatPlugin for Plugin {
    fn on_command(&mut self, command: &str, _args: &[&str]) {
        if command == "jira" {
            // Placeholder logic for jira
        }
    }
}

export_plugin!(Plugin);
