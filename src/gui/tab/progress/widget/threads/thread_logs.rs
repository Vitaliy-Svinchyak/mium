use tui::layout::Corner;
use tui::style::{Color, Modifier, Style};
use tui::text::Span;
use tui::text::Spans;
use tui::widgets::{List, ListItem};

use crate::gui::theme::{theme_block, THEME};
use crate::sync::thread_info_connection::TypedLog;

pub fn draw(items: &Vec<TypedLog>) -> List {
    let mut color_order = false;
    let len = items.len();
    let events: Vec<ListItem> = items
        .iter()
        .rev()
        .enumerate()
        .map(|(i, event)| {
            let log = Spans::from(vec![
                Span::styled(
                    format!("[{}]", len - i),
                    Style::default().add_modifier(Modifier::DIM),
                ),
                Span::styled(
                    event.clone().data(),
                    Style::default().bg(if event.is_error() {
                        THEME.red
                    } else if color_order {
                        Color::Rgb(43, 44, 52)
                    } else {
                        Color::Rgb(46, 47, 58)
                    }),
                ),
            ]);

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
