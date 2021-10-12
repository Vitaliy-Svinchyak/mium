use std::{error::Error, io};

use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Layout},
    Terminal,
};

use util::event::{Event, Events};

use crate::gui::app::App;
use crate::sync::thread_info_connection::ThreadInfoReceiver;

pub mod app;
mod util;
mod widget;
mod block;
mod theme;

pub fn main(threads: Vec<ThreadInfoReceiver>, pages: usize) -> Result<(), Box<dyn Error>> {
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let events = Events::new();

    let mut app = App::new(threads, pages);

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .constraints(
                    [
                        Constraint::Percentage(40),
                        Constraint::Percentage(40),
                        Constraint::Percentage(20),
                    ]
                    .as_ref(),
                )
                .split(f.size());

            widget::threads::draw(f, &mut app, chunks[0]);
            widget::threads_bar_chart::draw(f, &mut app, chunks[1]);
            widget::progress_bar::draw(f, &mut app, chunks[2]);
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
