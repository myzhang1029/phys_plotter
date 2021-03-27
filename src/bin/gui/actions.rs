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
use crate::ui::create_error_popup;
use crate::{unwrap_option_or_error_return, unwrap_result_or_error_return};
use clap::crate_version;
use gio::prelude::*;
use glib::clone;
use gtk::prelude::*;
use gtk::License::Gpl30;
use gtk::{AboutDialogBuilder, Dialog, DialogFlags, DrawingArea, ResponseType};
use phys_plotter::data::TwoVarDataSet;
use phys_plotter::plot;
use phys_plotter::save_format::PhysPlotterFile;
use plotters_cairo::CairoBackend;
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

/// Change the backend, the state is possibly altered
fn change_backend(
    application: &gtk::Application,
    window: &gtk::ApplicationWindow,
    state: &Rc<RefCell<UIState>>,
) {
    // Action to change the selected backend
    let dialog = gio::SimpleAction::new("change_backend", None);
    dialog.connect_activate(clone!(@weak window, @strong state => move |_, _| {
        let dialog = Dialog::with_buttons(
            Some("Plotting Backend"),
            Some(&window),
            DialogFlags::empty(),
            &[
                ("Apply", ResponseType::Apply),
                ("Cancel", ResponseType::Cancel)
            ]
        );
        dialog.connect_response(clone!(@strong state => move |_,resp_type| {
            if resp_type == ResponseType::Apply {
                //abc;
            }
        }));
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

/// Error type while plotting
enum PlotError {
    EmptyData,
}

macro_rules! impl_fmt_plot_error {
    ($item: ident) => {
        impl std::fmt::$item for PlotError {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    PlotError::EmptyData => write!(f, "Dataset cannot be empty"),
                }
            }
        }
    };
}

impl_fmt_plot_error!(Debug);
impl_fmt_plot_error!(Display);

impl std::error::Error for PlotError {}

fn do_generate_plot(
    application: &gtk::Application,
    window: &gtk::ApplicationWindow,
    state: &Rc<RefCell<UIState>>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Get the range of the dataset text
    let state_local = state.borrow();
    let range = state_local.dataset.get_bounds();
    let dataset_text = state_local
        .dataset
        .get_text(&range.0, &range.1, true)
        .unwrap_or_else(|| glib::GString::from(""));
    // Construct dataset from the input
    let dataset = TwoVarDataSet::from_string(
        dataset_text.as_str(),
        parse_state_float_or_return!(state_local.default_x_uncertainty),
        parse_state_float_or_return!(state_local.default_y_uncertainty),
    )?;
    // Empty values can crash some backends
    if dataset.is_empty() {
        return Err(Box::new(PlotError::EmptyData));
    }
    // Extract information here first
    let title = state_local.title.get_text();
    let x_label = state_local.x_label.get_text();
    let y_label = state_local.y_label.get_text();
    // Call plotting backend
    match state_local.backend_name.get_text().as_str() {
        "gnuplot" => plot::plot_gnuplot(&title, &x_label, &y_label, &dataset, None)?,
        "plotters" => {
            // Create a new window for drawing
            let plot_window = gtk::Window::new(gtk::WindowType::Toplevel);
            application.add_window(&plot_window);
            plot_window.set_title("Plotters Canva");
            plot_window.set_default_size(960, 540);

            // Create cairo drawing area
            let drawing_area = Box::new(DrawingArea::new)();
            drawing_area.connect_draw(clone!(@weak window, @weak plot_window => @default-return Inhibit(false), move |_, ctx| {
                let backend = CairoBackend::new(ctx, (960, 540)).unwrap();
                unwrap_result_or_error_return!(
                    plot::plot_plotters(
                        &title,
                        &x_label,
                        &y_label,
                        &dataset,
                        backend
                    ),
                    &window,
                    "Failed to open plot",
                    {
                        plot_window.close();
                        Inhibit(false)
                    }
                );
                Inhibit(false)
            }));
            plot_window.add(&drawing_area);
            plot_window.show_all();
        }
        _ => (),
    };
    Ok(())
}

/// Generate plot image or preview, reading the state
fn generate_plot(
    application: &gtk::Application,
    window: &gtk::ApplicationWindow,
    state: &Rc<RefCell<UIState>>,
) {
    let dialog = gio::SimpleAction::new("plot", None);
    dialog.connect_activate(
        clone!(@weak application, @weak window, @strong state => move |_, _| {
            unwrap_result_or_error_return!(
                do_generate_plot(&application, &window, &state),
                &window,
                "Failed to open plot",
                {}
            );
        }),
    );
    application.add_action(&dialog);
}

/// Open file, possibly altering the state
fn open_file(
    application: &gtk::Application,
    window: &gtk::ApplicationWindow,
    state: &Rc<RefCell<UIState>>,
) {
    let open_file = gio::SimpleAction::new("open", None);
    open_file.connect_activate(clone!(@weak window, @strong state => move |_, _| {
        // Use file chooser to choose file
        let file_chooser = gtk::FileChooserDialog::new(
            Some("Open File"),
            Some(&window),
            gtk::FileChooserAction::Open,
        );
        file_chooser.add_buttons(&[
            ("Open", gtk::ResponseType::Ok),
            ("Cancel", gtk::ResponseType::Cancel),
        ]);
        file_chooser.connect_response(clone!(@weak window, @strong state => move |file_chooser, response| {
            if response == gtk::ResponseType::Ok {
                let filename = unwrap_option_or_error_return!(
                    file_chooser.get_filename(),
                    &window,
                    "Couldn't get filename",
                    {file_chooser.close()}
                );
                let file = unwrap_result_or_error_return!(
                    File::open(&filename),
                    &window,
                    "Couldn't open file",
                    {file_chooser.close()}
                );
                // First try to parse it as saved file
                let reader = BufReader::new(file);
                if let Ok(val) = from_reader::<_, PhysPlotterFile>(reader) {
                    let new_state: UIState = unwrap_result_or_error_return!(
                        val.try_into(),
                        &window,
                        "Couldn't parse file",
                        {file_chooser.close()}
                    );
                        state.borrow_mut().replace(new_state);
                }
                // Else treat as plain dataset text
                let mut file = unwrap_result_or_error_return!(
                    File::open(&filename),
                    &window,
                    "Couldn't open file",
                    {file_chooser.close()}
                );
                let mut contents = String::new();
                let _ = file.read_to_string(&mut contents);
                let mut state = state.borrow_mut();
                state.dataset_file = filename.display().to_string();
                state.dataset.set_text(&contents);
            }
            file_chooser.close();
        }));

        file_chooser.show_all();
    }));
    application.add_action(&open_file);
}

/// Create a new plot, possibly altering state
fn new_plot(
    application: &gtk::Application,
    window: &gtk::ApplicationWindow,
    state: &Rc<RefCell<UIState>>,
) {
    let new_file = gio::SimpleAction::new("new", None);
    new_file.connect_activate(clone!(@weak window, @strong state => move |_, _| {
        if state.borrow().saved {
            let new_state = UIState::new();
            state.borrow_mut().replace(new_state);
        } else {
            // Not saved, ask if save
            let dialog = Dialog::with_buttons(
                Some("Are you sure?"),
                Some(&window),
                DialogFlags::empty(),
                &[
                    ("Discard", ResponseType::Yes),
                    ("Go back", ResponseType::No),
                ]
            );
            dialog.connect_response(clone!(@strong state => move |_,resp_type| {
                if resp_type == ResponseType::Yes {
                        let new_state = UIState::new();
                        state.borrow_mut().replace(new_state);
                }
            }));
            dialog.show_all();
            dialog.run();
            unsafe { dialog.destroy(); }
        }
    }));
    application.add_action(&new_file);
}

/// Register application actions
pub fn register_actions(
    application: &gtk::Application,
    window: &gtk::ApplicationWindow,
    state: Rc<RefCell<UIState>>,
) {
    about_action(application, window);
    change_backend(application, window, &state);
    generate_plot(application, window, &state);
    open_file(application, window, &state);
    new_plot(application, window, &state);
}
