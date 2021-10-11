use std::sync::mpsc::Receiver;

use crate::sync::thread_event::{EventType, ThreadEvent};

pub struct ThreadInfoReceiver {
    pub title: String,
    channel_receiver: Receiver<ThreadEvent>,
    pub log_events: Vec<String>,
    pub progress: u64,
    pub closed: bool,
}

impl ThreadInfoReceiver {
    pub fn new(title: String, channel_receiver: Receiver<ThreadEvent>) -> ThreadInfoReceiver {
        ThreadInfoReceiver {
            title,
            channel_receiver,
            log_events: vec![],
            progress: 0,
            closed: false,
        }
    }

    pub fn pull(&mut self) {
        loop {
            let event = self.channel_receiver.try_recv();

            match event {
                Ok(e) => match e.lvl {
                    EventType::INFO => {
                        self.log_events.push(e.data);
                    }
                    EventType::ERROR => {}
                    EventType::PROGRESS => {
                        self.progress += 1;
                    }
                    EventType::CLOSED => {
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
