use crate::sync::thread_info_connection::ThreadInfoReceiver;
use crate::gui::util::StatefulList;

pub(super) struct App {
    pub items: StatefulList,
}

impl App {
    pub fn new(items: Vec<ThreadInfoReceiver>) -> App {
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
