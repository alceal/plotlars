use std::process::Command;

use plotlars_core::components::axis::AxisType;
use plotlars_core::components::{Axis, Orientation};
use plotlars_core::ir::layout::LayoutIR;
use plotlars_core::ir::trace::TraceIR;
use plotlars_core::policy::enforce_strict;
use plotlars_core::Plot;
use plotters::chart::MeshStyle;
use plotters::coord::ranged1d::ValueFormatter;
use plotters::prelude::*;
use plotters::style::text_anchor::{HPos, Pos, VPos};

use crate::converters::components::{
    convert_rgb, default_color, resolve_trace_color, resolve_trace_shape, BaseShape, FillMode,
};
use crate::converters::layout::{
    extract_layout_config, format_thousands, resolve_tick_size, LayoutConfig,
};
use crate::converters::trace::{
    auto_compute_bins, collect_bar_categories, collect_timeseries_labels, compute_bar_ranges,
    compute_bins_from_ir, compute_numeric_ranges, count_bar_groups, extract_f64, extract_strings,
    extract_timeseries_points, extract_xy_pairs, histogram_max_count, is_horizontal_bar,
};

const DEFAULT_WIDTH: u32 = 800;
const DEFAULT_HEIGHT: u32 = 600;

#[derive(Clone, Copy)]
enum SwatchKind {
    Line(u32),
    Rect,
    Shape(BaseShape, FillMode),
}

#[derive(Clone)]
struct LegendEntry {
    name: String,
    color: RGBColor,
    opacity: f64,
    kind: SwatchKind,
}

fn draw_plot_title<DB: DrawingBackend>(
    root: &DrawingArea<DB, plotters::coord::Shift>,
    config: &LayoutConfig,
    width: u32,
    height: u32,
) {
    let title = match config.title {
        Some(ref t) => t,
        None => return,
    };

    let font_name = config.title_font.as_str();
    let font_size = config.title_font_size as f64;
    let color = config
        .title_color
        .as_ref()
        .map(convert_rgb)
        .unwrap_or(BLACK);

    let style = TextStyle::from((font_name, font_size).into_font())
        .color(&color)
        .pos(Pos::new(HPos::Center, VPos::Top));

    // Position: default is centered at top, user can override with x/y
    let tx = config
        .title_x
        .map(|x| (x * width as f64) as i32)
        .unwrap_or(width as i32 / 2);
    let ty = config
        .title_y
        .map(|y| ((1.0 - y) * height as f64) as i32)
        .unwrap_or((15 + title_top_margin(config) as i32) / 2);

    root.draw_text(title, &style, (tx, ty)).unwrap();
}

fn title_top_margin(config: &LayoutConfig) -> u32 {
    if config.title.is_some() {
        config.title_font_size + 10
    } else {
        0
    }
}

#[allow(clippy::too_many_arguments)]
fn draw_axis_titles<DB: DrawingBackend>(
    root: &DrawingArea<DB, plotters::coord::Shift>,
    config: &LayoutConfig,
    width: u32,
    height: u32,
    chart_margin: u32,
    y_label_area: u32,
    x_label_area: u32,
) {
    let top_margin = chart_margin + title_top_margin(config);

    // X-axis title
    if let Some(ref label) = config.x_label {
        let color = convert_rgb(&config.x_label_color);
        let size = config.x_label_size as f64;
        let style = TextStyle::from((config.x_label_font.as_str(), size).into_font())
            .color(&color)
            .pos(Pos::new(HPos::Center, VPos::Bottom));
        let cx = config
            .x_label_x
            .map(|x| (x * width as f64) as i32)
            .unwrap_or((chart_margin + y_label_area + width - chart_margin) as i32 / 2);
        let cy = config
            .x_label_y
            .map(|y| ((1.0 - y) * height as f64) as i32)
            .unwrap_or(height as i32 - 5);
        root.draw_text(label, &style, (cx, cy)).unwrap();
    }

    // Y-axis title (rotated)
    if let Some(ref label) = config.y_label {
        let color = convert_rgb(&config.y_label_color);
        let size = config.y_label_size as f64;
        let style = TextStyle::from(
            (config.y_label_font.as_str(), size)
                .into_font()
                .transform(FontTransform::Rotate270),
        )
        .color(&color)
        .pos(Pos::new(HPos::Center, VPos::Center));
        let cx = config
            .y_label_x
            .map(|x| (x * width as f64) as i32)
            .unwrap_or(config.y_label_size as i32 / 2 + 2);
        let cy = config
            .y_label_y
            .map(|y| ((1.0 - y) * height as f64) as i32)
            .unwrap_or((top_margin + height - chart_margin - x_label_area) as i32 / 2);
        root.draw_text(label, &style, (cx, cy)).unwrap();
    }
}

fn configure_label_areas(
    builder: &mut ChartBuilder<'_, '_, impl DrawingBackend>,
    config: &LayoutConfig,
    x_label_area: u32,
    y_label_area: u32,
) {
    use plotlars_core::components::axis::AxisSide;

    let x_side = config.x_axis.as_ref().and_then(|a| a.axis_side.as_ref());
    let y_side = config.y_axis.as_ref().and_then(|a| a.axis_side.as_ref());

    match x_side {
        Some(AxisSide::Top) => {
            builder.top_x_label_area_size(x_label_area);
        }
        _ => {
            builder.x_label_area_size(x_label_area);
        }
    }

    match y_side {
        Some(AxisSide::Right) => {
            builder.right_y_label_area_size(y_label_area);
        }
        _ => {
            builder.y_label_area_size(y_label_area);
        }
    }
}

fn is_horizontal_legend(layout: &LayoutIR) -> bool {
    layout
        .legend
        .as_ref()
        .and_then(|l| l.orientation.as_ref())
        .is_some_and(|o| matches!(o, Orientation::Horizontal))
}

fn estimate_text_width(text: &str, font_size: u32) -> u32 {
    (text.len() as f64 * font_size as f64 * 0.52).ceil() as u32
}

fn resolve_dimensions(layout: &LayoutIR) -> (u32, u32) {
    match &layout.dimensions {
        Some(dims) => {
            let w = dims.width.unwrap_or(DEFAULT_WIDTH as usize) as u32;
            let h = dims.height.unwrap_or(DEFAULT_HEIGHT as usize) as u32;
            (w, h)
        }
        None => (DEFAULT_WIDTH, DEFAULT_HEIGHT),
    }
}

fn render_to_backend<DB: DrawingBackend>(
    plot: &impl Plot,
    root: DrawingArea<DB, plotters::coord::Shift>,
) {
    let layout = plot.ir_layout();
    let traces = plot.ir_traces();

    root.fill(&WHITE).unwrap();

    let mut unsupported = Vec::new();

    if traces.is_empty() {
        root.present().unwrap();
        return;
    }

    match &traces[0] {
        TraceIR::BarPlot(_) => {
            if is_horizontal_bar(traces) {
                render_bar_horizontal(&root, layout, traces, &mut unsupported);
            } else {
                render_bar_vertical(&root, layout, traces, &mut unsupported);
            }
        }
        _ => render_numeric(&root, layout, traces, &mut unsupported),
    }

    enforce_strict("plotters", &unsupported);
    root.present().unwrap();
}

