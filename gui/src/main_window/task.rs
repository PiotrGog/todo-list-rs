use gtk;
use gtk::prelude::*;
use relm;
use relm_derive;

use gtk::{LabelExt, OrientableExt};

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
