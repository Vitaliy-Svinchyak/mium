use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::Style;
use tui::Frame;

use crate::gui::app::App;
use crate::gui::block::multicolored_barchart::MulticoloredBarChart;
use crate::gui::theme::{theme_block, THEME};

pub fn draw<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .constraints(vec![Constraint::Percentage(100)])
        .direction(Direction::Horizontal)
        .split(area);

    let data: Vec<_> = app
        .menu_items
        .items
        .iter()
        .map(|v| (v.title.as_str(), v.progress))
        .collect();
    let styles: Vec<Style> = app
        .menu_items
        .items
        .iter()
        .map(|v| {
            if v.has_errors() {
                Style::default().fg(THEME.red)
            } else if v.closed {
                Style::default().fg(THEME.green)
            } else {
                Style::default().fg(THEME.yellow)
            }
        })
        .collect();

    let barchart = MulticoloredBarChart::default()
        .block(theme_block("Thread progress"))
        .data(&data)
        .bar_width(7)
        .multi_bar_style(styles)
        .max(24)
        .value_style(Style::default().fg(THEME.white_text));

    f.render_widget(barchart, chunks[0]);
}
