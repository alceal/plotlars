use plotlars::{BarPlot, Plot, Rgb};
use polars::prelude::*;

fn main() {
    let dataset = LazyCsvReader::new(PlRefPath::new("data/animal_statistics.csv"))
        .finish()
        .unwrap()
        .collect()
        .unwrap();

    BarPlot::builder()
        .data(&dataset)
        .labels("animal")
        .values("value")
        .group("gender")
        .error("error")
        .colors(vec![Rgb(255, 127, 80), Rgb(64, 224, 208)])
        .plot_title("Bar Plot")
        .x_title("animal")
        .y_title("value")
        .legend_title("gender")
        .build()
        .plot();
}
