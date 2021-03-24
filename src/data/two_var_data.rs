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

use crate::data::{Line, Point};
use std::fs::File;
use std::io::prelude::*;
use std::io::{Error, ErrorKind};
use std::ops::Index;
use std::slice::Iter;

/// Parse error types
pub enum ParseError {
    EmptyField,
    BadFields,
}

/// Struct representing a two-variable data and the uncertainties
#[derive(Debug, Default, Copy, Clone)]
pub struct TwoVarDataPoint {
    pub x_value: f64,
    pub x_uncertainty: f64,
    pub y_value: f64,
    pub y_uncertainty: f64,
}

/// Rules to define the function to get minimum/maximum x/y
macro_rules! raw_defun_minmax {
    ($name: ident, $cmp: ident, $val_name: ident, $uncer_name: ident, $uncer_sign: tt) => {
        /// Get the $name value. if with_uncertainty is true, the uncertainties is also taken into account
        pub fn $name(&self, with_uncertainty: bool) -> f64 {
            if with_uncertainty {
                let k = self.iter()
                    .$cmp(|one, another| (one.$val_name $uncer_sign one.$uncer_name).partial_cmp(&(another.$val_name $uncer_sign another.$uncer_name)).unwrap())
                    .unwrap();
                k.$val_name $uncer_sign k.$uncer_name
            } else {
                self.iter()
                    .$cmp(|one, another| one.$val_name.partial_cmp(&another.$val_name).unwrap())
                    .unwrap()
                    .$val_name
            }
        }
    };
}

impl TwoVarDataPoint {
    /// Parse a line
    /// line: The line to parse
    /// dux: Default x uncertainty
    /// duy: Default y uncertainty
    pub fn from_line(line: &str, dux: f64, duy: f64) -> Result<Self, ParseError> {
        let mut fields: Vec<f64> = Vec::with_capacity(4);
        let mut line = line;
        // Exhaust this line by taking all numeric fields
        while let Some((number, (_, end_point))) = atof(line) {
            fields.push(number);
            line = &line[end_point..];
            // break when no data can be processed anymore
        }
        // Append to the object
        match fields.len() {
            0 => Err(ParseError::EmptyField),
            2 => Ok(TwoVarDataPoint {
                x_value: fields[0],
                x_uncertainty: dux,
                y_value: fields[1],
                y_uncertainty: duy,
            }),
            3 => Ok(TwoVarDataPoint {
                x_value: fields[0],
                x_uncertainty: dux,
                y_value: fields[1],
                y_uncertainty: fields[2],
            }),
            4 => Ok(TwoVarDataPoint {
                x_value: fields[0],
                x_uncertainty: fields[1],
                y_value: fields[2],
                y_uncertainty: fields[3],
            }),
            _ => Err(ParseError::BadFields),
        }
    }
}

/// Struct representing a set of two-variable data and their uncertainties
#[derive(Debug, Default, Clone)]
pub struct TwoVarDataSet {
    dataset: Vec<TwoVarDataPoint>,
}

impl Index<usize> for TwoVarDataSet {
    type Output = TwoVarDataPoint;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.dataset[idx]
    }
}

impl TwoVarDataSet {
    /// Parse a data file
    /// filename: Path to the file
    /// dux: Default x uncertainty
    /// duy: Default y uncertainty
    pub fn from_file(filename: &str, dux: f64, duy: f64) -> Result<Self, Error> {
        // Read the data file
        let mut data_file = File::open(filename)?;
        let mut contents = String::new();
        data_file.read_to_string(&mut contents)?;
        // Split into lines
        let lines: Vec<&str> = contents.split('\n').collect();
        let mut result: Vec<TwoVarDataPoint> = Vec::with_capacity(lines.len());
        for line in lines {
            match TwoVarDataPoint::from_line(line, dux, duy) {
                Ok(data) => result.push(data),
                Err(ParseError::EmptyField) => continue,
                Err(ParseError::BadFields) => {
                    return Err(Error::new(
                        ErrorKind::Other,
                        format!("unknown fields {:?}", line),
                    ))
                }
            }
        }
        Ok(TwoVarDataSet { dataset: result })
    }

    /// Length of the underlying vector
    pub fn len(&self) -> usize {
        self.dataset.len()
    }

    /// Get an iterator of the underlying vector
    pub fn iter(&self) -> Iter<TwoVarDataPoint> {
        self.dataset.iter()
    }

    /// Get the arithmetic average value of x
    pub fn mean_x(&self) -> f64 {
        let mut sum: f64 = 0.0;
        for value in self.iter() {
            sum += value.x_value;
        }
        sum / self.len() as f64
    }

    /// Get the arithmetic average value of y
    pub fn mean_y(&self) -> f64 {
        let mut sum: f64 = 0.0;
        for value in self.iter() {
            sum += value.y_value;
        }
        sum / self.len() as f64
    }

    /// Get all x values as a vector
    pub fn get_x_value(&self) -> Vec<f64> {
        self.iter().map(|item| item.x_value).collect()
    }

