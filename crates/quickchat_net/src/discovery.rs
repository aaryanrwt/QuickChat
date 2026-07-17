use anyhow::Result;
use mdns_sd::{ServiceDaemon, ServiceEvent, ServiceInfo};
use std::collections::HashMap;
use tokio::sync::mpsc;
use tracing::info;

pub const SERVICE_TYPE: &str = "_quickchat._udp.local.";

pub struct Discovery {
    mdns: ServiceDaemon,
}

impl Discovery {
    pub fn new() -> Result<Self> {
        let mdns = ServiceDaemon::new()?;
        Ok(Self { mdns })
    }

    pub fn register(
        &self,
        instance_name: &str,
        port: u16,
        txt_records: HashMap<String, String>,
    ) -> Result<()> {
        let host_name = format!("{}.local.", instance_name);

        let mut txt = std::collections::HashMap::new();
        for (k, v) in txt_records {
            txt.insert(k, v);
        }

        let service_info =
            ServiceInfo::new(SERVICE_TYPE, instance_name, &host_name, "", port, Some(txt))
                .map_err(|e| anyhow::anyhow!("ServiceInfo error: {}", e))?;

        self.mdns
            .register(service_info)
            .map_err(|e| anyhow::anyhow!("Failed to register mDNS: {}", e))?;
        info!(
            "Registered mDNS service '{}' on port {}",
            instance_name, port
        );
        Ok(())
    }

    pub fn browse(&self) -> Result<mpsc::Receiver<ServiceEvent>> {
        let receiver = self
            .mdns
            .browse(SERVICE_TYPE)
            .map_err(|e| anyhow::anyhow!("Browse error: {}", e))?;
        let (tx, rx) = mpsc::channel(32);

        tokio::spawn(async move {
            while let Ok(event) = receiver.recv() {
                if tx.send(event).await.is_err() {
                    break;
                }
            }
        });

        Ok(rx)
    }
}
