use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::widgets::{BarChart, Block, Borders};
use tui::Frame;

use crate::gui::app::App;

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

    let barchart = BarChart::default()
        .block(Block::default().borders(Borders::ALL))
        .data(&data)
        .bar_width(7)
        .bar_style(Style::default().fg(Color::Yellow))
        .value_style(Style::default().fg(Color::Black).bg(Color::Yellow));

    f.render_widget(barchart, chunks[0]);
}
