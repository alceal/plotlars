use polars::prelude::*;

use plotlars::{Axis, Legend, Plot, Rgb, ScatterPlot, Shape, Text, TickDirection};

fn main() {
    let dataset = LazyCsvReader::new(PlPath::new("data/penguins.csv"))
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
        .tick_direction(TickDirection::OutSide)
        .value_thousands(true);

    ScatterPlot::builder()
        .data(&dataset)
        .x("body_mass_g")
        .y("flipper_length_mm")
        .group("species")
        .opacity(0.5)
        .size(12)
        .colors(vec![
            Rgb(178, 34, 34),
            Rgb(65, 105, 225),
            Rgb(255, 140, 0),
        ])
        .shapes(vec![
            Shape::Circle,
            Shape::Square,
            Shape::Diamond,
        ])
        .plot_title(
            Text::from("Scatter Plot")
                .font("Arial")
                .size(20)
                .x(0.065)
        )
        .x_title("body mass (g)")
        .y_title("flipper length (mm)")
        .legend_title("species")
        .x_axis(
            &axis.clone()
                .value_range(vec![2500.0, 6500.0])
        )
        .y_axis(
            &axis.clone()
                .value_range(vec![170.0, 240.0])
        )
        .legend(
            &Legend::new()
                .x(0.85)
                .y(0.15)
        )
        .build()
        .plot();
}
