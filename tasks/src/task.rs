use crate::status::Status;

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Task {
    id: u32,
    pub title: String,
    pub description: String,
    pub status: Status,
}

impl Task {
    pub fn new(id: u32, title: &str, description: &str) -> Task {
        return Task {
            id,
            title: title.to_string(),
            description: description.to_string(),
            status: Status::ToDo,
        };
    }

    pub fn get_id(&self) -> u32 {
        return self.id;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn when_creating_task_expect_new_task_with_given_params() {
        let id = 10;
        let title = "DummyTitle";
        let description = "DummyDescription";
        let actual_task = Task::new(id, title, description);

        assert!(actual_task.id == id);
        assert!(actual_task.title == title);
        assert!(actual_task.description == description);
        assert!(actual_task.status == Status::ToDo);
    }
}
