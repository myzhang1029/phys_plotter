//
//  Copyright (C) 2021 Zhang Maiyun <myzhang1029@hotmail.com>
//
//  This file is part of physics plotter.
//
//  Physics plotter is free software: you can redistribute it and/or modify
//  it under the terms of the GNU General Public License as published by
//  the Free Software Foundation, either version 3 of the License, or
//  (at your option) any later version.
//
//  Physics plotter is distributed in the hope that it will be useful,
//  but WITHOUT ANY WARRANTY; without even the implied warranty of
//  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//  GNU General Public License for more details.
//
//  You should have received a copy of the GNU General Public License
//  along with physics plotter.  If not, see <https://www.gnu.org/licenses/>.
//

use crate::state::UIState;
use clap::crate_version;
use gio::prelude::*;
use glib::clone;
use gtk::prelude::*;
use gtk::License::Gpl30;
use gtk::{AboutDialogBuilder, Dialog, DialogFlags, ResponseType};
use phys_plotter::data::TwoVarDataSet;
use phys_plotter::plot::plot_gnuplot;

pub fn about_action(application: &gtk::Application, window: &gtk::ApplicationWindow) {
    // About action to show an about dialog
    let about = gio::SimpleAction::new("about", None);
    about.connect_activate(clone!(@weak window => move |_, _| {
        let dialog = AboutDialogBuilder::new()
            .program_name("Physics Plotter GTK Interface")
            .version(crate_version!())
            .title("About")
            .website_label("Project page on GitHub")
            .website("https://github.com/myzhang1029/phys_plotter")
            .authors(vec![String::from("Zhang Maiyun")])
            .copyright("Copyright (C) 2021 Zhang Maiyun.")
            .license_type(Gpl30)
            .transient_for(&window)
            .build();
        dialog.show_all();
        dialog.run();
        unsafe { dialog.destroy(); }
    }));
    application.add_action(&about);
}

/// TODO
pub fn change_backend(
    application: &gtk::Application,
    window: &gtk::ApplicationWindow,
    _state: &UIState,
) {
    // Action to change the selected backend
    let dialog = gio::SimpleAction::new("change_backend", None);
    dialog.connect_activate(clone!(@weak window => move |_, _| {
        let dialog = Dialog::with_buttons(
            Some("Plotting Backend"),
            Some(&window),
            DialogFlags::empty(),
            &[
                ("Apply", ResponseType::Apply),
                ("Cancel", ResponseType::Cancel)
            ]
        );
        dialog.show_all();
        dialog.run();
        unsafe { dialog.destroy(); }
    }));
    application.add_action(&dialog);
}

macro_rules! parse_state_float_or_return {
    ($var: expr) => {
        $var.borrow().get_text().parse()?
    };
}
fn do_generate_plot(state: &UIState) -> Result<(), Box<dyn std::error::Error>> {
    let borrowed_dataset = state.dataset.borrow();
    let range = borrowed_dataset.get_bounds();
    let dataset_text = borrowed_dataset
        .get_text(&range.0, &range.1, true)
        .unwrap_or_else(|| glib::GString::from(""));
    let dataset = TwoVarDataSet::from_string(
        dataset_text.as_str(),
        parse_state_float_or_return!(state.default_x_uncertainty),
        parse_state_float_or_return!(state.default_y_uncertainty),
    )?;
    plot_gnuplot(
        state.title.borrow().get_text().as_str(),
        state.x_label.borrow().get_text().as_str(),
        state.y_label.borrow().get_text().as_str(),
        dataset,
    )?;
    Ok(())
}

/// Generate plot image or preview
pub fn generate_plot(application: &gtk::Application, state: UIState) {
    let dialog = gio::SimpleAction::new("plot", None);
    dialog.connect_activate(move |_, _| {
        do_generate_plot(&state).unwrap();
    });
    application.add_action(&dialog);
}

pub fn register_actions(
    application: &gtk::Application,
    window: &gtk::ApplicationWindow,
    state: UIState,
) {
    about_action(application, window);
    change_backend(application, window, &state);
    generate_plot(application, state);
}
