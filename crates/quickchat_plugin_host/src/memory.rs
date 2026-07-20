use anyhow::{anyhow, Result};
use wasmtime::{Caller, Memory};

pub fn read_string(
    caller: &mut Caller<'_, super::runtime::PluginContext>,
    ptr: i32,
    len: i32,
) -> Result<String> {
    let memory = get_memory(caller)?;
    let data = memory.data(caller);

    let ptr = ptr as usize;
    let len = len as usize;

    if ptr + len > data.len() {
        return Err(anyhow!("Memory access out of bounds"));
    }

    let slice = &data[ptr..ptr + len];
    String::from_utf8(slice.to_vec()).map_err(|e| anyhow!("Invalid UTF-8: {}", e))
}

fn get_memory(caller: &mut Caller<'_, super::runtime::PluginContext>) -> Result<Memory> {
    match caller.get_export("memory") {
        Some(wasmtime::Extern::Memory(mem)) => Ok(mem),
        _ => Err(anyhow!("failed to find host memory")),
    }
}
