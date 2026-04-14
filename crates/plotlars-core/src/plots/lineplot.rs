use bon::bon;

use polars::{
    frame::DataFrame,
    prelude::{col, IntoLazy},
};

use crate::{
    components::{
        Axis, FacetConfig, Legend, Line as LineStyle, Mode, Rgb, Shape, Text, DEFAULT_PLOTLY_COLORS,
    },
    ir::data::ColumnData,
    ir::layout::LayoutIR,
    ir::line::LineIR,
    ir::marker::MarkerIR,
    ir::trace::{LinePlotIR, TraceIR},
};

/// A structure representing a line plot.
///
/// The `LinePlot` struct facilitates the creation and customization of line plots with various options
/// for data selection, layout configuration, and aesthetic adjustments. It supports the addition of multiple
/// lines, customization of marker shapes, line styles, colors, opacity settings, and comprehensive layout
/// customization including titles, axes, and legends.
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
/// * `additional_lines` - An optional vector of string slices specifying additional y-axis columns to be plotted as lines.
/// * `facet` - An optional string slice specifying the column name to be used for faceting (creating multiple subplots).
/// * `facet_config` - An optional reference to a `FacetConfig` struct for customizing facet behavior (grid dimensions, scales, gaps, etc.).
/// * `size` - An optional `usize` specifying the size of the markers or the thickness of the lines.
/// * `color` - An optional `Rgb` value specifying the color of the markers and lines. This is used when `additional_lines` is not specified.
/// * `colors` - An optional vector of `Rgb` values specifying the colors for the markers and lines. This is used when `additional_lines` is specified to differentiate between multiple lines.
/// * `shape` - An optional `Shape` specifying the shape of the markers. When set, markers are automatically displayed on the plot.
/// * `shapes` - An optional vector of `Shape` values specifying multiple shapes for the markers when plotting multiple lines. When set, markers are automatically displayed on the plot.
/// * `width` - An optional `f64` specifying the width of the plotted lines.
/// * `line` - An optional `Line` specifying the type of the line (e.g., solid, dashed). This is used when `additional_lines` is not specified.
/// * `lines` - An optional vector of `Line` enums specifying the types of lines (e.g., solid, dashed) for each plotted line. This is used when `additional_lines` is specified to differentiate between multiple lines.
/// * `plot_title` - An optional `Text` struct specifying the title of the plot.
/// * `x_title` - An optional `Text` struct specifying the title of the x-axis.
/// * `y_title` - An optional `Text` struct specifying the title of the y-axis.
/// * `y2_title` - An optional `Text` struct specifying the title of the secondary y-axis.
/// * `legend_title` - An optional `Text` struct specifying the title of the legend.
/// * `x_axis` - An optional reference to an `Axis` struct for customizing the x-axis.
/// * `y_axis` - An optional reference to an `Axis` struct for customizing the y-axis.
/// * `y2_axis` - An optional reference to an `Axis` struct for customizing the secondary y-axis.
/// * `legend` - An optional reference to a `Legend` struct for customizing the legend of the plot (e.g., positioning, font, etc.).
///
/// # Example
///
/// ```rust
/// use ndarray::Array;
/// use plotlars::{Axis, Line, LinePlot, Plot, Rgb, Text, TickDirection};
/// use polars::prelude::*;
///
///
/// let x_values = Array::linspace(0.0, 2.0 * std::f64::consts::PI, 1000).to_vec();
/// let sine_values = x_values.iter().map(|arg0: &f64| f64::sin(*arg0)).collect::<Vec<_>>();
/// let cosine_values = x_values.iter().map(|arg0: &f64| f64::cos(*arg0)).collect::<Vec<_>>();
///
/// let dataset = df![
///         "x" => &x_values,
///         "sine" => &sine_values,
///         "cosine" => &cosine_values,
///     ]
///     .unwrap();
///
/// LinePlot::builder()
///     .data(&dataset)
///     .x("x")
///     .y("sine")
///     .additional_lines(vec!["cosine"])
///     .colors(vec![
///         Rgb(255, 0, 0),
///         Rgb(0, 255, 0),
///     ])
///     .lines(vec![
///         Line::Solid,
///         Line::Dot,
///     ])
///     .width(3.0)
///     .plot_title(
///         Text::from("Line Plot")
///             .font("Arial")
///             .size(18)
///     )
///     .legend_title(
///         Text::from("series")
///             .font("Arial")
///             .size(15)
///     )
///     .x_axis(
///        &Axis::new()
///            .tick_direction(TickDirection::OutSide)
///            .axis_position(0.5)
///            .tick_values(vec![
///                0.5 * std::f64::consts::PI,
///                std::f64::consts::PI,
///                1.5 * std::f64::consts::PI,
///                2.0 * std::f64::consts::PI,
///            ])
///            .tick_labels(vec!["π/2", "π", "3π/2", "2π"])
///     )
///     .y_axis(
///        &Axis::new()
///            .tick_direction(TickDirection::OutSide)
///            .tick_values(vec![-1.0, 0.0, 1.0])
///            .tick_labels(vec!["-1", "0", "1"])
///     )
///     .build()
///     .plot();
/// ```
///
/// ![Example](https://imgur.com/PaXG300.png)
#[derive(Clone)]
#[allow(dead_code)]
pub struct LinePlot {
    traces: Vec<TraceIR>,
    layout: LayoutIR,
}

