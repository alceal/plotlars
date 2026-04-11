use plotlars_core::components::Orientation;
use plotlars_core::ir::layout::LayoutIR;
use plotlars_core::ir::trace::TraceIR;
use plotters::prelude::*;

use crate::converters::components::resolve_trace_color;
use crate::converters::layout::extract_layout_config;
use crate::converters::trace::{extract_f64, extract_strings};

use super::axis::configure_label_areas;
use super::legend::apply_legend_config;
use super::title::{draw_axis_titles, draw_plot_title, title_top_margin};
use super::{resolve_dimensions, LegendEntry, SwatchKind};

struct BoxStats {
    q1: f64,
    median: f64,
    q3: f64,
    whisker_lo: f64,
    whisker_hi: f64,
    outliers: Vec<f64>,
}

fn compute_box_stats(values: &mut [f64]) -> BoxStats {
    values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let q1 = percentile(values, 25.0);
    let median = percentile(values, 50.0);
    let q3 = percentile(values, 75.0);
    let iqr = q3 - q1;
    let lo_fence = q1 - 1.5 * iqr;
    let hi_fence = q3 + 1.5 * iqr;
    let whisker_lo = values
        .iter()
        .copied()
        .filter(|&v| v >= lo_fence)
        .fold(f64::INFINITY, f64::min);
    let whisker_hi = values
        .iter()
        .copied()
        .filter(|&v| v <= hi_fence)
        .fold(f64::NEG_INFINITY, f64::max);
    let outliers: Vec<f64> = values
        .iter()
        .copied()
        .filter(|&v| v < lo_fence || v > hi_fence)
        .collect();
    BoxStats {
        q1,
        median,
        q3,
        whisker_lo,
        whisker_hi,
        outliers,
    }
}

fn percentile(sorted: &[f64], p: f64) -> f64 {
    if sorted.is_empty() {
        return 0.0;
    }
    let idx = (p / 100.0) * (sorted.len() as f64 - 1.0);
    let lo = idx.floor() as usize;
    let hi = (lo + 1).min(sorted.len() - 1);
    let frac = idx - lo as f64;
    sorted[lo] * (1.0 - frac) + sorted[hi] * frac
}

/// Detect if any trace requests horizontal orientation.
fn is_horizontal(traces: &[TraceIR]) -> bool {
    traces.iter().any(|t| {
        if let TraceIR::BoxPlot(ir) = t {
            matches!(ir.orientation, Some(Orientation::Horizontal))
        } else {
            false
        }
    })
}

