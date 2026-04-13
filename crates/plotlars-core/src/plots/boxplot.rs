use bon::bon;

use polars::frame::DataFrame;

use crate::{
    components::{Axis, FacetConfig, Legend, Orientation, Rgb, Text, DEFAULT_PLOTLY_COLORS},
    ir::data::ColumnData,
    ir::layout::LayoutIR,
    ir::marker::MarkerIR,
    ir::trace::{BoxPlotIR, TraceIR},
};

/// A structure representing a box plot.
///
/// The `BoxPlot` struct facilitates the creation and customization of box plots with various options
/// for data selection, layout configuration, and aesthetic adjustments. It supports both horizontal
/// and vertical orientations, grouping of data, display of individual data points with jitter and offset,
/// opacity settings, and customizable markers and colors.
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
/// * `labels` - A string slice specifying the column name to be used for the x-axis (independent variable).
/// * `values` - A string slice specifying the column name to be used for the y-axis (dependent variable).
/// * `orientation` - An optional `Orientation` enum specifying whether the plot should be horizontal or vertical.
/// * `group` - An optional string slice specifying the column name to be used for grouping data points.
/// * `sort_groups_by` - Optional comparator `fn(&str, &str) -> std::cmp::Ordering` to control group ordering. Groups are sorted lexically by default.
/// * `facet` - An optional string slice specifying the column name to be used for creating facets (small multiples).
/// * `facet_config` - An optional reference to a `FacetConfig` struct for customizing facet layout and behavior.
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
/// ![Example](https://imgur.com/jdA3g9r.png)
#[derive(Clone)]
#[allow(dead_code)]
pub struct BoxPlot {
    traces: Vec<TraceIR>,
    layout: LayoutIR,
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
        sort_groups_by: Option<fn(&str, &str) -> std::cmp::Ordering>,
        facet: Option<&str>,
        facet_config: Option<&FacetConfig>,
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
            box_mode: if group.is_some() {
                Some(crate::ir::layout::BoxModeIR::Group)
            } else {
                None
            },
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
                    labels,
                    values,
                    orientation.clone(),
                    group,
                    sort_groups_by,
                    facet_column,
                    &config,
                    box_points,
                    point_offset,
                    jitter,
                    opacity,
                    color,
                    colors.clone(),
                )
            }
            None => Self::create_ir_traces(
                data,
                labels,
                values,
                orientation,
                group,
                sort_groups_by,
                box_points,
                point_offset,
                jitter,
                opacity,
                color,
                colors,
            ),
        };

        Self { traces, layout }
    }
}

#[bon]
impl BoxPlot {
    #[builder(
        start_fn = try_builder,
        finish_fn = try_build,
        builder_type = BoxPlotTryBuilder,
        on(String, into),
        on(Text, into),
    )]
    pub fn try_new(
        data: &DataFrame,
        labels: &str,
        values: &str,
        orientation: Option<Orientation>,
        group: Option<&str>,
        sort_groups_by: Option<fn(&str, &str) -> std::cmp::Ordering>,
        facet: Option<&str>,
        facet_config: Option<&FacetConfig>,
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
    ) -> Result<Self, crate::io::PlotlarsError> {
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            Self::__orig_new(
                data,
                labels,
                values,
                orientation,
                group,
                sort_groups_by,
                facet,
                facet_config,
                box_points,
                point_offset,
                jitter,
                opacity,
                color,
                colors,
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

impl BoxPlot {
    #[allow(clippy::too_many_arguments)]
    fn create_ir_traces(
        data: &DataFrame,
        labels: &str,
        values: &str,
        orientation: Option<Orientation>,
        group: Option<&str>,
        sort_groups_by: Option<fn(&str, &str) -> std::cmp::Ordering>,
        box_points: Option<bool>,
        point_offset: Option<f64>,
        jitter: Option<f64>,
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

                    traces.push(TraceIR::BoxPlot(BoxPlotIR {
                        labels: ColumnData::String(crate::data::get_string_column(&subset, labels)),
                        values: ColumnData::Numeric(crate::data::get_numeric_column(
                            &subset, values,
                        )),
                        name: Some(group_name.to_string()),
                        orientation: orientation.clone(),
                        marker: Some(marker_ir),
                        box_points,
                        point_offset,
                        jitter,
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

                traces.push(TraceIR::BoxPlot(BoxPlotIR {
                    labels: ColumnData::String(crate::data::get_string_column(data, labels)),
                    values: ColumnData::Numeric(crate::data::get_numeric_column(data, values)),
                    name: None,
                    orientation: orientation.clone(),
                    marker: Some(marker_ir),
                    box_points,
                    point_offset,
                    jitter,
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
        labels: &str,
        values: &str,
        orientation: Option<Orientation>,
        group: Option<&str>,
        sort_groups_by: Option<fn(&str, &str) -> std::cmp::Ordering>,
        facet_column: &str,
        config: &FacetConfig,
        box_points: Option<bool>,
        point_offset: Option<f64>,
        jitter: Option<f64>,
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

                        traces.push(TraceIR::BoxPlot(BoxPlotIR {
                            labels: ColumnData::String(crate::data::get_string_column(
                                &group_data,
                                labels,
                            )),
                            values: ColumnData::Numeric(crate::data::get_numeric_column(
                                &group_data,
                                values,
                            )),
                            name: Some(group_val.to_string()),
                            orientation: orientation.clone(),
                            marker: Some(marker_ir),
                            box_points,
                            point_offset,
                            jitter,
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

                    traces.push(TraceIR::BoxPlot(BoxPlotIR {
                        labels: ColumnData::String(crate::data::get_string_column(
                            &facet_data,
                            labels,
                        )),
                        values: ColumnData::Numeric(crate::data::get_numeric_column(
                            &facet_data,
                            values,
                        )),
                        name: None,
                        orientation: orientation.clone(),
                        marker: Some(marker_ir),
                        box_points,
                        point_offset,
                        jitter,
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

impl crate::Plot for BoxPlot {
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
        let df = df![
            "labels" => ["a", "b", "c"],
            "values" => [1.0, 2.0, 3.0]
        ]
        .unwrap();
        let plot = BoxPlot::builder()
            .data(&df)
            .labels("labels")
            .values("values")
            .build();
        assert_eq!(plot.ir_traces().len(), 1);
        assert!(matches!(plot.ir_traces()[0], TraceIR::BoxPlot(_)));
    }

    #[test]
    fn test_with_group() {
        let df = df![
            "labels" => ["a", "b", "a", "b"],
            "values" => [1.0, 2.0, 3.0, 4.0],
            "g" => ["x", "x", "y", "y"]
        ]
        .unwrap();
        let plot = BoxPlot::builder()
            .data(&df)
            .labels("labels")
            .values("values")
            .group("g")
            .build();
        assert_eq!(plot.ir_traces().len(), 2);
    }

    #[test]
    fn test_resolve_color_singular_priority() {
        let result = BoxPlot::resolve_color(0, Some(Rgb(255, 0, 0)), Some(vec![Rgb(0, 0, 255)]));
        assert_rgb(result, 255, 0, 0);
    }

    #[test]
    fn test_resolve_color_both_none() {
        let result = BoxPlot::resolve_color(0, None, None);
        assert!(result.is_none());
    }

    #[test]
    fn test_layout_has_axes() {
        let df = df![
            "labels" => ["a", "b"],
            "values" => [1.0, 2.0]
        ]
        .unwrap();
        let plot = BoxPlot::builder()
            .data(&df)
            .labels("labels")
            .values("values")
            .build();
        assert!(plot.ir_layout().axes_2d.is_some());
    }
}
