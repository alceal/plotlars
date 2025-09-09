use bon::bon;

use plotly::{
    Layout as LayoutPlotly, ScatterGeo as ScatterGeoPlotly, Trace,
    common::{Line as LinePlotly, Marker as MarkerPlotly},
};

use polars::frame::DataFrame;
use serde::Serialize;

use crate::{
    common::{Layout, Line, Marker, PlotHelper, Polar},
    components::{Legend, Mode, Rgb, Shape, Text},
};

/// A structure representing a geographic scatter plot.
///
/// The `ScatterGeo` struct facilitates the creation and customization of geographic scatter plots
/// with various options for data selection, grouping, layout configuration, and aesthetic adjustments.
/// It supports plotting data points on a map using latitude and longitude coordinates, with customization
/// for markers, lines, text labels, and comprehensive layout options.
///
/// # Arguments
///
/// * `data` - A reference to the `DataFrame` containing the data to be plotted.
/// * `lat` - A string slice specifying the column name to be used for latitude coordinates.
/// * `lon` - A string slice specifying the column name to be used for longitude coordinates.
/// * `mode` - An optional `Mode` specifying the drawing mode (markers, lines, or both).
/// * `text` - An optional string slice specifying the column name to be used for text labels.
/// * `group` - An optional string slice specifying the column name to be used for grouping data points.
/// * `sort_groups_by` - Optional comparator `fn(&str, &str) -> std::cmp::Ordering` to control group ordering. Groups are sorted lexically by default.
/// * `opacity` - An optional `f64` value specifying the opacity of the plot elements (range: 0.0 to 1.0).
/// * `size` - An optional `usize` specifying the size of the markers.
/// * `color` - An optional `Rgb` value specifying the color of the markers. This is used when `group` is not specified.
/// * `colors` - An optional vector of `Rgb` values specifying the colors for the markers. This is used when `group` is specified.
/// * `shape` - An optional `Shape` specifying the shape of the markers. This is used when `group` is not specified.
/// * `shapes` - An optional vector of `Shape` values specifying multiple shapes for the markers when plotting multiple groups.
/// * `line_width` - An optional `f64` value specifying the width of the lines (when mode includes lines).
/// * `line_color` - An optional `Rgb` value specifying the color of the lines.
/// * `plot_title` - An optional `Text` struct specifying the title of the plot.
/// * `legend_title` - An optional `Text` struct specifying the title of the legend.
/// * `legend` - An optional reference to a `Legend` struct for customizing the legend of the plot.
///
/// # Example
///
/// ```rust
/// use plotlars::{Plot, Rgb, ScatterGeo, Shape, Text, Mode};
/// use polars::prelude::*;
///
/// // Create sample data with cities and their coordinates
/// let data = df![
///     "city" => ["New York", "Los Angeles", "Chicago", "Houston", "Phoenix"],
///     "lat" => [40.7128, 34.0522, 41.8781, 29.7604, 33.4484],
///     "lon" => [-74.0060, -118.2437, -87.6298, -95.3698, -112.0740],
///     "population" => [8336817, 3979576, 2693976, 2320268, 1680992],
///     "region" => ["East", "West", "Central", "South", "West"]
/// ].unwrap();
///
/// ScatterGeo::builder()
///     .data(&data)
///     .lat("lat")
///     .lon("lon")
///     .mode(Mode::Markers)
///     .text("city")
///     .group("region")
///     .size(15)
///     .colors(vec![
///         Rgb(255, 0, 0),
///         Rgb(0, 255, 0),
///         Rgb(0, 0, 255),
///         Rgb(255, 165, 0),
///     ])
///     .plot_title(
///         Text::from("US Cities by Region")
///             .font("Arial")
///             .size(20)
///     )
///     .legend_title("Region")
///     .build()
///     .plot();
/// ```
///
/// ![Example](https://imgur.com/tYHM5FH.png)
#[derive(Clone, Serialize)]
pub struct ScatterGeo {
    traces: Vec<Box<dyn Trace + 'static>>,
    layout: LayoutPlotly,
}

