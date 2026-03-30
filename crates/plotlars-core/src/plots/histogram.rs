use bon::bon;

use polars::frame::DataFrame;

use crate::{
    components::{Axis, FacetConfig, FacetScales, Legend, Rgb, Text, DEFAULT_PLOTLY_COLORS},
    ir::data::ColumnData,
    ir::layout::LayoutIR,
    ir::marker::MarkerIR,
    ir::trace::{BinsIR, HistogramIR, TraceIR},
};

/// A structure representing a histogram.
///
/// The `Histogram` struct facilitates the creation and customization of histograms with various options
/// for data selection, layout configuration, and aesthetic adjustments. It supports grouping of data,
/// opacity settings, and customizable markers and colors.
///
/// # Arguments
///
/// * `data` - A reference to the `DataFrame` containing the data to be plotted.
/// * `x` - A string slice specifying the column name to be used for the x-axis (independent variable).
/// * `group` - An optional string slice specifying the column name to be used for grouping data points.
/// * `sort_groups_by` - Optional comparator `fn(&str, &str) -> std::cmp::Ordering` to control group ordering. Groups are sorted lexically by default.
/// * `facet` - An optional string slice specifying the column name to be used for faceting (creating multiple subplots).
/// * `facet_config` - An optional reference to a `FacetConfig` struct for customizing facet behavior (grid dimensions, scales, gaps, etc.).
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
/// use plotlars::{Axis, Histogram, Legend, Plot, Rgb, Text, TickDirection};
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
///     .show_grid(true)
///     .value_thousands(true)
///     .tick_direction(TickDirection::OutSide);
///
/// Histogram::builder()
///     .data(&dataset)
///     .x("body_mass_g")
///     .group("species")
///     .opacity(0.5)
///     .colors(vec![
///         Rgb(255, 165, 0),
///         Rgb(147, 112, 219),
///         Rgb(46, 139, 87),
///     ])
///     .plot_title(
///         Text::from("Histogram")
///             .font("Arial")
///             .size(18)
///     )
///     .x_title(
///         Text::from("body mass (g)")
///             .font("Arial")
///             .size(15)
///     )
///     .y_title(
///         Text::from("count")
///             .font("Arial")
///             .size(15)
///     )
///     .legend_title(
///         Text::from("species")
///             .font("Arial")
///             .size(15)
///     )
///     .x_axis(&axis)
///     .y_axis(&axis)
///     .legend(
///         &Legend::new()
///             .x(0.9)
///     )
///     .build()
///     .plot();
/// ```
///
/// ![Example](https://imgur.com/w2oiuIo.png)
#[derive(Clone)]
#[allow(dead_code)]
pub struct Histogram {
    traces: Vec<TraceIR>,
    layout: LayoutIR,
}

