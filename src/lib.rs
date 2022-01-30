#![warn(clippy::all, rust_2018_idioms)]
pub mod data;
pub mod default_values;
#[cfg(feature = "ui_egui")]
pub mod egui_ui;
pub mod plot;
pub mod save_format;

#[cfg(feature = "ui_egui")]
#[cfg(target_arch = "wasm32")]
use eframe::wasm_bindgen::{self, prelude::*};

#[cfg(feature = "ui_egui")]
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn start(canvas_id: &str) -> Result<(), eframe::wasm_bindgen::JsValue> {
    let app = egui_ui::App::default();
    eframe::start_web(canvas_id, Box::new(app))
}
