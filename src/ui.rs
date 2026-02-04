use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Line, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

use crate::app::App;

pub fn draw(f: &mut Frame, app: &App, filter_mode: bool) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Min(1), Constraint::Length(1)])
        .split(f.size());

    // Top bar
    let top = if filter_mode {
        format!("Filter: {}", app.filter)
    } else {
        "newsbox — j/k:move  Enter:read  r:refresh  o:open  /:filter  q:quit".to_string()
    };
    f.render_widget(Paragraph::new(top), chunks[0]);

    // Main panes
    let panes = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(45), Constraint::Percentage(55)])
        .split(chunks[1]);

    // Left: list (email inbox style)
    let items: Vec<ListItem> = app.filtered.iter().enumerate().map(|(pos, idx)| {
        let a = &app.articles[*idx];
        let prefix = if pos == app.selected { "▶ " } else { "  " };
        let line = format!(
            "{}{:<4} {:<16} {}",
            prefix,
            a.date_line(),
            a.sender_line(),
            a.title
        );
        ListItem::new(Line::from(line))
    }).collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Inbox"))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD));
    f.render_widget(list, panes[0]);

    // Right: preview (email body)
    let body = if let Some(a) = app.selected_article() {
        let mut text = Text::default();
        text.lines.push(Line::from(a.title.clone()).style(Style::default().add_modifier(Modifier::BOLD)));
        text.lines.push(Line::from(format!("From: {}    Date: {}", a.source, a.date_line())));
        text.lines.push(Line::from(format!("Link: {}", a.link)));
        text.lines.push(Line::from(""));
        let content = if app.show_full { a.summary.clone() } else { truncate(&a.summary, 700) };
        text.lines.extend(Text::from(content).lines);
        text
    } else {
        Text::from("No articles loaded. Press r to refresh.")
    };

    let preview = Paragraph::new(body)
        .block(Block::default().borders(Borders::ALL).title("Message"))
        .wrap(Wrap { trim: false });
    f.render_widget(preview, panes[1]);

    // Bottom status
    f.render_widget(Paragraph::new(app.status.clone()), chunks[2]);
}

fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max { return s.to_string(); }
    format!("{}…", &s[..max])
}

