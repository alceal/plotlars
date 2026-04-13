use std::f64::consts::PI;

use plotlars_core::components::{Rgb, Shape};
use plotters::style::RGBColor;

pub(crate) const DEFAULT_COLORS: [RGBColor; 10] = [
    RGBColor(99, 110, 250),  // #636efa
    RGBColor(239, 85, 59),   // #EF553B
    RGBColor(0, 204, 150),   // #00cc96
    RGBColor(171, 99, 250),  // #ab63fa
    RGBColor(255, 161, 90),  // #FFA15A
    RGBColor(25, 211, 243),  // #19d3f3
    RGBColor(255, 102, 146), // #FF6692
    RGBColor(182, 232, 128), // #B6E880
    RGBColor(255, 151, 255), // #FF97FF
    RGBColor(254, 203, 82),  // #FECB52
];

pub(crate) fn convert_rgb(rgb: &Rgb) -> RGBColor {
    RGBColor(rgb.0, rgb.1, rgb.2)
}

pub(crate) fn default_color(index: usize) -> RGBColor {
    DEFAULT_COLORS[index % DEFAULT_COLORS.len()]
}

pub(crate) fn resolve_trace_color(
    marker: &Option<plotlars_core::ir::marker::MarkerIR>,
    trace_idx: usize,
) -> RGBColor {
    marker
        .as_ref()
        .and_then(|m| m.color.as_ref())
        .map(convert_rgb)
        .unwrap_or_else(|| default_color(trace_idx))
}

// ── Shape support ──────────────────────────────────────────────────

