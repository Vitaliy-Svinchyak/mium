use std::sync::mpsc::Sender as StdSender;
use std::sync::mpsc::{channel, Receiver as StdReceiver};

use crossbeam_channel::{bounded, Receiver, Sender};

pub struct ThreadBroadcaster<T> {
    receiver: StdReceiver<Option<T>>,
    sender: Sender<Option<T>>,
    finished_threads: usize,
    threads_amount: usize,
}

impl<T> ThreadBroadcaster<T> {
    pub fn new(
        receiver: StdReceiver<Option<T>>,
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
    pub fn all_in_one(
        threads_amount: usize,
    ) -> (
        StdSender<Option<T>>,
        ThreadBroadcaster<T>,
        Receiver<Option<T>>,
    ) {
        let (std_s, std_r) = channel();
        let (s, r) = bounded(threads_amount);
        (
            std_s,
            ThreadBroadcaster {
                receiver: std_r,
                sender: s,
                finished_threads: 0,
                threads_amount,
            },
            r,
        )
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
