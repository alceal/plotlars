use polars::prelude::*;

use plotlars::{Plot, ScatterMap, Text};

fn main() {
    let dataset = LazyCsvReader::new(PlPath::new("data/cities.csv"))
        .finish()
        .unwrap()
        .collect()
        .unwrap();

    ScatterMap::builder()
        .data(&dataset)
        .latitude("latitude")
        .longitude("longitude")
        .center([48.856613, 2.352222])
        .zoom(4)
        .group("city")
        .opacity(0.5)
        .size(12)
        .plot_title(Text::from("Scatter Map").font("Arial").size(18))
        .legend_title("cities")
        .build()
        .plot();
}