#[derive(Clone, Copy)]
pub(crate) enum BaseShape {
    Circle,
    Square,
    Diamond,
    TriangleUp,
    TriangleDown,
    TriangleLeft,
    TriangleRight,
    Cross,
    X,
    Pentagon,
    Hexagon,
    Octagon,
    Star,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum FillMode {
    Filled,
    Open,
}

pub(crate) fn resolve_shape(shape: &Shape) -> (BaseShape, FillMode) {
    use Shape::*;

    match shape {
        // Circle
        Circle | CircleDot | CircleCross | CircleCrossOpen | CircleX | CircleXOpen => {
            (BaseShape::Circle, FillMode::Filled)
        }
        CircleOpen | CircleOpenDot => (BaseShape::Circle, FillMode::Open),

        // Square
        Square | SquareDot | SquareCross | SquareCrossOpen | SquareX | SquareXOpen => {
            (BaseShape::Square, FillMode::Filled)
        }
        SquareOpen | SquareOpenDot => (BaseShape::Square, FillMode::Open),

        // Diamond
        Diamond | DiamondDot | DiamondTall | DiamondTallDot | DiamondWide | DiamondWideDot
        | DiamondCross | DiamondCrossOpen | DiamondX | DiamondXOpen => {
            (BaseShape::Diamond, FillMode::Filled)
        }
        DiamondOpen | DiamondOpenDot | DiamondTallOpen | DiamondTallOpenDot | DiamondWideOpen
        | DiamondWideOpenDot => (BaseShape::Diamond, FillMode::Open),

        // Cross (+)
        Cross | CrossDot | CrossThin | CrossThinOpen | CrossOpen | CrossOpenDot => {
            (BaseShape::Cross, FillMode::Filled)
        }

        // X
        X | XDot | XThin | XThinOpen => (BaseShape::X, FillMode::Filled),
        XOpen | XOpenDot => (BaseShape::X, FillMode::Open),

        // Triangles
        TriangleUp | TriangleUpDot => (BaseShape::TriangleUp, FillMode::Filled),
        TriangleUpOpen | TriangleUpOpenDot => (BaseShape::TriangleUp, FillMode::Open),
        TriangleDown | TriangleDownDot => (BaseShape::TriangleDown, FillMode::Filled),
        TriangleDownOpen | TriangleDownOpenDot => (BaseShape::TriangleDown, FillMode::Open),
        TriangleLeft | TriangleLeftDot => (BaseShape::TriangleLeft, FillMode::Filled),
        TriangleLeftOpen | TriangleLeftOpenDot => (BaseShape::TriangleLeft, FillMode::Open),
        TriangleRight | TriangleRightDot => (BaseShape::TriangleRight, FillMode::Filled),
        TriangleRightOpen | TriangleRightOpenDot => (BaseShape::TriangleRight, FillMode::Open),
        TriangleNE | TriangleNEDot => (BaseShape::TriangleUp, FillMode::Filled),
        TriangleNEOpen | TriangleNEOpenDot => (BaseShape::TriangleUp, FillMode::Open),
        TriangleSE | TriangleSEDot => (BaseShape::TriangleDown, FillMode::Filled),
        TriangleSEOpen | TriangleSEOpenDot => (BaseShape::TriangleDown, FillMode::Open),
        TriangleSW | TriangleSWDot => (BaseShape::TriangleDown, FillMode::Filled),
        TriangleSWOpen | TriangleSWOpenDot => (BaseShape::TriangleDown, FillMode::Open),
        TriangleNW | TriangleNWDot => (BaseShape::TriangleUp, FillMode::Filled),
        TriangleNWOpen | TriangleNWOpenDot => (BaseShape::TriangleUp, FillMode::Open),

        // Pentagon, Hexagon, Octagon
        Pentagon | PentagonDot => (BaseShape::Pentagon, FillMode::Filled),
        PentagonOpen | PentagonOpenDot => (BaseShape::Pentagon, FillMode::Open),
        Hexagon | HexagonDot | Hexagon2 | Hexagon2Dot => (BaseShape::Hexagon, FillMode::Filled),
        HexagonOpen | HexagonOpenDot | Hexagon2Open | Hexagon2OpenDot => {
            (BaseShape::Hexagon, FillMode::Open)
        }
        Octagon | OctagonDot => (BaseShape::Octagon, FillMode::Filled),
        OctagonOpen | OctagonOpenDot => (BaseShape::Octagon, FillMode::Open),

        // Stars
        Star | StarDot | StarTriangleUp | StarTriangleUpDot | StarTriangleDown
        | StarTriangleDownDot | StarSquare | StarSquareDot | StarDiamond | StarDiamondDot
        | Hexagram | HexagramDot => (BaseShape::Star, FillMode::Filled),
        StarOpen
        | StarOpenDot
        | StarTriangleUpOpen
        | StarTriangleUpOpenDot
        | StarTriangleDownOpen
        | StarTriangleDownOpenDot
        | StarSquareOpen
        | StarSquareOpenDot
        | StarDiamondOpen
        | StarDiamondOpenDot
        | HexagramOpen
        | HexagramOpenDot => (BaseShape::Star, FillMode::Open),

        // Misc -> closest match
        Hourglass | HourglassOpen | BowTie | BowTieOpen => (BaseShape::Diamond, FillMode::Filled),
        Asterisk | AsteriskOpen => (BaseShape::Star, FillMode::Filled),
        Hash | HashDot | HashOpen | HashOpenDot => (BaseShape::Cross, FillMode::Filled),
        YUp | YUpOpen | YDown | YDownOpen | YLeft | YLeftOpen | YRight | YRightOpen => {
            (BaseShape::Cross, FillMode::Filled)
        }
        LineEW | LineEWOpen | LineNS | LineNSOpen | LineNE | LineNEOpen | LineNW | LineNWOpen => {
            (BaseShape::Cross, FillMode::Filled)
        }
    }
}

pub(crate) fn resolve_trace_shape(
    marker: &Option<plotlars_core::ir::marker::MarkerIR>,
) -> (BaseShape, FillMode) {
    marker
        .as_ref()
        .and_then(|m| m.shape.as_ref())
        .map(resolve_shape)
        .unwrap_or((BaseShape::Circle, FillMode::Filled))
}

// ── Vertex computation ─────────────────────────────────────────────

// Vertex functions operate in backend pixel coordinates (i32).
// The render pipeline converts chart (f64, f64) to pixel (i32, i32) via
// chart.backend_coord() before calling these.

pub(crate) fn square_vertices(cx: i32, cy: i32, r: i32) -> Vec<(i32, i32)> {
    vec![
        (cx - r, cy - r),
        (cx + r, cy - r),
        (cx + r, cy + r),
        (cx - r, cy + r),
    ]
}

pub(crate) fn diamond_vertices(cx: i32, cy: i32, r: i32) -> Vec<(i32, i32)> {
    vec![(cx, cy - r), (cx + r, cy), (cx, cy + r), (cx - r, cy)]
}

pub(crate) fn triangle_down_vertices(cx: i32, cy: i32, r: i32) -> Vec<(i32, i32)> {
    vec![(cx, cy + r), (cx + r, cy - r), (cx - r, cy - r)]
}

pub(crate) fn triangle_left_vertices(cx: i32, cy: i32, r: i32) -> Vec<(i32, i32)> {
    vec![(cx - r, cy), (cx + r, cy - r), (cx + r, cy + r)]
}

pub(crate) fn triangle_right_vertices(cx: i32, cy: i32, r: i32) -> Vec<(i32, i32)> {
    vec![(cx + r, cy), (cx - r, cy - r), (cx - r, cy + r)]
}

pub(crate) fn regular_polygon_vertices(cx: i32, cy: i32, r: i32, n: usize) -> Vec<(i32, i32)> {
    let offset = -PI / 2.0;
    (0..n)
        .map(|i| {
            let angle = offset + 2.0 * PI * (i as f64) / (n as f64);
            let x = cx as f64 + r as f64 * angle.cos();
            let y = cy as f64 + r as f64 * angle.sin();
            (x.round() as i32, y.round() as i32)
        })
        .collect()
}

pub(crate) fn star_vertices(cx: i32, cy: i32, r: i32) -> Vec<(i32, i32)> {
    let n = 5;
    let inner_r = (r as f64 * 0.38).round() as i32;
    let offset = -PI / 2.0;
    (0..n * 2)
        .map(|i| {
            let angle = offset + PI * (i as f64) / (n as f64);
            let radius = if i % 2 == 0 { r } else { inner_r };
            let x = cx as f64 + radius as f64 * angle.cos();
            let y = cy as f64 + radius as f64 * angle.sin();
            (x.round() as i32, y.round() as i32)
        })
        .collect()
}
