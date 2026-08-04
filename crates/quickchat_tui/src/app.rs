use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use quickchat_core::db::{ChatDatabase, Contact};
use ratatui::{Terminal, backend::Backend, widgets::ListState};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tui_input::Input;

pub enum AppEvent {
    Message(String),
    System(String),
    PluginOutput(String),
    AiResponse(String),
}

pub enum ActivePane {
    Chat,
    Contacts,
    Plugins,
}

pub struct App {
    pub input: Input,
    pub messages: Vec<String>,
    pub should_quit: bool,
    pub rx: std::sync::mpsc::Receiver<AppEvent>,
    pub tx: std::sync::mpsc::Sender<AppEvent>,
    pub tx_outbound: tokio::sync::broadcast::Sender<String>,

    pub contacts: Vec<Contact>,
    pub active_contact: Option<Vec<u8>>,
    pub contact_list_state: ListState,

    // SQLite integration for persistent history
    pub chat_db: Arc<Mutex<ChatDatabase>>,

    // Multi-pane state for V3
    pub active_pane: ActivePane,
    pub plugin_outputs: Vec<String>,
}

impl App {
    pub fn new(
        rx: std::sync::mpsc::Receiver<AppEvent>,
        tx: std::sync::mpsc::Sender<AppEvent>,
        tx_outbound: tokio::sync::broadcast::Sender<String>,
        chat_db_path: &str,
    ) -> Self {
        let contacts: Vec<Contact> = vec![Contact {
            public_key: b"mock_contact".to_vec(),
            alias: "Mock Contact".to_string(),
        }];

        let chat_db = Arc::new(Mutex::new(
            ChatDatabase::new(chat_db_path).expect("Failed to open ChatDatabase"),
        ));

        let mut state = ListState::default();
        let active = if !contacts.is_empty() {
            state.select(Some(0));
            Some(contacts[0].public_key.clone())
        } else {
            None
        };

        let mut app = Self {
            input: Input::default(),
            messages: Vec::new(),
            should_quit: false,
            rx,
            tx,
            tx_outbound,
            contacts,
            active_contact: active,
            contact_list_state: state,
            chat_db,
            active_pane: ActivePane::Chat,
            plugin_outputs: Vec::new(),
        };
        app.reload_messages();
        app
    }

