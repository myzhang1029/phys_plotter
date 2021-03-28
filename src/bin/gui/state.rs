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

use clap::crate_version;
use gtk::prelude::*;
use gtk::{EntryBuffer, TextBuffer, TextBufferBuilder};
use phys_plotter::default_values as defv;
use phys_plotter::save_format::PhysPlotterFile;
use std::convert::{TryFrom, TryInto};
use std::str::FromStr;

/// Available backends
#[derive(Debug, Copy, Clone)]
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

/// Struct for GUI app UI state
#[derive(Debug, Clone)]
pub struct UIState {
    pub saved: bool,
    pub file_path: String,
    pub backend: Backends,
    pub title: EntryBuffer,
    pub dataset: TextBuffer,
    pub x_label: EntryBuffer,
    pub y_label: EntryBuffer,
    pub default_x_uncertainty: EntryBuffer,
    pub default_y_uncertainty: EntryBuffer,
}

impl UIState {
    /// Create a new state
    pub fn new() -> Self {
        Self {
            saved: true,
            file_path: Default::default(),
            title: EntryBuffer::new(Some(defv::TITLE)),
            dataset: TextBufferBuilder::new().build(),
            backend: Backends::from_str(defv::BACKEND).unwrap(),
            x_label: EntryBuffer::new(Some(defv::X_LABEL)),
            y_label: EntryBuffer::new(Some(defv::Y_LABEL)),
            default_x_uncertainty: EntryBuffer::new(Some(defv::X_UNCERTAINTY)),
            default_y_uncertainty: EntryBuffer::new(Some(defv::X_UNCERTAINTY)),
        }
    }

    /// Get the value of the dataset
    pub fn dataset_str(&self) -> String {
        let range = self.dataset.get_bounds();
        self.dataset
            .get_text(&range.0, &range.1, true)
            .unwrap_or_else(|| glib::GString::from(""))
            .to_string()
    }

    /// Save to PhysPlotterFile
    pub fn save(&self) -> std::io::Result<()> {
        let try_save_file: Result<PhysPlotterFile, _> = self.clone().try_into();
        if let Ok(save_file) = try_save_file {
            save_file.save_to(&self.file_path)
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Cannot parse float",
            ))
        }
    }

    /// Safely replace this state, ensures that the views are updated
    pub fn replace(&mut self, other: UIState) {
        self.saved = other.saved;
        self.title.set_text(&other.title.get_text());
        self.dataset.set_text(&other.dataset_str());
        self.file_path = other.file_path;
        self.backend = other.backend;
        self.x_label.set_text(&other.x_label.get_text());
        self.y_label.set_text(&other.y_label.get_text());
        self.default_x_uncertainty
            .set_text(&other.default_x_uncertainty.get_text());
        self.default_y_uncertainty
            .set_text(&other.default_y_uncertainty.get_text());
    }
}

/// Create save file from the state
impl TryInto<PhysPlotterFile> for UIState {
    type Error = <f64 as FromStr>::Err;
    fn try_into(self) -> Result<PhysPlotterFile, Self::Error> {
        Ok(PhysPlotterFile {
            creator: defv::APP_ID.to_string(),
            version: crate_version!().to_string(),
            backend_name: format!("{}", self.backend),
            x_label: self.x_label.get_text(),
            y_label: self.y_label.get_text(),
            default_x_uncertainty: self.default_x_uncertainty.get_text().parse()?,
            default_y_uncertainty: self.default_x_uncertainty.get_text().parse()?,
            dataset: self.dataset_str(),
            title: self.file_path,
        })
    }
}

/// Load saved file
impl TryFrom<PhysPlotterFile> for UIState {
    type Error = std::io::Error;
    fn try_from(that: PhysPlotterFile) -> Result<Self, Self::Error> {
        Ok(Self {
            saved: true,
            file_path: Default::default(),
            title: EntryBuffer::new(Some(defv::TITLE)),
            dataset: TextBufferBuilder::new().text(&that.dataset).build(),
            backend: Backends::from_str(defv::BACKEND).unwrap(),
            x_label: EntryBuffer::new(Some(defv::X_LABEL)),
            y_label: EntryBuffer::new(Some(defv::Y_LABEL)),
            default_x_uncertainty: EntryBuffer::new(Some(defv::X_UNCERTAINTY)),
            default_y_uncertainty: EntryBuffer::new(Some(defv::X_UNCERTAINTY)),
        })
    }
}
