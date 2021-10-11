use std::sync::mpsc::Sender;

use crate::sync::thread_event::ThreadEvent;

#[derive(Clone)]
pub struct ThreadInfoSender {
    channel_receiver: Sender<ThreadEvent>,
}

impl ThreadInfoSender {
    pub fn new(channel_receiver: Sender<ThreadEvent>) -> ThreadInfoSender {
        ThreadInfoSender { channel_receiver }
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
