use gtk;
use gtk::prelude::*;
use relm;
use relm_derive;
use std::collections::HashMap;

use gtk::{LabelExt, OrientableExt};
use relm::ContainerWidget;

#[allow(dead_code)]
#[derive(relm_derive::Msg)]
pub enum TaskMsg {
    SetTitle(String),
    SetDescription(String),
    Edit,
}

pub struct TaskModel {
    pub id: u32,
    pub title: String,
    pub description: String,
}

#[relm_derive::widget]
impl relm::Widget for Task {
    fn model(param: (u32, String, String)) -> TaskModel {
        TaskModel {
            id: param.0,
            title: param.1,
            description: param.2,
        }
    }

    fn update(&mut self, event: TaskMsg) {
        match event {
            TaskMsg::SetDescription(description) => {
                println!("SetDescription({})", description);
                self.model.description = description;
            }
            TaskMsg::SetTitle(title) => {
                println!("SetTitle({})", title);
                self.model.title = title;
            }
            TaskMsg::Edit => {
                println!("EditTask");
            }
        }
    }

    view! {
        gtk::Frame {
            gtk::Box {
                orientation: gtk::Orientation::Vertical,
                border_width: 10,

                #[name="label"]
                gtk::Label {
                    text: &self.model.title,
                },

                #[name="description"]
                gtk::Label {
                    text: &self.model.description,
                },

                #[name="edit_button"]
                gtk::Button {
                    label: "Edit",
                    clicked => TaskMsg::Edit,
                }
            },
        }
    }
}

#[derive(relm_derive::Msg)]
pub enum Msg {
    AddTask(u32, String, String),
}

pub struct Model {
    label: String,
    tasks: HashMap<u32, relm::Component<Task>>,
}

#[relm_derive::widget]
impl relm::Widget for Column {
    fn model(column_name: String) -> Model {
        return Model {
            label: column_name,
            tasks: HashMap::new(),
        };
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::AddTask(id, title, description) => {
                println!("Msg::AddTask({}, {}, {})", id, title, description);
                let component =
                    self.widgets
                        .column_tasks
                        .add_widget::<Task>((id, title, description));
                self.model.tasks.insert(id, component);
            }
        }
    }

    view! {
        gtk::Frame {
            gtk::Box {
                orientation: gtk::Orientation::Vertical,
                border_width: 10,
                spacing: 20,

                #[name="column_name"]
                gtk::Label {
                    label: &self.model.label,
                },

                #[name="column_tasks"]
                gtk::Box {
                    spacing: 10,
                    orientation: gtk::Orientation::Vertical,
                },
            },
        },
    }
}