pub(super) fn render_boxplot<DB: DrawingBackend>(
    root: &DrawingArea<DB, plotters::coord::Shift>,
    layout: &LayoutIR,
    traces: &[TraceIR],
    unsupported: &mut Vec<String>,
) {
    let config = extract_layout_config(layout, unsupported);
    let horiz = is_horizontal(traces);

    // Collect unique category labels across all traces
    let mut categories: Vec<String> = Vec::new();
    for trace in traces {
        if let TraceIR::BoxPlot(ir) = trace {
            for label in extract_strings(&ir.labels) {
                if !categories.contains(&label) {
                    categories.push(label);
                }
            }
        }
    }
    let n_cats = categories.len();
    let n_groups = traces
        .iter()
        .filter(|t| matches!(t, TraceIR::BoxPlot(_)))
        .count();

    // Compute value range from all box data
    let mut v_min = f64::INFINITY;
    let mut v_max = f64::NEG_INFINITY;
    for trace in traces {
        if let TraceIR::BoxPlot(ir) = trace {
            for v in extract_f64(&ir.values) {
                v_min = v_min.min(v);
                v_max = v_max.max(v);
            }
        }
    }
    let v_margin = (v_max - v_min).abs() * 0.05;
    v_min -= v_margin.max(0.01);
    v_max += v_margin.max(0.01);

    // Apply user-specified ranges
    if horiz {
        if let Some((lo, hi)) = config.x_range {
            v_min = lo;
            v_max = hi;
        }
    } else if let Some((lo, hi)) = config.y_range {
        v_min = lo;
        v_max = hi;
    }

    let cat_range = -0.5..(n_cats as f64 - 0.5);

    let (w, h) = resolve_dimensions(layout);
    draw_plot_title(root, &config, w, h);

    let mut builder = ChartBuilder::on(root);
    builder
        .margin_top(15 + title_top_margin(&config))
        .margin_bottom(15)
        .margin_left(15)
        .margin_right(15);
    configure_label_areas(&mut builder, &config, 40, 50);

    if horiz {
        let mut chart = builder.build_cartesian_2d(v_min..v_max, cat_range).unwrap();

        // Mesh with category labels on y-axis
        {
            let cats = categories.clone();
            let cat_formatter = move |v: &f64| -> String {
                let idx = v.round() as usize;
                cats.get(idx).cloned().unwrap_or_default()
            };
            let mut mesh = chart.configure_mesh();
            mesh.y_labels(n_cats);
            mesh.y_label_formatter(&cat_formatter);
            mesh.draw().unwrap();
        }

        let mut has_legend = false;
        let mut legend_entries: Vec<LegendEntry> = Vec::new();
        // slot_width: total space per group (for offset/spacing)
        // box_width: visual box width (narrower, so points fit between groups)
        let (slot_width, box_width) = if n_groups > 1 {
            let slot = 0.8 / n_groups as f64;
            (slot, slot * 0.5)
        } else {
            (0.5, 0.5)
        };

        for (trace_idx, trace) in traces.iter().enumerate() {
            if let TraceIR::BoxPlot(ir) = trace {
                draw_boxes_horizontal(
                    &mut chart,
                    root,
                    ir,
                    trace_idx,
                    &categories,
                    n_cats,
                    n_groups,
                    slot_width,
                    box_width,
                    &mut has_legend,
                    &mut legend_entries,
                );
            }
        }

        draw_axis_titles(root, &config, w, h, 15, 50, 40);
        if has_legend {
            apply_legend_config(&mut chart, root, &config, w, h, 15, 50, 40, &legend_entries);
        }
    } else {
        let mut chart = builder.build_cartesian_2d(cat_range, v_min..v_max).unwrap();

        // Mesh with category labels on x-axis
        {
            let cats = categories.clone();
            let cat_formatter = move |v: &f64| -> String {
                let idx = v.round() as usize;
                cats.get(idx).cloned().unwrap_or_default()
            };
            let mut mesh = chart.configure_mesh();
            mesh.x_labels(n_cats);
            mesh.x_label_formatter(&cat_formatter);
            mesh.draw().unwrap();
        }

        let mut has_legend = false;
        let mut legend_entries: Vec<LegendEntry> = Vec::new();
        // slot_width: total space per group (for offset/spacing)
        // box_width: visual box width (narrower, so points fit between groups)
        let (slot_width, box_width) = if n_groups > 1 {
            let slot = 0.8 / n_groups as f64;
            (slot, slot * 0.5)
        } else {
            (0.5, 0.5)
        };

        for (trace_idx, trace) in traces.iter().enumerate() {
            if let TraceIR::BoxPlot(ir) = trace {
                draw_boxes_vertical(
                    &mut chart,
                    root,
                    ir,
                    trace_idx,
                    &categories,
                    n_cats,
                    n_groups,
                    slot_width,
                    box_width,
                    &mut has_legend,
                    &mut legend_entries,
                );
            }
        }

        draw_axis_titles(root, &config, w, h, 15, 50, 40);
        if has_legend {
            apply_legend_config(&mut chart, root, &config, w, h, 15, 50, 40, &legend_entries);
        }
    }
}

