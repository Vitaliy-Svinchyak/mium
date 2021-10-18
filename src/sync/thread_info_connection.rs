use std::sync::mpsc::Receiver;

use crate::sync::thread_event::{EventType, ThreadEvent};

#[derive(Clone)]
pub enum TypedLog {
    Info(String),
    Error(String),
}

impl TypedLog {
    pub const fn is_info(&self) -> bool {
        matches!(*self, TypedLog::Info(_))
    }
    pub const fn is_error(&self) -> bool {
        matches!(*self, TypedLog::Error(_))
    }

    pub fn data(self) -> String {
        match self {
            TypedLog::Info(d) => d,
            TypedLog::Error(d) => d,
        }
    }
}

pub struct ThreadInfoReceiver {
    pub title: String,
    channel_receiver: Receiver<ThreadEvent>,
    pub log_events: Vec<TypedLog>,
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

    pub fn has_errors(&self) -> bool {
        self.log_events.iter().find(|v| v.is_error()).is_some()
    }

    pub fn pull(&mut self) {
        loop {
            let event = self.channel_receiver.try_recv();

            match event {
                Ok(e) => match e.lvl {
                    EventType::INFO => {
                        self.log_events.push(TypedLog::Info(e.data));
                    }
                    EventType::ERROR => {
                        self.log_events.push(TypedLog::Error(e.data));
                    }
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
