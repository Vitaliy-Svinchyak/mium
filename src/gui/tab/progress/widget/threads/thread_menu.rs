use tui::style::{Color, Modifier, Style};
use tui::text::Spans;
use tui::widgets::{List, ListItem};

use crate::gui::theme::{theme_block, THEME};
use crate::sync::thread_info_connection::ThreadInfoReceiver;

pub fn draw(items: &Vec<ThreadInfoReceiver>) -> List {
    let thread_menu: Vec<ListItem> = items
        .iter()
        .map(|thread_connection| {
            let mut title = thread_connection.title.clone();

            if thread_connection.has_errors() {
                title = vec![title.as_str(), " [failed]"].concat();
            } else if thread_connection.closed {
                title = vec![title.as_str(), " [closed]"].concat();
            }

            let style = if thread_connection.has_errors() {
                Style::default().fg(Color::Black).bg(THEME.red)
            } else if thread_connection.closed {
                Style::default().fg(Color::Black).bg(THEME.green)
            } else {
                Style::default().fg(THEME.white_text)
            };

            ListItem::new(Spans::from(title)).style(style)
        })
        .collect();

    List::new(thread_menu)
        .block(theme_block("Threads"))
        .highlight_style(
            Style::default()
                .bg(THEME.contrast)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ")
}
