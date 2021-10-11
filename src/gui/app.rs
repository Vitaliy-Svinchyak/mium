use std::sync::mpsc::Receiver;

use crate::gui::util::StatefulList;

#[derive(Debug, Clone)]
pub enum EventType {
    INFO,
    ERROR,
    PROGRESS,
    CLOSE,
}

#[derive(Debug, Clone)]
pub struct ThreadEvent {
    pub lvl: EventType,
    pub data: String,
}

impl ThreadEvent {
    pub fn progress() -> ThreadEvent {
        ThreadEvent {
            lvl: EventType::PROGRESS,
            data: "".to_owned(),
        }
    }

    pub fn close() -> ThreadEvent {
        ThreadEvent {
            lvl: EventType::CLOSE,
            data: "".to_owned(),
        }
    }

    pub fn info(data: String) -> ThreadEvent {
        ThreadEvent {
            lvl: EventType::INFO,
            data,
        }
    }
}

pub struct ThreadConnection {
    pub title: String,
    log_channel: Receiver<ThreadEvent>,
    pub log_events: Vec<ThreadEvent>,
    pub progress: u64,
    pub closed: bool,
}

impl ThreadConnection {
    pub fn new(title: String, log_channel: Receiver<ThreadEvent>) -> ThreadConnection {
        ThreadConnection {
            title,
            log_channel,
            log_events: vec![],
            progress: 0,
            closed: false,
        }
    }

    pub fn pull(&mut self) {
        loop {
            let event = self.log_channel.try_recv();

            match event {
                Ok(e) => match e.lvl {
                    EventType::INFO => {
                        self.log_events.push(e);
                    }
                    EventType::ERROR => {}
                    EventType::PROGRESS => {
                        self.progress += 1;
                    }
                    EventType::CLOSE => {
                        self.closed = true;
                    }
                },
                Err(_) => {
                    break;
                }
            }
        }
    }
}

pub(super) struct App {
    pub items: StatefulList,
}

impl App {
    pub fn new(items: Vec<ThreadConnection>) -> App {
        App {
            items: StatefulList::with_items(items),
        }
    }

    pub fn tick(&mut self) {
        for connection in &mut self.items.items {
            connection.pull();
        }
    }
}
