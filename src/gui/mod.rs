use std::{error::Error, io};
use std::sync::mpsc::Receiver;

use image::DynamicImage;
use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Layout},
    Terminal,
};
use tui::style::Style;
use tui::text::{Span, Spans};
use tui::widgets::{Borders, Tabs};

use util::event::{Event, Events};

use crate::gui::app::App;
use crate::gui::theme::{THEME, theme_block};
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

    let mut menu_in_focus = true;
    loop {
        terminal.draw(|f| {
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
                0 => tab::progress::draw(f, &mut app, chunks[1]),
                1 => tab::result::draw(f, &mut app, chunks[1], &image_rx),
                _ => {}
            };
        })?;

        match events.next()? {
            Event::Input(input) => match input {
                Key::Char('q' | '`') => {
                    break;
                }
                Key::Char('1') => {
                    app.tabs.select(0);
                }
                Key::Char('2') => {
                    app.tabs.select(1);
                }
                Key::Left => {
                    if menu_in_focus {
                        app.menu_items.unselect();
                    } else {
                        menu_in_focus = true;
                        app.log_items.clear();
                    }
                }
                Key::Right => {
                    if menu_in_focus {
                        app.log_items.next();
                        menu_in_focus = false;
                    }
                }
                Key::Down => {
                    if menu_in_focus {
                        app.menu_items.next();
                        app.log_items.clear();
                    } else {
                        app.log_items.previous();
                    }
                }
                Key::Up => {
                    if menu_in_focus {
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
    }

    Ok(())
}
