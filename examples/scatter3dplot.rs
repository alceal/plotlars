use plotlars::{Legend, Plot, Rgb, Scatter3dPlot, Shape};
use polars::prelude::*;

fn main() {
    let dataset = LazyCsvReader::new(PlPath::new("data/penguins.csv"))
        .finish()
        .unwrap()
        .select([
            col("species"),
            col("sex").alias("gender"),
            col("bill_length_mm").cast(DataType::Float32),
            col("flipper_length_mm").cast(DataType::Int16),
            col("body_mass_g").cast(DataType::Int16),
        ])
        .collect()
        .unwrap();

    Scatter3dPlot::builder()
        .data(&dataset)
        .x("body_mass_g")
        .y("flipper_length_mm")
        .z("bill_length_mm")
        .group("species")
        .opacity(0.25)
        .size(8)
        .colors(vec![Rgb(178, 34, 34), Rgb(65, 105, 225), Rgb(255, 140, 0)])
        .shapes(vec![Shape::Circle, Shape::Square, Shape::Diamond])
        .plot_title("Scatter 3D Plot")
        .legend(&Legend::new().x(0.6))
        .build()
        .plot();
}
