use plotlars::{BoxPlot, Plot, Rgb};
use polars::prelude::*;

fn main() {
    let dataset = LazyCsvReader::new(PlRefPath::new("data/penguins.csv"))
        .finish()
        .unwrap()
        .select([col("species"), col("body_mass_g").cast(DataType::Float64)])
        .collect()
        .unwrap();

    BoxPlot::builder()
        .data(&dataset)
        .labels("species")
        .values("body_mass_g")
        .group("species")
        .colors(vec![Rgb(178, 34, 34), Rgb(65, 105, 225), Rgb(255, 140, 0)])
        .plot_title("Box Plot")
        .x_title("species")
        .y_title("body mass (g)")
        .build()
        .plot();
}
