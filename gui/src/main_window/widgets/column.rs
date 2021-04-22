extern crate tasks as tasks_model;

use gtk;
use gtk::prelude::*;
use relm;
use relm_derive;
use std::collections::HashMap;

use gtk::{LabelExt, OrientableExt};
use relm::ContainerWidget;

use crate::main_window::main_window;
use crate::main_window::widgets::task;

#[derive(relm_derive::Msg)]
pub enum ColumnMsg {
    AddTask(
        relm::StreamHandle<main_window::MainWindowMsg>,
        u32,
        String,
        String,
        tasks_model::status::Status,
    ),
    DeleteTask(u32),
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
            ColumnMsg::AddTask(main_window_event_stream, id, title, description, status) => {
                println!("Msg::AddTask({}, {}, {})", id, title, description);
                let component = self.widgets.column_tasks.add_widget::<task::Task>((
                    main_window_event_stream,
                    id,
                    title,
                    description,
                    status,
                ));
                self.model.tasks.insert(id, component);
            }
            ColumnMsg::DeleteTask(task_id) => {
                let task_widget = self.model.tasks.remove(&task_id).unwrap();
                self.widgets.column_tasks.remove_widget(task_widget);
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
