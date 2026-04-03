use std::process::Command;

use plotlars_core::components::Orientation;
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
    Circle,
    Line(u32),
    Rect,
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
        .margin_right(15)
        .x_label_area_size(x_label_area)
        .y_label_area_size(50);

    let mut chart = builder
        .build_cartesian_2d(x_min..x_max, y_min..y_max)
        .unwrap();

    {
        let mut mesh = chart.configure_mesh();

        apply_mesh_axis_config(&mut mesh, &config);

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

        let ts_formatter;
        if !ts_labels.is_empty() {
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
        } else if x_thousands {
            x_fmt = |v: &f64| format_thousands(*v);
            mesh.x_label_formatter(&x_fmt);
        }

        if y_thousands {
            y_fmt = |v: &f64| format_thousands(*v);
            mesh.y_label_formatter(&y_fmt);
        }

        mesh.draw().unwrap();
    }



    let mut has_legend = false;
    let mut legend_entries: Vec<LegendEntry> = Vec::new();

    for (idx, trace) in traces.iter().enumerate() {
        match trace {
            TraceIR::ScatterPlot(ir) => {
                let points = extract_xy_pairs(&ir.x, &ir.y);
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
                        kind: SwatchKind::Circle,
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
                let points = extract_xy_pairs(&ir.x, &ir.y);
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
                let (points, _) = extract_timeseries_points(&ir.x, &ir.y);
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
                let values = extract_f64(&ir.x);
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

                let series =
                    chart
                        .draw_series(bins.iter().zip(counts.iter()).map(
                            |(&(start, end), &count)| {
                                Rectangle::new([(start, 0.0), (end, count as f64)], style)
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

    draw_axis_titles(root, &config, w, h, 15, 50, x_label_area as u32);

    if has_legend {
        apply_legend_config(&mut chart, root, &config, w, h, 15, 50, x_label_area as u32, &legend_entries);
    }
}

// ── Shape-aware scatter rendering ──────────────────────────────────────

// Vertex functions centered at origin (0, 0) for use with EmptyElement composition.
// These produce backend-pixel offsets that get composed onto a data-coordinate anchor.
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
        .margin_right(15)
        .x_label_area_size(40)
        .y_label_area_size(50);

    let mut chart = builder.build_cartesian_2d(x_range, y_range).unwrap();

    let categories_clone = categories.clone();
    let x_formatter = move |v: &f64| {
        let idx = v.round() as usize;
        categories_clone.get(idx).cloned().unwrap_or_default()
    };
    {
        let mut mesh = chart.configure_mesh();
        mesh.x_labels(n_cats).x_label_formatter(&x_formatter);

        apply_mesh_axis_config(&mut mesh, &config);

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

            if ir.error.is_some() {
                plotlars_core::policy::report_unsupported(
                    "plotters",
                    "BarPlot",
                    "error",
                    unsupported,
                );
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
        .margin_right(15)
        .x_label_area_size(40)
        .y_label_area_size(70);

    let mut chart = builder.build_cartesian_2d(x_range, y_range).unwrap();

    let categories_clone = categories.clone();
    let y_formatter = move |v: &f64| {
        let idx = v.round() as usize;
        categories_clone.get(idx).cloned().unwrap_or_default()
    };
    {
        let mut mesh = chart.configure_mesh();
        mesh.y_labels(n_cats).y_label_formatter(&y_formatter);

        apply_mesh_axis_config(&mut mesh, &config);

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
) where
    DB: DrawingBackend + 'a,
    X: Ranged<ValueType = f64> + ValueFormatter<f64>,
    Y: Ranged<ValueType = f64> + ValueFormatter<f64>,
{
    // Grid visibility
    let x_show_grid = config.x_axis.as_ref().and_then(|a| a.show_grid);
    let y_show_grid = config.y_axis.as_ref().and_then(|a| a.show_grid);

    if x_show_grid == Some(false) && y_show_grid == Some(false) {
        mesh.disable_mesh();
    } else if x_show_grid == Some(false) {
        mesh.disable_x_mesh();
    } else if y_show_grid == Some(false) {
        mesh.disable_y_mesh();
    }

    // Grid color/width (from first axis that specifies it)
    let grid_color = config
        .x_axis
        .as_ref()
        .and_then(|a| a.grid_color.as_ref())
        .or_else(|| config.y_axis.as_ref().and_then(|a| a.grid_color.as_ref()));
    let grid_width = config
        .x_axis
        .as_ref()
        .and_then(|a| a.grid_width)
        .or(config.y_axis.as_ref().and_then(|a| a.grid_width));

    if let Some(gc) = grid_color {
        let gw = grid_width.unwrap_or(1) as u32;
        let grid_style = ShapeStyle {
            color: convert_rgb(gc).to_rgba(),
            filled: false,
            stroke_width: gw,
        };
        mesh.bold_line_style(grid_style);
        mesh.light_line_style(TRANSPARENT);
    } else if let Some(gw) = grid_width {
        let grid_style = ShapeStyle {
            color: RGBColor(200, 200, 200).to_rgba(),
            filled: false,
            stroke_width: gw as u32,
        };
        mesh.bold_line_style(grid_style);
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

    if x_show_line == Some(false) || y_show_line == Some(false) {
        // show_line(false) hides axis lines but keeps labels
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

    // Per-axis label font
    let x_tick_font = config.x_axis.as_ref().and_then(|a| a.tick_font.as_ref());
    let y_tick_font = config.y_axis.as_ref().and_then(|a| a.tick_font.as_ref());

    if let Some(font) = x_tick_font {
        mesh.x_label_style((font.as_str(), 12).into_font());
    }
    if let Some(font) = y_tick_font {
        mesh.y_label_style((font.as_str(), 12).into_font());
    }

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
        .unwrap_or_else(|| {
            (
                plot_left + plot_w as i32 - box_w as i32 - 5,
                plot_top + 5,
            )
        });

    // Background
    let bg_color = config
        .legend
        .as_ref()
        .and_then(|l| l.background_color.as_ref())
        .map(convert_rgb)
        .unwrap_or(WHITE);
    root.draw(&Rectangle::new(
        [
            (box_x, box_y),
            (box_x + box_w as i32, box_y + box_h as i32),
        ],
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
        let title_style =
            TextStyle::from((font_name, title_font_size as f64).into_font().style(FontStyle::Bold))
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
            SwatchKind::Circle => {
                root.draw(&Circle::new((x + swatch_w as i32 / 2, center_y), 4, style))
                    .unwrap();
            }
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

    let title_style_est: TextStyle =
        (font_name, title_font_size as f64).into_font().into();
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
        .unwrap_or_else(|| {
            (
                plot_left + plot_w as i32 - box_w as i32 - 5,
                plot_top + 5,
            )
        });

    // Background
    let bg_color = config
        .legend
        .as_ref()
        .and_then(|l| l.background_color.as_ref())
        .map(convert_rgb)
        .unwrap_or(WHITE);
    root.draw(&Rectangle::new(
        [
            (box_x, box_y),
            (box_x + box_w as i32, box_y + box_h as i32),
        ],
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
        let title_style =
            TextStyle::from((font_name, title_font_size as f64).into_font().style(FontStyle::Bold))
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
            SwatchKind::Circle => {
                root.draw(&Circle::new((x + sw / 2, entry_center_y), 4, style))
                    .unwrap();
            }
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
