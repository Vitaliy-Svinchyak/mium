use std::sync::mpsc::Receiver;

use crate::gui::sync::thread_event::{EventType, ThreadEvent};

pub struct ThreadConnection {
    pub title: String,
    log_channel: Receiver<ThreadEvent>,
    pub log_events: Vec<String>,
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
                        self.log_events.push(e.data);
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
