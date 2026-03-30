use bon::bon;

use polars::frame::DataFrame;

use crate::{
    components::{
        FacetConfig, Fill, Legend, Line as LineStyle, Mode, Rgb, Shape, Text, DEFAULT_PLOTLY_COLORS,
    },
    ir::data::ColumnData,
    ir::layout::LayoutIR,
    ir::line::LineIR,
    ir::marker::MarkerIR,
    ir::trace::{ScatterPolarIR, TraceIR},
};

/// A structure representing a scatter polar plot.
///
/// The `ScatterPolar` struct facilitates the creation and customization of polar scatter plots with various options
/// for data selection, grouping, layout configuration, and aesthetic adjustments. It supports grouping of data,
/// customization of marker shapes, colors, sizes, line styles, and comprehensive layout customization
/// including titles and legends.
///
/// # Arguments
///
/// * `data` - A reference to the `DataFrame` containing the data to be plotted.
/// * `theta` - A string slice specifying the column name to be used for the angular coordinates (in degrees).
/// * `r` - A string slice specifying the column name to be used for the radial coordinates.
/// * `group` - An optional string slice specifying the column name to be used for grouping data points.
/// * `sort_groups_by` - Optional comparator `fn(&str, &str) -> std::cmp::Ordering` to control group ordering. Groups are sorted lexically by default.
/// * `facet` - An optional string slice specifying the column name to be used for faceting (creating multiple subplots).
/// * `facet_config` - An optional reference to a `FacetConfig` struct for customizing facet behavior (grid dimensions, scales, gaps, etc.).
/// * `mode` - An optional `Mode` specifying the drawing mode (lines, markers, or both). Defaults to markers.
/// * `opacity` - An optional `f64` value specifying the opacity of the plot elements (range: 0.0 to 1.0).
/// * `fill` - An optional `Fill` type specifying how to fill the area under the trace.
/// * `size` - An optional `usize` specifying the size of the markers.
/// * `color` - An optional `Rgb` value specifying the color of the markers. This is used when `group` is not specified.
/// * `colors` - An optional vector of `Rgb` values specifying the colors for the markers. This is used when `group` is specified to differentiate between groups.
/// * `shape` - An optional `Shape` specifying the shape of the markers. This is used when `group` is not specified.
/// * `shapes` - An optional vector of `Shape` values specifying multiple shapes for the markers when plotting multiple groups.
/// * `width` - An optional `f64` specifying the width of the lines.
/// * `line` - An optional `LineStyle` specifying the style of the line (e.g., solid, dashed).
/// * `lines` - An optional vector of `LineStyle` enums specifying the styles of lines for multiple traces.
/// * `plot_title` - An optional `Text` struct specifying the title of the plot.
/// * `legend_title` - An optional `Text` struct specifying the title of the legend.
/// * `legend` - An optional reference to a `Legend` struct for customizing the legend of the plot (e.g., positioning, font, etc.).
///
/// # Example
///
/// ```rust
/// use plotlars::{Legend, Line, Mode, Plot, Rgb, ScatterPolar, Shape, Text};
/// use polars::prelude::*;
///
/// let dataset = LazyCsvReader::new(PlRefPath::new("data/product_comparison_polar.csv"))
///     .finish()
///     .unwrap()
///     .collect()
///     .unwrap();
///
/// ScatterPolar::builder()
///     .data(&dataset)
///     .theta("angle")
///     .r("score")
///     .group("product")
///     .mode(Mode::LinesMarkers)
///     .colors(vec![
///         Rgb(255, 99, 71),
///         Rgb(60, 179, 113),
///     ])
///     .shapes(vec![
///         Shape::Circle,
///         Shape::Square,
///     ])
///     .lines(vec![
///         Line::Solid,
///         Line::Dash,
///     ])
///     .width(2.5)
///     .size(8)
///     .plot_title(
///         Text::from("Scatter Polar Plot")
///             .font("Arial")
///             .size(24)
///     )
///     .legend_title(
///         Text::from("Products")
///             .font("Arial")
///             .size(14)
///     )
///     .legend(
///         &Legend::new()
///             .x(0.85)
///             .y(0.95)
///     )
///     .build()
///     .plot();
/// ```
///
/// ![Example](https://imgur.com/kl1pY9c.png)
#[derive(Clone)]
#[allow(dead_code)]
pub struct ScatterPolar {
    traces: Vec<TraceIR>,
    layout: LayoutIR,
}

