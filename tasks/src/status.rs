use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}
