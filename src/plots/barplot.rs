use bon::bon;

use plotly::{
    Bar, Layout as LayoutPlotly, Trace,
    common::{ErrorData, ErrorType, Marker as MarkerPlotly},
    layout::BarMode,
};

use polars::frame::DataFrame;
use serde::Serialize;

use crate::{
    common::{Layout, Marker, PlotHelper, Polar},
    components::{Axis, Legend, Orientation, Rgb, Text},
};

/// A structure representing a bar plot.
///
/// The `BarPlot` struct allows for the creation and customization of bar plots with various options
/// for data, layout, and aesthetics. It supports both vertical and horizontal orientations, grouping
/// of data, error bars, and customizable markers and colors.
///
/// # Arguments
///
/// * `data` - A reference to the `DataFrame` containing the data to be plotted.
/// * `labels` - A string slice specifying the column name to be used for the x-axis (independent variable).
/// * `values` - A string slice specifying the column name to be used for the y-axis (dependent variable).
/// * `orientation` - An optional `Orientation` enum specifying whether the plot should be horizontal or vertical.
/// * `group` - An optional string slice specifying the column name to be used for grouping data points.
/// * `error` - An optional string slice specifying the column name containing error values for the y-axis data.
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
/// use plotlars::{BarPlot, Legend, Orientation, Plot, Rgb, Text};
///
/// let dataset = df![
///         "animal" => &["giraffe", "giraffe", "orangutan", "orangutan", "monkey", "monkey"],
///         "gender" => &vec!["female", "male", "female", "male", "female", "male"],
///         "value" => &vec![20.0f32, 25.0, 14.0, 18.0, 23.0, 31.0],
///         "error" => &vec![1.0, 0.5, 1.5, 1.0, 0.5, 1.5],
///     ]
///     .unwrap();
///
/// BarPlot::builder()
///     .data(&dataset)
///     .labels("animal")
///     .values("value")
///     .orientation(Orientation::Vertical)
///     .group("gender")
///     .error("error")
///     .colors(vec![
///         Rgb(255, 127, 80),
///         Rgb(64, 224, 208),
///     ])
///     .plot_title(
///         Text::from("Bar Plot")
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
///     .legend(
///         &Legend::new()
///             .orientation(Orientation::Horizontal)
///             .y(1.0)
///             .x(0.4)
///     )
///     .build()
///     .plot();
/// ```
///
/// ![Example](https://imgur.com/2alZlO5.png)
#[derive(Clone, Serialize)]
pub struct BarPlot {
    traces: Vec<Box<dyn Trace + 'static>>,
    layout: LayoutPlotly,
}

#[bon]
impl BarPlot {
    #[builder(on(String, into), on(Text, into))]
    pub fn new(
        data: &DataFrame,
        labels: &str,
        values: &str,
        orientation: Option<Orientation>,
        group: Option<&str>,
        error: Option<&str>,
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
        let y2_title = None;
        let z_title = None;
        let y2_axis = None;
        let z_axis = None;

        let mut layout = Self::create_layout(
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

        layout = layout.bar_mode(BarMode::Group);

        let traces = Self::create_traces(
            data,
            labels,
            values,
            orientation,
            group,
            error,
            color,
            colors,
        );

        Self { traces, layout }
    }

    #[allow(clippy::too_many_arguments)]
    fn create_traces(
        data: &DataFrame,
        labels: &str,
        values: &str,
        orientation: Option<Orientation>,
        group: Option<&str>,
        error: Option<&str>,
        color: Option<Rgb>,
        colors: Option<Vec<Rgb>>,
    ) -> Vec<Box<dyn Trace + 'static>> {
        let mut traces: Vec<Box<dyn Trace + 'static>> = Vec::new();

        let opacity = None;
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

                    let trace = Self::create_trace(
                        &subset,
                        labels,
                        values,
                        orientation.clone(),
                        Some(group),
                        error,
                        marker,
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

                let trace =
                    Self::create_trace(data, labels, values, orientation, group, error, marker);

                traces.push(trace);
            }
        }

        traces
    }

    fn create_trace(
        data: &DataFrame,
        labels: &str,
        values: &str,
        orientation: Option<Orientation>,
        group: Option<&str>,
        error: Option<&str>,
        marker: MarkerPlotly,
    ) -> Box<dyn Trace + 'static> {
        let values = Self::get_numeric_column(data, values);
        let labels = Self::get_string_column(data, labels);

        let orientation = orientation.unwrap_or(Orientation::Vertical);

        match orientation {
            Orientation::Vertical => {
                let mut trace = Bar::default()
                    .x(labels)
                    .y(values)
                    .orientation(orientation.to_plotly());

                if let Some(error) = error {
                    let error = Self::get_numeric_column(data, error)
                        .iter()
                        .map(|x| x.unwrap() as f64)
                        .collect::<Vec<_>>();

                    trace = trace.error_y(ErrorData::new(ErrorType::Data).array(error))
                }

                trace = trace.marker(marker);

                if let Some(group) = group {
                    trace = trace.name(group);
                }

                trace
            }
            Orientation::Horizontal => {
                let mut trace = Bar::default()
                    .x(values)
                    .y(labels)
                    .orientation(orientation.to_plotly());

                if let Some(error) = error {
                    let error = Self::get_numeric_column(data, error)
                        .iter()
                        .map(|x| x.unwrap() as f64)
                        .collect::<Vec<_>>();

                    trace = trace.error_x(ErrorData::new(ErrorType::Data).array(error))
                }

                trace = trace.marker(marker);

                if let Some(group) = group {
                    trace = trace.name(group);
                }

                trace
            }
        }
    }
}

impl Layout for BarPlot {}
impl Marker for BarPlot {}
impl Polar for BarPlot {}

impl PlotHelper for BarPlot {
    fn get_layout(&self) -> &LayoutPlotly {
        &self.layout
    }

    fn get_traces(&self) -> &Vec<Box<dyn Trace + 'static>> {
        &self.traces
    }
}
