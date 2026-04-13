use plotlars::polars::prelude::*;
use plotlars::{CsvReader, Histogram, Plot, Rgb};

fn main() {
    let dataset = CsvReader::new("data/penguins.csv")
        .finish()
        .unwrap()
        .lazy()
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