    pub fn reload_messages(&mut self) {
        if let Some(ref pubkey) = self.active_contact {
            let pubkey_str = String::from_utf8_lossy(pubkey);
            let db = self.chat_db.lock().unwrap();
            if let Ok(msgs) = db.get_messages(&pubkey_str) {
                self.messages = msgs;
            } else {
                self.messages.clear();
            }
        }
    }

    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> std::io::Result<()>
    where
        std::io::Error: From<<B as Backend>::Error>,
    {
        while !self.should_quit {
            // Process all pending events
            while let Ok(event) = self.rx.try_recv() {
                match event {
                    AppEvent::Message(m) => {
                        self.messages.push(m.clone());
                        if let Some(ref pubkey) = self.active_contact {
                            let db = self.chat_db.lock().unwrap();
                            let pubkey_str = String::from_utf8_lossy(pubkey);
                            let _ = db.insert_message(&pubkey_str, &pubkey_str, &m);
                        }
                    }
                    AppEvent::System(s) => self.messages.push(format!("[SYSTEM] {}", s)),
                    AppEvent::PluginOutput(p) => self.plugin_outputs.push(p),
                    AppEvent::AiResponse(r) => self.messages.push(format!("[AI] {}", r)),
                }
            }

            terminal.draw(|f| crate::ui::render(f, self))?;

            if event::poll(Duration::from_millis(50))?
                && let Event::Key(key) = event::read()?
                && key.kind == KeyEventKind::Press
            {
                if key
                    .modifiers
                    .contains(crossterm::event::KeyModifiers::CONTROL)
                    && key.code == KeyCode::Char('o')
                {
                    if let Some(msg) = self.messages.last() {
                        let re = regex::Regex::new(r"([a-zA-Z0-9_/\.\-]+\.[a-z]+:\d+)").unwrap();
                        if let Some(mat) = re.find(msg) {
                            let path_line = mat.as_str();
                            let _ = std::process::Command::new("code")
                                .arg("--goto")
                                .arg(path_line)
                                .spawn();
                            self.messages
                                .push(format!("[SYSTEM] Launched editor for: {}", path_line));
                        }
                    }
                    continue;
                }

                match key.code {
                    KeyCode::Esc => self.should_quit = true,
                    KeyCode::Up => {
                        if !self.contacts.is_empty() {
                            let i = match self.contact_list_state.selected() {
                                Some(i) => {
                                    if i == 0 {
                                        self.contacts.len() - 1
                                    } else {
                                        i - 1
                                    }
                                }
                                None => 0,
                            };
                            self.contact_list_state.select(Some(i));
                            self.active_contact = Some(self.contacts[i].public_key.clone());
                            self.reload_messages();
                        }
                    }
                    KeyCode::Down => {
                        if !self.contacts.is_empty() {
                            let i = match self.contact_list_state.selected() {
                                Some(i) => {
                                    if i >= self.contacts.len() - 1 {
                                        0
                                    } else {
                                        i + 1
                                    }
                                }
                                None => 0,
                            };
                            self.contact_list_state.select(Some(i));
                            self.active_contact = Some(self.contacts[i].public_key.clone());
                            self.reload_messages();
                        }
                    }
                    KeyCode::Enter => {
                        let msg = self.input.value().to_string();
                        if msg.starts_with("/ai ") {
                            let prompt = msg.trim_start_matches("/ai ").trim().to_string();
                            let tx_ai = self.tx.clone();
                            self.messages.push(format!("You: /ai {}", prompt));
                            self.messages
                                .push(format!("[SYSTEM] Querying Offline AI for: {}...", prompt));

                            tokio::spawn(async move {
                                let ai_client = quickchat_core::ai::LocalLlmClient::new(
                                    "http://localhost:11434",
                                );
                                if let Ok(resp) = ai_client.analyze_code("", &prompt).await {
                                    let _ = tx_ai.send(AppEvent::AiResponse(resp));
                                } else {
                                    let _ = tx_ai.send(AppEvent::System(
                                        "Offline AI Error: Could not connect to daemon."
                                            .to_string(),
                                    ));
                                }
                            });

                            self.input.reset();
                        } else if msg.starts_with("/clip push") {
                            if let Ok(mut ctx) = arboard::Clipboard::new() {
                                if let Ok(text) = ctx.get_text() {
                                    self.messages
                                        .push(format!("You pushed clipboard: {}", text));
                                    // Normally we would send this over tx_outbound or format it in a specific way
                                    // For now, send as a command message for the CLI to parse and send as Payload::ClipboardSync
                                    let _ = self.tx_outbound.send(format!("/clip push {}", text));
                                }
                            }
                            self.input.reset();
                        } else if msg.starts_with("/group join ") {
                            let group_name = msg.trim_start_matches("/group join ").trim();
                            self.messages
                                .push(format!("[SYSTEM] Joined group: {}", group_name));
                            let _ = self.tx_outbound.send(format!("/group join {}", group_name));
                            self.input.reset();
                        } else if msg.starts_with("/pair ") {
                            let file_name = msg.trim_start_matches("/pair ").trim();
                            self.messages
                                .push(format!("[SYSTEM] Pair programming on: {}", file_name));
                            let _ = self.tx_outbound.send(format!("/pair {}", file_name));
                            self.input.reset();
                        } else if msg.starts_with("/voice") {
                            self.messages
                                .push("[SYSTEM] Recording voice note (10s)...".to_string());
                            let _ = self.tx_outbound.send("/voice".to_string());
                            self.input.reset();
                        } else if !msg.is_empty() {
                            self.messages.push(format!("You: {}", msg));

                            // Persist outgoing message
                            if let Some(ref pubkey) = self.active_contact {
                                let db = self.chat_db.lock().unwrap();
                                let pubkey_str = String::from_utf8_lossy(pubkey);
                                let _ = db.insert_message(&pubkey_str, "You", &msg);
                            }

                            let _ = self.tx_outbound.send(msg);
                            self.input.reset();
                        }
                    }
                    _ => {
                        // Delegate to tui-input backend for handling character inputs
                        use tui_input::backend::crossterm::EventHandler;
                        self.input.handle_event(&Event::Key(key));
                    }
                }
            }
        }
        Ok(())
    }
}
