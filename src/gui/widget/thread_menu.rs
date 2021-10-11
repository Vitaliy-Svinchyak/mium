use tui::style::{Color, Modifier, Style};
use tui::text::Spans;
use tui::widgets::{Block, Borders, List, ListItem};

use crate::gui::app::{App, ThreadConnection};

pub fn draw(items: &Vec<ThreadConnection>) -> List {
    let thread_menu: Vec<ListItem> = items
        .iter()
        .map(|thread_connection| {
            ListItem::new(Spans::from(thread_connection.title.clone()))
                .style(Style::default().fg(Color::White))
        })
        .collect();

    List::new(thread_menu)
        .block(Block::default().borders(Borders::ALL).title("List"))
        .highlight_style(
            Style::default()
                .bg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ")
}
