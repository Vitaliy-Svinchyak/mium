use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders};
use tui::Frame;

use crate::gui::app::App;
use crate::gui::block::multicolored_barchart::MulticoloredBarChart;

pub fn draw<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .constraints(vec![Constraint::Percentage(100)])
        .direction(Direction::Horizontal)
        .split(area);

    let data: Vec<_> = app
        .items
        .items
        .iter()
        .map(|v| (v.title.as_str(), v.progress))
        .collect();
    let styles: Vec<Style> = app
        .items
        .items
        .iter()
        .map(|v| {
            if v.closed {
                Style::default().fg(Color::Green)
            } else {
                Style::default().fg(Color::Yellow)
            }
        })
        .collect();

    let barchart = MulticoloredBarChart::default()
        .block(Block::default().borders(Borders::ALL).title("Thread progress"))
        .data(&data)
        .bar_width(7)
        .multi_bar_style(styles)
        .max(24)
        .value_style(Style::default().fg(Color::Black).bg(Color::Yellow));

    f.render_widget(barchart, chunks[0]);
}
