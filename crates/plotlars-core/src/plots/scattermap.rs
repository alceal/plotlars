use bon::bon;

use polars::frame::DataFrame;

use crate::{
    components::{Legend, Rgb, Shape, Text},
    ir::data::ColumnData,
    ir::layout::{LayoutIR, MapboxIR},
    ir::marker::MarkerIR,
    ir::trace::{ScatterMapIR, TraceIR},
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
/// let dataset = LazyCsvReader::new(PlRefPath::new("data/cities.csv"))
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
#[derive(Clone)]
#[allow(dead_code)]
pub struct ScatterMap {
    traces: Vec<TraceIR>,
    layout: LayoutIR,
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
        let traces = Self::create_ir_traces(
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

        let layout = LayoutIR {
            title: plot_title,
            x_title: None,
            y_title: None,
            y2_title: None,
            z_title: None,
            legend_title,
            legend: legend.cloned(),
            dimensions: None,
            bar_mode: None,
            box_mode: None,
            box_gap: None,
            margin_bottom: Some(0),
            axes_2d: None,
            scene_3d: None,
            polar: None,
            mapbox: Some(MapboxIR {
                center: center.map(|c| (c[0], c[1])),
                zoom: zoom.map(|z| z as f64),
                style: None,
            }),
            grid: None,
            annotations: vec![],
        };

        Self { traces, layout }
    }
}

#[bon]
impl ScatterMap {
    #[builder(
        start_fn = try_builder,
        finish_fn = try_build,
        builder_type = ScatterMapTryBuilder,
        on(String, into),
        on(Text, into),
    )]
    pub fn try_new(
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
    ) -> Result<Self, crate::io::PlotlarsError> {
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            Self::__orig_new(
                data,
                latitude,
                longitude,
                center,
                zoom,
                group,
                sort_groups_by,
                opacity,
                size,
                color,
                colors,
                shape,
                shapes,
                plot_title,
                legend_title,
                legend,
            )
        }))
        .map_err(|panic| {
            let msg = panic
                .downcast_ref::<String>()
                .cloned()
                .or_else(|| panic.downcast_ref::<&str>().map(|s| s.to_string()))
                .unwrap_or_else(|| "unknown error".to_string());
            crate::io::PlotlarsError::PlotBuild { message: msg }
        })
    }
}

impl ScatterMap {
    #[allow(clippy::too_many_arguments)]
    fn create_ir_traces(
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
    ) -> Vec<TraceIR> {
        let mut traces = Vec::new();

        match group {
            Some(group_col) => {
                let groups = crate::data::get_unique_groups(data, group_col, sort_groups_by);
                let groups = groups.iter().map(|s| s.as_str());

                for (i, group_name) in groups.enumerate() {
                    let subset = crate::data::filter_data_by_group(data, group_col, group_name);

                    let resolved_color = Self::resolve_color(i, color, colors.clone());
                    let resolved_shape = Self::resolve_shape(i, shape, shapes.clone());

                    let marker_ir = MarkerIR {
                        opacity,
                        size,
                        color: resolved_color,
                        shape: resolved_shape,
                    };

                    let lat_data =
                        ColumnData::Numeric(crate::data::get_numeric_column(&subset, latitude));
                    let lon_data =
                        ColumnData::Numeric(crate::data::get_numeric_column(&subset, longitude));

                    traces.push(TraceIR::ScatterMap(ScatterMapIR {
                        lat: lat_data,
                        lon: lon_data,
                        name: Some(group_name.to_string()),
                        marker: Some(marker_ir),
                        show_legend: None,
                    }));
                }
            }
            None => {
                let resolved_color = Self::resolve_color(0, color, colors.clone());
                let resolved_shape = Self::resolve_shape(0, shape, shapes.clone());

                let marker_ir = MarkerIR {
                    opacity,
                    size,
                    color: resolved_color,
                    shape: resolved_shape,
                };

                let lat_data = ColumnData::Numeric(crate::data::get_numeric_column(data, latitude));
                let lon_data =
                    ColumnData::Numeric(crate::data::get_numeric_column(data, longitude));

                traces.push(TraceIR::ScatterMap(ScatterMapIR {
                    lat: lat_data,
                    lon: lon_data,
                    name: None,
                    marker: Some(marker_ir),
                    show_legend: None,
                }));
            }
        }

        traces
    }

    fn resolve_color(index: usize, color: Option<Rgb>, colors: Option<Vec<Rgb>>) -> Option<Rgb> {
        if let Some(c) = color {
            return Some(c);
        }
        if let Some(ref cs) = colors {
            return cs.get(index).copied();
        }
        None
    }

    fn resolve_shape(
        index: usize,
        shape: Option<Shape>,
        shapes: Option<Vec<Shape>>,
    ) -> Option<Shape> {
        if let Some(s) = shape {
            return Some(s);
        }
        if let Some(ref ss) = shapes {
            return ss.get(index).copied();
        }
        None
    }
}

impl crate::Plot for ScatterMap {
    fn ir_traces(&self) -> &[TraceIR] {
        &self.traces
    }

    fn ir_layout(&self) -> &LayoutIR {
        &self.layout
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Plot;
    use polars::prelude::*;

    #[test]
    fn test_basic_one_trace() {
        let df = df![
            "latitude" => [48.8, 51.5, 40.7],
            "longitude" => [2.3, -0.1, -74.0]
        ]
        .unwrap();
        let plot = ScatterMap::builder()
            .data(&df)
            .latitude("latitude")
            .longitude("longitude")
            .build();
        assert_eq!(plot.ir_traces().len(), 1);
    }

    #[test]
    fn test_trace_variant() {
        let df = df![
            "latitude" => [48.8],
            "longitude" => [2.3]
        ]
        .unwrap();
        let plot = ScatterMap::builder()
            .data(&df)
            .latitude("latitude")
            .longitude("longitude")
            .build();
        assert!(matches!(plot.ir_traces()[0], TraceIR::ScatterMap(_)));
    }

    #[test]
    fn test_with_group() {
        let df = df![
            "latitude" => [48.8, 51.5, 40.7],
            "longitude" => [2.3, -0.1, -74.0],
            "city" => ["paris", "london", "nyc"]
        ]
        .unwrap();
        let plot = ScatterMap::builder()
            .data(&df)
            .latitude("latitude")
            .longitude("longitude")
            .group("city")
            .build();
        assert_eq!(plot.ir_traces().len(), 3);
    }

    #[test]
    fn test_layout_has_mapbox() {
        let df = df![
            "latitude" => [48.8],
            "longitude" => [2.3]
        ]
        .unwrap();
        let plot = ScatterMap::builder()
            .data(&df)
            .latitude("latitude")
            .longitude("longitude")
            .build();
        assert!(plot.ir_layout().mapbox.is_some());
    }
}
