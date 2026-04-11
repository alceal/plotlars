use plotlars_core::components::{BarMode, Orientation};
use plotlars_core::ir::layout::LayoutIR;
use plotlars_core::ir::trace::TraceIR;
use plotters::prelude::*;

use crate::converters::components::resolve_trace_color;
use crate::converters::layout::{extract_layout_config, format_thousands};
use crate::converters::trace::{
    collect_bar_categories, compute_bar_ranges, count_bar_groups, extract_f64, extract_strings,
};

use super::axis::{apply_mesh_axis_config, axis_value_color, configure_label_areas};
use super::legend::apply_legend_config;
use super::title::{draw_axis_titles, draw_plot_title, title_top_margin};
use super::{resolve_dimensions, LegendEntry, SwatchKind};

fn is_horizontal_legend(layout: &LayoutIR) -> bool {
    layout
        .legend
        .as_ref()
        .and_then(|l| l.orientation.as_ref())
        .is_some_and(|o| matches!(o, Orientation::Horizontal))
}

fn resolve_bar_mode(layout: &LayoutIR) -> &BarMode {
    layout.bar_mode.as_ref().unwrap_or(&BarMode::Group)
}

fn is_stacked(mode: &BarMode) -> bool {
    matches!(mode, BarMode::Stack | BarMode::Relative)
}

pub(super) fn render_bar_vertical<DB: DrawingBackend>(
    root: &DrawingArea<DB, plotters::coord::Shift>,
    layout: &LayoutIR,
    traces: &[TraceIR],
    unsupported: &mut Vec<String>,
) {
    let config = extract_layout_config(layout, unsupported);
    let bar_mode = resolve_bar_mode(layout);
    let categories = collect_bar_categories(traces);
    let n_cats = categories.len();
    let n_groups = count_bar_groups(traces);
    let (_, max_val) = compute_bar_ranges(traces, is_stacked(bar_mode));

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

        let xvc = axis_value_color(config.x_axis.as_ref());
        let yvc = axis_value_color(config.y_axis.as_ref());
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
    let bar_width = match bar_mode {
        BarMode::Group => {
            if n_groups > 0 {
                bar_total_width / n_groups as f64
            } else {
                bar_total_width
            }
        }
        _ => bar_total_width,
    };

    let mut has_legend = false;
    let mut legend_entries: Vec<LegendEntry> = Vec::new();
    let mut group_idx = 0usize;
    let mut stack_bases = vec![0.0f64; n_cats];
    // Error bars are accumulated during the bar pass and drawn at the end so
    // they sit on top of all bar rectangles regardless of trace order.
    let mut err_lines: Vec<PathElement<(f64, f64)>> = Vec::new();
    let cap_half_w = bar_width * 0.15;
    let err_style = ShapeStyle {
        color: BLACK.to_rgba(),
        filled: false,
        stroke_width: 1,
    };

    for trace in traces {
        if let TraceIR::BarPlot(ir) = trace {
            let labels = extract_strings(&ir.labels);
            let values = extract_f64(&ir.values);
            let errors = ir.error.as_ref().map(extract_f64);
            let color = resolve_trace_color(&ir.marker, group_idx);
            let alpha = if matches!(bar_mode, BarMode::Overlay) {
                0.6
            } else {
                1.0
            };
            let style = ShapeStyle {
                color: color.to_rgba().mix(alpha),
                filled: true,
                stroke_width: 0,
            };

            let rects: Vec<_> = labels
                .iter()
                .zip(values.iter())
                .filter_map(|(label, &val)| {
                    let cat_idx = categories.iter().position(|c| c == label)?;
                    let (center, base, top) = bar_geometry(
                        bar_mode,
                        cat_idx,
                        group_idx,
                        n_groups,
                        bar_width,
                        stack_bases[cat_idx],
                        val,
                    );
                    let x0 = center - bar_width / 2.0;
                    let x1 = center + bar_width / 2.0;
                    Some(Rectangle::new([(x0, base), (x1, top)], style))
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
                    opacity: alpha,
                    kind: SwatchKind::Rect,
                });
            }

            // Collect error-bar geometry now (using the current stack_bases)
            // and draw later, on top of all bars.
            if let Some(ref errors) = errors {
                for ((label, &val), &err) in labels.iter().zip(values.iter()).zip(errors.iter()) {
                    if let Some(cat_idx) = categories.iter().position(|c| c == label) {
                        let (center, _, top) = bar_geometry(
                            bar_mode,
                            cat_idx,
                            group_idx,
                            n_groups,
                            bar_width,
                            stack_bases[cat_idx],
                            val,
                        );
                        let lo = top - err;
                        let hi = top + err;
                        err_lines.push(PathElement::new(
                            vec![(center, lo), (center, hi)],
                            err_style,
                        ));
                        err_lines.push(PathElement::new(
                            vec![(center - cap_half_w, hi), (center + cap_half_w, hi)],
                            err_style,
                        ));
                        err_lines.push(PathElement::new(
                            vec![(center - cap_half_w, lo), (center + cap_half_w, lo)],
                            err_style,
                        ));
                    }
                }
            }

            if is_stacked(bar_mode) {
                for (label, &val) in labels.iter().zip(values.iter()) {
                    if let Some(cat_idx) = categories.iter().position(|c| c == label) {
                        stack_bases[cat_idx] += val;
                    }
                }
            }

            group_idx += 1;
        }
    }

    if !err_lines.is_empty() {
        chart.draw_series(err_lines).unwrap();
    }

    if has_legend {
        let (w, h) = resolve_dimensions(layout);
        apply_legend_config(&mut chart, root, &config, w, h, 15, 50, 40, &legend_entries);
    }

    draw_axis_titles(root, &config, w, h, 15, 50, 40);
}

