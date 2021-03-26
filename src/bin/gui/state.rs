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
use std::cell::{Cell, RefCell};
use std::convert::{TryFrom, TryInto};
use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;

/// Struct for GUI app UI state
#[derive(Debug)]
pub struct UIState {
    pub saved: Cell<bool>,
    pub file_path: RefCell<String>,
    pub dataset_file: RefCell<String>,
    pub title: RefCell<EntryBuffer>,
    pub dataset: RefCell<TextBuffer>,
    pub backend_name: RefCell<EntryBuffer>,
    pub x_label: RefCell<EntryBuffer>,
    pub y_label: RefCell<EntryBuffer>,
    pub default_x_uncertainty: RefCell<EntryBuffer>,
    pub default_y_uncertainty: RefCell<EntryBuffer>,
}

impl UIState {
    /// Create a new state
    pub fn new() -> Self {
        Self {
            saved: Cell::new(true),
            file_path: Default::default(),
            dataset_file: Default::default(),
            title: RefCell::new(EntryBuffer::new(Some(defv::TITLE))),
            dataset: RefCell::new(TextBufferBuilder::new().build()),
            backend_name: RefCell::new(EntryBuffer::new(Some(defv::BACKEND))),
            x_label: RefCell::new(EntryBuffer::new(Some(defv::X_LABEL))),
            y_label: RefCell::new(EntryBuffer::new(Some(defv::Y_LABEL))),
            default_x_uncertainty: RefCell::new(EntryBuffer::new(Some(defv::X_UNCERTAINTY))),
            default_y_uncertainty: RefCell::new(EntryBuffer::new(Some(defv::X_UNCERTAINTY))),
        }
    }

    /// Save modified dataset
    pub fn save_dataset(&self) -> Result<(), std::io::Error> {
        let borrowed_path = &*self.dataset_file.borrow();
        let mut dataset_file = File::create(&borrowed_path)?;
        let borrowed_dataset = self.dataset.borrow();
        let range = borrowed_dataset.get_bounds();
        let dataset = borrowed_dataset.get_text(&range.0, &range.1, true);
        if let Some(dataset) = dataset {
            dataset_file.write_all(dataset.as_bytes())?;
        }
        Ok(())
    }
}

/// Create save file from the state
impl TryInto<PhysPlotterFile> for UIState {
    type Error = <f64 as FromStr>::Err;
    fn try_into(self) -> Result<PhysPlotterFile, Self::Error> {
        Ok(PhysPlotterFile {
            title: self.file_path.take(),
            backend_name: self.backend_name.borrow().get_text(),
            x_label: self.x_label.borrow().get_text(),
            y_label: self.y_label.borrow().get_text(),
            default_x_uncertainty: self.default_x_uncertainty.borrow().get_text().parse()?,
            default_y_uncertainty: self.default_x_uncertainty.borrow().get_text().parse()?,
            dataset_file: self.dataset_file.take(),
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
            saved: Cell::new(true),
            file_path: Default::default(),
            dataset_file: RefCell::new(that.dataset_file),
            title: RefCell::new(EntryBuffer::new(Some(defv::TITLE))),
            dataset: RefCell::new(TextBufferBuilder::new().build()),
            backend_name: RefCell::new(EntryBuffer::new(Some(defv::BACKEND))),
            x_label: RefCell::new(EntryBuffer::new(Some(defv::X_LABEL))),
            y_label: RefCell::new(EntryBuffer::new(Some(defv::Y_LABEL))),
            default_x_uncertainty: RefCell::new(EntryBuffer::new(Some(defv::X_UNCERTAINTY))),
            default_y_uncertainty: RefCell::new(EntryBuffer::new(Some(defv::X_UNCERTAINTY))),
        })
    }
}
