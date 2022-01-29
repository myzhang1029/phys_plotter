use super::default_values as defv;
use super::plot::Backends;
use clap::crate_version;
use eframe::{egui, epi};
use std::str::FromStr;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct App {
    saved: bool,
    show_about: bool,
    file_path: String,
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
            file_path: Default::default(),
            backend: Backends::from_str(defv::BACKEND).unwrap(),
            title: String::from(defv::TITLE),
            dataset: Default::default(),
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
            *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
        }
    }

    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::CtxRef, frame: &epi::Frame) {
        self.draw_top_menu(ctx, frame);
        self.draw_side_panel(ctx);
        self.draw_dataset_area(ctx);
        self.draw_about_window(ctx);
        self.ui_file_drag_and_drop(ctx);
    }
}

impl App {
    fn draw_top_menu(&mut self, ctx: &egui::CtxRef, frame: &epi::Frame) {
        let Self {
            saved,
            show_about,
            title,
            x_label,
            y_label,
            default_x_uncertainty,
            default_y_uncertainty,
            dataset,
            backend,
            file_path,
        } = self;

        egui::TopBottomPanel::top("oper_menu").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                if ui.button("New").clicked() {
                    /*if !saved {
                        self.not_saved_confirmation();
                    }*/
                }
                if ui.button("Open").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_file() {
                        self.file_path = path.display().to_string();
                    }
                }
                if ui.button("Save").clicked() {
                    frame.quit();
                }
                if ui.button("Save As").clicked() {
                    if let Some(path) = rfd::FileDialog::new().save_file() {
                        self.file_path = path.display().to_string();
                    }
                }
                if ui.button("Plot").clicked() {
                    frame.quit();
                }
                if ui.button("About").clicked() {
                    self.show_about = true;
                }
                if ui.button("Quit").clicked() {
                    frame.quit();
                }
            });
        });
    }

    fn draw_side_panel(&mut self, ctx: &egui::CtxRef) {
        let Self {
            title,
            x_label,
            y_label,
            default_x_uncertainty,
            default_y_uncertainty,
            backend,
            ..
        } = self;

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Properties");

            ui.horizontal(|ui| {
                ui.label("Plot title");
                ui.text_edit_singleline(title);
            });
            ui.horizontal(|ui| {
                ui.label("X axis label");
                ui.text_edit_singleline(x_label);
            });
            ui.horizontal(|ui| {
                ui.label("Y axis label");
                ui.text_edit_singleline(y_label);
            });
            ui.horizontal(|ui| {
                ui.label("Default x uncertainty");
                ui.text_edit_singleline(default_x_uncertainty);
            });
            ui.horizontal(|ui| {
                ui.label("Default y uncertainty");
                ui.text_edit_singleline(default_y_uncertainty);
            });
            ui.horizontal(|ui| {
                ui.label("Plotting backend");
                ui.selectable_value(backend, Backends::Gnuplot, "GNU Plot");
                ui.selectable_value(backend, Backends::Plotters, "Plotters");
            })
        });
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
                    ui.label("Copyright (C) 2021 Zhang Maiyun. ");
                });
                if ui.add(egui::Button::new("Close")).clicked() {
                    *show_about = false;
                }
            });
        }
    }

    fn draw_dataset_area(&mut self, ctx: &egui::CtxRef) {
        let Self { dataset, .. } = self;
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Dataset");
            let dsbox = egui::TextEdit::multiline(dataset)
                .desired_width(f32::INFINITY)
                .desired_rows(10)
                .cursor_at_end(true);
            ui.add(dsbox);
        });
    }

    fn ui_file_drag_and_drop(&mut self, ctx: &egui::Context) {
        if !ctx.input().raw.dropped_files.is_empty() {
            let dropped_file = &ctx.input().raw.dropped_files;
            if dropped_file.len() != 1 {}
            dbg!(dropped_file);
        }
    }
}
