use super::data::TwoVarDataSet;
use super::default_values as defv;
use super::plot;
use super::plot::Backends;
use super::save_format::PhysPlotterFile;
use clap::crate_version;
use eframe::{
    egui::{
        self,
        plot::{Legend, Line, LineStyle, MarkerShape, Plot, Points, Value, Values},
        Color32,
    },
    epi,
};
use std::convert::TryInto;
use std::fs::File;
use std::io::Read;
use std::ops::Deref;
use std::str::FromStr;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct App {
    saved: bool,
    file_path: String,
    #[cfg_attr(feature = "persistence", serde(skip))]
    show_about: bool,
    #[cfg_attr(feature = "persistence", serde(skip))]
    show_confirm_then_new: bool,
    #[cfg_attr(feature = "persistence", serde(skip))]
    show_confirm_then_open: bool,
    #[cfg_attr(feature = "persistence", serde(skip))]
    error: Option<String>,

    backend: Backends,
    title: String,
    dataset: String,
    x_label: String,
    y_label: String,
    default_x_uncertainty: String,
    default_y_uncertainty: String,
}

impl Default for App {
    fn default() -> Self {
        Self {
            saved: true,
            show_about: false,
            show_confirm_then_new: false,
            show_confirm_then_open: false,
            file_path: String::default(),
            error: None,

            backend: Backends::from_str(defv::BACKEND).unwrap(),
            title: String::from(defv::TITLE),
            dataset: String::default(),
            x_label: String::from(defv::X_LABEL),
            y_label: String::from(defv::Y_LABEL),
            default_x_uncertainty: String::from(defv::X_UNCERTAINTY),
            default_y_uncertainty: String::from(defv::Y_UNCERTAINTY),
        }
    }
}

impl epi::App for App {
    fn name(&self) -> &str {
        "Physics Plotter"
    }

    fn setup(
        &mut self,
        _ctx: &egui::CtxRef,
        _frame: &epi::Frame,
        storage: Option<&dyn epi::Storage>,
    ) {
        if let Some(storage) = storage {
            *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default();
        }
    }

    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::CtxRef, frame: &epi::Frame) {
        self.draw_top_menu(ctx, frame);
        self.draw_side_panel(ctx);
        self.draw_preview_area(ctx);
        self.draw_about_window(ctx);
        self.draw_confirm_window(ctx);
        self.draw_error_window(ctx);
        self.ui_file_drag_and_drop(ctx);
    }
}

