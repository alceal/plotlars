use bon::bon;
use plotly::{layout::Layout as LayoutPlotly, Trace};
use serde::ser::{Serialize, SerializeStruct, Serializer};
use serde_json::Value;

use crate::common::{Layout, PlotHelper, Polar};
use crate::components::{Dimensions, Text};

mod custom_legend;
mod irregular;
mod regular;
mod shared;

/// A structure representing a subplot grid layout.
///
/// The `SubplotGrid` struct facilitates the creation of multi-plot layouts arranged in a grid configuration.
/// Plots are automatically arranged in rows and columns in row-major order (left-to-right, top-to-bottom).
/// Each subplot retains its own title, axis labels, and legend, providing flexibility for displaying
/// multiple related visualizations in a single figure.
///
/// # Features
///
/// - Automatic grid layout with configurable rows and columns
/// - Individual subplot titles (extracted from plot titles)
/// - Independent axis labels for each subplot
/// - Configurable horizontal and vertical spacing
/// - Overall figure title
/// - Sparse grid support (fewer plots than grid capacity)
///
#[derive(Clone)]
pub struct SubplotGrid {
    traces: Vec<Box<dyn Trace + 'static>>,
    layout: LayoutPlotly,
    layout_json: Option<Value>,
}

impl Serialize for SubplotGrid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("SubplotGrid", 2)?;
        state.serialize_field("traces", &self.traces)?;

        if let Some(ref layout_json) = self.layout_json {
            state.serialize_field("layout", layout_json)?;
        } else {
            state.serialize_field("layout", &self.layout)?;
        }

        state.end()
    }
}

