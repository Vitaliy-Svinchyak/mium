#[derive(Debug, Clone)]
pub enum EventType {
    INFO,
    ERROR,
    PROGRESS,
    CLOSED,
}

#[derive(Debug, Clone)]
pub struct ThreadEvent {
    pub lvl: EventType,
    pub data: String,
}

impl ThreadEvent {
    pub fn progress() -> ThreadEvent {
        ThreadEvent {
            lvl: EventType::PROGRESS,
            data: "".to_owned(),
        }
    }

    pub fn closed() -> ThreadEvent {
        ThreadEvent {
            lvl: EventType::CLOSED,
            data: "".to_owned(),
        }
    }

    pub fn info(data: String) -> ThreadEvent {
        ThreadEvent {
            lvl: EventType::INFO,
            data,
        }
    }
}
