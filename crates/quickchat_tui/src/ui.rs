use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

use crate::app::App;

pub fn render(f: &mut Frame, app: &mut App) {
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([Constraint::Length(25), Constraint::Min(0)].as_ref())
        .split(f.area());

    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(3)].as_ref())
        .split(main_chunks[1]);

    // Contacts sidebar
    let contact_items: Vec<ListItem> = app
        .contacts
        .iter()
        .map(|c| {
            let content = ratatui::text::Line::from(ratatui::text::Span::raw(&c.alias));
            ListItem::new(content)
        })
        .collect();

    let contacts_list = List::new(contact_items)
        .block(Block::default().borders(Borders::ALL).title("Contacts"))
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
        .highlight_symbol(">> ");

    f.render_stateful_widget(contacts_list, main_chunks[0], &mut app.contact_list_state);

    // Messages Pane
    let messages: Vec<ListItem> = app
        .messages
        .iter()
        .map(|m| {
            let content = ratatui::text::Line::from(ratatui::text::Span::raw(m));
            ListItem::new(content)
        })
        .collect();

    let messages_list =
        List::new(messages).block(Block::default().borders(Borders::ALL).title("Messages"));

    f.render_widget(messages_list, right_chunks[0]);

    // Input Pane
    let input = Paragraph::new(app.input.value())
        .style(Style::default().fg(Color::Yellow))
        .block(Block::default().borders(Borders::ALL).title("Input"));

    f.render_widget(input, right_chunks[1]);

    f.set_cursor_position(ratatui::layout::Position::new(
        right_chunks[1].x + app.input.visual_cursor() as u16 + 1,
        right_chunks[1].y + 1,
    ));
}
