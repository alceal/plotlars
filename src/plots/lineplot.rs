use bon::bon;

use plotly::{
    Layout as LayoutPlotly, Scatter, Trace,
    common::{Line as LinePlotly, Marker as MarkerPlotly, Mode},
};

use polars::{
    frame::DataFrame,
    prelude::{IntoLazy, col},
};
use serde::Serialize;

use crate::{
    common::{Layout, Line, Marker, PlotHelper, Polar},
    components::{Axis, Legend, Line as LineStyle, Rgb, Shape, Text},
};

/// A structure representing a line plot.
///
/// The `LinePlot` struct facilitates the creation and customization of line plots with various options
/// for data selection, layout configuration, and aesthetic adjustments. It supports the addition of multiple
/// lines, customization of marker shapes, line styles, colors, opacity settings, and comprehensive layout
/// customization including titles, axes, and legends.
///
/// # Arguments
///
/// * `data` - A reference to the `DataFrame` containing the data to be plotted.
/// * `x` - A string slice specifying the column name to be used for the x-axis (independent variable).
/// * `y` - A string slice specifying the column name to be used for the y-axis (dependent variable).
/// * `additional_lines` - An optional vector of string slices specifying additional y-axis columns to be plotted as lines.
/// * `size` - An optional `usize` specifying the size of the markers or the thickness of the lines.
/// * `color` - An optional `Rgb` value specifying the color of the markers and lines. This is used when `additional_lines` is not specified.
/// * `colors` - An optional vector of `Rgb` values specifying the colors for the markers and lines. This is used when `additional_lines` is specified to differentiate between multiple lines.
/// * `with_shape` - An optional `bool` indicating whether to display markers with shapes on the plot.
/// * `shape` - An optional `Shape` specifying the shape of the markers.
/// * `shapes` - An optional vector of `Shape` values specifying multiple shapes for the markers when plotting multiple lines.
/// * `width` - An optional `f64` specifying the width of the plotted lines.
/// * `line` - An optional `Line` specifying the type of the line (e.g., solid, dashed). This is used when `additional_lines` is not specified.
/// * `lines` - An optional vector of `Line` enums specifying the types of lines (e.g., solid, dashed) for each plotted line. This is used when `additional_lines` is specified to differentiate between multiple lines.
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
/// use ndarray::Array;
///
/// use polars::prelude::*;
/// use plotlars::{Axis, Line, LinePlot, Plot, Rgb, Text, TickDirection};
///
/// let x_values: Array<f64, _> = Array::linspace(0.0, 2.0 * std::f64::consts::PI, 1000);
/// let sine_values = x_values.mapv(f64::sin).to_vec();
/// let cosine_values = x_values.mapv(f64::cos).to_vec();
/// let x_values = x_values.to_vec();
///
/// let dataset = DataFrame::new(vec![
///     Column::new("x".into(), x_values),
///     Column::new("sine".into(), sine_values),
///     Column::new("cosine".into(), cosine_values),
/// ])
/// .unwrap();
///
/// LinePlot::builder()
///     .data(&dataset)
///     .x("x")
///     .y("sine")
///     .additional_lines(vec!["cosine"])
///     .colors(vec![
///         Rgb(255, 0, 0),
///         Rgb(0, 255, 0),
///     ])
///     .lines(vec![Line::Solid, Line::Dot])
///     .width(3.0)
///     .with_shape(false)
///     .plot_title(
///         Text::from("Line Plot")
///             .font("Arial")
///             .size(18)
///     )
///     .legend_title(
///         Text::from("series")
///             .font("Arial")
///             .size(15)
///     )
///     .x_axis(
///        &Axis::new()
///            .tick_direction(TickDirection::OutSide)
///            .axis_position(0.5)
///            .tick_values(vec![
///                0.5 * std::f64::consts::PI,
///                std::f64::consts::PI,
///                1.5 * std::f64::consts::PI,
///                2.0 * std::f64::consts::PI,
///            ])
///            .tick_labels(vec!["π/2", "π", "3π/2", "2π"])
///     )
///     .y_axis(
///        &Axis::new()
///            .tick_direction(TickDirection::OutSide)
///            .tick_values(vec![-1.0, 0.0, 1.0])
///            .tick_labels(vec!["-1", "0", "1"])
///     )
///     .build()
///     .plot();
/// ```
///
/// ![Example](https://imgur.com/PaXG300.png)
#[derive(Clone, Serialize)]
pub struct LinePlot {
    traces: Vec<Box<dyn Trace + 'static>>,
    layout: LayoutPlotly,
}

