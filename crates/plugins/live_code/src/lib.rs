use quickchat_plugin_sdk::{Plugin, Message, Action};

#[no_mangle]
pub extern "C" fn on_message(msg_ptr: *const u8, len: usize) -> *const u8 {
    // Basic logic to intercept a "Live Code Pointer" string like `code://file.rs:42`
    // and trigger an IPC action to the host.
    std::ptr::null()
}

#[no_mangle]
pub extern "C" fn get_plugin_info() -> *const u8 {
    let info = b"Live Code Pointers v1.0";
    info.as_ptr()
}
