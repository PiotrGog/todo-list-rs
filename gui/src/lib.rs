extern crate gdk;
extern crate gio;
extern crate glib;
extern crate gtk;

use tasks::task::Task;
use tasks::tasks::Tasks;

use glib::clone;
use gtk::prelude::*;
use rand::Rng;
use std::cell::RefCell;
use std::rc::Rc;

type TasksRc = Rc<RefCell<Tasks>>;

const TASKS_FILE_NAME: &str = "tasks.json";

pub fn build_ui(application: &gtk::Application) {
    let tasks: TasksRc = Rc::new(RefCell::new(Tasks::new()));
    if let Err(e) = tasks.borrow_mut().load_from_file_json(TASKS_FILE_NAME) {
        panic!("Application error: {}", e)
    }

    let todo_tasks = create_labeled_box_fox_tasks("To do");
    let in_progress_tasks = create_labeled_box_fox_tasks("In progress");
    let done_tasks = create_labeled_box_fox_tasks("Done");

    let tasks_main_container = gtk::Box::new(gtk::Orientation::Horizontal, 0);

    update_visible_tasks(&todo_tasks, &tasks);

    tasks_main_container.pack_start(&todo_tasks, true, true, 0);
    tasks_main_container.pack_start(&in_progress_tasks, true, true, 0);
    tasks_main_container.pack_start(&done_tasks, true, true, 0);

    let main_container = gtk::Box::new(gtk::Orientation::Vertical, 0);
    let new_task_button = create_new_task_button(tasks.clone(), &todo_tasks);
    main_container.pack_start(&tasks_main_container, true, true, 0);
    main_container.pack_start(&new_task_button, true, true, 0);

    let window = gtk::ApplicationWindow::new(application);

    let tasks_clone = tasks.clone();
    window.connect_delete_event(move |_, _| {
        println!("Closing application window");
        tasks_clone
            .borrow()
            .save_to_file_json(TASKS_FILE_NAME)
            .unwrap();

        return gtk::Inhibit(false);
    });

    window.set_title("To do list");
    window.set_default_size(500, 500);
    window.add(&main_container);
    window.show_all();
}

fn update_visible_tasks(container: &gtk::Box, tasks: &TasksRc) {
    let tasks_container = container
        .get_children()
        .get(1)
        .unwrap()
        .clone()
        .downcast::<gtk::Box>()
        .unwrap();
    for elements in tasks_container.get_children() {
        tasks_container.remove(&elements);
    }

    for (_, task) in &tasks.borrow().tasks {
        println!("{:?}", task);
        let todo_label = gtk::Label::new(Some(&task.title[..]));
        todo_label.show();

        tasks_container.pack_start(&todo_label, false, true, 0);
    }
    println!("{}", container.get_children().len());
}

fn create_labeled_box_fox_tasks(label_text: &str) -> gtk::Box {
    let label = gtk::Label::new(Some(label_text));
    let tasks_container = gtk::Box::new(gtk::Orientation::Vertical, 0);
    let labeled_container = gtk::Box::new(gtk::Orientation::Vertical, 0);
    labeled_container.pack_start(&label, false, true, 0);
    labeled_container.pack_start(&tasks_container, false, true, 0);

    return labeled_container;
}

fn create_new_task_button(tasks: TasksRc, container: &gtk::Box) -> gtk::Button {
    let button = gtk::Button::with_label("New task");

    let create_labeled_text_field = |label_name: &str| {
        let label = gtk::Label::new(Some(label_name));
        let text_field = gtk::Entry::new();

        let layout = gtk::Box::new(gtk::Orientation::Vertical, 0);
        layout.pack_start(&label, false, true, 0);
        layout.pack_start(&text_field, true, true, 0);

        return layout;
    };

    button.connect_clicked(clone!(@weak container => move |_| {
        println!("Create new task button clicked");
        let new_task_window = gtk::Window::new(gtk::WindowType::Toplevel);

        let layout = gtk::Box::new(gtk::Orientation::Vertical, 0);
        let title_label = create_labeled_text_field("Title:");
        let description_label = create_labeled_text_field("Description");
        layout.pack_start(&title_label, true, true, 0);
        layout.pack_start(&description_label, true, true, 0);

        let create_task_button = gtk::Button::with_label("Create task");

        create_task_button.connect_clicked(clone!(@weak new_task_window, @weak tasks, @weak container => move |_| {
            println!("Create new task accepted");

            let title = title_label.get_children().get(1).unwrap().clone().downcast::<gtk::Entry>().unwrap();
            let description = description_label.get_children().get(1).unwrap().clone().downcast::<gtk::Entry>().unwrap();
            let task = Task::new(rand::thread_rng().gen(), title.get_text().as_str(), description.get_text().as_str());

            tasks.borrow_mut().add_task(task);

            update_visible_tasks(&container, &tasks);
            new_task_window.close();
        }));
        layout.pack_start(&create_task_button, true, true, 0);

        new_task_window.set_title("New task");
        new_task_window.set_default_size(400, 200);
        new_task_window.add(&layout);
        new_task_window.show_all();
    }));

    return button;
}
