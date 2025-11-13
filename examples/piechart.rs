use plotlars::{PieChart, Plot, Text};
use polars::prelude::*;

fn main() {
    let dataset = LazyCsvReader::new(PlPath::new("data/penguins.csv"))
        .finish()
        .unwrap()
        .select([col("species")])
        .collect()
        .unwrap();

    PieChart::builder()
        .data(&dataset)
        .labels("species")
        .hole(0.4)
        .pull(0.01)
        .rotation(20.0)
        .plot_title(Text::from("Pie Chart").font("Arial").size(18).x(0.485))
        .build()
        .plot();
}
