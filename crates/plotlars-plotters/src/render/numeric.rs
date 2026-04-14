use plotlars_core::components::Line as LineStyle;
use plotlars_core::ir::layout::LayoutIR;
use plotlars_core::ir::line::LineIR;
use plotlars_core::ir::trace::TraceIR;
use plotters::coord::cartesian::Cartesian2d;
use plotters::prelude::*;
use plotters::style::text_anchor::{HPos, Pos, VPos};

use crate::converters::components::{
    convert_rgb, default_color, resolve_trace_color, resolve_trace_shape, BaseShape, FillMode,
};
use crate::converters::layout::{extract_layout_config, format_thousands};
use crate::converters::trace::{
    auto_compute_bins, collect_candlestick_labels, collect_timeseries_labels, compute_bins_from_ir,
    compute_numeric_ranges, extract_f64, extract_strings, extract_timeseries_points,
    extract_xy_pairs, histogram_max_count,
};

use super::axis::{
    apply_mesh_axis_config, axis_value_color, category_xy_pairs, collect_string_x_labels,
    configure_label_areas, format_axis_value, format_exponent, format_log_label, is_category_axis,
    is_date_axis, is_log_axis, log_range, log_transform_points,
};
use super::legend::apply_legend_config;
use super::title::{draw_axis_titles, draw_plot_title, title_top_margin};
use super::{polygon_vertices_at_origin, resolve_dimensions, LegendEntry, SwatchKind};

/// Pairs a stroke color with its SVG `stroke-dasharray` value, captured during
/// rendering and applied to matching `<polyline>` elements in the SVG post-pass.
pub(super) type DashEntry = (RGBColor, &'static str);

/// Returns the SVG `stroke-dasharray` value for a given line style, or `None` for Solid.
fn dash_pattern(line_ir: Option<&LineIR>) -> Option<&'static str> {
    let style = line_ir?.style.as_ref()?;
    match style {
        LineStyle::Solid => None,
        LineStyle::Dot => Some("2,4"),
        LineStyle::Dash => Some("8,6"),
        LineStyle::LongDash => Some("14,6"),
        LineStyle::DashDot => Some("8,4,2,4"),
        LineStyle::LongDashDot => Some("14,4,2,4"),
    }
}

/// Draw a line series on a cartesian chart.
///
/// Always emits a single solid `LineSeries`; dashed styles are applied later by
/// `apply_svg_dash_patterns` so dense series stay as one polyline rather than
/// exploding into thousands of dash segments.
#[allow(clippy::too_many_arguments)]
fn draw_line_on_chart<DB: DrawingBackend>(
    chart: &mut ChartContext<
        DB,
        Cartesian2d<plotters::coord::types::RangedCoordf64, plotters::coord::types::RangedCoordf64>,
    >,
    points: &[(f64, f64)],
    line_style: ShapeStyle,
    dash: Option<&'static str>,
    name: Option<&str>,
    has_legend: &mut bool,
    legend_entries: &mut Vec<LegendEntry>,
    color: RGBColor,
    width: u32,
    dash_entries: &mut Vec<DashEntry>,
    marker_shape: Option<(BaseShape, FillMode)>,
) {
    let series = chart
        .draw_series(LineSeries::new(
            points.iter().map(|&(x, y)| (x, y)),
            line_style,
        ))
        .unwrap();

    if let Some(name) = name {
        *has_legend = true;
        series
            .label(name)
            .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], line_style));
        let kind = match marker_shape {
            Some((base, fill)) => SwatchKind::LineShape(width, base, fill),
            None => SwatchKind::Line(width),
        };
        legend_entries.push(LegendEntry {
            name: name.to_string(),
            color,
            opacity: 1.0,
            kind,
        });
    }

    if let Some(pattern) = dash {
        dash_entries.push((color, pattern));
    }
}

/// Transform a y-value from y2-space to primary y-space.
fn y2_to_primary(val: f64, y2_min: f64, y2_max: f64, y_min: f64, y_max: f64) -> f64 {
    let y2_range = y2_max - y2_min;
    if y2_range == 0.0 {
        return (y_min + y_max) / 2.0;
    }
    let t = (val - y2_min) / y2_range;
    y_min + t * (y_max - y_min)
}

