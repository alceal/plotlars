use plotlars::{HeatMap, Palette, Plot};
use polars::prelude::*;

fn main() {
    let dataset = LazyCsvReader::new(PlRefPath::new("data/heatmap.csv"))
        .finish()
        .unwrap()
        .collect()
        .unwrap();

    HeatMap::builder()
        .data(&dataset)
        .x("x")
        .y("y")
        .z("z")
        .color_scale(Palette::Viridis)
        .plot_title("Heat Map")
        .x_title("X")
        .y_title("Y")
        .build()
        .plot();
}
