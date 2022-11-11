//
//  Copyright (C) 2021 Zhang Maiyun <me@myzhangll.xyz>
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
use phys_plotter::default_values as defv;
use phys_plotter::plot;
use phys_plotter::save_format::PhysPlotterFile;
use plotters::prelude::*;
use std::process::exit;

/// Validator for uncertainties
fn du_validator(num: &str) -> Result<(), String> {
    match num.parse::<f64>() {
        Ok(_) => Ok(()),
        Err(error) => Err(format!("{}", error)),
    }
}

/// Validator for height and width
fn size_validator(num: &str) -> Result<(), String> {
    match num.parse::<u32>() {
        Ok(_) => Ok(()),
        Err(error) => Err(format!("{}", error)),
    }
}

fn main() {
    let matches = App::new("Physics Plotter")
        .version(crate_version!())
        .author("Zhang Maiyun <me@myzhangll.xyz>")
        .about("Plot physics two-variable observation data with best-fit lines, max,min-gradient lines, and error bars.")
        .arg(Arg::new("DATASET_FILE")
            .help("Sets the data file to parse")
            .required(true)
            .index(1))
        .arg(Arg::new("psp_file")
            .help("Indicates that aphysics plotter saved file is used as DATASET_FILE")
            .short('p')
            .long("psp-file")
            .conflicts_with_all(&["title", "x_label", "y_label", "dux", "duy", "backend"]))
        .arg(Arg::new("title")
            .short('t')
            .long("title")
            .value_name("TITLE")
            .default_value(defv::TITLE)
            .help("Sets the title of the plot"))
        .arg(Arg::new("x_label")
            .short('x')
            .long("x-label")
            .value_name("X_LABEL")
            .default_value(defv::X_LABEL)
            .help("Sets the x axis label"))
        .arg(Arg::new("y_label")
            .short('y')
            .long("y-label")
            .value_name("Y_LABEL")
            .default_value(defv::Y_LABEL)
            .help("Sets the y axis label"))
        .arg(Arg::new("dux")
            .short('X')
            .long("default-ux")
            .value_name("DEFAULT_X_UNCERTAINTY")
            .default_value(defv::X_UNCERTAINTY)
            .validator(du_validator)
            .help("Sets a default value for x uncertainty"))
        .arg(Arg::new("duy")
            .short('Y')
            .long("default-uy")
            .value_name("DEFAULT_Y_UNCERTAINTY")
            .default_value(defv::Y_UNCERTAINTY)
            .validator(du_validator)
            .help("Sets a default value for y uncertainty"))
        .arg(Arg::new("backend")
            .short('b')
            .long("backend")
            .value_name("BACKEND")
            .possible_value("gnuplot")
            .possible_value("plotters")
            .default_value(defv::BACKEND)
            .help("Sets the plotting backend"))
        .arg(Arg::new("out_file")
            .short('s')
            .long("save-to")
            .value_name("PATH")
            .requires("width")
            .requires("height")
            .required_if_eq("backend", "plotters")
            .help("Saves the graph to PATH instead of showing it"))
        .arg(Arg::new("width")
            .short('w')
            .long("width")
            .value_name("WIDTH")
            .requires("out_file")
            .validator(size_validator)
            .help("Sets the image width in pixels (recommended: 960)"))
        .arg(Arg::new("height")
            .short('h')
            .long("height")
            .value_name("HEIGHT")
            .requires("out_file")
            .validator(size_validator)
            .help("Sets the image height in pixels (recommended: 540)"))
        .get_matches();

    let (dataset, title, x_label, y_label) = if matches.is_present("psp_file") {
        // Parse as PhysPlotterFile
        let save_file =
            PhysPlotterFile::from_file(&matches.value_of("DATASET_FILE").unwrap()).unwrap();
        (
            TwoVarDataSet::from_string(
                &save_file.dataset,
                save_file.default_x_uncertainty,
                save_file.default_y_uncertainty,
            ),
            save_file.title,
            save_file.x_label,
            save_file.y_label,
        )
    } else {
        // Parse as plain dataset
        (
            TwoVarDataSet::from_file(
                &matches.value_of("DATASET_FILE").unwrap(),
                matches.value_of("dux").unwrap().parse().unwrap(),
                matches.value_of("dux").unwrap().parse().unwrap(),
            ),
            matches.value_of("title").unwrap().to_string(),
            matches.value_of("x_label").unwrap().to_string(),
            matches.value_of("y_label").unwrap().to_string(),
        )
    };
    if let Err(error) = dataset {
        eprintln!("Error: {}", error);
        exit(2);
    }
    match matches.value_of("backend").unwrap() {
        "plotters" => plot::plotters(
            &title,
            &x_label,
            &y_label,
            &dataset.unwrap(),
            // The clap rule will ensure that this argument exists
            BitMapBackend::new(
                matches.value_of("out_file").unwrap(),
                (
                    matches.value_of("width").unwrap().parse().unwrap(),
                    matches.value_of("height").unwrap().parse().unwrap(),
                ),
            ),
        )
        .unwrap(),
        "gnuplot" => plot::gnuplot(
            &title,
            &x_label,
            &y_label,
            &dataset.unwrap(),
            matches.value_of("out_file").map(|path| plot::SaveOptions {
                path: std::path::Path::new(path),
                width: matches.value_of("width").unwrap().parse().unwrap(),
                height: matches.value_of("height").unwrap().parse().unwrap(),
            }),
        )
        .unwrap(),
        _ => (),
    }
}
