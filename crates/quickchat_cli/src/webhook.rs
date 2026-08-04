use quickchat_tui::app::AppEvent;
use std::sync::mpsc::Sender;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

pub async fn spawn_webhook_listener(tx: Sender<AppEvent>) {
    let addr = "127.0.0.1:9999";
    let listener = match TcpListener::bind(addr).await {
        Ok(l) => l,
        Err(e) => {
            let _ = tx.send(AppEvent::System(format!(
                "Failed to bind webhook listener on {}: {}",
                addr, e
            )));
            return;
        }
    };

    let _ = tx.send(AppEvent::System(format!(
        "CI/CD Webhook listener running on http://{}",
        addr
    )));

    loop {
        if let Ok((mut socket, _)) = listener.accept().await {
            let tx_clone = tx.clone();
            tokio::spawn(async move {
                let mut buffer = [0; 4096];
                if let Ok(n) = socket.read(&mut buffer).await
                    && n > 0
                {
                    let request = String::from_utf8_lossy(&buffer[..n]);

                    // Extremely basic HTTP parser to extract body
                    let parts: Vec<&str> = request.split("\r\n\r\n").collect();
                    if parts.len() > 1 {
                        let body = parts[1].trim();
                        if !body.is_empty() {
                            let _ = tx_clone
                                .send(AppEvent::System(format!("🔔 [CI/CD Webhook] {}", body)));
                        }
                    }

                    // Send basic 200 OK
                    let response = "HTTP/1.1 200 OK\r\nContent-Length: 2\r\n\r\nOK";
                    let _ = socket.write_all(response.as_bytes()).await;
                }
            });
        }
    }
}
