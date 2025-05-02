use polars::prelude::*;

use plotlars::{Line, Plot, Rgb, TimeSeriesPlot};

fn main() {
    let dataset = LazyCsvReader::new("data/debilt_2023_temps.csv")
        .with_has_header(true)
        .with_try_parse_dates(true)
        .finish()
        .unwrap()
        .with_columns(vec![
            (col("tavg") / lit(10)).alias("tavg"),
            (col("tmin") / lit(10)).alias("tmin"),
            (col("tmax") / lit(10)).alias("tmax"),
        ])
        .collect()
        .unwrap();

    TimeSeriesPlot::builder()
        .data(&dataset)
        .x("date")
        .y("tavg")
        .additional_series(vec!["tmin", "tmax"])
        .colors(vec![
            Rgb(128, 128, 128),
            Rgb(0, 122, 255),
            Rgb(255, 128, 0),
        ])
        .lines(vec![
            Line::Solid,
            Line::Dot,
            Line::Dot,
        ])
        .plot_title("Temperature at De Bilt (2023)")
        .legend_title("Legend")
        .build()
        .plot();
}
