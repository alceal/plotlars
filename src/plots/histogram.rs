use bon::bon;

use plotly::{
    common::Marker as MarkerPlotly, histogram::HistFunc, layout::BarMode,
    Histogram as HistogramPlotly, Layout as LayoutPlotly, Trace,
};

use polars::frame::DataFrame;

use crate::{
    common::{Layout, Marker, Plot, Polar},
    components::{Axis, Legend, Rgb, Text},
};

/// A structure representing a histogram.
///
/// The `Histogram` struct facilitates the creation and customization of histograms with various options
/// for data selection, layout configuration, and aesthetic adjustments. It supports grouping of data,
/// opacity settings, and customizable markers and colors.
///
/// # Arguments
///
/// * `data` - A reference to the `DataFrame` containing the data to be plotted.
/// * `x` - A string slice specifying the column name to be used for the x-axis (independent variable).
/// * `group` - An optional string slice specifying the column name to be used for grouping data points.
/// * `opacity` - An optional `f64` value specifying the opacity of the plot markers (range: 0.0 to 1.0).
/// * `color` - An optional `Rgb` value specifying the color of the markers to be used for the plot. This is used when `group` is not specified.
/// * `colors` - An optional vector of `Rgb` values specifying the colors to be used for the plot. This is used when `group` is specified to differentiate between groups.
/// * `plot_title` - An optional `Text` struct specifying the title of the plot.
/// * `x_title` - An optional `Text` struct specifying the title of the x-axis.
/// * `y_title` - An optional `Text` struct specifying the title of the y-axis.
/// * `legend_title` - An optional `Text` struct specifying the title of the legend.
/// * `x_axis` - An optional reference to an `Axis` struct for customizing the x-axis.
/// * `y_axis` - An optional reference to an `Axis` struct for customizing the y-axis.
/// * `legend` - An optional reference to a `Legend` struct for customizing the legend of the plot (e.g., positioning, font, etc.).
///
/// # Example
///
/// ```rust
/// use plotlars::{Axis, Histogram, Legend, Plot, Rgb, Text, TickDirection};
///
/// let dataset = LazyCsvReader::new("data/penguins.csv")
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
/// Histogram::builder()
///     .data(&dataset)
///     .x("body_mass_g")
///     .group("species")
///     .opacity(0.5)
///     .colors(vec![
///         Rgb(255, 165, 0),
///         Rgb(147, 112, 219),
///         Rgb(46, 139, 87),
///     ])
///     .plot_title(
///         Text::from("Histogram")
///             .font("Arial")
///             .size(18)
///     )
///     .x_title(
///         Text::from("body mass (g)")
///             .font("Arial")
///             .size(15)
///     )
///     .y_title(
///         Text::from("count")
///             .font("Arial")
///             .size(15)
///     )
///     .legend_title(
///         Text::from("species")
///             .font("Arial")
///             .size(15)
///     )
///     .x_axis(&axis)
///     .y_axis(&axis)
///     .legend(
///         &Legend::new()
///             .x(0.9)
///     )
///     .build()
///     .plot();
/// ```
///
/// ![Example](https://imgur.com/w2oiuIo.png)
pub struct Histogram {
    traces: Vec<Box<dyn Trace + 'static>>,
    layout: LayoutPlotly,
}

#[bon]
impl Histogram {
    #[builder(on(String, into), on(Text, into))]
    pub fn new(
        data: &DataFrame,
        x: &str,
        group: Option<&str>,
        opacity: Option<f64>,
        color: Option<Rgb>,
        colors: Option<Vec<Rgb>>,
        plot_title: Option<Text>,
        x_title: Option<Text>,
        y_title: Option<Text>,
        legend_title: Option<Text>,
        x_axis: Option<&Axis>,
        y_axis: Option<&Axis>,
        legend: Option<&Legend>,
    ) -> Self {
        let mut layout = Self::create_layout(
            plot_title,
            x_title,
            y_title,
            legend_title,
            x_axis,
            y_axis,
            legend,
        );

        layout = layout.bar_mode(BarMode::Overlay);

        let traces = Self::create_traces(data, x, group, opacity, color, colors);

        Self { traces, layout }
    }

    fn create_traces(
        data: &DataFrame,
        x: &str,
        group: Option<&str>,
        opacity: Option<f64>,
        color: Option<Rgb>,
        colors: Option<Vec<Rgb>>,
    ) -> Vec<Box<dyn Trace + 'static>> {
        let mut traces: Vec<Box<dyn Trace + 'static>> = Vec::new();

        let size = None;
        let shape = None;
        let shapes = None;

        match group {
            Some(group_col) => {
                let groups = Self::get_unique_groups(data, group_col);

                let groups = groups.iter().map(|s| s.as_str());

                for (i, group) in groups.enumerate() {
                    let marker = Self::create_marker(
                        i,
                        opacity,
                        size,
                        color,
                        colors.clone(),
                        shape,
                        shapes.clone(),
                    );

                    let subset = Self::filter_data_by_group(data, group_col, group);

                    let trace = Self::create_trace(&subset, x, Some(group), marker);

                    traces.push(trace);
                }
            }
            None => {
                let group = None;

                let marker = Self::create_marker(
                    0,
                    opacity,
                    size,
                    color,
                    colors.clone(),
                    shape,
                    shapes.clone(),
                );

                let trace = Self::create_trace(data, x, group, marker);

                traces.push(trace);
            }
        }

        traces
    }

    fn create_trace(
        data: &DataFrame,
        x: &str,
        group_name: Option<&str>,
        marker: MarkerPlotly,
    ) -> Box<dyn Trace + 'static> {
        let x = Self::get_numeric_column(data, x);

        let mut trace = HistogramPlotly::default().x(x).hist_func(HistFunc::Count);

        trace = trace.marker(marker);

        if let Some(name) = group_name {
            trace = trace.name(name);
        }

        trace
    }
}

impl Layout for Histogram {}
impl Marker for Histogram {}
impl Polar for Histogram {}

impl Plot for Histogram {
    fn get_layout(&self) -> &LayoutPlotly {
        &self.layout
    }

    fn get_traces(&self) -> &Vec<Box<dyn Trace + 'static>> {
        &self.traces
    }
}
