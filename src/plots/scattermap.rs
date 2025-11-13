use bon::bon;

use plotly::{
    common::{Marker as MarkerPlotly, Mode},
    layout::{Center, Layout as LayoutPlotly, Mapbox, MapboxStyle, Margin},
    ScatterMapbox, Trace,
};

use polars::frame::DataFrame;
use serde::Serialize;

use crate::{
    common::{Layout, Marker, PlotHelper, Polar},
    components::{Legend, Rgb, Shape, Text},
};

/// A structure representing a scatter plot on a map.
///
/// The `ScatterMap` struct allows for visualizing geographical data points on an interactive map.
/// Each data point is defined by its latitude and longitude, with additional options for grouping,
/// coloring, size, opacity, and map configuration such as zoom level and center coordinates.
/// This struct is ideal for displaying spatial data distributions, such as city locations or geospatial datasets.
///
/// # Arguments
///
/// * `data` - A reference to the `DataFrame` containing the data to be plotted.
/// * `latitude` - A string slice specifying the column name containing latitude values.
/// * `longitude` - A string slice specifying the column name containing longitude values.
/// * `center` - An optional array `[f64; 2]` specifying the initial center point of the map ([latitude, longitude]).
/// * `zoom` - An optional `u8` specifying the initial zoom level of the map.
/// * `group` - An optional string slice specifying the column name for grouping data points (e.g., by city or category).
/// * `sort_groups_by` - Optional comparator `fn(&str, &str) -> std::cmp::Ordering` to control group ordering. Groups are sorted lexically by default.
/// * `opacity` - An optional `f64` value between `0.0` and `1.0` specifying the opacity of the points.
/// * `size` - An optional `usize` specifying the size of the scatter points.
/// * `color` - An optional `Rgb` value specifying the color of the points (if no grouping is applied).
/// * `colors` - An optional vector of `Rgb` values specifying colors for grouped points.
/// * `shape` - An optional `Shape` enum specifying the marker shape for the points.
/// * `shapes` - An optional vector of `Shape` enums specifying shapes for grouped points.
/// * `plot_title` - An optional `Text` struct specifying the title of the plot.
/// * `legend_title` - An optional `Text` struct specifying the title of the legend.
/// * `legend` - An optional reference to a `Legend` struct for customizing the legend (e.g., positioning, font, etc.).
///
/// # Example
///
/// ## Basic Scatter Map Plot
///
/// ```rust
/// use plotlars::{Plot, ScatterMap, Text};
/// use polars::prelude::*;
///
/// let dataset = LazyCsvReader::new(PlPath::new("data/cities.csv"))
///     .finish()
///     .unwrap()
///     .collect()
///     .unwrap();
///
/// ScatterMap::builder()
///     .data(&dataset)
///     .latitude("latitude")
///     .longitude("longitude")
///     .center([48.856613, 2.352222])
///     .zoom(4)
///     .group("city")
///     .opacity(0.5)
///     .size(12)
///     .plot_title(
///         Text::from("Scatter Map")
///             .font("Arial")
///             .size(18)
///     )
///     .legend_title("cities")
///     .build()
///     .plot();
/// ```
///
/// ![Example](https://imgur.com/8MCjVOd.png)
#[derive(Clone, Serialize)]
pub struct ScatterMap {
    traces: Vec<Box<dyn Trace + 'static>>,
    layout: LayoutPlotly,
}

#[bon]
impl ScatterMap {
    #[builder(on(String, into), on(Text, into))]
    pub fn new(
        data: &DataFrame,
        latitude: &str,
        longitude: &str,
        center: Option<[f64; 2]>,
        zoom: Option<u8>,
        group: Option<&str>,
        sort_groups_by: Option<fn(&str, &str) -> std::cmp::Ordering>,
        opacity: Option<f64>,
        size: Option<usize>,
        color: Option<Rgb>,
        colors: Option<Vec<Rgb>>,
        shape: Option<Shape>,
        shapes: Option<Vec<Shape>>,
        plot_title: Option<Text>,
        legend_title: Option<Text>,
        legend: Option<&Legend>,
    ) -> Self {
        let x_title = None;
        let y_title = None;
        let z_title = None;
        let x_axis = None;
        let y_axis = None;
        let z_axis = None;
        let y2_title = None;
        let y2_axis = None;

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
        )
        .margin(Margin::new().bottom(0));

        let mut map_box = Mapbox::new().style(MapboxStyle::OpenStreetMap).zoom(0);

        if let Some(center) = center {
            map_box = map_box.center(Center::new(center[0], center[1]));
        }

        if let Some(zoom) = zoom {
            map_box = map_box.zoom(zoom);
        }

        layout = layout.mapbox(map_box);

        let traces = Self::create_traces(
            data,
            latitude,
            longitude,
            group,
            sort_groups_by,
            opacity,
            size,
            color,
            colors,
            shape,
            shapes,
        );

        Self { traces, layout }
    }

    #[allow(clippy::too_many_arguments)]
    fn create_traces(
        data: &DataFrame,
        latitude: &str,
        longitude: &str,
        group: Option<&str>,
        sort_groups_by: Option<fn(&str, &str) -> std::cmp::Ordering>,
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
                let groups = Self::get_unique_groups(data, group_col, sort_groups_by);

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

                    let trace =
                        Self::create_trace(&subset, latitude, longitude, Some(group), marker);

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

                let trace = Self::create_trace(data, latitude, longitude, group, marker);

                traces.push(trace);
            }
        }

        traces
    }

    fn create_trace(
        data: &DataFrame,
        latitude: &str,
        longitude: &str,
        group_name: Option<&str>,
        marker: MarkerPlotly,
    ) -> Box<dyn Trace + 'static> {
        let latitude = Self::get_numeric_column(data, latitude);
        let longitude = Self::get_numeric_column(data, longitude);

        let mut trace = ScatterMapbox::default()
            .lat(latitude)
            .lon(longitude)
            .mode(Mode::Markers);

        trace = trace.marker(marker);

        if let Some(name) = group_name {
            trace = trace.name(name);
        }

        trace
    }
}

impl Layout for ScatterMap {}
impl Marker for ScatterMap {}
impl Polar for ScatterMap {}

impl PlotHelper for ScatterMap {
    fn get_layout(&self) -> &LayoutPlotly {
        &self.layout
    }

    fn get_traces(&self) -> &Vec<Box<dyn Trace + 'static>> {
        &self.traces
    }
}
