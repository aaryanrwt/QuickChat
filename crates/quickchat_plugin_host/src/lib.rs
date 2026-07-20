use anyhow::Result;
use std::path::Path;

pub mod memory;
pub mod runtime;

pub use runtime::PluginManager;

/// Opaque handle to a loaded plugin.
pub struct Plugin {
    pub id: String,
}

/// Execution output from a plugin.
pub struct Output {
    pub stdout: String,
    pub stderr: String,
    pub status: i32,
}

/// Abstract interface for executing plugins, decoupling QuickChat from Wasmtime directly.
pub trait PluginRuntime {
    fn load(&mut self, path: &Path) -> Result<Plugin>;
    fn execute(&mut self, plugin: &Plugin, command: &str) -> Result<Output>;
}
