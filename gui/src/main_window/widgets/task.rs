extern crate tasks as tasks_model;

use gtk;
use gtk::prelude::*;
use relm;
use relm_derive;

use gtk::{LabelExt, OrientableExt};

use crate::main_window::dialogs::edit_task;
use crate::main_window::widgets::column;

#[allow(dead_code)]
#[derive(relm_derive::Msg)]
pub enum TaskMsg {
    SetTitle(String),
    SetDescription(String),
    UpdateTask(u32, String, String, tasks_model::status::Status),
    OpenEditTaskWindow,
    Delete,
}

pub struct TaskModel {
    relm: relm::Relm<Task>,
    column_widget_event_stream: relm::StreamHandle<column::ColumnMsg>,
    pub id: u32,
    pub title: String,
    pub description: String,
    status: tasks_model::status::Status,
    edit_task_window: Option<relm::Component<edit_task::EditTask>>,
}

#[relm_derive::widget]
impl relm::Widget for Task {
    fn model(
        relm: &relm::Relm<Self>,
        param: (
            relm::StreamHandle<column::ColumnMsg>,
            u32,
            String,
            String,
            tasks_model::status::Status,
        ),
    ) -> TaskModel {
        TaskModel {
            relm: relm.clone(),
            column_widget_event_stream: param.0,
            id: param.1,
            title: param.2,
            description: param.3,
            status: param.4,
            edit_task_window: None,
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
            TaskMsg::OpenEditTaskWindow => {
                println!("EditTask");
                self.model.edit_task_window = Some(
                    relm::init::<edit_task::EditTask>((
                        self.model.relm.stream().clone(),
                        self.model.id,
                        self.model.title.clone(),
                        self.model.description.clone(),
                        self.model.status.clone(),
                    ))
                    .expect("secondary window"),
                );
            }
            TaskMsg::UpdateTask(id, title, description, status) => {
                self.model
                    .edit_task_window
                    .as_ref()
                    .unwrap()
                    .widget()
                    .close();
                self.model.edit_task_window = None;
                self.model.description = description.clone();
                self.model.title = title.clone();

                self.model
                    .column_widget_event_stream
                    .emit(column::ColumnMsg::UpdateTask(
                        id,
                        title,
                        description,
                        status,
                    ))
            }
            TaskMsg::Delete => {
                println!("DeleteTask");
                self.model
                    .column_widget_event_stream
                    .emit(column::ColumnMsg::DeleteTask(self.model.id));
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
                    clicked => TaskMsg::OpenEditTaskWindow,
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
