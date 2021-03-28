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

use crate::state::{Backends, UIState};
use crate::ui::{create_error_popup, disp_save_dialog};
use crate::{unwrap_option_or_error_return, unwrap_result_or_error_return};
use clap::crate_version;
use gio::prelude::*;
use glib::clone;
use gtk::prelude::*;
use gtk::License::Gpl30;
use gtk::{
    AboutDialogBuilder, Button, ButtonsType, DialogBuilder, DrawingArea, MessageDialogBuilder,
    Orientation, RadioButton, ResponseType,
};
use phys_plotter::data::TwoVarDataSet;
use phys_plotter::plot;
use phys_plotter::save_format::PhysPlotterFile;
use plotters::prelude::*;
use plotters_cairo::CairoBackend;
use std::cell::RefCell;
use std::convert::TryInto;
use std::fs::File;
use std::io::prelude::*;
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
        let dialog = DialogBuilder::new()
            .title("Plotting Backend")
            .attached_to(&window)
            .transient_for(&window)
            .build();
        let radiobutton_1 = RadioButton::new();
        radiobutton_1.set_label("plotters");
        let radiobutton_2 = RadioButton::new();
        radiobutton_2.set_label("gnuplot");
        match state.borrow().backend {
            Backends::Plotters => {radiobutton_2.join_group(Some(&radiobutton_1));},
            Backends::Gnuplot => {
            radiobutton_1.join_group(Some(&radiobutton_2));
            }
        }
        dialog.add_action_widget(&radiobutton_1, ResponseType::Other(1));
        dialog.add_action_widget(&radiobutton_2, ResponseType::Other(2));
        dialog.connect_response(clone!(@strong state => move |_,resp_type| {
            match resp_type {
                ResponseType::Other(1) => {
                    state.borrow_mut().backend = Backends::Plotters
                },
                ResponseType::Other(2) => {
                    state.borrow_mut().backend =Backends::Gnuplot
                },
                _ => ()
            }
        }));
        //radiobuttons.show_all();
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

/// Image formats that can be exported
#[derive(Debug, Copy, Clone)]
enum ImageFormat {
    SVG,
    PNG,
}

