use plotters::prelude::*;
use plotters::style::text_anchor::{HPos, Pos, VPos};

use crate::converters::components::convert_rgb;
use crate::converters::layout::LayoutConfig;

pub(super) fn draw_plot_title<DB: DrawingBackend>(
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

pub(super) fn title_top_margin(config: &LayoutConfig) -> u32 {
    if config.title.is_some() {
        config.title_font_size + 10
    } else {
        0
    }
}

#[allow(clippy::too_many_arguments)]
pub(super) fn draw_axis_titles<DB: DrawingBackend>(
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

    // Y2-axis title (rotated, right side)
    if let Some(ref label) = config.y2_label {
        let color = convert_rgb(&config.y2_label_color);
        let size = config.y2_label_size as f64;
        let style = TextStyle::from(
            (config.y2_label_font.as_str(), size)
                .into_font()
                .transform(FontTransform::Rotate90),
        )
        .color(&color)
        .pos(Pos::new(HPos::Center, VPos::Center));
        let cx = width as i32 - config.y2_label_size as i32 / 2 - 2;
        let cy = (top_margin + height - chart_margin - x_label_area) as i32 / 2;
        root.draw_text(label, &style, (cx, cy)).unwrap();
    }
}
