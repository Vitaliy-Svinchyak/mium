use tui::layout::Corner;
use tui::style::{Color, Modifier, Style};
use tui::text::Span;
use tui::text::Spans;
use tui::widgets::{List, ListItem};

use crate::gui::theme::theme_block;

pub fn draw(items: &Vec<String>) -> List {
    let mut color_order = false;
    let events: Vec<ListItem> = items
        .iter()
        .rev()
        .enumerate()
        .map(|(i, event)| {
            let log = Spans::from(vec![Span::styled(
                event.clone(),
                Style::default().bg(if color_order {
                    Color::Rgb(43, 44, 52)
                } else {
                    Color::Rgb(46, 47, 58)
                }),
            )]);

            color_order = !color_order;

            ListItem::new(log)
        })
        .collect();

    List::new(events)
        .block(theme_block("Thread logs"))
        .start_corner(Corner::BottomLeft)
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol(">> ")
}
