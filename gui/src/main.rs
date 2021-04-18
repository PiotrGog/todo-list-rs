use tasks::tasks::Tasks;

use relm::Widget;

mod main_window;

use std::{cell::RefCell, rc::Rc};

const TASKS_FILE_NAME: &str = "./tmp/tasks.json";

fn main() {
    let mut _tasks = Rc::new(RefCell::new(Tasks::new()));
    if let Err(e) = _tasks.borrow_mut().load_from_file_json(TASKS_FILE_NAME) {
        println!(
            "Loading file {} with tasks failed with error: {}",
            TASKS_FILE_NAME, e
        );
    }
    main_window::Win::run(Rc::clone(&_tasks)).unwrap();
    if let Err(e) = _tasks.borrow_mut().save_to_file_json(TASKS_FILE_NAME) {
        println!(
            "Saving tasks to file {} failed with error: {}",
            TASKS_FILE_NAME, e
        );
    };
}
