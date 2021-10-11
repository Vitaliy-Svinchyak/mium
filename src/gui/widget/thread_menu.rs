use tui::style::{Color, Modifier, Style};
use tui::text::Spans;
use tui::widgets::{Block, Borders, List, ListItem};

use crate::sync::thread_info_connection::ThreadInfoReceiver;

pub fn draw(items: &Vec<ThreadInfoReceiver>) -> List {
    let thread_menu: Vec<ListItem> = items
        .iter()
        .map(|thread_connection| {
            let mut title = thread_connection.title.clone();
            if thread_connection.closed {
                title = vec![title.as_str(), " [closed]"].concat();
            }

            ListItem::new(Spans::from(title)).style(Style::default().fg(Color::White))
        })
        .collect();

    List::new(thread_menu)
        .block(Block::default().borders(Borders::ALL).title("Threads"))
        .highlight_style(
            Style::default()
                .bg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ")
}
