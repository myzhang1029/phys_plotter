#[macro_use]
mod label;
mod plot_gnuplot;
mod plot_plotters;
mod save_options;

pub use plot_gnuplot::plot_gnuplot;
pub use plot_plotters::plot_plotters;
pub use save_options::SaveOptions;
