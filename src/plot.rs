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

use crate::two_var_data::TwoVarDataSet;
use gnuplot::{Auto, AxesCommon, Caption, Color, Dash, Figure, Font, Graph, LineStyle, LineWidth};

pub fn plot(title: &str, x_label: &str, y_label: &str, data: TwoVarDataSet) {
    // Extra length before min and after max
    let extra = (data.max_x() - data.min_x()) * 0.1;
    // Two points for plotting the lines
    let ln_plt_x = Vec::from([data.min_x() - extra, data.max_x() + extra]);
    // Three lines
    let line_best_fit = data.line_best_fit();
    let line_min_grad = data.line_min_grad();
    let line_max_grad = data.line_max_grad();
    let y_best: Vec<f64> = ln_plt_x.iter().map(|x| line_best_fit.y(*x)).collect();
    let y_min: Vec<f64> = ln_plt_x.iter().map(|x| line_min_grad.y(*x)).collect();
    let y_max: Vec<f64> = ln_plt_x.iter().map(|x| line_max_grad.y(*x)).collect();
    let x_values = data.get_x_value();
    let y_values = data.get_y_value();
    let mut fg = Figure::new();
    fg.axes2d()
        .set_title(title, &[Font("Times", 22.0)])
        .set_x_label(x_label, &[Font("Times", 13.0)])
        .set_y_label(y_label, &[Font("Times", 13.0)])
        // Automatically generate ticks
        .set_x_ticks(Some((Auto, 1)), &[], &[Font("Times", 13.0)])
        .set_y_ticks(Some((Auto, 1)), &[], &[Font("Times", 13.0)])
        // Scatter points "Real data"
        //.points(&x_values, &y_values, &[Color("#4477AA"), PointSymbol('.')])
        // Plot error bars
        .x_error_bars(
            &x_values,
            &y_values,
            &data.get_x_uncertainty(),
            &[LineWidth(1.5), Color("#4477AA")],
        )
        .y_error_bars(
            &x_values,
            &y_values,
            &data.get_y_uncertainty(),
            &[LineWidth(1.5), Color("#4477AA")],
        )
        // Three required lines
        .lines(
            &ln_plt_x,
            &y_best,
            &[
                Caption(format!("Best fit {}", line_best_fit).as_str()),
                LineWidth(2.0),
                Color("#EE7733"),
            ],
        )
        .lines(
            &ln_plt_x,
            &y_min,
            &[
                Caption(format!("Minimum gradient {}", line_min_grad).as_str()),
                LineStyle(Dash),
                LineWidth(1.5),
                Color("#009988"),
            ],
        )
        .lines(
            &ln_plt_x,
            &y_max,
            &[
                Caption(format!("Maximum gradient {}", line_max_grad).as_str()),
                LineStyle(Dash),
                LineWidth(1.5),
                Color("#0077BB"),
            ],
        )
        .set_legend(Graph(0.99), Graph(0.95), &[], &[Font("Times", 13.0)]);
    fg.show().unwrap();
}
