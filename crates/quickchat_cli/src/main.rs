use anyhow::Result;
use clap::{Parser, Subcommand};
use quickchat_core::identity::Identity;
use quickchat_net::discovery::Discovery;
use quickchat_net::quic::QuicNode;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use tracing::{error, info};

#[derive(Parser)]
#[command(author, version, about = "QuickChat: The Terminal-Native P2P Communicator", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Show the local QuickChat identity
    Id,
    Serve {
        #[arg(short, long, default_value = "8080")]
        port: u16,
        #[arg(short, long, default_value = "quickchat-dev")]
        name: String,
    },
    /// Send a message or pipe stdin to the running daemon
    Send {
        #[arg(short, long, help = "Read from stdin")]
        stdin: bool,
    },
    /// Share a pointer to a file and line number
    Share {
        #[arg(help = "Pointer format: file:line")]
        pointer: String,
    },
}

fn get_db_path() -> Result<PathBuf> {
    let mut path = dirs::data_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push("quickchat");
    fs::create_dir_all(&path)?;
    path.push("quickchat.db");
    Ok(path)
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().with_env_filter("info").init();

    let cli = Cli::parse();

    let db_path = get_db_path()?;
    let conn = rusqlite::Connection::open(&db_path)?;

    match cli.command {
        Commands::Id => {
            let identity = Identity::load_or_create(&conn)?;
            println!("Local QuickChat Identity");
            println!("------------------------");
            let ed_pub = hex::encode(identity.ed25519_public_key().as_bytes());
            let x_pub = hex::encode(identity.x25519_public_key().as_bytes());
            println!("Ed25519 Public Key: {}", ed_pub);
            println!("X25519 Public Key:  {}", x_pub);
        }
        Commands::Serve { port, name } => {
            let db_path = get_db_path().unwrap();
            let db_path_str = db_path.to_string_lossy().to_string();

            // Dummy rusqlite connection for Identity
            let conn = rusqlite::Connection::open(&db_path).unwrap();
            let identity = Identity::load_or_create(&conn)?;
            let my_ed_pub = hex::encode(identity.ed25519_public_key().as_bytes());

            let addr = format!("0.0.0.0:{}", port).parse()?;
            let quic_node = std::sync::Arc::new(QuicNode::new(addr)?);

            info!("QUIC Node listening on {}", port);

            let (tx, rx) = std::sync::mpsc::channel();
            let (tx_outbound, _) = tokio::sync::broadcast::channel::<String>(100);

            // Accept incoming connections
            let accept_node = quic_node.clone();
            let my_ed_pub_clone = my_ed_pub.clone();
            let tx_accept = tx.clone();
            let tx_outbound_accept = tx_outbound.clone();
            tokio::spawn(async move {
                while let Some(incoming) = accept_node.endpoint.accept().await {
                    match incoming.await {
                        Ok(connection) => {
                            let _ = tx_accept.send(quickchat_tui::app::AppEvent::System(format!(
                                "Accepted connection from {}",
                                connection.remote_address()
                            )));
                            // Handle connection (handshake)
                            tokio::spawn(handle_connection(
                                connection,
                                my_ed_pub_clone.clone(),
                                tx_accept.clone(),
                                tx_outbound_accept.subscribe(),
                            ));
                        }
                        Err(e) => {
                            let _ = tx_accept.send(quickchat_tui::app::AppEvent::System(format!(
                                "Connection failed: {}",
                                e
                            )));
                        }
                    }
                }
            });

            let discovery = Discovery::new()?;

            let mut txt_records = HashMap::new();
            txt_records.insert("pubkey".to_string(), my_ed_pub.clone());

            discovery.register(&name, port, txt_records)?;

            let mut rx_discovery = discovery.browse()?;
            let quic_node_for_discovery = quic_node.clone();
            let my_ed_pub_for_discovery = my_ed_pub.clone();

            let tx_discovery = tx.clone();
            let tx_outbound_discovery = tx_outbound.clone();
            tokio::spawn(async move {
                while let Some(event) = rx_discovery.recv().await {
                    if let mdns_sd::ServiceEvent::ServiceResolved(info) = event {
                        let peer_pubkey = info.get_property_val_str("pubkey");

                        if let Some(peer_pubkey) = peer_pubkey {
                            if peer_pubkey == my_ed_pub_for_discovery {
                                continue; // Don't connect to ourselves
                            }

                            // Tie-breaker: only dial if their pubkey is lexicographically greater
                            if peer_pubkey > my_ed_pub_for_discovery.as_str() {
                                let addrs = info.get_addresses();
                                if let Some(peer_ip) = addrs.iter().next() {
                                    // ScopedIp implements Display, parse it to get an IpAddr if needed, or if it can be directly formatted
                                    let ip_str = peer_ip.to_string();
                                    if let Ok(ip) = ip_str.parse::<std::net::IpAddr>() {
                                        let peer_addr =
                                            std::net::SocketAddr::new(ip, info.get_port());
                                        let _ = tx_discovery.send(
                                            quickchat_tui::app::AppEvent::System(format!(
                                                "Dialing discovered peer {} at {}",
                                                peer_pubkey, peer_addr
                                            )),
                                        );

                                        let quic_node_clone = quic_node_for_discovery.clone();
                                        let my_pub_for_dial = my_ed_pub_for_discovery.clone();
                                        let tx_dial = tx_discovery.clone();
                                        let rx_outbound_dial = tx_outbound_discovery.subscribe();
                                        tokio::spawn(async move {
                                            match quic_node_clone
                                                .connect(peer_addr, "localhost")
                                                .await
                                            {
                                                Ok(connection) => {
                                                    let _ = tx_dial.send(
                                                        quickchat_tui::app::AppEvent::System(
                                                            format!(
                                                                "Successfully connected to {}",
                                                                peer_addr
                                                            ),
                                                        ),
                                                    );
                                                    tokio::spawn(handle_connection(
                                                        connection,
                                                        my_pub_for_dial,
                                                        tx_dial.clone(),
                                                        rx_outbound_dial,
                                                    ));
                                                }
                                                Err(e) => {
                                                    let _ = tx_dial.send(
                                                        quickchat_tui::app::AppEvent::System(
                                                            format!(
                                                                "Failed to connect to {}: {}",
                                                                peer_addr, e
                                                            ),
                                                        ),
                                                    );
                                                }
                                            }
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
            });

            let mut rx_global_outbound = tx_outbound.subscribe();
            let quic_node_for_magic = quic_node.clone();
            let tx_magic = tx.clone();
            let my_pub_for_magic = my_ed_pub.clone();
            let tx_outbound_magic = tx_outbound.clone();
            let magic_port = port;

            tokio::spawn(async move {
                while let Ok(msg) = rx_global_outbound.recv().await {
                    if msg == "/host" {
                        let tx_m = tx_magic.clone();
                        let my_pub_m = my_pub_for_magic.clone();
                        tokio::spawn(async move {
                            let _ = tx_m.send(quickchat_tui::app::AppEvent::System(
                                "Generating Wormhole Code...".to_string(),
                            ));
                            if let Ok((mut wormhole, code)) =
                                quickchat_net::magic::host_wormhole().await
                            {
                                let _ = tx_m.send(quickchat_tui::app::AppEvent::System(format!(
                                    "Wormhole Code: {}",
                                    code
                                )));

                                let my_ip = local_ip_address::local_ip().unwrap_or(
                                    std::net::IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1)),
                                );
                                let address = format!("{}:{}", my_ip, magic_port);

                                let sig = quickchat_net::magic::SignalingMessage {
                                    public_key: my_pub_m,
                                    address,
                                };

                                // Send our info
                                let _ =
                                    quickchat_net::magic::send_signaling(&mut wormhole, &sig).await;

                                // Receive their info (though host usually just waits for connection on their QUIC endpoint)
                                if let Ok(peer_sig) =
                                    quickchat_net::magic::receive_signaling(&mut wormhole).await
                                {
                                    let _ =
                                        tx_m.send(quickchat_tui::app::AppEvent::System(format!(
                                            "Peer discovered via Wormhole: {}",
                                            peer_sig.address
                                        )));
                                }
                            }
                        });
                    } else if msg.starts_with("/join ") {
                        let code_str = msg.trim_start_matches("/join ").trim().to_string();
                        let tx_m = tx_magic.clone();
                        let quic_node_m = quic_node_for_magic.clone();
                        let my_pub_m = my_pub_for_magic.clone();
                        let tx_outbound_m = tx_outbound_magic.clone();

                        tokio::spawn(async move {
                            let _ = tx_m.send(quickchat_tui::app::AppEvent::System(format!(
                                "Joining Wormhole: {}",
                                code_str
                            )));
                            if let Ok(mut wormhole) =
                                quickchat_net::magic::join_wormhole(&code_str).await
                            {
                                // Receive host info
                                if let Ok(peer_sig) =
                                    quickchat_net::magic::receive_signaling(&mut wormhole).await
                                {
                                    let _ =
                                        tx_m.send(quickchat_tui::app::AppEvent::System(format!(
                                            "Host discovered via Wormhole: {}",
                                            peer_sig.address
                                        )));

                                    let my_ip = local_ip_address::local_ip().unwrap_or(
                                        std::net::IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1)),
                                    );
                                    let address = format!("{}:{}", my_ip, magic_port);
                                    let sig = quickchat_net::magic::SignalingMessage {
                                        public_key: my_pub_m.clone(),
                                        address,
                                    };
                                    let _ =
                                        quickchat_net::magic::send_signaling(&mut wormhole, &sig)
                                            .await;

                                    // Now dial QUIC
                                    if let Ok(peer_addr) =
                                        peer_sig.address.parse::<std::net::SocketAddr>()
                                    {
                                        let _ = tx_m.send(quickchat_tui::app::AppEvent::System(
                                            format!("Dialing {} over QUIC...", peer_addr),
                                        ));
                                        if let Ok(connection) =
                                            quic_node_m.connect(peer_addr, "localhost").await
                                        {
                                            let _ =
                                                tx_m.send(quickchat_tui::app::AppEvent::System(
                                                    "Wormhole QUIC connection established!"
                                                        .to_string(),
                                                ));
                                            tokio::spawn(handle_connection(
                                                connection,
                                                my_pub_m,
                                                tx_m,
                                                tx_outbound_m.subscribe(),
                                            ));
                                        }
                                    }
                                }
                            }
                        });
                    }
                }
            });

            // TUI setup
            crossterm::terminal::enable_raw_mode()?;
            let mut stdout = std::io::stdout();
            crossterm::execute!(stdout, crossterm::terminal::EnterAlternateScreen)?;
            let backend = ratatui::backend::CrosstermBackend::new(stdout);
            let mut terminal = ratatui::Terminal::new(backend)?;

            // IPC Listener
            let tx_outbound_ipc = tx_outbound.clone();
            tokio::spawn(async move {
                if let Ok(socket) = tokio::net::UdpSocket::bind("127.0.0.1:18080").await {
                    let mut buf = [0u8; 65535];
                    while let Ok((len, _)) = socket.recv_from(&mut buf).await {
                        if let Ok(msg) = String::from_utf8(buf[..len].to_vec()) {
                            let _ = tx_outbound_ipc.send(msg);
                        }
                    }
                }
            });

            let mut app = quickchat_tui::app::App::new(rx, tx_outbound, &db_path_str);

            // Run TUI main loop (blocks main thread)
            let res = app.run(&mut terminal);
            crossterm::terminal::disable_raw_mode()?;
            crossterm::execute!(
                terminal.backend_mut(),
                crossterm::terminal::LeaveAlternateScreen,
                crossterm::event::DisableMouseCapture
            )?;

            if let Err(err) = res {
                error!("{:?}", err);
            }
        }
        Commands::Send { stdin } => {
            if stdin {
                use std::io::Read;
                let mut buffer = String::new();
                std::io::stdin().read_to_string(&mut buffer)?;
                let formatted = format!("```\n{}\n```", buffer);

                let socket = std::net::UdpSocket::bind("0.0.0.0:0")?;
                socket.send_to(formatted.as_bytes(), "127.0.0.1:18080")?;
                println!("Sent piped input to running daemon.");
            }
        }
        Commands::Share { pointer } => {
            if let Some((file, line)) = pointer.split_once(':')
                && let Ok(line_num) = line.parse::<usize>()
            {
                let content = std::fs::read_to_string(file)?;
                let lines: Vec<&str> = content.lines().collect();
                if line_num > 0 && line_num <= lines.len() {
                    let snippet = lines[line_num - 1];
                    let formatted =
                        format!("**Shared Pointer** `{}`\n```\n{}\n```", pointer, snippet);

                    let socket = std::net::UdpSocket::bind("0.0.0.0:0")?;
                    socket.send_to(formatted.as_bytes(), "127.0.0.1:18080")?;
                    println!("Shared pointer {} to running daemon.", pointer);
                }
            }
        }
    }

    Ok(())
}

async fn handle_connection(
    connection: quinn::Connection,
    my_pubkey: String,
    tx: std::sync::mpsc::Sender<quickchat_tui::app::AppEvent>,
    mut rx_outbound: tokio::sync::broadcast::Receiver<String>,
) {
    use bytes::Bytes;
    use prost::Message;
    use quickchat_types::proto::{ChatMessage, Envelope, Handshake, envelope::Payload};

    let _my_pub_bytes = hex::decode(&my_pubkey).unwrap_or_default();

    // Open a bidirectional stream for the handshake
    if let Ok((mut send, mut recv)) = connection.open_bi().await {
        let handshake = Handshake {
            public_key: hex::decode(my_pubkey.clone()).unwrap_or_default(),
            signature: vec![], // TODO: actual ed25519 signature
            display_name: "QuickChatUser".to_string(),
        };

        let mut buf = Vec::new();
        if handshake.encode_length_delimited(&mut buf).is_ok() {
            let _ = send.write_all(&buf).await;
        }

        // Wait for their handshake
        let mut recv_buf = [0u8; 1024];
        let mut peer_name = String::new();
        let mut peer_pub_bytes = Vec::new();
        if let Ok(Some(n)) = recv.read(&mut recv_buf).await {
            let bytes = Bytes::copy_from_slice(&recv_buf[..n]);
            if let Ok(peer_handshake) = Handshake::decode_length_delimited(bytes) {
                let peer_pub = hex::encode(&peer_handshake.public_key);
                peer_name = peer_handshake.display_name.clone();
                peer_pub_bytes = peer_handshake.public_key.clone();
                let _ = tx.send(quickchat_tui::app::AppEvent::System(format!(
                    "Received handshake from {}. Display Name: {}",
                    peer_pub, peer_name
                )));

                // Save contact
            }
        }

        let conn_rx = connection.clone();
        let tx_rx = tx.clone();
        let peer_name_rx = peer_name.clone();
        let peer_pub_rx = peer_pub_bytes.clone();

        // Task for reading incoming chat streams
        tokio::spawn(async move {
            while let Ok(recv) = conn_rx.accept_uni().await {
                let peer_name_rx = peer_name_rx.clone();
                let _peer_pub_rx = peer_pub_rx.clone();
                let tx_rx = tx_rx.clone();

                tokio::spawn(async move {
                    use futures::StreamExt;
                    use tokio_util::codec::{FramedRead, LengthDelimitedCodec};

                    let mut framed = FramedRead::new(recv, LengthDelimitedCodec::new());

                    while let Some(Ok(bytes_mut)) = framed.next().await {
                        let b = bytes_mut.freeze();
                        if let Ok(envelope) = Envelope::decode(b) {
                            match envelope.payload {
                                Some(Payload::ChatMessage(chat)) => {
                                    let _ = tx_rx.send(quickchat_tui::app::AppEvent::Message(
                                        format!("{}: {}", peer_name_rx, chat.content),
                                    ));

                                    // DB logic handled by app.rs now
                                }
                                Some(Payload::FileOffer(offer)) => {
                                    let _ =
                                        tx_rx.send(quickchat_tui::app::AppEvent::System(format!(
                                            "{} is sending file: {}",
                                            peer_name_rx, offer.filename
                                        )));
                                }
                                Some(Payload::FileChunk(chunk)) => {
                                    let download_path = std::path::PathBuf::from(format!(
                                        "downloads_{}.tmp",
                                        chunk.id
                                    ));
                                    if let Err(e) = quickchat_core::file_manager::append_chunk(
                                        &download_path,
                                        &chunk,
                                    )
                                    .await
                                    {
                                        let _ = tx_rx.send(quickchat_tui::app::AppEvent::System(
                                            format!("Failed to save chunk: {}", e),
                                        ));
                                    } else {
                                        let _ = tx_rx.send(quickchat_tui::app::AppEvent::System(
                                            format!(
                                                "Received file chunk {} ({} bytes) -> {:?}",
                                                chunk.chunk_index,
                                                chunk.data.len(),
                                                download_path
                                            ),
                                        ));
                                    }
                                }
                                None => {}
                            }
                        }
                    }
                });
            }
        });

        // Task for sending outbound chat streams
        let conn_tx = connection.clone();
        tokio::spawn(async move {
            while let Ok(msg) = rx_outbound.recv().await {
                if msg.starts_with("/file ") {
                    let path = msg.trim_start_matches("/file ").trim();
                    let path_buf = std::path::PathBuf::from(path);
                    let file_id = uuid::Uuid::new_v4().to_string();

                    if let Ok(send) = conn_tx.open_uni().await {
                        // Spawn file sending task
                        tokio::spawn(async move {
                            let _ =
                                quickchat_core::file_manager::send_file(send, &path_buf, file_id)
                                    .await;
                        });
                    }
                    continue;
                }

                let chat = ChatMessage {
                    id: uuid::Uuid::new_v4().to_string(),
                    content: msg,
                    timestamp: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_millis() as u64,
                };

                if let Ok(send) = conn_tx.open_uni().await {
                    let envelope = Envelope {
                        payload: Some(Payload::ChatMessage(chat.clone())),
                    };

                    let mut chat_encoded = bytes::BytesMut::new();
                    if envelope.encode(&mut chat_encoded).is_ok() {
                        use futures::SinkExt;
                        use tokio_util::codec::{FramedWrite, LengthDelimitedCodec};
                        let mut framed = FramedWrite::new(send, LengthDelimitedCodec::new());
                        let _ = framed.send(chat_encoded.freeze()).await;

                        // DB logic handled by app.rs now
                    }
                }
            }
        });
    }
}
