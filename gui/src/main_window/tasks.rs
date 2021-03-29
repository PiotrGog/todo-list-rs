use gtk::prelude::*;
use gtk;
use relm;
use relm_derive;
use std::collections::HashMap;

#[derive(relm_derive::Msg)]
pub enum TaskMsg {

}

pub struct TaskModel {
    title: String,
    description: String
}

#[relm_derive::widget]
impl relm::Widget for Task {
    fn model() -> TaskModel {
        TaskModel {
            title: "".to_string(),
            description: "".to_string(),
        }
    }

    fn update(&mut self, event: TaskMsg) {

    }

    view! {
        gtk::Box {
            orientation: gtk::Orientation::Vertical,
            gtk::Label {
                text: &self.model.title,
            },
            gtk::Label {
                text: &self.model.description,
            }

        }
    }
}

#[derive(relm_derive::Msg)]
pub enum Msg {
}

pub struct Model {
    label: String,
    tasks: HashMap<u32, relm::Component<Task>>
}

#[relm_derive::widget]
impl relm::Widget for Column {
    fn model(column_name: String) -> Model {
        return Model {
            label: column_name,
            tasks: HashMap::new(),
        }
    }

    fn update(&mut self, event: Msg) {
        match event {

        }
    }

    view! {
        gtk::Box {
            orientation: gtk::Orientation::Vertical,
            #[name="column_name"]
            gtk::Label {
                label: &self.model.label,
            },
            gtk::Box {
                orientation: gtk::Orientation::Vertical,
            },
        },
    }
}
