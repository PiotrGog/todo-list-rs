use gtk::prelude::*;
use gtk;
use relm;
use relm_derive;

mod tasks;

use self::tasks::Column as TasksColumn;
use self::tasks::Task as Task;

#[derive(relm_derive::Msg)]
pub enum Msg {
    AddNewTask,
    Quit,
}

pub struct Model {
    a: String,
    b: String,
}

#[relm_derive::widget]
impl relm::Widget for Win {
    fn model(_: &relm::Relm<Self>, param: (String, String)) -> Model {
        return Model{
            a: param.0,
            b: param.1,
        };
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::AddNewTask => {
                println!("Msg::AddNewTask");
                self.components.to_do_tasks.emit(
                    tasks::Msg::AddTask(self.model.a.clone(), self.model.b.clone())
                );
            },
            Msg::Quit => {
                println!("Msg::Quit");
                gtk::main_quit()
            },
        }
    }

    view! {
        gtk::Window {
            gtk::Box {
                orientation: gtk::Orientation::Horizontal,

                #[name="to_do_tasks"]
                TasksColumn("To do".to_string()),

                #[name="in_progress_tasks"]
                TasksColumn("In progress".to_string()),

                #[name="done_tasks"]
                TasksColumn("Done".to_string()),

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
}
