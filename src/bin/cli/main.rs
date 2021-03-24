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

use clap::{crate_version, App, Arg};
use phys_plotter::data::TwoVarDataSet;
use phys_plotter::plot;
use std::process::exit;

/// Validator for uncertainties
fn du_validator(num: String) -> Result<(), String> {
    match num.parse::<f64>() {
        Ok(_) => Ok(()),
        Err(error) => Err(format!("{}", error)),
    }
}

fn main() {
    let matches = App::new("Physics Plotter")
        .version(crate_version!())
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
            .default_value("Some Title")
            .help("Sets the title of the plot"))
        .arg(Arg::with_name("x_label")
            .short("x")
            .long("x-label")
            .value_name("X_LABEL")
            .default_value("x")
            .help("Sets the x axis label"))
        .arg(Arg::with_name("y_label")
            .short("y")
            .long("y-label")
            .value_name("Y_LABEL")
            .default_value("y")
            .help("Sets the y axis label"))
        .arg(Arg::with_name("dux")
            .short("X")
            .long("default-ux")
            .value_name("DEFAULT_X_UNCERTAINTY")
            .default_value("0.01")
            .validator(du_validator)
            .help("Sets a default value for x uncertainty"))
        .arg(Arg::with_name("duy")
            .short("Y")
            .long("default-uy")
            .value_name("DEFAULT_Y_UNCERTAINTY")
            .validator(du_validator)
            .default_value("0.01")
            .help("Sets a default value for y uncertainty"))
        .arg(Arg::with_name("backend")
            .short("b")
            .long("backend")
            .value_name("BACKEND")
            .possible_value("gnuplot")
            .possible_value("plotters")
            .default_value("plotters")
            .help("Sets the plotting backend"))
        .get_matches();
    let dataset = TwoVarDataSet::from_file(
        &matches.value_of("DATASET_FILE").unwrap(),
        matches.value_of("dux").unwrap().parse().unwrap(),
        matches.value_of("dux").unwrap().parse().unwrap(),
    );
    if let Err(error) = dataset {
        eprintln!("Error: {}", error);
        exit(2);
    }
    match matches.value_of("backend").unwrap() {
        "plotters" => plot::plot_plotters(
            matches.value_of("title").unwrap(),
            matches.value_of("x_label").unwrap(),
            matches.value_of("y_label").unwrap(),
            dataset.unwrap(),
        )
        .unwrap(),
        "gnuplot" => plot::plot_gnuplot(
            matches.value_of("title").unwrap(),
            matches.value_of("x_label").unwrap(),
            matches.value_of("y_label").unwrap(),
            dataset.unwrap(),
        )
        .unwrap(),
        _ => (),
    }
}
