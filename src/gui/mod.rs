use std::{error::Error, io};

use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::backend::Backend;
use tui::layout::Rect;
use tui::style::{Color, Style};
use tui::widgets::{BarChart, Block, Borders};
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Direction, Layout},
    Frame, Terminal,
};

use util::event::{Event, Events};

use crate::gui::app::{App, ThreadConnection};

pub mod app;
mod util;
mod widget;

pub fn main(threads: Vec<ThreadConnection>) -> Result<(), Box<dyn Error>> {
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let events = Events::new();

    // Create a new app with some example state
    let mut app = App::new(threads);

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(f.size());

            draw_threads(f, &mut app, chunks[0]);
            draw_bar_chart(f, &mut app, chunks[1]);
        })?;

        match events.next()? {
            Event::Input(input) => match input {
                Key::Char('q' | '`') => {
                    break;
                }
                Key::Left => {
                    app.items.unselect();
                }
                Key::Down => {
                    app.items.next();
                }
                Key::Up => {
                    app.items.previous();
                }
                _ => {}
            },
            Event::Tick => {
                app.tick();
            }
        }
    }

    Ok(())
}

fn draw_bar_chart<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
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
        .bar_width(5)
        .bar_style(Style::default().fg(Color::Yellow))
        .value_style(Style::default().fg(Color::Black).bg(Color::Yellow));

    f.render_widget(barchart, chunks[0]);
}

fn draw_threads<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
        .split(area);

    let menu = widget::thread_menu::draw(&app.items.items);
    f.render_stateful_widget(menu, chunks[0], &mut app.items.state);

    let events_data = app.items.get_selected_logs();
    let thread_logs = widget::thread_logs::draw(&events_data);
    f.render_widget(thread_logs, chunks[1]);
}
