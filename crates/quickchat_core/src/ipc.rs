//! Inter-Process Communication (IPC) Socket for Editor Integrations
//!
//! This module opens a local Unix Domain Socket (or TCP on Windows)
//! to allow external editors like Neovim and VSCode to communicate with QuickChat.

use std::error::Error;
use tokio::io::AsyncReadExt;
use tokio::net::TcpListener;

pub struct EditorIpcServer {
    port: u16,
}

impl EditorIpcServer {
    pub fn new(port: u16) -> Self {
        Self { port }
    }

    pub async fn start(&self) -> Result<(), Box<dyn Error>> {
        let addr = format!("127.0.0.1:{}", self.port);
        let listener = TcpListener::bind(&addr).await?;
        println!("Editor IPC server listening on {}", addr);

        tokio::spawn(async move {
            while let Ok((mut socket, _)) = listener.accept().await {
                let mut buf = [0; 1024];
                if let Ok(n) = socket.read(&mut buf).await {
                    if n == 0 {
                        return;
                    }
                    let cmd = String::from_utf8_lossy(&buf[..n]);
                    if cmd.starts_with("OPEN_FILE:") {
                        let file_path = cmd.trim_start_matches("OPEN_FILE:").trim();
                        println!("IPC Command received: Open file in editor: {}", file_path);

                        // Execute the host OS command to open the editor
                        #[cfg(target_os = "windows")]
                        let _ = std::process::Command::new("cmd")
                            .args(["/C", "code", file_path])
                            .spawn();

                        #[cfg(not(target_os = "windows"))]
                        let _ = std::process::Command::new("code").arg(file_path).spawn();
                    }
                }
            }
        });

        Ok(())
    }
}