#[bon]
impl ScatterGeo {
    #[builder(on(String, into), on(Text, into))]
    pub fn new(
        data: &DataFrame,
        lat: &str,
        lon: &str,
        mode: Option<Mode>,
        text: Option<&str>,
        group: Option<&str>,
        sort_groups_by: Option<fn(&String, &String) -> std::cmp::Ordering>,
        opacity: Option<f64>,
        size: Option<usize>,
        color: Option<Rgb>,
        colors: Option<Vec<Rgb>>,
        shape: Option<Shape>,
        shapes: Option<Vec<Shape>>,
        line_width: Option<f64>,
        line_color: Option<Rgb>,
        plot_title: Option<Text>,
        legend_title: Option<Text>,
        legend: Option<&Legend>,
    ) -> Self {
        let x_title = None;
        let y_title = None;
        let z_title = None;
        let x_axis = None;
        let y_axis = None;
        let z_axis = None;
        let y2_title = None;
        let y2_axis = None;

        let layout = Self::create_layout(
            plot_title,
            x_title,
            y_title,
            y2_title,
            z_title,
            legend_title,
            x_axis,
            y_axis,
            y2_axis,
            z_axis,
            legend,
        );

        let traces = Self::create_traces(
            data, lat, lon, mode, text, group, sort_groups_by, opacity, size, color, colors, shape, shapes,
            line_width, line_color,
        );

        Self { traces, layout }
    }

    #[allow(clippy::too_many_arguments)]
    fn create_traces(
        data: &DataFrame,
        lat: &str,
        lon: &str,
        mode: Option<Mode>,
        text: Option<&str>,
        group: Option<&str>,
        sort_groups_by: Option<fn(&String, &String) -> std::cmp::Ordering>,
        opacity: Option<f64>,
        size: Option<usize>,
        color: Option<Rgb>,
        colors: Option<Vec<Rgb>>,
        shape: Option<Shape>,
        shapes: Option<Vec<Shape>>,
        line_width: Option<f64>,
        line_color: Option<Rgb>,
    ) -> Vec<Box<dyn Trace + 'static>> {
        let mut traces: Vec<Box<dyn Trace + 'static>> = Vec::new();

        match group {
            Some(group_col) => {
                let groups = Self::get_unique_groups(data, group_col, sort_groups_by);
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

                    let line = Self::create_line(line_width, line_color);

                    let subset = Self::filter_data_by_group(data, group_col, group);

                    let trace = Self::create_trace(
                        &subset,
                        lat,
                        lon,
                        mode,
                        text,
                        Some(group),
                        marker,
                        line,
                    );

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

                let line = Self::create_line(line_width, line_color);

                let trace = Self::create_trace(data, lat, lon, mode, text, group, marker, line);

                traces.push(trace);
            }
        }

        traces
    }

    #[allow(clippy::too_many_arguments)]
    fn create_trace(
        data: &DataFrame,
        lat: &str,
        lon: &str,
        mode: Option<Mode>,
        text: Option<&str>,
        group_name: Option<&str>,
        marker: MarkerPlotly,
        line: LinePlotly,
    ) -> Box<dyn Trace + 'static> {
        let lat_data = Self::get_numeric_column(data, lat);
        let lon_data = Self::get_numeric_column(data, lon);

        let mut trace = ScatterGeoPlotly::new(lat_data, lon_data);

        if let Some(mode) = mode {
            trace = trace.mode(mode.to_plotly());
        } else {
            trace = trace.mode(plotly::common::Mode::Markers);
        }

        trace = trace.marker(marker);
        trace = trace.line(line);

        if let Some(text_col) = text {
            let text_data = Self::get_string_column(data, text_col);
            let text_strings: Vec<String> = text_data
                .iter()
                .map(|opt| opt.clone().unwrap_or_default())
                .collect();
            trace = trace.text_array(text_strings);
        }

        if let Some(name) = group_name {
            trace = trace.name(name);
        }

        trace
    }

    fn create_line(line_width: Option<f64>, line_color: Option<Rgb>) -> LinePlotly {
        let mut line = LinePlotly::new();

        if let Some(width) = line_width {
            line = line.width(width);
        }

        if let Some(color) = line_color {
            line = line.color(color.to_plotly());
        }

        line
    }
}

impl Layout for ScatterGeo {}
impl Marker for ScatterGeo {}
impl Line for ScatterGeo {}
impl Polar for ScatterGeo {}

impl PlotHelper for ScatterGeo {
    fn get_layout(&self) -> &LayoutPlotly {
        &self.layout
    }

    fn get_traces(&self) -> &Vec<Box<dyn Trace + 'static>> {
        &self.traces
    }
}
