use std::sync::mpsc::{Receiver, RecvError};

use crate::gui::util::StatefulList;

#[derive(Debug, Clone)]
pub enum LogLvl {
    INFO,
    WARNING,
    ERROR,
    CRITICAL,
}

#[derive(Debug, Clone)]
pub struct LogEvent {
    pub lvl: LogLvl,
    pub data: String,
}

impl LogEvent {
    pub fn info(data: String) -> LogEvent {
        LogEvent {
            lvl: LogLvl::INFO,
            data,
        }
    }
}

pub struct ThreadConnection {
    pub title: String,
    log_channel: Receiver<LogEvent>,
    pub log_events: Vec<LogEvent>,
}

impl ThreadConnection {
    pub fn new(title: String, log_channel: Receiver<LogEvent>) -> ThreadConnection {
        ThreadConnection {
            title,
            log_channel,
            log_events: vec![],
        }
    }

    pub fn pull(&mut self) {
        loop {
            let event = self.log_channel.try_recv();

            match event {
                Ok(e) => {
                    self.log_events.push(e);
                }
                Err(_) => {
                    break;
                }
            }
        }
    }
}

pub(super) struct App {
    pub items: StatefulList<ThreadConnection>,
}

impl App {
    pub fn new(items: Vec<ThreadConnection>) -> App {
        App {
            items: StatefulList::with_items(items)
        }
    }

    pub fn tick(&mut self) {
        for connection in &mut self.items.items {
            connection.pull();
        }
    }
}