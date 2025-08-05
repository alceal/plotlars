use bon::bon;

use plotly::{
    Layout as LayoutPlotly, ScatterPolar as ScatterPolarPlotly, Trace,
    common::{Line as LinePlotly, Marker as MarkerPlotly},
};

use polars::frame::DataFrame;
use serde::Serialize;

use crate::{
    common::{Layout, Marker, PlotHelper, Polar},
    components::{Fill, Legend, Line as LineStyle, Mode, Rgb, Shape, Text},
};

/// A structure representing a scatter polar plot.
///
/// The `ScatterPolar` struct facilitates the creation and customization of polar scatter plots with various options
/// for data selection, grouping, layout configuration, and aesthetic adjustments. It supports grouping of data,
/// customization of marker shapes, colors, sizes, line styles, and comprehensive layout customization
/// including titles and legends.
///
/// # Arguments
///
/// * `data` - A reference to the `DataFrame` containing the data to be plotted.
/// * `theta` - A string slice specifying the column name to be used for the angular coordinates (in degrees).
/// * `r` - A string slice specifying the column name to be used for the radial coordinates.
/// * `group` - An optional string slice specifying the column name to be used for grouping data points.
/// * `mode` - An optional `Mode` specifying the drawing mode (lines, markers, or both). Defaults to markers.
/// * `opacity` - An optional `f64` value specifying the opacity of the plot elements (range: 0.0 to 1.0).
/// * `fill` - An optional `Fill` type specifying how to fill the area under the trace.
/// * `size` - An optional `usize` specifying the size of the markers.
/// * `color` - An optional `Rgb` value specifying the color of the markers. This is used when `group` is not specified.
/// * `colors` - An optional vector of `Rgb` values specifying the colors for the markers. This is used when `group` is specified to differentiate between groups.
/// * `shape` - An optional `Shape` specifying the shape of the markers. This is used when `group` is not specified.
/// * `shapes` - An optional vector of `Shape` values specifying multiple shapes for the markers when plotting multiple groups.
/// * `width` - An optional `f64` specifying the width of the lines.
/// * `line` - An optional `LineStyle` specifying the style of the line (e.g., solid, dashed).
/// * `lines` - An optional vector of `LineStyle` enums specifying the styles of lines for multiple traces.
/// * `plot_title` - An optional `Text` struct specifying the title of the plot.
/// * `legend_title` - An optional `Text` struct specifying the title of the legend.
/// * `legend` - An optional reference to a `Legend` struct for customizing the legend of the plot (e.g., positioning, font, etc.).
///
/// # Example
///
/// ```rust
/// use polars::prelude::*;
/// use plotlars::{Legend, Line, Mode, Plot, Rgb, ScatterPolar, Shape, Text};
///
/// // Create sample data - comparing two products across multiple metrics
/// let angles = vec![
///     0., 60., 120., 180., 240., 300., 360., // Product A
///     0., 60., 120., 180., 240., 300., 360., // Product B
/// ];
/// let values = vec![
///     7.0, 8.5, 6.0, 5.5, 9.0, 8.0, 7.0, // Product A values
///     6.0, 7.0, 8.0, 9.0, 6.5, 7.5, 6.0, // Product B values
/// ];
/// let products = vec![
///     "Product A", "Product A", "Product A", "Product A",
///     "Product A", "Product A", "Product A",
///     "Product B", "Product B", "Product B", "Product B",
///     "Product B", "Product B", "Product B",
/// ];
///
/// let dataset = DataFrame::new(vec![
///     Column::new("angle".into(), angles),
///     Column::new("score".into(), values),
///     Column::new("product".into(), products),
/// ])
/// .unwrap();
///
/// ScatterPolar::builder()
///     .data(&dataset)
///     .theta("angle")
///     .r("score")
///     .group("product")
///     .mode(Mode::LinesMarkers)
///     .colors(vec![
///         Rgb(255, 99, 71),  // Tomato red
///         Rgb(60, 179, 113), // Medium sea green
///     ])
///     .shapes(vec![Shape::Circle, Shape::Square])
///     .lines(vec![Line::Solid, Line::Dash])
///     .width(2.5)
///     .size(8)
///     .plot_title(Text::from("Product Comparison").font("Arial").size(24))
///     .legend_title(Text::from("Products").font("Arial").size(14))
///     .legend(&Legend::new().x(0.65).y(0.75))
///     .build()
///     .plot();
/// ```
///
/// ![Example](https://imgur.com/fJiNlqn.png)
#[derive(Clone, Serialize)]
pub struct ScatterPolar {
    traces: Vec<Box<dyn Trace + 'static>>,
    layout: LayoutPlotly,
}

