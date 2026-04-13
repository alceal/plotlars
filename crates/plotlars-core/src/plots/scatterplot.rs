use bon::bon;

use polars::frame::DataFrame;

use crate::{
    components::{Axis, FacetConfig, Legend, Rgb, Shape, Text, DEFAULT_PLOTLY_COLORS},
    ir::data::ColumnData,
    ir::layout::LayoutIR,
    ir::marker::MarkerIR,
    ir::trace::{ScatterPlotIR, TraceIR},
};

/// A structure representing a scatter plot.
///
/// The `ScatterPlot` struct facilitates the creation and customization of scatter plots with various options
/// for data selection, grouping, layout configuration, and aesthetic adjustments. It supports grouping of data,
/// customization of marker shapes, colors, sizes, opacity settings, and comprehensive layout customization
/// including titles, axes, and legends.
///
/// # Backend Support
///
/// | Backend | Supported |
/// |---------|-----------|
/// | Plotly  | Yes       |
/// | Plotters| Yes       |
///
/// # Arguments
///
/// * `data` - A reference to the `DataFrame` containing the data to be plotted.
/// * `x` - A string slice specifying the column name to be used for the x-axis (independent variable).
/// * `y` - A string slice specifying the column name to be used for the y-axis (dependent variable).
/// * `group` - An optional string slice specifying the column name to be used for grouping data points.
/// * `sort_groups_by` - Optional comparator `fn(&str, &str) -> std::cmp::Ordering` to control group ordering. Groups are sorted lexically by default.
/// * `facet` - An optional string slice specifying the column name to be used for faceting (creating multiple subplots).
/// * `facet_config` - An optional reference to a `FacetConfig` struct for customizing facet behavior (grid dimensions, scales, gaps, etc.).
/// * `opacity` - An optional `f64` value specifying the opacity of the plot markers (range: 0.0 to 1.0).
/// * `size` - An optional `usize` specifying the size of the markers.
/// * `color` - An optional `Rgb` value specifying the color of the markers. This is used when `group` is not specified.
/// * `colors` - An optional vector of `Rgb` values specifying the colors for the markers. This is used when `group` is specified to differentiate between groups.
/// * `shape` - An optional `Shape` specifying the shape of the markers. This is used when `group` is not specified.
/// * `shapes` - An optional vector of `Shape` values specifying multiple shapes for the markers when plotting multiple groups.
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
/// use plotlars::{Axis, Legend, Plot, Rgb, ScatterPlot, Shape, Text, TickDirection};
/// use polars::prelude::*;
///
/// let dataset = LazyCsvReader::new(PlRefPath::new("data/penguins.csv"))
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
/// let axis = Axis::new()
///     .show_line(true)
///     .tick_direction(TickDirection::OutSide)
///     .value_thousands(true);
///
/// ScatterPlot::builder()
///     .data(&dataset)
///     .x("body_mass_g")
///     .y("flipper_length_mm")
///     .group("species")
///     .sort_groups_by(|a, b| {
///         if a.len() == b.len() {
///             a.cmp(b)
///         } else {
///             a.len().cmp(&b.len())
///         }
///     })
///     .opacity(0.5)
///     .size(12)
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
///     .plot_title(
///         Text::from("Scatter Plot")
///             .font("Arial")
///             .size(20)
///             .x(0.065)
///     )
///     .x_title("body mass (g)")
///     .y_title("flipper length (mm)")
///     .legend_title("species")
///     .x_axis(
///         &axis.clone()
///             .value_range(vec![2500.0, 6500.0])
///     )
///     .y_axis(
///         &axis.clone()
///             .value_range(vec![170.0, 240.0])
///     )
///     .legend(
///         &Legend::new()
///             .x(0.85)
///             .y(0.15)
///     )
///     .build()
///     .plot();
/// ```
///
/// ![Example](https://imgur.com/9jfO8RU.png)
#[derive(Clone)]
#[allow(dead_code)]
pub struct ScatterPlot {
    traces: Vec<TraceIR>,
    layout: LayoutIR,
}

