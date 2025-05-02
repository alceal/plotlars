use polars::prelude::*;

use plotlars::{Axis, Histogram, Legend, Plot, Rgb, Text, TickDirection};

fn main() {
    let dataset = LazyCsvReader::new("data/penguins.csv")
        .finish()
        .unwrap()
        .select([
            col("species"),
            col("sex").alias("gender"),
            col("flipper_length_mm").cast(DataType::Int16),
            col("body_mass_g").cast(DataType::Int16),
        ])
        .collect()
        .unwrap();

    let axis = Axis::new()
        .show_line(true)
        .show_grid(true)
        .value_thousands(true)
        .tick_direction(TickDirection::OutSide);

    Histogram::builder()
        .data(&dataset)
        .x("body_mass_g")
        .group("species")
        .opacity(0.5)
        .colors(vec![
            Rgb(255, 165, 0),
            Rgb(147, 112, 219),
            Rgb(46, 139, 87),
        ])
        .plot_title(
            Text::from("Histogram")
                .font("Arial")
                .size(18)
        )
        .x_title(
            Text::from("body mass (g)")
                .font("Arial")
                .size(15)
        )
        .y_title(
            Text::from("count")
                .font("Arial")
                .size(15)
        )
        .legend_title(
            Text::from("species")
                .font("Arial")
                .size(15)
        )
        .x_axis(&axis)
        .y_axis(&axis)
        .legend(
            &Legend::new()
                .x(0.9)
        )
        .build()
        .plot();
}
