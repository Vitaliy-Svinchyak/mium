use tui::layout::Corner;
use tui::style::{Color, Style};
use tui::text::Span;
use tui::text::Spans;
use tui::widgets::{Block, Borders, List, ListItem};

use crate::gui::app::{ThreadEvent, EventType};

pub fn draw(items: &Vec<ThreadEvent>) -> List {
    let events: Vec<ListItem> = items
        .iter()
        .rev()
        .map(|event| {
            let s = match event.lvl {
                EventType::INFO => Style::default().fg(Color::Blue),
                EventType::ERROR => Style::default().fg(Color::Magenta),
                EventType::PROGRESS => {Style::default()}
                EventType::CLOSE => {Style::default()}
            };
            let log = Spans::from(vec![Span::styled(event.data.clone(), s)]);

            ListItem::new(vec![
                Spans::from(""),
                log,
            ])
        })
        .collect();

    List::new(events)
        .block(Block::default().borders(Borders::ALL).title("List"))
        .start_corner(Corner::BottomLeft)
}
