use anyhow::{Context, Result};
use std::path::PathBuf;
use std::process::Command;

/// An open-source, decentralized plugin registry.
pub struct DecentralizedRegistry {
    pub base_url: String,
}

impl DecentralizedRegistry {
    pub fn new(url: &str) -> Self {
        Self {
            base_url: url.to_string(),
        }
    }

    /// Fetches a community plugin from the open registry using Git.
    pub async fn download_plugin(
        &self,
        plugin_id: &str,
        out_dir: &std::path::Path,
    ) -> Result<PathBuf> {
        let plugin_repo = format!("{}/{}", self.base_url, plugin_id);
        let target_dir = out_dir.join(plugin_id);

        println!(
            "Cloning plugin {} from decentralized git registry {}...",
            plugin_id, plugin_repo
        );

        let status = Command::new("git")
            .arg("clone")
            .arg(&plugin_repo)
            .arg(&target_dir)
            .status()
            .context("Failed to execute git clone")?;

        if !status.success() {
            anyhow::bail!("Failed to clone plugin repository: {}", plugin_id);
        }

        let file_path = target_dir.join(format!("{}.wasm", plugin_id));
        Ok(file_path)
    }
}