#[bon]
impl ScatterPlot {
    #[builder(on(String, into), on(Text, into))]
    pub fn new(
        data: &DataFrame,
        x: &str,
        y: &str,
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
        x_title: Option<Text>,
        y_title: Option<Text>,
        legend_title: Option<Text>,
        x_axis: Option<&Axis>,
        y_axis: Option<&Axis>,
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
                legend_title: legend_title.clone(),
                legend: legend.cloned(),
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
            legend_title: if grid.is_some() {
                None
            } else {
                legend_title.clone()
            },
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
                    group,
                    sort_groups_by,
                    facet_column,
                    &config,
                    opacity,
                    size,
                    color,
                    colors.clone(),
                    shape,
                    shapes.clone(),
                )
            }
            None => Self::create_ir_traces(
                data,
                x,
                y,
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

        Self { traces, layout }
    }
}

#[bon]
impl ScatterPlot {
    #[builder(
        start_fn = try_builder,
        finish_fn = try_build,
        builder_type = ScatterPlotTryBuilder,
        on(String, into),
        on(Text, into),
    )]
    pub fn try_new(
        data: &DataFrame,
        x: &str,
        y: &str,
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
        x_title: Option<Text>,
        y_title: Option<Text>,
        legend_title: Option<Text>,
        x_axis: Option<&Axis>,
        y_axis: Option<&Axis>,
        legend: Option<&Legend>,
    ) -> Result<Self, crate::io::PlotlarsError> {
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            Self::__orig_new(
                data,
                x,
                y,
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
                x_title,
                y_title,
                legend_title,
                x_axis,
                y_axis,
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

impl ScatterPlot {
    #[allow(clippy::too_many_arguments)]
    fn create_ir_traces(
        data: &DataFrame,
        x: &str,
        y: &str,
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

                    traces.push(TraceIR::ScatterPlot(ScatterPlotIR {
                        x: ColumnData::Numeric(crate::data::get_numeric_column(&subset, x)),
                        y: ColumnData::Numeric(crate::data::get_numeric_column(&subset, y)),
                        name: Some(group_name.to_string()),
                        marker: Some(marker_ir),
                        fill: None,
                        show_legend: None,
                        legend_group: None,
                        subplot_ref: None,
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

                traces.push(TraceIR::ScatterPlot(ScatterPlotIR {
                    x: ColumnData::Numeric(crate::data::get_numeric_column(data, x)),
                    y: ColumnData::Numeric(crate::data::get_numeric_column(data, y)),
                    name: None,
                    marker: Some(marker_ir),
                    fill: None,
                    show_legend: None,
                    legend_group: None,
                    subplot_ref: None,
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
                "Facet column '{}' has {} unique values, but plotly.rs supports maximum {} subplots",
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
                         Expected {} colors for {} facets, but got {} colors. \
                         Each facet must be assigned exactly one color.",
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
                let subplot_ref = format!(
                    "{}{}",
                    crate::faceting::get_axis_reference(facet_idx, "x"),
                    crate::faceting::get_axis_reference(facet_idx, "y")
                );

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

                        traces.push(TraceIR::ScatterPlot(ScatterPlotIR {
                            x: ColumnData::Numeric(crate::data::get_numeric_column(&other_data, x)),
                            y: ColumnData::Numeric(crate::data::get_numeric_column(&other_data, y)),
                            name: None,
                            marker: Some(marker_ir),
                            fill: None,
                            show_legend: Some(false),
                            legend_group: None,
                            subplot_ref: Some(subplot_ref.clone()),
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

                            traces.push(TraceIR::ScatterPlot(ScatterPlotIR {
                                x: ColumnData::Numeric(crate::data::get_numeric_column(
                                    &group_data,
                                    x,
                                )),
                                y: ColumnData::Numeric(crate::data::get_numeric_column(
                                    &group_data,
                                    y,
                                )),
                                name: Some(group_val.to_string()),
                                marker: Some(marker_ir),
                                fill: None,
                                show_legend: Some(facet_idx == 0),
                                legend_group: Some(group_val.to_string()),
                                subplot_ref: Some(subplot_ref.clone()),
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

                        traces.push(TraceIR::ScatterPlot(ScatterPlotIR {
                            x: ColumnData::Numeric(crate::data::get_numeric_column(&facet_data, x)),
                            y: ColumnData::Numeric(crate::data::get_numeric_column(&facet_data, y)),
                            name: None,
                            marker: Some(marker_ir),
                            fill: None,
                            show_legend: Some(false),
                            legend_group: None,
                            subplot_ref: Some(subplot_ref.clone()),
                        }));
                    }
                }
            }
        } else {
            for (facet_idx, facet_value) in facet_categories.iter().enumerate() {
                let facet_data = crate::data::filter_data_by_group(data, facet_column, facet_value);

                let subplot_ref = format!(
                    "{}{}",
                    crate::faceting::get_axis_reference(facet_idx, "x"),
                    crate::faceting::get_axis_reference(facet_idx, "y")
                );

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

                            traces.push(TraceIR::ScatterPlot(ScatterPlotIR {
                                x: ColumnData::Numeric(crate::data::get_numeric_column(
                                    &group_data,
                                    x,
                                )),
                                y: ColumnData::Numeric(crate::data::get_numeric_column(
                                    &group_data,
                                    y,
                                )),
                                name: Some(group_val.to_string()),
                                marker: Some(marker_ir),
                                fill: None,
                                show_legend: Some(facet_idx == 0),
                                legend_group: Some(group_val.to_string()),
                                subplot_ref: Some(subplot_ref.clone()),
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

                        traces.push(TraceIR::ScatterPlot(ScatterPlotIR {
                            x: ColumnData::Numeric(crate::data::get_numeric_column(&facet_data, x)),
                            y: ColumnData::Numeric(crate::data::get_numeric_column(&facet_data, y)),
                            name: None,
                            marker: Some(marker_ir),
                            fill: None,
                            show_legend: Some(false),
                            legend_group: None,
                            subplot_ref: Some(subplot_ref.clone()),
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

impl crate::Plot for ScatterPlot {
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

    fn assert_rgb(actual: Option<Rgb>, r: u8, g: u8, b: u8) {
        let c = actual.expect("expected Some(Rgb)");
        assert_eq!((c.0, c.1, c.2), (r, g, b));
    }

    #[test]
    fn test_resolve_color_singular_priority() {
        let result =
            ScatterPlot::resolve_color(0, Some(Rgb(255, 0, 0)), Some(vec![Rgb(0, 0, 255)]));
        assert_rgb(result, 255, 0, 0);
    }

    #[test]
    fn test_resolve_color_from_vec() {
        let result = ScatterPlot::resolve_color(
            1,
            None,
            Some(vec![Rgb(1, 0, 0), Rgb(0, 1, 0), Rgb(0, 0, 1)]),
        );
        assert_rgb(result, 0, 1, 0);
    }

    #[test]
    fn test_resolve_color_out_of_bounds() {
        let result = ScatterPlot::resolve_color(5, None, Some(vec![Rgb(1, 0, 0)]));
        assert!(result.is_none());
    }

    #[test]
    fn test_resolve_color_both_none() {
        let result = ScatterPlot::resolve_color(0, None, None);
        assert!(result.is_none());
    }

    #[test]
    fn test_resolve_shape_singular_priority() {
        let result = ScatterPlot::resolve_shape(0, Some(Shape::Circle), Some(vec![Shape::Square]));
        assert!(matches!(result, Some(Shape::Circle)));
    }

    #[test]
    fn test_resolve_shape_from_vec() {
        let result = ScatterPlot::resolve_shape(
            1,
            None,
            Some(vec![Shape::Circle, Shape::Diamond, Shape::Square]),
        );
        assert!(matches!(result, Some(Shape::Diamond)));
    }

    #[test]
    fn test_resolve_shape_out_of_bounds() {
        let result = ScatterPlot::resolve_shape(5, None, Some(vec![Shape::Circle]));
        assert!(result.is_none());
    }

    #[test]
    fn test_resolve_shape_both_none() {
        let result = ScatterPlot::resolve_shape(0, None, None);
        assert!(result.is_none());
    }

    #[test]
    fn test_no_group_one_trace() {
        let df = df!["x" => [1.0, 2.0, 3.0], "y" => [4.0, 5.0, 6.0]].unwrap();
        let plot = ScatterPlot::builder().data(&df).x("x").y("y").build();
        assert_eq!(plot.ir_traces().len(), 1);
    }

    #[test]
    fn test_with_group_multiple_traces() {
        let df = df![
            "x" => [1.0, 2.0, 3.0, 4.0],
            "y" => [4.0, 5.0, 6.0, 7.0],
            "g" => ["a", "b", "a", "b"]
        ]
        .unwrap();
        let plot = ScatterPlot::builder()
            .data(&df)
            .x("x")
            .y("y")
            .group("g")
            .build();
        assert_eq!(plot.ir_traces().len(), 2);
    }

    #[test]
    fn test_faceted_trace_count() {
        let df = df![
            "x" => [1.0, 2.0, 3.0, 4.0, 5.0, 6.0],
            "y" => [10.0, 20.0, 30.0, 40.0, 50.0, 60.0],
            "f" => ["a", "b", "c", "a", "b", "c"]
        ]
        .unwrap();
        let plot = ScatterPlot::builder()
            .data(&df)
            .x("x")
            .y("y")
            .facet("f")
            .build();
        assert_eq!(plot.ir_traces().len(), 3);
    }

    #[test]
    fn test_faceted_with_group_trace_count() {
        let df = df![
            "x" => [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0],
            "y" => [10.0, 20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0],
            "f" => ["f1", "f1", "f1", "f1", "f2", "f2", "f2", "f2"],
            "g" => ["g1", "g2", "g1", "g2", "g1", "g2", "g1", "g2"]
        ]
        .unwrap();
        let plot = ScatterPlot::builder()
            .data(&df)
            .x("x")
            .y("y")
            .facet("f")
            .group("g")
            .build();
        // 2 facets * 2 groups = 4 traces
        assert_eq!(plot.ir_traces().len(), 4);
    }

    #[test]
    fn test_faceted_show_legend_first_only() {
        let df = df![
            "x" => [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0],
            "y" => [10.0, 20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0],
            "f" => ["f1", "f1", "f1", "f1", "f2", "f2", "f2", "f2"],
            "g" => ["g1", "g2", "g1", "g2", "g1", "g2", "g1", "g2"]
        ]
        .unwrap();
        let plot = ScatterPlot::builder()
            .data(&df)
            .x("x")
            .y("y")
            .facet("f")
            .group("g")
            .build();

        for trace in plot.ir_traces() {
            match trace {
                TraceIR::ScatterPlot(ir) => {
                    let subplot = ir.subplot_ref.as_deref().unwrap();
                    if subplot == "xy" {
                        // First facet -> show legend
                        assert_eq!(ir.show_legend, Some(true));
                    } else {
                        // Later facets -> hide legend
                        assert_eq!(ir.show_legend, Some(false));
                    }
                }
                _ => panic!("expected ScatterPlot trace"),
            }
        }
    }

    #[test]
    fn test_faceted_subplot_ref() {
        let df = df![
            "x" => [1.0, 2.0, 3.0, 4.0],
            "y" => [10.0, 20.0, 30.0, 40.0],
            "f" => ["a", "b", "a", "b"]
        ]
        .unwrap();
        let plot = ScatterPlot::builder()
            .data(&df)
            .x("x")
            .y("y")
            .facet("f")
            .build();

        let refs: Vec<&str> = plot
            .ir_traces()
            .iter()
            .map(|t| match t {
                TraceIR::ScatterPlot(ir) => ir.subplot_ref.as_deref().unwrap(),
                _ => panic!("expected ScatterPlot trace"),
            })
            .collect();
        assert_eq!(refs[0], "xy");
        assert_eq!(refs[1], "x2y2");
    }

    #[test]
    #[should_panic(expected = "maximum")]
    fn test_max_facets_panics() {
        let facet_values: Vec<&str> = (0..9)
            .map(|i| match i {
                0 => "a",
                1 => "b",
                2 => "c",
                3 => "d",
                4 => "e",
                5 => "f",
                6 => "g",
                7 => "h",
                _ => "i",
            })
            .collect();
        let n = facet_values.len();
        let x_vals: Vec<f64> = (0..n).map(|i| i as f64).collect();
        let y_vals: Vec<f64> = (0..n).map(|i| i as f64 * 10.0).collect();
        let df = DataFrame::new(
            n,
            vec![
                Column::new("x".into(), &x_vals),
                Column::new("y".into(), &y_vals),
                Column::new("f".into(), &facet_values),
            ],
        )
        .unwrap();
        ScatterPlot::builder()
            .data(&df)
            .x("x")
            .y("y")
            .facet("f")
            .build();
    }

    #[test]
    #[should_panic(expected = "colors.len() must equal number of facets")]
    fn test_faceted_colors_mismatch_panics() {
        let df = df![
            "x" => [1.0, 2.0, 3.0],
            "y" => [10.0, 20.0, 30.0],
            "f" => ["a", "b", "c"]
        ]
        .unwrap();
        ScatterPlot::builder()
            .data(&df)
            .x("x")
            .y("y")
            .facet("f")
            .colors(vec![Rgb(255, 0, 0), Rgb(0, 255, 0)])
            .build();
    }
}
