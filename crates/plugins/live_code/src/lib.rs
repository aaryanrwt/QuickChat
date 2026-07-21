#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[unsafe(no_mangle)]
pub extern "C" fn on_message(msg_ptr: *const u8, len: usize) -> *const u8 {
    let msg_slice = unsafe { std::slice::from_raw_parts(msg_ptr, len) };
    let msg_str = String::from_utf8_lossy(msg_slice);

    // Basic logic to intercept a "Live Code Pointer" string like `code://file.rs:42`
    if msg_str.contains("code://") {
        let parts: Vec<&str> = msg_str.split("code://").collect();
        if parts.len() > 1 {
            let file_info = parts[1].split_whitespace().next().unwrap_or("");
            let _ipc_cmd = format!("OPEN_FILE:{}", file_info);
            // In a real plugin SDK, we would call a host-provided function `host_send_ipc`.
            // For now, we simulate the memory return for the host to interpret.
        }
    }

    std::ptr::null()
}

#[unsafe(no_mangle)]
pub extern "C" fn get_plugin_info() -> *const u8 {
    let info = b"Live Code Pointers v1.0";
    info.as_ptr()
}
