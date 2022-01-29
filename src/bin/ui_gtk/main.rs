mod actions;
mod menu;
mod state;
mod ui;

use gio::prelude::*;
use phys_plotter::default_values::APP_ID;
use std::env;

fn main() {
    let uiapp = gtk::Application::new(Some(APP_ID), gio::ApplicationFlags::FLAGS_NONE)
        .expect("Application::new failed");
    uiapp.connect_activate(ui::app);
    uiapp.run(&env::args().collect::<Vec<_>>());
}
