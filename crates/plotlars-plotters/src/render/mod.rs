use std::process::Command;

use plotlars_core::ir::trace::TraceIR;
use plotlars_core::policy::enforce_strict;
use plotlars_core::Plot;
use plotters::prelude::*;

use crate::converters::components::BaseShape;
use crate::converters::trace::is_horizontal_bar;

mod axis;
mod bar;
mod legend;
mod numeric;
mod title;

const DEFAULT_WIDTH: u32 = 800;
const DEFAULT_HEIGHT: u32 = 600;

#[derive(Clone, Copy)]
pub(super) enum SwatchKind {
    Line(u32),
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
                bar::render_bar_horizontal(&root, layout, traces, &mut unsupported);
            } else {
                bar::render_bar_vertical(&root, layout, traces, &mut unsupported);
            }
        }
        _ => numeric::render_numeric(&root, layout, traces, &mut unsupported),
    }

    enforce_strict("plotters", &unsupported);
    root.present().unwrap();
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
