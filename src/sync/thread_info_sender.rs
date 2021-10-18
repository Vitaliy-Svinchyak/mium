use std::sync::mpsc::Sender;

use anyhow::Error;

use crate::sync::thread_event::ThreadEvent;

#[derive(Clone)]
pub struct ThreadInfoSender {
    channel_receiver: Sender<ThreadEvent>,
}

impl ThreadInfoSender {
    pub fn new(channel_receiver: Sender<ThreadEvent>) -> ThreadInfoSender {
        ThreadInfoSender { channel_receiver }
    }

    pub fn error(&self, data: Error) {
        self.channel_receiver
            .send(ThreadEvent::error(data.to_string()))
            .expect("Can't send info event");
    }

    pub fn info(&self, data: String) {
        self.channel_receiver
            .send(ThreadEvent::info(data))
            .expect("Can't send info event");
    }

    pub fn progress(&self) {
        self.channel_receiver
            .send(ThreadEvent::progress())
            .expect("Can't send progress event");
    }

    pub fn closed(&self) {
        self.info("Closed.".to_owned());
        self.channel_receiver
            .send(ThreadEvent::closed())
            .expect("Can't send closed event");
    }
}
