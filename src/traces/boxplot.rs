//! This module provides implementations for vertical and horizontal box plots using the Plotly library.
//!
//! The `BoxPlot` structs allow for the creation and customization of box plots
//! with various options for data, layout, and aesthetics.

#![allow(deprecated)]

use bon::bon;

use plotly::{
    box_plot::BoxPoints,
    common::{Line as LinePlotly, Marker, Orientation as OrientationPlotly},
    BoxPlot as BoxPlotly, Layout, Trace as TracePlotly,
};

use polars::frame::DataFrame;

use crate::{
    aesthetics::{line::Line, mark::Mark, orientation::Orientation},
    colors::Rgb,
    texts::Text,
    traits::{layout::LayoutPlotly, plot::Plot, polar::Polar, trace::Trace},
    Axis, Legend,
};

/// A structure representing a box plot.
pub struct BoxPlot {
    traces: Vec<Box<dyn TracePlotly + 'static>>,
    layout: Layout,
}

#[bon]
impl BoxPlot {
    /// Creates a new `BoxPlot`.
    ///
    /// # Arguments
    ///
    /// * `data` - A reference to the `DataFrame` containing the data to be plotted.
    /// * `values` - A string specifying the column name to be used for the y-axis (the dependent variable).
    /// * `labels` - A string specifying the column name to be used for the x-axis (the independent variable).
    /// * `orientation` - An optional `Orientation` enum specifying whether the plot should be horizontal or vertical.
    /// * `group` - An optional string specifying the column name to be used for grouping data points.
    /// * `box_points` - An optional boolean indicating whether individual data points should be plotted along with the box plot.
    /// * `point_offset` - An optional f64 value specifying the horizontal offset for individual data points when `box_points` is enabled.
    /// * `jitter` - An optional f64 value indicating the amount of jitter (random noise) to apply to individual data points for visibility.
    /// * `opacity` - An optional f64 value specifying the opacity of the plot markers (range: 0.0 to 1.0).
    /// * `color` - An optional `Rgb` value specifying the color of the markers to be used for the plot.
    /// * `colors` - An optional vector of `Rgb` values specifying the colors to be used for the plot.
    /// * `plot_title` - An optional `Text` struct specifying the title of the plot.
    /// * `x_title` - An optional `Text` struct specifying the title of the x-axis.
    /// * `y_title` - An optional `Text` struct specifying the title of the y-axis.
    /// * `legend_title` - An optional `Text` struct specifying the title of the legend.
    /// * `x_axis` - An optional reference to an `Axis` struct for customizing the x-axis.
    /// * `y_axis` - An optional reference to an `Axis` struct for customizing the y-axis.
    /// * `legend` - An optional reference to a `Legend` struct for customizing the legend of the plot (e.g., positioning, font, etc.).
    ///
    /// # Returns
    ///
    /// Returns an instance of `BoxPlot`.
    ///
    /// **Examples**
    ///
    /// ```
    /// BoxPlot::builder()
    ///     .data(&dataset)
    ///     .labels("species")
    ///     .values("body_mass_g")
    ///     .orientation(Orientation::Vertical)
    ///     .group("gender")
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
    ///         Text::from("Vertical Box Plot")
    ///             .font("Arial")
    ///             .size(18)
    ///     )
    ///     .x_title(
    ///         Text::from("species")
    ///             .font("Arial")
    ///             .size(15)
    ///     )
    ///     .y_title(
    ///         Text::from("body mass (g)")
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
    /// ![Vertical Box Plot](https://imgur.com/0Zn0mFu.png)
    ///
    /// ```
    /// BoxPlot::builder()
    ///     .data(&dataset)
    ///     .labels("species")
    ///     .values("body_mass_g")
    ///     .orientation(Orientation::Horizontal)
    ///     .group("gender")
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
    ///         Text::from("Horizontal Box Plot")
    ///             .font("Arial")
    ///             .size(18)
    ///     )
    ///     .x_title(
    ///         Text::from("body mass (g)")
    ///             .font("Arial")
    ///             .size(15)
    ///     )
    ///     .y_title(
    ///         Text::from("species")
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
    /// ![Horizontal Box Plot](https://imgur.com/Lu92liU.png)
    #[builder(on(String, into), on(Text, into))]
    pub fn new(
        data: &DataFrame,
        values: String,
        labels: String,
        orientation: Option<Orientation>,
        group: Option<String>,
        box_points: Option<bool>,
        point_offset: Option<f64>,
        jitter: Option<f64>,
        // Marker
        opacity: Option<f64>,
        color: Option<Rgb>,
        colors: Option<Vec<Rgb>>,
        // Layout
        plot_title: Option<Text>,
        x_title: Option<Text>,
        y_title: Option<Text>,
        legend_title: Option<Text>,
        x_axis: Option<&Axis>,
        y_axis: Option<&Axis>,
        legend: Option<&Legend>,
    ) -> Self {
        let value_column = values.as_str();
        let label_column = labels.as_str();

        // Layout
        let bar_mode = None;

        let layout = Self::create_layout(
            bar_mode,
            plot_title,
            x_title,
            x_axis,
            y_title,
            y_axis,
            legend_title,
            legend,
        );

        // Trace
        let error = None;
        let additional_series = None;

        let size = None;
        let with_shape = None;
        let shape = None;
        let shapes = None;
        let line_types = None;
        let line_width = None;

        let traces = Self::create_traces(
            data,
            value_column,
            label_column,
            orientation,
            group,
            error,
            box_points,
            point_offset,
            jitter,
            additional_series,
            opacity,
            size,
            color,
            colors,
            with_shape,
            shape,
            shapes,
            line_types,
            line_width,
        );

        Self { traces, layout }
    }
}

