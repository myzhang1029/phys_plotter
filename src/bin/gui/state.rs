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

use gtk::{EntryBuffer, TextBuffer, TextBufferBuilder};
use phys_plotter::default_values as defv;
use phys_plotter::save_format::PhysPlotterFile;
use std::convert::TryInto;
use std::str::FromStr;

/// Struct for GUI app UI state
#[derive(Debug)]
pub struct UIState {
    pub file_path: String,
    pub dataset_file: String,
    pub title: EntryBuffer,
    pub dataset: TextBuffer,
    pub backend_name: EntryBuffer,
    pub x_label: EntryBuffer,
    pub y_label: EntryBuffer,
    pub default_x_uncertainty: EntryBuffer,
    pub default_y_uncertainty: EntryBuffer,
}

impl UIState {
    /// Create a new state
    pub fn new() -> Self {
        Self {
            file_path: Default::default(),
            dataset_file: Default::default(),
            title: EntryBuffer::new(Some(defv::TITLE)),
            dataset: TextBufferBuilder::new().build(),
            backend_name: EntryBuffer::new(Some(defv::BACKEND)),
            x_label: EntryBuffer::new(Some(defv::X_LABEL)),
            y_label: EntryBuffer::new(Some(defv::Y_LABEL)),
            default_x_uncertainty: EntryBuffer::new(Some(defv::X_UNCERTAINTY)),
            default_y_uncertainty: EntryBuffer::new(Some(defv::X_UNCERTAINTY)),
        }
    }
}

/// Create save file from the state
impl TryInto<PhysPlotterFile> for UIState {
    type Error = <f64 as FromStr>::Err;
    fn try_into(self) -> Result<PhysPlotterFile, Self::Error> {
        Ok(PhysPlotterFile {
            title: self.file_path.to_string(),
            backend_name: self.backend_name.get_text(),
            x_label: self.x_label.get_text(),
            y_label: self.y_label.get_text(),
            default_x_uncertainty: self.default_x_uncertainty.get_text().parse()?,
            default_y_uncertainty: self.default_x_uncertainty.get_text().parse()?,
            dataset_file: self.dataset_file,
        })
    }
}