    /// Get all x uncertainties as a vector
    pub fn get_x_uncertainty(&self) -> Vec<f64> {
        self.iter().map(|item| item.x_uncertainty).collect()
    }

    /// Get all y values as a vector
    pub fn get_y_value(&self) -> Vec<f64> {
        self.iter().map(|item| item.y_value).collect()
    }

    /// Get all y uncertainties as a vector
    pub fn get_y_uncertainty(&self) -> Vec<f64> {
        self.iter().map(|item| item.y_uncertainty).collect()
    }

    // Get the maximum x value
    raw_defun_minmax! {max_x, max_by, x_value, x_uncertainty, +}

    // Get the minimum x value
    raw_defun_minmax! {min_x, min_by, x_value, x_uncertainty, -}

    // Get the maximum y value
    raw_defun_minmax! {max_y, max_by, y_value, y_uncertainty, +}

    // Get the minimum y value
    raw_defun_minmax! {min_y, min_by, y_value, y_uncertainty, -}

    /// Get line of best fit
    pub fn line_best_fit(&self) -> Line {
        let ax = self.mean_x();
        let ay = self.mean_y();
        let mut numerator: f64 = 0.0;
        let mut denominator: f64 = 0.0;
        for data in self.iter() {
            let x = data.x_value;
            let y = data.y_value;
            numerator += (x - ax) * (y - ay);
            denominator += (x - ax).powi(2);
        }
        let b = numerator / denominator;
        let a = ay - b * ax;
        Line {
            gradient: b,
            y_intercept: a,
        }
    }

    /// Permute all possible lines by connecting the ends
    fn lines(&self) -> Vec<Line> {
        let firstx = self[0].x_value;
        let ufirstx = self[0].x_uncertainty;
        let firsty = self[0].y_value;
        let ufirsty = self[0].y_uncertainty;
        let last_idx = self.len() - 1;
        let lastx = self[last_idx].x_value;
        let ulastx = self[last_idx].x_uncertainty;
        let lasty = self[last_idx].y_value;
        let ulasty = self[last_idx].y_uncertainty;
        let firstpoints = vec![
            Point {
                x: firstx + ufirstx,
                y: firsty + ufirsty,
            },
            Point {
                x: firstx + ufirstx,
                y: firsty - ufirsty,
            },
            Point {
                x: firstx - ufirstx,
                y: firsty + ufirsty,
            },
            Point {
                x: firstx - ufirstx,
                y: firsty - ufirsty,
            },
        ];
        let lastpoints = vec![
            Point {
                x: lastx + ulastx,
                y: lasty + ulasty,
            },
            Point {
                x: lastx + ulastx,
                y: lasty - ulasty,
            },
            Point {
                x: lastx - ulastx,
                y: lasty + ulasty,
            },
            Point {
                x: lastx - ulastx,
                y: lasty - ulasty,
            },
        ];
        firstpoints
            .iter()
            .map(|&first_point| {
                lastpoints
                    .iter()
                    .map(|&last_point| Line::from_points(first_point, last_point))
                    .collect()
            })
            .collect::<Vec<Vec<Line>>>()
            .concat()
    }

    /// Get the maximum gradient line
    pub fn line_max_grad(&self) -> Line {
        let lns = self.lines();
        *lns.iter()
            .max_by(|one, another| one.gradient.partial_cmp(&another.gradient).unwrap())
            .unwrap()
    }

    // Get the minimum gradient line
    pub fn line_min_grad(&self) -> Line {
        let lns = self.lines();
        *lns.iter()
            .min_by(|one, another| one.gradient.partial_cmp(&another.gradient).unwrap())
            .unwrap()
    }
}

/// Convert string to float number, returning a tuple of
/// the number and a tuple of the index of the first digit and the first non-digit,
/// or None if no number found
fn atof(string: &str) -> Option<(f64, (usize, usize))> {
    // Resulting number without decimal point
    let mut result: i32 = 0;
    // Whether decimal point is met
    let mut fracpart = false;
    // -1 for a negative number
    let mut sign: f64 = 1.0;
    // Numbers of fractional digits
    let mut fracdig = 0;
    // Index of the first number
    let mut startpoint: Option<usize> = None;
    // Index of the first non-number
    let mut endpoint: usize = string.len();
    // Go over every character
    for (idx, chr) in string.chars().enumerate() {
        // Found a digit
        if chr.is_digit(10) {
            if startpoint == None {
                // Set processing start point
                startpoint = if idx != 0 && Some('-') == string.chars().nth(idx - 1) {
                    sign = -1.0;
                    Some(idx - 1)
                } else {
                    Some(idx)
                }
            }
            result = result * 10 + chr.to_digit(10).unwrap() as i32;
            if fracpart {
                fracdig += 1;
            }
        } else if chr == '.' && !fracpart {
            // Start of decimal point
            fracpart = true;
        } else if startpoint != None {
            // Processing has started, now end it
            endpoint = idx;
            break;
        }
    }
    if startpoint == None {
        // Still no digits
        None
    } else {
        Some((
            result as f64 / 10_i32.pow(fracdig) as f64 * sign,
            (startpoint.unwrap(), endpoint),
        ))
    }
}
