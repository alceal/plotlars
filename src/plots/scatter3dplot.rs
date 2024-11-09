use bon::bon;

use plotly::{
    common::{Marker as MarkerPlotly, Mode},
    Layout as LayoutPlotly, Scatter3D, Trace,
};

use polars::frame::DataFrame;
use serde::Serialize;

use crate::{
    common::{Layout, Marker, PlotHelper, Polar},
    components::{Axis, Legend, Text},
    Rgb, Shape,
};

/// A structure representing a 3D scatter plot.
///
/// The `Scatter3dPlot` struct is designed to create and customize 3D scatter plots with options for data selection,
/// grouping, layout configuration, and aesthetic adjustments. It supports visual differentiation in data groups
/// through varied marker shapes, colors, sizes, opacity levels, and comprehensive layout customization, including
/// titles, axis labels, and legends.
///
/// # Arguments
///
/// * `data` - A reference to the `DataFrame` containing the data to be plotted.
/// * `x` - A string slice specifying the column name to be used for the x-axis (independent variable).
/// * `y` - A string slice specifying the column name to be used for the y-axis (dependent variable).
/// * `z` - A string slice specifying the column name to be used for the z-axis, adding a third dimension to the scatter plot.
/// * `group` - An optional string slice specifying the column name used for grouping data points by category.
/// * `opacity` - An optional `f64` value specifying the opacity of the plot markers (range: 0.0 to 1.0).
/// * `size` - An optional `usize` specifying the size of the markers.
/// * `color` - An optional `Rgb` value for marker color when `group` is not specified.
/// * `colors` - An optional vector of `Rgb` values specifying colors for markers when `group` is specified, enhancing group distinction.
/// * `shape` - An optional `Shape` specifying the shape of markers when `group` is not specified.
/// * `shapes` - An optional vector of `Shape` values defining multiple marker shapes for different groups.
/// * `plot_title` - An optional `Text` struct specifying the plot title.
/// * `x_title` - An optional `Text` struct for the x-axis title.
/// * `y_title` - An optional `Text` struct for the y-axis title.
/// * `z_title` - An optional `Text` struct for the z-axis title.
/// * `legend_title` - An optional `Text` struct specifying the legend title.
/// * `x_axis` - An optional reference to an `Axis` struct for custom x-axis settings.
/// * `y_axis` - An optional reference to an `Axis` struct for custom y-axis settings.
/// * `z_axis` - An optional reference to an `Axis` struct for custom z-axis settings, adding depth perspective.
/// * `legend` - An optional reference to a `Legend` struct for legend customization, including position and font settings.
///
/// # Example
///
/// ```rust
/// use plotlars::{Legend, Plot, Rgb, Scatter3dPlot, Shape};
///
/// let dataset = LazyCsvReader::new("data/penguins.csv")
///     .finish()
///     .unwrap()
///     .select([
///         col("species"),
///         col("sex").alias("gender"),
///         col("bill_length_mm").cast(DataType::Float32),
///         col("flipper_length_mm").cast(DataType::Int16),
///         col("body_mass_g").cast(DataType::Int16),
///     ])
///     .collect()
///     .unwrap();
///
/// Scatter3dPlot::builder()
///     .data(&dataset)
///     .x("body_mass_g")
///     .y("flipper_length_mm")
///     .z("bill_length_mm")
///     .group("species")
///     .opacity(0.25)
///     .size(8)
///     .colors(vec![
///         Rgb(178, 34, 34),
///         Rgb(65, 105, 225),
///         Rgb(255, 140, 0),
///     ])
///     .shapes(vec![
///         Shape::Circle,
///         Shape::Square,
///         Shape::Diamond,
///     ])
///     .plot_title("Scatter Plot")
///     .legend(
///         &Legend::new()
///             .x(0.6)
///     )
///     .build()
///     .plot();
/// ```
///
/// ![Example](https://imgur.com/BXlxKfg.png)
#[derive(Clone, Serialize)]
pub struct Scatter3dPlot {
    traces: Vec<Box<dyn Trace + 'static>>,
    layout: LayoutPlotly,
}

#[bon]
impl Scatter3dPlot {
    #[builder(on(String, into), on(Text, into))]
    pub fn new(
        data: &DataFrame,
        x: &str,
        y: &str,
        z: &str,
        group: Option<&str>,
        opacity: Option<f64>,
        size: Option<usize>,
        color: Option<Rgb>,
        colors: Option<Vec<Rgb>>,
        shape: Option<Shape>,
        shapes: Option<Vec<Shape>>,
        plot_title: Option<Text>,
        x_title: Option<Text>,
        y_title: Option<Text>,
        z_title: Option<Text>,
        legend_title: Option<Text>,
        x_axis: Option<&Axis>,
        y_axis: Option<&Axis>,
        z_axis: Option<&Axis>,
        legend: Option<&Legend>,
    ) -> Self {
        let layout = Self::create_layout(
            plot_title,
            x_title,
            y_title,
            z_title,
            legend_title,
            x_axis,
            y_axis,
            z_axis,
            legend,
        );

        let traces = Self::create_traces(
            data, x, y, z, group, opacity, size, color, colors, shape, shapes,
        );

        Self { traces, layout }
    }

    #[allow(clippy::too_many_arguments)]
    fn create_traces(
        data: &DataFrame,
        x: &str,
        y: &str,
        z: &str,
        group: Option<&str>,
        opacity: Option<f64>,
        size: Option<usize>,
        color: Option<Rgb>,
        colors: Option<Vec<Rgb>>,
        shape: Option<Shape>,
        shapes: Option<Vec<Shape>>,
    ) -> Vec<Box<dyn Trace + 'static>> {
        let mut traces: Vec<Box<dyn Trace + 'static>> = Vec::new();

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

                    let trace = Self::create_trace(&subset, x, y, z, Some(group), marker);

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

                let trace = Self::create_trace(data, x, y, z, group, marker);

                traces.push(trace);
            }
        }

        traces
    }

    fn create_trace(
        data: &DataFrame,
        x: &str,
        y: &str,
        z: &str,
        group_name: Option<&str>,
        marker: MarkerPlotly,
    ) -> Box<dyn Trace + 'static> {
        let x = Self::get_numeric_column(data, x);
        let y = Self::get_numeric_column(data, y);
        let z = Self::get_numeric_column(data, z);

        let mut trace = Scatter3D::default().x(x).y(y).z(z).mode(Mode::Markers);

        trace = trace.marker(marker);

        if let Some(name) = group_name {
            trace = trace.name(name);
        }

        trace
    }
}

impl Layout for Scatter3dPlot {}
impl Marker for Scatter3dPlot {}
impl Polar for Scatter3dPlot {}

impl PlotHelper for Scatter3dPlot {
    fn get_layout(&self) -> &LayoutPlotly {
        &self.layout
    }

    fn get_traces(&self) -> &Vec<Box<dyn Trace + 'static>> {
        &self.traces
    }
}
