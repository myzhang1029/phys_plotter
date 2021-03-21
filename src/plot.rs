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

use crate::two_var_data::TwoVarDataSet;
use gnuplot::{
    AlignCenter, AlignLeft, AlignRight, AxesCommon, Bottom, Caption, Figure, Auto, Graph,
    Left, LineWidth, Mirror, Placement, TextAlign, LineStyle, Dash, Color, PointSymbol
};

pub fn plot(title: &str, x_label: &str, y_label: &str, data: TwoVarDataSet) {
    let ln_plt_x = Vec::from([data.min_x(), data.max_x()]);
    let y_best: Vec<f64> = ln_plt_x.iter().map(|x| data.line_best_fit().y(*x)).collect();
    let y_min: Vec<f64> = ln_plt_x.iter().map(|x| data.line_min_grad().y(*x)).collect();
    let y_max: Vec<f64> = ln_plt_x.iter().map(|x| data.line_max_grad().y(*x)).collect();
    let x_values = data.get_x_value();
    let y_values = data.get_y_value();
    let mut fg = Figure::new();
    fg.axes2d()
        .set_title(title, &[])
        .set_x_label(x_label, &[])
        .set_y_label(y_label, &[])
        // Automatically generate ticks
        .set_x_ticks(Some((Auto, 1)), &[Mirror(false)], &[])
        .set_y_ticks(Some((Auto, 1)), &[Mirror(false)], &[])
        .set_border(true, &[Left, Bottom], &[LineWidth(2.0)])
        // Scatter points "Real data"
		.points(
			&x_values,
			&y_values,
			&[Color("blue"), PointSymbol('x')],
		)
        // Plot error bars
        .x_error_bars(
			&x_values,
            &y_values,
			&data.get_x_uncertainty(),
			&[LineWidth(0.9), Color("blue"), PointSymbol('.')],
		)
        .y_error_bars(
			&x_values,
            &y_values,
			&data.get_y_uncertainty(),
			&[LineWidth(0.9), Color("blue"), PointSymbol('.')],
		)
        // Three required lines
        .lines(
            &ln_plt_x,
            &y_best,
            &[Caption("Best-fit Line")],
        )
        .lines(
            &ln_plt_x,
            &y_min,
            &[Caption("Minimum Gradient"), LineStyle(Dash), LineWidth(0.7)],
        )
        .lines(
            &ln_plt_x,
            &y_max,
            &[Caption("Maximum Gradient"), LineStyle(Dash), LineWidth(0.7)],
        )
        .set_legend(Graph(1.0), Graph(0.7), &[], &[]);
    fg.show();
}
