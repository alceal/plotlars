use plotlars::{
    Axis, BarPlot, Legend, Line, Orientation, Plot, Rgb, ScatterPlot, Shape, SubplotGrid, Text,
    TickDirection, TimeSeriesPlot,
};
use polars::prelude::*;

fn main() {
    let dataset1 = LazyCsvReader::new(PlPath::new("data/animal_statistics.csv"))
        .finish()
        .unwrap()
        .collect()
        .unwrap();

    let plot1 = BarPlot::builder()
        .data(&dataset1)
        .labels("animal")
        .values("value")
        .orientation(Orientation::Vertical)
        .group("gender")
        .sort_groups_by(|a, b| a.len().cmp(&b.len()))
        .error("error")
        .colors(vec![Rgb(255, 127, 80), Rgb(64, 224, 208)])
        .plot_title(Text::from("Bar Plot").x(-0.045).y(1.35).size(14))
        .y_title(Text::from("value").x(-0.05).y(0.76))
        .x_title(Text::from("animal").x(0.97).y(-0.2))
        .legend(
            &Legend::new()
                .orientation(Orientation::Horizontal)
                .x(0.4)
                .y(1.2),
        )
        .build();

    let dataset2 = LazyCsvReader::new(PlPath::new("data/penguins.csv"))
        .finish()
        .unwrap()
        .select([
            col("species"),
            col("sex").alias("gender"),
            col("flipper_length_mm").cast(DataType::Int16),
            col("body_mass_g").cast(DataType::Int16),
        ])
        .collect()
        .unwrap();

    let axis = Axis::new()
        .show_line(true)
        .tick_direction(TickDirection::OutSide)
        .value_thousands(true);

    let plot2 = ScatterPlot::builder()
        .data(&dataset2)
        .x("body_mass_g")
        .y("flipper_length_mm")
        .group("species")
        .sort_groups_by(|a, b| {
            if a.len() == b.len() {
                a.cmp(b)
            } else {
                a.len().cmp(&b.len())
            }
        })
        .opacity(0.5)
        .size(12)
        .colors(vec![Rgb(178, 34, 34), Rgb(65, 105, 225), Rgb(255, 140, 0)])
        .shapes(vec![Shape::Circle, Shape::Square, Shape::Diamond])
        .plot_title(Text::from("Scatter Plot").x(-0.065).y(1.35).size(14))
        .x_title("body mass (g)")
        .y_title("flipper length (mm)")
        .legend_title("species")
        .x_axis(&axis.clone().value_range(vec![2500.0, 6500.0]))
        .y_axis(&axis.clone().value_range(vec![170.0, 240.0]))
        .legend(&Legend::new().x(0.98).y(0.95))
        .build();

    let dataset3 = LazyCsvReader::new(PlPath::new("data/debilt_2023_temps.csv"))
        .with_has_header(true)
        .with_try_parse_dates(true)
        .finish()
        .unwrap()
        .with_columns(vec![
            (col("tavg") / lit(10)).alias("avg"),
            (col("tmin") / lit(10)).alias("min"),
            (col("tmax") / lit(10)).alias("max"),
        ])
        .collect()
        .unwrap();

    let plot3 = TimeSeriesPlot::builder()
        .data(&dataset3)
        .x("date")
        .y("avg")
        .additional_series(vec!["min", "max"])
        .colors(vec![Rgb(128, 128, 128), Rgb(0, 122, 255), Rgb(255, 128, 0)])
        .lines(vec![Line::Solid, Line::Dot, Line::Dot])
        .plot_title(Text::from("Time Series Plot").x(-0.045).y(1.35).size(14))
        .y_title(Text::from("Temperature (ºC)").x(-0.05).y(0.6))
        .legend_title("")
        .build();

    SubplotGrid::regular()
        .plots(vec![&plot1, &plot2, &plot3])
        .rows(2)
        .cols(2)
        .v_gap(0.4)
        .title(
            Text::from("Subplot Grid")
                .size(16)
                .font("Arial bold")
                .y(0.95),
        )
        .build()
        .plot();
}
