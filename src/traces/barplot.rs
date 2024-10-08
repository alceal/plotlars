//! This module provides implementations for bar plots using the Plotly library.
//!
//! The `BarPlot` struct allow for the creation and customization of bar plots
//! with various options for data, layout, and aesthetics.

use bon::bon;

use plotly::{
    common::{ErrorData, ErrorType, Line as LinePlotly, Marker},
    layout::BarMode,
    Bar, Layout, Trace as TracePlotly,
};

use polars::frame::DataFrame;

use crate::{
    aesthetics::{line::Line, mark::Mark},
    colors::Rgb,
    texts::Text,
    traits::{layout::LayoutPlotly, plot::Plot, polar::Polar, trace::Trace},
    Axis, Legend, Orientation,
};

/// A structure representing a bar plot.
pub struct BarPlot {
    traces: Vec<Box<dyn TracePlotly + 'static>>,
    layout: Layout,
}

#[bon]
impl BarPlot {
    /// Creates a new `BarPlot`.
    ///
    /// # Arguments
    ///
    /// * `data` - A reference to the `DataFrame` containing the data to be plotted.
    /// * `values` - A string specifying the column name to be used for the y-axis (the dependent variable).
    /// * `labels` - A string specifying the column name to be used for the x-axis (the independent variable).
    /// * `orientation` - An optional `Orientation` enum specifying whether the plot should be horizontal or vertical.
    /// * `group` - An optional string specifying the column name to be used for grouping data points.
    /// * `error` - An optional string specifying the column name containing error values for the y-axis data.
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
    /// Returns an instance of `BarPlot`.
    ///
    /// **Example**
    ///
    /// ```
    /// let legend = Legend::new()
    ///     .orientation(Orientation::Horizontal)
    ///     .y(1.0)
    ///     .x(0.4);
    ///
    /// BarPlot::builder()
    ///     .data(&barplot_dataset)
    ///     .labels("animals")
    ///     .values("values")
    ///     .orientation(Orientation::Vertical)
    ///     .group("gender")
    ///     .error("errors")
    ///     .colors(vec![Rgb(255, 127, 80), Rgb(64, 224, 208)])
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
    ///     .legend(&legend)
    ///     .build()
    ///     .plot();
    /// ```
    ///
    /// ![Bar Plot](https://imgur.com/2alZlO5.png)
    #[builder(on(String, into), on(Text, into))]
    pub fn new(
        data: &DataFrame,
        values: String,
        labels: String,
        orientation: Option<Orientation>,
        group: Option<String>,
        error: Option<String>,
        // Marker
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
        let bar_mode = Some(BarMode::Group);

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
        let box_points = None;
        let point_offset = None;
        let jitter = None;
        let additional_series = None;

        let opacity = None;
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

impl LayoutPlotly for BarPlot {}
impl Polar for BarPlot {}
impl Mark for BarPlot {}
impl Line for BarPlot {}

impl Trace for BarPlot {
    fn create_trace(
        data: &DataFrame,
        x_col: &str,
        y_col: &str,
        orientation: Option<Orientation>,
        group_name: Option<&str>,
        error: Option<String>,
        #[allow(unused_variables)] box_points: Option<bool>,
        #[allow(unused_variables)] point_offset: Option<f64>,
        #[allow(unused_variables)] jitter: Option<f64>,
        #[allow(unused_variables)] with_shape: Option<bool>,
        marker: Marker,
        #[allow(unused_variables)] line: LinePlotly,
    ) -> Box<dyn TracePlotly + 'static> {
        let value_data = Self::get_numeric_column(data, x_col);
        let category_data = Self::get_string_column(data, y_col);

        match orientation {
            Some(orientation) => match orientation {
                Orientation::Vertical => {
                    let mut trace = Bar::default()
                        .x(category_data)
                        .y(value_data)
                        .orientation(orientation.get_orientation());

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
                Orientation::Horizontal => {
                    let mut trace = Bar::default()
                        .x(value_data)
                        .y(category_data)
                        .orientation(orientation.get_orientation());

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
            },
            None => {
                let mut trace = Bar::default().x(category_data).y(value_data);

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
    }
}

impl Plot for BarPlot {
    fn get_layout(&self) -> &Layout {
        &self.layout
    }

    fn get_traces(&self) -> &Vec<Box<dyn TracePlotly + 'static>> {
        &self.traces
    }
}
