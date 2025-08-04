use polars::prelude::*;

use plotlars::{Axis, BoxPlot, Legend, Orientation, Plot, Rgb, Text};

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

    BoxPlot::builder()
        .data(&dataset)
        .labels("species")
        .values("body_mass_g")
        .orientation(Orientation::Vertical)
        .group("gender")
        .box_points(true)
        .point_offset(-1.5)
        .jitter(0.01)
        .opacity(0.1)
        .colors(vec![
            Rgb(0, 191, 255),
            Rgb(57, 255, 20),
            Rgb(255, 105, 180),
        ])
        .plot_title(
            Text::from("Box Plot")
                .font("Arial")
                .size(18)
        )
        .x_title(
            Text::from("species")
                .font("Arial")
                .size(15)
        )
        .y_title(
            Text::from("body mass (g)")
                .font("Arial")
                .size(15)
        )
        .legend_title(
            Text::from("gender")
                .font("Arial")
                .size(15)
        )
        .y_axis(
            &Axis::new()
                .value_thousands(true)
        )
        .legend(
            &Legend::new()
                .border_width(1)
                .x(0.9)
        )
        .build()
        .plot();
}
