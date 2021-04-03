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

use crate::data::TwoVarDataSet;
use plotters::prelude::*;
use plotters::style::RGBColor;

macro_rules! line_best_fit_style {
    () => {
        ShapeStyle {
            color: RGBColor(238, 119, 51).to_rgba(),
            filled: true,
            stroke_width: 2,
        }
    };
}

macro_rules! line_min_grad_style {
    () => {
        ShapeStyle {
            color: RGBColor(0, 153, 136).to_rgba(),
            filled: true,
            stroke_width: 1,
        }
    };
}

macro_rules! line_max_grad_style {
    () => {
        ShapeStyle {
            color: RGBColor(0, 119, 187).to_rgba(),
            filled: true,
            stroke_width: 1,
        }
    };
}

/// Generic plotter for all kinds of backends.
/// WARNING: Cannot proceed with empty values
pub fn plot_plotters<ET: std::error::Error + Send + Sync, T: DrawingBackend<ErrorType = ET>>(
    title: &str,
    x_label: &str,
    y_label: &str,
    data: &TwoVarDataSet,
    backend: T,
) -> Result<(), plotters::drawing::DrawingAreaErrorKind<ET>> {
    // Those generic type parameters are so dreadful
    // Extra length before min and after max
    let extrax = (data.max_x(false) - data.min_x(false)) * 0.1;
    let extray = (data.max_y(false) - data.min_y(false)) * 0.1;
    // Axis ranges
    let axis_x = (data.min_x(true) - extrax)..(data.max_x(true) + extrax);
    let axis_y = (data.min_y(true) - extray)..(data.max_y(true) + extray);
    // Points for plotting the lines
    let plot_x = Vec::from([data.min_x(true) - extrax, data.max_x(true) + extrax]);
    // Create drawing area
    let root_drawing_area = backend.into_drawing_area();
    root_drawing_area.fill(&WHITE)?;
    let mut ctx = ChartBuilder::on(&root_drawing_area)
        .margin(5)
        .caption(title, ("Times", 22))
        .set_label_area_size(
            LabelAreaPosition::Left,
            (16.0 * data.max_y(false).log10()) as u32,
        )
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .build_cartesian_2d(axis_x, axis_y)?;
    ctx.configure_mesh()
        .disable_mesh()
        .x_desc(x_label)
        .y_desc(y_label)
        .axis_desc_style(("Times", 13))
        .draw()?;
    // Three lines
    let line_best_fit = data.line_best_fit();
    ctx.draw_series(LineSeries::new(
        plot_x.iter().map(|x| (*x, line_best_fit.y(*x))),
        line_best_fit_style!(),
    ))?
    .label(line_best_fit_name!(line_best_fit).as_str())
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], line_best_fit_style!()));
    if let Some(line_min_grad) = data.line_min_grad() {
        ctx.draw_series(LineSeries::new(
            plot_x.iter().map(|x| (*x, line_min_grad.y(*x))),
            line_min_grad_style!(),
        ))?
        .label(line_min_grad_name!(line_min_grad).as_str())
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], line_min_grad_style!()));
    }
    if let Some(line_max_grad) = data.line_max_grad() {
        ctx.draw_series(LineSeries::new(
            plot_x.iter().map(|x| (*x, line_max_grad.y(*x))),
            line_max_grad_style!(),
        ))?
        .label(line_max_grad_name!(line_max_grad).as_str())
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], line_max_grad_style!()));
    }
    // Scatter series and uncertainties
    /*ctx.draw_series(
        data.iter().map(|point|
            Circle::new((point.x_value, point.y_value), 5, &BLUE)
        ),
    )?;*/
    ctx.draw_series(data.iter().map(|point| {
        ErrorBar::new_vertical(
            point.x_value,
            point.y_value - point.y_uncertainty,
            point.y_value,
            point.y_value + point.y_uncertainty,
            &BLUE,
            10,
        )
    }))?;
    ctx.draw_series(data.iter().map(|point| {
        ErrorBar::new_horizontal(
            point.y_value,
            point.x_value - point.x_uncertainty,
            point.x_value,
            point.x_value + point.x_uncertainty,
            &BLUE,
            10,
        )
    }))?;
    ctx.configure_series_labels().border_style(&BLACK).draw()?;
    Ok(())
}
