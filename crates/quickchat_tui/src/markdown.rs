use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Span;
use regex::Regex;
use std::sync::OnceLock;

static POINTER_REGEX: OnceLock<Regex> = OnceLock::new();

pub fn parse_markdown_line<'a>(text: &'a str) -> Vec<Span<'a>> {
    let re = POINTER_REGEX.get_or_init(|| {
        Regex::new(r"([a-zA-Z0-9_/\.\-]+\.[a-z]+:\d+)").unwrap()
    });

    let mut spans = Vec::new();
    let mut last_end = 0;

    for mat in re.find_iter(text) {
        if mat.start() > last_end {
            spans.extend(parse_basic_markdown(&text[last_end..mat.start()]));
        }
        spans.push(Span::styled(
            mat.as_str().to_string(),
            Style::default().fg(Color::Green).add_modifier(Modifier::UNDERLINED),
        ));
        last_end = mat.end();
    }

    if last_end < text.len() {
        spans.extend(parse_basic_markdown(&text[last_end..]));
    }

    spans
}

fn parse_basic_markdown<'a>(text: &'a str) -> Vec<Span<'a>> {
    let mut spans = Vec::new();
    let mut current = String::new();
    let mut i = 0;
    
    let chars: Vec<char> = text.chars().collect();
    let len = chars.len();

    while i < len {
        // Bold
        if i + 1 < len && chars[i] == '*' && chars[i+1] == '*' {
            if !current.is_empty() {
                spans.push(Span::raw(current.clone()));
                current.clear();
            }
            i += 2;
            let mut bold_text = String::new();
            while i < len && !(i + 1 < len && chars[i] == '*' && chars[i+1] == '*') {
                bold_text.push(chars[i]);
                i += 1;
            }
            spans.push(Span::styled(bold_text, Style::default().add_modifier(Modifier::BOLD)));
            i += 2; // skip closing **
            continue;
        }

        // Italic
        if chars[i] == '*' {
            if !current.is_empty() {
                spans.push(Span::raw(current.clone()));
                current.clear();
            }
            i += 1;
            let mut italic_text = String::new();
            while i < len && chars[i] != '*' {
                italic_text.push(chars[i]);
                i += 1;
            }
            spans.push(Span::styled(italic_text, Style::default().add_modifier(Modifier::ITALIC)));
            i += 1; // skip closing *
            continue;
        }

        // Code
        if chars[i] == '`' {
            if !current.is_empty() {
                spans.push(Span::raw(current.clone()));
                current.clear();
            }
            i += 1;
            let mut code_text = String::new();
            while i < len && chars[i] != '`' {
                code_text.push(chars[i]);
                i += 1;
            }
            spans.push(Span::styled(code_text, Style::default().fg(Color::Cyan)));
            i += 1; // skip closing `
            continue;
        }

        current.push(chars[i]);
        i += 1;
    }

    if !current.is_empty() {
        spans.push(Span::raw(current));
    }

    spans
}
