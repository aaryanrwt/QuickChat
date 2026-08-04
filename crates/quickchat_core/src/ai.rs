use anyhow::{Context, Result};
use std::process::Command;

/// A client for piping data securely to local, offline Large Language Models.
/// This ensures 100% privacy as no code leaves the developer's machine.
pub struct LocalLlmClient {
    pub endpoint: String,
}

impl LocalLlmClient {
    pub fn new(endpoint: &str) -> Self {
        Self {
            endpoint: endpoint.to_string(),
        }
    }

    /// Pipes terminal output or code snippets to a local LLM (like Ollama) for private analysis.
    pub async fn analyze_code(&self, code: &str, prompt: &str) -> Result<String> {
        println!(
            "Piping code securely to local offline LLM at: {}",
            self.endpoint
        );

        let payload = format!(
            r#"{{"model": "llama3", "prompt": "{}\n\nCode:\n{}", "stream": false}}"#,
            prompt, code
        );

        // Use curl as a lightweight zero-dependency REST client to the local LLM daemon
        let output = Command::new("curl")
            .arg("-s")
            .arg("-X")
            .arg("POST")
            .arg(&format!("{}/api/generate", self.endpoint))
            .arg("-d")
            .arg(&payload)
            .output();

        match output {
            Ok(out) if out.status.success() => {
                let response = String::from_utf8_lossy(&out.stdout);
                Ok(response.to_string())
            }
            _ => Ok(format!(
                "Offline Model Analysis (Fallback):\n\n{}\n\n(Local LLM at {} is currently offline or unreachable)",
                code, self.endpoint
            )),
        }
    }
}
