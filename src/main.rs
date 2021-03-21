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
//  along with sib secure shell.  If not, see <https://www.gnu.org/licenses/>.
//

extern crate gnuplot;

mod plot;
mod two_var_data;

use std::env;
use std::process::exit;

fn main() {
    let arguments: Vec<String> = env::args().collect();
    if arguments.len() <= 1 {
        eprintln!("Usage: {} <dataset_file>", arguments[0]);
        exit(1);
    }
    let dataset = two_var_data::TwoVarDataSet::from_file(&arguments[1], 0.01, 0.01);
    if let Err(error) = dataset {
        eprintln!("Error: {}", error);
        exit(2);
    }
    plot::plot(
        "Atmospheric Pressure and Height",
        "Relative Height/m",
        "Atmospheric Pressure/Pa",
        dataset.unwrap(),
    );
}
