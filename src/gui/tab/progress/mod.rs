use tui::backend::Backend;
use tui::Frame;
use tui::layout::{Constraint, Layout, Rect};

use crate::gui::app::App;

mod widget;

pub fn draw<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .constraints(
            [
                Constraint::Percentage(40),
                Constraint::Percentage(40),
                Constraint::Percentage(20),
            ]
            .as_ref(),
        )
        .split(area);

    widget::threads::draw(f, app, chunks[0]);
    widget::threads_bar_chart::draw(f, app, chunks[1]);
    widget::progress_bar::draw(f, app, chunks[2]);
}
