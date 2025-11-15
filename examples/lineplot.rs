use ndarray::Array;
use plotlars::{Axis, Line, LinePlot, Plot, Rgb, Text, TickDirection};
use polars::prelude::*;

fn main() {
    let x_values = Array::linspace(0.0, 2.0 * std::f64::consts::PI, 1000).to_vec();
    let sine_values = x_values
        .iter()
        .map(|arg0: &f64| f64::sin(*arg0))
        .collect::<Vec<_>>();
    let cosine_values = x_values
        .iter()
        .map(|arg0: &f64| f64::cos(*arg0))
        .collect::<Vec<_>>();

    let dataset = df![
        "x" => &x_values,
        "sine" => &sine_values,
        "cosine" => &cosine_values,
    ]
    .unwrap();

    LinePlot::builder()
        .data(&dataset)
        .x("x")
        .y("sine")
        .additional_lines(vec!["cosine"])
        .colors(vec![Rgb(255, 0, 0), Rgb(0, 255, 0)])
        .lines(vec![Line::Solid, Line::Dot])
        .width(3.0)
        .with_shape(false)
        .plot_title(Text::from("Line Plot").font("Arial").size(18))
        .x_axis(
            &Axis::new()
                .tick_direction(TickDirection::OutSide)
                .axis_position(0.5)
                .tick_values(vec![
                    0.5 * std::f64::consts::PI,
                    std::f64::consts::PI,
                    1.5 * std::f64::consts::PI,
                    2.0 * std::f64::consts::PI,
                ])
                .tick_labels(vec!["π/2", "π", "3π/2", "2π"]),
        )
        .y_axis(
            &Axis::new()
                .tick_direction(TickDirection::OutSide)
                .tick_values(vec![-1.0, 0.0, 1.0])
                .tick_labels(vec!["-1", "0", "1"]),
        )
        .build()
        .plot();
}
