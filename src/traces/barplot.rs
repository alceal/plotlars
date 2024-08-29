//! This module provides implementations for vertical and horizontal bar plots using the Plotly library.
//!
//! The `VerticalBarPlot` and `HorizontalBarPlot` structs allow for the creation and customization of bar plots
//! with various options for data, layout, and aesthetics.

use bon::bon;

use plotly::{
    common::{ErrorData, ErrorType, Line as LinePlotly, Marker, Orientation},
    layout::BarMode,
    Bar, Layout, Trace as TracePlotly,
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
pub struct VerticalBarPlot {
    traces: Vec<Box<dyn TracePlotly + 'static>>,
    layout: Layout,
}

#[bon]
impl VerticalBarPlot {
    /// Creates a new `VerticalBarPlot`.
    ///
    /// # Arguments
    ///
    /// * `data` - A reference to the `DataFrame` containing the data to be plotted.
    /// * `x` - A string specifying the column name to be used for the x-axis.
    /// * `y` - A string specifying the column name to be used for the y-axis.
    /// * `group` - An optional string specifying the column name to be used for grouping data points.
    /// * `error` - An optional string specifying the column name containing error values for the y-axis data.
    /// * `colors` - An optional vector of `Rgb` values specifying the colors to be used for the plot.
    /// * `plot_title` - An optional `Text` struct specifying the title of the plot.
    /// * `x_title` - An optional `Text` struct specifying the title of the x-axis.
    /// * `y_title` - An optional `Text` struct specifying the title of the y-axis.
    /// * `legend_title` - An optional `Text` struct specifying the title of the legend.
    ///
    /// # Returns
    ///
    /// Returns an instance of `VerticalBarPlot`.
    ///
    /// # Example
    ///
    /// ```
    /// VerticalBarPlot::builder()
    ///     .data(&dataset)
    ///     .x("animals")
    ///     .y("values")
    ///     .group("gender")
    ///     .error("errors")
    ///     .colors(vec![
    ///         Rgb(255, 0, 0),
    ///     ])
    ///     .plot_title(
    ///         Text::from("Vertical Bar Plot")
    ///             .font("Arial")
    ///             .size(18)
    ///     )
    ///     .x_title(
    ///         Text::from("animal")
    ///             .font("Arial")
    ///             .size(15)
    ///     )
    ///     .y_title(
    ///         Text::from("value")
    ///             .font("Arial")
    ///             .size(15)
    ///     )
    ///     .legend_title(
    ///         Text::from("gender")
    ///             .font("Arial")
    ///             .size(15)
    ///     )
    ///     .build()
    ///     .plot();
    /// ```
    ///
    /// ![Vertical Bar Plot](https://imgur.com/Fd6DpB0.png)
    ///
    #[builder(on(String, into), on(Text, into))]
    pub fn new(
        data: &DataFrame,
        x: String,
        y: String,
        group: Option<String>,
        error: Option<String>,
        // Marker
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
        let bar_mode = Some(BarMode::Group);

        let layout = Self::create_layout(bar_mode, plot_title, x_title, y_title, legend_title);

        // Trace
        let box_points = None;
        let point_offset = None;
        let jitter = None;
        let aditional_series = None;

        let opacity = None;
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

impl LayoutPlotly for VerticalBarPlot {}
impl Polar for VerticalBarPlot {}
impl Mark for VerticalBarPlot {}
impl Line for VerticalBarPlot {}

impl Trace for VerticalBarPlot {
    fn create_trace(
        data: &DataFrame,
        x_col: &str,
        y_col: &str,
        group_name: Option<&str>,
        error: Option<String>,
        #[allow(unused_variables)] box_points: Option<bool>,
        #[allow(unused_variables)] point_offset: Option<f64>,
        #[allow(unused_variables)] jitter: Option<f64>,
        marker: Marker,
        #[allow(unused_variables)] line: LinePlotly,
    ) -> Box<dyn TracePlotly + 'static> {
        let x_data = Self::get_string_column(data, x_col);
        let y_data = Self::get_numeric_column(data, y_col);

        let mut trace = Bar::default().x(x_data).y(y_data);

        if let Some(error) = error {
            let error = Self::get_numeric_column(data, error.as_str())
                .iter()
                .map(|x| x.unwrap() as f64)
                .collect::<Vec<_>>();

            trace = trace.error_y(ErrorData::new(ErrorType::Data).array(error))
        }

        trace = trace.marker(marker);

        if let Some(name) = group_name {
            trace = trace.name(name);
        }

        trace
    }
}

impl Plot for VerticalBarPlot {
    fn get_layout(&self) -> &Layout {
        &self.layout
    }

    fn get_traces(&self) -> &Vec<Box<dyn TracePlotly + 'static>> {
        &self.traces
    }
}

/// A structure representing a horizontal bar plot.
pub struct HorizontalBarPlot {
    traces: Vec<Box<dyn TracePlotly + 'static>>,
    layout: Layout,
}

#[bon]
impl HorizontalBarPlot {
    /// Creates a new `HorizontalBarPlot`.
    ///
    /// # Arguments
    ///
    /// * `data` - A reference to the `DataFrame` containing the data to be plotted.
    /// * `x` - A string specifying the column name to be used for the x-axis.
    /// * `y` - A string specifying the column name to be used for the y-axis.
    /// * `group` - An optional string specifying the column name to be used for grouping data points.
    /// * `error` - An optional string specifying the column name containing error values for the x-axis data.
    /// * `colors` - An optional vector of `Rgb` values specifying the colors to be used for the plot.
    /// * `plot_title` - An optional `Text` struct specifying the title of the plot.
    /// * `x_title` - An optional `Text` struct specifying the title of the x-axis.
    /// * `y_title` - An optional `Text` struct specifying the title of the y-axis.
    /// * `legend_title` - An optional `Text` struct specifying the title of the legend.
    ///
    /// # Returns
    ///
    /// Returns an instance of `HorizontalBarPlot`.
    ///
    /// # Example
    ///
    /// ```
    /// HorizontalBarPlot::builder()
    ///     .data(&dataset)
    ///     .x("values")
    ///     .y("animals")
    ///     .group("gender")
    ///     .error("errors")
    ///     .colors(vec![
    ///         Rgb(255, 0, 0),
    ///     ])
    ///     .plot_title(
    ///         Text::from("Horizontal Bar Plot")
    ///             .font("Arial")
    ///             .size(18)
    ///     )
    ///     .x_title(
    ///         Text::from("value")
    ///             .font("Arial")
    ///             .size(15)
    ///     )
    ///     .y_title(
    ///         Text::from("animal")
    ///             .font("Arial")
    ///             .size(15)
    ///     )
    ///     .legend_title(
    ///         Text::from("gender")
    ///             .font("Arial")
    ///             .size(15)
    ///     )
    ///     .build()
    ///     .plot();
    /// ```
    ///
    /// ![Horizontal Bar Plot](https://imgur.com/saoTcNg.png)
    ///
    #[builder(on(String, into), on(Text, into))]
    pub fn new(
        data: &DataFrame,
        x: String,
        y: String,
        group: Option<String>,
        error: Option<String>,
        // Marker
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
        let bar_mode = Some(BarMode::Group);

        let layout = Self::create_layout(bar_mode, plot_title, x_title, y_title, legend_title);

        // Trace
        let box_points = None;
        let point_offset = None;
        let jitter = None;
        let aditional_series = None;

        let opacity = None;
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

impl LayoutPlotly for HorizontalBarPlot {}
impl Polar for HorizontalBarPlot {}
impl Mark for HorizontalBarPlot {}
impl Line for HorizontalBarPlot {}

impl Trace for HorizontalBarPlot {
    fn create_trace(
        data: &DataFrame,
        x_col: &str,
        y_col: &str,
        group_name: Option<&str>,
        error: Option<String>,
        #[allow(unused_variables)] box_points: Option<bool>,
        #[allow(unused_variables)] point_offset: Option<f64>,
        #[allow(unused_variables)] jitter: Option<f64>,
        marker: Marker,
        #[allow(unused_variables)] line: LinePlotly,
    ) -> Box<dyn TracePlotly + 'static> {
        let x_data = Self::get_numeric_column(data, x_col);
        let y_data = Self::get_string_column(data, y_col);

        let mut trace = Bar::default()
            .x(x_data)
            .y(y_data)
            .orientation(Orientation::Horizontal);

        if let Some(error) = error {
            let error = Self::get_numeric_column(data, error.as_str())
                .iter()
                .map(|x| x.unwrap() as f64)
                .collect::<Vec<_>>();

            trace = trace.error_x(ErrorData::new(ErrorType::Data).array(error))
        }

        trace = trace.marker(marker);

        if let Some(name) = group_name {
            trace = trace.name(name);
        }

        trace
    }
}

impl Plot for HorizontalBarPlot {
    fn get_layout(&self) -> &Layout {
        &self.layout
    }

    fn get_traces(&self) -> &Vec<Box<dyn TracePlotly + 'static>> {
        &self.traces
    }
}
