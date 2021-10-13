use std::sync::mpsc::Receiver;
use std::{error::Error, io};

use image::DynamicImage;
use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::backend::Backend;
use tui::style::Style;
use tui::text::{Span, Spans};
use tui::widgets::{Borders, Tabs};
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Layout},
    Frame, Terminal,
};

use util::event::{Event, Events};

use crate::gui::app::App;
use crate::gui::theme::{theme_block, THEME};
use crate::sync::thread_info_connection::ThreadInfoReceiver;

pub mod app;
mod block;
mod tab;
mod theme;
mod util;

pub fn main(
    threads: Vec<ThreadInfoReceiver>,
    pages: usize,
    image_rx: Receiver<DynamicImage>,
) -> Result<(), Box<dyn Error>> {
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let events = Events::new();

    let mut app = App::new(threads, pages);

    loop {
        terminal.draw(|f| {
            draw(f, &mut app, &image_rx);
        })?;

        let event = events.next()?;
        let close_app = handle_key_event(event, &mut app);

        if close_app {
            break;
        }
    }

    Ok(())
}

fn handle_key_event(event: Event<Key>, app: &mut App) -> bool {
    let mut close_app = false;

    match event {
        Event::Input(input) => match input {
            Key::Char('q' | '`') => {
                close_app = true;
            }
            Key::Char('1') => {
                app.tabs.select(0);
            }
            Key::Char('2') => {
                app.tabs.select(1);
            }
            Key::Left => {
                if app.menu_in_focus {
                    app.menu_items.unselect();
                } else {
                    app.menu_in_focus = true;
                    app.log_items.clear();
                }
            }
            Key::Right => {
                if app.menu_in_focus {
                    app.log_items.next();
                    app.menu_in_focus = false;
                }
            }
            Key::Down => {
                if app.menu_in_focus {
                    app.menu_items.next();
                    app.log_items.clear();
                } else {
                    app.log_items.previous();
                }
            }
            Key::Up => {
                if app.menu_in_focus {
                    app.menu_items.previous();
                    app.log_items.clear();
                } else {
                    app.log_items.next();
                }
            }
            _ => {}
        },
        Event::Tick => {
            app.tick();
        }
    }

    close_app
}

fn draw<B>(f: &mut Frame<B>, app: &mut App, image_rx: &Receiver<DynamicImage>)
where
    B: Backend,
{
    let chunks = Layout::default()
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(f.size());
    let titles = app
        .tabs
        .titles
        .iter()
        .map(|t| {
            Spans::from(Span::styled(
                t.clone(),
                Style::default().fg(THEME.white_text),
            ))
        })
        .collect();
    let tabs = Tabs::new(titles)
        .block(theme_block("Navigation").borders(Borders::BOTTOM))
        .highlight_style(Style::default().fg(THEME.contrast))
        .select(app.tabs.index.clone());
    f.render_widget(tabs, chunks[0]);

    match app.tabs.index {
        0 => tab::progress::draw(f, app, chunks[1]),
        1 => tab::result::draw(f, app, chunks[1], &image_rx),
        _ => {}
    };
}
