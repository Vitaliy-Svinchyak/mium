#[derive(Debug, Clone)]
pub enum EventType {
    INFO,
    ERROR,
    PROGRESS,
    CLOSE,
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

    pub fn close() -> ThreadEvent {
        ThreadEvent {
            lvl: EventType::CLOSE,
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