/// Draw vertical boxes: categories on x-axis, values on y-axis.
#[allow(clippy::too_many_arguments)]
fn draw_boxes_vertical<DB: DrawingBackend>(
    chart: &mut ChartContext<
        DB,
        Cartesian2d<plotters::coord::types::RangedCoordf64, plotters::coord::types::RangedCoordf64>,
    >,
    _root: &DrawingArea<DB, plotters::coord::Shift>,
    ir: &plotlars_core::ir::trace::BoxPlotIR,
    trace_idx: usize,
    categories: &[String],
    n_cats: usize,
    n_groups: usize,
    slot_width: f64,
    box_width: f64,
    has_legend: &mut bool,
    legend_entries: &mut Vec<LegendEntry>,
) {
    let color = resolve_trace_color(&ir.marker, trace_idx);
    let opacity = ir.marker.as_ref().and_then(|m| m.opacity).unwrap_or(1.0);
    let fill_style = color.mix(opacity).filled();
    let line_style = ShapeStyle {
        color: color.mix(opacity),
        filled: false,
        stroke_width: 2,
    };

    let labels = extract_strings(&ir.labels);
    let values = extract_f64(&ir.values);

    let mut cat_values: Vec<Vec<f64>> = vec![Vec::new(); n_cats];
    for (label, val) in labels.iter().zip(values.iter()) {
        if let Some(cat_idx) = categories.iter().position(|c| c == label) {
            cat_values[cat_idx].push(*val);
        }
    }

    let group_offset = if n_groups > 1 {
        (trace_idx as f64 - (n_groups as f64 - 1.0) / 2.0) * slot_width
    } else {
        0.0
    };

    for (cat_idx, mut vals) in cat_values.into_iter().enumerate() {
        if vals.len() < 2 {
            continue;
        }
        let stats = compute_box_stats(&mut vals);
        let c = cat_idx as f64 + group_offset;
        let hw = box_width / 2.0 * 0.9;

        // Box filled
        chart
            .draw_series(std::iter::once(Rectangle::new(
                [(c - hw, stats.q1), (c + hw, stats.q3)],
                fill_style,
            )))
            .unwrap();
        // Box border
        chart
            .draw_series(std::iter::once(PathElement::new(
                vec![
                    (c - hw, stats.q1),
                    (c + hw, stats.q1),
                    (c + hw, stats.q3),
                    (c - hw, stats.q3),
                    (c - hw, stats.q1),
                ],
                line_style,
            )))
            .unwrap();
        // Median
        chart
            .draw_series(std::iter::once(PathElement::new(
                vec![(c - hw, stats.median), (c + hw, stats.median)],
                ShapeStyle {
                    color: BLACK.to_rgba(),
                    filled: false,
                    stroke_width: 2,
                },
            )))
            .unwrap();
        // Whiskers
        chart
            .draw_series(std::iter::once(PathElement::new(
                vec![(c, stats.whisker_lo), (c, stats.q1)],
                line_style,
            )))
            .unwrap();
        chart
            .draw_series(std::iter::once(PathElement::new(
                vec![
                    (c - hw * 0.5, stats.whisker_lo),
                    (c + hw * 0.5, stats.whisker_lo),
                ],
                line_style,
            )))
            .unwrap();
        chart
            .draw_series(std::iter::once(PathElement::new(
                vec![(c, stats.q3), (c, stats.whisker_hi)],
                line_style,
            )))
            .unwrap();
        chart
            .draw_series(std::iter::once(PathElement::new(
                vec![
                    (c - hw * 0.5, stats.whisker_hi),
                    (c + hw * 0.5, stats.whisker_hi),
                ],
                line_style,
            )))
            .unwrap();
        // Outliers
        if !stats.outliers.is_empty() {
            let os = ShapeStyle {
                color: color.mix(opacity),
                filled: true,
                stroke_width: 1,
            };
            chart
                .draw_series(stats.outliers.iter().map(|&v| Circle::new((c, v), 3, os)))
                .unwrap();
        }

        // Box points: draw all individual data points
        if ir.box_points.unwrap_or(false) {
            let slot_hw = slot_width / 2.0;
            let offset = ir.point_offset.unwrap_or(0.0) / 2.0 * slot_hw;
            let jitter_amt = ir.jitter.unwrap_or(0.0) * hw;
            let pt_style = ShapeStyle {
                color: color.mix(opacity * 0.6),
                filled: true,
                stroke_width: 1,
            };
            chart
                .draw_series(vals.iter().enumerate().map(|(i, &v)| {
                    let jit = pseudo_jitter(i, jitter_amt);
                    Circle::new((c + offset + jit, v), 2, pt_style)
                }))
                .unwrap();
        }
    }

    add_legend_entry(ir, has_legend, legend_entries, color, opacity);
}

