use plotlars_core::components::{Axis, Legend, TickDirection};
use plotlars_core::ir::layout::LayoutIR;
use plotlars_core::policy::report_unsupported;

pub(crate) struct LayoutConfig {
    pub title: Option<String>,
    pub title_font_size: u32,
    pub x_label: Option<String>,
    pub y_label: Option<String>,
    pub x_axis: Option<Axis>,
    pub y_axis: Option<Axis>,
    pub legend: Option<Legend>,
    pub legend_title: Option<String>,
    pub x_range: Option<(f64, f64)>,
    pub y_range: Option<(f64, f64)>,
}

pub(crate) fn extract_layout_config(
    layout: &LayoutIR,
    unsupported: &mut Vec<String>,
) -> LayoutConfig {
    let title = layout.title.as_ref().map(|t| t.content.clone());
    let title_font_size = layout.title.as_ref().map(|t| t.size).unwrap_or(20) as u32;

    let x_label = layout.x_title.as_ref().map(|t| t.content.clone());
    let y_label = layout.y_title.as_ref().map(|t| t.content.clone());

    let (x_axis, y_axis) = match &layout.axes_2d {
        Some(axes) => (axes.x_axis.clone(), axes.y_axis.clone()),
        None => (None, None),
    };

    let legend = layout.legend.clone();
    let legend_title = layout.legend_title.as_ref().map(|t| t.content.clone());

    let x_range = x_axis
        .as_ref()
        .and_then(|a| a.value_range.as_ref())
        .and_then(|r| {
            if r.len() >= 2 {
                Some((r[0], r[1]))
            } else {
                None
            }
        });

    let y_range = y_axis
        .as_ref()
        .and_then(|a| a.value_range.as_ref())
        .and_then(|r| {
            if r.len() >= 2 {
                Some((r[0], r[1]))
            } else {
                None
            }
        });

    // Report unsupported layout fields
    if layout.y2_title.is_some() {
        report_unsupported("plotters", "Layout", "y2_title", unsupported);
    }
    if layout.z_title.is_some() {
        report_unsupported("plotters", "Layout", "z_title", unsupported);
    }
    if layout.scene_3d.is_some() {
        report_unsupported("plotters", "Layout", "scene_3d", unsupported);
    }
    if layout.polar.is_some() {
        report_unsupported("plotters", "Layout", "polar", unsupported);
    }
    if layout.mapbox.is_some() {
        report_unsupported("plotters", "Layout", "mapbox", unsupported);
    }
    if !layout.annotations.is_empty() {
        report_unsupported("plotters", "Layout", "annotations", unsupported);
    }


    // Report unsupported axis fields
    for (axis_opt, name) in [(&x_axis, "x_axis"), (&y_axis, "y_axis")] {
        if let Some(axis) = axis_opt {
            report_unsupported_axis_fields(axis, name, unsupported);
        }
    }

    LayoutConfig {
        title,
        title_font_size,
        x_label,
        y_label,
        x_axis,
        y_axis,
        legend,
        legend_title,
        x_range,
        y_range,
    }
}

fn report_unsupported_axis_fields(axis: &Axis, name: &str, unsupported: &mut Vec<String>) {
    if axis.axis_side.is_some() {
        report_unsupported("plotters", name, "axis_side", unsupported);
    }
    if axis.axis_position.is_some() {
        report_unsupported("plotters", name, "axis_position", unsupported);
    }
    if axis.axis_type.is_some() {
        report_unsupported("plotters", name, "axis_type", unsupported);
    }
    if axis.tick_angle.is_some() {
        report_unsupported("plotters", name, "tick_angle", unsupported);
    }
    if axis.tick_width.is_some() {
        report_unsupported("plotters", name, "tick_width", unsupported);
    }
    if axis.tick_color.is_some() {
        report_unsupported("plotters", name, "tick_color", unsupported);
    }
    if axis.value_color.is_some() {
        report_unsupported("plotters", name, "value_color", unsupported);
    }
    if axis.tick_values.is_some() {
        report_unsupported("plotters", name, "tick_values", unsupported);
    }
    if axis.value_exponent.is_some() {
        report_unsupported("plotters", name, "value_exponent", unsupported);
    }
    if axis.show_zero_line.is_some() {
        report_unsupported("plotters", name, "show_zero_line", unsupported);
    }
    if axis.zero_line_color.is_some() {
        report_unsupported("plotters", name, "zero_line_color", unsupported);
    }
    if axis.zero_line_width.is_some() {
        report_unsupported("plotters", name, "zero_line_width", unsupported);
    }
}

/// Resolve tick mark size for an axis, combining tick_length and tick_direction.
/// Positive = outward, negative = inward, 0 = hidden.
pub(crate) fn resolve_tick_size(axis: &Axis) -> Option<i32> {
    let length = axis.tick_length.unwrap_or(5) as i32;
    match axis.tick_direction {
        Some(TickDirection::InSide) => Some(-length),
        Some(TickDirection::OutSide) => Some(length),
        Some(TickDirection::None) => Some(0),
        None => {
            if axis.tick_length.is_some() {
                Some(length)
            } else {
                None
            }
        }
    }
}

/// Format a number with thousands separators (e.g., 1234567 -> "1,234,567").
pub(crate) fn format_thousands(val: f64) -> String {
    let is_negative = val < 0.0;
    let abs_val = val.abs();

    if abs_val.fract() == 0.0 {
        let int_part = abs_val as u64;
        let s = int_part.to_string();
        let formatted = add_thousands_sep(&s);
        if is_negative {
            format!("-{formatted}")
        } else {
            formatted
        }
    } else {
        let s = format!("{abs_val:.2}");
        let parts: Vec<&str> = s.split('.').collect();
        let formatted = add_thousands_sep(parts[0]);
        if is_negative {
            format!("-{formatted}.{}", parts[1])
        } else {
            format!("{formatted}.{}", parts[1])
        }
    }
}

fn add_thousands_sep(s: &str) -> String {
    let bytes = s.as_bytes();
    let len = bytes.len();
    let mut result = String::with_capacity(len + len / 3);
    for (i, &b) in bytes.iter().enumerate() {
        if i > 0 && (len - i) % 3 == 0 {
            result.push(',');
        }
        result.push(b as char);
    }
    result
}