impl LayoutPlotly for BoxPlot {}
impl Polar for BoxPlot {}
impl Mark for BoxPlot {}
impl Line for BoxPlot {}

impl Trace for BoxPlot {
    fn create_trace(
        data: &DataFrame,
        x_col: &str,
        y_col: &str,
        orientation: Option<Orientation>,
        group_name: Option<&str>,
        #[allow(unused_variables)] error: Option<String>,
        box_points: Option<bool>,
        point_offset: Option<f64>,
        jitter: Option<f64>,
        #[allow(unused_variables)] with_shape: Option<bool>,
        marker: Marker,
        #[allow(unused_variables)] line: LinePlotly,
    ) -> Box<dyn TracePlotly + 'static> {
        let value_data = Self::get_numeric_column(data, x_col);
        let category_data = Self::get_string_column(data, y_col);

        match orientation {
            Some(orientation) => match orientation {
                Orientation::Vertical => {
                    let mut trace = BoxPlotly::default()
                        .x(category_data)
                        .y(value_data)
                        .orientation(orientation.get_orientation());

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
                Orientation::Horizontal => {
                    let mut trace = BoxPlotly::default()
                        .x(value_data)
                        .y(category_data)
                        .orientation(orientation.get_orientation());

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
            },
            None => {
                let mut trace = BoxPlotly::default().x(category_data).y(value_data);

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
    }
}

impl Plot for BoxPlot {
    fn get_layout(&self) -> &Layout {
        &self.layout
    }

    fn get_traces(&self) -> &Vec<Box<dyn TracePlotly + 'static>> {
        &self.traces
    }
}

#[deprecated(
    since = "0.5.0",
    note = "`VerticalBoxPlot` will be removed in v0.6.0. Please use `BoxPlot` instead."
)]
/// A structure representing a vertical box plot.
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
    /// * `color` - An optional `Rgb` value specifying the color of the markers to be used for the plot.
    /// * `colors` - An optional vector of `Rgb` values specifying the colors to be used for the plot.
    /// * `plot_title` - An optional `Text` struct specifying the title of the plot.
    /// * `x_title` - An optional `Text` struct specifying the title of the x-axis.
    /// * `y_title` - An optional `Text` struct specifying the title of the y-axis.
    /// * `legend_title` - An optional `Text` struct specifying the title of the legend.
    /// * `x_axis` - An optional reference to an `Axis` struct for customizing the x-axis.
    /// * `y_axis` - An optional reference to an `Axis` struct for customizing the y-axis.
    /// * `legend` - An optional reference to a `Legend` struct for customizing the legend of the plot (e.g., positioning, font, etc.).
    ///
    /// # Returns
    ///
    /// Returns an instance of `VerticalBoxPlot`.
    ///
    /// **Example**
    ///
    /// ```
    /// VerticalBoxPlot::builder()
    ///     .data(&dataset)
    ///     .x("species")
    ///     .y("body_mass_g")
    ///     .group("gender")
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
    ///         Text::from("Vertical Box Plot")
    ///             .font("Arial")
    ///             .size(18)
    ///     )
    ///     .x_title(
    ///         Text::from("species")
    ///             .font("Arial")
    ///             .size(15)
    ///     )
    ///     .y_title(
    ///         Text::from("body mass (g)")
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
    /// ![Vertical Box Plot](https://imgur.com/0Zn0mFu.png)
    #[builder(on(String, into), on(Text, into))]
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
        color: Option<Rgb>,
        colors: Option<Vec<Rgb>>,
        // Layout
        plot_title: Option<Text>,
        x_title: Option<Text>,
        y_title: Option<Text>,
        legend_title: Option<Text>,
        x_axis: Option<&Axis>,
        y_axis: Option<&Axis>,
        legend: Option<&Legend>,
    ) -> Self {
        let x_col = x.as_str();
        let y_col = y.as_str();

        // Layout
        let bar_mode = None;

        let layout = Self::create_layout(
            bar_mode,
            plot_title,
            x_title,
            x_axis,
            y_title,
            y_axis,
            legend_title,
            legend,
        );

        // Trace
        let orientation = None;
        let error = None;
        let additional_series = None;

        let size = None;
        let with_shape = None;
        let shape = None;
        let shapes = None;
        let line_types = None;
        let line_width = None;

        let traces = Self::create_traces(
            data,
            x_col,
            y_col,
            orientation,
            group,
            error,
            box_points,
            point_offset,
            jitter,
            additional_series,
            opacity,
            size,
            color,
            colors,
            with_shape,
            shape,
            shapes,
            line_types,
            line_width,
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
        #[allow(unused_variables)] orientation: Option<Orientation>,
        group_name: Option<&str>,
        #[allow(unused_variables)] error: Option<String>,
        box_points: Option<bool>,
        point_offset: Option<f64>,
        jitter: Option<f64>,
        #[allow(unused_variables)] with_shape: Option<bool>,
        marker: Marker,
        #[allow(unused_variables)] line: LinePlotly,
    ) -> Box<dyn TracePlotly + 'static> {
        let x_data = Self::get_string_column(data, x_col);
        let y_data = Self::get_numeric_column(data, y_col);

        let mut trace = BoxPlotly::default().x(x_data).y(y_data);

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

#[deprecated(
    since = "0.5.0",
    note = "`HorizontalBoxPlot` will be removed in v0.6.0. Please use `BoxPlot` instead."
)]
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
    /// * `color` - An optional `Rgb` value specifying the color of the markers to be used for the plot.
    /// * `colors` - An optional vector of `Rgb` values specifying the colors to be used for the plot.
    /// * `plot_title` - An optional `Text` struct specifying the title of the plot.
    /// * `x_title` - An optional `Text` struct specifying the title of the x-axis.
    /// * `y_title` - An optional `Text` struct specifying the title of the y-axis.
    /// * `legend_title` - An optional `Text` struct specifying the title of the legend.
    /// * `x_axis` - An optional reference to an `Axis` struct for customizing the x-axis.
    /// * `y_axis` - An optional reference to an `Axis` struct for customizing the y-axis.
    /// * `legend` - An optional reference to a `Legend` struct for customizing the legend of the plot (e.g., positioning, font, etc.).
    ///
    /// # Returns
    ///
    /// Returns an instance of `HorizontalBoxPlot`.
    ///
    /// **Example**
    ///
    /// ```
    /// HorizontalBoxPlot::builder()
    ///     .data(&dataset)
    ///     .x("body_mass_g")
    ///     .y("species")
    ///     .group("gender")
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
    ///         Text::from("Horizontal Box Plot")
    ///             .font("Arial")
    ///             .size(18)
    ///     )
    ///     .x_title(
    ///         Text::from("body mass (g)")
    ///             .font("Arial")
    ///             .size(15)
    ///     )
    ///     .y_title(
    ///         Text::from("species")
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
    /// ![Horizontal Box Plot](https://imgur.com/Lu92liU.png)
    #[builder(on(String, into), on(Text, into))]
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
        color: Option<Rgb>,
        colors: Option<Vec<Rgb>>,
        // Layout
        plot_title: Option<Text>,
        x_title: Option<Text>,
        y_title: Option<Text>,
        legend_title: Option<Text>,
        x_axis: Option<&Axis>,
        y_axis: Option<&Axis>,
        legend: Option<&Legend>,
    ) -> Self {
        let x_col = x.as_str();
        let y_col = y.as_str();

        // Layout
        let bar_mode = None;

        let layout = Self::create_layout(
            bar_mode,
            plot_title,
            x_title,
            x_axis,
            y_title,
            y_axis,
            legend_title,
            legend,
        );

        // Trace
        let orientation = None;
        let error = None;
        let additional_series = None;

        let size = None;
        let with_shape = None;
        let shape = None;
        let shapes = None;
        let line_type = None;
        let line_width = None;

        let traces = Self::create_traces(
            data,
            x_col,
            y_col,
            orientation,
            group,
            error,
            box_points,
            point_offset,
            jitter,
            additional_series,
            opacity,
            size,
            color,
            colors,
            with_shape,
            shape,
            shapes,
            line_type,
            line_width,
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
        #[allow(unused_variables)] orientation: Option<Orientation>,
        group_name: Option<&str>,
        #[allow(unused_variables)] error: Option<String>,
        box_points: Option<bool>,
        point_offset: Option<f64>,
        jitter: Option<f64>,
        #[allow(unused_variables)] with_shape: Option<bool>,
        marker: Marker,
        #[allow(unused_variables)] line: LinePlotly,
    ) -> Box<dyn TracePlotly + 'static> {
        let x_data = Self::get_numeric_column(data, x_col);
        let y_data = Self::get_string_column(data, y_col);

        let mut trace = BoxPlotly::default()
            .x(x_data)
            .y(y_data)
            .orientation(OrientationPlotly::Horizontal);

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
