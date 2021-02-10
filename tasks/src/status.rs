use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}
