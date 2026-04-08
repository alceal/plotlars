use plotlars_core::components::axis::AxisType;
use plotlars_core::components::Axis;
use plotlars_core::ir::trace::TraceIR;
use plotters::chart::MeshStyle;
use plotters::coord::ranged1d::ValueFormatter;
use plotters::prelude::*;

use crate::converters::components::convert_rgb;
use crate::converters::layout::{resolve_tick_size, LayoutConfig};
use crate::converters::trace::{extract_f64, extract_strings};

pub(super) fn configure_label_areas(
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

// ── Log-axis helpers ────────────────────────────────────────────────────

pub(super) fn is_log_axis(axis: &Option<Axis>) -> bool {
    axis.as_ref()
        .and_then(|a| a.axis_type.as_ref())
        .is_some_and(|t| matches!(t, AxisType::Log))
}

/// Transform a range to log10 space, clamping the lower bound to a positive value.
pub(super) fn log_range(min: f64, max: f64) -> (f64, f64) {
    let lo = if min > 0.0 { min } else { max * 1e-6 };
    (lo.log10(), max.log10())
}

/// Transform a slice of (x, y) pairs to log10 space on the requested axes,
/// filtering out non-positive values on log axes.
pub(super) fn log_transform_points(
    points: &[(f64, f64)],
    x_log: bool,
    y_log: bool,
) -> Vec<(f64, f64)> {
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
pub(super) fn format_log_label(v: &f64) -> String {
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
pub(super) fn format_exponent(v: f64, exp: &plotlars_core::components::ValueExponent) -> String {
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

pub(super) fn is_category_axis(axis: &Option<Axis>) -> bool {
    axis.as_ref()
        .and_then(|a| a.axis_type.as_ref())
        .is_some_and(|t| matches!(t, AxisType::Category | AxisType::Date))
}

pub(super) fn is_date_axis(axis: &Option<Axis>) -> bool {
    axis.as_ref()
        .and_then(|a| a.axis_type.as_ref())
        .is_some_and(|t| matches!(t, AxisType::Date))
}

/// Collect unique x-string labels from scatter/line traces for category/date axes.
pub(super) fn collect_string_x_labels(traces: &[TraceIR]) -> Vec<String> {
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
pub(super) fn category_xy_pairs(
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

// ── Mesh configuration shared by all chart types ───────────────────────

pub(super) fn apply_mesh_axis_config<'a, 'b, X, Y, DB>(
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
