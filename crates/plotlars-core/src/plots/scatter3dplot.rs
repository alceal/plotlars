use bon::bon;

use polars::frame::DataFrame;

use crate::{
    components::{FacetConfig, Legend, Rgb, Shape, Text, DEFAULT_PLOTLY_COLORS},
    ir::data::ColumnData,
    ir::layout::LayoutIR,
    ir::marker::MarkerIR,
    ir::trace::{Scatter3dPlotIR, TraceIR},
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
/// * `sort_groups_by` - Optional comparator `fn(&str, &str) -> std::cmp::Ordering` to control group ordering. Groups are sorted lexically by default.
/// * `facet` - An optional string slice specifying the column name to be used for faceting (creating multiple subplots).
/// * `facet_config` - An optional reference to a `FacetConfig` struct for customizing facet behavior (grid dimensions, scales, gaps, etc.).
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
/// use polars::prelude::*;
///
/// let dataset = LazyCsvReader::new(PlRefPath::new("data/penguins.csv"))
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
/// ![Example](https://imgur.com/WYTQxHA.png)
#[derive(Clone)]
#[allow(dead_code)]
pub struct Scatter3dPlot {
    traces: Vec<TraceIR>,
    layout: LayoutIR,
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
        sort_groups_by: Option<fn(&str, &str) -> std::cmp::Ordering>,
        facet: Option<&str>,
        facet_config: Option<&FacetConfig>,
        opacity: Option<f64>,
        size: Option<usize>,
        color: Option<Rgb>,
        colors: Option<Vec<Rgb>>,
        shape: Option<Shape>,
        shapes: Option<Vec<Shape>>,
        plot_title: Option<Text>,
        legend: Option<&Legend>,
    ) -> Self {
        let grid = facet.map(|facet_column| {
            let config = facet_config.cloned().unwrap_or_default();
            let facet_categories =
                crate::data::get_unique_groups(data, facet_column, config.sorter);
            let n_facets = facet_categories.len();
            let (ncols, nrows) =
                crate::faceting::calculate_grid_dimensions(n_facets, config.cols, config.rows);
            crate::ir::facet::GridSpec {
                kind: crate::ir::facet::FacetKind::Scene,
                rows: nrows,
                cols: ncols,
                h_gap: config.h_gap,
                v_gap: config.v_gap,
                scales: config.scales.clone(),
                n_facets,
                facet_categories,
                title_style: config.title_style.clone(),
                x_title: None,
                y_title: None,
                x_axis: None,
                y_axis: None,
                legend_title: None,
                legend: legend.cloned(),
            }
        });

        let traces = match facet {
            Some(facet_column) => {
                let config = facet_config.cloned().unwrap_or_default();
                Self::create_ir_traces_faceted(
                    data,
                    x,
                    y,
                    z,
                    group,
                    sort_groups_by,
                    facet_column,
                    &config,
                    opacity,
                    size,
                    color,
                    colors,
                    shape,
                    shapes,
                )
            }
            None => Self::create_ir_traces(
                data,
                x,
                y,
                z,
                group,
                sort_groups_by,
                opacity,
                size,
                color,
                colors,
                shape,
                shapes,
            ),
        };

        let layout = LayoutIR {
            title: plot_title,
            x_title: None,
            y_title: None,
            y2_title: None,
            z_title: None,
            legend_title: None,
            legend: if grid.is_some() {
                None
            } else {
                legend.cloned()
            },
            dimensions: None,
            bar_mode: None,
            box_mode: None,
            box_gap: None,
            margin_bottom: None,
            axes_2d: None,
            scene_3d: None,
            polar: None,
            mapbox: None,
            grid,
            annotations: vec![],
        };

        Self { traces, layout }
    }
}

