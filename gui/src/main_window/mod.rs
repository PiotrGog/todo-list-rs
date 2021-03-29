use gtk::prelude::*;
use gtk;
use relm;
use relm_derive;

mod tasks;

use self::tasks::Column as TasksColumn;

#[derive(relm_derive::Msg)]
pub enum Msg {
    Quit,
}

pub struct Model {
}

#[relm_derive::widget]
impl relm::Widget for Win {
    fn model() -> Model {
        return Model{};
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::Quit => gtk::main_quit(),
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
            },
            // Use a tuple when you want to both send a message and return a value to
            // the GTK+ callback.
            delete_event(_, _) => (Msg::Quit, gtk::Inhibit(false)),
        }
    }
}
