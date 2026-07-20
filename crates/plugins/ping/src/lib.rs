use quickchat_plugin_sdk::{export_plugin, host_bindings::PluginHost, QuickChatPlugin};
use quickchat_types::proto::ChatMessage;

#[derive(Default)]
pub struct PingPlugin;

impl QuickChatPlugin for PingPlugin {
    fn initialize(&mut self) {
        PluginHost::log_info("Ping plugin initialized.");
    }

    fn on_message_received(&mut self, message: ChatMessage) {
        if message.content == "/ping" {
            PluginHost::log_info("Ping received. Responding with Pong!");
            // In a real implementation, we would call host_send_message
            // with the sender's pubkey and "Pong!" as the content.
        }
    }
}

export_plugin!(PingPlugin);
