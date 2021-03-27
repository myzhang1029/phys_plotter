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

use std::fmt;

/// Struct representing a simple point
#[derive(Debug, Default, Copy, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

/// Struct representing a straight line
#[derive(Debug, Default, Copy, Clone)]
pub struct Line {
    pub gradient: f64,
    pub y_intercept: f64,
}

impl Line {
    /// Construct a line from two points
    pub fn from_points(first: Point, last: Point) -> Self {
        let dx = last.x - first.x;
        let dy = last.y - first.y;
        let b = dy / dx;
        // gradient can be NaN and y-intercept can be Inf
        Line {
            gradient: b,
            y_intercept: last.y - b * last.x,
        }
    }

    /// y value of the x
    pub fn y(&self, x: f64) -> f64 {
        self.gradient * x + self.y_intercept
    }
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Decimal precision in usize
        let precision = f.precision().unwrap_or(6_usize);
        // Minimum shown resolution
        let epsilon = 10.0_f64.powi(-(precision as i32));
        let y_intercept = if self.y_intercept >= epsilon {
            format!("+{:.*}", precision, self.y_intercept)
        } else if self.y_intercept <= -epsilon {
            format!("{:.*}", precision, self.y_intercept)
        } else {
            String::from("")
        };
        write!(f, "y = {:.*}x{}", precision, self.gradient, y_intercept)
    }
}