// ── Log-axis helpers ────────────────────────────────────────────────────

fn is_log_axis(axis: &Option<Axis>) -> bool {
    axis.as_ref()
        .and_then(|a| a.axis_type.as_ref())
        .is_some_and(|t| matches!(t, AxisType::Log))
}

/// Transform a range to log10 space, clamping the lower bound to a positive value.
fn log_range(min: f64, max: f64) -> (f64, f64) {
    let lo = if min > 0.0 { min } else { max * 1e-6 };
    (lo.log10(), max.log10())
}

/// Transform a slice of (x, y) pairs to log10 space on the requested axes,
/// filtering out non-positive values on log axes.
fn log_transform_points(points: &[(f64, f64)], x_log: bool, y_log: bool) -> Vec<(f64, f64)> {
    points
        .iter()
        .filter(|(x, y)| (!x_log || *x > 0.0) && (!y_log || *y > 0.0))
        .map(|&(x, y)| {
            let lx = if x_log { x.log10() } else { x };
            let ly = if y_log { y.log10() } else { y };
            (lx, ly)
        })
        .collect()
}

/// Format a log10 value back to the original scale for mesh labels.
fn format_log_label(v: &f64) -> String {
    let original = 10.0_f64.powf(*v);
    if original >= 1.0 && (original - original.round()).abs() < original * 1e-9 {
        format!("{}", original.round() as u64)
    } else if original >= 0.01 {
        let s = format!("{original:.6}");
        s.trim_end_matches('0').trim_end_matches('.').to_string()
    } else {
        format!("{original:.2e}")
    }
}

/// Format a value using the given ValueExponent style.
fn format_exponent(v: f64, exp: &plotlars_core::components::ValueExponent) -> String {
    use plotlars_core::components::ValueExponent;
    match exp {
        ValueExponent::None => {
            let s = format!("{v:.6}");
            s.trim_end_matches('0').trim_end_matches('.').to_string()
        }
        ValueExponent::SmallE => format!("{v:.2e}"),
        ValueExponent::CapitalE => {
            let s = format!("{v:.2e}");
            s.replace('e', "E")
        }
        ValueExponent::Power => {
            if v == 0.0 {
                return "0".to_string();
            }
            let exp10 = v.abs().log10().floor() as i32;
            let mantissa = v / 10.0_f64.powi(exp10);
            if (mantissa - 1.0).abs() < 1e-9 {
                format!("10^{exp10}")
            } else {
                format!("{mantissa:.1}\u{00d7}10^{exp10}")
            }
        }
        ValueExponent::SI => {
            let abs = v.abs();
            let (divisor, suffix) = if abs >= 1e12 {
                (1e12, "T")
            } else if abs >= 1e9 {
                (1e9, "G")
            } else if abs >= 1e6 {
                (1e6, "M")
            } else if abs >= 1e3 {
                (1e3, "k")
            } else if abs >= 1.0 || abs == 0.0 {
                (1.0, "")
            } else if abs >= 1e-3 {
                (1e-3, "m")
            } else if abs >= 1e-6 {
                (1e-6, "\u{00b5}")
            } else if abs >= 1e-9 {
                (1e-9, "n")
            } else {
                (1e-12, "p")
            };
            let scaled = v / divisor;
            let s = format!("{scaled:.2}");
            let trimmed = s.trim_end_matches('0').trim_end_matches('.');
            format!("{trimmed}{suffix}")
        }
        ValueExponent::B => {
            let abs = v.abs();
            let (divisor, suffix) = if abs >= 1e9 {
                (1e9, "B")
            } else if abs >= 1e6 {
                (1e6, "M")
            } else if abs >= 1e3 {
                (1e3, "K")
            } else {
                (1.0, "")
            };
            let scaled = v / divisor;
            let s = format!("{scaled:.2}");
            let trimmed = s.trim_end_matches('0').trim_end_matches('.');
            format!("{trimmed}{suffix}")
        }
    }
}

// ── Category / Date axis helpers ───────────────────────────────────────

fn is_category_axis(axis: &Option<Axis>) -> bool {
    axis.as_ref()
        .and_then(|a| a.axis_type.as_ref())
        .is_some_and(|t| matches!(t, AxisType::Category | AxisType::Date))
}

fn is_date_axis(axis: &Option<Axis>) -> bool {
    axis.as_ref()
        .and_then(|a| a.axis_type.as_ref())
        .is_some_and(|t| matches!(t, AxisType::Date))
}

/// Collect unique x-string labels from scatter/line traces for category/date axes.
fn collect_string_x_labels(traces: &[TraceIR]) -> Vec<String> {
    let mut labels = Vec::new();
    for trace in traces {
        let col = match trace {
            TraceIR::ScatterPlot(ir) => Some(&ir.x),
            TraceIR::LinePlot(ir) => Some(&ir.x),
            _ => None,
        };
        if let Some(col) = col {
            labels.extend(extract_strings(col));
        }
    }
    let mut seen = std::collections::HashSet::new();
    labels.retain(|s| seen.insert(s.clone()));
    labels
}

/// Convert string x-values + f64 y-values into (index, y) pairs using a category map.
fn category_xy_pairs(
    x_col: &plotlars_core::ir::data::ColumnData,
    y_col: &plotlars_core::ir::data::ColumnData,
    cat_labels: &[String],
    y_log: bool,
) -> Vec<(f64, f64)> {
    let x_strs = extract_strings(x_col);
    let y_vals = extract_f64(y_col);
    x_strs
        .iter()
        .zip(y_vals.iter())
        .filter_map(|(xs, &y)| {
            let idx = cat_labels.iter().position(|c| c == xs)?;
            let y = if y_log {
                if y <= 0.0 {
                    return None;
                }
                y.log10()
            } else {
                y
            };
            Some((idx as f64, y))
        })
        .collect()
}

// ── Numeric chart (scatter, line, histogram) ────────────────────────────

