use plotlars::{ColorBar, HeatMap, Palette, Plot, Text, ValueExponent};
use polars::prelude::*;

fn main() {
    let dataset = LazyCsvReader::new(PlPath::new("data/heatmap.csv"))
        .finish()
        .unwrap()
        .collect()
        .unwrap();

    HeatMap::builder()
        .data(&dataset)
        .x("x")
        .y("y")
        .z("z")
        .color_bar(
            &ColorBar::new()
                .length(0.7)
                .value_exponent(ValueExponent::None)
                .separate_thousands(true)
                .tick_length(5)
                .tick_step(2500.0),
        )
        .plot_title(Text::from("Heat Map").font("Arial").size(18))
        .color_scale(Palette::Viridis)
        .build()
        .plot();
}
