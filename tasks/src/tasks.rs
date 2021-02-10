use crate::task::Task;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs;

#[derive(Serialize, Deserialize)]
pub struct Tasks {
    pub tasks: HashMap<u32, Task>,
}

impl Tasks {
    pub fn new() -> Tasks {
        return Tasks {
            tasks: HashMap::<u32, Task>::new(),
        };
    }

    pub fn load_from_file_json(&mut self, filename: &str) -> Result<(), Box<dyn Error>> {
        // TODO: mock file, pass that by arg and test
        let file_content = fs::read_to_string(filename)?;

        let tmp_tasks: Tasks = serde_json::from_str(&file_content)?;
        self.tasks = tmp_tasks.tasks;

        return Ok(());
    }

    pub fn save_to_file_json(&self, filename: &str) -> Result<(), Box<dyn Error>> {
        // TODO: mock file, pass that by arg and test
        let file_content = serde_json::to_string(&self)?;

        fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(filename)?;

        fs::write(filename, file_content)?;

        return Ok(());
    }

    pub fn add_task(&mut self, task: Task) -> bool {
        if self.tasks.contains_key(&task.get_id()) {
            return false;
        }
        self.tasks.insert(task.get_id(), task);
        return true;
    }

    pub fn remove_task(&mut self, id: u32) -> Option<Task> {
        return self.tasks.remove(&id);
    }

    pub fn get_task(&self, id: u32) -> Option<&Task> {
        return self.tasks.get(&id);
    }

    pub fn get_task_mut(&mut self, id: u32) -> Option<&mut Task> {
        return self.tasks.get_mut(&id);
    }

    pub fn size(&self) -> usize {
        return self.tasks.len();
    }
}

#[cfg(test)]
mod tests {
    // TODO: remove duplicated creation of the same tasks in tests
    use super::*;

    #[test]
    fn when_creating_tasks_expect_empty_task_container() {
        let tasks = Tasks::new();
        assert!(tasks.tasks.is_empty());
    }

    #[test]
    fn when_adding_new_task_expect_return_true_and_task_added() {
        let mut tasks = Tasks::new();
        assert!(tasks.tasks.is_empty());

        let id = 10;
        let title = "DummyTitle";
        let description = "DummyDescription";

        let task = Task::new(id, title, description);
        let expected_task = Task::new(id, title, description);

        assert!(tasks.add_task(task));
        assert!(!tasks.tasks.is_empty());
        assert_eq!(1, tasks.tasks.len());
        assert_eq!(&expected_task, tasks.tasks.get(&id).unwrap());
    }

    #[test]
    fn when_task_with_existing_id_expect_return_false_and_task_not_added() {
        let mut tasks = Tasks::new();
        assert!(tasks.tasks.is_empty());

        let id = 10;
        let title = "DummyTitle";
        let description = "DummyDescription";

        let task = Task::new(id, title, description);
        let expected_task = Task::new(id, title, description);

        assert!(tasks.add_task(task));
        assert!(!tasks.tasks.is_empty());
        assert_eq!(1, tasks.tasks.len());
        assert_eq!(&expected_task, tasks.tasks.get(&id).unwrap());

        let id = 10;
        let title = "DummyTitle22";
        let description = "DummyDescription22";
        let other_task_with_the_same_id = Task::new(id, title, description);

        assert!(!tasks.add_task(other_task_with_the_same_id));
        assert!(!tasks.tasks.is_empty());
        assert_eq!(1, tasks.tasks.len());
        assert_eq!(&expected_task, tasks.tasks.get(&id).unwrap());
        let other_task_with_the_same_id = Task::new(id, title, description);
        assert_ne!(&other_task_with_the_same_id, tasks.tasks.get(&id).unwrap());
    }

    #[test]
    fn when_getting_task_which_does_not_exist_expect_return_none() {
        let mut tasks = Tasks::new();
        assert!(tasks.tasks.is_empty());

        let id = 10;
        let title = "DummyTitle";
        let description = "DummyDescription";

        let task = Task::new(id, title, description);
        let not_existing_task_id = 20;

        tasks.add_task(task);
        assert!(!tasks.tasks.is_empty());

        assert_eq!(None, tasks.get_task(not_existing_task_id));
        assert_eq!(None, tasks.get_task_mut(not_existing_task_id));
    }

    #[test]
    fn when_getting_task_which_exists_expect_return_that_task() {
        let mut tasks = Tasks::new();
        assert!(tasks.tasks.is_empty());

        let id = 10;
        let title = "DummyTitle";
        let description = "DummyDescription";

        let task = Task::new(id, title, description);
        let expected_task = Task::new(id, title, description);

        tasks.add_task(task);
        assert!(!tasks.tasks.is_empty());

        assert_eq!(&expected_task, tasks.get_task(id).unwrap());
        assert_eq!(&expected_task, tasks.get_task_mut(id).unwrap());
    }

    #[test]
    fn when_removing_task_which_does_not_exist_expect_return_none_and_none_task_removed() {
        let mut tasks = Tasks::new();
        assert!(tasks.tasks.is_empty());

        let id = 10;
        let title = "DummyTitle";
        let description = "DummyDescription";

        let task = Task::new(id, title, description);
        let not_existing_task_id = 20;

        tasks.add_task(task);
        assert!(!tasks.tasks.is_empty());
        assert_eq!(1, tasks.tasks.len());

        assert_eq!(None, tasks.remove_task(not_existing_task_id));

        assert_eq!(1, tasks.tasks.len());
    }

    #[test]
    fn when_removing_task_which_exists_expect_return_that_task_and_remove() {
        let mut tasks = Tasks::new();
        assert!(tasks.tasks.is_empty());

        let id = 10;
        let title = "DummyTitle";
        let description = "DummyDescription";

        let task = Task::new(id, title, description);
        let expected_task = Task::new(id, title, description);

        tasks.add_task(task);
        assert!(!tasks.tasks.is_empty());
        assert_eq!(1, tasks.tasks.len());

        assert_eq!(expected_task, tasks.remove_task(id).unwrap());

        assert_eq!(0, tasks.tasks.len());
    }

    #[test]
    fn when_tasks_are_saved_to_file_expect_load_tasks_from_file_with_the_same_data() {
        let mut tasks = Tasks::new();

        let id = 10;
        let title = "Title";
        let description = "Description";

        tasks.add_task(Task::new(id, title, description));

        if let Err(error) = tasks.save_to_file_json("./tmp/file.json") {
            assert!(false, "{}", error);
        }

        let mut tasks = Tasks::new();
        assert_eq!(0, tasks.size());

        if let Err(error) = tasks.load_from_file_json("./tmp/file.json") {
            assert!(false, "{}", error);
        }

        assert_eq!(id, tasks.get_task(id).unwrap().get_id());
        assert_eq!(title, tasks.get_task(id).unwrap().title);
        assert_eq!(description, tasks.get_task(id).unwrap().description);
    }
}
