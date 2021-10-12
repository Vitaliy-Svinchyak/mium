use tui::widgets::ListState;

use crate::sync::thread_info_connection::ThreadInfoReceiver;

pub struct StatefulList
{
    pub state: ListState,
    pub items: Vec<ThreadInfoReceiver>,
    pub selected: Option<usize>,
}

impl StatefulList {
    pub fn with_items(items: Vec<ThreadInfoReceiver>) -> StatefulList {
        StatefulList {
            state: ListState::default(),
            items,
            selected: None,
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
        self.selected = Some(i);
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
        self.selected = Some(i);
    }

    pub fn unselect(&mut self) {
        self.state.select(None);
        self.selected = None;
    }

    pub fn get_selected_logs(&self) -> Vec<String> {
        match self.selected {
            None => {
                vec![]
            }
            Some(index) => self.items[index].log_events.clone(),
        }
    }
}
