use anyhow::Result;
use magic_wormhole::{AppConfig, AppID, Code, MailboxConnection, Wormhole};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::str::FromStr;

pub const APP_ID: &str = "quickchat.dev/v1";

#[derive(Serialize, Deserialize, Debug)]
pub struct SignalingMessage {
    pub public_key: String,
    pub address: String, // e.g. "198.51.100.1:8080"
}

pub async fn host_wormhole() -> Result<(Wormhole, Code)> {
    let app_config = AppConfig {
        id: AppID::new(APP_ID),
        rendezvous_url: Cow::Borrowed(magic_wormhole::rendezvous::DEFAULT_RENDEZVOUS_SERVER),
        app_version: (),
    };
    let mailbox = MailboxConnection::create(app_config, 2)
        .await
        .map_err(|e| anyhow::anyhow!("Wormhole error: {:?}", e))?;
    let code = mailbox.code().clone();
    let wormhole = Wormhole::connect(mailbox)
        .await
        .map_err(|e| anyhow::anyhow!("Wormhole error: {:?}", e))?;
    Ok((wormhole, code))
}

pub async fn join_wormhole(code_str: &str) -> Result<Wormhole> {
    let app_config = AppConfig {
        id: AppID::new(APP_ID),
        rendezvous_url: Cow::Borrowed(magic_wormhole::rendezvous::DEFAULT_RENDEZVOUS_SERVER),
        app_version: (),
    };
    let code =
        Code::from_str(code_str).map_err(|e| anyhow::anyhow!("Code parse error: {:?}", e))?;
    let mailbox = MailboxConnection::connect(app_config, code, false)
        .await
        .map_err(|e| anyhow::anyhow!("Wormhole error: {:?}", e))?;
    let wormhole = Wormhole::connect(mailbox)
        .await
        .map_err(|e| anyhow::anyhow!("Wormhole error: {:?}", e))?;
    Ok(wormhole)
}

pub async fn send_signaling(wormhole: &mut Wormhole, msg: &SignalingMessage) -> Result<()> {
    let data = serde_json::to_vec(msg)?;
    wormhole
        .send(data)
        .await
        .map_err(|e| anyhow::anyhow!("Wormhole error: {:?}", e))?;
    Ok(())
}

pub async fn receive_signaling(wormhole: &mut Wormhole) -> Result<SignalingMessage> {
    let data = wormhole
        .receive()
        .await
        .map_err(|e| anyhow::anyhow!("Wormhole error: {:?}", e))?;
    let msg: SignalingMessage = serde_json::from_slice(&data)?;
    Ok(msg)
}
