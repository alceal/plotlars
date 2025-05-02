use polars::prelude::*;

use plotlars::{BarPlot, Legend, Orientation, Plot, Rgb, Text};

fn main() {
    let dataset = df![
            "animal" => &["giraffe", "giraffe", "orangutan", "orangutan", "monkey", "monkey"],
            "gender" => &vec!["female", "male", "female", "male", "female", "male"],
            "value" => &vec![20.0f32, 25.0, 14.0, 18.0, 23.0, 31.0],
            "error" => &vec![1.0, 0.5, 1.5, 1.0, 0.5, 1.5],
        ]
        .unwrap();

    BarPlot::builder()
        .data(&dataset)
        .labels("animal")
        .values("value")
        .orientation(Orientation::Vertical)
        .group("gender")
        .error("error")
        .colors(vec![
            Rgb(255, 127, 80),
            Rgb(64, 224, 208),
        ])
        .plot_title(
            Text::from("Bar Plot")
                .font("Arial")
                .size(18)
        )
        .x_title(
            Text::from("animal")
                .font("Arial")
                .size(15)
        )
        .y_title(
            Text::from("value")
                .font("Arial")
                .size(15)
        )
        .legend_title(
            Text::from("gender")
                .font("Arial")
                .size(15)
        )
        .legend(
            &Legend::new()
                .orientation(Orientation::Horizontal)
                .y(1.0)
                .x(0.4)
        )
        .build()
        .plot();
}
