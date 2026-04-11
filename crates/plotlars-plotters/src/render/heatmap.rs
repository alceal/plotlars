use plotlars_core::components::colorbar::ColorBar;
use plotlars_core::components::Palette;
use plotlars_core::ir::layout::LayoutIR;
use plotlars_core::ir::trace::TraceIR;
use plotters::prelude::*;
use plotters::style::text_anchor::{HPos, Pos, VPos};

use crate::converters::components::convert_rgb;
use crate::converters::layout::{extract_layout_config, format_thousands};

use super::axis::{configure_label_areas, format_exponent};
use super::resolve_dimensions;
use super::title::{draw_axis_titles, draw_plot_title, title_top_margin};

use crate::converters::trace::{extract_f64, extract_strings};

/// Color stops for palette interpolation. Each entry is (t, r, g, b).
fn palette_stops(palette: &Palette) -> Vec<(f64, u8, u8, u8)> {
    match palette {
        Palette::Viridis => vec![
            (0.0, 68, 1, 84),
            (0.25, 59, 82, 139),
            (0.5, 33, 145, 140),
            (0.75, 94, 201, 98),
            (1.0, 253, 231, 37),
        ],
        Palette::Hot => vec![
            (0.0, 0, 0, 0),
            (0.33, 230, 0, 0),
            (0.66, 255, 210, 0),
            (1.0, 255, 255, 255),
        ],
        Palette::Blues => vec![
            (0.0, 247, 251, 255),
            (0.5, 107, 174, 214),
            (1.0, 8, 48, 107),
        ],
        Palette::Reds => vec![(0.0, 255, 245, 240), (0.5, 251, 106, 74), (1.0, 103, 0, 13)],
        Palette::Greens => vec![(0.0, 247, 252, 245), (0.5, 116, 196, 118), (1.0, 0, 68, 27)],
        Palette::Greys => vec![(0.0, 255, 255, 255), (1.0, 0, 0, 0)],
        Palette::YlGnBu => vec![
            (0.0, 255, 255, 217),
            (0.33, 127, 205, 187),
            (0.66, 44, 127, 184),
            (1.0, 8, 29, 88),
        ],
        Palette::YlOrRd => vec![
            (0.0, 255, 255, 178),
            (0.33, 254, 178, 76),
            (0.66, 240, 59, 32),
            (1.0, 128, 0, 38),
        ],
        Palette::RdBu => vec![
            (0.0, 178, 24, 43),
            (0.5, 247, 247, 247),
            (1.0, 33, 102, 172),
        ],
        Palette::Bluered => vec![(0.0, 0, 0, 255), (1.0, 255, 0, 0)],
        Palette::Picnic => vec![(0.0, 0, 0, 255), (0.5, 255, 255, 255), (1.0, 255, 0, 0)],
        Palette::Rainbow => vec![
            (0.0, 150, 0, 90),
            (0.25, 0, 0, 200),
            (0.5, 0, 200, 0),
            (0.75, 200, 200, 0),
            (1.0, 200, 0, 0),
        ],
        Palette::Portland => vec![
            (0.0, 12, 51, 131),
            (0.25, 10, 136, 186),
            (0.5, 242, 211, 56),
            (0.75, 242, 143, 56),
            (1.0, 217, 30, 30),
        ],
        Palette::Jet => vec![
            (0.0, 0, 0, 131),
            (0.125, 0, 0, 255),
            (0.375, 0, 255, 255),
            (0.625, 255, 255, 0),
            (0.875, 255, 0, 0),
            (1.0, 128, 0, 0),
        ],
        Palette::Blackbody => vec![
            (0.0, 0, 0, 0),
            (0.33, 230, 0, 0),
            (0.66, 255, 200, 0),
            (1.0, 255, 255, 255),
        ],
        Palette::Earth => vec![
            (0.0, 0, 0, 130),
            (0.33, 0, 180, 0),
            (0.66, 200, 200, 100),
            (1.0, 255, 255, 255),
        ],
        Palette::Electric => vec![
            (0.0, 0, 0, 0),
            (0.25, 30, 0, 100),
            (0.5, 120, 0, 100),
            (0.75, 160, 90, 0),
            (1.0, 230, 200, 0),
        ],
        Palette::Cividis => vec![
            (0.0, 0, 32, 76),
            (0.25, 60, 77, 110),
            (0.5, 127, 127, 127),
            (0.75, 186, 175, 104),
            (1.0, 253, 231, 56),
        ],
    }
}

