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
    AddTask(u32, String, String, tasks_model::status::Status),
    UpdateTask(u32, String, String, tasks_model::status::Status),
    DeleteTask(u32),
}

pub struct Model {
    relm: relm::Relm<Column>,
    main_window_event_stream: relm::StreamHandle<main_window::MainWindowMsg>,
    label: String,
    tasks: HashMap<u32, relm::Component<task::Task>>,
    status: tasks_model::status::Status,
}

#[relm_derive::widget]
impl relm::Widget for Column {
    fn model(
        relm: &relm::Relm<Self>,
        param: (
            relm::StreamHandle<main_window::MainWindowMsg>,
            tasks_model::status::Status,
        ),
    ) -> Model {
        return Model {
            relm: relm.clone(),
            main_window_event_stream: param.0,
            label: param.1.to_string().to_string(),
            tasks: HashMap::<u32, relm::Component<task::Task>>::new(),
            status: param.1,
        };
    }

    fn update(&mut self, event: ColumnMsg) {
        match event {
            ColumnMsg::AddTask(id, title, description, status) => {
                println!("Msg::AddTask({}, {}, {})", id, title, description);
                let component = self.widgets.column_tasks.add_widget::<task::Task>((
                    self.model.relm.stream().clone(),
                    id,
                    title,
                    description,
                    status,
                ));
                self.model.tasks.insert(id, component);
            }
            ColumnMsg::UpdateTask(id, title, description, status) => {
                println!("Msg::UpdateTask({}, {}, {})", id, title, description);

                if status != self.model.status {
                    self.delete_task_widget(id)
                }

                self.model
                    .main_window_event_stream
                    .emit(main_window::MainWindowMsg::UpdateTask {
                        id,
                        title,
                        description,
                        status,
                    });
            }
            ColumnMsg::DeleteTask(id) => {
                self.delete_task_widget(id);
                self.model
                    .main_window_event_stream
                    .emit(main_window::MainWindowMsg::DeleteTask { id })
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

impl Column {
    fn delete_task_widget(&mut self, id: u32) {
        let task_widget = self.model.tasks.remove(&id).unwrap();
        self.widgets.column_tasks.remove_widget(task_widget);
    }
}