#[bon]
impl ScatterPolar {
    #[builder(on(String, into), on(Text, into))]
    pub fn new(
        data: &DataFrame,
        theta: &str,
        r: &str,
        group: Option<&str>,
        sort_groups_by: Option<fn(&str, &str) -> std::cmp::Ordering>,
        facet: Option<&str>,
        facet_config: Option<&FacetConfig>,
        mode: Option<Mode>,
        opacity: Option<f64>,
        fill: Option<Fill>,
        size: Option<usize>,
        color: Option<Rgb>,
        colors: Option<Vec<Rgb>>,
        shape: Option<Shape>,
        shapes: Option<Vec<Shape>>,
        width: Option<f64>,
        line: Option<LineStyle>,
        lines: Option<Vec<LineStyle>>,
        plot_title: Option<Text>,
        legend_title: Option<Text>,
        legend: Option<&Legend>,
    ) -> Self {
        let traces = match facet {
            Some(facet_column) => {
                let config = facet_config.cloned().unwrap_or_default();
                Self::create_ir_traces_faceted(
                    data,
                    theta,
                    r,
                    group,
                    sort_groups_by,
                    facet_column,
                    &config,
                    mode,
                    opacity,
                    fill,
                    size,
                    color,
                    colors,
                    shape,
                    shapes,
                    width,
                    line,
                    lines,
                )
            }
            None => Self::create_ir_traces(
                data,
                theta,
                r,
                group,
                sort_groups_by,
                mode,
                opacity,
                fill,
                size,
                color,
                colors,
                shape,
                shapes,
                width,
                line,
                lines,
            ),
        };

        let grid = facet.map(|facet_column| {
            let config = facet_config.cloned().unwrap_or_default();
            let facet_categories =
                crate::data::get_unique_groups(data, facet_column, config.sorter);
            let n_facets = facet_categories.len();
            let (ncols, nrows) =
                crate::faceting::calculate_grid_dimensions(n_facets, config.cols, config.rows);
            crate::ir::facet::GridSpec {
                kind: crate::ir::facet::FacetKind::Polar,
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
                legend_title: legend_title.clone(),
                legend: legend.cloned(),
            }
        });

        let layout = LayoutIR {
            title: plot_title,
            x_title: None,
            y_title: None,
            y2_title: None,
            z_title: None,
            legend_title: if grid.is_some() { None } else { legend_title },
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
    fn get_polar_subplot_reference(index: usize) -> String {
        match index {
            0 => "polar".to_string(),
            1 => "polar2".to_string(),
            2 => "polar3".to_string(),
            3 => "polar4".to_string(),
            4 => "polar5".to_string(),
            5 => "polar6".to_string(),
            6 => "polar7".to_string(),
            7 => "polar8".to_string(),
            _ => "polar".to_string(),
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn create_ir_traces(
        data: &DataFrame,
        theta: &str,
        r: &str,
        group: Option<&str>,
        sort_groups_by: Option<fn(&str, &str) -> std::cmp::Ordering>,
        mode: Option<Mode>,
        opacity: Option<f64>,
        fill: Option<Fill>,
        size: Option<usize>,
        color: Option<Rgb>,
        colors: Option<Vec<Rgb>>,
        shape: Option<Shape>,
        shapes: Option<Vec<Shape>>,
        width: Option<f64>,
        line: Option<LineStyle>,
        lines: Option<Vec<LineStyle>>,
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

                    let line_ir = LineIR {
                        width,
                        color: Self::resolve_color(i, color, colors.clone()),
                        style: Self::resolve_line_style(i, line, lines.clone()),
                    };

                    traces.push(TraceIR::ScatterPolar(ScatterPolarIR {
                        theta: ColumnData::Numeric(crate::data::get_numeric_column(&subset, theta)),
                        r: ColumnData::Numeric(crate::data::get_numeric_column(&subset, r)),
                        name: Some(group_name.to_string()),
                        mode,
                        marker: Some(marker_ir),
                        line: Some(line_ir),
                        fill,
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

                let line_ir = LineIR {
                    width,
                    color,
                    style: Self::resolve_line_style(0, line, lines),
                };

                traces.push(TraceIR::ScatterPolar(ScatterPolarIR {
                    theta: ColumnData::Numeric(crate::data::get_numeric_column(data, theta)),
                    r: ColumnData::Numeric(crate::data::get_numeric_column(data, r)),
                    name: None,
                    mode,
                    marker: Some(marker_ir),
                    line: Some(line_ir),
                    fill,
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
        theta: &str,
        r: &str,
        group: Option<&str>,
        sort_groups_by: Option<fn(&str, &str) -> std::cmp::Ordering>,
        facet_column: &str,
        config: &FacetConfig,
        mode: Option<Mode>,
        opacity: Option<f64>,
        fill: Option<Fill>,
        size: Option<usize>,
        color: Option<Rgb>,
        colors: Option<Vec<Rgb>>,
        shape: Option<Shape>,
        shapes: Option<Vec<Shape>>,
        width: Option<f64>,
        line: Option<LineStyle>,
        lines: Option<Vec<LineStyle>>,
    ) -> Vec<TraceIR> {
        const MAX_FACETS: usize = 8;

        let facet_categories = crate::data::get_unique_groups(data, facet_column, config.sorter);

        if facet_categories.len() > MAX_FACETS {
            panic!(
                "Facet column '{}' has {} unique values, but plotly.rs supports maximum {} polar subplots",
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
                let subplot = Self::get_polar_subplot_reference(facet_idx);

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

                        let line_ir = LineIR {
                            width,
                            color: Some(grey_color),
                            style: Self::resolve_line_style(0, line, None),
                        };

                        traces.push(TraceIR::ScatterPolar(ScatterPolarIR {
                            theta: ColumnData::Numeric(crate::data::get_numeric_column(
                                &other_data,
                                theta,
                            )),
                            r: ColumnData::Numeric(crate::data::get_numeric_column(&other_data, r)),
                            name: None,
                            mode,
                            marker: Some(marker_ir),
                            line: Some(line_ir),
                            fill,
                            show_legend: Some(false),
                            legend_group: None,
                            subplot_ref: Some(subplot.clone()),
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

                            let line_ir = LineIR {
                                width,
                                color: Self::resolve_color(global_idx, color, colors.clone()),
                                style: Self::resolve_line_style(global_idx, line, lines.clone()),
                            };

                            traces.push(TraceIR::ScatterPolar(ScatterPolarIR {
                                theta: ColumnData::Numeric(crate::data::get_numeric_column(
                                    &group_data,
                                    theta,
                                )),
                                r: ColumnData::Numeric(crate::data::get_numeric_column(
                                    &group_data,
                                    r,
                                )),
                                name: Some(group_val.to_string()),
                                mode,
                                marker: Some(marker_ir),
                                line: Some(line_ir),
                                fill,
                                show_legend: Some(facet_idx == 0),
                                legend_group: Some(group_val.to_string()),
                                subplot_ref: Some(subplot.clone()),
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

                        let line_ir = LineIR {
                            width,
                            color: Self::resolve_color(facet_idx, color, colors.clone()),
                            style: Self::resolve_line_style(facet_idx, line, lines.clone()),
                        };

                        traces.push(TraceIR::ScatterPolar(ScatterPolarIR {
                            theta: ColumnData::Numeric(crate::data::get_numeric_column(
                                &facet_data,
                                theta,
                            )),
                            r: ColumnData::Numeric(crate::data::get_numeric_column(&facet_data, r)),
                            name: None,
                            mode,
                            marker: Some(marker_ir),
                            line: Some(line_ir),
                            fill,
                            show_legend: Some(false),
                            legend_group: None,
                            subplot_ref: Some(subplot.clone()),
                        }));
                    }
                }
            }
        } else {
            for (facet_idx, facet_value) in facet_categories.iter().enumerate() {
                let facet_data = crate::data::filter_data_by_group(data, facet_column, facet_value);
                let subplot = Self::get_polar_subplot_reference(facet_idx);

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

                            let line_ir = LineIR {
                                width,
                                color: Self::resolve_color(global_idx, color, colors.clone()),
                                style: Self::resolve_line_style(global_idx, line, lines.clone()),
                            };

                            traces.push(TraceIR::ScatterPolar(ScatterPolarIR {
                                theta: ColumnData::Numeric(crate::data::get_numeric_column(
                                    &group_data,
                                    theta,
                                )),
                                r: ColumnData::Numeric(crate::data::get_numeric_column(
                                    &group_data,
                                    r,
                                )),
                                name: Some(group_val.to_string()),
                                mode,
                                marker: Some(marker_ir),
                                line: Some(line_ir),
                                fill,
                                show_legend: Some(facet_idx == 0),
                                legend_group: Some(group_val.to_string()),
                                subplot_ref: Some(subplot.clone()),
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

                        let line_ir = LineIR {
                            width,
                            color: Self::resolve_color(facet_idx, color, colors.clone()),
                            style: Self::resolve_line_style(facet_idx, line, lines.clone()),
                        };

                        traces.push(TraceIR::ScatterPolar(ScatterPolarIR {
                            theta: ColumnData::Numeric(crate::data::get_numeric_column(
                                &facet_data,
                                theta,
                            )),
                            r: ColumnData::Numeric(crate::data::get_numeric_column(&facet_data, r)),
                            name: None,
                            mode,
                            marker: Some(marker_ir),
                            line: Some(line_ir),
                            fill,
                            show_legend: Some(false),
                            legend_group: None,
                            subplot_ref: Some(subplot.clone()),
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

    fn resolve_line_style(
        index: usize,
        style: Option<LineStyle>,
        styles: Option<Vec<LineStyle>>,
    ) -> Option<LineStyle> {
        if let Some(s) = style {
            return Some(s);
        }
        if let Some(ref ss) = styles {
            return ss.get(index).cloned();
        }
        None
    }
}

impl crate::Plot for ScatterPolar {
    fn ir_traces(&self) -> &[TraceIR] {
        &self.traces
    }

    fn ir_layout(&self) -> &LayoutIR {
        &self.layout
    }
}
