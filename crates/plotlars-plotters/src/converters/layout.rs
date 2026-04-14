use plotlars_core::components::axis::AxisType;
use plotlars_core::components::{Axis, Legend, Rgb, Text};
use plotlars_core::ir::layout::LayoutIR;
use plotlars_core::policy::report_unsupported;

/// Default font for axis labels when the user doesn't set one.
const DEFAULT_LABEL_FONT: &str = "sans-serif";

/// Default rendered size (px) for axis-title labels when the user leaves
/// `Text::size` at its default of 12. We promote 12 → 15 because plotlars'
/// `Text` default is too small for axis titles in plotters' raster output.
///
/// This conflates "user explicitly set 12" with "default" — the proper fix is
/// `Text::size: Option<usize>` upstream. Until then, an explicit `.size(12)`
/// will render at 15.
const DEFAULT_AXIS_LABEL_SIZE: u32 = 15;

/// Resolve a `Text` font, falling back to `sans-serif` for both unset titles
/// and titles with an empty `font` field.
fn label_font(text: Option<&Text>) -> String {
    text.map(|t| {
        if t.font.is_empty() {
            DEFAULT_LABEL_FONT.to_string()
        } else {
            t.font.clone()
        }
    })
    .unwrap_or_else(|| DEFAULT_LABEL_FONT.to_string())
}

/// Resolve an axis-title font size, applying the 12 → `DEFAULT_AXIS_LABEL_SIZE`
/// promotion described on `DEFAULT_AXIS_LABEL_SIZE`.
fn label_size(text: Option<&Text>) -> u32 {
    text.map(|t| {
        if t.size == 12 {
            DEFAULT_AXIS_LABEL_SIZE
        } else {
            t.size as u32
        }
    })
    .unwrap_or(DEFAULT_AXIS_LABEL_SIZE)
}

/// Resolve a `Text` color, defaulting to black.
fn label_color(text: Option<&Text>) -> Rgb {
    text.map(|t| t.color).unwrap_or(Rgb(0, 0, 0))
}

/// Return the user-set x position only when it differs from the `Text` default
/// of 0.5 (so callers know whether to apply manual positioning).
fn label_x(text: Option<&Text>) -> Option<f64> {
    text.and_then(|t| {
        if (t.x - 0.5).abs() < f64::EPSILON {
            None
        } else {
            Some(t.x)
        }
    })
}

/// Return the user-set y position only when it differs from the `Text` default
/// of 0.9.
fn label_y(text: Option<&Text>) -> Option<f64> {
    text.and_then(|t| {
        if (t.y - 0.9).abs() < f64::EPSILON {
            None
        } else {
            Some(t.y)
        }
    })
}

pub(crate) struct LayoutConfig {
    pub title: Option<String>,
    pub title_font_size: u32,
    pub title_font: String,
    pub title_color: Option<Rgb>,
    pub title_x: Option<f64>,
    pub title_y: Option<f64>,
    pub x_label: Option<String>,
    pub x_label_font: String,
    pub x_label_size: u32,
    pub x_label_color: Rgb,
    pub x_label_x: Option<f64>,
    pub x_label_y: Option<f64>,
    pub y_label: Option<String>,
    pub y_label_font: String,
    pub y_label_size: u32,
    pub y_label_color: Rgb,
    pub y_label_x: Option<f64>,
    pub y_label_y: Option<f64>,
    pub x_axis: Option<Axis>,
    pub y_axis: Option<Axis>,
    pub y2_axis: Option<Axis>,
    pub y2_label: Option<String>,
    pub y2_label_font: String,
    pub y2_label_size: u32,
    pub y2_label_color: Rgb,
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
    // Plot title default size is 20, not the 12→15 axis-title promotion.
    let title_font_size = layout.title.as_ref().map(|t| t.size).unwrap_or(20) as u32;
    let title_font = label_font(layout.title.as_ref());
    let title_color = layout.title.as_ref().map(|t| t.color);
    let title_x = label_x(layout.title.as_ref());
    let title_y = label_y(layout.title.as_ref());

    let x_label = layout.x_title.as_ref().map(|t| t.content.clone());
    let x_label_font = label_font(layout.x_title.as_ref());
    let x_label_size = label_size(layout.x_title.as_ref());
    let x_label_color = label_color(layout.x_title.as_ref());
    let x_label_x = label_x(layout.x_title.as_ref());
    let x_label_y = label_y(layout.x_title.as_ref());

    let y_label = layout.y_title.as_ref().map(|t| t.content.clone());
    let y_label_font = label_font(layout.y_title.as_ref());
    let y_label_size = label_size(layout.y_title.as_ref());
    let y_label_color = label_color(layout.y_title.as_ref());
    let y_label_x = label_x(layout.y_title.as_ref());
    let y_label_y = label_y(layout.y_title.as_ref());

    let (x_axis, y_axis, y2_axis) = match &layout.axes_2d {
        Some(axes) => (
            axes.x_axis.clone(),
            axes.y_axis.clone(),
            axes.y2_axis.clone(),
        ),
        None => (None, None, None),
    };

    let y2_label = layout.y2_title.as_ref().map(|t| t.content.clone());
    let y2_label_font = label_font(layout.y2_title.as_ref());
    let y2_label_size = label_size(layout.y2_title.as_ref());
    let y2_label_color = label_color(layout.y2_title.as_ref());

    let legend = layout.legend.clone();
    let legend_title = layout.legend_title.as_ref().map(|t| t.content.clone());

    let x_range = x_axis
        .as_ref()
        .and_then(|a| a.value_range.as_ref())
        .map(|r| (r[0], r[1]));

    let y_range = y_axis
        .as_ref()
        .and_then(|a| a.value_range.as_ref())
        .map(|r| (r[0], r[1]));

    // Report unsupported layout fields
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
        title_font,
        title_color,
        title_x,
        title_y,
        x_label,
        x_label_font,
        x_label_size,
        x_label_color,
        x_label_x,
        x_label_y,
        y_label,
        y_label_font,
        y_label_size,
        y_label_color,
        y_label_x,
        y_label_y,
        x_axis,
        y_axis,
        y2_axis,
        y2_label,
        y2_label_font,
        y2_label_size,
        y2_label_color,
        legend,
        legend_title,
        x_range,
        y_range,
    }
}

fn report_unsupported_axis_fields(axis: &Axis, name: &str, unsupported: &mut Vec<String>) {
    if axis.axis_position.is_some() {
        report_unsupported("plotters", name, "axis_position", unsupported);
    }
    if axis
        .axis_type
        .as_ref()
        .is_some_and(|t| matches!(t, AxisType::MultiCategory))
    {
        report_unsupported("plotters", name, "axis_type", unsupported);
    }
    if axis.tick_direction.is_some() {
        report_unsupported("plotters", name, "tick_direction", unsupported);
    }
    if axis.tick_width.is_some() {
        report_unsupported("plotters", name, "tick_width", unsupported);
    }
    if axis.tick_angle.is_some() {
        report_unsupported("plotters", name, "tick_angle", unsupported);
    }
    if axis.tick_color.is_some() {
        report_unsupported("plotters", name, "tick_color", unsupported);
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
    if axis.tick_length.is_some() {
        Some(length)
    } else {
        None
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
