use bon::bon;

use crate::{
    components::{Axis, ColorBar, FacetConfig, Palette, Text},
    ir::data::ColumnData,
    ir::layout::LayoutIR,
    ir::trace::{HeatMapIR, TraceIR},
};
use polars::frame::DataFrame;

/// A structure representing a heat map.
///
/// The `HeatMap` struct enables the creation of heat map visualizations with options for color scaling,
/// axis customization, legend adjustments, and data value formatting. Users can customize the color
/// scale, adjust the color bar, and set titles for the plot and axes, as well as format ticks and scales
/// for improved data readability.
///
/// # Arguments
///
/// * `data` - A reference to the `DataFrame` containing the data to be plotted.
/// * `x` - A string slice specifying the column name for x-axis values.
/// * `y` - A string slice specifying the column name for y-axis values.
/// * `z` - A string slice specifying the column name for z-axis values, which are represented by the color intensity.
/// * `facet` - An optional string slice specifying the column name to be used for faceting (creating multiple subplots).
/// * `facet_config` - An optional reference to a `FacetConfig` struct for customizing facet behavior (grid dimensions, scales, gaps, etc.).
/// * `auto_color_scale` - An optional boolean for enabling automatic color scaling based on data.
/// * `color_bar` - An optional reference to a `ColorBar` struct for customizing the color bar appearance.
/// * `color_scale` - An optional `Palette` enum for specifying the color scale (e.g., Viridis).
/// * `reverse_scale` - An optional boolean to reverse the color scale direction.
/// * `show_scale` - An optional boolean to display the color scale on the plot.
/// * `plot_title` - An optional `Text` struct for setting the title of the plot.
/// * `x_title` - An optional `Text` struct for labeling the x-axis.
/// * `y_title` - An optional `Text` struct for labeling the y-axis.
/// * `x_axis` - An optional reference to an `Axis` struct for customizing x-axis appearance.
/// * `y_axis` - An optional reference to an `Axis` struct for customizing y-axis appearance.
///
/// # Example
///
/// ```rust
/// use plotlars::{ColorBar, HeatMap, Palette, Plot, Text, ValueExponent};
/// use polars::prelude::*;
///
/// let dataset = LazyCsvReader::new(PlRefPath::new("data/heatmap.csv"))
///     .finish()
///     .unwrap()
///     .collect()
///     .unwrap();
///
/// HeatMap::builder()
///     .data(&dataset)
///     .x("x")
///     .y("y")
///     .z("z")
///     .color_bar(
///         &ColorBar::new()
///             .length(0.7)
///             .value_exponent(ValueExponent::None)
///             .separate_thousands(true)
///             .tick_length(5)
///             .tick_step(2500.0)
///     )
///     .plot_title(
///         Text::from("Heat Map")
///             .font("Arial")
///             .size(18)
///     )
///     .color_scale(Palette::Viridis)
///     .build()
///     .plot();
/// ```
///
/// ![Example](https://imgur.com/5uFih4M.png)
#[derive(Clone)]
#[allow(dead_code)]
pub struct HeatMap {
    traces: Vec<TraceIR>,
    layout: LayoutIR,
}

#[bon]
impl HeatMap {
    #[builder(on(String, into), on(Text, into))]
    pub fn new(
        data: &DataFrame,
        x: &str,
        y: &str,
        z: &str,
        facet: Option<&str>,
        facet_config: Option<&FacetConfig>,
        auto_color_scale: Option<bool>,
        color_bar: Option<&ColorBar>,
        color_scale: Option<Palette>,
        reverse_scale: Option<bool>,
        show_scale: Option<bool>,
        plot_title: Option<Text>,
        x_title: Option<Text>,
        y_title: Option<Text>,
        x_axis: Option<&Axis>,
        y_axis: Option<&Axis>,
    ) -> Self {
        let grid = facet.map(|facet_column| {
            let config = facet_config.cloned().unwrap_or_default();
            let facet_categories =
                crate::data::get_unique_groups(data, facet_column, config.sorter);
            let n_facets = facet_categories.len();
            let (ncols, nrows) =
                crate::faceting::calculate_grid_dimensions(n_facets, config.cols, config.rows);
            crate::ir::facet::GridSpec {
                kind: crate::ir::facet::FacetKind::Axis,
                rows: nrows,
                cols: ncols,
                h_gap: config.h_gap,
                v_gap: config.v_gap,
                scales: config.scales.clone(),
                n_facets,
                facet_categories,
                title_style: config.title_style.clone(),
                x_title: x_title.clone(),
                y_title: y_title.clone(),
                x_axis: x_axis.cloned(),
                y_axis: y_axis.cloned(),
                legend_title: None,
                legend: None,
            }
        });

        let layout = LayoutIR {
            title: plot_title.clone(),
            x_title: if grid.is_some() {
                None
            } else {
                x_title.clone()
            },
            y_title: if grid.is_some() {
                None
            } else {
                y_title.clone()
            },
            y2_title: None,
            z_title: None,
            legend_title: None,
            legend: None,
            dimensions: None,
            bar_mode: None,
            box_mode: None,
            box_gap: None,
            margin_bottom: None,
            axes_2d: if grid.is_some() {
                None
            } else {
                Some(crate::ir::layout::Axes2dIR {
                    x_axis: x_axis.cloned(),
                    y_axis: y_axis.cloned(),
                    y2_axis: None,
                })
            },
            scene_3d: None,
            polar: None,
            mapbox: None,
            grid,
            annotations: vec![],
        };

        let traces = match facet {
            Some(facet_column) => {
                let config = facet_config.cloned().unwrap_or_default();

                Self::create_ir_traces_faceted(
                    data,
                    x,
                    y,
                    z,
                    facet_column,
                    &config,
                    auto_color_scale,
                    color_bar,
                    color_scale,
                    reverse_scale,
                    show_scale,
                )
            }
            None => Self::create_ir_traces(
                data,
                x,
                y,
                z,
                auto_color_scale,
                color_bar,
                color_scale,
                reverse_scale,
                show_scale,
            ),
        };

        Self { traces, layout }
    }

