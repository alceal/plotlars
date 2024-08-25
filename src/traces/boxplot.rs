//! This module provides implementations for vertical and horizontal box plots using the Plotly library.
//!
//! The `VerticalBoxPlot` and `HorizontalBoxPlot` structs allow for the creation and customization of box plots
//! with various options for data, layout, and aesthetics.

use bon::bon;

use plotly::{
    box_plot::BoxPoints,
    common::{Line as LinePlotly, Marker, Orientation},
    BoxPlot, Layout, Trace as TracePlotly,
};

use polars::frame::DataFrame;

use crate::{
    aesthetics::{line::Line, mark::Mark},
    colors::Rgb,
    texts::Text,
    traits::layout::LayoutPlotly,
    traits::plot::Plot,
    traits::polar::Polar,
    traits::trace::Trace,
};

/// A structure representing a vertical bar plot.
pub struct VerticalBoxPlot {
    traces: Vec<Box<dyn TracePlotly + 'static>>,
    layout: Layout,
}

#[bon]
impl VerticalBoxPlot {
    /// Creates a new `VerticalBoxPlot`.
    ///
    /// # Arguments
    ///
    /// * `data` - A reference to the `DataFrame` containing the data to be plotted.
    /// * `x` - A string specifying the column name to be used for the x-axis.
    /// * `y` - A string specifying the column name to be used for the y-axis.
    /// * `group` - An optional string specifying the column name to be used for grouping data points.
    /// * `box_points` - An optional boolean indicating whether individual data points should be plotted along with the box plot.
    /// * `point_offset` - An optional f64 value specifying the horizontal offset for individual data points when `box_points` is enabled.
    /// * `jitter` - An optional f64 value indicating the amount of jitter (random noise) to apply to individual data points for visibility.
    /// * `opacity` - An optional f64 value specifying the opacity of the plot markers (range: 0.0 to 1.0).
    /// * `colors` - An optional vector of `Rgb` values specifying the colors to be used for the plot.
    /// * `plot_title` - An optional `Text` struct specifying the title of the plot.
    /// * `x_title` - An optional `Text` struct specifying the title of the x-axis.
    /// * `y_title` - An optional `Text` struct specifying the title of the y-axis.
    /// * `legend_title` - An optional `Text` struct specifying the title of the legend.
    ///
    /// # Returns
    ///
    /// Returns an instance of `VerticalBoxPlot`.
    ///
    /// # Example
    ///
    /// ```
    /// VerticalBoxPlot::builder()
    ///     .data(&dataset)
    ///     .x("x variable")
    ///     .y("y variable")
    ///     .group("group")
    ///     .box_points(true)
    ///     .point_offset(-1.5)
    ///     .jitter(0.01)
    ///     .opacity(0.1)
    ///     .colors(vec![
    ///         Rgb(255, 0, 0),
    ///         Rgb(0, 255, 0),
    ///         Rgb(0, 0, 255),
    ///     ])
    ///     .plot_title(
    ///         Text::from("Box Plot")
    ///             .font("Arial")
    ///             .size(18)
    ///     )
    ///     .x_title(
    ///         Text::from("x variable")
    ///             .font("Arial")
    ///             .size(15)
    ///     )
    ///     .y_title(
    ///         Text::from("y variable")
    ///             .font("Arial")
    ///             .size(15)
    ///     )
    ///     .legend_title(
    ///         Text::from("group")
    ///             .font("Arial")
    ///             .size(15)
    ///     )
    ///     .build()
    ///     .plot();
    /// ```
    #[builder]
    pub fn new(
        data: &DataFrame,
        x: String,
        y: String,
        group: Option<String>,
        box_points: Option<bool>,
        point_offset: Option<f64>,
        jitter: Option<f64>,
        // Marker
        opacity: Option<f64>,
        colors: Option<Vec<Rgb>>,
        // Layout
        plot_title: Option<Text>,
        x_title: Option<Text>,
        y_title: Option<Text>,
        legend_title: Option<Text>,
    ) -> Self {
        let x_col = x.as_str();
        let y_col = y.as_str();

        // Layout
        let bar_mode = None;

        let layout = Self::create_layout(bar_mode, plot_title, x_title, y_title, legend_title);

        // Trace
        let error = None;
        let aditional_series = None;

        let size = None;
        let line_types = None;

        let traces = Self::create_traces(
            data,
            x_col,
            y_col,
            group,
            error,
            box_points,
            point_offset,
            jitter,
            aditional_series,
            opacity,
            size,
            colors,
            line_types,
        );

        Self { traces, layout }
    }
}