fn interpolate_color(stops: &[(f64, u8, u8, u8)], t: f64) -> RGBColor {
    let t = t.clamp(0.0, 1.0);
    if stops.is_empty() {
        return RGBColor(0, 0, 0);
    }
    if t <= stops[0].0 {
        return RGBColor(stops[0].1, stops[0].2, stops[0].3);
    }
    let last = stops[stops.len() - 1];
    if t >= last.0 {
        return RGBColor(last.1, last.2, last.3);
    }
    for i in 0..stops.len() - 1 {
        if t >= stops[i].0 && t <= stops[i + 1].0 {
            let range = stops[i + 1].0 - stops[i].0;
            let local_t = if range > 0.0 {
                (t - stops[i].0) / range
            } else {
                0.0
            };
            let r = (stops[i].1 as f64 * (1.0 - local_t) + stops[i + 1].1 as f64 * local_t) as u8;
            let g = (stops[i].2 as f64 * (1.0 - local_t) + stops[i + 1].2 as f64 * local_t) as u8;
            let b = (stops[i].3 as f64 * (1.0 - local_t) + stops[i + 1].3 as f64 * local_t) as u8;
            return RGBColor(r, g, b);
        }
    }
    RGBColor(0, 0, 0)
}

pub(super) fn render_heatmap<DB: DrawingBackend>(
    root: &DrawingArea<DB, plotters::coord::Shift>,
    layout: &LayoutIR,
    traces: &[TraceIR],
    unsupported: &mut Vec<String>,
) {
    let config = extract_layout_config(layout, unsupported);

    let ir = match &traces[0] {
        TraceIR::HeatMap(ir) => ir,
        _ => return,
    };

    let x_labels = extract_strings(&ir.x);
    let y_labels = extract_strings(&ir.y);
    let z_values = extract_f64(&ir.z);

    // Collect unique labels preserving order
    let mut x_cats: Vec<String> = Vec::new();
    for l in &x_labels {
        if !x_cats.contains(l) {
            x_cats.push(l.clone());
        }
    }
    let mut y_cats: Vec<String> = Vec::new();
    for l in &y_labels {
        if !y_cats.contains(l) {
            y_cats.push(l.clone());
        }
    }
    let n_x = x_cats.len();
    let n_y = y_cats.len();

    // Build z grid (row=y, col=x)
    let mut grid = vec![vec![f64::NAN; n_x]; n_y];
    for ((xl, yl), &z) in x_labels.iter().zip(y_labels.iter()).zip(z_values.iter()) {
        if let (Some(xi), Some(yi)) = (
            x_cats.iter().position(|c| c == xl),
            y_cats.iter().position(|c| c == yl),
        ) {
            grid[yi][xi] = z;
        }
    }

    // Compute z range
    let mut z_min = ir.z_min.unwrap_or(f64::INFINITY);
    let mut z_max = ir.z_max.unwrap_or(f64::NEG_INFINITY);
    if ir.z_min.is_none() || ir.z_max.is_none() {
        for row in &grid {
            for &v in row {
                if v.is_finite() {
                    if ir.z_min.is_none() {
                        z_min = z_min.min(v);
                    }
                    if ir.z_max.is_none() {
                        z_max = z_max.max(v);
                    }
                }
            }
        }
    }
    if z_min == z_max {
        z_min -= 0.5;
        z_max += 0.5;
    }
    let z_range = z_max - z_min;

    let palette = ir.color_scale.unwrap_or(Palette::Viridis);
    let reverse = ir.reverse_scale.unwrap_or(false);
    let show_scale = ir.show_scale.unwrap_or(true);
    let stops = palette_stops(&palette);

    let right_margin = if show_scale { 80 } else { 15 };

    let (w, h) = resolve_dimensions(layout);
    draw_plot_title(root, &config, w, h);

    let mut builder = ChartBuilder::on(root);
    builder
        .margin_top(15 + title_top_margin(&config))
        .margin_bottom(15)
        .margin_left(15)
        .margin_right(right_margin);
    configure_label_areas(&mut builder, &config, 40, 50);

    let mut chart = builder
        .build_cartesian_2d(-0.5..(n_x as f64 - 0.5), -0.5..(n_y as f64 - 0.5))
        .unwrap();

    // Configure mesh with category labels
    {
        let x_c = x_cats.clone();
        let y_c = y_cats.clone();
        let x_fmt = move |v: &f64| -> String {
            let idx = v.round() as usize;
            x_c.get(idx).cloned().unwrap_or_default()
        };
        let y_fmt = move |v: &f64| -> String {
            let idx = v.round() as usize;
            y_c.get(idx).cloned().unwrap_or_default()
        };
        let mut mesh = chart.configure_mesh();
        mesh.disable_mesh();
        mesh.x_labels(n_x);
        mesh.y_labels(n_y);
        mesh.x_label_formatter(&x_fmt);
        mesh.y_label_formatter(&y_fmt);
        mesh.draw().unwrap();
    }

    // Draw cells
    for (yi, row) in grid.iter().enumerate() {
        for (xi, &val) in row.iter().enumerate() {
            if !val.is_finite() {
                continue;
            }
            let mut t = (val - z_min) / z_range;
            if reverse {
                t = 1.0 - t;
            }
            let color = interpolate_color(&stops, t);
            chart
                .draw_series(std::iter::once(Rectangle::new(
                    [
                        (xi as f64 - 0.5, yi as f64 - 0.5),
                        (xi as f64 + 0.5, yi as f64 + 0.5),
                    ],
                    color.filled(),
                )))
                .unwrap();
        }
    }

    draw_axis_titles(root, &config, w, h, 15, 50, 40);

    // Draw color bar
    if show_scale {
        let cb = ir.color_bar.as_ref();
        draw_color_bar(
            root,
            &stops,
            reverse,
            z_min,
            z_max,
            z_range,
            w,
            right_margin,
            &config,
            cb,
        );
    }

    if ir.auto_color_scale.is_some() {
        plotlars_core::policy::report_unsupported(
            "plotters",
            "HeatMap",
            "auto_color_scale",
            unsupported,
        );
    }
}