fn save_image(
    window: &gtk::ApplicationWindow,
    format: ImageFormat,
    title: &str,
    x_label: &str,
    y_label: &str,
    dataset: &TwoVarDataSet,
) {
    // These variables must be owned in order for clone to succeed
    let title = title.to_string();
    let x_label = x_label.to_string();
    let y_label = y_label.to_string();
    disp_save_dialog(
        &window,
        "Save Image to",
        clone!(@weak window, @strong title, @strong x_label, @strong y_label, @strong dataset => move |filename| {
            match format {
                ImageFormat::SVG => unwrap_result_or_error_return!(
                    plot::plot_plotters(
                        &title,
                        &x_label,
                        &y_label,
                        &dataset,
                        SVGBackend::new(filename, (960, 540))
                    ),
                    &window,
                    "Failed to open plot",
                    {}
                ),
                ImageFormat::PNG => unwrap_result_or_error_return!(
                    plot::plot_plotters(
                        &title,
                        &x_label,
                        &y_label,
                        &dataset,
                        BitMapBackend::new(filename, (960, 540))
                    ),
                    &window,
                    "Failed to open plot",
                    {}
                ),
            };
        }),
    );
}

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
    match state_local.backend {
        Backends::Gnuplot => plot::plot_gnuplot(&title, &x_label, &y_label, &dataset, None)?,
        Backends::Plotters => {
            // Create a new window for drawing
            let plot_window = gtk::Window::new(gtk::WindowType::Toplevel);
            application.add_window(&plot_window);
            plot_window.set_title("Plotters Canva");
            plot_window.set_default_size(960, 584);

            let container = gtk::Box::new(Orientation::Vertical, 5);
            // Create cairo drawing area
            let drawing_area = DrawingArea::new();
            // The drawing area has to expand or button will take all the space
            drawing_area.set_vexpand(true);
            container.add(&drawing_area);
            drawing_area.connect_draw(clone!(@weak window, @weak plot_window, @strong title, @strong x_label, @strong y_label, @strong dataset => @default-return Inhibit(false), move |_, ctx| {
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
            // Save options
            let button_area = gtk::Box::new(Orientation::Horizontal, 5);
            let button_png = Button::with_label("Save to PNG");
            button_png.connect_clicked(clone!(@weak window, @strong title, @strong x_label, @strong y_label, @strong dataset => move |_| {
                save_image(&window, ImageFormat::PNG, &title, &x_label, &y_label, &dataset);
            }));
            let button_svg = Button::with_label("Save to SVG");
            button_svg.connect_clicked(clone!(@weak window, @strong title, @strong x_label, @strong y_label, @strong dataset => move |_| {
                save_image(&window, ImageFormat::SVG, &title, &x_label, &y_label, &dataset);
            }));
            let button_close = Button::with_label("Close");
            button_close.connect_clicked(clone!(@weak plot_window => move |_| {
                plot_window.close();
            }));
            button_area.add(&button_png);
            button_area.add(&button_svg);
            button_area.add(&button_close);
            container.add(&button_area);
            plot_window.add(&container);
            plot_window.show_all();
        }
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

/// Immediately save two files without any check
fn save_imm(window: &gtk::ApplicationWindow, state: &Rc<RefCell<UIState>>) {
    unwrap_result_or_error_return!(state.borrow().save(), &window, "Cannot save file", {});
}

/// Wrapper to save dataset and project. If check is false, a new path of
/// the file is always asked from the used. Otherwise, if there is
/// recorded path, that one is used. check==false is useful for "Save As",
/// while true is useful for "Save". If the target exists, the user is prompted
/// whether to overwrite.
fn maybe_check_main_file_save_all(
    window: &gtk::ApplicationWindow,
    state: &Rc<RefCell<UIState>>,
    check: bool,
) {
    if !check || state.borrow().file_path == String::default() {
        disp_save_dialog(
            window,
            "Save Project File",
            clone!(@weak window, @strong state => move |filename| {
                state.borrow_mut().file_path = filename.display().to_string();
                save_imm(&window, &state);
            }),
        );
    } else {
        save_imm(window, state);
    }
}

/// Save file, saves both the dataset and the project, possibly altering the state
fn save(
    application: &gtk::Application,
    window: &gtk::ApplicationWindow,
    state: &Rc<RefCell<UIState>>,
) {
    let save = gio::SimpleAction::new("save", None);
    save.connect_activate(clone!(@weak window, @strong state => move |_, _| {
        maybe_check_main_file_save_all(&window, &state, true);
    }));
    application.add_action(&save);
}

/// Save file as, saves both the dataset and the project, possibly altering the state
fn save_as(
    application: &gtk::Application,
    window: &gtk::ApplicationWindow,
    state: &Rc<RefCell<UIState>>,
) {
    let save_as = gio::SimpleAction::new("save_as", None);
    save_as.connect_activate(clone!(@weak window, @strong state => move |_, _| {
        maybe_check_main_file_save_all(&window, &state, false);
    }));
    application.add_action(&save_as);
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
                // First try to parse it as saved file
                if let Ok(val) = PhysPlotterFile::from_file(&filename) {
                    let new_state: UIState = unwrap_result_or_error_return!(
                        val.try_into(),
                        &window,
                        "Couldn't parse file",
                        {file_chooser.close()}
                    );
                    state.borrow_mut().replace(new_state);
                    state.borrow_mut().file_path = filename.display().to_string();
                } else {
                    // Else treat as plain dataset text
                    let mut file = unwrap_result_or_error_return!(
                        File::open(&filename),
                        &window,
                        "Couldn't open file",
                        {file_chooser.close()}
                    );
                    let mut contents = String::new();
                    let _ = file.read_to_string(&mut contents);
                    state.borrow_mut().dataset.set_text(&contents);
                }
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
            let dialog = MessageDialogBuilder::new()
                .transient_for(&window)
                .title("Confirmation")
                .text("File modified but not saved, proceed?")
                .buttons(ButtonsType::YesNo)
                .build();
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
    state: &Rc<RefCell<UIState>>,
) {
    let quit = gio::SimpleAction::new("quit", None);
    quit.connect_activate(clone!(@weak application => move |_,_| {
        application.quit();
    }));
    application.add_action(&quit);
    about_action(application, window);
    change_backend(application, window, &state);
    generate_plot(application, window, &state);
    save(application, window, &state);
    save_as(application, window, &state);
    open_file(application, window, &state);
    new_plot(application, window, &state);
    application.set_accels_for_action("app.quit", &["<Primary>Q"]);
    application.set_accels_for_action("app.change_backend", &["<Primary>B"]);
    application.set_accels_for_action("app.plot", &["<Primary>G"]);
    application.set_accels_for_action("app.save", &["<Primary>S"]);
    application.set_accels_for_action("app.save_as", &["<Primary><Shift>S"]);
    application.set_accels_for_action("app.open", &["<Primary>O"]);
    application.set_accels_for_action("app.new", &["<Primary>N"]);
}
