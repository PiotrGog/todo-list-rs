mod main_window;

extern crate tasks as tasks_model;

use relm::Widget;

use std::cell::RefCell;
use std::rc::Rc;

pub fn run_gui_app(tasks: Rc<RefCell<tasks_model::tasks::Tasks>>) {
    main_window::main_window::MainWindow::run(tasks).unwrap();
}