#[bon]
impl Histogram {
    #[builder(on(String, into), on(Text, into))]
    pub fn new(
        data: &DataFrame,
        x: &str,
        group: Option<&str>,
        sort_groups_by: Option<fn(&str, &str) -> std::cmp::Ordering>,
        facet: Option<&str>,
        facet_config: Option<&FacetConfig>,
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
            bar_mode: Some(crate::components::BarMode::Overlay),
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
                    group,
                    sort_groups_by,
                    facet_column,
                    &config,
                    opacity,
                    color,
                    colors.clone(),
                )
            }
            None => Self::create_ir_traces(data, x, group, sort_groups_by, opacity, color, colors),
        };

        Self { traces, layout }
    }

    fn should_use_global_bins(scales: &FacetScales) -> bool {
        match scales {
            FacetScales::Fixed | FacetScales::FreeY => true,
            FacetScales::Free | FacetScales::FreeX => false,
        }
    }
    fn calculate_global_bins_ir(data: &DataFrame, x: &str) -> BinsIR {
        let x_data = crate::data::get_numeric_column(data, x);

        let values: Vec<f32> = x_data.iter().filter_map(|v| *v).collect();

        if values.is_empty() {
            return BinsIR {
                start: 0.0,
                end: 1.0,
                size: 0.1,
            };
        }

        let min = values.iter().cloned().fold(f32::INFINITY, f32::min);
        let max = values.iter().cloned().fold(f32::NEG_INFINITY, f32::max);

        let n = values.len() as f32;
        let nbins = (n.sqrt().ceil() as usize).clamp(10, 100);

        let range = max - min;
        let bin_size = if range > 0.0 {
            range / nbins as f32
        } else {
            1.0
        };

        BinsIR {
            start: min as f64,
            end: max as f64,
            size: bin_size as f64,
        }
    }

    fn create_ir_traces(
        data: &DataFrame,
        x: &str,
        group: Option<&str>,
        sort_groups_by: Option<fn(&str, &str) -> std::cmp::Ordering>,
        opacity: Option<f64>,
        color: Option<Rgb>,
        colors: Option<Vec<Rgb>>,
    ) -> Vec<TraceIR> {
        let mut traces = Vec::new();

        match group {
            Some(group_col) => {
                let groups = crate::data::get_unique_groups(data, group_col, sort_groups_by);

                for (i, group_name) in groups.iter().enumerate() {
                    let subset = crate::data::filter_data_by_group(data, group_col, group_name);

                    let marker_ir = MarkerIR {
                        opacity,
                        size: None,
                        color: Self::resolve_color(i, color, colors.clone()),
                        shape: None,
                    };

                    traces.push(TraceIR::Histogram(HistogramIR {
                        x: ColumnData::Numeric(crate::data::get_numeric_column(&subset, x)),
                        name: Some(group_name.to_string()),
                        marker: Some(marker_ir),
                        bins: None,
                        show_legend: None,
                        legend_group: None,
                        subplot_ref: None,
                    }));
                }
            }
            None => {
                let marker_ir = MarkerIR {
                    opacity,
                    size: None,
                    color: Self::resolve_color(0, color, colors),
                    shape: None,
                };

                traces.push(TraceIR::Histogram(HistogramIR {
                    x: ColumnData::Numeric(crate::data::get_numeric_column(data, x)),
                    name: None,
                    marker: Some(marker_ir),
                    bins: None,
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
        group: Option<&str>,
        sort_groups_by: Option<fn(&str, &str) -> std::cmp::Ordering>,
        facet_column: &str,
        config: &FacetConfig,
        opacity: Option<f64>,
        color: Option<Rgb>,
        colors: Option<Vec<Rgb>>,
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

        let global_bins = Self::should_use_global_bins(&config.scales);
        let bins_ir = if global_bins {
            Some(Self::calculate_global_bins_ir(data, x))
        } else {
            None
        };

        let mut traces = Vec::new();

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
                        let group_data =
                            crate::data::filter_data_by_group(&facet_data, group_col, group_val);

                        let global_idx = global_group_indices.get(group_val).copied().unwrap_or(0);

                        let marker_ir = MarkerIR {
                            opacity,
                            size: None,
                            color: Self::resolve_color(global_idx, color, colors.clone()),
                            shape: None,
                        };

                        traces.push(TraceIR::Histogram(HistogramIR {
                            x: ColumnData::Numeric(crate::data::get_numeric_column(&group_data, x)),
                            name: Some(group_val.to_string()),
                            marker: Some(marker_ir),
                            bins: bins_ir.clone(),
                            show_legend: Some(facet_idx == 0),
                            legend_group: Some(group_val.to_string()),
                            subplot_ref: Some(subplot_ref.clone()),
                        }));
                    }
                }
                None => {
                    let marker_ir = MarkerIR {
                        opacity,
                        size: None,
                        color: Self::resolve_color(facet_idx, color, colors.clone()),
                        shape: None,
                    };

                    traces.push(TraceIR::Histogram(HistogramIR {
                        x: ColumnData::Numeric(crate::data::get_numeric_column(&facet_data, x)),
                        name: None,
                        marker: Some(marker_ir),
                        bins: bins_ir.clone(),
                        show_legend: Some(false),
                        legend_group: None,
                        subplot_ref: Some(subplot_ref.clone()),
                    }));
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
}

impl crate::Plot for Histogram {
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
    fn test_basic_one_trace() {
        let df = df!["x" => [1.0, 2.0, 3.0, 4.0, 5.0]].unwrap();
        let plot = Histogram::builder().data(&df).x("x").build();
        assert_eq!(plot.ir_traces().len(), 1);
        assert!(matches!(plot.ir_traces()[0], TraceIR::Histogram(_)));
    }

    #[test]
    fn test_with_group() {
        let df = df![
            "x" => [1.0, 2.0, 3.0, 4.0],
            "g" => ["a", "b", "a", "b"]
        ]
        .unwrap();
        let plot = Histogram::builder().data(&df).x("x").group("g").build();
        assert_eq!(plot.ir_traces().len(), 2);
    }

    #[test]
    fn test_resolve_color_singular_priority() {
        let result = Histogram::resolve_color(0, Some(Rgb(255, 0, 0)), Some(vec![Rgb(0, 0, 255)]));
        assert_rgb(result, 255, 0, 0);
    }

    #[test]
    fn test_layout_has_axes() {
        let df = df!["x" => [1.0, 2.0, 3.0]].unwrap();
        let plot = Histogram::builder().data(&df).x("x").build();
        assert!(plot.ir_layout().axes_2d.is_some());
    }

    #[test]
    fn test_faceted_trace_count() {
        let df = df![
            "x" => [1.0, 2.0, 3.0, 4.0, 5.0, 6.0],
            "facet_col" => ["a", "a", "a", "b", "b", "b"]
        ]
        .unwrap();
        let plot = Histogram::builder()
            .data(&df)
            .x("x")
            .facet("facet_col")
            .build();
        // 2 facets, no group = 2 traces
        assert_eq!(plot.ir_traces().len(), 2);
    }
}