pub(super) fn render_numeric<DB: DrawingBackend>(
    root: &DrawingArea<DB, plotters::coord::Shift>,
    layout: &LayoutIR,
    traces: &[TraceIR],
    unsupported: &mut Vec<String>,
    dash_entries: &mut Vec<DashEntry>,
) {
    let config = extract_layout_config(layout, unsupported);

    let (mut x_min, mut x_max, mut y_min, mut y_max) = compute_numeric_ranges(traces);

    // For histograms, y axis is bin counts
    let hist_max = histogram_max_count(traces);
    if hist_max > 0.0 {
        y_min = 0.0;
        y_max = y_max.max(hist_max * 1.1);
    }
    // Ensure valid ranges
    if !x_min.is_finite() || !x_max.is_finite() {
        x_min = 0.0;
        x_max = 1.0;
    }
    if !y_min.is_finite() || !y_max.is_finite() {
        y_min = 0.0;
        y_max = 1.0;
    }

    // Apply user-specified axis ranges
    if let Some((lo, hi)) = config.x_range {
        x_min = lo;
        x_max = hi;
    }
    if let Some((lo, hi)) = config.y_range {
        y_min = lo;
        y_max = hi;
    }

    // Detect logarithmic axes and transform ranges to log10 space
    let x_log = is_log_axis(&config.x_axis);
    let y_log = is_log_axis(&config.y_axis);

    if x_log {
        let (lo, hi) = log_range(x_min, x_max);
        x_min = lo;
        x_max = hi;
    }
    if y_log {
        let (lo, hi) = log_range(y_min, y_max);
        y_min = lo;
        y_max = hi;
    }

    // Detect category/date x-axis and collect string labels
    let x_cat = is_category_axis(&config.x_axis);
    let mut cat_labels = if x_cat {
        collect_string_x_labels(traces)
    } else {
        Vec::new()
    };
    if is_date_axis(&config.x_axis) {
        cat_labels.sort();
    }
    if x_cat && !cat_labels.is_empty() {
        x_min = -0.5;
        x_max = cat_labels.len() as f64 - 0.5;
    }

    // Clamp range to tick_values extent when specified
    if let Some(tvs) = config.x_axis.as_ref().and_then(|a| a.tick_values.as_ref()) {
        if let (Some(&lo), Some(&hi)) = (tvs.first(), tvs.last()) {
            x_min = lo;
            x_max = hi;
        }
    }
    if let Some(tvs) = config.y_axis.as_ref().and_then(|a| a.tick_values.as_ref()) {
        if let (Some(&lo), Some(&hi)) = (tvs.first(), tvs.last()) {
            y_min = lo;
            y_max = hi;
        }
    }

    // Collect timeseries/candlestick labels for custom x-axis formatting
    let mut ts_labels = collect_timeseries_labels(traces);
    if ts_labels.is_empty() {
        ts_labels = collect_candlestick_labels(traces);
    }

    // Single-pass dual y-axis detection: walk TimeSeriesPlot traces once,
    // accumulating min/max for y2 traces (`y_axis_ref == "y2"`) and primary
    // traces separately. `has_y2` is implied by whether any y2 trace was seen.
    let mut y2_lo = f64::INFINITY;
    let mut y2_hi = f64::NEG_INFINITY;
    let mut primary_lo = f64::INFINITY;
    let mut primary_hi = f64::NEG_INFINITY;
    for trace in traces {
        if let TraceIR::TimeSeriesPlot(ir) = trace {
            let is_y2 = ir.y_axis_ref.as_deref() == Some("y2");
            if is_y2 {
                for v in extract_f64(&ir.y) {
                    y2_lo = y2_lo.min(v);
                    y2_hi = y2_hi.max(v);
                }
            } else {
                let (pts, _) = extract_timeseries_points(&ir.x, &ir.y);
                for (_, y) in &pts {
                    primary_lo = primary_lo.min(*y);
                    primary_hi = primary_hi.max(*y);
                }
            }
        }
    }
    let has_y2 = y2_lo.is_finite();

    let (mut y2_min, mut y2_max) = if has_y2 {
        let margin = (y2_hi - y2_lo).abs() * 0.05;
        (y2_lo - margin.max(0.01), y2_hi + margin.max(0.01))
    } else {
        (0.0, 1.0)
    };

    // Re-bound primary y-range from non-y2 traces only when y2 is in play
    // (otherwise the earlier `compute_numeric_ranges` result is correct).
    if has_y2 && primary_lo.is_finite() && primary_hi.is_finite() {
        let margin = (primary_hi - primary_lo).abs() * 0.05;
        y_min = primary_lo - margin.max(0.01);
        y_max = primary_hi + margin.max(0.01);
    }

    // Apply user-specified y2 axis range
    if has_y2 {
        if let Some(y2_range) = config
            .y2_axis
            .as_ref()
            .and_then(|a| a.value_range.as_ref())
            .map(|r| (r[0], r[1]))
        {
            y2_min = y2_range.0;
            y2_max = y2_range.1;
        }
    }

    let (w, h) = resolve_dimensions(layout);
    draw_plot_title(root, &config, w, h);

    let right_margin = if has_y2 { 60 } else { 15 };
    let mut builder = ChartBuilder::on(root);
    let x_label_area = if ts_labels.is_empty() { 40 } else { 60 };
    builder
        .margin_top(15 + title_top_margin(&config))
        .margin_bottom(15)
        .margin_left(15)
        .margin_right(right_margin);
    configure_label_areas(&mut builder, &config, x_label_area as u32, 50);

    let mut chart = builder
        .build_cartesian_2d(x_min..x_max, y_min..y_max)
        .unwrap();

    let is_line_plot = traces.iter().any(|t| matches!(t, TraceIR::LinePlot(_)));

    {
        let mut mesh = chart.configure_mesh();

        // Hide edge axis lines and minor grid for line plots (replaced by zero lines + custom ticks)
        if is_line_plot {
            mesh.axis_style(TRANSPARENT);
            mesh.light_line_style(TRANSPARENT);
        }

        let xvc = axis_value_color(config.x_axis.as_ref());
        let yvc = axis_value_color(config.y_axis.as_ref());
        apply_mesh_axis_config(&mut mesh, &config, &xvc, &yvc);

        // Thousands formatter (applied before timeseries override)
        let x_thousands = config
            .x_axis
            .as_ref()
            .and_then(|a| a.value_thousands)
            .unwrap_or(false);
        let y_thousands = config
            .y_axis
            .as_ref()
            .and_then(|a| a.value_thousands)
            .unwrap_or(false);

        let x_fmt;
        let y_fmt;
        let x_log_fmt;
        let y_log_fmt;
        let x_exp_fmt;
        let y_exp_fmt;
        let x_tick_fmt;
        let y_tick_fmt;

        let x_exponent = config
            .x_axis
            .as_ref()
            .and_then(|a| a.value_exponent.as_ref());
        let y_exponent = config
            .y_axis
            .as_ref()
            .and_then(|a| a.value_exponent.as_ref());

        let x_tick_values = config.x_axis.as_ref().and_then(|a| a.tick_values.clone());
        let y_tick_values = config.y_axis.as_ref().and_then(|a| a.tick_values.clone());

        if x_tick_values.is_some() {
            mesh.disable_x_mesh();
            mesh.set_tick_mark_size(LabelAreaPosition::Bottom, 0);
            mesh.set_tick_mark_size(LabelAreaPosition::Top, 0);
        }
        if y_tick_values.is_some() {
            mesh.disable_y_mesh();
            mesh.set_tick_mark_size(LabelAreaPosition::Left, 0);
            mesh.set_tick_mark_size(LabelAreaPosition::Right, 0);
        }

        let cat_formatter;
        let ts_formatter;
        if x_tick_values.is_some() {
            x_tick_fmt = |_v: &f64| String::new();
            mesh.x_label_formatter(&x_tick_fmt);
        } else if x_cat && !cat_labels.is_empty() {
            let labels = cat_labels.clone();
            let n = labels.len();
            let step = (n / 10).max(1);
            cat_formatter = move |v: &f64| -> String {
                let idx = v.round() as usize;
                if idx < labels.len() && idx % step == 0 {
                    labels[idx].clone()
                } else {
                    String::new()
                }
            };
            mesh.x_labels(cat_labels.len().min(10));
            mesh.x_label_formatter(&cat_formatter);
        } else if !ts_labels.is_empty() {
            let labels = ts_labels.clone();
            let n = labels.len();
            let step = (n / 10).max(1);
            ts_formatter = move |v: &f64| -> String {
                let idx = v.round() as usize;
                if idx < labels.len() && idx % step == 0 {
                    labels[idx].clone()
                } else {
                    String::new()
                }
            };
            mesh.x_label_formatter(&ts_formatter);
        } else if x_log {
            x_log_fmt = |v: &f64| format_log_label(v);
            mesh.x_label_formatter(&x_log_fmt);
        } else if let Some(exp) = x_exponent {
            let exp = exp.clone();
            x_exp_fmt = move |v: &f64| format_exponent(*v, &exp);
            mesh.x_label_formatter(&x_exp_fmt);
        } else if x_thousands {
            x_fmt = |v: &f64| format_thousands(*v);
            mesh.x_label_formatter(&x_fmt);
        }

        if y_tick_values.is_some() {
            y_tick_fmt = |_v: &f64| String::new();
            mesh.y_label_formatter(&y_tick_fmt);
        } else if y_log {
            y_log_fmt = |v: &f64| format_log_label(v);
            mesh.y_label_formatter(&y_log_fmt);
        } else if let Some(exp) = y_exponent {
            let exp = exp.clone();
            y_exp_fmt = move |v: &f64| format_exponent(*v, &exp);
            mesh.y_label_formatter(&y_exp_fmt);
        } else if y_thousands {
            y_fmt = |v: &f64| format_thousands(*v);
            mesh.y_label_formatter(&y_fmt);
        }

        mesh.draw().unwrap();
    }

    // Draw per-axis grid lines when grid_color differs between axes
    {
        let x_grid_color = config.x_axis.as_ref().and_then(|a| a.grid_color.as_ref());
        let y_grid_color = config.y_axis.as_ref().and_then(|a| a.grid_color.as_ref());
        let x_show_grid = config.x_axis.as_ref().and_then(|a| a.show_grid);
        let y_show_grid = config.y_axis.as_ref().and_then(|a| a.show_grid);
        let x_grid_width = config
            .x_axis
            .as_ref()
            .and_then(|a| a.grid_width)
            .unwrap_or(1) as u32;
        let y_grid_width = config
            .y_axis
            .as_ref()
            .and_then(|a| a.grid_width)
            .unwrap_or(1) as u32;

        if x_grid_color.is_some() || y_grid_color.is_some() {
            let default_color = RGBColor(200, 200, 200);

            // X-axis grid: vertical lines at x tick positions
            if x_show_grid != Some(false) {
                let color = x_grid_color.map(convert_rgb).unwrap_or(default_color);
                let style = ShapeStyle {
                    color: color.to_rgba(),
                    filled: false,
                    stroke_width: x_grid_width,
                };
                // Use plotters' generated label positions (approximately 11 ticks)
                let range = x_max - x_min;
                let step = range / 10.0;
                if step > 0.0 {
                    let mut v = x_min;
                    while v <= x_max + step * 0.01 {
                        let (px, py_lo) = chart.backend_coord(&(v, y_min));
                        let (_, py_hi) = chart.backend_coord(&(v, y_max));
                        root.draw(&PathElement::new(vec![(px, py_hi), (px, py_lo)], style))
                            .unwrap();
                        v += step;
                    }
                }
            }

            // Y-axis grid: horizontal lines at y tick positions
            if y_show_grid != Some(false) {
                let color = y_grid_color.map(convert_rgb).unwrap_or(default_color);
                let style = ShapeStyle {
                    color: color.to_rgba(),
                    filled: false,
                    stroke_width: y_grid_width,
                };
                let range = y_max - y_min;
                let step = range / 10.0;
                if step > 0.0 {
                    let mut v = y_min;
                    while v <= y_max + step * 0.01 {
                        let (px_lo, py) = chart.backend_coord(&(x_min, v));
                        let (px_hi, _) = chart.backend_coord(&(x_max, v));
                        root.draw(&PathElement::new(vec![(px_lo, py), (px_hi, py)], style))
                            .unwrap();
                        v += step;
                    }
                }
            }
        }
    }

    // Redraw individual axis lines when only one is hidden
    let x_show_line = config.x_axis.as_ref().and_then(|a| a.show_line);
    let y_show_line = config.y_axis.as_ref().and_then(|a| a.show_line);
    let axis_line_style = ShapeStyle {
        color: BLACK.to_rgba(),
        filled: false,
        stroke_width: 1,
    };
    if x_show_line != Some(false) && y_show_line == Some(false) {
        // x-axis visible, y-axis hidden: draw bottom line
        let (px_lo, py) = chart.backend_coord(&(x_min, y_min));
        let (px_hi, _) = chart.backend_coord(&(x_max, y_min));
        root.draw(&PathElement::new(
            vec![(px_lo, py), (px_hi, py)],
            axis_line_style,
        ))
        .unwrap();
    }
    if y_show_line != Some(false) && x_show_line == Some(false) {
        // y-axis visible, x-axis hidden: draw left line
        let (px, py_lo) = chart.backend_coord(&(x_min, y_min));
        let (_, py_hi) = chart.backend_coord(&(x_min, y_max));
        root.draw(&PathElement::new(
            vec![(px, py_hi), (px, py_lo)],
            axis_line_style,
        ))
        .unwrap();
    }

    let mut has_legend = false;
    let mut legend_entries: Vec<LegendEntry> = Vec::new();

    for (idx, trace) in traces.iter().enumerate() {
        match trace {
            TraceIR::ScatterPlot(ir) => {
                let points = if x_cat {
                    category_xy_pairs(&ir.x, &ir.y, &cat_labels, y_log)
                } else {
                    let raw = extract_xy_pairs(&ir.x, &ir.y);
                    if x_log || y_log {
                        log_transform_points(&raw, x_log, y_log)
                    } else {
                        raw
                    }
                };
                if points.is_empty() {
                    continue;
                }
                let color = resolve_trace_color(&ir.marker, idx);
                let size = ir.marker.as_ref().and_then(|m| m.size).unwrap_or(5) as i32;
                let opacity = ir.marker.as_ref().and_then(|m| m.opacity).unwrap_or(1.0);
                let (base_shape, fill_mode) = resolve_trace_shape(&ir.marker);

                draw_scatter_series(
                    &mut chart,
                    &points,
                    size,
                    color,
                    opacity,
                    base_shape,
                    fill_mode,
                    ir.name.as_deref(),
                    &mut has_legend,
                );

                if let Some(ref name) = ir.name {
                    legend_entries.push(LegendEntry {
                        name: name.clone(),
                        color,
                        opacity,
                        kind: SwatchKind::Shape(base_shape, fill_mode),
                    });
                }

                if ir.fill.is_some() {
                    plotlars_core::policy::report_unsupported(
                        "plotters",
                        "ScatterPlot",
                        "fill",
                        unsupported,
                    );
                }
            }
            TraceIR::LinePlot(ir) => {
                let points = if x_cat {
                    category_xy_pairs(&ir.x, &ir.y, &cat_labels, y_log)
                } else {
                    let raw = extract_xy_pairs(&ir.x, &ir.y);
                    if x_log || y_log {
                        log_transform_points(&raw, x_log, y_log)
                    } else {
                        raw
                    }
                };
                if points.is_empty() {
                    continue;
                }
                let color = ir
                    .line
                    .as_ref()
                    .and_then(|l| l.color.as_ref())
                    .map(crate::converters::components::convert_rgb)
                    .or_else(|| {
                        ir.marker
                            .as_ref()
                            .and_then(|m| m.color.as_ref())
                            .map(crate::converters::components::convert_rgb)
                    })
                    .unwrap_or_else(|| default_color(idx));
                let width = ir.line.as_ref().and_then(|l| l.width).unwrap_or(2.0) as u32;
                let line_style = ShapeStyle {
                    color: color.to_rgba(),
                    filled: false,
                    stroke_width: width,
                };

                // Determine mode
                let draw_lines = !matches!(ir.mode, Some(plotlars_core::components::Mode::Markers));
                let draw_markers = matches!(
                    ir.mode,
                    Some(plotlars_core::components::Mode::Markers)
                        | Some(plotlars_core::components::Mode::LinesMarkers)
                );

                let shape_for_legend = if draw_markers {
                    Some(resolve_trace_shape(&ir.marker))
                } else {
                    None
                };

                if draw_lines {
                    let dash = dash_pattern(ir.line.as_ref());
                    draw_line_on_chart(
                        &mut chart,
                        &points,
                        line_style,
                        dash,
                        ir.name.as_deref(),
                        &mut has_legend,
                        &mut legend_entries,
                        color,
                        width,
                        dash_entries,
                        shape_for_legend,
                    );
                }

                if draw_markers {
                    let marker_size = ir.marker.as_ref().and_then(|m| m.size).unwrap_or(4) as i32;
                    let (base_shape, fill_mode) = resolve_trace_shape(&ir.marker);
                    draw_scatter_series(
                        &mut chart,
                        &points,
                        marker_size,
                        color,
                        1.0,
                        base_shape,
                        fill_mode,
                        None,
                        &mut has_legend,
                    );
                }
            }
            TraceIR::TimeSeriesPlot(ir) => {
                let is_y2_trace = has_y2 && ir.y_axis_ref.as_deref() == Some("y2");
                let (raw_points, _) = extract_timeseries_points(&ir.x, &ir.y);
                let points = if is_y2_trace {
                    raw_points
                        .into_iter()
                        .map(|(x, y)| (x, y2_to_primary(y, y2_min, y2_max, y_min, y_max)))
                        .collect()
                } else if x_log || y_log {
                    log_transform_points(&raw_points, x_log, y_log)
                } else {
                    raw_points
                };
                if points.is_empty() {
                    continue;
                }
                let color = ir
                    .line
                    .as_ref()
                    .and_then(|l| l.color.as_ref())
                    .map(crate::converters::components::convert_rgb)
                    .or_else(|| {
                        ir.marker
                            .as_ref()
                            .and_then(|m| m.color.as_ref())
                            .map(crate::converters::components::convert_rgb)
                    })
                    .unwrap_or_else(|| default_color(idx));
                let width = ir.line.as_ref().and_then(|l| l.width).unwrap_or(2.0) as u32;
                let line_style = ShapeStyle {
                    color: color.to_rgba(),
                    filled: false,
                    stroke_width: width,
                };

                // Determine mode
                let draw_lines = !matches!(ir.mode, Some(plotlars_core::components::Mode::Markers));
                let draw_markers = matches!(
                    ir.mode,
                    Some(plotlars_core::components::Mode::Markers)
                        | Some(plotlars_core::components::Mode::LinesMarkers)
                );

                let shape_for_legend = if draw_markers {
                    Some(resolve_trace_shape(&ir.marker))
                } else {
                    None
                };

                if draw_lines {
                    let dash = dash_pattern(ir.line.as_ref());
                    draw_line_on_chart(
                        &mut chart,
                        &points,
                        line_style,
                        dash,
                        ir.name.as_deref(),
                        &mut has_legend,
                        &mut legend_entries,
                        color,
                        width,
                        dash_entries,
                        shape_for_legend,
                    );
                }

                if draw_markers {
                    let marker_size = ir.marker.as_ref().and_then(|m| m.size).unwrap_or(4) as i32;
                    let (base_shape, fill_mode) = resolve_trace_shape(&ir.marker);
                    draw_scatter_series(
                        &mut chart,
                        &points,
                        marker_size,
                        color,
                        1.0,
                        base_shape,
                        fill_mode,
                        None,
                        &mut has_legend,
                    );
                }
            }
            TraceIR::Histogram(ir) => {
                let raw_values = extract_f64(&ir.x);
                if raw_values.is_empty() {
                    continue;
                }

                // For log x-axis, transform values to log10 before binning
                let values = if x_log {
                    raw_values
                        .iter()
                        .filter(|v| **v > 0.0)
                        .map(|v| v.log10())
                        .collect::<Vec<_>>()
                } else {
                    raw_values
                };
                if values.is_empty() {
                    continue;
                }

                let (bins, counts) = if let Some(ref bins_ir) = ir.bins {
                    compute_bins_from_ir(&values, bins_ir)
                } else {
                    auto_compute_bins(&values)
                };

                let color = resolve_trace_color(&ir.marker, idx);
                let opacity = ir.marker.as_ref().and_then(|m| m.opacity).unwrap_or(0.7);
                let style = color.mix(opacity).filled();

                // For log y-axis, transform counts to log10 space
                let y_base = if y_log { f64::NEG_INFINITY } else { 0.0 };
                let series =
                    chart
                        .draw_series(bins.iter().zip(counts.iter()).map(
                            |(&(start, end), &count)| {
                                let y_val = if y_log && count > 0 {
                                    (count as f64).log10()
                                } else if y_log {
                                    y_base
                                } else {
                                    count as f64
                                };
                                Rectangle::new([(start, y_base), (end, y_val)], style)
                            },
                        ))
                        .unwrap();

                if let Some(ref name) = ir.name {
                    has_legend = true;
                    series
                        .label(name)
                        .legend(move |(x, y)| Rectangle::new([(x, y - 5), (x + 15, y + 5)], style));
                    legend_entries.push(LegendEntry {
                        name: name.clone(),
                        color,
                        opacity,
                        kind: SwatchKind::Rect,
                    });
                }
            }
            TraceIR::CandlestickPlot(ir) => {
                let dates = extract_strings(&ir.dates);
                let open = extract_f64(&ir.open);
                let high = extract_f64(&ir.high);
                let low = extract_f64(&ir.low);
                let close = extract_f64(&ir.close);

                let inc_color = ir
                    .increasing
                    .as_ref()
                    .and_then(|d| d.line_color.as_ref())
                    .map(convert_rgb)
                    .unwrap_or(RGBColor(38, 166, 91));
                let dec_color = ir
                    .decreasing
                    .as_ref()
                    .and_then(|d| d.line_color.as_ref())
                    .map(convert_rgb)
                    .unwrap_or(RGBColor(239, 85, 59));
                let line_width = ir
                    .increasing
                    .as_ref()
                    .and_then(|d| d.line_width)
                    .unwrap_or(1.0) as u32;
                let candle_width = ir.whisker_width.unwrap_or(0.6) / 2.0;

                let n = dates
                    .len()
                    .min(open.len())
                    .min(high.len())
                    .min(low.len())
                    .min(close.len());

                for i in 0..n {
                    let x = i as f64;
                    let is_increasing = close[i] >= open[i];
                    let color = if is_increasing { inc_color } else { dec_color };
                    let body_lo = open[i].min(close[i]);
                    let body_hi = open[i].max(close[i]);

                    // Wick (low to high)
                    let wick_style = ShapeStyle {
                        color: color.to_rgba(),
                        filled: false,
                        stroke_width: line_width,
                    };
                    chart
                        .draw_series(std::iter::once(PathElement::new(
                            vec![(x, low[i]), (x, high[i])],
                            wick_style,
                        )))
                        .unwrap();

                    // Body (open to close)
                    chart
                        .draw_series(std::iter::once(Rectangle::new(
                            [(x - candle_width, body_lo), (x + candle_width, body_hi)],
                            color.filled(),
                        )))
                        .unwrap();
                }
            }
            _ => {}
        }
    }

    // Draw custom tick labels at exact positions
    let x_tick_values = config.x_axis.as_ref().and_then(|a| a.tick_values.clone());
    let x_tick_labels_cfg = config.x_axis.as_ref().and_then(|a| a.tick_labels.clone());
    let y_tick_values = config.y_axis.as_ref().and_then(|a| a.tick_values.clone());
    let y_tick_labels_cfg = config.y_axis.as_ref().and_then(|a| a.tick_labels.clone());

    let grid_style = ShapeStyle {
        color: RGBColor(200, 200, 200).to_rgba(),
        filled: false,
        stroke_width: 1,
    };
    let tick_style = ShapeStyle {
        color: BLACK.to_rgba(),
        filled: false,
        stroke_width: 1,
    };
    let tick_len = 5i32;
    // Default tick direction: x-axis outside (down), y-axis outside (left)
    let (x_tick_lo, x_tick_hi) = (0, tick_len);
    let (y_tick_lo, y_tick_hi) = (-tick_len, 0);
    let line_zero = traces.iter().any(|t| matches!(t, TraceIR::LinePlot(_)));
    let x_label_anchor = if line_zero && y_min <= 0.0 && y_max >= 0.0 {
        0.0
    } else {
        y_min
    };
    let y_label_anchor = x_min;

    let x_show_axis = config
        .x_axis
        .as_ref()
        .and_then(|a| a.show_axis)
        .unwrap_or(true);
    let y_show_axis = config
        .y_axis
        .as_ref()
        .and_then(|a| a.show_axis)
        .unwrap_or(true);
    let x_val_color_tv = axis_value_color(config.x_axis.as_ref());
    let y_val_color_tv = axis_value_color(config.y_axis.as_ref());
    let x_exponent_tv = config
        .x_axis
        .as_ref()
        .and_then(|a| a.value_exponent.as_ref());
    let y_exponent_tv = config
        .y_axis
        .as_ref()
        .and_then(|a| a.value_exponent.as_ref());
    let x_thousands_tv = config
        .x_axis
        .as_ref()
        .and_then(|a| a.value_thousands)
        .unwrap_or(false);
    let y_thousands_tv = config
        .y_axis
        .as_ref()
        .and_then(|a| a.value_thousands)
        .unwrap_or(false);

    use plotlars_core::components::axis::AxisSide;
    let x_on_top = config
        .x_axis
        .as_ref()
        .and_then(|a| a.axis_side.as_ref())
        .is_some_and(|s| matches!(s, AxisSide::Top));
    let y_on_right = config
        .y_axis
        .as_ref()
        .and_then(|a| a.axis_side.as_ref())
        .is_some_and(|s| matches!(s, AxisSide::Right));

    if x_show_axis {
        if let Some(ref tvs) = x_tick_values {
            let label_style = TextStyle::from(("sans-serif", 12).into_font())
                .color(&x_val_color_tv)
                .pos(Pos::new(
                    HPos::Center,
                    if x_on_top { VPos::Bottom } else { VPos::Top },
                ));
            let anchor_y = if x_on_top { y_max } else { x_label_anchor };
            for (i, &tv) in tvs.iter().enumerate() {
                let label = x_tick_labels_cfg
                    .as_ref()
                    .and_then(|l| l.get(i).cloned())
                    .unwrap_or_else(|| {
                        if let Some(exp) = x_exponent_tv {
                            format_exponent(tv, exp)
                        } else if x_thousands_tv {
                            format_thousands(tv)
                        } else {
                            format!("{tv}")
                        }
                    });
                let (px, _) = chart.backend_coord(&(tv, anchor_y));
                let (_, py_hi) = chart.backend_coord(&(tv, y_max));
                let (_, py_lo) = chart.backend_coord(&(tv, y_min));
                root.draw(&PathElement::new(
                    vec![(px, py_hi), (px, py_lo)],
                    grid_style,
                ))
                .unwrap();
                let label_y = chart.backend_coord(&(tv, anchor_y)).1;
                // Tick mark
                root.draw(&PathElement::new(
                    vec![(px, label_y + x_tick_lo), (px, label_y + x_tick_hi)],
                    tick_style,
                ))
                .unwrap();
                let text_y = if x_on_top {
                    label_y + x_tick_lo - 2
                } else {
                    label_y + x_tick_hi + 2
                };
                root.draw_text(&label, &label_style, (px, text_y)).unwrap();
            }
        }
    }
    if y_show_axis {
        if let Some(ref tvs) = y_tick_values {
            let anchor_x = if y_on_right { x_max } else { y_label_anchor };
            let label_style = TextStyle::from(("sans-serif", 12).into_font())
                .color(&y_val_color_tv)
                .pos(Pos::new(
                    if y_on_right { HPos::Left } else { HPos::Right },
                    VPos::Center,
                ));
            for (i, &tv) in tvs.iter().enumerate() {
                let label = y_tick_labels_cfg
                    .as_ref()
                    .and_then(|l| l.get(i).cloned())
                    .unwrap_or_else(|| {
                        if let Some(exp) = y_exponent_tv {
                            format_exponent(tv, exp)
                        } else if y_thousands_tv {
                            format_thousands(tv)
                        } else {
                            format!("{tv}")
                        }
                    });
                let (_, py) = chart.backend_coord(&(anchor_x, tv));
                let (px_lo, _) = chart.backend_coord(&(x_min, tv));
                let (px_hi, _) = chart.backend_coord(&(x_max, tv));
                root.draw(&PathElement::new(
                    vec![(px_lo, py), (px_hi, py)],
                    grid_style,
                ))
                .unwrap();
                let label_x = chart.backend_coord(&(anchor_x, tv)).0;
                // Tick mark
                root.draw(&PathElement::new(
                    vec![(label_x + y_tick_lo, py), (label_x + y_tick_hi, py)],
                    tick_style,
                ))
                .unwrap();
                let text_x = if y_on_right {
                    label_x + y_tick_hi + 2
                } else {
                    label_x + y_tick_lo - 2
                };
                root.draw_text(&label, &label_style, (text_x, py)).unwrap();
            }
        }
    }

    // Draw zero lines for line plots (after custom tick grid lines so they're on top)
    let has_line_traces = traces.iter().any(|t| matches!(t, TraceIR::LinePlot(_)));
    if has_line_traces {
        let zero_style = ShapeStyle {
            color: BLACK.to_rgba(),
            filled: false,
            stroke_width: 1,
        };
        // Horizontal zero line (X axis at y=0)
        if y_min <= 0.0 && y_max >= 0.0 {
            let (px_lo, py) = chart.backend_coord(&(x_min, 0.0));
            let (px_hi, _) = chart.backend_coord(&(x_max, 0.0));
            root.draw(&PathElement::new(
                vec![(px_lo, py), (px_hi, py)],
                zero_style,
            ))
            .unwrap();
        }
        // Vertical Y axis at left edge
        let (px, py_lo) = chart.backend_coord(&(x_min, y_min));
        let (_, py_hi) = chart.backend_coord(&(x_min, y_max));
        root.draw(&PathElement::new(
            vec![(px, py_hi), (px, py_lo)],
            zero_style,
        ))
        .unwrap();
    }

    draw_axis_titles(root, &config, w, h, 15, 50, x_label_area as u32);

    // Draw right-side y2 axis labels
    if has_y2 {
        let y2_color = axis_value_color(config.y2_axis.as_ref());
        let label_style = TextStyle::from(("sans-serif", 12).into_font())
            .color(&y2_color)
            .pos(Pos::new(HPos::Left, VPos::Center));

        // Draw ~7 evenly spaced tick labels on the right side
        let n_ticks = 7usize;
        let y2_range = y2_max - y2_min;
        for i in 0..=n_ticks {
            let t = i as f64 / n_ticks as f64;
            let y2_val = y2_min + t * y2_range;
            let y_primary = y2_to_primary(y2_val, y2_min, y2_max, y_min, y_max);
            let (px, py) = chart.backend_coord(&(x_max, y_primary));
            // Tick mark
            root.draw(&PathElement::new(
                vec![(px, py), (px + 5, py)],
                ShapeStyle {
                    color: y2_color.to_rgba(),
                    filled: false,
                    stroke_width: 1,
                },
            ))
            .unwrap();
            let label = format_axis_value(y2_val, config.y2_axis.as_ref());
            root.draw_text(&label, &label_style, (px + 7, py)).unwrap();
        }

        // Right axis line
        let (px_lo, py_lo) = chart.backend_coord(&(x_max, y_min));
        let (_, py_hi) = chart.backend_coord(&(x_max, y_max));
        root.draw(&PathElement::new(
            vec![(px_lo, py_hi), (px_lo, py_lo)],
            ShapeStyle {
                color: y2_color.to_rgba(),
                filled: false,
                stroke_width: 1,
            },
        ))
        .unwrap();
    }

    if has_legend {
        apply_legend_config(
            &mut chart,
            root,
            &config,
            w,
            h,
            15,
            50,
            x_label_area as u32,
            &legend_entries,
        );
    }
}

