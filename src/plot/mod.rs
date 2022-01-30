#[macro_use]
mod label;
mod plot_gnuplot;
mod plot_plotters;
mod save_options;

pub use plot_gnuplot::gnuplot;
pub use plot_plotters::plotters;
pub use save_options::SaveOptions;
use std::str::FromStr;

/// Available backends
#[derive(PartialEq, Eq, Debug, Copy, Clone, serde::Deserialize, serde::Serialize)]
pub enum Backends {
    Gnuplot,
    Plotters,
}

impl std::fmt::Display for Backends {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Backends::Gnuplot => write!(f, "gnuplot"),
            Backends::Plotters => write!(f, "plotters"),
        }
    }
}

/// Error when converting from str to Backends
#[derive(Clone)]
pub enum BackendsFromStrError {
    UnknownBackend(String),
}

impl std::fmt::Display for BackendsFromStrError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BackendsFromStrError::UnknownBackend(bstr) => write!(f, "Unknown backend: {}", bstr),
        }
    }
}

impl std::fmt::Debug for BackendsFromStrError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BackendsFromStrError::UnknownBackend(bstr) => write!(f, "Unknown backend: {:?}", bstr),
        }
    }
}

impl FromStr for Backends {
    type Err = BackendsFromStrError;

    /// Parse backend description
    fn from_str(bstr: &str) -> Result<Self, Self::Err> {
        match bstr.to_lowercase().as_str() {
            "plotters" => Ok(Backends::Plotters),
            "gnuplot" => Ok(Backends::Gnuplot),
            other => Err(Self::Err::UnknownBackend(other.to_string())),
        }
    }
}
