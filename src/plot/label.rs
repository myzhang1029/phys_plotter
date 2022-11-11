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

macro_rules! line_best_fit_name {
    ($line_var: ident) => {
        format!("Best fit {}", $line_var)
    };
}

macro_rules! line_min_grad_name {
    ($line_var: ident) => {
        format!("Minimum gradient {}", $line_var)
    };
}

macro_rules! line_max_grad_name {
    ($line_var: ident) => {
        format!("Maximum gradient {}", $line_var)
    };
}