#[bon]
impl SubplotGrid {
    /// Creates a subplot grid layout.
    ///
    /// Arranges plots in a row × column grid with automatic positioning. Plots are placed
    /// in row-major order (left-to-right, top-to-bottom). Each subplot retains its individual title
    /// (from the plot's `plot_title`), axis labels, and legend.
    ///
    /// # Arguments
    ///
    /// * `plots` - Vector of plot references to arrange in the grid. Plots are positioned in row-major order.
    /// * `rows` - Number of rows in the grid (default: 1).
    /// * `cols` - Number of columns in the grid (default: 1).
    /// * `title` - Overall title for the entire subplot figure (optional).
    /// * `h_gap` - Horizontal spacing between columns (range: 0.0 to 1.0, default: 0.1).
    /// * `v_gap` - Vertical spacing between rows (range: 0.0 to 1.0, default: 0.1).
    ///
    /// # Example
    ///
    /// ```rust
    /// use plotlars::{
    ///     Axis, BarPlot, BoxPlot, Legend, Line, Orientation, Plot, Rgb, ScatterPlot, Shape,
    ///     SubplotGrid, Text, TickDirection, TimeSeriesPlot,
    /// };
    /// use polars::prelude::*;
    ///
    /// let dataset1 = LazyCsvReader::new(PlPath::new("data/animal_statistics.csv"))
    ///     .finish()
    ///     .unwrap()
    ///     .collect()
    ///     .unwrap();
    ///
    /// let plot1 = BarPlot::builder()
    ///     .data(&dataset1)
    ///     .labels("animal")
    ///     .values("value")
    ///     .orientation(Orientation::Vertical)
    ///     .group("gender")
    ///     .sort_groups_by(|a, b| a.len().cmp(&b.len()))
    ///     .error("error")
    ///     .colors(vec![Rgb(255, 127, 80), Rgb(64, 224, 208)])
    ///     .plot_title(Text::from("Bar Plot").x(-0.05).y(1.35).size(14))
    ///     .y_title(Text::from("value").x(-0.055).y(0.76))
    ///     .x_title(Text::from("animal").x(0.97).y(-0.2))
    ///     .legend(
    ///         &Legend::new()
    ///             .orientation(Orientation::Horizontal)
    ///             .x(0.4)
    ///             .y(1.2),
    ///     )
    ///     .build();
    ///
    /// let dataset2 = LazyCsvReader::new(PlPath::new("data/penguins.csv"))
    ///     .finish()
    ///     .unwrap()
    ///     .select([
    ///         col("species"),
    ///         col("sex").alias("gender"),
    ///         col("flipper_length_mm").cast(DataType::Int16),
    ///         col("body_mass_g").cast(DataType::Int16),
    ///     ])
    ///     .collect()
    ///     .unwrap();
    ///
    /// let axis = Axis::new()
    ///     .show_line(true)
    ///     .tick_direction(TickDirection::OutSide)
    ///     .value_thousands(true);
    ///
    /// let plot2 = ScatterPlot::builder()
    ///     .data(&dataset2)
    ///     .x("body_mass_g")
    ///     .y("flipper_length_mm")
    ///     .group("species")
    ///     .sort_groups_by(|a, b| {
    ///         if a.len() == b.len() {
    ///             a.cmp(b)
    ///         } else {
    ///             a.len().cmp(&b.len())
    ///         }
    ///     })
    ///     .opacity(0.5)
    ///     .size(12)
    ///     .colors(vec![Rgb(178, 34, 34), Rgb(65, 105, 225), Rgb(255, 140, 0)])
    ///     .shapes(vec![Shape::Circle, Shape::Square, Shape::Diamond])
    ///     .plot_title(Text::from("Scatter Plot").x(-0.075).y(1.35).size(14))
    ///     .x_title(Text::from("body mass (g)").y(-0.4))
    ///     .y_title(Text::from("flipper length (mm)").x(-0.078).y(0.5))
    ///     .legend_title("species")
    ///     .x_axis(&axis.clone().value_range(vec![2500.0, 6500.0]))
    ///     .y_axis(&axis.clone().value_range(vec![170.0, 240.0]))
    ///     .legend(&Legend::new().x(0.98).y(0.95))
    ///     .build();
    ///
    /// let dataset3 = LazyCsvReader::new(PlPath::new("data/debilt_2023_temps.csv"))
    ///     .with_has_header(true)
    ///     .with_try_parse_dates(true)
    ///     .finish()
    ///     .unwrap()
    ///     .with_columns(vec![
    ///         (col("tavg") / lit(10)).alias("avg"),
    ///         (col("tmin") / lit(10)).alias("min"),
    ///         (col("tmax") / lit(10)).alias("max"),
    ///     ])
    ///     .collect()
    ///     .unwrap();
    ///
    /// let plot3 = TimeSeriesPlot::builder()
    ///     .data(&dataset3)
    ///     .x("date")
    ///     .y("avg")
    ///     .additional_series(vec!["min", "max"])
    ///     .colors(vec![Rgb(128, 128, 128), Rgb(0, 122, 255), Rgb(255, 128, 0)])
    ///     .lines(vec![Line::Solid, Line::Dot, Line::Dot])
    ///     .plot_title(Text::from("Time Series Plot").x(-0.05).y(1.35).size(14))
    ///     .y_title(Text::from("temperature (ºC)").x(-0.055).y(0.6))
    ///     .legend(&Legend::new().x(0.9).y(1.25))
    ///     .build();
    ///
    /// let plot4 = BoxPlot::builder()
    ///     .data(&dataset2)
    ///     .labels("species")
    ///     .values("body_mass_g")
    ///     .orientation(Orientation::Vertical)
    ///     .group("gender")
    ///     .box_points(true)
    ///     .point_offset(-1.5)
    ///     .jitter(0.01)
    ///     .opacity(0.1)
    ///     .colors(vec![Rgb(0, 191, 255), Rgb(57, 255, 20), Rgb(255, 105, 180)])
    ///     .plot_title(Text::from("Box Plot").x(-0.075).y(1.35).size(14))
    ///     .x_title(Text::from("species").y(-0.3))
    ///     .y_title(Text::from("body mass (g)").x(-0.08).y(0.5))
    ///     .legend_title(Text::from("gender").size(12))
    ///     .y_axis(&Axis::new().value_thousands(true))
    ///     .legend(&Legend::new().x(1.0))
    ///     .build();
    ///
    /// SubplotGrid::regular()
    ///     .plots(vec![&plot1, &plot2, &plot3, &plot4])
    ///     .rows(2)
    ///     .cols(2)
    ///     .v_gap(0.4)
    ///     .title(
    ///         Text::from("Regular Subplot Grid")
    ///             .size(16)
    ///             .font("Arial bold")
    ///             .y(0.95),
    ///     )
    ///     .build()
    ///     .plot();
    /// ```
    ///
    /// ![Example](https://imgur.com/q0K7cyP.png)
    #[builder(on(String, into), on(Text, into), finish_fn = build)]
    pub fn regular(
        plots: Vec<&dyn PlotHelper>,
        rows: Option<usize>,
        cols: Option<usize>,
        title: Option<Text>,
        h_gap: Option<f64>,
        v_gap: Option<f64>,
        dimensions: Option<&Dimensions>,
    ) -> Self {
        regular::build_regular(plots, rows, cols, title, h_gap, v_gap, None, dimensions)
    }