#[bon]
impl LinePlot {
    #[builder(on(String, into), on(Text, into))]
    pub fn new(
        data: &DataFrame,
        x: &str,
        y: &str,
        additional_lines: Option<Vec<&str>>,
        facet: Option<&str>,
        facet_config: Option<&FacetConfig>,
        size: Option<usize>,
        color: Option<Rgb>,
        colors: Option<Vec<Rgb>>,
        shape: Option<Shape>,
        shapes: Option<Vec<Shape>>,
        width: Option<f64>,
        line: Option<LineStyle>,
        lines: Option<Vec<LineStyle>>,
        plot_title: Option<Text>,
        x_title: Option<Text>,
        y_title: Option<Text>,
        y2_title: Option<Text>,
        legend_title: Option<Text>,
        x_axis: Option<&Axis>,
        y_axis: Option<&Axis>,
        y2_axis: Option<&Axis>,
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
            y2_title: if grid.is_some() {
                None
            } else {
                y2_title.clone()
            },
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
                    y2_axis: y2_axis.cloned(),
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
                    additional_lines,
                    facet_column,
                    &config,
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
                x,
                y,
                additional_lines,
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

        Self { traces, layout }
    }
}

#[bon]
impl LinePlot {
    #[builder(
        start_fn = try_builder,
        finish_fn = try_build,
        builder_type = LinePlotTryBuilder,
        on(String, into),
        on(Text, into),
    )]
    pub fn try_new(
        data: &DataFrame,
        x: &str,
        y: &str,
        additional_lines: Option<Vec<&str>>,
        facet: Option<&str>,
        facet_config: Option<&FacetConfig>,
        size: Option<usize>,
        color: Option<Rgb>,
        colors: Option<Vec<Rgb>>,
        shape: Option<Shape>,
        shapes: Option<Vec<Shape>>,
        width: Option<f64>,
        line: Option<LineStyle>,
        lines: Option<Vec<LineStyle>>,
        plot_title: Option<Text>,
        x_title: Option<Text>,
        y_title: Option<Text>,
        y2_title: Option<Text>,
        legend_title: Option<Text>,
        x_axis: Option<&Axis>,
        y_axis: Option<&Axis>,
        y2_axis: Option<&Axis>,
        legend: Option<&Legend>,
    ) -> Result<Self, crate::io::PlotlarsError> {
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            Self::__orig_new(
                data,
                x,
                y,
                additional_lines,
                facet,
                facet_config,
                size,
                color,
                colors,
                shape,
                shapes,
                width,
                line,
                lines,
                plot_title,
                x_title,
                y_title,
                y2_title,
                legend_title,
                x_axis,
                y_axis,
                y2_axis,
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

impl LinePlot {
    #[allow(clippy::too_many_arguments)]
    fn create_ir_traces(
        data: &DataFrame,
        x_col: &str,
        y_col: &str,
        additional_lines: Option<Vec<&str>>,
        size: Option<usize>,
        color: Option<Rgb>,
        colors: Option<Vec<Rgb>>,
        shape: Option<Shape>,
        shapes: Option<Vec<Shape>>,
        width: Option<f64>,
        style: Option<LineStyle>,
        styles: Option<Vec<LineStyle>>,
    ) -> Vec<TraceIR> {
        let mut traces = Vec::new();

        let mode = Self::resolve_mode(shape, shapes.as_ref());

        let marker_ir = MarkerIR {
            opacity: None,
            size,
            color: Self::resolve_color(0, color, colors.clone()),
            shape: Self::resolve_shape(0, shape, shapes.clone()),
        };

        let line_ir = Self::resolve_line_ir(0, width, style, styles.clone());

        traces.push(TraceIR::LinePlot(LinePlotIR {
            x: ColumnData::Numeric(crate::data::get_numeric_column(data, x_col)),
            y: ColumnData::Numeric(crate::data::get_numeric_column(data, y_col)),
            name: Some(y_col.to_string()),
            marker: Some(marker_ir),
            line: Some(line_ir),
            mode,
            show_legend: None,
            legend_group: None,
            subplot_ref: None,
        }));

        if let Some(additional_lines) = additional_lines {
            for (i, series) in additional_lines.into_iter().enumerate() {
                let subset = data
                    .clone()
                    .lazy()
                    .select([col(x_col), col(series)])
                    .collect()
                    .unwrap();

                let marker_ir = MarkerIR {
                    opacity: None,
                    size,
                    color: Self::resolve_color(i + 1, color, colors.clone()),
                    shape: Self::resolve_shape(i + 1, shape, shapes.clone()),
                };

                let line_ir = Self::resolve_line_ir(i + 1, width, style, styles.clone());

                traces.push(TraceIR::LinePlot(LinePlotIR {
                    x: ColumnData::Numeric(crate::data::get_numeric_column(&subset, x_col)),
                    y: ColumnData::Numeric(crate::data::get_numeric_column(&subset, series)),
                    name: Some(series.to_string()),
                    marker: Some(marker_ir),
                    line: Some(line_ir),
                    mode,
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
        additional_lines: Option<Vec<&str>>,
        facet_column: &str,
        config: &FacetConfig,
        size: Option<usize>,
        color: Option<Rgb>,
        colors: Option<Vec<Rgb>>,
        shape: Option<Shape>,
        shapes: Option<Vec<Shape>>,
        width: Option<f64>,
        style: Option<LineStyle>,
        styles: Option<Vec<LineStyle>>,
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

        let all_y_cols = if let Some(ref add_lines) = additional_lines {
            let mut cols = vec![y];
            cols.extend(add_lines.iter().copied());
            cols
        } else {
            vec![y]
        };

        if let Some(ref color_vec) = colors {
            if additional_lines.is_none() {
                let color_count = color_vec.len();
                let facet_count = facet_categories.len();
                if color_count != facet_count {
                    panic!(
                        "When using colors with facet (without additional_lines), colors.len() must equal number of facets. \
                         Expected {} colors for {} facets, but got {} colors. \
                         Each facet must be assigned exactly one color.",
                        facet_count, facet_count, color_count
                    );
                }
            } else {
                let color_count = color_vec.len();
                let line_count = all_y_cols.len();
                if color_count < line_count {
                    panic!(
                        "When using colors with additional_lines, colors.len() must be >= number of lines. \
                         Need at least {} colors for {} lines, but got {} colors",
                        line_count, line_count, color_count
                    );
                }
            }
        }

        let colors = if additional_lines.is_some() && colors.is_none() {
            Some(DEFAULT_PLOTLY_COLORS.to_vec())
        } else {
            colors
        };

        let mut traces = Vec::new();

        let mode_for_facet = Self::resolve_mode(shape, shapes.as_ref());

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

                        for (line_idx, y_col) in all_y_cols.iter().enumerate() {
                            let marker_ir = MarkerIR {
                                opacity: None,
                                size,
                                color: Some(grey_color),
                                shape: Self::resolve_shape(line_idx, shape, None),
                            };

                            let line_ir =
                                Self::resolve_line_ir(line_idx, width, style, styles.clone());

                            traces.push(TraceIR::LinePlot(LinePlotIR {
                                x: ColumnData::Numeric(crate::data::get_numeric_column(
                                    &other_data,
                                    x,
                                )),
                                y: ColumnData::Numeric(crate::data::get_numeric_column(
                                    &other_data,
                                    y_col,
                                )),
                                name: None,
                                marker: Some(marker_ir),
                                line: Some(line_ir),
                                mode: mode_for_facet,
                                show_legend: Some(false),
                                legend_group: None,
                                subplot_ref: Some(subplot_ref.clone()),
                            }));
                        }
                    }
                }

                let facet_data = crate::data::filter_data_by_group(data, facet_column, facet_value);

                for (line_idx, y_col) in all_y_cols.iter().enumerate() {
                    let color_index = if additional_lines.is_none() {
                        facet_idx
                    } else {
                        line_idx
                    };

                    let marker_ir = MarkerIR {
                        opacity: None,
                        size,
                        color: Self::resolve_color(color_index, color, colors.clone()),
                        shape: Self::resolve_shape(color_index, shape, shapes.clone()),
                    };

                    let line_ir = Self::resolve_line_ir(line_idx, width, style, styles.clone());

                    let show_legend = facet_idx == 0;
                    let name = if show_legend {
                        Some(y_col.to_string())
                    } else {
                        None
                    };

                    traces.push(TraceIR::LinePlot(LinePlotIR {
                        x: ColumnData::Numeric(crate::data::get_numeric_column(&facet_data, x)),
                        y: ColumnData::Numeric(crate::data::get_numeric_column(&facet_data, y_col)),
                        name,
                        marker: Some(marker_ir),
                        line: Some(line_ir),
                        mode: mode_for_facet,
                        show_legend: Some(show_legend),
                        legend_group: Some(y_col.to_string()),
                        subplot_ref: Some(subplot_ref.clone()),
                    }));
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

                for (line_idx, y_col) in all_y_cols.iter().enumerate() {
                    let color_index = if additional_lines.is_none() {
                        facet_idx
                    } else {
                        line_idx
                    };

                    let marker_ir = MarkerIR {
                        opacity: None,
                        size,
                        color: Self::resolve_color(color_index, color, colors.clone()),
                        shape: Self::resolve_shape(color_index, shape, shapes.clone()),
                    };

                    let line_ir = Self::resolve_line_ir(line_idx, width, style, styles.clone());

                    let show_legend = facet_idx == 0;
                    let name = if show_legend {
                        Some(y_col.to_string())
                    } else {
                        None
                    };

                    traces.push(TraceIR::LinePlot(LinePlotIR {
                        x: ColumnData::Numeric(crate::data::get_numeric_column(&facet_data, x)),
                        y: ColumnData::Numeric(crate::data::get_numeric_column(&facet_data, y_col)),
                        name,
                        marker: Some(marker_ir),
                        line: Some(line_ir),
                        mode: mode_for_facet,
                        show_legend: Some(show_legend),
                        legend_group: Some(y_col.to_string()),
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

    fn resolve_line_ir(
        index: usize,
        width: Option<f64>,
        style: Option<LineStyle>,
        styles: Option<Vec<LineStyle>>,
    ) -> LineIR {
        let resolved_style = if style.is_some() {
            style
        } else {
            styles.and_then(|ss| ss.get(index).cloned())
        };

        LineIR {
            width,
            style: resolved_style,
            color: None,
        }
    }

    fn resolve_mode(shape: Option<Shape>, shapes: Option<&Vec<Shape>>) -> Option<Mode> {
        if shape.is_some() || shapes.is_some() {
            Some(Mode::LinesMarkers)
        } else {
            Some(Mode::Lines)
        }
    }
}

impl crate::Plot for LinePlot {
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
        let df = df!["x" => [1.0, 2.0, 3.0], "y" => [4.0, 5.0, 6.0]].unwrap();
        let plot = LinePlot::builder().data(&df).x("x").y("y").build();
        assert_eq!(plot.ir_traces().len(), 1);
        assert!(matches!(plot.ir_traces()[0], TraceIR::LinePlot(_)));
    }

    #[test]
    fn test_with_additional_lines() {
        let df = df![
            "x" => [1.0, 2.0, 3.0],
            "y" => [4.0, 5.0, 6.0],
            "y2" => [7.0, 8.0, 9.0]
        ]
        .unwrap();
        let plot = LinePlot::builder()
            .data(&df)
            .x("x")
            .y("y")
            .additional_lines(vec!["y2"])
            .build();
        assert_eq!(plot.ir_traces().len(), 2);
        assert!(matches!(plot.ir_traces()[1], TraceIR::LinePlot(_)));
    }

    #[test]
    fn test_layout_titles() {
        let df = df!["x" => [1.0, 2.0], "y" => [3.0, 4.0]].unwrap();
        let plot = LinePlot::builder()
            .data(&df)
            .x("x")
            .y("y")
            .plot_title("My Title")
            .x_title("X Axis")
            .y_title("Y Axis")
            .build();
        let layout = plot.ir_layout();
        assert!(layout.title.is_some());
        assert!(layout.x_title.is_some());
        assert!(layout.y_title.is_some());
        assert!(layout.axes_2d.is_some());
    }

    #[test]
    fn test_resolve_color_singular_priority() {
        let result = LinePlot::resolve_color(0, Some(Rgb(255, 0, 0)), Some(vec![Rgb(0, 0, 255)]));
        assert_rgb(result, 255, 0, 0);
    }

    #[test]
    fn test_resolve_color_both_none() {
        let result = LinePlot::resolve_color(0, None, None);
        assert!(result.is_none());
    }

    #[test]
    fn test_faceted_trace_count() {
        let df = df![
            "x" => [1.0, 2.0, 3.0, 4.0],
            "y" => [4.0, 5.0, 6.0, 7.0],
            "facet_col" => ["a", "a", "b", "b"]
        ]
        .unwrap();
        let plot = LinePlot::builder()
            .data(&df)
            .x("x")
            .y("y")
            .facet("facet_col")
            .build();
        // 2 facets, 1 line each = 2 traces
        assert_eq!(plot.ir_traces().len(), 2);
    }
}
