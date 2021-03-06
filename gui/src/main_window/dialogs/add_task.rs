use gtk;
use gtk::prelude::*;
use relm;
use relm_derive;

use crate::main_window::main_window;

#[derive(relm_derive::Msg)]
pub enum AddTaskMsg {
    CreateTask,
}

pub struct Model {
    main_window_event_stream: relm::StreamHandle<main_window::MainWindowMsg>,
}

#[relm_derive::widget]
impl relm::Widget for AddTask {
    fn model(main_window_event_stream: relm::StreamHandle<main_window::MainWindowMsg>) -> Model {
        return Model {
            main_window_event_stream,
        };
    }

    fn update(&mut self, event: AddTaskMsg) {
        match event {
            AddTaskMsg::CreateTask => {
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

                self.model
                    .main_window_event_stream
                    .emit(main_window::MainWindowMsg::NewTaskWindowCallback { title, description });
            }
        }
    }

    view! {
        #[name="window"]
        gtk::Window {
            property_default_height: 350,
            property_default_width: 350,
            title: "Task Manager [Add Task]",

            gtk::Box {
                orientation: gtk::Orientation::Vertical,
                gtk::Box {
                    orientation: gtk::Orientation::Vertical,
                    gtk::Label {
                        text: "Task's title:"
                    },
                    #[name="title_entry"]
                    gtk::Entry {},
                },
                gtk::Box {
                    orientation: gtk::Orientation::Vertical,
                    gtk::Label {
                        text: "Task's description:"
                    },
                    #[name="description_entry"]
                    gtk::Entry {},
                },

                #[name="create_task"]
                gtk::Button {
                    label: "Create",
                    clicked => AddTaskMsg::CreateTask,
                }
            },
        }
    }
}