// ── Shape-aware scatter rendering ──────────────────────────────────────

#[allow(clippy::too_many_arguments)]
fn draw_scatter_series<'a, DB: DrawingBackend + 'a>(
    chart: &mut ChartContext<
        'a,
        DB,
        Cartesian2d<plotters::coord::types::RangedCoordf64, plotters::coord::types::RangedCoordf64>,
    >,
    points: &[(f64, f64)],
    size: i32,
    color: RGBColor,
    opacity: f64,
    base_shape: BaseShape,
    fill_mode: FillMode,
    name: Option<&str>,
    has_legend: &mut bool,
) {
    let filled_style = color.mix(opacity).filled();
    let open_style = ShapeStyle {
        color: color.mix(opacity),
        filled: false,
        stroke_width: 2,
    };
    let style = if fill_mode == FillMode::Filled {
        filled_style
    } else {
        open_style
    };

    match base_shape {
        BaseShape::Circle => {
            let series = chart
                .draw_series(
                    points
                        .iter()
                        .map(|&(x, y)| Circle::new((x, y), size, style)),
                )
                .unwrap();
            if let Some(name) = name {
                *has_legend = true;
                series
                    .label(name)
                    .legend(move |(x, y)| Circle::new((x + 4, y), 4, style));
            }
        }
        BaseShape::Cross | BaseShape::X => {
            let series = chart
                .draw_series(points.iter().map(|&(x, y)| Cross::new((x, y), size, style)))
                .unwrap();
            if let Some(name) = name {
                *has_legend = true;
                series
                    .label(name)
                    .legend(move |(x, y)| Cross::new((x + 4, y), 4, style));
            }
        }
        BaseShape::TriangleUp => {
            let series = chart
                .draw_series(
                    points
                        .iter()
                        .map(|&(x, y)| TriangleMarker::new((x, y), size, style)),
                )
                .unwrap();
            if let Some(name) = name {
                *has_legend = true;
                series
                    .label(name)
                    .legend(move |(x, y)| TriangleMarker::new((x + 4, y), 4, style));
            }
        }
        _ => {
            let verts = polygon_vertices_at_origin(base_shape, size);
            let legend_verts = polygon_vertices_at_origin(base_shape, 4);

            if fill_mode == FillMode::Filled {
                let series = chart
                    .draw_series(points.iter().map(|&(x, y)| {
                        EmptyElement::at((x, y)) + Polygon::new(verts.clone(), filled_style)
                    }))
                    .unwrap();
                if let Some(name) = name {
                    *has_legend = true;
                    let lv = legend_verts.clone();
                    series.label(name).legend(move |(x, y)| {
                        EmptyElement::at((x + 4, y)) + Polygon::new(lv.clone(), filled_style)
                    });
                }
            } else {
                let series = chart
                    .draw_series(points.iter().map(|&(x, y)| {
                        let mut v = verts.clone();
                        v.push(v[0]);
                        EmptyElement::at((x, y)) + PathElement::new(v, open_style)
                    }))
                    .unwrap();
                if let Some(name) = name {
                    *has_legend = true;
                    let lv = legend_verts.clone();
                    series.label(name).legend(move |(x, y)| {
                        let mut v = lv.clone();
                        v.push(v[0]);
                        EmptyElement::at((x + 4, y)) + PathElement::new(v, open_style)
                    });
                }
            }
        }
    }
}
