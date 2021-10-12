use crate::gui::util::StatefulList;
use crate::sync::thread_info_connection::ThreadInfoReceiver;

pub struct App {
    pub menu_items: StatefulList<ThreadInfoReceiver>,
    pub log_items: StatefulList<String>,
    pages: usize,
    progress_by_tick: Vec<u64>,
}

impl App {
    pub fn new(items: Vec<ThreadInfoReceiver>, pages: usize) -> App {
        App {
            menu_items: StatefulList::with_items(items),
            pages,
            progress_by_tick: vec![0],
            log_items: StatefulList::with_items(vec![]),
        }
    }

    pub fn tick(&mut self) {
        for connection in &mut self.menu_items.items {
            connection.pull();
        }

        if let Some(i) = self.menu_items.state.selected() {
            let mut last_synced = self.log_items.len();
            let connection_logs = self.menu_items.items[i].log_events.clone();
            let connection_last_synced = connection_logs.len();

            while last_synced < connection_last_synced {
                self.log_items.add(connection_logs[last_synced].clone());
                last_synced += 1;
            }
        }

        let current_progress = self.progress();
        self.progress_by_tick.push(current_progress);
    }

    pub fn progress_history(&self) -> Vec<u64> {
        let mut progress = vec![];

        let mut prev_progress = 0;
        for p in &self.progress_by_tick {
            progress.push(p - prev_progress);
            prev_progress = *p;
        }

        progress
    }

    pub fn total_progress(&self) -> f64 {
        let parse_pages = self.pages;
        let download_images = parse_pages * 24;
        let accumulate_images = parse_pages * 24;
        let summarize_images = parse_pages;

        let progress_needed = parse_pages + download_images + accumulate_images + summarize_images;
        let progress_current = self.progress();

        progress_current as f64 / progress_needed as f64
    }

    fn progress(&self) -> u64 {
        self.menu_items
            .items
            .iter()
            .fold(0_u64, |t, v| t + v.progress)
    }
}