fn render_numeric<DB: DrawingBackend>(
    root: &DrawingArea<DB, plotters::coord::Shift>,
    layout: &LayoutIR,
    traces: &[TraceIR],
    unsupported: &mut Vec<String>,
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

    // Collect timeseries labels for custom x-axis formatting
    let ts_labels = collect_timeseries_labels(traces);

    let (w, h) = resolve_dimensions(layout);
    draw_plot_title(root, &config, w, h);

    let mut builder = ChartBuilder::on(root);
    let x_label_area = if ts_labels.is_empty() { 40 } else { 60 };
    builder
        .margin_top(15 + title_top_margin(&config))
        .margin_bottom(15)
        .margin_left(15)
        .margin_right(15);
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

        let xvc = config.x_axis.as_ref().and_then(|a| a.value_color.as_ref()).map(convert_rgb).unwrap_or(BLACK);
        let yvc = config.y_axis.as_ref().and_then(|a| a.value_color.as_ref()).map(convert_rgb).unwrap_or(BLACK);
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

        let x_exponent = config.x_axis.as_ref().and_then(|a| a.value_exponent.as_ref());
        let y_exponent = config.y_axis.as_ref().and_then(|a| a.value_exponent.as_ref());

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
        let x_grid_width = config.x_axis.as_ref().and_then(|a| a.grid_width).unwrap_or(1) as u32;
        let y_grid_width = config.y_axis.as_ref().and_then(|a| a.grid_width).unwrap_or(1) as u32;

        if x_grid_color.is_some() || y_grid_color.is_some() {
            let default_color = RGBColor(200, 200, 200);

            // X-axis grid: vertical lines at x tick positions
            if x_show_grid != Some(false) {
                let color = x_grid_color.map(convert_rgb).unwrap_or(default_color);
                let style = ShapeStyle { color: color.to_rgba(), filled: false, stroke_width: x_grid_width };
                // Use plotters' generated label positions (approximately 11 ticks)
                let range = x_max - x_min;
                let step = range / 10.0;
                if step > 0.0 {
                    let mut v = x_min;
                    while v <= x_max + step * 0.01 {
                        let (px, py_lo) = chart.backend_coord(&(v, y_min));
                        let (_, py_hi) = chart.backend_coord(&(v, y_max));
                        root.draw(&PathElement::new(vec![(px, py_hi), (px, py_lo)], style)).unwrap();
                        v += step;
                    }
                }
            }

            // Y-axis grid: horizontal lines at y tick positions
            if y_show_grid != Some(false) {
                let color = y_grid_color.map(convert_rgb).unwrap_or(default_color);
                let style = ShapeStyle { color: color.to_rgba(), filled: false, stroke_width: y_grid_width };
                let range = y_max - y_min;
                let step = range / 10.0;
                if step > 0.0 {
                    let mut v = y_min;
                    while v <= y_max + step * 0.01 {
                        let (px_lo, py) = chart.backend_coord(&(x_min, v));
                        let (px_hi, _) = chart.backend_coord(&(x_max, v));
                        root.draw(&PathElement::new(vec![(px_lo, py), (px_hi, py)], style)).unwrap();
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
        root.draw(&PathElement::new(vec![(px_lo, py), (px_hi, py)], axis_line_style))
            .unwrap();
    }
    if y_show_line != Some(false) && x_show_line == Some(false) {
        // y-axis visible, x-axis hidden: draw left line
        let (px, py_lo) = chart.backend_coord(&(x_min, y_min));
        let (_, py_hi) = chart.backend_coord(&(x_min, y_max));
        root.draw(&PathElement::new(vec![(px, py_hi), (px, py_lo)], axis_line_style))
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
                    if x_log || y_log { log_transform_points(&raw, x_log, y_log) } else { raw }
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
                    if x_log || y_log { log_transform_points(&raw, x_log, y_log) } else { raw }
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

                if draw_lines {
                    let series = chart
                        .draw_series(LineSeries::new(
                            points.iter().map(|&(x, y)| (x, y)),
                            line_style,
                        ))
                        .unwrap();

                    if let Some(ref name) = ir.name {
                        has_legend = true;
                        series.label(name).legend(move |(x, y)| {
                            PathElement::new(vec![(x, y), (x + 20, y)], line_style)
                        });
                        legend_entries.push(LegendEntry {
                            name: name.clone(),
                            color,
                            opacity: 1.0,
                            kind: SwatchKind::Line(width),
                        });
                    }
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
                let (raw_points, _) = extract_timeseries_points(&ir.x, &ir.y);
                let points = if x_log || y_log {
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

                let series = chart
                    .draw_series(LineSeries::new(
                        points.iter().map(|&(x, y)| (x, y)),
                        line_style,
                    ))
                    .unwrap();

                if let Some(ref name) = ir.name {
                    has_legend = true;
                    series.label(name).legend(move |(x, y)| {
                        PathElement::new(vec![(x, y), (x + 20, y)], line_style)
                    });
                    legend_entries.push(LegendEntry {
                        name: name.clone(),
                        color,
                        opacity: 1.0,
                        kind: SwatchKind::Line(width),
                    });
                }

                if ir.y_axis_ref.is_some() {
                    plotlars_core::policy::report_unsupported(
                        "plotters",
                        "TimeSeriesPlot",
                        "y_axis_ref",
                        unsupported,
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
    let x_label_anchor = if line_zero && y_min <= 0.0 && y_max >= 0.0 { 0.0 } else { y_min };
    let y_label_anchor = x_min;

    let x_show_axis = config.x_axis.as_ref().and_then(|a| a.show_axis).unwrap_or(true);
    let y_show_axis = config.y_axis.as_ref().and_then(|a| a.show_axis).unwrap_or(true);
    let x_val_color_tv = config.x_axis.as_ref().and_then(|a| a.value_color.as_ref()).map(convert_rgb).unwrap_or(BLACK);
    let y_val_color_tv = config.y_axis.as_ref().and_then(|a| a.value_color.as_ref()).map(convert_rgb).unwrap_or(BLACK);
    let x_exponent_tv = config.x_axis.as_ref().and_then(|a| a.value_exponent.as_ref());
    let y_exponent_tv = config.y_axis.as_ref().and_then(|a| a.value_exponent.as_ref());
    let x_thousands_tv = config.x_axis.as_ref().and_then(|a| a.value_thousands).unwrap_or(false);
    let y_thousands_tv = config.y_axis.as_ref().and_then(|a| a.value_thousands).unwrap_or(false);

    use plotlars_core::components::axis::AxisSide;
    let x_on_top = config.x_axis.as_ref().and_then(|a| a.axis_side.as_ref()).is_some_and(|s| matches!(s, AxisSide::Top));
    let y_on_right = config.y_axis.as_ref().and_then(|a| a.axis_side.as_ref()).is_some_and(|s| matches!(s, AxisSide::Right));

    if x_show_axis {
        if let Some(ref tvs) = x_tick_values {
            let label_style = TextStyle::from(("sans-serif", 12).into_font())
                .color(&x_val_color_tv)
                .pos(Pos::new(HPos::Center, if x_on_top { VPos::Bottom } else { VPos::Top }));
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
                root.draw(&PathElement::new(vec![(px, py_hi), (px, py_lo)], grid_style))
                    .unwrap();
                let label_y = chart.backend_coord(&(tv, anchor_y)).1;
                // Tick mark
                root.draw(&PathElement::new(
                    vec![(px, label_y + x_tick_lo), (px, label_y + x_tick_hi)],
                    tick_style,
                ))
                .unwrap();
                let text_y = if x_on_top { label_y + x_tick_lo - 2 } else { label_y + x_tick_hi + 2 };
                root.draw_text(&label, &label_style, (px, text_y))
                    .unwrap();
            }
        }
    }
    if y_show_axis {
        if let Some(ref tvs) = y_tick_values {
            let anchor_x = if y_on_right { x_max } else { y_label_anchor };
            let label_style = TextStyle::from(("sans-serif", 12).into_font())
                .color(&y_val_color_tv)
                .pos(Pos::new(if y_on_right { HPos::Left } else { HPos::Right }, VPos::Center));
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
                root.draw(&PathElement::new(vec![(px_lo, py), (px_hi, py)], grid_style))
                    .unwrap();
                let label_x = chart.backend_coord(&(anchor_x, tv)).0;
                // Tick mark
                root.draw(&PathElement::new(
                    vec![(label_x + y_tick_lo, py), (label_x + y_tick_hi, py)],
                    tick_style,
                ))
                .unwrap();
                let text_x = if y_on_right { label_x + y_tick_hi + 2 } else { label_x + y_tick_lo - 2 };
                root.draw_text(&label, &label_style, (text_x, py))
                    .unwrap();
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
            root.draw(&PathElement::new(vec![(px_lo, py), (px_hi, py)], zero_style))
                .unwrap();
        }
        // Vertical Y axis at left edge
        let (px, py_lo) = chart.backend_coord(&(x_min, y_min));
        let (_, py_hi) = chart.backend_coord(&(x_min, y_max));
        root.draw(&PathElement::new(vec![(px, py_hi), (px, py_lo)], zero_style))
            .unwrap();
    }

    draw_axis_titles(root, &config, w, h, 15, 50, x_label_area as u32);

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

// Vertex functions centered at origin (0, 0) for use with EmptyElement composition.
// These produce backend-pixel offsets that get composed onto a data-coordinate anchor.
/// Draw a legend swatch shape at a pixel center using the same shape as data points.
fn draw_legend_swatch_shape<DB: DrawingBackend>(
    root: &DrawingArea<DB, plotters::coord::Shift>,
    cx: i32,
    cy: i32,
    base_shape: BaseShape,
    fill_mode: FillMode,
    color: RGBColor,
    opacity: f64,
) {
    let r = 4i32;
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
            root.draw(&Circle::new((cx, cy), r, style)).unwrap();
        }
        BaseShape::Cross | BaseShape::X => {
            root.draw(&Cross::new((cx, cy), r, style)).unwrap();
        }
        BaseShape::TriangleUp => {
            root.draw(&TriangleMarker::new((cx, cy), r, style)).unwrap();
        }
        _ => {
            let verts: Vec<(i32, i32)> = polygon_vertices_at_origin(base_shape, r)
                .into_iter()
                .map(|(x, y)| (cx + x, cy + y))
                .collect();
            if fill_mode == FillMode::Filled {
                root.draw(&Polygon::new(verts, filled_style)).unwrap();
            } else {
                let mut closed = verts.clone();
                closed.push(verts[0]);
                root.draw(&PathElement::new(closed, open_style)).unwrap();
            }
        }
    }
}

fn polygon_vertices_at_origin(base_shape: BaseShape, r: i32) -> Vec<(i32, i32)> {
    use crate::converters::components::*;
    match base_shape {
        BaseShape::Square => square_vertices(0, 0, r),
        BaseShape::Diamond => diamond_vertices(0, 0, r),
        BaseShape::TriangleDown => triangle_down_vertices(0, 0, r),
        BaseShape::TriangleLeft => triangle_left_vertices(0, 0, r),
        BaseShape::TriangleRight => triangle_right_vertices(0, 0, r),
        BaseShape::Pentagon => regular_polygon_vertices(0, 0, r, 5),
        BaseShape::Hexagon => regular_polygon_vertices(0, 0, r, 6),
        BaseShape::Octagon => regular_polygon_vertices(0, 0, r, 8),
        BaseShape::Star => star_vertices(0, 0, r),
        _ => unreachable!(),
    }
}

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

// ── Vertical bar chart ──────────────────────────────────────────────────

fn render_bar_vertical<DB: DrawingBackend>(
    root: &DrawingArea<DB, plotters::coord::Shift>,
    layout: &LayoutIR,
    traces: &[TraceIR],
    unsupported: &mut Vec<String>,
) {
    let config = extract_layout_config(layout, unsupported);
    let categories = collect_bar_categories(traces);
    let n_cats = categories.len();
    let n_groups = count_bar_groups(traces);
    let (_, max_val) = compute_bar_ranges(traces);

    let x_range = -0.5..(n_cats as f64 - 0.5);
    let y_hi = config
        .y_range
        .map(|(_, hi)| hi)
        .unwrap_or((max_val * 1.1).max(1.0));
    let y_lo = config.y_range.map(|(lo, _)| lo).unwrap_or(0.0);
    let y_range = y_lo..y_hi;

    let (w, h) = resolve_dimensions(layout);
    draw_plot_title(root, &config, w, h);

    let mut builder = ChartBuilder::on(root);
    builder
        .margin_top(15 + title_top_margin(&config))
        .margin_bottom(15)
        .margin_left(15)
        .margin_right(15);
    configure_label_areas(&mut builder, &config, 40, 50);

    let mut chart = builder.build_cartesian_2d(x_range, y_range).unwrap();

    let categories_clone = categories.clone();
    let x_formatter = move |v: &f64| {
        let idx = v.round() as usize;
        categories_clone.get(idx).cloned().unwrap_or_default()
    };
    {
        let mut mesh = chart.configure_mesh();
        mesh.x_labels(n_cats).x_label_formatter(&x_formatter);

        let xvc = config.x_axis.as_ref().and_then(|a| a.value_color.as_ref()).map(convert_rgb).unwrap_or(BLACK);
        let yvc = config.y_axis.as_ref().and_then(|a| a.value_color.as_ref()).map(convert_rgb).unwrap_or(BLACK);
        apply_mesh_axis_config(&mut mesh, &config, &xvc, &yvc);

        let y_fmt;
        let y_thousands = config
            .y_axis
            .as_ref()
            .and_then(|a| a.value_thousands)
            .unwrap_or(false);
        if y_thousands {
            y_fmt = |v: &f64| format_thousands(*v);
            mesh.y_label_formatter(&y_fmt);
        }

        mesh.draw().unwrap();
    }

    if layout.legend_title.is_some() && !is_horizontal_legend(layout) {
        chart
            .draw_series(std::iter::empty::<Circle<(f64, f64), i32>>())
            .unwrap()
            .label("  ")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x, y)], TRANSPARENT));
    }

    let bar_total_width = 0.8;
    let bar_width = if n_groups > 0 {
        bar_total_width / n_groups as f64
    } else {
        bar_total_width
    };
    let mut has_legend = false;
    let mut legend_entries: Vec<LegendEntry> = Vec::new();
    let mut group_idx = 0usize;

    for trace in traces {
        if let TraceIR::BarPlot(ir) = trace {
            let labels = extract_strings(&ir.labels);
            let values = extract_f64(&ir.values);
            let errors = ir.error.as_ref().map(extract_f64);
            let color = resolve_trace_color(&ir.marker, group_idx);
            let style = color.filled();

            let offset = (group_idx as f64 - (n_groups as f64 - 1.0) / 2.0) * bar_width;

            let rects: Vec<_> = labels
                .iter()
                .zip(values.iter())
                .filter_map(|(label, &val)| {
                    let cat_idx = categories.iter().position(|c| c == label)?;
                    let center = cat_idx as f64 + offset;
                    let x0 = center - bar_width / 2.0;
                    let x1 = center + bar_width / 2.0;
                    Some(Rectangle::new([(x0, 0.0), (x1, val)], style))
                })
                .collect();

            let series = chart.draw_series(rects).unwrap();

            if let Some(ref name) = ir.name {
                has_legend = true;
                series
                    .label(name)
                    .legend(move |(x, y)| Rectangle::new([(x, y - 5), (x + 15, y + 5)], style));
                legend_entries.push(LegendEntry {
                    name: name.clone(),
                    color,
                    opacity: 1.0,
                    kind: SwatchKind::Rect,
                });
            }

            // Draw error bars
            if let Some(errors) = errors {
                let cap_half_w = bar_width * 0.15;
                let err_style = ShapeStyle {
                    color: BLACK.to_rgba(),
                    filled: false,
                    stroke_width: 1,
                };
                let mut err_lines: Vec<PathElement<(f64, f64)>> = Vec::new();
                for ((label, &val), &err) in
                    labels.iter().zip(values.iter()).zip(errors.iter())
                {
                    if let Some(cat_idx) = categories.iter().position(|c| c == label) {
                        let center = cat_idx as f64 + offset;
                        let lo = val - err;
                        let hi = val + err;
                        // Vertical bar
                        err_lines.push(PathElement::new(
                            vec![(center, lo), (center, hi)],
                            err_style,
                        ));
                        // Top cap
                        err_lines.push(PathElement::new(
                            vec![(center - cap_half_w, hi), (center + cap_half_w, hi)],
                            err_style,
                        ));
                        // Bottom cap
                        err_lines.push(PathElement::new(
                            vec![(center - cap_half_w, lo), (center + cap_half_w, lo)],
                            err_style,
                        ));
                    }
                }
                chart.draw_series(err_lines).unwrap();
            }

            group_idx += 1;
        }
    }

    if layout.bar_mode.is_some() {
        plotlars_core::policy::report_unsupported("plotters", "BarPlot", "bar_mode", unsupported);
    }

    if has_legend {
        let (w, h) = resolve_dimensions(layout);
        apply_legend_config(&mut chart, root, &config, w, h, 15, 50, 40, &legend_entries);
    }

    draw_axis_titles(root, &config, w, h, 15, 50, 40);
}

// ── Horizontal bar chart ────────────────────────────────────────────────

fn render_bar_horizontal<DB: DrawingBackend>(
    root: &DrawingArea<DB, plotters::coord::Shift>,
    layout: &LayoutIR,
    traces: &[TraceIR],
    unsupported: &mut Vec<String>,
) {
    let config = extract_layout_config(layout, unsupported);
    let categories = collect_bar_categories(traces);
    let n_cats = categories.len();
    let n_groups = count_bar_groups(traces);
    let (_, max_val) = compute_bar_ranges(traces);

    let x_hi = config
        .x_range
        .map(|(_, hi)| hi)
        .unwrap_or((max_val * 1.1).max(1.0));
    let x_lo = config.x_range.map(|(lo, _)| lo).unwrap_or(0.0);
    let x_range = x_lo..x_hi;
    let y_range = -0.5..(n_cats as f64 - 0.5);

    let (w, h) = resolve_dimensions(layout);
    draw_plot_title(root, &config, w, h);

    let mut builder = ChartBuilder::on(root);
    builder
        .margin_top(15 + title_top_margin(&config))
        .margin_bottom(15)
        .margin_left(15)
        .margin_right(15);
    configure_label_areas(&mut builder, &config, 40, 70);

    let mut chart = builder.build_cartesian_2d(x_range, y_range).unwrap();

    let categories_clone = categories.clone();
    let y_formatter = move |v: &f64| {
        let idx = v.round() as usize;
        categories_clone.get(idx).cloned().unwrap_or_default()
    };
    {
        let mut mesh = chart.configure_mesh();
        mesh.y_labels(n_cats).y_label_formatter(&y_formatter);

        let xvc = config.x_axis.as_ref().and_then(|a| a.value_color.as_ref()).map(convert_rgb).unwrap_or(BLACK);
        let yvc = config.y_axis.as_ref().and_then(|a| a.value_color.as_ref()).map(convert_rgb).unwrap_or(BLACK);
        apply_mesh_axis_config(&mut mesh, &config, &xvc, &yvc);

        let x_fmt;
        let x_thousands = config
            .x_axis
            .as_ref()
            .and_then(|a| a.value_thousands)
            .unwrap_or(false);
        if x_thousands {
            x_fmt = |v: &f64| format_thousands(*v);
            mesh.x_label_formatter(&x_fmt);
        }

        mesh.draw().unwrap();
    }

    if layout.legend_title.is_some() && !is_horizontal_legend(layout) {
        chart
            .draw_series(std::iter::empty::<Circle<(f64, f64), i32>>())
            .unwrap()
            .label("  ")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x, y)], TRANSPARENT));
    }

    let bar_total_width = 0.8;
    let bar_width = if n_groups > 0 {
        bar_total_width / n_groups as f64
    } else {
        bar_total_width
    };
    let mut has_legend = false;
    let mut legend_entries: Vec<LegendEntry> = Vec::new();
    let mut group_idx = 0usize;

    for trace in traces {
        if let TraceIR::BarPlot(ir) = trace {
            let labels = extract_strings(&ir.labels);
            let values = extract_f64(&ir.values);
            let errors = ir.error.as_ref().map(extract_f64);
            let color = resolve_trace_color(&ir.marker, group_idx);
            let style = color.filled();

            let offset = (group_idx as f64 - (n_groups as f64 - 1.0) / 2.0) * bar_width;

            let rects: Vec<_> = labels
                .iter()
                .zip(values.iter())
                .filter_map(|(label, &val)| {
                    let cat_idx = categories.iter().position(|c| c == label)?;
                    let center = cat_idx as f64 + offset;
                    let y0 = center - bar_width / 2.0;
                    let y1 = center + bar_width / 2.0;
                    Some(Rectangle::new([(0.0, y0), (val, y1)], style))
                })
                .collect();

            let series = chart.draw_series(rects).unwrap();

            if let Some(ref name) = ir.name {
                has_legend = true;
                series
                    .label(name)
                    .legend(move |(x, y)| Rectangle::new([(x, y - 5), (x + 15, y + 5)], style));
                legend_entries.push(LegendEntry {
                    name: name.clone(),
                    color,
                    opacity: 1.0,
                    kind: SwatchKind::Rect,
                });
            }

            // Draw error bars (horizontal direction for horizontal bars)
            if let Some(errors) = errors {
                let cap_half_h = bar_width * 0.15;
                let err_style = ShapeStyle {
                    color: BLACK.to_rgba(),
                    filled: false,
                    stroke_width: 1,
                };
                let mut err_lines: Vec<PathElement<(f64, f64)>> = Vec::new();
                for ((label, &val), &err) in
                    labels.iter().zip(values.iter()).zip(errors.iter())
                {
                    if let Some(cat_idx) = categories.iter().position(|c| c == label) {
                        let center = cat_idx as f64 + offset;
                        let lo = val - err;
                        let hi = val + err;
                        // Horizontal bar
                        err_lines.push(PathElement::new(
                            vec![(lo, center), (hi, center)],
                            err_style,
                        ));
                        // Right cap
                        err_lines.push(PathElement::new(
                            vec![(hi, center - cap_half_h), (hi, center + cap_half_h)],
                            err_style,
                        ));
                        // Left cap
                        err_lines.push(PathElement::new(
                            vec![(lo, center - cap_half_h), (lo, center + cap_half_h)],
                            err_style,
                        ));
                    }
                }
                chart.draw_series(err_lines).unwrap();
            }

            group_idx += 1;
        }
    }

    let (w, h) = resolve_dimensions(layout);
    draw_axis_titles(root, &config, w, h, 15, 70, 40);

    if has_legend {
        apply_legend_config(&mut chart, root, &config, w, h, 15, 70, 40, &legend_entries);
    }
}