#[allow(clippy::too_many_arguments)]
fn draw_color_bar<DB: DrawingBackend>(
    root: &DrawingArea<DB, plotters::coord::Shift>,
    stops: &[(f64, u8, u8, u8)],
    reverse: bool,
    z_min: f64,
    z_max: f64,
    z_range: f64,
    w: u32,
    right_margin: i32,
    config: &crate::converters::layout::LayoutConfig,
    cb: Option<&ColorBar>,
) {
    let plot_h = root.dim_in_pixel().1 as i32;
    let top_offset = (15 + title_top_margin(config)) as i32 + 5;
    let available_h = plot_h - top_offset - 40;

    // Length as fraction of available height (default 1.0)
    let length_frac = cb.and_then(|c| c.length).unwrap_or(1.0).clamp(0.0, 1.0);
    let bar_height = (available_h as f64 * length_frac) as i32;
    let bar_top = top_offset + (available_h - bar_height) / 2;

    // Width
    let bar_width = cb
        .and_then(|c| c.width)
        .map(|w| (w * 100.0) as i32)
        .unwrap_or(20);
    let bar_x = w as i32 - right_margin + 10;

    // Draw gradient
    let n_steps = bar_height.max(1) as usize;
    for i in 0..n_steps {
        let mut t = 1.0 - (i as f64 / n_steps as f64);
        if reverse {
            t = 1.0 - t;
        }
        let color = interpolate_color(stops, t);
        let y = bar_top + i as i32;
        root.draw(&Rectangle::new(
            [(bar_x, y), (bar_x + bar_width, y + 1)],
            color.filled(),
        ))
        .unwrap();
    }

    // Border / outline
    let outline_color = cb
        .and_then(|c| c.outline_color.as_ref())
        .map(convert_rgb)
        .unwrap_or(BLACK);
    let outline_width = cb.and_then(|c| c.outline_width).unwrap_or(1) as u32;
    root.draw(&Rectangle::new(
        [(bar_x, bar_top), (bar_x + bar_width, bar_top + bar_height)],
        ShapeStyle {
            color: outline_color.to_rgba(),
            filled: false,
            stroke_width: outline_width,
        },
    ))
    .unwrap();

    // Tick / label configuration
    let tick_font = cb
        .and_then(|c| c.tick_font.as_deref())
        .unwrap_or("sans-serif");
    let tick_color = cb
        .and_then(|c| c.tick_color.as_ref())
        .map(convert_rgb)
        .unwrap_or(BLACK);
    let tick_len = cb.and_then(|c| c.tick_length).unwrap_or(5) as i32;
    let tick_width = cb.and_then(|c| c.tick_width).unwrap_or(1) as u32;
    let separate_thousands = cb.and_then(|c| c.separate_thousands).unwrap_or(false);
    let value_exponent = cb.and_then(|c| c.value_exponent.as_ref());
    let tick_labels_custom = cb.and_then(|c| c.tick_labels.as_ref());

    let label_style = TextStyle::from((tick_font, 12).into_font())
        .color(&tick_color)
        .pos(Pos::new(HPos::Left, VPos::Center));
    let tick_style = ShapeStyle {
        color: tick_color.to_rgba(),
        filled: false,
        stroke_width: tick_width,
    };

    // Compute tick values
    let tick_vals: Vec<f64> = if let Some(custom_vals) = cb.and_then(|c| c.tick_values.as_ref()) {
        custom_vals.clone()
    } else if let Some(step) = cb.and_then(|c| c.tick_step) {
        let mut vals = Vec::new();
        let start = (z_min / step).ceil() * step;
        let mut v = start;
        while v <= z_max + step * 0.001 {
            vals.push(v);
            v += step;
        }
        vals
    } else {
        let n = cb.and_then(|c| c.n_ticks).unwrap_or(5);
        (0..=n)
            .map(|i| z_min + (i as f64 / n as f64) * z_range)
            .collect()
    };

    // Draw ticks and labels
    for (i, &val) in tick_vals.iter().enumerate() {
        let t = if z_range > 0.0 {
            (val - z_min) / z_range
        } else {
            0.5
        };
        let y = bar_top + ((1.0 - t) * bar_height as f64) as i32;

        // Tick mark
        root.draw(&PathElement::new(
            vec![(bar_x + bar_width, y), (bar_x + bar_width + tick_len, y)],
            tick_style,
        ))
        .unwrap();

        // Label
        let label = if let Some(custom_labels) = tick_labels_custom {
            custom_labels
                .get(i)
                .cloned()
                .unwrap_or_else(|| format_cb_value(val, separate_thousands, value_exponent))
        } else {
            format_cb_value(val, separate_thousands, value_exponent)
        };
        root.draw_text(&label, &label_style, (bar_x + bar_width + tick_len + 3, y))
            .unwrap();
    }

    // Title
    if let Some(title) = cb.and_then(|c| c.title.as_ref()) {
        let title_style = TextStyle::from((tick_font, 13).into_font())
            .color(&BLACK)
            .pos(Pos::new(HPos::Center, VPos::Bottom));
        let tx = bar_x + bar_width / 2;
        let ty = bar_top - 5;
        root.draw_text(&title.content, &title_style, (tx, ty))
            .unwrap();
    }
}

fn format_cb_value(
    val: f64,
    separate_thousands: bool,
    value_exponent: Option<&plotlars_core::components::ValueExponent>,
) -> String {
    use plotlars_core::components::ValueExponent;

    let use_exp = value_exponent.is_some_and(|e| !matches!(e, ValueExponent::None));

    if use_exp {
        format_exponent(val, value_exponent.unwrap())
    } else if separate_thousands {
        format_thousands(val)
    } else if val.abs() >= 10.0 || val == 0.0 {
        format!("{val:.0}")
    } else {
        format!("{val:.1}")
    }
}
