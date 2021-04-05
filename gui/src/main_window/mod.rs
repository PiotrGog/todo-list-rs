extern crate tasks as tasks_model;

use gtk;
use gtk::prelude::*;
use relm;
use relm_derive;

use std::{cell::RefCell, rc::Rc};

mod tasks;

use self::tasks::Column as TasksColumn;
use self::tasks::Task;

#[derive(relm_derive::Msg)]
pub enum Msg {
    AddNewTask,
    Quit,
}

pub struct Model {
    tasks: Rc<RefCell<tasks_model::tasks::Tasks>>,
}

#[relm_derive::widget]
impl relm::Widget for Win {
    fn model(_: &relm::Relm<Self>, tasks: Rc<RefCell<tasks_model::tasks::Tasks>>) -> Model {
        return Model {
            tasks
        };
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::AddNewTask => {
                println!("Msg::AddNewTask");
                // for (_, task) in &self.model.tasks.borrow().tasks {
                //     self.components.to_do_tasks.emit(
                //         tasks::Msg::AddTask(task.get_id(), task.title.clone(), task.description.clone())
                //     );
                // }
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
                    clicked => Msg::AddNewTask,
                }
            },
            // Use a tuple when you want to both send a message and return a value to
            // the GTK+ callback.
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
                not_known => {
                    panic!("Not known task's status type {:?}", not_known);
                }
            }
        }
    }
}
