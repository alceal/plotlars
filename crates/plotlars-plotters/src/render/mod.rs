use std::process::Command;

use plotlars_core::ir::trace::TraceIR;
use plotlars_core::policy::enforce_strict;
use plotlars_core::Plot;
use plotters::prelude::*;

use crate::converters::components::BaseShape;
use crate::converters::trace::is_horizontal_bar;

mod axis;
mod bar;
mod boxplot;
mod heatmap;
mod legend;
mod numeric;
mod title;

const DEFAULT_WIDTH: u32 = 800;
const DEFAULT_HEIGHT: u32 = 600;

#[derive(Clone, Copy)]
pub(super) enum SwatchKind {
    Line(u32),
    LineShape(u32, BaseShape, crate::converters::components::FillMode),
    Rect,
    Shape(BaseShape, crate::converters::components::FillMode),
}

#[derive(Clone)]
pub(super) struct LegendEntry {
    pub(super) name: String,
    pub(super) color: RGBColor,
    pub(super) opacity: f64,
    pub(super) kind: SwatchKind,
}

pub(super) fn resolve_dimensions(layout: &plotlars_core::ir::layout::LayoutIR) -> (u32, u32) {
    match &layout.dimensions {
        Some(dims) => {
            let w = dims.width.unwrap_or(DEFAULT_WIDTH as usize) as u32;
            let h = dims.height.unwrap_or(DEFAULT_HEIGHT as usize) as u32;
            (w, h)
        }
        None => (DEFAULT_WIDTH, DEFAULT_HEIGHT),
    }
}

