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

use gtk::prelude::*;
use gtk::{EntryBuffer, TextBuffer, TextBufferBuilder};
use phys_plotter::default_values as defv;
use phys_plotter::save_format::PhysPlotterFile;
use std::convert::{TryFrom, TryInto};
use std::fs::File;
use std::io::prelude::*;
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
#[derive(Debug)]
pub struct UIState {
    pub saved: bool,
    pub file_path: String,
    pub dataset_file: String,
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
            dataset_file: Default::default(),
            title: EntryBuffer::new(Some(defv::TITLE)),
            dataset: TextBufferBuilder::new().build(),
            backend: Backends::from_str(defv::BACKEND).unwrap(),
            x_label: EntryBuffer::new(Some(defv::X_LABEL)),
            y_label: EntryBuffer::new(Some(defv::Y_LABEL)),
            default_x_uncertainty: EntryBuffer::new(Some(defv::X_UNCERTAINTY)),
            default_y_uncertainty: EntryBuffer::new(Some(defv::X_UNCERTAINTY)),
        }
    }

    /// Save modified dataset
    pub fn save_dataset(&self) -> Result<(), std::io::Error> {
        let mut dataset_file = File::create(&self.dataset_file)?;
        let range = self.dataset.get_bounds();
        let dataset = self.dataset.get_text(&range.0, &range.1, true);
        if let Some(dataset) = dataset {
            dataset_file.write_all(dataset.as_bytes())?;
        }
        Ok(())
    }

    /// Safely  this state, ensures that the views are updated
    pub fn replace(&mut self, other: UIState) {
        self.saved = other.saved;
        self.file_path = other.file_path;
        self.dataset_file = other.dataset_file;
        self.title.set_text(&other.title.get_text());
        let range = other.dataset.get_bounds();
        self.dataset.set_text(
            &other
                .dataset
                .get_text(&range.0, &range.1, true)
                .unwrap_or_else(|| glib::GString::from("")),
        );
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
            title: self.file_path,
            backend_name: format!("{}", self.backend),
            x_label: self.x_label.get_text(),
            y_label: self.y_label.get_text(),
            default_x_uncertainty: self.default_x_uncertainty.get_text().parse()?,
            default_y_uncertainty: self.default_x_uncertainty.get_text().parse()?,
            dataset_file: self.dataset_file,
        })
    }
}

/// Load saved file
impl TryFrom<PhysPlotterFile> for UIState {
    type Error = std::io::Error;
    fn try_from(that: PhysPlotterFile) -> Result<Self, Self::Error> {
        let mut data_file = File::open(&that.dataset_file)?;
        let mut contents = String::new();
        data_file.read_to_string(&mut contents)?;
        Ok(Self {
            saved: true,
            file_path: Default::default(),
            dataset_file: that.dataset_file,
            title: EntryBuffer::new(Some(defv::TITLE)),
            dataset: TextBufferBuilder::new().build(),
            backend: Backends::from_str(defv::BACKEND).unwrap(),
            x_label: EntryBuffer::new(Some(defv::X_LABEL)),
            y_label: EntryBuffer::new(Some(defv::Y_LABEL)),
            default_x_uncertainty: EntryBuffer::new(Some(defv::X_UNCERTAINTY)),
            default_y_uncertainty: EntryBuffer::new(Some(defv::X_UNCERTAINTY)),
        })
    }
}
