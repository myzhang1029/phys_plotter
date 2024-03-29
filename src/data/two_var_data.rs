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

use crate::data::{Line, Point};
use std::fs::File;
use std::io::prelude::*;
use std::io::{Error, ErrorKind};
use std::ops::{Deref, DerefMut};
use std::path::Path;

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
    ($name: ident, $cmp: ident, $val_name: ident, $uncer_name: ident, $uncer_sign: tt, $default: expr) => {
        /// Get the $name value. if `with_uncertainty` is true, the uncertainties is also taken into account
        #[must_use]
        pub fn $name(&self, with_uncertainty: bool) -> f64 {
            // Error check
            if self.is_empty() {
                return 0.0
            }
            // The unwraps are safe because empty is handled
            if with_uncertainty {
                let k = self.iter()
                    .$cmp(|one, another|
                        (one.$val_name $uncer_sign one.$uncer_name)
                            .partial_cmp(&(another.$val_name $uncer_sign another.$uncer_name))
                            .unwrap_or($default)
                    )
                    .unwrap();
                k.$val_name $uncer_sign k.$uncer_name
            } else {
                self.iter()
                    .$cmp(|one, another|
                        one.$val_name
                            .partial_cmp(&another.$val_name)
                            .unwrap_or($default)
                    )
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
        let mut line = line.to_string();
        // Exhaust this line by taking all numeric fields
        while let Some((number, (_, end_point))) = atof(&line) {
            fields.push(number);
            line = line.chars().skip(end_point).collect::<String>();
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
pub struct TwoVarDataSet(Vec<TwoVarDataPoint>);

impl Deref for TwoVarDataSet {
    type Target = Vec<TwoVarDataPoint>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for TwoVarDataSet {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl TwoVarDataSet {
    /// Parse a data file
    /// filename: Path to the file
    /// dux: Default x uncertainty
    /// duy: Default y uncertainty
    pub fn from_file<P: AsRef<Path>>(filename: P, dux: f64, duy: f64) -> Result<Self, Error> {
        // Read the data file
        let mut data_file = File::open(filename)?;
        let mut contents = String::new();
        data_file.read_to_string(&mut contents)?;
        Self::from_string(&contents, dux, duy)
    }

    /// Parse a data string
    /// buf: data string
    /// dux: Default x uncertainty
    /// duy: Default y uncertainty
    pub fn from_string(buf: &str, dux: f64, duy: f64) -> Result<Self, Error> {
        // Split into lines
        let lines: Vec<&str> = buf.split('\n').collect();
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
        Ok(Self(result))
    }

    /// Get the arithmetic average value of x
    #[must_use]
    pub fn mean_x(&self) -> f64 {
        let mut sum: f64 = 0.0;
        for value in self.iter() {
            sum += value.x_value;
        }
        sum / self.len() as f64
    }

    /// Get the arithmetic average value of y
    #[must_use]
    pub fn mean_y(&self) -> f64 {
        let mut sum: f64 = 0.0;
        for value in self.iter() {
            sum += value.y_value;
        }
        sum / self.len() as f64
    }

    /// Get all x values as a vector
    #[must_use]
    pub fn get_x_value(&self) -> Vec<f64> {
        self.iter().map(|item| item.x_value).collect()
    }

    /// Get all x uncertainties as a vector
    #[must_use]
    pub fn get_x_uncertainty(&self) -> Vec<f64> {
        self.iter().map(|item| item.x_uncertainty).collect()
    }

    /// Get all y values as a vector
    #[must_use]
    pub fn get_y_value(&self) -> Vec<f64> {
        self.iter().map(|item| item.y_value).collect()
    }

    /// Get all y uncertainties as a vector
    #[must_use]
    pub fn get_y_uncertainty(&self) -> Vec<f64> {
        self.iter().map(|item| item.y_uncertainty).collect()
    }

    // All default to false
    // Get the maximum x value
    raw_defun_minmax! {max_x, max_by, x_value, x_uncertainty, +, std::cmp::Ordering::Less}

    // Get the minimum x value
    raw_defun_minmax! {min_x, min_by, x_value, x_uncertainty, -, std::cmp::Ordering::Greater}

    // Get the maximum y value
    raw_defun_minmax! {max_y, max_by, y_value, y_uncertainty, +, std::cmp::Ordering::Less}

    // Get the minimum y value
    raw_defun_minmax! {min_y, min_by, y_value, y_uncertainty, -, std::cmp::Ordering::Greater}

    /// Get line of best fit
    #[must_use]
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
    fn lines(&self) -> Option<Vec<Line>> {
        // Error check
        if self.is_empty() {
            return None;
        }
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
        Some(
            firstpoints
                .iter()
                .map(|&first_point| {
                    lastpoints
                        .iter()
                        .map(|&last_point| Line::from_points(first_point, last_point))
                        .collect()
                })
                .collect::<Vec<Vec<Line>>>()
                .concat(),
        )
    }

    /// Get the maximum gradient line
    #[must_use]
    pub fn line_max_grad(&self) -> Option<Line> {
        let lns = self.lines()?;
        Some(*lns.iter().max_by(|one, another| {
            one.gradient
                .partial_cmp(&another.gradient)
                .unwrap_or(std::cmp::Ordering::Less)
        })?)
    }

    // Get the minimum gradient line
    #[must_use]
    pub fn line_min_grad(&self) -> Option<Line> {
        let lns = self.lines()?;
        Some(*lns.iter().min_by(|one, another| {
            one.gradient
                .partial_cmp(&another.gradient)
                .unwrap_or(std::cmp::Ordering::Greater)
        })?)
    }
}

/// Convert string to float number, returning a tuple of
/// the number and a tuple of the index of the first digit and the first non-digit,
/// or None if no number found
fn atof(string: &str) -> Option<(f64, (usize, usize))> {
    // Resulting number without decimal point
    let mut result: f64 = 0.0;
    // -1 for a negative number
    let mut sign: f64 = 1.0;
    // Numbers of fractional digits processed (or to be processed)
    // Negative as it's 10's power
    let mut fracdig: i32 = 0;
    // Index of the first number
    let mut startpoint: Option<usize> = None;
    // Index of the first non-number
    let mut endpoint: usize = string.len();
    // Go over every character
    for (idx, chr) in string.chars().enumerate() {
        // Found a digit
        if chr.is_ascii_digit() {
            if startpoint == None {
                // Set processing start point
                startpoint = if idx != 0 && Some('-') == string.chars().nth(idx - 1) {
                    sign = -1.0;
                    Some(idx - 1)
                } else {
                    Some(idx)
                }
            }
            if fracdig == 0 {
                result = result * 10.0 + f64::from(chr.to_digit(10).unwrap());
            } else {
                /* We cannot use the multiply-shift method as it may cause overflow */
                result += f64::from(chr.to_digit(10).unwrap()) * 10_f64.powi(fracdig);
                fracdig -= 1;
            }
        } else if chr == '.' && fracdig == 0 {
            // Start of decimal point
            fracdig = -1;
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
        Some((result * sign, (startpoint.unwrap(), endpoint)))
    }
}
