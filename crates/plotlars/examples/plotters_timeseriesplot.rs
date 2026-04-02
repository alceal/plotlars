use plotlars::{Plot, Rgb, TimeSeriesPlot};
use polars::prelude::*;

fn main() {
    let dataset = LazyCsvReader::new(PlRefPath::new("data/revenue_and_cost.csv"))
        .finish()
        .unwrap()
        .select([
            col("Date").cast(DataType::String),
            col("Revenue").cast(DataType::Int32),
            col("Cost").cast(DataType::Int32),
        ])
        .collect()
        .unwrap();

    TimeSeriesPlot::builder()
        .data(&dataset)
        .x("Date")
        .y("Revenue")
        .additional_series(vec!["Cost"])
        .colors(vec![Rgb(0, 0, 255), Rgb(255, 0, 0)])
        .plot_title("Time Series Plot")
        .x_title("date")
        .y_title("value")
        .build()
        .plot();
}
