use bon::bon;

use polars::frame::DataFrame;

use crate::{
    components::{Legend, Mode, Rgb, Shape, Text},
    ir::data::ColumnData,
    ir::layout::LayoutIR,
    ir::line::LineIR,
    ir::marker::MarkerIR,
    ir::trace::{ScatterGeoIR, TraceIR},
};

/// A structure representing a geographic scatter plot.
///
/// The `ScatterGeo` struct facilitates the creation and customization of geographic scatter plots
/// with various options for data selection, grouping, layout configuration, and aesthetic adjustments.
/// It supports plotting data points on a map using latitude and longitude coordinates, with customization
/// for markers, lines, text labels, and comprehensive layout options.
///
/// # Backend Support
///
/// | Backend | Supported |
/// |---------|-----------|
/// | Plotly  | Yes       |
/// | Plotters| --        |
///
/// # Arguments
///
/// * `data` - A reference to the `DataFrame` containing the data to be plotted.
/// * `lat` - A string slice specifying the column name to be used for latitude coordinates.
/// * `lon` - A string slice specifying the column name to be used for longitude coordinates.
/// * `mode` - An optional `Mode` specifying the drawing mode (markers, lines, or both).
/// * `text` - An optional string slice specifying the column name to be used for text labels.
/// * `group` - An optional string slice specifying the column name to be used for grouping data points.
/// * `sort_groups_by` - Optional comparator `fn(&str, &str) -> std::cmp::Ordering` to control group ordering. Groups are sorted lexically by default.
/// * `opacity` - An optional `f64` value specifying the opacity of the plot elements (range: 0.0 to 1.0).
/// * `size` - An optional `usize` specifying the size of the markers.
/// * `color` - An optional `Rgb` value specifying the color of the markers. This is used when `group` is not specified.
/// * `colors` - An optional vector of `Rgb` values specifying the colors for the markers. This is used when `group` is specified.
/// * `shape` - An optional `Shape` specifying the shape of the markers. This is used when `group` is not specified.
/// * `shapes` - An optional vector of `Shape` values specifying multiple shapes for the markers when plotting multiple groups.
/// * `line_width` - An optional `f64` value specifying the width of the lines (when mode includes lines).
/// * `line_color` - An optional `Rgb` value specifying the color of the lines.
/// * `plot_title` - An optional `Text` struct specifying the title of the plot.
/// * `legend_title` - An optional `Text` struct specifying the title of the legend.
/// * `legend` - An optional reference to a `Legend` struct for customizing the legend of the plot.
///
/// # Example
///
/// ```rust
/// use plotlars::{Plot, Rgb, ScatterGeo, Shape, Text, Mode};
/// use polars::prelude::*;
///
/// let data = LazyCsvReader::new(PlRefPath::new("data/us_cities_regions.csv"))
///     .finish()
///     .unwrap()
///     .collect()
///     .unwrap();
///
/// ScatterGeo::builder()
///     .data(&data)
///     .lat("lat")
///     .lon("lon")
///     .mode(Mode::Markers)
///     .text("city")
///     .group("region")
///     .size(15)
///     .colors(vec![
///         Rgb(255, 0, 0),
///         Rgb(0, 255, 0),
///         Rgb(0, 0, 255),
///         Rgb(255, 165, 0),
///     ])
///     .plot_title(
///         Text::from("Scatter Geo Plot")
///             .font("Arial")
///             .size(24)
///             .x(0.5)
///     )
///     .legend_title(
///         Text::from("Region")
///             .size(14)
///     )
///     .build()
///     .plot();
/// ```
///
/// ![Example](https://imgur.com/8PCEbhN.png)
#[derive(Clone)]
#[allow(dead_code)]
pub struct ScatterGeo {
    traces: Vec<TraceIR>,
    layout: LayoutIR,
}

#[bon]
impl ScatterGeo {
    #[builder(on(String, into), on(Text, into))]
    pub fn new(
        data: &DataFrame,
        lat: &str,
        lon: &str,
        mode: Option<Mode>,
        text: Option<&str>,
        group: Option<&str>,
        sort_groups_by: Option<fn(&str, &str) -> std::cmp::Ordering>,
        opacity: Option<f64>,
        size: Option<usize>,
        color: Option<Rgb>,
        colors: Option<Vec<Rgb>>,
        shape: Option<Shape>,
        shapes: Option<Vec<Shape>>,
        line_width: Option<f64>,
        line_color: Option<Rgb>,
        plot_title: Option<Text>,
        legend_title: Option<Text>,
        legend: Option<&Legend>,
    ) -> Self {
        // Build IR traces
        let traces = Self::create_ir_traces(
            data,
            lat,
            lon,
            mode,
            text,
            group,
            sort_groups_by,
            opacity,
            size,
            color,
            colors,
            shape,
            shapes,
            line_width,
            line_color,
        );

        let layout = LayoutIR {
            title: plot_title.clone(),
            x_title: None,
            y_title: None,
            y2_title: None,
            z_title: None,
            legend_title: legend_title.clone(),
            legend: legend.cloned(),
            dimensions: None,
            bar_mode: None,
            box_mode: None,
            box_gap: None,
            margin_bottom: None,
            axes_2d: None,
            scene_3d: None,
            polar: None,
            mapbox: None,
            grid: None,
            annotations: vec![],
        };
        Self { traces, layout }
    }
}

