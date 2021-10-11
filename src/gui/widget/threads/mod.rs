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

    let menu = thread_menu::draw(&app.items.items);
    f.render_stateful_widget(menu, chunks[0], &mut app.items.state);

    let events_data = app.items.get_selected_logs();
    let thread_logs = thread_logs::draw(&events_data);
    f.render_widget(thread_logs, chunks[1]);
}
