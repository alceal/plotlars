use polars::prelude::*;

use plotlars::{Axis, Legend, Line, Plot, Rgb, Shape, Text, TimeSeriesPlot};

fn main() {
    let dataset = LazyCsvReader::new(PlPath::new("data/revenue_and_cost.csv"))
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
        .size(8)
        .colors(vec![
            Rgb(0, 0, 255),
            Rgb(255, 0, 0),
        ])
        .lines(vec![Line::Dash, Line::Solid])
        .with_shape(true)
        .shapes(vec![Shape::Circle, Shape::Square])
        .plot_title(
            Text::from("Time Series Plot")
                .font("Arial")
                .size(18)
        )
        .legend(
            &Legend::new()
                .x(0.05)
                .y(0.9)
        )
        .x_title("x")
        .y_title(
            Text::from("y")
                .color(Rgb(0, 0, 255))
        )
        .y_title2(
            Text::from("y2")
                .color(Rgb(255, 0, 0))
        )
        .y_axis(
            &Axis::new()
                .value_color(Rgb(0, 0, 255))
                .show_grid(false)
                .zero_line_color(Rgb(0, 0, 0))
        )
        .y_axis2(
            &Axis::new()
                .axis_side(plotlars::AxisSide::Right)
                .value_color(Rgb(255, 0, 0))
                .show_grid(false)
        )
        .build()
        .plot();
}