impl LayoutPlotly for VerticalBoxPlot {}
impl Polar for VerticalBoxPlot {}
impl Mark for VerticalBoxPlot {}
impl Line for VerticalBoxPlot {}

impl Trace for VerticalBoxPlot {
    fn create_trace(
        data: &DataFrame,
        x_col: &str,
        y_col: &str,
        group_name: Option<&str>,
        #[allow(unused_variables)] error: Option<String>,
        box_points: Option<bool>,
        point_offset: Option<f64>,
        jitter: Option<f64>,
        marker: Marker,
        #[allow(unused_variables)] line: LinePlotly,
    ) -> Box<dyn TracePlotly + 'static> {
        let x_data = Self::get_string_column(data, x_col);
        let y_data = Self::get_numeric_column(data, y_col);

        let mut trace = BoxPlot::default().x(x_data).y(y_data);

        if let Some(all) = box_points {
            if all {
                trace = trace.box_points(BoxPoints::All);
            } else {
                trace = trace.box_points(BoxPoints::False);
            }
        }

        if let Some(point_position) = point_offset {
            trace = trace.point_pos(point_position);
        }

        if let Some(jitter) = jitter {
            trace = trace.jitter(jitter);
        }

        trace = trace.marker(marker);

        if let Some(name) = group_name {
            trace = trace.name(name);
        }

        trace
    }
}

impl Plot for VerticalBoxPlot {
    fn get_layout(&self) -> &Layout {
        &self.layout
    }

    fn get_traces(&self) -> &Vec<Box<dyn TracePlotly + 'static>> {
        &self.traces
    }
}

/// A structure representing a horizontal box plot.
pub struct HorizontalBoxPlot {
    traces: Vec<Box<dyn TracePlotly + 'static>>,
    layout: Layout,
}