/// Compute the (category center, value base, value top) of a bar for the given mode.
/// In vertical orientation `center` is x and `base/top` are y; in horizontal it
/// flips. For stacked modes the base/top incorporate the running stack accumulator.
fn bar_geometry(
    bar_mode: &BarMode,
    cat_idx: usize,
    group_idx: usize,
    n_groups: usize,
    bar_width: f64,
    stack_base: f64,
    val: f64,
) -> (f64, f64, f64) {
    match bar_mode {
        BarMode::Group => {
            let offset = (group_idx as f64 - (n_groups as f64 - 1.0) / 2.0) * bar_width;
            (cat_idx as f64 + offset, 0.0, val)
        }
        BarMode::Stack | BarMode::Relative => (cat_idx as f64, stack_base, stack_base + val),
        BarMode::Overlay => (cat_idx as f64, 0.0, val),
    }
}

pub(super) fn render_bar_horizontal<DB: DrawingBackend>(
    root: &DrawingArea<DB, plotters::coord::Shift>,
    layout: &LayoutIR,
    traces: &[TraceIR],
    unsupported: &mut Vec<String>,
) {
    let config = extract_layout_config(layout, unsupported);
    let bar_mode = resolve_bar_mode(layout);
    let categories = collect_bar_categories(traces);
    let n_cats = categories.len();
    let n_groups = count_bar_groups(traces);
    let (_, max_val) = compute_bar_ranges(traces, is_stacked(bar_mode));

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

        let xvc = axis_value_color(config.x_axis.as_ref());
        let yvc = axis_value_color(config.y_axis.as_ref());
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
    let bar_width = match bar_mode {
        BarMode::Group => {
            if n_groups > 0 {
                bar_total_width / n_groups as f64
            } else {
                bar_total_width
            }
        }
        _ => bar_total_width,
    };

    let mut has_legend = false;
    let mut legend_entries: Vec<LegendEntry> = Vec::new();
    let mut group_idx = 0usize;
    let mut stack_bases = vec![0.0f64; n_cats];
    let mut err_lines: Vec<PathElement<(f64, f64)>> = Vec::new();
    let cap_half_h = bar_width * 0.15;
    let err_style = ShapeStyle {
        color: BLACK.to_rgba(),
        filled: false,
        stroke_width: 1,
    };

    for trace in traces {
        if let TraceIR::BarPlot(ir) = trace {
            let labels = extract_strings(&ir.labels);
            let values = extract_f64(&ir.values);
            let errors = ir.error.as_ref().map(extract_f64);
            let color = resolve_trace_color(&ir.marker, group_idx);
            let alpha = if matches!(bar_mode, BarMode::Overlay) {
                0.6
            } else {
                1.0
            };
            let style = ShapeStyle {
                color: color.to_rgba().mix(alpha),
                filled: true,
                stroke_width: 0,
            };

            let rects: Vec<_> = labels
                .iter()
                .zip(values.iter())
                .filter_map(|(label, &val)| {
                    let cat_idx = categories.iter().position(|c| c == label)?;
                    let (center, base, top) = bar_geometry(
                        bar_mode,
                        cat_idx,
                        group_idx,
                        n_groups,
                        bar_width,
                        stack_bases[cat_idx],
                        val,
                    );
                    let y0 = center - bar_width / 2.0;
                    let y1 = center + bar_width / 2.0;
                    Some(Rectangle::new([(base, y0), (top, y1)], style))
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
                    opacity: alpha,
                    kind: SwatchKind::Rect,
                });
            }

            if let Some(ref errors) = errors {
                for ((label, &val), &err) in labels.iter().zip(values.iter()).zip(errors.iter()) {
                    if let Some(cat_idx) = categories.iter().position(|c| c == label) {
                        let (center, _, top) = bar_geometry(
                            bar_mode,
                            cat_idx,
                            group_idx,
                            n_groups,
                            bar_width,
                            stack_bases[cat_idx],
                            val,
                        );
                        let lo = top - err;
                        let hi = top + err;
                        err_lines.push(PathElement::new(
                            vec![(lo, center), (hi, center)],
                            err_style,
                        ));
                        err_lines.push(PathElement::new(
                            vec![(hi, center - cap_half_h), (hi, center + cap_half_h)],
                            err_style,
                        ));
                        err_lines.push(PathElement::new(
                            vec![(lo, center - cap_half_h), (lo, center + cap_half_h)],
                            err_style,
                        ));
                    }
                }
            }

            if is_stacked(bar_mode) {
                for (label, &val) in labels.iter().zip(values.iter()) {
                    if let Some(cat_idx) = categories.iter().position(|c| c == label) {
                        stack_bases[cat_idx] += val;
                    }
                }
            }

            group_idx += 1;
        }
    }

    if !err_lines.is_empty() {
        chart.draw_series(err_lines).unwrap();
    }

    let (w, h) = resolve_dimensions(layout);
    draw_axis_titles(root, &config, w, h, 15, 70, 40);

    if has_legend {
        apply_legend_config(&mut chart, root, &config, w, h, 15, 70, 40, &legend_entries);
    }
}
