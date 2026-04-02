use plotlars::{BarPlot, Histogram, LinePlot, Orientation, Plot, Rgb, ScatterPlot, TimeSeriesPlot};
use polars::prelude::*;

fn main() {
    let penguins = LazyCsvReader::new(PlRefPath::new("data/penguins.csv"))
        .finish()
        .unwrap()
        .select([
            col("species"),
            col("flipper_length_mm").cast(DataType::Int16),
            col("body_mass_g").cast(DataType::Int16),
        ])
        .collect()
        .unwrap();

    // Scatter Plot -- grouped by species
    ScatterPlot::builder()
        .data(&penguins)
        .x("body_mass_g")
        .y("flipper_length_mm")
        .group("species")
        .opacity(0.5)
        .size(10)
        .colors(vec![Rgb(178, 34, 34), Rgb(65, 105, 225), Rgb(255, 140, 0)])
        .plot_title("Scatter Plot")
        .x_title("body mass (g)")
        .y_title("flipper length (mm)")
        .legend_title("species")
        .build()
        .plot();

    // Histogram -- overlapping distributions
    Histogram::builder()
        .data(&penguins)
        .x("body_mass_g")
        .group("species")
        .opacity(0.5)
        .colors(vec![Rgb(255, 165, 0), Rgb(147, 112, 219), Rgb(46, 139, 87)])
        .plot_title("Histogram")
        .x_title("body mass (g)")
        .y_title("count")
        .legend_title("species")
        .build()
        .plot();

    // Line Plot -- sine and cosine curves
    let x: Vec<f64> = (0..500)
        .map(|i| i as f64 * 2.0 * std::f64::consts::PI / 499.0)
        .collect();
    let sine: Vec<f64> = x.iter().map(|v| v.sin()).collect();
    let cosine: Vec<f64> = x.iter().map(|v| v.cos()).collect();
    let trig = df!["x" => &x, "sine" => &sine, "cosine" => &cosine].unwrap();

    LinePlot::builder()
        .data(&trig)
        .x("x")
        .y("sine")
        .additional_lines(vec!["cosine"])
        .colors(vec![Rgb(255, 0, 0), Rgb(0, 0, 255)])
        .plot_title("Line Plot")
        .x_title("x")
        .y_title("y")
        .build()
        .plot();

    // Bar Plot -- grouped by gender
    let animals = LazyCsvReader::new(PlRefPath::new("data/animal_statistics.csv"))
        .finish()
        .unwrap()
        .collect()
        .unwrap();

    BarPlot::builder()
        .data(&animals)
        .labels("animal")
        .values("value")
        .group("gender")
        .colors(vec![Rgb(255, 127, 80), Rgb(64, 224, 208)])
        .plot_title("Bar Plot")
        .x_title("animal")
        .y_title("value")
        .legend_title("gender")
        .build()
        .plot();

    // Bar Plot -- horizontal orientation
    BarPlot::builder()
        .data(&animals)
        .labels("animal")
        .values("value")
        .group("gender")
        .orientation(Orientation::Horizontal)
        .colors(vec![Rgb(255, 127, 80), Rgb(64, 224, 208)])
        .plot_title("Horizontal Bar Plot")
        .x_title("value")
        .y_title("animal")
        .legend_title("gender")
        .build()
        .plot();

    // Time Series Plot -- revenue and cost
    let revenue = LazyCsvReader::new(PlRefPath::new("data/revenue_and_cost.csv"))
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
        .data(&revenue)
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
