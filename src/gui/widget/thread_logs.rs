use tui::layout::Corner;
use tui::style::{Color, Style};
use tui::text::Span;
use tui::text::Spans;
use tui::widgets::{Block, Borders, List, ListItem};

use crate::gui::app::{LogEvent, LogLvl};

pub fn draw(items: &Vec<LogEvent>) -> List {
    let events: Vec<ListItem> = items
        .iter()
        .rev()
        .map(|event| {
            let s = match event.lvl {
                LogLvl::INFO => Style::default().fg(Color::Blue),
                LogLvl::WARNING => Style::default().fg(Color::Yellow),
                LogLvl::ERROR => Style::default().fg(Color::Magenta),
                LogLvl::CRITICAL => Style::default().fg(Color::Red),
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
