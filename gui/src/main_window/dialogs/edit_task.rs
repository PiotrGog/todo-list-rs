extern crate tasks as tasks_model;

use gtk;
use gtk::prelude::*;
use gtk::ComboBoxTextExt;
use relm;
use relm_derive;

use crate::main_window::widgets::task;

#[derive(relm_derive::Msg)]
pub enum EditTaskMsg {
    UpdateTask,
}

pub struct EditTaskModel {
    _relm: relm::Relm<EditTask>,
    task_event_stream: relm::StreamHandle<task::TaskMsg>,
    pub id: u32,
    pub title: String,
    pub description: String,
    status: tasks_model::status::Status,
}

#[relm_derive::widget]
impl relm::Widget for EditTask {
    fn model(
        relm: &relm::Relm<Self>,
        param: (
            relm::StreamHandle<task::TaskMsg>,
            u32,
            String,
            String,
            tasks_model::status::Status,
        ),
    ) -> EditTaskModel {
        EditTaskModel {
            _relm: relm.clone(),
            task_event_stream: param.0,
            id: param.1,
            title: param.2,
            description: param.3,
            status: param.4,
        }
    }

    fn update(&mut self, event: EditTaskMsg) {
        match event {
            EditTaskMsg::UpdateTask => {
                let title = self.widgets.title_entry.get_text().as_str().to_string();
                let description = self
                    .widgets
                    .description_entry
                    .get_text()
                    .as_str()
                    .to_string();

                if title.is_empty() || description.is_empty() {
                    let message = "Title and description cannot be empty!";

                    let dialog = gtk::MessageDialog::new(
                        Some(&self.widgets.window),
                        gtk::DialogFlags::all(),
                        gtk::MessageType::Warning,
                        gtk::ButtonsType::Ok,
                        message,
                    );
                    let result = dialog.run();
                    if result == gtk::ResponseType::Accept {}
                    dialog.close();
                    return;
                }

                let status = self
                    .widgets
                    .status_entry
                    .get_active_id()
                    .unwrap()
                    .as_str()
                    .to_string();

                self.model.task_event_stream.emit(task::TaskMsg::UpdateTask(
                    self.model.id,
                    title,
                    description,
                    if status == tasks_model::status::Status::ToDo.to_string() {
                        tasks_model::status::Status::ToDo
                    } else if status == tasks_model::status::Status::InProgress.to_string() {
                        tasks_model::status::Status::InProgress
                    } else {
                        tasks_model::status::Status::Done
                    },
                ));
            }
        }
    }

    view! {
        #[name="window"]
        gtk::Window {
            property_default_height: 350,
            property_default_width: 350,
            title: "Task Manager [Edit Task]",

            gtk::Box {
                orientation: gtk::Orientation::Vertical,
                gtk::Box {
                    orientation: gtk::Orientation::Vertical,
                    gtk::Label {
                        text: "Task's title:"
                    },
                    #[name="title_entry"]
                    gtk::Entry {
                        text: &self.model.title,
                    },
                },
                gtk::Box {
                    orientation: gtk::Orientation::Vertical,
                    gtk::Label {
                        text: "Task's description:"
                    },
                    #[name="description_entry"]
                    gtk::Entry {
                        text: &self.model.description,
                    },
                },

                gtk::Box {
                    orientation: gtk::Orientation::Vertical,
                    gtk::Label {
                        text: "Task's status:"
                    },

                    #[name="status_entry"]
                    gtk::ComboBoxText {
                        visible: true,
                    },
                },

                #[name="create_task"]
                gtk::Button {
                    label: "Save",
                    clicked => EditTaskMsg::UpdateTask,
                }
            },
        }
    }

    fn init_view(&mut self) {
        println!("Init view");

        let todo_status = tasks_model::status::Status::ToDo.to_string();
        let in_progress_status = tasks_model::status::Status::InProgress.to_string();
        let done_status = tasks_model::status::Status::Done.to_string();
        self.widgets
            .status_entry
            .append(Some(todo_status), todo_status);
        self.widgets
            .status_entry
            .append(Some(in_progress_status), in_progress_status);
        self.widgets
            .status_entry
            .append(Some(done_status), done_status);

        self.widgets
            .status_entry
            .set_active_id(Some(self.model.status.to_string()));
    }
}
