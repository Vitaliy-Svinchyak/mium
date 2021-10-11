use std::{error::Error, io};

use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Corner, Direction, Layout},
    style::{Color, Modifier, Style},
    Terminal,
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem},
};

use util::{
    event::{Event, Events},
};
use crate::gui::app::{App, ThreadConnection, LogLvl};

mod util;
pub mod app;

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
            // Create two chunks with equal horizontal screen space
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(f.size());

            // Iterate through all elements in the `items` app and append some debug text to it.
            let thread_menu: Vec<ListItem> = app
                .items
                .items
                .iter()
                .map(|thread_connection|
                    ListItem::new(Spans::from(thread_connection.title.clone()))
                        .style(Style::default().fg(Color::White))
                )
                .collect();

            // Create a List from all list items and highlight the currently selected one
            let items = List::new(thread_menu)
                .block(Block::default().borders(Borders::ALL).title("List"))
                .highlight_style(
                    Style::default()
                        .bg(Color::LightGreen)
                        .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol(">> ");

            // We can now render the item list
            f.render_stateful_widget(items, chunks[0], &mut app.items.state);

            let selected_thread = app.items.selected;
            // Let's do the same for the events.
            // The event list doesn't have any state and only displays the current state of the list.
            let events_data = match selected_thread {
                None => {
                    vec![]
                }
                Some(index) => {
                   app.items.items[index].log_events.clone()
                }
            };

            let events: Vec<ListItem> = events_data
                .iter()
                .rev()
                .map(|event| {
                    let s = match event.lvl {
                        LogLvl::INFO => Style::default().fg(Color::Blue),
                        LogLvl::WARNING => Style::default().fg(Color::Yellow),
                        LogLvl::ERROR => Style::default().fg(Color::Magenta),
                        LogLvl::CRITICAL => Style::default().fg(Color::Red),
                    };
                    let log = Spans::from(vec![Span::styled(event.data.clone(), s)]);

                    ListItem::new(vec![
                        Spans::from("-".repeat(chunks[1].width as usize)),
                        Spans::from(""),
                        log,
                    ])
                })
                .collect();
            let events_list = List::new(events)
                .block(Block::default().borders(Borders::ALL).title("List"))
                .start_corner(Corner::BottomLeft);
            f.render_widget(events_list, chunks[1]);
        })?;

        match events.next()? {
            Event::Input(input) => match input {
                Key::Char('q') => {
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