use plotlars::polars::prelude::*;
use plotlars::{CsvReader, Plot, Rgb, ScatterPlot};

fn main() {
    let dataset = CsvReader::new("data/penguins.csv")
        .finish()
        .unwrap()
        .lazy()
        .select([
            col("species"),
            col("flipper_length_mm").cast(DataType::Int16),
            col("body_mass_g").cast(DataType::Int16),
        ])
        .collect()
        .unwrap();

    ScatterPlot::builder()
        .data(&dataset)
        .x("body_mass_g")
        .y("flipper_length_mm")
        .group("species")
        .opacity(0.5)
        .size(10)
        .colors(vec![Rgb(178, 34, 34), Rgb(65, 105, 225), Rgb(255, 140, 0)])
        .plot_title("Scatter Plot")
        .x_title("body mass (g)")
        .y_title("flipper length (mm)")
        .legend_title("species")
        .build()
        .plot();
}
