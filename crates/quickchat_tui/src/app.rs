use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use quickchat_core::db::{
    Connection, Contact, get_contacts, get_messages_for_contact,
};
use ratatui::{Terminal, backend::Backend, widgets::ListState};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tui_input::Input;

pub enum AppEvent {
    Message(String),
    System(String),
}

pub struct App {
    pub input: Input,
    pub messages: Vec<String>,
    pub should_quit: bool,
    pub rx: std::sync::mpsc::Receiver<AppEvent>,
    pub tx_outbound: tokio::sync::broadcast::Sender<String>,

    pub db: Arc<Mutex<Connection>>,
    pub contacts: Vec<Contact>,
    pub active_contact: Option<Vec<u8>>,
    pub contact_list_state: ListState,
}

impl App {
    pub fn new(
        rx: std::sync::mpsc::Receiver<AppEvent>,
        tx_outbound: tokio::sync::broadcast::Sender<String>,
        db: Arc<Mutex<Connection>>,
    ) -> Self {
        let contacts = {
            let conn = db.lock().unwrap();
            get_contacts(&conn).unwrap_or_default()
        };

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
            tx_outbound,
            db,
            contacts,
            active_contact: active,
            contact_list_state: state,
        };
        app.reload_messages();
        app
    }

    pub fn reload_messages(&mut self) {
        if let Some(ref pubkey) = self.active_contact {
            let conn = self.db.lock().unwrap();
            if let Ok(msgs) = get_messages_for_contact(&conn, pubkey) {
                self.messages = msgs
                    .into_iter()
                    .map(|m| {
                        if m.sender_id == *pubkey {
                            format!("Peer: {}", m.content)
                        } else {
                            format!("You: {}", m.content)
                        }
                    })
                    .collect();
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
                    AppEvent::Message(m) => self.messages.push(m),
                    AppEvent::System(s) => self.messages.push(format!("[SYSTEM] {}", s)),
                }
            }

            terminal.draw(|f| crate::ui::render(f, self))?;

            if event::poll(Duration::from_millis(50))?
                && let Event::Key(key) = event::read()?
                    && key.kind == KeyEventKind::Press {
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
                                if !msg.is_empty() {
                                    self.messages.push(format!("You: {}", msg));
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
