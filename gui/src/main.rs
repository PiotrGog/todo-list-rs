use gio::prelude::*;

use gui::build_ui;
use std::env::args;

fn main() {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.drag_and_drop"),
        Default::default(),
    )
    .expect("Initialization failed...");

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());
}
