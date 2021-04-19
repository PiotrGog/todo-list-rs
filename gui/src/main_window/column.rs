// mod task;

use gtk;
use gtk::prelude::*;
use relm;
use relm_derive;
use std::collections::HashMap;

use gtk::{LabelExt, OrientableExt};
use relm::ContainerWidget;

use crate::main_window::task;

#[derive(relm_derive::Msg)]
pub enum ColumnMsg {
    AddTask(u32, String, String),
}

pub struct Model {
    label: String,
    tasks: HashMap<u32, relm::Component<task::Task>>,
}

#[relm_derive::widget]
impl relm::Widget for Column {
    fn model(column_name: String) -> Model {
        return Model {
            label: column_name,
            tasks: HashMap::<u32, relm::Component<task::Task>>::new(),
        };
    }

    fn update(&mut self, event: ColumnMsg) {
        match event {
            ColumnMsg::AddTask(id, title, description) => {
                println!("Msg::AddTask({}, {}, {})", id, title, description);
                let component =
                    self.widgets
                        .column_tasks
                        .add_widget::<task::Task>((id, title, description));
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
