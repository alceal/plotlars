//! This module provides implementations for box plots using the Plotly library.
//!
//! The `BoxPlot` structs allow for the creation and customization of box plots
//! with various options for data, layout, and aesthetics.

use bon::bon;

use plotly::{
    box_plot::BoxPoints,
    common::{Line as LinePlotly, Marker},
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
    /// * `x_axis` - An optional reference to an `Axis` struct for customizing the x-axis.
    /// * `y_axis` - An optional reference to an `Axis` struct for customizing the y-axis.
    /// * `legend_title` - An optional `Text` struct specifying the title of the legend.
    /// * `legend` - An optional reference to a `Legend` struct for customizing the legend of the plot (e.g., positioning, font, etc.).
    ///
    /// # Returns
    ///
    /// Returns an instance of `BoxPlot`.
    ///
    /// **Example**
    ///
    /// ```
    /// let axis_format = Axis::new()
    ///     .value_thousands(true);
    ///
    /// let legend_format = Legend::new()
    ///     .border_width(1)
    ///     .x(0.9);
    ///
    /// BoxPlot::builder()
    ///     .data(&scatterplot_dataset)
    ///     .labels("species")
    ///     .values("body_mass_g")
    ///     .orientation(Orientation::Vertical)
    ///     .group("gender")
    ///     .box_points(true)
    ///     .point_offset(-1.5)
    ///     .jitter(0.01)
    ///     .opacity(0.1)
    ///     .colors(vec![
    ///         Rgb(0, 191, 255),
    ///         Rgb(57, 255, 20),
    ///         Rgb(255, 105, 180),
    ///     ])
    ///     .plot_title(
    ///         Text::from("Box Plot")
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
    ///     .y_axis(&axis_format)
    ///     .legend_title(
    ///         Text::from("gender")
    ///             .font("Arial")
    ///             .size(15)
    ///     )
    ///     .legend(&legend_format)
    ///     .build()
    ///     .plot();
    /// ```
    ///
    /// ![Box Plot](https://imgur.com/uj1LY90.png)
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