#[bon]
impl HorizontalBoxPlot {
    /// Creates a new `HorizontalBoxPlot`.
    ///
    /// # Arguments
    ///
    /// * `data` - A reference to the `DataFrame` containing the data to be plotted.
    /// * `x` - A string specifying the column name to be used for the x-axis.
    /// * `y` - A string specifying the column name to be used for the y-axis.
    /// * `group` - An optional string specifying the column name to be used for grouping data points.
    /// * `box_points` - An optional boolean indicating whether individual data points should be plotted along with the box plot.
    /// * `point_offset` - An optional f64 value specifying the horizontal offset for individual data points when `box_points` is enabled.
    /// * `jitter` - An optional f64 value indicating the amount of jitter (random noise) to apply to individual data points for visibility.
    /// * `opacity` - An optional f64 value specifying the opacity of the plot markers (range: 0.0 to 1.0).
    /// * `colors` - An optional vector of `Rgb` values specifying the colors to be used for the plot.
    /// * `plot_title` - An optional `Text` struct specifying the title of the plot.
    /// * `x_title` - An optional `Text` struct specifying the title of the x-axis.
    /// * `y_title` - An optional `Text` struct specifying the title of the y-axis.
    /// * `legend_title` - An optional `Text` struct specifying the title of the legend.
    ///
    /// # Returns
    ///
    /// Returns an instance of `HorizontalBoxPlot`.
    ///
    /// # Example
    ///
    /// ```
    /// HorizontalBoxPlot::builder()
    ///     .data(&dataset)
    ///     .x("x_variable")
    ///     .y("y_variable")
    ///     .group("group")
    ///     .box_points(true)
    ///     .point_offset(-1.5)
    ///     .jitter(0.01)
    ///     .opacity(0.1)
    ///     .colors(vec![
    ///         Rgb(255, 0, 0),
    ///         Rgb(0, 255, 0),
    ///         Rgb(0, 0, 255),
    ///     ])
    ///     .plot_title(
    ///         Text::from("Box Plot")
    ///             .font("Arial")
    ///             .size(18)
    ///     )
    ///     .x_title(
    ///         Text::from("x variable")
    ///             .font("Arial")
    ///             .size(15)
    ///     )
    ///     .y_title(
    ///         Text::from("y variable")
    ///             .font("Arial")
    ///             .size(15)
    ///     )
    ///     .legend_title(
    ///         Text::from("group")
    ///             .font("Arial")
    ///             .size(15)
    ///     )
    ///     .build()
    ///     .plot();
    /// ```
    #[builder]
    pub fn new(
        data: &DataFrame,
        x: String,
        y: String,
        group: Option<String>,
        box_points: Option<bool>,
        point_offset: Option<f64>,
        jitter: Option<f64>,
        // Marker
        opacity: Option<f64>,
        colors: Option<Vec<Rgb>>,
        // Layout
        plot_title: Option<Text>,
        x_title: Option<Text>,
        y_title: Option<Text>,
        legend_title: Option<Text>,
    ) -> Self {
        let x_col = x.as_str();
        let y_col = y.as_str();

        // Layout
        let bar_mode = None;

        let layout = Self::create_layout(bar_mode, plot_title, x_title, y_title, legend_title);

        // Trace
        let error = None;
        let aditional_series = None;

        let size = None;
        let line_type = None;

        let traces = Self::create_traces(
            data,
            x_col,
            y_col,
            group,
            error,
            box_points,
            point_offset,
            jitter,
            aditional_series,
            opacity,
            size,
            colors,
            line_type,
        );

        Self { traces, layout }
    }
}

impl LayoutPlotly for HorizontalBoxPlot {}
impl Polar for HorizontalBoxPlot {}
impl Mark for HorizontalBoxPlot {}
impl Line for HorizontalBoxPlot {}

impl Trace for HorizontalBoxPlot {
    fn create_trace(
        data: &DataFrame,
        x_col: &str,
        y_col: &str,
        group_name: Option<&str>,
        #[allow(unused_variables)] error: Option<String>,
        box_points: Option<bool>,
        point_offset: Option<f64>,
        jitter: Option<f64>,
        marker: Marker,
        #[allow(unused_variables)] line: LinePlotly,
    ) -> Box<dyn TracePlotly + 'static> {
        let x_data = Self::get_numeric_column(data, x_col);
        let y_data = Self::get_string_column(data, y_col);

        let mut trace = BoxPlot::default()
            .x(x_data)
            .y(y_data)
            .orientation(Orientation::Horizontal);

        if let Some(all) = box_points {
            if all {
                trace = trace.box_points(BoxPoints::All);
            } else {
                trace = trace.box_points(BoxPoints::False);
            }
        }

        if let Some(point_position) = point_offset {
            trace = trace.point_pos(point_position);
        }

        if let Some(jitter) = jitter {
            trace = trace.jitter(jitter);
        }

        trace = trace.marker(marker);

        if let Some(name) = group_name {
            trace = trace.name(name);
        }

        trace
    }
}

impl Plot for HorizontalBoxPlot {
    fn get_layout(&self) -> &Layout {
        &self.layout
    }

    fn get_traces(&self) -> &Vec<Box<dyn TracePlotly + 'static>> {
        &self.traces
    }
}
