use plotlars::{ColorBar, Palette, Plot, SurfacePlot, Text};
use polars::prelude::*;
use std::iter;

fn main() {
    let n: usize = 100;
    let x_base: Vec<f64> = (0..n)
        .map(|i| {
            let step = (10.0 - (-10.0)) / (n - 1) as f64;
            -10.0 + step * i as f64
        })
        .collect();
    let y_base: Vec<f64> = (0..n)
        .map(|i| {
            let step = (10.0 - (-10.0)) / (n - 1) as f64;
            -10.0 + step * i as f64
        })
        .collect();

    let x = x_base
        .iter()
        .flat_map(|&xi| iter::repeat_n(xi, n))
        .collect::<Vec<_>>();

    let y = y_base
        .iter()
        .cycle()
        .take(n * n)
        .cloned()
        .collect::<Vec<_>>();

    let z = x_base
        .iter()
        .flat_map(|i| {
            y_base
                .iter()
                .map(|j| 1.0 / (j * j + 5.0) * j.sin() + 1.0 / (i * i + 5.0) * i.cos())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let dataset = df![
        "x" => &x,
        "y" => &y,
        "z" => &z,
    ]
    .unwrap();

    SurfacePlot::builder()
        .data(&dataset)
        .x("x")
        .y("y")
        .z("z")
        .plot_title(Text::from("Surface Plot").font("Arial").size(18))
        .color_bar(&ColorBar::new().border_width(1))
        .color_scale(Palette::Cividis)
        .reverse_scale(true)
        .opacity(0.5)
        .build()
        .plot();
}
