extern crate tasks as tasks_model;

use gtk;
use gtk::prelude::*;
use relm;
use relm_derive;

use rand::Rng;
use std::{cell::RefCell, rc::Rc};

use super::add_task;
use super::column;
use super::column::Column as TasksColumn;

#[derive(Debug, relm_derive::Msg)]
pub enum MainWindowMsg {
    CreateTask(String, String),
    OpenNewTaskWindow,
    Quit,
}

pub struct Model {
    tasks: Rc<RefCell<tasks_model::tasks::Tasks>>,
    relm: relm::Relm<MainWindow>,
    add_task_window: Option<relm::Component<add_task::AddTask>>,
}

#[relm_derive::widget]
impl relm::Widget for MainWindow {
    fn model(relm: &relm::Relm<Self>, tasks: Rc<RefCell<tasks_model::tasks::Tasks>>) -> Model {
        return Model {
            tasks,
            relm: relm.clone(),
            add_task_window: None,
        };
    }

    fn update(&mut self, event: MainWindowMsg) {
        match event {
            MainWindowMsg::CreateTask(title, description) => {
                let task = tasks_model::task::Task::new(
                    rand::thread_rng().gen(),
                    &title[..],
                    &description[..],
                );
                self.components.to_do_tasks.emit(column::ColumnMsg::AddTask(
                    task.get_id(),
                    task.title.clone(),
                    task.description.clone(),
                ));
                self.model.tasks.borrow_mut().add_task(task);

                self.model
                    .add_task_window
                    .as_ref()
                    .unwrap()
                    .widget()
                    .close();
                self.model.add_task_window = None;
            }
            MainWindowMsg::OpenNewTaskWindow => {
                println!("Msg::OpenNewTaskWindow");
                self.model.add_task_window = Some(
                    relm::init::<add_task::AddTask>(self.model.relm.stream().clone())
                        .expect("secondary window"),
                );
            }
            MainWindowMsg::Quit => {
                println!("Msg::Quit");
                gtk::main_quit()
            }
        }
    }

    view! {
        gtk::Window {
            gtk::Box {
                orientation: gtk::Orientation::Vertical,

                gtk::Box {
                    orientation: gtk::Orientation::Horizontal,

                    #[name="to_do_tasks"]
                    TasksColumn("To do".to_string()),

                    #[name="in_progress_tasks"]
                    TasksColumn("In progress".to_string()),

                    #[name="done_tasks"]
                    TasksColumn("Done".to_string()),

                },

                #[name="new_task_button"]
                gtk::Button {
                    label: "Add",
                    clicked => MainWindowMsg::OpenNewTaskWindow,
                }
            },
            delete_event(_, _) => (MainWindowMsg::Quit, gtk::Inhibit(false)),
        }
    }

    fn init_view(&mut self) {
        println!("Init view");

        for (_, task) in &self.model.tasks.borrow().tasks {
            match &task.status {
                tasks_model::status::Status::ToDo => {
                    println!("Msg::to_do_tasks");
                    self.components.to_do_tasks.emit(column::ColumnMsg::AddTask(
                        task.get_id(),
                        task.title.clone(),
                        task.description.clone(),
                    ));
                }
                tasks_model::status::Status::InProgress => {
                    println!("Msg::in_progress_tasks");
                    self.components
                        .in_progress_tasks
                        .emit(column::ColumnMsg::AddTask(
                            task.get_id(),
                            task.title.clone(),
                            task.description.clone(),
                        ));
                }
                tasks_model::status::Status::Done => {
                    println!("Msg::done_tasks");
                    self.components.done_tasks.emit(column::ColumnMsg::AddTask(
                        task.get_id(),
                        task.title.clone(),
                        task.description.clone(),
                    ));
                }
                #[allow(unreachable_patterns)]
                _not_known => {
                    panic!("Not known task's status type {:?}", _not_known);
                }
            }
        }
    }
}
