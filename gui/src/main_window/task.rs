extern crate tasks as tasks_model;

use gtk;
use gtk::prelude::*;
use relm;
use relm_derive;

use gtk::{LabelExt, OrientableExt};

use crate::main_window::main_window;

#[allow(dead_code)]
#[derive(relm_derive::Msg)]
pub enum TaskMsg {
    SetTitle(String),
    SetDescription(String),
    Edit,
    Delete,
}

pub struct TaskModel {
    main_window_event_stream: relm::StreamHandle<main_window::MainWindowMsg>,
    pub id: u32,
    pub title: String,
    pub description: String,
    status: tasks_model::status::Status,
}

#[relm_derive::widget]
impl relm::Widget for Task {
    fn model(
        param: (
            relm::StreamHandle<main_window::MainWindowMsg>,
            u32,
            String,
            String,
            tasks_model::status::Status,
        ),
    ) -> TaskModel {
        TaskModel {
            main_window_event_stream: param.0,
            id: param.1,
            title: param.2,
            description: param.3,
            status: param.4,
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
            TaskMsg::Delete => {
                println!("DeleteTask");
                self.model
                    .main_window_event_stream
                    .emit(main_window::MainWindowMsg::DeleteTask(
                        self.model.id,
                        self.model.status.clone(),
                    ));
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
                },

                #[name="delete_button"]
                gtk::Button {
                    label: "Delete",
                    clicked => TaskMsg::Delete,
                }
            },
        }
    }
}
