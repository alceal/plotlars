use plotlars::{Histogram, Plot, Rgb};
use polars::prelude::*;

fn main() {
    let dataset = LazyCsvReader::new(PlRefPath::new("data/penguins.csv"))
        .finish()
        .unwrap()
        .select([col("species"), col("body_mass_g").cast(DataType::Int16)])
        .collect()
        .unwrap();

    Histogram::builder()
        .data(&dataset)
        .x("body_mass_g")
        .group("species")
        .opacity(0.5)
        .colors(vec![Rgb(255, 165, 0), Rgb(147, 112, 219), Rgb(46, 139, 87)])
        .plot_title("Histogram")
        .x_title("body mass (g)")
        .y_title("count")
        .legend_title("species")
        .build()
        .plot();
}
