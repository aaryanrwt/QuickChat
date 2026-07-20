pub mod host_bindings;

use quickchat_types::proto::ChatMessage;

/// The primary trait that all QuickChat plugins must implement.
pub trait QuickChatPlugin {
    fn initialize(&mut self) {}
    fn on_message_received(&mut self, _message: ChatMessage) {}
    fn on_command(&mut self, _command: &str, _args: &[&str]) {}
}

/// Macro to generate the necessary FFI bindings and export the plugin implementation.
#[macro_export]
macro_rules! export_plugin {
    ($plugin_type:ty) => {
        // Global plugin instance
        static mut PLUGIN: Option<$plugin_type> = None;

        #[no_mangle]
        pub extern "C" fn _initialize() {
            unsafe {
                let mut plugin = <$plugin_type>::default();
                plugin.initialize();
                PLUGIN = Some(plugin);
            }
        }

        #[no_mangle]
        pub extern "C" fn on_message_received(ptr: *mut u8, len: usize) {
            unsafe {
                // Real implementation would deserialize the message from memory
                // let message = deserialize(ptr, len);
                if let Some(ref mut plugin) = PLUGIN {
                    // plugin.on_message_received(message);
                }
            }
        }

        // Memory allocation for the host to pass data into the plugin
        #[no_mangle]
        pub extern "C" fn _allocate(size: usize) -> *mut u8 {
            let mut buffer = Vec::with_capacity(size);
            let ptr = buffer.as_mut_ptr();
            std::mem::forget(buffer); // Prevent deallocation
            ptr
        }

        #[no_mangle]
        pub extern "C" fn _deallocate(ptr: *mut u8, size: usize) {
            unsafe {
                let _ = Vec::from_raw_parts(ptr, 0, size);
            }
        }
    };
}
