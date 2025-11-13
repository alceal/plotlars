use bon::bon;
use plotly::{layout::Layout as LayoutPlotly, Trace};
use serde::ser::{Serialize, SerializeStruct, Serializer};
use serde_json::Value;

use crate::common::{Layout, PlotHelper, Polar};
use crate::components::Text;

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
        state.serialize_field("data", &self.traces)?;

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
    /// * `plots` - A vector of plot references to arrange in the grid. Plots are positioned in row-major order.
    /// * `rows` - An optional `usize` specifying the number of rows in the grid (default: 1).
    /// * `cols` - An optional `usize` specifying the number of columns in the grid (default: 1).
    /// * `title` - An optional `Text` struct specifying the overall title for the entire subplot figure.
    /// * `h_gap` - An optional `f64` value specifying the horizontal spacing between columns (range: 0.0 to 1.0, default: 0.1).
    /// * `v_gap` - An optional `f64` value specifying the vertical spacing between rows (range: 0.0 to 1.0, default: 0.1).
    ///
    /// # Panics
    ///
    /// This method will panic if:
    /// - The plots vector is empty
    /// - `rows` is 0
    /// - `cols` is 0
    /// - Number of plots exceeds grid capacity (rows × cols)
    ///
    /// # Example
    ///
    /// ```rust
    /// use plotlars::{
    ///     Axis, BarPlot, Legend, Line, Orientation, Plot, Rgb, ScatterPlot, Shape, SubplotGrid, Text,
    ///     TickDirection, TimeSeriesPlot,
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
    ///     .plot_title(Text::from("Bar Plot").x(-0.045).y(1.35).size(14))
    ///     .y_title(Text::from("value").x(-0.05).y(0.76))
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
    ///     .plot_title(Text::from("Scatter Plot").x(-0.065).y(1.35).size(14))
    ///     .x_title("body mass (g)")
    ///     .y_title("flipper length (mm)")
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
    ///     .plot_title(Text::from("Time Series Plot").x(-0.045).y(1.35).size(14))
    ///     .y_title(Text::from("Temperature (ºC)").x(-0.05).y(0.6))
    ///     .legend_title("")
    ///     .build();
    ///
    /// SubplotGrid::regular()
    ///     .plots(vec![&plot1, &plot2, &plot3])
    ///     .rows(2)
    ///     .cols(2)
    ///     .v_gap(0.4)
    ///     .title(
    ///         Text::from("Subplot Grid")
    ///             .size(16)
    ///             .font("Arial bold")
    ///             .y(0.95),
    ///     )
    ///     .build()
    ///     .plot();
    /// ```
    ///
    /// ![Example](https://imgur.com/e0wnuPJ.png)
    #[builder(on(String, into), on(Text, into), finish_fn = build)]
    pub fn regular(
        plots: Vec<&dyn PlotHelper>,
        rows: Option<usize>,
        cols: Option<usize>,
        title: Option<Text>,
        h_gap: Option<f64>,
        v_gap: Option<f64>,
    ) -> Self {
        regular::build_regular(plots, rows, cols, title, h_gap, v_gap, None)
    }

    #[builder(on(String, into), on(Text, into), finish_fn = build)]
    fn irregular(
        plots: Vec<(&dyn PlotHelper, usize, usize, usize, usize)>,
        rows: Option<usize>,
        cols: Option<usize>,
        title: Option<Text>,
    ) -> Self {
        irregular::build_irregular(plots, rows, cols, title)
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
}
