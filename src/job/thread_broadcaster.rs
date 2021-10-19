use std::sync::mpsc::Sender as StdSender;
use std::sync::mpsc::{channel, Receiver as StdReceiver};
use std::thread;

use crossbeam_channel::{bounded, Receiver, Sender};

struct ThreadBroadcaster<T> {
    receiver: StdReceiver<Option<T>>,
    sender: Sender<Option<T>>,
    finished_threads: usize,
    threads_amount: usize,
}

impl<T> ThreadBroadcaster<T>
where
    T: Send + 'static,
{
    pub fn new(threads_amount: usize) -> (StdSender<Option<T>>, Receiver<Option<T>>) {
        let (std_s, std_r) = channel();
        let (s, r) = bounded(threads_amount);

        let mut broadcaster = ThreadBroadcaster {
            receiver: std_r,
            sender: s,
            finished_threads: 0,
            threads_amount,
        };
        thread::spawn(move || {
            broadcaster.tick();
        });

        (std_s, r)
    }

    fn tick(&mut self) {
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

pub fn broadcast_channel<T>(threads_amount: usize) -> (StdSender<Option<T>>, Receiver<Option<T>>)
where
    T: Send + 'static,
{
    ThreadBroadcaster::new(threads_amount)
}
