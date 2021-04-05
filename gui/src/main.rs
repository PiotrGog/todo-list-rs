use tasks::task::Task;
use tasks::tasks::Tasks;

use relm::Widget;

mod main_window;

use std::{
    rc::Rc,
    cell::RefCell
};

const TASKS_FILE_NAME: &str = "./tmp/tasks.json";

fn main() {
    let mut tasks = Rc::new(RefCell::new(Tasks::new()));
    if let Err(e) = tasks.borrow_mut().load_from_file_json(TASKS_FILE_NAME) {
        println!("Loading file with tasks failes with error: {}", e);
    }
    main_window::Win::run(Rc::clone(&tasks)).unwrap();
}
