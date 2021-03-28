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

use serde::{Deserialize, Serialize};
use serde_json::to_writer;
use std::fs::File;
use std::io::BufWriter;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PhysPlotterFile {
    pub version: String,
    pub creator: String,
    pub title: String,
    pub backend_name: String,
    pub x_label: String,
    pub y_label: String,
    pub default_x_uncertainty: f64,
    pub default_y_uncertainty: f64,
    pub dataset: String,
}

impl PhysPlotterFile {
    /// Save this file
    pub fn save_to(&self, filename: &str) -> std::io::Result<()> {
        let file = File::create(filename)?;
        let writer = BufWriter::new(file);
        to_writer(writer, self)?;
        Ok(())
    }
}