// ── Shared mesh & legend helpers ───────────────────────────────────────

fn apply_mesh_axis_config<'a, 'b, X, Y, DB>(
    mesh: &mut MeshStyle<'a, 'b, X, Y, DB>,
    config: &'b LayoutConfig,
    x_val_color: &'b RGBColor,
    y_val_color: &'b RGBColor,
) where
    DB: DrawingBackend + 'a,
    X: Ranged<ValueType = f64> + ValueFormatter<f64>,
    Y: Ranged<ValueType = f64> + ValueFormatter<f64>,
{
    // Grid visibility
    let x_show_grid = config.x_axis.as_ref().and_then(|a| a.show_grid);
    let y_show_grid = config.y_axis.as_ref().and_then(|a| a.show_grid);

    let x_grid_color = config.x_axis.as_ref().and_then(|a| a.grid_color.as_ref());
    let y_grid_color = config.y_axis.as_ref().and_then(|a| a.grid_color.as_ref());
    let x_grid_width = config.x_axis.as_ref().and_then(|a| a.grid_width);
    let y_grid_width = config.y_axis.as_ref().and_then(|a| a.grid_width);

    let has_per_axis_grid = x_grid_color.is_some() || y_grid_color.is_some();

    if has_per_axis_grid {
        // Disable built-in mesh; we draw per-axis grid lines manually after mesh.draw()
        mesh.disable_mesh();
    } else {
        if x_show_grid == Some(false) && y_show_grid == Some(false) {
            mesh.disable_mesh();
        } else if x_show_grid == Some(false) {
            mesh.disable_x_mesh();
        } else if y_show_grid == Some(false) {
            mesh.disable_y_mesh();
        }

        let grid_width = x_grid_width.or(y_grid_width);
        if let Some(gw) = grid_width {
            let grid_style = ShapeStyle {
                color: RGBColor(200, 200, 200).to_rgba(),
                filled: false,
                stroke_width: gw as u32,
            };
            mesh.bold_line_style(grid_style);
        }
    }

    // Axis visibility: show_axis(false) hides line + labels + ticks
    let x_show_axis = config.x_axis.as_ref().and_then(|a| a.show_axis);
    let y_show_axis = config.y_axis.as_ref().and_then(|a| a.show_axis);

    if x_show_axis == Some(false) {
        mesh.disable_x_axis();
    }
    if y_show_axis == Some(false) {
        mesh.disable_y_axis();
    }

    // Axis line styling (global -- axis_style applies to both axes)
    let x_show_line = config.x_axis.as_ref().and_then(|a| a.show_line);
    let y_show_line = config.y_axis.as_ref().and_then(|a| a.show_line);
    let line_color = config
        .x_axis
        .as_ref()
        .and_then(|a| a.line_color.as_ref())
        .or_else(|| config.y_axis.as_ref().and_then(|a| a.line_color.as_ref()));
    let line_width = config
        .x_axis
        .as_ref()
        .and_then(|a| a.line_width)
        .or(config.y_axis.as_ref().and_then(|a| a.line_width));

    if x_show_line == Some(false) && y_show_line == Some(false) {
        mesh.axis_style(TRANSPARENT);
    } else if x_show_line == Some(false) || y_show_line == Some(false) {
        // Hide both via mesh, then manually redraw the visible one after mesh.draw()
        mesh.axis_style(TRANSPARENT);
    } else if line_color.is_some() || line_width.is_some() {
        let color = line_color.map(convert_rgb).unwrap_or(BLACK);
        let width = line_width.unwrap_or(1) as u32;
        mesh.axis_style(ShapeStyle {
            color: color.to_rgba(),
            filled: false,
            stroke_width: width,
        });
    }

    // Per-axis tick label font and color
    let x_tick_font = config.x_axis.as_ref().and_then(|a| a.tick_font.as_ref());
    let y_tick_font = config.y_axis.as_ref().and_then(|a| a.tick_font.as_ref());

    let x_font_name = x_tick_font.map(|f| f.as_str()).unwrap_or("sans-serif");
    let y_font_name = y_tick_font.map(|f| f.as_str()).unwrap_or("sans-serif");
    mesh.x_label_style(TextStyle::from((x_font_name, 12).into_font()).color(x_val_color));
    mesh.y_label_style(TextStyle::from((y_font_name, 12).into_font()).color(y_val_color));

    // Tick mark size and direction
    if let Some(ref x_axis) = config.x_axis {
        if let Some(tick_size) = resolve_tick_size(x_axis) {
            mesh.set_tick_mark_size(LabelAreaPosition::Bottom, tick_size);
        }
    }
    if let Some(ref y_axis) = config.y_axis {
        if let Some(tick_size) = resolve_tick_size(y_axis) {
            mesh.set_tick_mark_size(LabelAreaPosition::Left, tick_size);
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn apply_legend_config<'a, DB, CT>(
    _chart: &mut ChartContext<'a, DB, CT>,
    root: &DrawingArea<DB, plotters::coord::Shift>,
    config: &LayoutConfig,
    width: u32,
    height: u32,
    chart_margin: u32,
    y_label_area: u32,
    x_label_area: u32,
    entries: &[LegendEntry],
) where
    DB: DrawingBackend + 'a,
    CT: CoordTranslate,
{
    let is_horizontal = config
        .legend
        .as_ref()
        .and_then(|l| l.orientation.as_ref())
        .is_some_and(|o| matches!(o, Orientation::Horizontal));

    let draw = if is_horizontal {
        draw_horizontal_legend
    } else {
        draw_vertical_legend
    };
    draw(
        root,
        config,
        entries,
        width,
        height,
        chart_margin,
        y_label_area,
        x_label_area,
    );
}

#[allow(clippy::too_many_arguments)]
fn draw_vertical_legend<DB: DrawingBackend>(
    root: &DrawingArea<DB, plotters::coord::Shift>,
    config: &LayoutConfig,
    entries: &[LegendEntry],
    width: u32,
    height: u32,
    chart_margin: u32,
    y_label_area: u32,
    x_label_area: u32,
) {
    if entries.is_empty() {
        return;
    }

    let font_name = config
        .legend
        .as_ref()
        .and_then(|l| l.font.as_ref())
        .map(|s| s.as_str())
        .unwrap_or("sans-serif");
    let font_size = 12u32;
    let title_font_size = 13u32;
    let padding = 10u32;
    let swatch_w = 8u32;
    let swatch_gap = 5u32;
    let title_gap = 4u32;

    // Measure text using backend
    let label_style: TextStyle = (font_name, font_size as f64).into_font().into();
    let line_h = root
        .estimate_text_size("Xg", &label_style)
        .map(|(_, h)| h + 3)
        .unwrap_or(font_size + 3);

    let has_title = config.legend_title.is_some();
    let title_style_est: TextStyle = (font_name, title_font_size as f64).into_font().into();
    let title_h = if has_title {
        root.estimate_text_size("X", &title_style_est)
            .map(|(_, h)| h)
            .unwrap_or(title_font_size)
    } else {
        0
    };

    let max_label_w = entries
        .iter()
        .filter_map(|e| root.estimate_text_size(&e.name, &label_style).ok())
        .map(|(w, _)| w)
        .max()
        .unwrap_or(0);
    let title_w = config
        .legend_title
        .as_ref()
        .and_then(|t| root.estimate_text_size(t, &title_style_est).ok())
        .map(|(w, _)| w)
        .unwrap_or(0);

    let content_w = (swatch_w + swatch_gap + max_label_w).max(title_w);
    let box_w = content_w + 2 * padding;
    let n = entries.len() as u32;
    let box_h = padding + if has_title { title_h + title_gap } else { 0 } + n * line_h + padding;

    // Plot area geometry (canvas coordinates)
    let caption_h = if config.title.is_some() {
        config.title_font_size
    } else {
        0
    };
    let plot_left = (chart_margin + y_label_area) as i32;
    let plot_top = (chart_margin + caption_h) as i32;
    let plot_w = width.saturating_sub(2 * chart_margin + y_label_area);
    let plot_h = height.saturating_sub(2 * chart_margin + x_label_area + caption_h);

    // Position: user-specified or default upper-right
    let (box_x, box_y) = config
        .legend
        .as_ref()
        .and_then(|l| {
            let x = l.x?;
            let y = l.y?;
            Some((
                plot_left + (x * plot_w as f64) as i32,
                plot_top + ((1.0 - y) * plot_h as f64) as i32,
            ))
        })
        .unwrap_or_else(|| (plot_left + plot_w as i32 - box_w as i32 - 5, plot_top + 5));

    // Background
    let bg_color = config
        .legend
        .as_ref()
        .and_then(|l| l.background_color.as_ref())
        .map(convert_rgb)
        .unwrap_or(WHITE);
    root.draw(&Rectangle::new(
        [(box_x, box_y), (box_x + box_w as i32, box_y + box_h as i32)],
        ShapeStyle {
            color: bg_color.to_rgba(),
            filled: true,
            stroke_width: 0,
        },
    ))
    .unwrap();

    // Border
    let border_color = config
        .legend
        .as_ref()
        .and_then(|l| l.border_color.as_ref())
        .map(convert_rgb)
        .unwrap_or(BLACK);
    let border_width = config
        .legend
        .as_ref()
        .and_then(|l| l.border_width)
        .unwrap_or(0) as u32;
    if border_width > 0 {
        let bx1 = box_x + box_w as i32;
        let by1 = box_y + box_h as i32;
        let border_style = ShapeStyle {
            color: border_color.to_rgba(),
            filled: false,
            stroke_width: border_width,
        };
        root.draw(&PathElement::new(
            vec![
                (box_x, box_y),
                (bx1, box_y),
                (bx1, by1),
                (box_x, by1),
                (box_x, box_y),
                (bx1, box_y),
            ],
            border_style,
        ))
        .unwrap();
    }

    // Title
    let mut content_y = box_y + padding as i32;
    if let Some(ref title) = config.legend_title {
        let title_style = TextStyle::from(
            (font_name, title_font_size as f64)
                .into_font()
                .style(FontStyle::Bold),
        )
        .color(&BLACK)
        .pos(Pos::new(HPos::Center, VPos::Top));
        let title_x = box_x + box_w as i32 / 2;
        root.draw_text(title, &title_style, (title_x, content_y))
            .unwrap();
        content_y += title_h as i32 + title_gap as i32;
    }

    // Entries stacked vertically
    for entry in entries {
        let center_y = content_y + line_h as i32 / 2;
        let x = box_x + padding as i32;
        let style = entry.color.mix(entry.opacity).filled();

        match entry.kind {
            SwatchKind::Line(w) => {
                let line_style = ShapeStyle {
                    color: entry.color.mix(entry.opacity),
                    filled: false,
                    stroke_width: w,
                };
                root.draw(&PathElement::new(
                    vec![(x, center_y), (x + swatch_w as i32, center_y)],
                    line_style,
                ))
                .unwrap();
            }
            SwatchKind::Rect => {
                root.draw(&Rectangle::new(
                    [(x, center_y - 5), (x + swatch_w as i32, center_y + 5)],
                    style,
                ))
                .unwrap();
            }
            SwatchKind::Shape(base_shape, fill_mode) => {
                draw_legend_swatch_shape(
                    root,
                    x + swatch_w as i32 / 2,
                    center_y,
                    base_shape,
                    fill_mode,
                    entry.color,
                    entry.opacity,
                );
            }
        }

        // Label
        let label_x = x + swatch_w as i32 + swatch_gap as i32;
        let label_style = TextStyle::from((font_name, font_size as f64).into_font())
            .color(&BLACK)
            .pos(Pos::new(HPos::Left, VPos::Center));
        root.draw_text(&entry.name, &label_style, (label_x, center_y))
            .unwrap();

        content_y += line_h as i32;
    }
}

#[allow(clippy::too_many_arguments)]
fn draw_horizontal_legend<DB: DrawingBackend>(
    root: &DrawingArea<DB, plotters::coord::Shift>,
    config: &LayoutConfig,
    entries: &[LegendEntry],
    width: u32,
    height: u32,
    chart_margin: u32,
    y_label_area: u32,
    x_label_area: u32,
) {
    if entries.is_empty() {
        return;
    }

    let font_name = config
        .legend
        .as_ref()
        .and_then(|l| l.font.as_ref())
        .map(|s| s.as_str())
        .unwrap_or("sans-serif");
    let font_size = 12u32;
    let title_font_size = 13u32;
    let padding = 8u32;
    let swatch_w = 8u32;
    let swatch_gap = 5u32;
    let entry_gap = 14u32;
    let title_gap = 2u32;

    let entry_row_h = font_size;
    let has_title = config.legend_title.is_some();
    let title_h = if has_title { title_font_size } else { 0 };

    // Compute entry widths using backend text measurement
    let label_style: TextStyle = (font_name, font_size as f64).into_font().into();
    let entry_widths: Vec<u32> = entries
        .iter()
        .map(|e| {
            let sw = match e.kind {
                SwatchKind::Line(_) => 12,
                _ => swatch_w,
            };
            let text_w = root
                .estimate_text_size(&e.name, &label_style)
                .map(|(w, _)| w)
                .unwrap_or_else(|_| estimate_text_width(&e.name, font_size));
            sw + swatch_gap + text_w
        })
        .collect();
    let row_w: u32 =
        entry_widths.iter().sum::<u32>() + entry_gap * entries.len().saturating_sub(1) as u32;

    let title_style_est: TextStyle = (font_name, title_font_size as f64).into_font().into();
    let title_w = config
        .legend_title
        .as_ref()
        .map(|t| {
            root.estimate_text_size(t, &title_style_est)
                .map(|(w, _)| w)
                .unwrap_or_else(|_| estimate_text_width(t, title_font_size))
        })
        .unwrap_or(0);

    let inner_w = row_w.max(title_w);
    let box_w = inner_w + 2 * padding;
    let box_h = padding + title_h + if has_title { title_gap } else { 0 } + entry_row_h + padding;

    // Plot area geometry (canvas coordinates)
    let caption_h = if config.title.is_some() {
        config.title_font_size
    } else {
        0
    };
    let plot_left = (chart_margin + y_label_area) as i32;
    let plot_top = (chart_margin + caption_h) as i32;
    let plot_w = width.saturating_sub(2 * chart_margin + y_label_area);
    let plot_h = height.saturating_sub(2 * chart_margin + x_label_area + caption_h);

    // Position: user-specified or default upper-right
    let (box_x, box_y) = config
        .legend
        .as_ref()
        .and_then(|l| {
            let x = l.x?;
            let y = l.y?;
            let px_x = (x * plot_w as f64) as i32;
            let px_y = ((1.0 - y) * plot_h as f64) as i32;
            Some((plot_left + px_x, plot_top + px_y))
        })
        .unwrap_or_else(|| (plot_left + plot_w as i32 - box_w as i32 - 5, plot_top + 5));

    // Background
    let bg_color = config
        .legend
        .as_ref()
        .and_then(|l| l.background_color.as_ref())
        .map(convert_rgb)
        .unwrap_or(WHITE);
    root.draw(&Rectangle::new(
        [(box_x, box_y), (box_x + box_w as i32, box_y + box_h as i32)],
        ShapeStyle {
            color: bg_color.to_rgba(),
            filled: true,
            stroke_width: 0,
        },
    ))
    .unwrap();

    // Border
    let border_color = config
        .legend
        .as_ref()
        .and_then(|l| l.border_color.as_ref())
        .map(convert_rgb)
        .unwrap_or(BLACK);
    let border_width = config
        .legend
        .as_ref()
        .and_then(|l| l.border_width)
        .unwrap_or(0) as u32;
    if border_width > 0 {
        let bx1 = box_x + box_w as i32;
        let by1 = box_y + box_h as i32;
        let border_style = ShapeStyle {
            color: border_color.to_rgba(),
            filled: false,
            stroke_width: border_width,
        };
        root.draw(&PathElement::new(
            vec![
                (box_x, box_y),
                (bx1, box_y),
                (bx1, by1),
                (box_x, by1),
                (box_x, box_y),
                (bx1, box_y),
            ],
            border_style,
        ))
        .unwrap();
    }

    // Title
    let mut content_y = box_y + padding as i32;
    if let Some(ref title) = config.legend_title {
        let title_style = TextStyle::from(
            (font_name, title_font_size as f64)
                .into_font()
                .style(FontStyle::Bold),
        )
        .color(&BLACK)
        .pos(Pos::new(HPos::Center, VPos::Top));
        let title_x = box_x + box_w as i32 / 2;
        root.draw_text(title, &title_style, (title_x, content_y))
            .unwrap();
        content_y += title_h as i32 + title_gap as i32;
    }

    // Entries laid out horizontally
    let entry_center_y = content_y + entry_row_h as i32 / 2;
    let mut x = box_x + padding as i32;

    for (i, entry) in entries.iter().enumerate() {
        let sw = match entry.kind {
            SwatchKind::Line(_) => 12i32,
            _ => swatch_w as i32,
        };
        let style = entry.color.mix(entry.opacity).filled();

        match entry.kind {
            SwatchKind::Line(w) => {
                let line_style = ShapeStyle {
                    color: entry.color.mix(entry.opacity),
                    filled: false,
                    stroke_width: w,
                };
                root.draw(&PathElement::new(
                    vec![(x, entry_center_y), (x + sw, entry_center_y)],
                    line_style,
                ))
                .unwrap();
            }
            SwatchKind::Rect => {
                root.draw(&Rectangle::new(
                    [(x, entry_center_y - 5), (x + sw, entry_center_y + 5)],
                    style,
                ))
                .unwrap();
            }
            SwatchKind::Shape(base_shape, fill_mode) => {
                draw_legend_swatch_shape(
                    root,
                    x + sw / 2,
                    entry_center_y,
                    base_shape,
                    fill_mode,
                    entry.color,
                    entry.opacity,
                );
            }
        }

        // Label
        let label_x = x + sw + swatch_gap as i32;
        let label_style = TextStyle::from((font_name, font_size as f64).into_font())
            .color(&BLACK)
            .pos(Pos::new(HPos::Left, VPos::Center));
        root.draw_text(&entry.name, &label_style, (label_x, entry_center_y))
            .unwrap();

        x += entry_widths[i] as i32 + entry_gap as i32;
    }
}

// ── Output methods ──────────────────────────────────────────────────────

pub fn plot_interactive(plot: &impl Plot) {
    if std::env::var("EVCXR_IS_RUNTIME").is_ok() {
        let svg = render_to_svg_string(plot);
        println!(
            "EVCXR_BEGIN_CONTENT image/svg+xml\n{}\nEVCXR_END_CONTENT",
            svg
        );
        return;
    }

    let tmp = std::env::temp_dir().join("plotlars_tmp.png");
    let path = tmp.to_str().unwrap();
    save_to_file(plot, path);
    open_file(path);
}

pub fn save_to_file(plot: &impl Plot, path: &str) {
    let (w, h) = resolve_dimensions(plot.ir_layout());

    if path.ends_with(".svg") {
        let root = SVGBackend::new(path, (w, h)).into_drawing_area();
        render_to_backend(plot, root);
    } else {
        let root = BitMapBackend::new(path, (w, h)).into_drawing_area();
        render_to_backend(plot, root);
    }
}

pub fn render_to_svg_string(plot: &impl Plot) -> String {
    let (w, h) = resolve_dimensions(plot.ir_layout());
    let mut svg_string = String::new();
    {
        let root = SVGBackend::with_string(&mut svg_string, (w, h)).into_drawing_area();
        render_to_backend(plot, root);
    }
    svg_string
}

fn open_file(path: &str) {
    #[cfg(target_os = "macos")]
    {
        let _ = Command::new("open").arg(path).spawn();
    }
    #[cfg(target_os = "linux")]
    {
        let _ = Command::new("xdg-open").arg(path).spawn();
    }
    #[cfg(target_os = "windows")]
    {
        let _ = Command::new("cmd").args(["/c", "start", path]).spawn();
    }
}
