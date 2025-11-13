use plotlars::{Axis, Legend, Line, Plot, Rgb, Shape, Text, TimeSeriesPlot};
use polars::prelude::*;

fn main() {
    let revenue_dataset = LazyCsvReader::new(PlPath::new("data/revenue_and_cost.csv"))
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
        .data(&revenue_dataset)
        .x("Date")
        .y("Revenue")
        .additional_series(vec!["Cost"])
        .size(8)
        .colors(vec![Rgb(0, 0, 255), Rgb(255, 0, 0)])
        .lines(vec![Line::Dash, Line::Solid])
        .with_shape(true)
        .shapes(vec![Shape::Circle, Shape::Square])
        .plot_title(Text::from("Time Series Plot").font("Arial").size(18))
        .legend(&Legend::new().x(0.05).y(0.9))
        .x_title("x")
        .y_title(Text::from("y").color(Rgb(0, 0, 255)))
        .y2_title(Text::from("y2").color(Rgb(255, 0, 0)))
        .y_axis(
            &Axis::new()
                .value_color(Rgb(0, 0, 255))
                .show_grid(false)
                .zero_line_color(Rgb(0, 0, 0)),
        )
        .y2_axis(
            &Axis::new()
                .axis_side(plotlars::AxisSide::Right)
                .value_color(Rgb(255, 0, 0))
                .show_grid(false),
        )
        .build()
        .plot();

    let temperature_dataset = LazyCsvReader::new(PlPath::new("data/debilt_2023_temps.csv"))
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
        .data(&temperature_dataset)
        .x("date")
        .y("tavg")
        .additional_series(vec!["tmin", "tmax"])
        .colors(vec![Rgb(128, 128, 128), Rgb(0, 122, 255), Rgb(255, 128, 0)])
        .lines(vec![Line::Solid, Line::Dot, Line::Dot])
        .plot_title("Temperature at De Bilt (2023)")
        .legend_title("Legend")
        .build()
        .plot();
}