impl App {
    #[cfg(feature = "ui_egui_with_file")]
    fn draw_top_menu(&mut self, ctx: &egui::CtxRef, frame: &epi::Frame) {
        egui::TopBottomPanel::top("open_menu").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                if ui.button("New").clicked() {
                    if self.saved {
                        self.reset();
                    } else {
                        self.show_confirm_then_new = true;
                    }
                }
                if ui.button("Open").clicked() {
                    if self.saved {
                        self.open();
                    } else {
                        self.show_confirm_then_open = true;
                    }
                }
                if ui.button("Save").clicked() {
                    self.save();
                }
                if ui.button("Save As").clicked() {
                    // This dialog does overwrite confirmation for us
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter("Physics Plotter File", &["pyp"])
                        .save_file()
                    {
                        self.file_path = path.display().to_string();
                    }
                    self.save();
                }
                if ui.button("Plot").clicked() {
                    self.plot();
                }
                if ui.button("About").clicked() {
                    self.show_about = true;
                }
                if ui.button("Quit").clicked() {
                    // State is preserved
                    frame.quit();
                }
            });
        });
    }

    #[cfg(not(feature = "ui_egui_with_file"))]
    fn draw_top_menu(&mut self, ctx: &egui::CtxRef, frame: &epi::Frame) {
        egui::TopBottomPanel::top("open_menu").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                if ui.button("New").clicked() {
                    if self.saved {
                        self.reset();
                    } else {
                        self.show_confirm_then_new = true;
                    }
                }
                if ui.button("Plot").clicked() {
                    self.plot();
                }
                if ui.button("About").clicked() {
                    self.show_about = true;
                }
                if ui.button("Quit").clicked() {
                    // State is preserved
                    frame.quit();
                }
            });
        });
    }

    fn draw_side_panel(&mut self, ctx: &egui::CtxRef) {
        let Self {
            saved,
            title,
            x_label,
            y_label,
            dataset,
            default_x_uncertainty,
            default_y_uncertainty,
            backend,
            ..
        } = self;

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Properties");

            ui.horizontal(|ui| {
                ui.label("Plot title");
                if ui.text_edit_singleline(title).changed() {
                    *saved = false;
                }
            });
            ui.horizontal(|ui| {
                ui.label("X axis label");
                if ui.text_edit_singleline(x_label).changed() {
                    *saved = false;
                }
            });
            ui.horizontal(|ui| {
                ui.label("Y axis label");
                if ui.text_edit_singleline(y_label).changed() {
                    *saved = false;
                }
            });
            ui.horizontal(|ui| {
                ui.label("Default x uncertainty");
                if ui.text_edit_singleline(default_x_uncertainty).changed() {
                    *saved = false;
                }
            });
            ui.horizontal(|ui| {
                ui.label("Default y uncertainty");
                if ui.text_edit_singleline(default_y_uncertainty).changed() {
                    *saved = false;
                }
            });
            ui.horizontal(|ui| {
                ui.label("Plotting backend");
                ui.selectable_value(backend, Backends::Gnuplot, "GNU Plot");
                ui.selectable_value(backend, Backends::Plotters, "Plotters");
            });

            ui.add_space(20.0);

            ui.heading("Dataset");
            egui::ScrollArea::vertical()
                .max_height(300.0)
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    let dsbox = egui::TextEdit::multiline(dataset)
                        .desired_width(f32::INFINITY)
                        .desired_rows(10)
                        .cursor_at_end(true);
                    ui.add(dsbox);
                });
        });
    }

    fn draw_preview_area(&mut self, ctx: &egui::CtxRef) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Preview");
                ui.collapsing("Instructions", |ui| {
                    ui.label("Pan by dragging, or scroll (+ shift = horizontal).");
                    if cfg!(target_arch = "wasm32") {
                        ui.label("Zoom with ctrl / ⌘ + pointer wheel, or with pinch gesture.");
                    } else if cfg!(target_os = "macos") {
                        ui.label("Zoom with ctrl / ⌘ + scroll.");
                    } else {
                        ui.label("Zoom with ctrl + scroll.");
                    }
                    ui.label("Reset view with double-click.");
                });
            });
            let plot = Plot::new("preview").legend(Legend::default());
            if let Ok(dataset) = self.parse_dataset() {
                // Extra length before min and after max
                let extra = (dataset.max_x(false) - dataset.min_x(false)) * 0.1;
                // Two points for plotting the lines
                let ln_plt_x = [dataset.min_x(true) - extra, dataset.max_x(true) + extra];
                plot.show(ui, |plot_ui| {
                    let best_fit = dataset.line_best_fit();
                    plot_ui.line(
                        Line::new(Values::from_values(vec![
                            Value::new(ln_plt_x[0], best_fit.y(ln_plt_x[0])),
                            Value::new(ln_plt_x[1], best_fit.y(ln_plt_x[1])),
                        ]))
                        .name("Best Fit"),
                    );
                    if let Some(line_min_grad) = dataset.line_min_grad() {
                        plot_ui.line(
                            Line::new(Values::from_values(vec![
                                Value::new(ln_plt_x[0], line_min_grad.y(ln_plt_x[0])),
                                Value::new(ln_plt_x[1], line_min_grad.y(ln_plt_x[1])),
                            ]))
                            .name("Minimum Gradient")
                            .style(LineStyle::Dashed { length: 5.0 }),
                        );
                    }
                    if let Some(line_max_grad) = dataset.line_max_grad() {
                        plot_ui.line(
                            Line::new(Values::from_values(vec![
                                Value::new(ln_plt_x[0], line_max_grad.y(ln_plt_x[0])),
                                Value::new(ln_plt_x[1], line_max_grad.y(ln_plt_x[1])),
                            ]))
                            .name("Maximum Gradient")
                            .style(LineStyle::Dashed { length: 5.0 }),
                        );
                    }
                    for point in dataset.deref() {
                        let main = Points::new(Values::from_values(vec![Value::new(
                            point.x_value,
                            point.y_value,
                        )]))
                        .shape(MarkerShape::Cross)
                        .color(Color32::DARK_GREEN)
                        .radius(3.0 * extra as f32)
                        .filled(false);
                        plot_ui.points(main);
                        // Tips of error bars
                        let around = Points::new(Values::from_values(vec![
                            Value::new(point.x_value + point.x_uncertainty, point.y_value),
                            Value::new(point.x_value - point.x_uncertainty, point.y_value),
                            Value::new(point.x_value, point.y_value + point.y_uncertainty),
                            Value::new(point.x_value, point.y_value - point.y_uncertainty),
                        ]))
                        .shape(MarkerShape::Plus)
                        .color(Color32::DARK_GREEN)
                        .radius(3.0 * extra as f32)
                        .filled(false);
                        plot_ui.points(around);
                        // Error bar
                        plot_ui.line(
                            Line::new(Values::from_values(vec![
                                Value::new(point.x_value + point.x_uncertainty, point.y_value),
                                Value::new(point.x_value - point.x_uncertainty, point.y_value),
                            ]))
                            .color(Color32::DARK_GREEN),
                        );
                        plot_ui.line(
                            Line::new(Values::from_values(vec![
                                Value::new(point.x_value, point.y_value + point.y_uncertainty),
                                Value::new(point.x_value, point.y_value - point.y_uncertainty),
                            ]))
                            .color(Color32::DARK_GREEN),
                        );
                    }
                });
            }
        });
    }

    fn draw_confirm_window(&mut self, ctx: &egui::CtxRef) {
        macro_rules! create_save_confirm_window {
            ($action: block, $var: expr) => {
                if $var {
                    egui::Window::new("Confirm").show(ctx, |ui| {
                        ui.label("The file has not been saved. Continue?");
                        ui.horizontal(|ui| {
                            if ui.add(egui::Button::new("Yes")).clicked() {
                                $action
                                $var = false;
                            }
                            if ui.add(egui::Button::new("No")).clicked() {
                                $var = false;
                            }
                        });
                    });

                }
            };
        }
        create_save_confirm_window! {{self.reset();}, self.show_confirm_then_new}
        create_save_confirm_window! {{self.open();}, self.show_confirm_then_open}
    }

    fn draw_error_window(&mut self, ctx: &egui::CtxRef) {
        let Self { error, .. } = self;
        if let Some(error_desc) = error {
            let error_desc = error_desc.clone();
            egui::Window::new("Error").show(ctx, |ui| {
                ui.label(error_desc);
                if ui.add(egui::Button::new("Close")).clicked() {
                    self.error = None;
                }
            });
        }
    }

    fn draw_about_window(&mut self, ctx: &egui::CtxRef) {
        let Self { show_about, .. } = self;
        if *show_about {
            egui::Window::new("About").show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("Physics Plotter ");
                    ui.label(crate_version!());
                    egui::warn_if_debug_build(ui);
                });
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("Interface powered by ");
                    ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                });
                ui.hyperlink_to(
                    "Project page",
                    "https://github.com/myzhang1029/phys_plotter",
                );
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label(defv::COPYRIGHT);
                });
                if ui.add(egui::Button::new("Close")).clicked() {
                    self.show_about = false;
                }
            });
        }
    }

    fn ui_file_drag_and_drop(&mut self, ctx: &egui::Context) {
        if !ctx.input().raw.dropped_files.is_empty() {
            let dropped_file = &ctx.input().raw.dropped_files;
            if dropped_file.len() != 1 {
                self.error = Some(String::from("Please only drop one file"));
            } else if let Some(path) = &dropped_file[0].path {
                let path = path.display().to_string();
                self.load_file(&path);
            }
        }
    }

    fn reset(&mut self) {
        *self = App::default();
    }

    /// Load path into the state and decide whether to modify `self.file_path`
    fn load_file(&mut self, path: &str) {
        // First try to parse it as saved file
        if let Ok(val) = PhysPlotterFile::from_file(&path) {
            match Backends::from_str(&val.backend_name) {
                Ok(result) => {
                    self.file_path = path.to_owned();
                    self.saved = true;
                    self.backend = result;
                    self.title = val.title;
                    self.dataset = val.dataset;
                    self.x_label = val.x_label;
                    self.y_label = val.y_label;
                    self.default_x_uncertainty = format!("{}", val.default_x_uncertainty);
                    self.default_y_uncertainty = format!("{}", val.default_y_uncertainty);
                }
                Err(error) => {
                    self.error = Some(format!(
                        "Unknown backend type {}: {}",
                        val.backend_name, error
                    ));
                }
            }
        } else {
            // Else treat as plain dataset text
            // Treat this as not saved
            self.file_path = String::default();
            self.saved = false;
            match File::open(&path) {
                Ok(mut file) => {
                    let _ignore = file.read_to_string(&mut self.dataset);
                }
                Err(error) => {
                    self.error = Some(format!("Cannot open file: {}", error));
                }
            }
        }
    }

    #[cfg(feature = "ui_egui_with_file")]
    fn open(&mut self) {
        if let Some(path) = rfd::FileDialog::new().pick_file() {
            let path = path.display().to_string();
            self.load_file(&path);
        }
    }

    #[cfg(not(feature = "ui_egui_with_file"))]
    fn open(&mut self) {}

    #[cfg(feature = "ui_egui_with_file")]
    fn save(&mut self) {
        let try_save_file: Result<PhysPlotterFile, _> = self.clone().try_into();
        match try_save_file {
            Ok(save_file) => {
                if self.file_path.is_empty() {
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter("Physics Plotter File", &["pyp"])
                        .save_file()
                    {
                        self.file_path = path.display().to_string();
                    } else {
                        return;
                    }
                }
                if let Err(error) = save_file.save_to(&self.file_path) {
                    self.error = Some(format!("{}", error));
                } else {
                    self.saved = true;
                }
            }
            Err(error) => {
                self.error = Some(format!("{}", error));
            }
        }
    }

    fn parse_dataset(&self) -> Result<TwoVarDataSet, String> {
        match (
            self.default_x_uncertainty.parse(),
            self.default_y_uncertainty.parse(),
        ) {
            (Ok(dux), Ok(duy)) => match TwoVarDataSet::from_string(&self.dataset, dux, duy) {
                Ok(dataset) => {
                    // Empty values can crash some backends
                    if dataset.is_empty() {
                        Err(String::from("Empty dataset"))
                    } else {
                        Ok(dataset)
                    }
                }
                Err(error) => Err(format!("Invalid dataset: {}", error)),
            },
            (Err(_), _) => Err(format!(
                "Invalid x uncertainty: {}",
                self.default_x_uncertainty
            )),
            (_, Err(_)) => Err(format!(
                "Invalid y uncertainty: {}",
                self.default_y_uncertainty
            )),
        }
    }

    fn plot(&mut self) {
        let dataset = match self.parse_dataset() {
            Ok(dataset) => dataset,
            Err(error) => {
                self.error = Some(error);
                return;
            }
        };

        // Call plotting backend
        match self.backend {
            Backends::Gnuplot => {
                if let Err(error) =
                    plot::gnuplot(&self.title, &self.x_label, &self.y_label, &dataset, None)
                {
                    self.error = Some(format!("Error while opening GNU Plot: {}", error));
                }
            }
            Backends::Plotters => {
                self.error = Some(String::from("Not supported yet."));
            }
        };
    }
}

/// Create save file from the state
impl TryInto<PhysPlotterFile> for App {
    type Error = <f64 as FromStr>::Err;

    fn try_into(self) -> Result<PhysPlotterFile, Self::Error> {
        Ok(PhysPlotterFile {
            creator: defv::APP_ID.to_string(),
            version: crate_version!().to_string(),
            title: self.title.clone(),
            backend_name: format!("{}", self.backend),
            x_label: self.x_label.clone(),
            y_label: self.y_label.clone(),
            default_x_uncertainty: self.default_x_uncertainty.parse()?,
            default_y_uncertainty: self.default_y_uncertainty.parse()?,
            dataset: self.dataset.clone(),
        })
    }
}