    /// Creates an irregular grid subplot layout with custom row/column spanning.
    ///
    /// Allows plots to span multiple rows and/or columns, enabling dashboard-style
    /// layouts and asymmetric grid arrangements. Each plot explicitly specifies its
    /// position and span.
    ///
    /// # Arguments
    ///
    /// * `plots` - Vector of tuples `(plot, row, col, rowspan, colspan)` where:
    ///   - `plot`: Reference to the plot
    ///   - `row`: Starting row (0-indexed)
    ///   - `col`: Starting column (0-indexed)
    ///   - `rowspan`: Number of rows to span (minimum 1)
    ///   - `colspan`: Number of columns to span (minimum 1)
    /// * `rows` - Total number of rows in the grid (default: 1).
    /// * `cols` - Total number of columns in the grid (default: 1).
    /// * `title` - Overall title for the subplot figure (optional).
    /// * `h_gap` - Horizontal spacing between columns (range: 0.0 to 1.0, default: 0.1).
    /// * `v_gap` - Vertical spacing between rows (range: 0.0 to 1.0, default: 0.1).
    ///
    /// # Example
    ///
    /// ```rust
    /// use plotlars::{
    ///     Axis, CandlestickPlot, ColorBar, Direction, HeatMap, Histogram, Legend, Palette, Plot,
    ///     Rgb, SubplotGrid, Text, TickDirection, ValueExponent,
    /// };
    /// use polars::prelude::*;
    ///
    /// let dataset1 = LazyCsvReader::new(PlPath::new("data/penguins.csv"))
    ///     .finish()
    ///     .unwrap()
    ///     .select([
    ///         col("species"),
    ///         col("sex").alias("gender"),
    ///         col("flipper_length_mm").cast(DataType::Int16),
    ///         col("body_mass_g").cast(DataType::Int16),
    ///     ])
    ///     .collect()
    ///     .unwrap();
    ///
    /// let axis = Axis::new()
    ///     .show_line(true)
    ///     .show_grid(true)
    ///     .value_thousands(true)
    ///     .tick_direction(TickDirection::OutSide);
    ///
    /// let plot1 = Histogram::builder()
    ///     .data(&dataset1)
    ///     .x("body_mass_g")
    ///     .group("species")
    ///     .opacity(0.5)
    ///     .colors(vec![Rgb(255, 165, 0), Rgb(147, 112, 219), Rgb(46, 139, 87)])
    ///     .plot_title(Text::from("Histogram").x(0.0).y(1.35).size(14))
    ///     .x_title(Text::from("body mass (g)").x(0.94).y(-0.35))
    ///     .y_title(Text::from("count").x(-0.062).y(0.83))
    ///     .x_axis(&axis)
    ///     .y_axis(&axis)
    ///     .legend_title(Text::from("species"))
    ///     .legend(&Legend::new().x(0.87).y(1.2))
    ///     .build();
    ///
    /// let dataset2 = LazyCsvReader::new(PlPath::new("data/stock_prices.csv"))
    ///     .finish()
    ///     .unwrap()
    ///     .collect()
    ///     .unwrap();
    ///
    /// let increasing = Direction::new()
    ///     .line_color(Rgb(0, 200, 100))
    ///     .line_width(0.5);
    ///
    /// let decreasing = Direction::new()
    ///     .line_color(Rgb(200, 50, 50))
    ///     .line_width(0.5);
    ///
    /// let plot2 = CandlestickPlot::builder()
    ///     .data(&dataset2)
    ///     .dates("date")
    ///     .open("open")
    ///     .high("high")
    ///     .low("low")
    ///     .close("close")
    ///     .increasing(&increasing)
    ///     .decreasing(&decreasing)
    ///     .whisker_width(0.1)
    ///     .plot_title(Text::from("Candlestick").x(0.0).y(1.35).size(14))
    ///     .y_title(Text::from("Price ($)").x(-0.06).y(0.76))
    ///     .y_axis(&Axis::new().show_axis(true).show_grid(true))
    ///     .build();
    ///
    /// let dataset3 = LazyCsvReader::new(PlPath::new("data/heatmap.csv"))
    ///     .finish()
    ///     .unwrap()
    ///     .collect()
    ///     .unwrap();
    ///
    /// let plot3 = HeatMap::builder()
    ///     .data(&dataset3)
    ///     .x("x")
    ///     .y("y")
    ///     .z("z")
    ///     .color_bar(
    ///         &ColorBar::new()
    ///             .value_exponent(ValueExponent::None)
    ///             .separate_thousands(true)
    ///             .tick_length(5)
    ///             .tick_step(5000.0),
    ///     )
    ///     .plot_title(Text::from("Heat Map").x(0.0).y(1.35).size(14))
    ///     .color_scale(Palette::Viridis)
    ///     .build();
    ///
    /// SubplotGrid::irregular()
    ///     .plots(vec![
    ///         (&plot1, 0, 0, 1, 1),
    ///         (&plot2, 0, 1, 1, 1),
    ///         (&plot3, 1, 0, 1, 2),
    ///     ])
    ///     .rows(2)
    ///     .cols(2)
    ///     .v_gap(0.35)
    ///     .h_gap(0.05)
    ///     .title(
    ///         Text::from("Irregular Subplot Grid")
    ///             .size(16)
    ///             .font("Arial bold")
    ///             .y(0.95),
    ///     )
    ///     .build()
    ///     .plot();
    /// ```
    ///
    /// ![Example](https://imgur.com/RvZwv3O.png)
    #[builder(on(String, into), on(Text, into), finish_fn = build)]
    pub fn irregular(
        plots: Vec<(&dyn PlotHelper, usize, usize, usize, usize)>,
        rows: Option<usize>,
        cols: Option<usize>,
        title: Option<Text>,
        h_gap: Option<f64>,
        v_gap: Option<f64>,
        dimensions: Option<&Dimensions>,
    ) -> Self {
        irregular::build_irregular(plots, rows, cols, title, h_gap, v_gap, dimensions)
    }
}

impl Layout for SubplotGrid {}
impl Polar for SubplotGrid {}

#[doc(hidden)]
impl PlotHelper for SubplotGrid {
    fn get_layout(&self) -> &LayoutPlotly {
        &self.layout
    }

    fn get_traces(&self) -> &Vec<Box<dyn Trace + 'static>> {
        &self.traces
    }

    fn get_layout_override(&self) -> Option<&Value> {
        self.layout_json.as_ref()
    }
}
