use plotlars::{LinePlot, Plot, Rgb};
use polars::prelude::*;

fn main() {
    let x: Vec<f64> = (0..500)
        .map(|i| i as f64 * 2.0 * std::f64::consts::PI / 499.0)
        .collect();
    let sine: Vec<f64> = x.iter().map(|v| v.sin()).collect();
    let cosine: Vec<f64> = x.iter().map(|v| v.cos()).collect();

    let dataset = df!["x" => &x, "sine" => &sine, "cosine" => &cosine].unwrap();

    LinePlot::builder()
        .data(&dataset)
        .x("x")
        .y("sine")
        .additional_lines(vec!["cosine"])
        .colors(vec![Rgb(255, 0, 0), Rgb(0, 0, 255)])
        .plot_title("Line Plot")
        .x_title("x")
        .y_title("y")
        .build()
        .plot();
}
