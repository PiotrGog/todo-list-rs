use gtk;
use gtk::prelude::*;
use relm;
use relm_derive;

use crate::main_window::Msg as MainWindowMsg;

#[derive(relm_derive::Msg)]
pub enum Msg {
    CreateTask,
}

pub struct Model {
    event_stream: relm::StreamHandle<MainWindowMsg>,
}

#[relm_derive::widget]
impl relm::Widget for AddTask {
    fn model(win_stream: relm::StreamHandle<MainWindowMsg>) -> Model {
        return Model {
            event_stream: win_stream,
        };
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::CreateTask => {
                let title = self.widgets.title_entry.get_text().as_str().to_string();
                let description = self
                    .widgets
                    .description_entry
                    .get_text()
                    .as_str()
                    .to_string();
                self.model
                    .event_stream
                    .emit(MainWindowMsg::CreateTask(title, description));
            }
        }
    }

    view! {
        gtk::Window {
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
                    clicked => Msg::CreateTask,
                }
            },
        }
    }
}
