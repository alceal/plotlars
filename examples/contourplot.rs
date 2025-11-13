use plotlars::{Coloring, ContourPlot, Palette, Plot, Text};
use polars::prelude::*;

fn main() {
    let dataset = LazyCsvReader::new(PlPath::new("data/contour_surface.csv"))
        .finish()
        .unwrap()
        .collect()
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
        .plot_title(Text::from("Contour Plot").font("Arial").size(18))
        .build()
        .plot();
}
