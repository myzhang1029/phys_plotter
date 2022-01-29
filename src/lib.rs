#![warn(clippy::all, rust_2018_idioms)]
pub mod data;
pub mod default_values;
#[cfg(feature = "ui_egui")]
pub mod egui_ui;
pub mod plot;
pub mod save_format;
