use tui::layout::Corner;
use tui::style::{Color, Style};
use tui::text::Span;
use tui::text::Spans;
use tui::widgets::{Block, Borders, List, ListItem};

pub fn draw(items: &Vec<String>) -> List {
    let events: Vec<ListItem> = items
        .iter()
        .rev()
        .map(|event| {
            let log = Spans::from(vec![Span::styled(
                event.clone(),
                Style::default().fg(Color::Blue),
            )]);

            ListItem::new(vec![Spans::from(""), log])
        })
        .collect();

    List::new(events)
        .block(Block::default().borders(Borders::ALL).title("List"))
        .start_corner(Corner::BottomLeft)
}
