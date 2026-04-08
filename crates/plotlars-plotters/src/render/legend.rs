use plotlars_core::components::Orientation;
use plotters::coord::CoordTranslate;
use plotters::prelude::*;
use plotters::style::text_anchor::{HPos, Pos, VPos};

use crate::converters::components::{convert_rgb, BaseShape, FillMode};
use crate::converters::layout::LayoutConfig;

use super::{polygon_vertices_at_origin, LegendEntry, SwatchKind};

fn estimate_text_width(text: &str, font_size: u32) -> u32 {
    (text.len() as f64 * font_size as f64 * 0.52).ceil() as u32
}

#[allow(clippy::too_many_arguments)]
pub(super) fn apply_legend_config<'a, DB, CT>(
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
