use polars::prelude::*;

use plotlars::{Coloring, ContourPlot, Palette, Plot, Text};

fn main() {
    let dataset = df!(
            "x" => &[0.0, 0.0, 0.0, 2.5, 2.5, 2.5, 5.0, 5.0, 5.0],
            "y" => &[0.0, 7.5, 15.0, 0.0, 7.5, 15.0, 0.0, 7.5, 15.0],
            "z" => &[0.0, 5.0, 10.0, 5.0, 2.5, 5.0, 10.0, 0.0, 0.0],
        )
        .unwrap();

    ContourPlot::builder()
        .data(&dataset)
        .x("x")
        .y("y")
        .z("z")
        .color_scale(Palette::Viridis)
        .reverse_scale(true)
        .coloring(Coloring::Fill)
        .show_lines(false)
        .plot_title(
            Text::from("Contour Plot")
                .font("Arial")
                .size(18)
        )
        .build()
        .plot();
}