#[bon]
impl LinePlot {
    #[builder(on(String, into), on(Text, into))]
    pub fn new(
        data: &DataFrame,
        x: &str,
        y: &str,
        additional_lines: Option<Vec<&str>>,
        size: Option<usize>,
        color: Option<Rgb>,
        colors: Option<Vec<Rgb>>,
        with_shape: Option<bool>,
        shape: Option<Shape>,
        shapes: Option<Vec<Shape>>,
        width: Option<f64>,
        line: Option<LineStyle>,
        lines: Option<Vec<LineStyle>>,
        plot_title: Option<Text>,
        x_title: Option<Text>,
        y_title: Option<Text>,
        y2_title: Option<Text>,
        legend_title: Option<Text>,
        x_axis: Option<&Axis>,
        y_axis: Option<&Axis>,
        y2_axis: Option<&Axis>,
        legend: Option<&Legend>,
    ) -> Self {
        let z_title = None;
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
            data,
            x,
            y,
            additional_lines,
            size,
            color,
            colors,
            with_shape,
            shape,
            shapes,
            width,
            line,
            lines,
        );

        Self { traces, layout }
    }

    #[allow(clippy::too_many_arguments)]
    fn create_traces(
        data: &DataFrame,
        x_col: &str,
        y_col: &str,
        additional_lines: Option<Vec<&str>>,
        size: Option<usize>,
        color: Option<Rgb>,
        colors: Option<Vec<Rgb>>,
        with_shape: Option<bool>,
        shape: Option<Shape>,
        shapes: Option<Vec<Shape>>,
        width: Option<f64>,
        style: Option<LineStyle>,
        styles: Option<Vec<LineStyle>>,
    ) -> Vec<Box<dyn Trace + 'static>> {
        let mut traces: Vec<Box<dyn Trace + 'static>> = Vec::new();

        let opacity = None;

        let marker = Self::create_marker(
            0,
            opacity,
            size,
            color,
            colors.clone(),
            shape,
            shapes.clone(),
        );

        let line = Self::create_line(0, width, style, styles.clone());

        let name = Some(y_col);

        let trace = Self::create_trace(data, x_col, y_col, name, with_shape, marker, line);

        traces.push(trace);

        if let Some(additional_lines) = additional_lines {
            let additional_lines = additional_lines.into_iter();

            for (i, series) in additional_lines.enumerate() {
                let marker = Self::create_marker(
                    i + 1,
                    opacity,
                    size,
                    color,
                    colors.clone(),
                    shape,
                    shapes.clone(),
                );

                let line = Self::create_line(i + 1, width, style, styles.clone());

                let subset = data
                    .clone()
                    .lazy()
                    .select([col(x_col), col(series)])
                    .collect()
                    .unwrap();

                let name = Some(series);

                let trace =
                    Self::create_trace(&subset, x_col, series, name, with_shape, marker, line);

                traces.push(trace);
            }
        }

        traces
    }

    fn create_trace(
        data: &DataFrame,
        x_col: &str,
        y_col: &str,
        name: Option<&str>,
        with_shape: Option<bool>,
        marker: MarkerPlotly,
        line: LinePlotly,
    ) -> Box<dyn Trace + 'static> {
        let x_data = Self::get_numeric_column(data, x_col);
        let y_data = Self::get_numeric_column(data, y_col);

        let mut trace = Scatter::default().x(x_data).y(y_data);

        if let Some(with_shape) = with_shape {
            if with_shape {
                trace = trace.mode(Mode::LinesMarkers);
            } else {
                trace = trace.mode(Mode::Lines);
            }
        }

        trace = trace.marker(marker);
        trace = trace.line(line);

        if let Some(name) = name {
            trace = trace.name(name);
        }

        trace
    }
}

impl Layout for LinePlot {}
impl Line for LinePlot {}
impl Marker for LinePlot {}
impl Polar for LinePlot {}

impl PlotHelper for LinePlot {
    fn get_layout(&self) -> &LayoutPlotly {
        &self.layout
    }

    fn get_traces(&self) -> &Vec<Box<dyn Trace + 'static>> {
        &self.traces
    }
}
