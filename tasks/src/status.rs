use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}

impl Status {
    pub fn to_string(&self) -> &str {
        match self {
            Status::ToDo => "To Do",
            Status::InProgress => "In progress",
            Status::Done => "Done",
        }
    }
}
