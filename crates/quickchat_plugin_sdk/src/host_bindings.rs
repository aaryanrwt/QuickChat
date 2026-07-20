// Definitions for functions imported from the QuickChat WASM Host.
#[cfg(target_arch = "wasm32")]
#[link(wasm_import_module = "quickchat_host")]
extern "C" {
    pub fn host_send_message(
        recipient_pubkey_ptr: *const u8,
        recipient_pubkey_len: usize,
        content_ptr: *const u8,
        content_len: usize,
    ) -> i32;
    pub fn host_log(level: i32, message_ptr: *const u8, message_len: usize);
    pub fn host_register_command(
        command_name_ptr: *const u8,
        command_name_len: usize,
        description_ptr: *const u8,
        description_len: usize,
    ) -> i32;
}

/// # Safety
/// This is a mock implementation for non-wasm architectures and is intrinsically safe.
#[cfg(not(target_arch = "wasm32"))]
#[no_mangle]
pub unsafe extern "C" fn host_send_message(
    _a: *const u8,
    _b: usize,
    _c: *const u8,
    _d: usize,
) -> i32 {
    0
}

/// # Safety
/// This is a mock implementation for non-wasm architectures and is intrinsically safe.
#[cfg(not(target_arch = "wasm32"))]
#[no_mangle]
pub unsafe extern "C" fn host_log(_level: i32, _msg: *const u8, _len: usize) {}

/// # Safety
/// This is a mock implementation for non-wasm architectures and is intrinsically safe.
#[cfg(not(target_arch = "wasm32"))]
#[no_mangle]
pub unsafe extern "C" fn host_register_command(
    _a: *const u8,
    _b: usize,
    _c: *const u8,
    _d: usize,
) -> i32 {
    0
}

pub struct PluginHost;

impl PluginHost {
    pub fn log_info(message: &str) {
        unsafe {
            host_log(0, message.as_ptr(), message.len());
        }
    }

    pub fn log_error(message: &str) {
        unsafe {
            host_log(2, message.as_ptr(), message.len());
        }
    }
}