#[bon]
impl Scatter3dPlot {
    #[builder(
        start_fn = try_builder,
        finish_fn = try_build,
        builder_type = Scatter3dPlotTryBuilder,
        on(String, into),
        on(Text, into),
    )]
    pub fn try_new(
        data: &DataFrame,
        x: &str,
        y: &str,
        z: &str,
        group: Option<&str>,
        sort_groups_by: Option<fn(&str, &str) -> std::cmp::Ordering>,
        facet: Option<&str>,
        facet_config: Option<&FacetConfig>,
        opacity: Option<f64>,
        size: Option<usize>,
        color: Option<Rgb>,
        colors: Option<Vec<Rgb>>,
        shape: Option<Shape>,
        shapes: Option<Vec<Shape>>,
        plot_title: Option<Text>,
        legend: Option<&Legend>,
    ) -> Result<Self, crate::io::PlotlarsError> {
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            Self::__orig_new(
                data,
                x,
                y,
                z,
                group,
                sort_groups_by,
                facet,
                facet_config,
                opacity,
                size,
                color,
                colors,
                shape,
                shapes,
                plot_title,
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

impl Scatter3dPlot {
    fn get_scene_reference(index: usize) -> String {
        match index {
            0 => "scene".to_string(),
            1 => "scene2".to_string(),
            2 => "scene3".to_string(),
            3 => "scene4".to_string(),
            4 => "scene5".to_string(),
            5 => "scene6".to_string(),
            6 => "scene7".to_string(),
            7 => "scene8".to_string(),
            _ => "scene".to_string(),
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn create_ir_traces(
        data: &DataFrame,
        x: &str,
        y: &str,
        z: &str,
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

                for (i, group_name) in groups.iter().enumerate() {
                    let subset = crate::data::filter_data_by_group(data, group_col, group_name);

                    let marker_ir = MarkerIR {
                        opacity,
                        size,
                        color: Self::resolve_color(i, color, colors.clone()),
                        shape: Self::resolve_shape(i, shape, shapes.clone()),
                    };

                    traces.push(TraceIR::Scatter3dPlot(Scatter3dPlotIR {
                        x: ColumnData::Numeric(crate::data::get_numeric_column(&subset, x)),
                        y: ColumnData::Numeric(crate::data::get_numeric_column(&subset, y)),
                        z: ColumnData::Numeric(crate::data::get_numeric_column(&subset, z)),
                        name: Some(group_name.to_string()),
                        mode: None,
                        marker: Some(marker_ir),
                        show_legend: None,
                        legend_group: None,
                        scene_ref: None,
                    }));
                }
            }
            None => {
                let marker_ir = MarkerIR {
                    opacity,
                    size,
                    color: Self::resolve_color(0, color, colors),
                    shape: Self::resolve_shape(0, shape, shapes),
                };

                traces.push(TraceIR::Scatter3dPlot(Scatter3dPlotIR {
                    x: ColumnData::Numeric(crate::data::get_numeric_column(data, x)),
                    y: ColumnData::Numeric(crate::data::get_numeric_column(data, y)),
                    z: ColumnData::Numeric(crate::data::get_numeric_column(data, z)),
                    name: None,
                    mode: None,
                    marker: Some(marker_ir),
                    show_legend: None,
                    legend_group: None,
                    scene_ref: None,
                }));
            }
        }

        traces
    }

    #[allow(clippy::too_many_arguments)]
    fn create_ir_traces_faceted(
        data: &DataFrame,
        x: &str,
        y: &str,
        z: &str,
        group: Option<&str>,
        sort_groups_by: Option<fn(&str, &str) -> std::cmp::Ordering>,
        facet_column: &str,
        config: &FacetConfig,
        opacity: Option<f64>,
        size: Option<usize>,
        color: Option<Rgb>,
        colors: Option<Vec<Rgb>>,
        shape: Option<Shape>,
        shapes: Option<Vec<Shape>>,
    ) -> Vec<TraceIR> {
        const MAX_FACETS: usize = 8;

        let facet_categories = crate::data::get_unique_groups(data, facet_column, config.sorter);

        if facet_categories.len() > MAX_FACETS {
            panic!(
                "Facet column '{}' has {} unique values, but plotly.rs supports maximum {} 3D scenes",
                facet_column,
                facet_categories.len(),
                MAX_FACETS
            );
        }

        if let Some(ref color_vec) = colors {
            if group.is_none() {
                let color_count = color_vec.len();
                let facet_count = facet_categories.len();
                if color_count != facet_count {
                    panic!(
                        "When using colors with facet (without group), colors.len() must equal number of facets. \
                         Expected {} colors for {} facets, but got {} colors.",
                        facet_count, facet_count, color_count
                    );
                }
            } else if let Some(group_col) = group {
                let groups = crate::data::get_unique_groups(data, group_col, sort_groups_by);
                let color_count = color_vec.len();
                let group_count = groups.len();
                if color_count < group_count {
                    panic!(
                        "When using colors with group, colors.len() must be >= number of groups. \
                         Need at least {} colors for {} groups, but got {} colors",
                        group_count, group_count, color_count
                    );
                }
            }
        }

        let global_group_indices: std::collections::HashMap<String, usize> =
            if let Some(group_col) = group {
                let global_groups = crate::data::get_unique_groups(data, group_col, sort_groups_by);
                global_groups
                    .into_iter()
                    .enumerate()
                    .map(|(idx, group_name)| (group_name, idx))
                    .collect()
            } else {
                std::collections::HashMap::new()
            };

        let colors = if group.is_some() && colors.is_none() {
            Some(DEFAULT_PLOTLY_COLORS.to_vec())
        } else {
            colors
        };

        let mut traces = Vec::new();

        if config.highlight_facet {
            for (facet_idx, facet_value) in facet_categories.iter().enumerate() {
                let scene = Self::get_scene_reference(facet_idx);

                for other_facet_value in facet_categories.iter() {
                    if other_facet_value != facet_value {
                        let other_data = crate::data::filter_data_by_group(
                            data,
                            facet_column,
                            other_facet_value,
                        );

                        let grey_color = config.unhighlighted_color.unwrap_or(Rgb(200, 200, 200));
                        let marker_ir = MarkerIR {
                            opacity,
                            size,
                            color: Some(grey_color),
                            shape: Self::resolve_shape(0, shape, None),
                        };

                        traces.push(TraceIR::Scatter3dPlot(Scatter3dPlotIR {
                            x: ColumnData::Numeric(crate::data::get_numeric_column(&other_data, x)),
                            y: ColumnData::Numeric(crate::data::get_numeric_column(&other_data, y)),
                            z: ColumnData::Numeric(crate::data::get_numeric_column(&other_data, z)),
                            name: None,
                            mode: None,
                            marker: Some(marker_ir),
                            show_legend: Some(false),
                            legend_group: None,
                            scene_ref: Some(scene.clone()),
                        }));
                    }
                }

                let facet_data = crate::data::filter_data_by_group(data, facet_column, facet_value);

                match group {
                    Some(group_col) => {
                        let groups =
                            crate::data::get_unique_groups(&facet_data, group_col, sort_groups_by);

                        for group_val in groups.iter() {
                            let group_data = crate::data::filter_data_by_group(
                                &facet_data,
                                group_col,
                                group_val,
                            );

                            let global_idx =
                                global_group_indices.get(group_val).copied().unwrap_or(0);

                            let marker_ir = MarkerIR {
                                opacity,
                                size,
                                color: Self::resolve_color(global_idx, color, colors.clone()),
                                shape: Self::resolve_shape(global_idx, shape, shapes.clone()),
                            };

                            traces.push(TraceIR::Scatter3dPlot(Scatter3dPlotIR {
                                x: ColumnData::Numeric(crate::data::get_numeric_column(
                                    &group_data,
                                    x,
                                )),
                                y: ColumnData::Numeric(crate::data::get_numeric_column(
                                    &group_data,
                                    y,
                                )),
                                z: ColumnData::Numeric(crate::data::get_numeric_column(
                                    &group_data,
                                    z,
                                )),
                                name: Some(group_val.to_string()),
                                mode: None,
                                marker: Some(marker_ir),
                                show_legend: Some(facet_idx == 0),
                                legend_group: Some(group_val.to_string()),
                                scene_ref: Some(scene.clone()),
                            }));
                        }
                    }
                    None => {
                        let marker_ir = MarkerIR {
                            opacity,
                            size,
                            color: Self::resolve_color(facet_idx, color, colors.clone()),
                            shape: Self::resolve_shape(facet_idx, shape, shapes.clone()),
                        };

                        traces.push(TraceIR::Scatter3dPlot(Scatter3dPlotIR {
                            x: ColumnData::Numeric(crate::data::get_numeric_column(&facet_data, x)),
                            y: ColumnData::Numeric(crate::data::get_numeric_column(&facet_data, y)),
                            z: ColumnData::Numeric(crate::data::get_numeric_column(&facet_data, z)),
                            name: None,
                            mode: None,
                            marker: Some(marker_ir),
                            show_legend: Some(false),
                            legend_group: None,
                            scene_ref: Some(scene.clone()),
                        }));
                    }
                }
            }
        } else {
            for (facet_idx, facet_value) in facet_categories.iter().enumerate() {
                let facet_data = crate::data::filter_data_by_group(data, facet_column, facet_value);
                let scene = Self::get_scene_reference(facet_idx);

                match group {
                    Some(group_col) => {
                        let groups =
                            crate::data::get_unique_groups(&facet_data, group_col, sort_groups_by);

                        for group_val in groups.iter() {
                            let group_data = crate::data::filter_data_by_group(
                                &facet_data,
                                group_col,
                                group_val,
                            );

                            let global_idx =
                                global_group_indices.get(group_val).copied().unwrap_or(0);

                            let marker_ir = MarkerIR {
                                opacity,
                                size,
                                color: Self::resolve_color(global_idx, color, colors.clone()),
                                shape: Self::resolve_shape(global_idx, shape, shapes.clone()),
                            };

                            traces.push(TraceIR::Scatter3dPlot(Scatter3dPlotIR {
                                x: ColumnData::Numeric(crate::data::get_numeric_column(
                                    &group_data,
                                    x,
                                )),
                                y: ColumnData::Numeric(crate::data::get_numeric_column(
                                    &group_data,
                                    y,
                                )),
                                z: ColumnData::Numeric(crate::data::get_numeric_column(
                                    &group_data,
                                    z,
                                )),
                                name: Some(group_val.to_string()),
                                mode: None,
                                marker: Some(marker_ir),
                                show_legend: Some(facet_idx == 0),
                                legend_group: Some(group_val.to_string()),
                                scene_ref: Some(scene.clone()),
                            }));
                        }
                    }
                    None => {
                        let marker_ir = MarkerIR {
                            opacity,
                            size,
                            color: Self::resolve_color(facet_idx, color, colors.clone()),
                            shape: Self::resolve_shape(facet_idx, shape, shapes.clone()),
                        };

                        traces.push(TraceIR::Scatter3dPlot(Scatter3dPlotIR {
                            x: ColumnData::Numeric(crate::data::get_numeric_column(&facet_data, x)),
                            y: ColumnData::Numeric(crate::data::get_numeric_column(&facet_data, y)),
                            z: ColumnData::Numeric(crate::data::get_numeric_column(&facet_data, z)),
                            name: None,
                            mode: None,
                            marker: Some(marker_ir),
                            show_legend: Some(false),
                            legend_group: None,
                            scene_ref: Some(scene.clone()),
                        }));
                    }
                }
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
            return ss.get(index).cloned();
        }
        None
    }
}

impl crate::Plot for Scatter3dPlot {
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
            "x" => [1.0, 2.0, 3.0],
            "y" => [4.0, 5.0, 6.0],
            "z" => [7.0, 8.0, 9.0]
        ]
        .unwrap();
        let plot = Scatter3dPlot::builder()
            .data(&df)
            .x("x")
            .y("y")
            .z("z")
            .build();
        assert_eq!(plot.ir_traces().len(), 1);
        assert!(matches!(plot.ir_traces()[0], TraceIR::Scatter3dPlot(_)));
    }

    #[test]
    fn test_with_group() {
        let df = df![
            "x" => [1.0, 2.0, 3.0, 4.0],
            "y" => [4.0, 5.0, 6.0, 7.0],
            "z" => [7.0, 8.0, 9.0, 10.0],
            "g" => ["a", "b", "a", "b"]
        ]
        .unwrap();
        let plot = Scatter3dPlot::builder()
            .data(&df)
            .x("x")
            .y("y")
            .z("z")
            .group("g")
            .build();
        assert_eq!(plot.ir_traces().len(), 2);
    }

    #[test]
    fn test_layout_no_axes_2d() {
        let df = df![
            "x" => [1.0, 2.0],
            "y" => [3.0, 4.0],
            "z" => [5.0, 6.0]
        ]
        .unwrap();
        let plot = Scatter3dPlot::builder()
            .data(&df)
            .x("x")
            .y("y")
            .z("z")
            .build();
        assert!(plot.ir_layout().axes_2d.is_none());
    }

    #[test]
    fn test_resolve_color_both_none() {
        let result = Scatter3dPlot::resolve_color(0, None, None);
        assert!(result.is_none());
    }
}
