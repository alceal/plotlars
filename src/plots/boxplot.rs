use bon::bon;

use plotly::{
    box_plot::BoxPoints, common::Marker as MarkerPlotly, layout::BoxMode, BoxPlot as BoxPlotly,
    Layout as LayoutPlotly, Trace,
};

use polars::frame::DataFrame;

use crate::{
    common::{Layout, Marker, Plot, Polar},
    components::{Axis, Legend, Orientation, Rgb, Text},
};

/// A structure representing a box plot.
///
/// The `BoxPlot` struct facilitates the creation and customization of box plots with various options
/// for data selection, layout configuration, and aesthetic adjustments. It supports both horizontal
/// and vertical orientations, grouping of data, display of individual data points with jitter and offset,
/// opacity settings, and customizable markers and colors.
///
/// # Arguments
///
/// * `data` - A reference to the `DataFrame` containing the data to be plotted.
/// * `labels` - A string slice specifying the column name to be used for the x-axis (independent variable).
/// * `values` - A string slice specifying the column name to be used for the y-axis (dependent variable).
/// * `orientation` - An optional `Orientation` enum specifying whether the plot should be horizontal or vertical.
/// * `group` - An optional string slice specifying the column name to be used for grouping data points.
/// * `box_points` - An optional boolean indicating whether individual data points should be plotted along with the box plot.
/// * `point_offset` - An optional `f64` value specifying the horizontal offset for individual data points when `box_points` is enabled.
/// * `jitter` - An optional `f64` value indicating the amount of jitter (random noise) to apply to individual data points for visibility.
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
/// use plotlars::{Axis, BoxPlot, Legend, Orientation, Plot, Rgb, Text};
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
///     .legend_title(
///         Text::from("gender")
///             .font("Arial")
///             .size(15)
///     )
///     .y_axis(
///         &Axis::new()
///             .value_thousands(true)
///     )
///     .legend(
///         &Legend::new()
///             .border_width(1)
///             .x(0.9)
///     )
///     .build()
///     .plot();
/// ```
///
/// ![Example](https://imgur.com/uj1LY90.png)
pub struct BoxPlot {
    traces: Vec<Box<dyn Trace + 'static>>,
    layout: LayoutPlotly,
}

#[bon]
impl BoxPlot {
    #[builder(on(String, into), on(Text, into))]
    pub fn new(
        data: &DataFrame,
        labels: &str,
        values: &str,
        orientation: Option<Orientation>,
        group: Option<&str>,
        box_points: Option<bool>,
        point_offset: Option<f64>,
        jitter: Option<f64>,
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

        layout = layout.box_mode(BoxMode::Group);

        let traces = Self::create_traces(
            data,
            labels,
            values,
            orientation,
            group,
            box_points,
            point_offset,
            jitter,
            opacity,
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
        box_points: Option<bool>,
        point_offset: Option<f64>,
        jitter: Option<f64>,
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

                    let trace = Self::create_trace(
                        &subset,
                        labels,
                        values,
                        orientation.clone(),
                        Some(group),
                        box_points,
                        point_offset,
                        jitter,
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

                let trace = Self::create_trace(
                    data,
                    labels,
                    values,
                    orientation,
                    group,
                    box_points,
                    point_offset,
                    jitter,
                    marker,
                );

                traces.push(trace);
            }
        }

        traces
    }

    #[allow(clippy::too_many_arguments)]
    fn create_trace(
        data: &DataFrame,
        labels: &str,
        values: &str,
        orientation: Option<Orientation>,
        group_name: Option<&str>,
        box_points: Option<bool>,
        point_offset: Option<f64>,
        jitter: Option<f64>,
        marker: MarkerPlotly,
    ) -> Box<dyn Trace + 'static> {
        let category_data = Self::get_string_column(data, labels);
        let value_data = Self::get_numeric_column(data, values);

        let orientation = orientation.unwrap_or(Orientation::Vertical);

        match orientation {
            Orientation::Vertical => {
                let mut trace = BoxPlotly::default()
                    .x(category_data)
                    .y(value_data)
                    .orientation(orientation.to_plotly());

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
                    .orientation(orientation.to_plotly());

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

impl Layout for BoxPlot {}
impl Marker for BoxPlot {}
impl Polar for BoxPlot {}

impl Plot for BoxPlot {
    fn get_layout(&self) -> &LayoutPlotly {
        &self.layout
    }

    fn get_traces(&self) -> &Vec<Box<dyn Trace + 'static>> {
        &self.traces
    }
}
