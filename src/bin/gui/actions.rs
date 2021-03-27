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
use phys_plotter::save_format::PhysPlotterFile;
use serde_json::from_reader;
use std::cell::RefCell;
use std::convert::TryInto;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::rc::Rc;

fn about_action(application: &gtk::Application, window: &gtk::ApplicationWindow) {
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
fn change_backend(
    application: &gtk::Application,
    window: &gtk::ApplicationWindow,
    _state: &Rc<RefCell<UIState>>,
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
        $var.get_text().parse()?
    };
}

fn do_generate_plot(state: &UIState) -> Result<(), Box<dyn std::error::Error>> {
    let range = state.dataset.get_bounds();
    let dataset_text = state
        .dataset
        .get_text(&range.0, &range.1, true)
        .unwrap_or_else(|| glib::GString::from(""));
    let dataset = TwoVarDataSet::from_string(
        dataset_text.as_str(),
        parse_state_float_or_return!(state.default_x_uncertainty),
        parse_state_float_or_return!(state.default_y_uncertainty),
    )?;
    plot_gnuplot(
        state.title.get_text().as_str(),
        state.x_label.get_text().as_str(),
        state.y_label.get_text().as_str(),
        dataset,
    )?;
    Ok(())
}

/// Generate plot image or preview
fn generate_plot(application: &gtk::Application, state: &Rc<RefCell<UIState>>) {
    let dialog = gio::SimpleAction::new("plot", None);
    dialog.connect_activate(clone!(@strong state => move |_, _| {
        do_generate_plot(&state.borrow()).unwrap();
    }));
    application.add_action(&dialog);
}

/// Open file
fn open_file(
    application: &gtk::Application,
    window: &gtk::ApplicationWindow,
    state: &Rc<RefCell<UIState>>,
) {
    let open_file = gio::SimpleAction::new("open", None);
    open_file.connect_activate(clone!(@weak window, @strong state => move |_, _| {
        let file_chooser = gtk::FileChooserDialog::new(
            Some("Open File"),
            Some(&window),
            gtk::FileChooserAction::Open,
        );
        file_chooser.add_buttons(&[
            ("Open", gtk::ResponseType::Ok),
            ("Cancel", gtk::ResponseType::Cancel),
        ]);
        file_chooser.connect_response(clone!(@strong state => move |file_chooser, response| {
            /* TODO: Resolve unwraps and expects here */
            if response == gtk::ResponseType::Ok {
                let filename = file_chooser.get_filename().expect("Couldn't get filename");
                let mut file = File::open(&filename).expect("Couldn't open file");
                let reader = BufReader::new(&file);
                if let Ok(val) = from_reader::<_, PhysPlotterFile>(reader) {
                    state.replace(val.try_into().unwrap());
                }
                let mut contents = String::new();
                let _ = file.read_to_string(&mut contents);
            }
            file_chooser.close();
        }));

        file_chooser.show_all();
    }));
    application.add_action(&open_file);
}

pub fn register_actions(
    application: &gtk::Application,
    window: &gtk::ApplicationWindow,
    state: Rc<RefCell<UIState>>,
) {
    about_action(application, window);
    change_backend(application, window, &state);
    generate_plot(application, &state);
    open_file(application, window, &state);
}