pub(super) fn polygon_vertices_at_origin(base_shape: BaseShape, r: i32) -> Vec<(i32, i32)> {
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

fn render_to_backend<DB: DrawingBackend>(
    plot: &impl Plot,
    root: DrawingArea<DB, plotters::coord::Shift>,
) -> Vec<numeric::DashEntry> {
    let layout = plot.ir_layout();
    let traces = plot.ir_traces();

    root.fill(&WHITE).unwrap();

    let mut unsupported = Vec::new();
    let mut dash_entries = Vec::new();

    if traces.is_empty() {
        root.present().unwrap();
        return dash_entries;
    }

    match &traces[0] {
        TraceIR::BarPlot(_) => {
            if is_horizontal_bar(traces) {
                bar::render_bar_horizontal(&root, layout, traces, &mut unsupported);
            } else {
                bar::render_bar_vertical(&root, layout, traces, &mut unsupported);
            }
        }
        TraceIR::BoxPlot(_) => {
            boxplot::render_boxplot(&root, layout, traces, &mut unsupported);
        }
        TraceIR::HeatMap(_) => {
            heatmap::render_heatmap(&root, layout, traces, &mut unsupported);
        }
        _ => numeric::render_numeric(&root, layout, traces, &mut unsupported, &mut dash_entries),
    }

    enforce_strict("plotters", &unsupported);
    root.present().unwrap();
    dash_entries
}

pub fn plot_interactive(plot: &impl Plot) {
    if std::env::var("EVCXR_IS_RUNTIME").is_ok() {
        let svg = render_to_svg_string(plot);
        println!(
            "EVCXR_BEGIN_CONTENT image/svg+xml\n{}\nEVCXR_END_CONTENT",
            svg
        );
        return;
    }

    let tmp = std::env::temp_dir().join("plotlars_tmp.svg");
    let path = tmp.to_str().unwrap();
    save_to_file(plot, path);
    open_file(path);
}

pub fn save_to_file(plot: &impl Plot, path: &str) {
    let (w, h) = resolve_dimensions(plot.ir_layout());

    if path.ends_with(".svg") {
        let mut svg = String::new();
        let dashes;
        {
            let root = SVGBackend::with_string(&mut svg, (w, h)).into_drawing_area();
            dashes = render_to_backend(plot, root);
        }
        smooth_svg_lines(&mut svg);
        apply_svg_dash_patterns(&mut svg, &dashes);
        std::fs::write(path, svg).unwrap();
    } else {
        let root = BitMapBackend::new(path, (w, h)).into_drawing_area();
        render_to_backend(plot, root);
    }
}

pub fn render_to_svg_string(plot: &impl Plot) -> String {
    let (w, h) = resolve_dimensions(plot.ir_layout());
    let mut svg_string = String::new();
    let dashes;
    {
        let root = SVGBackend::with_string(&mut svg_string, (w, h)).into_drawing_area();
        dashes = render_to_backend(plot, root);
    }
    smooth_svg_lines(&mut svg_string);
    apply_svg_dash_patterns(&mut svg_string, &dashes);
    svg_string
}

/// Apply `stroke-dasharray` only to `<polyline>` elements whose stroke color
/// matches a dash entry. `<line>`, `<rect>`, and other elements are left untouched
/// so axis lines, grid lines, and tick marks are never dashed (they often share
/// the default `#000000` stroke with data series).
///
/// Walks each polyline tag once and collects in-place `replace_range` edits.
/// Edits are applied in reverse byte order so earlier offsets stay valid as the
/// string grows.
fn apply_svg_dash_patterns(svg: &mut String, dashes: &[numeric::DashEntry]) {
    if dashes.is_empty() {
        return;
    }

    let mut edits: Vec<(usize, usize, String)> = Vec::new();
    let mut search_from = 0usize;
    while let Some(rel_start) = svg[search_from..].find("<polyline ") {
        let tag_start = search_from + rel_start;
        let tag_end = svg[tag_start..]
            .find("/>")
            .map(|i| tag_start + i + 2)
            .unwrap_or(svg.len());
        let tag = &svg[tag_start..tag_end];

        if !tag.contains("stroke-dasharray") {
            for &(color, pattern) in dashes {
                let hex = format!("#{:02X}{:02X}{:02X}", color.0, color.1, color.2);
                let needle = format!("stroke=\"{hex}\"");
                if let Some(off) = tag.find(&needle) {
                    let edit_start = tag_start + off;
                    let replacement = format!("stroke=\"{hex}\" stroke-dasharray=\"{pattern}\"");
                    edits.push((edit_start, needle.len(), replacement));
                    break;
                }
            }
        }
        search_from = tag_end;
    }

    for (start, len, replacement) in edits.into_iter().rev() {
        svg.replace_range(start..start + len, &replacement);
    }
}

/// Post-process SVG to improve line rendering quality:
/// 1. Inject round joins/caps so thick strokes blend smoothly at vertices.
/// 2. Simplify dense polylines (Ramer-Douglas-Peucker) to remove sub-pixel
///    zigzag noise caused by plotters' integer coordinate rounding.
///
/// No-op for plots without polylines (bar/box/heatmap).
fn smooth_svg_lines(svg: &mut String) {
    if !svg.contains("<polyline ") {
        return;
    }
    *svg = svg.replace(
        "<polyline fill=\"none\"",
        "<polyline stroke-linejoin=\"round\" stroke-linecap=\"round\" fill=\"none\"",
    );
    simplify_svg_polylines(svg);
}

/// Find every `points="..."` attribute in the SVG and, when the polyline has
/// enough vertices, apply Ramer-Douglas-Peucker to remove redundant ones.
fn simplify_svg_polylines(svg: &mut String) {
    // MIN_POINTS: skip simplification for short polylines (axis ticks, error-bar
    // caps, scatter markers); RDP overhead outweighs savings below this size.
    // EPSILON: 0.5 px matches plotters' integer-pixel rounding granularity, so
    // collinear-after-rounding vertices collapse without altering visible shape.
    const MIN_POINTS: usize = 20;
    const EPSILON: f64 = 0.5;

    let mut result = String::with_capacity(svg.len());
    let mut remaining = svg.as_str();
    let mut modified = false;

    while let Some(idx) = remaining.find("points=\"") {
        let prefix_end = idx + 8; // length of `points="`
        result.push_str(&remaining[..prefix_end]);
        remaining = &remaining[prefix_end..];

        if let Some(end) = remaining.find('"') {
            let raw = &remaining[..end];
            // Cheap pre-check: count separators before parsing. A polyline with
            // <MIN_POINTS vertices has <MIN_POINTS spaces between coord pairs.
            let space_count = raw.bytes().filter(|&b| b == b' ').count();
            if space_count >= MIN_POINTS {
                let points = parse_svg_points(raw);
                if points.len() > MIN_POINTS {
                    let simplified = rdp_simplify(&points, EPSILON);
                    result.push_str(&format_svg_points(&simplified));
                    modified = true;
                } else {
                    result.push_str(raw);
                }
            } else {
                result.push_str(raw);
            }
            result.push('"');
            remaining = &remaining[end + 1..];
        }
    }
    if modified {
        result.push_str(remaining);
        *svg = result;
    }
}

fn parse_svg_points(s: &str) -> Vec<(f64, f64)> {
    s.split_whitespace()
        .filter_map(|p| {
            let (x, y) = p.split_once(',')?;
            Some((x.parse().ok()?, y.parse().ok()?))
        })
        .collect()
}

fn format_svg_points(pts: &[(f64, f64)]) -> String {
    pts.iter()
        .map(|(x, y)| format!("{},{}", *x as i32, *y as i32))
        .collect::<Vec<_>>()
        .join(" ")
}

/// Ramer-Douglas-Peucker polyline simplification.
fn rdp_simplify(points: &[(f64, f64)], epsilon: f64) -> Vec<(f64, f64)> {
    if points.len() <= 2 {
        return points.to_vec();
    }
    let first = points[0];
    let last = points[points.len() - 1];

    let mut max_dist = 0.0f64;
    let mut max_idx = 0;
    for (i, &p) in points.iter().enumerate().skip(1).take(points.len() - 2) {
        let d = perp_distance(p, first, last);
        if d > max_dist {
            max_dist = d;
            max_idx = i;
        }
    }

    if max_dist > epsilon {
        let mut left = rdp_simplify(&points[..=max_idx], epsilon);
        let right = rdp_simplify(&points[max_idx..], epsilon);
        left.pop();
        left.extend(right);
        left
    } else {
        vec![first, last]
    }
}

fn perp_distance((px, py): (f64, f64), (ax, ay): (f64, f64), (bx, by): (f64, f64)) -> f64 {
    let dx = bx - ax;
    let dy = by - ay;
    let len_sq = dx * dx + dy * dy;
    if len_sq == 0.0 {
        return ((px - ax).powi(2) + (py - ay).powi(2)).sqrt();
    }
    (dy * px - dx * py + bx * ay - by * ax).abs() / len_sq.sqrt()
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