#[bon]
impl ScatterGeo {
    #[builder(
        start_fn = try_builder,
        finish_fn = try_build,
        builder_type = ScatterGeoTryBuilder,
        on(String, into),
        on(Text, into),
    )]
    pub fn try_new(
        data: &DataFrame,
        lat: &str,
        lon: &str,
        mode: Option<Mode>,
        text: Option<&str>,
        group: Option<&str>,
        sort_groups_by: Option<fn(&str, &str) -> std::cmp::Ordering>,
        opacity: Option<f64>,
        size: Option<usize>,
        color: Option<Rgb>,
        colors: Option<Vec<Rgb>>,
        shape: Option<Shape>,
        shapes: Option<Vec<Shape>>,
        line_width: Option<f64>,
        line_color: Option<Rgb>,
        plot_title: Option<Text>,
        legend_title: Option<Text>,
        legend: Option<&Legend>,
    ) -> Result<Self, crate::io::PlotlarsError> {
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            Self::__orig_new(
                data,
                lat,
                lon,
                mode,
                text,
                group,
                sort_groups_by,
                opacity,
                size,
                color,
                colors,
                shape,
                shapes,
                line_width,
                line_color,
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

impl ScatterGeo {
    #[allow(clippy::too_many_arguments)]
    fn create_ir_traces(
        data: &DataFrame,
        lat: &str,
        lon: &str,
        mode: Option<Mode>,
        text: Option<&str>,
        group: Option<&str>,
        sort_groups_by: Option<fn(&str, &str) -> std::cmp::Ordering>,
        opacity: Option<f64>,
        size: Option<usize>,
        color: Option<Rgb>,
        colors: Option<Vec<Rgb>>,
        shape: Option<Shape>,
        shapes: Option<Vec<Shape>>,
        line_width: Option<f64>,
        line_color: Option<Rgb>,
    ) -> Vec<TraceIR> {
        let mut traces = Vec::new();

        let line_ir = if line_width.is_some() || line_color.is_some() {
            Some(LineIR {
                width: line_width,
                style: None,
                color: line_color,
            })
        } else {
            None
        };

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
                        ColumnData::Numeric(crate::data::get_numeric_column(&subset, lat));
                    let lon_data =
                        ColumnData::Numeric(crate::data::get_numeric_column(&subset, lon));

                    let text_data = text.map(|text_col| {
                        ColumnData::String(crate::data::get_string_column(&subset, text_col))
                    });

                    traces.push(TraceIR::ScatterGeo(ScatterGeoIR {
                        lat: lat_data,
                        lon: lon_data,
                        name: Some(group_name.to_string()),
                        text: text_data,
                        mode,
                        marker: Some(marker_ir),
                        line: line_ir.clone(),
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

                let lat_data = ColumnData::Numeric(crate::data::get_numeric_column(data, lat));
                let lon_data = ColumnData::Numeric(crate::data::get_numeric_column(data, lon));

                let text_data = text.map(|text_col| {
                    ColumnData::String(crate::data::get_string_column(data, text_col))
                });

                traces.push(TraceIR::ScatterGeo(ScatterGeoIR {
                    lat: lat_data,
                    lon: lon_data,
                    name: None,
                    text: text_data,
                    mode,
                    marker: Some(marker_ir),
                    line: line_ir,
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

impl crate::Plot for ScatterGeo {
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
            "lat" => [40.7, 34.0, 41.8],
            "lon" => [-74.0, -118.2, -87.6]
        ]
        .unwrap();
        let plot = ScatterGeo::builder()
            .data(&df)
            .lat("lat")
            .lon("lon")
            .build();
        assert_eq!(plot.ir_traces().len(), 1);
    }

    #[test]
    fn test_trace_variant() {
        let df = df![
            "lat" => [40.7],
            "lon" => [-74.0]
        ]
        .unwrap();
        let plot = ScatterGeo::builder()
            .data(&df)
            .lat("lat")
            .lon("lon")
            .build();
        assert!(matches!(plot.ir_traces()[0], TraceIR::ScatterGeo(_)));
    }

    #[test]
    fn test_with_group() {
        let df = df![
            "lat" => [40.7, 34.0, 41.8, 29.7],
            "lon" => [-74.0, -118.2, -87.6, -95.3],
            "region" => ["east", "west", "east", "south"]
        ]
        .unwrap();
        let plot = ScatterGeo::builder()
            .data(&df)
            .lat("lat")
            .lon("lon")
            .group("region")
            .build();
        assert_eq!(plot.ir_traces().len(), 3);
    }

    #[test]
    fn test_layout_no_cartesian_axes() {
        let df = df![
            "lat" => [40.7],
            "lon" => [-74.0]
        ]
        .unwrap();
        let plot = ScatterGeo::builder()
            .data(&df)
            .lat("lat")
            .lon("lon")
            .build();
        assert!(plot.ir_layout().axes_2d.is_none());
    }
}
