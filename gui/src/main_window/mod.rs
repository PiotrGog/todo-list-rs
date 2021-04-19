extern crate tasks as tasks_model;

use gtk;
use gtk::prelude::*;
use relm;
use relm_derive;

use rand::Rng;
use std::{cell::RefCell, rc::Rc};

mod add_task;
mod tasks;

use self::tasks::Column as TasksColumn;

#[derive(Debug, relm_derive::Msg)]
pub enum Msg {
    CreateTask(String, String),
    OpenNewTaskWindow,
    Quit,
}

pub struct Model {
    tasks: Rc<RefCell<tasks_model::tasks::Tasks>>,
    relm: relm::Relm<Win>,
    _win: Option<relm::Component<add_task::AddTask>>,
}

#[relm_derive::widget]
impl relm::Widget for Win {
    fn model(relm: &relm::Relm<Self>, tasks: Rc<RefCell<tasks_model::tasks::Tasks>>) -> Model {
        return Model {
            tasks,
            relm: relm.clone(),
            _win: None,
        };
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::CreateTask(title, description) => {
                let task = tasks_model::task::Task::new(
                    rand::thread_rng().gen(),
                    &title[..],
                    &description[..],
                );
                self.components.to_do_tasks.emit(tasks::Msg::AddTask(
                    task.get_id(),
                    task.title.clone(),
                    task.description.clone(),
                ));
                self.model.tasks.borrow_mut().add_task(task);

                self.model._win.as_ref().unwrap().widget().close();
                self.model._win = None;
            }
            Msg::OpenNewTaskWindow => {
                println!("Msg::OpenNewTaskWindow");
                self.model._win = Some(
                    relm::init::<add_task::AddTask>(self.model.relm.stream().clone())
                        .expect("secondary window"),
                );
            }
            Msg::Quit => {
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
                    clicked => Msg::OpenNewTaskWindow,
                }
            },
            delete_event(_, _) => (Msg::Quit, gtk::Inhibit(false)),
        }
    }

    fn init_view(&mut self) {
        println!("Init view");

        for (_, task) in &self.model.tasks.borrow().tasks {
            match &task.status {
                tasks_model::status::Status::ToDo => {
                    println!("Msg::to_do_tasks");
                    self.components.to_do_tasks.emit(tasks::Msg::AddTask(
                        task.get_id(),
                        task.title.clone(),
                        task.description.clone(),
                    ));
                }
                tasks_model::status::Status::InProgress => {
                    println!("Msg::in_progress_tasks");
                    self.components.in_progress_tasks.emit(tasks::Msg::AddTask(
                        task.get_id(),
                        task.title.clone(),
                        task.description.clone(),
                    ));
                }
                tasks_model::status::Status::Done => {
                    println!("Msg::done_tasks");
                    self.components.done_tasks.emit(tasks::Msg::AddTask(
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
