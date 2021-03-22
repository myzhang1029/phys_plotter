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

extern crate clap;
extern crate gnuplot;

mod linear_data;
mod plot;
mod two_var_data;

use clap::{App, Arg};
use std::process::exit;

fn main() {
    let matches = App::new("Physics Plotter")
        .version("0.1")
        .author("Zhang Maiyun <myzhang1029@hotmail.com>")
        .about("Plot physics two-variable observation data with best-fit lines, max,min-gradient lines, and error bars.")
        .arg(Arg::with_name("DATASET_FILE")
            .help("Sets the data file to parse")
            .required(true)
            .index(1))
        .arg(Arg::with_name("title")
            .short("t")
            .long("config")
            .value_name("TITLE")
            .help("Sets the title of the plot"))
        .arg(Arg::with_name("x_label")
            .short("x")
            .long("x-label")
            .value_name("X_LABEL")
            .help("Sets the x axis label"))
        .arg(Arg::with_name("y_label")
            .short("y")
            .long("y-label")
            .value_name("Y_LABEL")
            .help("Sets the y axis label"))
        .get_matches();
    let dataset = two_var_data::TwoVarDataSet::from_file(
        &matches.value_of("DATASET_FILE").unwrap(),
        0.01,
        0.01,
    );
    if let Err(error) = dataset {
        eprintln!("Error: {}", error);
        exit(2);
    }
    plot::plot(
        matches.value_of("title").unwrap_or("Some Plot"),
        matches.value_of("x_label").unwrap_or("x"),
        matches.value_of("y_label").unwrap_or("y"),
        dataset.unwrap(),
    );
}
