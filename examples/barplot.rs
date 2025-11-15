use plotlars::{BarPlot, Legend, Orientation, Plot, Rgb, Text};
use polars::prelude::*;

fn main() {
    let dataset = LazyCsvReader::new(PlPath::new("data/animal_statistics.csv"))
        .finish()
        .unwrap()
        .collect()
        .unwrap();

    BarPlot::builder()
        .data(&dataset)
        .labels("animal")
        .values("value")
        .orientation(Orientation::Vertical)
        .group("gender")
        .sort_groups_by(|a, b| a.len().cmp(&b.len()))
        .error("error")
        .colors(vec![Rgb(255, 127, 80), Rgb(64, 224, 208)])
        .plot_title(Text::from("Bar Plot").font("Arial").size(18))
        .x_title(Text::from("animal").font("Arial").size(15))
        .y_title(Text::from("value").font("Arial").size(15))
        .legend_title(Text::from("gender").font("Arial").size(15))
        .legend(
            &Legend::new()
                .orientation(Orientation::Horizontal)
                .y(1.0)
                .x(0.43),
        )
        .build()
        .plot();
}