/// Draw horizontal boxes: categories on y-axis, values on x-axis.
#[allow(clippy::too_many_arguments)]
fn draw_boxes_horizontal<DB: DrawingBackend>(
    chart: &mut ChartContext<
        DB,
        Cartesian2d<plotters::coord::types::RangedCoordf64, plotters::coord::types::RangedCoordf64>,
    >,
    _root: &DrawingArea<DB, plotters::coord::Shift>,
    ir: &plotlars_core::ir::trace::BoxPlotIR,
    trace_idx: usize,
    categories: &[String],
    n_cats: usize,
    n_groups: usize,
    slot_width: f64,
    box_width: f64,
    has_legend: &mut bool,
    legend_entries: &mut Vec<LegendEntry>,
) {
    let color = resolve_trace_color(&ir.marker, trace_idx);
    let opacity = ir.marker.as_ref().and_then(|m| m.opacity).unwrap_or(1.0);
    let fill_style = color.mix(opacity).filled();
    let line_style = ShapeStyle {
        color: color.mix(opacity),
        filled: false,
        stroke_width: 2,
    };

    let labels = extract_strings(&ir.labels);
    let values = extract_f64(&ir.values);

    let mut cat_values: Vec<Vec<f64>> = vec![Vec::new(); n_cats];
    for (label, val) in labels.iter().zip(values.iter()) {
        if let Some(cat_idx) = categories.iter().position(|c| c == label) {
            cat_values[cat_idx].push(*val);
        }
    }

    let group_offset = if n_groups > 1 {
        (trace_idx as f64 - (n_groups as f64 - 1.0) / 2.0) * slot_width
    } else {
        0.0
    };

    for (cat_idx, mut vals) in cat_values.into_iter().enumerate() {
        if vals.len() < 2 {
            continue;
        }
        let stats = compute_box_stats(&mut vals);
        let c = cat_idx as f64 + group_offset;
        let hw = box_width / 2.0 * 0.9;

        // Box filled (x=values, y=category)
        chart
            .draw_series(std::iter::once(Rectangle::new(
                [(stats.q1, c - hw), (stats.q3, c + hw)],
                fill_style,
            )))
            .unwrap();
        // Box border
        chart
            .draw_series(std::iter::once(PathElement::new(
                vec![
                    (stats.q1, c - hw),
                    (stats.q1, c + hw),
                    (stats.q3, c + hw),
                    (stats.q3, c - hw),
                    (stats.q1, c - hw),
                ],
                line_style,
            )))
            .unwrap();
        // Median
        chart
            .draw_series(std::iter::once(PathElement::new(
                vec![(stats.median, c - hw), (stats.median, c + hw)],
                ShapeStyle {
                    color: BLACK.to_rgba(),
                    filled: false,
                    stroke_width: 2,
                },
            )))
            .unwrap();
        // Whiskers (horizontal: along x-axis)
        chart
            .draw_series(std::iter::once(PathElement::new(
                vec![(stats.whisker_lo, c), (stats.q1, c)],
                line_style,
            )))
            .unwrap();
        chart
            .draw_series(std::iter::once(PathElement::new(
                vec![
                    (stats.whisker_lo, c - hw * 0.5),
                    (stats.whisker_lo, c + hw * 0.5),
                ],
                line_style,
            )))
            .unwrap();
        chart
            .draw_series(std::iter::once(PathElement::new(
                vec![(stats.q3, c), (stats.whisker_hi, c)],
                line_style,
            )))
            .unwrap();
        chart
            .draw_series(std::iter::once(PathElement::new(
                vec![
                    (stats.whisker_hi, c - hw * 0.5),
                    (stats.whisker_hi, c + hw * 0.5),
                ],
                line_style,
            )))
            .unwrap();
        // Outliers
        if !stats.outliers.is_empty() {
            let os = ShapeStyle {
                color: color.mix(opacity),
                filled: true,
                stroke_width: 1,
            };
            chart
                .draw_series(stats.outliers.iter().map(|&v| Circle::new((v, c), 3, os)))
                .unwrap();
        }

        // Box points: draw all individual data points
        if ir.box_points.unwrap_or(false) {
            let slot_hw = slot_width / 2.0;
            let offset = ir.point_offset.unwrap_or(0.0) / 2.0 * slot_hw;
            let jitter_amt = ir.jitter.unwrap_or(0.0) * hw;
            let pt_style = ShapeStyle {
                color: color.mix(opacity * 0.6),
                filled: true,
                stroke_width: 1,
            };
            chart
                .draw_series(vals.iter().enumerate().map(|(i, &v)| {
                    let jit = pseudo_jitter(i, jitter_amt);
                    Circle::new((v, c + offset + jit), 2, pt_style)
                }))
                .unwrap();
        }
    }

    add_legend_entry(ir, has_legend, legend_entries, color, opacity);
}

fn add_legend_entry(
    ir: &plotlars_core::ir::trace::BoxPlotIR,
    has_legend: &mut bool,
    legend_entries: &mut Vec<LegendEntry>,
    color: RGBColor,
    opacity: f64,
) {
    if let Some(ref name) = ir.name {
        *has_legend = true;
        legend_entries.push(LegendEntry {
            name: name.clone(),
            color,
            opacity,
            kind: SwatchKind::Rect,
        });
    }
}

/// Simple deterministic pseudo-random jitter from value index.
fn pseudo_jitter(index: usize, jitter_amount: f64) -> f64 {
    let hash = (index as u32).wrapping_mul(2654435761) as f64 / u32::MAX as f64;
    (hash - 0.5) * 2.0 * jitter_amount
}