#[bon]
impl ScatterPolar {
    #[builder(on(String, into), on(Text, into))]
    pub fn new(
        data: &DataFrame,
        theta: &str,
        r: &str,
        group: Option<&str>,
        mode: Option<Mode>,
        opacity: Option<f64>,
        fill: Option<Fill>,
        size: Option<usize>,
        color: Option<Rgb>,
        colors: Option<Vec<Rgb>>,
        shape: Option<Shape>,
        shapes: Option<Vec<Shape>>,
        width: Option<f64>,
        line: Option<LineStyle>,
        lines: Option<Vec<LineStyle>>,
        plot_title: Option<Text>,
        legend_title: Option<Text>,
        legend: Option<&Legend>,
    ) -> Self {
        let x_title = None;
        let y_title = None;
        let y2_title = None;
        let z_title = None;
        let x_axis = None;
        let y_axis = None;
        let y2_axis = None;
        let z_axis = None;

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
            data, theta, r, group, mode, opacity, fill, size, color, colors, shape, shapes, width,
            line, lines,
        );

        Self { traces, layout }
    }

    #[allow(clippy::too_many_arguments)]
    fn create_traces(
        data: &DataFrame,
        theta: &str,
        r: &str,
        group: Option<&str>,
        mode: Option<Mode>,
        opacity: Option<f64>,
        fill: Option<Fill>,
        size: Option<usize>,
        color: Option<Rgb>,
        colors: Option<Vec<Rgb>>,
        shape: Option<Shape>,
        shapes: Option<Vec<Shape>>,
        width: Option<f64>,
        line: Option<LineStyle>,
        lines: Option<Vec<LineStyle>>,
    ) -> Vec<Box<dyn Trace + 'static>> {
        let mut traces: Vec<Box<dyn Trace + 'static>> = Vec::new();
        let mode = mode
            .map(|m| m.to_plotly())
            .unwrap_or(plotly::common::Mode::Markers);

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

                    let line_style = Self::create_line_with_color(
                        i,
                        width,
                        color,
                        colors.clone(),
                        line,
                        lines.clone(),
                    );

                    let subset = Self::filter_data_by_group(data, group_col, group);

                    let trace = Self::create_trace(
                        &subset,
                        theta,
                        r,
                        Some(group),
                        mode.clone(),
                        marker,
                        line_style,
                        fill,
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

                let line_style = Self::create_line_with_color(
                    0,
                    width,
                    color,
                    colors.clone(),
                    line,
                    lines.clone(),
                );

                let trace =
                    Self::create_trace(data, theta, r, group, mode, marker, line_style, fill);

                traces.push(trace);
            }
        }

        traces
    }

    #[allow(clippy::too_many_arguments)]
    fn create_trace(
        data: &DataFrame,
        theta: &str,
        r: &str,
        group_name: Option<&str>,
        mode: plotly::common::Mode,
        marker: MarkerPlotly,
        line: LinePlotly,
        fill: Option<Fill>,
    ) -> Box<dyn Trace + 'static> {
        let theta_values = Self::get_numeric_column(data, theta);
        let r_values = Self::get_numeric_column(data, r);

        let mut trace = ScatterPolarPlotly::default()
            .theta(theta_values)
            .r(r_values)
            .mode(mode);

        trace = trace.marker(marker);
        trace = trace.line(line);

        if let Some(fill_type) = fill {
            trace = trace.fill(fill_type.to_plotly());
        }

        if let Some(name) = group_name {
            trace = trace.name(name);
        }

        trace
    }

    fn create_line_with_color(
        index: usize,
        width: Option<f64>,
        color: Option<Rgb>,
        colors: Option<Vec<Rgb>>,
        style: Option<LineStyle>,
        styles: Option<Vec<LineStyle>>,
    ) -> LinePlotly {
        let mut line = LinePlotly::new();

        // Set width
        if let Some(width) = width {
            line = line.width(width);
        }

        // Set style
        if let Some(style) = style {
            line = line.dash(style.to_plotly());
        } else if let Some(styles) = styles {
            if let Some(style) = styles.get(index) {
                line = line.dash(style.to_plotly());
            }
        }

        // Set color
        if let Some(color) = color {
            line = line.color(color.to_plotly());
        } else if let Some(colors) = colors {
            if let Some(color) = colors.get(index) {
                line = line.color(color.to_plotly());
            }
        }

        line
    }
}

impl Layout for ScatterPolar {}
impl Marker for ScatterPolar {}
impl Polar for ScatterPolar {}

impl PlotHelper for ScatterPolar {
    fn get_layout(&self) -> &LayoutPlotly {
        &self.layout
    }

    fn get_traces(&self) -> &Vec<Box<dyn Trace + 'static>> {
        &self.traces
    }
}
