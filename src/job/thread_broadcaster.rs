use std::sync::mpsc::Receiver;

use crossbeam_channel::Sender;

pub struct ThreadBroadcaster<T> {
    receiver: Receiver<Option<T>>,
    sender: Sender<Option<T>>,
    finished_threads: usize,
    threads_amount: usize,
}

impl<T> ThreadBroadcaster<T> {
    pub fn new(
        receiver: Receiver<Option<T>>,
        sender: Sender<Option<T>>,
        threads_amount: usize,
    ) -> ThreadBroadcaster<T> {
        ThreadBroadcaster {
            receiver,
            sender,
            finished_threads: 0,
            threads_amount,
        }
    }

    pub fn tick(&mut self) {
        for data in &self.receiver {
            match data {
                None => {
                    self.finished_threads += 1;

                    if self.finished_threads >= self.threads_amount {
                        loop {
                            match self.sender.send(None) {
                                Err(_) => break,
                                _ => {}
                            }
                        }
                    }
                }
                Some(d) => {
                    self.sender
                        .send(Some(d))
                        .expect("Can't send data into sender");
                }
            }
        }
    }
}
