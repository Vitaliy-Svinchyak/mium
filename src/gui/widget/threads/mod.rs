use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::Frame;

use crate::gui::app::App;

mod thread_logs;
mod thread_menu;

pub fn draw<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
        .split(area);

    let menu = thread_menu::draw(&app.menu_items.items);
    f.render_stateful_widget(menu, chunks[0], &mut app.menu_items.state);

    let thread_logs = thread_logs::draw(&app.log_items.items);
    f.render_stateful_widget(thread_logs, chunks[1], &mut app.log_items.state);
}