    #[allow(clippy::too_many_arguments)]
    fn create_ir_traces(
        data: &DataFrame,
        x: &str,
        y: &str,
        z: &str,
        auto_color_scale: Option<bool>,
        color_bar: Option<&ColorBar>,
        color_scale: Option<Palette>,
        reverse_scale: Option<bool>,
        show_scale: Option<bool>,
    ) -> Vec<TraceIR> {
        vec![TraceIR::HeatMap(HeatMapIR {
            x: ColumnData::String(crate::data::get_string_column(data, x)),
            y: ColumnData::String(crate::data::get_string_column(data, y)),
            z: ColumnData::Numeric(crate::data::get_numeric_column(data, z)),
            color_scale,
            color_bar: color_bar.cloned(),
            auto_color_scale,
            reverse_scale,
            show_scale,
            z_min: None,
            z_max: None,
            subplot_ref: None,
        })]
    }

    #[allow(clippy::too_many_arguments)]
    fn create_ir_traces_faceted(
        data: &DataFrame,
        x: &str,
        y: &str,
        z: &str,
        facet_column: &str,
        config: &FacetConfig,
        auto_color_scale: Option<bool>,
        color_bar: Option<&ColorBar>,
        color_scale: Option<Palette>,
        reverse_scale: Option<bool>,
        show_scale: Option<bool>,
    ) -> Vec<TraceIR> {
        const MAX_FACETS: usize = 8;

        let facet_categories = crate::data::get_unique_groups(data, facet_column, config.sorter);

        if facet_categories.len() > MAX_FACETS {
            panic!(
                "Facet column '{}' has {} unique values, but plotly.rs supports maximum {} subplots",
                facet_column,
                facet_categories.len(),
                MAX_FACETS
            );
        }

        let global_z_range = Self::calculate_global_z_range(data, z);

        let mut traces = Vec::new();

        for (facet_idx, facet_value) in facet_categories.iter().enumerate() {
            let facet_data = crate::data::filter_data_by_group(data, facet_column, facet_value);

            let subplot_ref = format!(
                "{}{}",
                crate::faceting::get_axis_reference(facet_idx, "x"),
                crate::faceting::get_axis_reference(facet_idx, "y")
            );

            let show_scale_for_trace = if facet_idx == 0 {
                show_scale
            } else {
                Some(false)
            };

            traces.push(TraceIR::HeatMap(HeatMapIR {
                x: ColumnData::String(crate::data::get_string_column(&facet_data, x)),
                y: ColumnData::String(crate::data::get_string_column(&facet_data, y)),
                z: ColumnData::Numeric(crate::data::get_numeric_column(&facet_data, z)),
                color_scale,
                color_bar: color_bar.cloned(),
                auto_color_scale,
                reverse_scale,
                show_scale: show_scale_for_trace,
                z_min: Some(global_z_range.0 as f64),
                z_max: Some(global_z_range.1 as f64),
                subplot_ref: Some(subplot_ref),
            }));
        }

        traces
    }

    fn calculate_global_z_range(data: &DataFrame, z: &str) -> (f32, f32) {
        let z_data = crate::data::get_numeric_column(data, z);

        let values: Vec<f32> = z_data.iter().filter_map(|v| *v).collect();

        if values.is_empty() {
            return (0.0, 1.0);
        }

        let min = values.iter().cloned().fold(f32::INFINITY, f32::min);
        let max = values.iter().cloned().fold(f32::NEG_INFINITY, f32::max);

        (min, max)
    }
}

impl crate::Plot for HeatMap {
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
            "x" => ["a", "b", "c"],
            "y" => ["d", "e", "f"],
            "z" => [1.0, 2.0, 3.0]
        ]
        .unwrap();
        let plot = HeatMap::builder().data(&df).x("x").y("y").z("z").build();
        assert_eq!(plot.ir_traces().len(), 1);
        assert!(matches!(plot.ir_traces()[0], TraceIR::HeatMap(_)));
    }

    #[test]
    fn test_layout_has_axes() {
        let df = df![
            "x" => ["a", "b"],
            "y" => ["c", "d"],
            "z" => [1.0, 2.0]
        ]
        .unwrap();
        let plot = HeatMap::builder().data(&df).x("x").y("y").z("z").build();
        assert!(plot.ir_layout().axes_2d.is_some());
    }

    #[test]
    fn test_layout_title() {
        let df = df![
            "x" => ["a"],
            "y" => ["b"],
            "z" => [1.0]
        ]
        .unwrap();
        let plot = HeatMap::builder()
            .data(&df)
            .x("x")
            .y("y")
            .z("z")
            .plot_title("Heat")
            .build();
        assert!(plot.ir_layout().title.is_some());
    }
}
