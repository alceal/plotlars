#![allow(dead_code)]

use plotly::color::Rgb as RgbPlotly;
use plotly::common::{
    Anchor, AxisSide as AxisSidePlotly, ColorBar as ColorBarPlotly, ColorScale, ColorScalePalette,
    DashType, Direction as DirectionPlotly, ExponentFormat, Fill as PlotlyFill, Font,
    Line as LinePlotly, MarkerSymbol, Mode as PlotlyMode, Orientation as OrientationPlotly, Ticks,
    Title,
};
use plotly::contour::Coloring as ColoringPlotly;
use plotly::layout::{
    Annotation, Axis as AxisPlotly, AxisType as AxisTypePlotly, BarMode as BarModePlotly,
    Legend as LegendPlotly, TicksDirection,
};
use plotly::sankey::Arrangement as ArrangementPlotly;
use plotly::traces::mesh3d::IntensityMode as PlotlyIntensityMode;
use plotly::traces::table::{
    Align, Cells as CellsPlotly, Fill as TableFill, Font as TableFont, Header as HeaderPlotly,
};

use plotlars_core::components::axis::{Axis, AxisSide, AxisType};
use plotlars_core::components::cell::Cell;
use plotlars_core::components::colorbar::ColorBar;
use plotlars_core::components::direction::Direction;
use plotlars_core::components::header::Header;
use plotlars_core::components::legend::Legend;
use plotlars_core::components::lighting::Lighting;
use plotlars_core::components::text::Text;
use plotlars_core::components::tick::TickDirection;
use plotlars_core::components::Rgb;
use plotlars_core::components::{
    Arrangement, BarMode, Coloring, Fill, IntensityMode, Line, Mode, Orientation, Palette, Shape,
    ValueExponent,
};

//── Leaf component conversions (Tier 1) ──────────────────────────────────

pub(crate) fn convert_rgb(rgb: &Rgb) -> RgbPlotly {
    RgbPlotly::new(rgb.0, rgb.1, rgb.2)
}

pub(crate) fn convert_line(line: &Line) -> DashType {
    match line {
        Line::Solid => DashType::Solid,
        Line::Dot => DashType::Dot,
        Line::Dash => DashType::Dash,
        Line::LongDash => DashType::LongDash,
        Line::DashDot => DashType::DashDot,
        Line::LongDashDot => DashType::LongDashDot,
    }
}

pub(crate) fn convert_mode(mode: &Mode) -> PlotlyMode {
    match mode {
        Mode::Lines => PlotlyMode::Lines,
        Mode::Markers => PlotlyMode::Markers,
        Mode::Text => PlotlyMode::Text,
        Mode::LinesMarkers => PlotlyMode::LinesMarkers,
        Mode::LinesText => PlotlyMode::LinesText,
        Mode::MarkersText => PlotlyMode::MarkersText,
        Mode::LinesMarkersText => PlotlyMode::LinesMarkersText,
        Mode::None => PlotlyMode::None,
    }
}

pub(crate) fn convert_orientation(orientation: &Orientation) -> OrientationPlotly {
    match orientation {
        Orientation::Horizontal => OrientationPlotly::Horizontal,
        Orientation::Vertical => OrientationPlotly::Vertical,
    }
}

pub(crate) fn convert_fill(fill: &Fill) -> PlotlyFill {
    match fill {
        Fill::ToZeroY => PlotlyFill::ToZeroY,
        Fill::ToZeroX => PlotlyFill::ToZeroX,
        Fill::ToNextY => PlotlyFill::ToNextY,
        Fill::ToNextX => PlotlyFill::ToNextX,
        Fill::ToSelf => PlotlyFill::ToSelf,
        Fill::ToNext => PlotlyFill::ToNext,
        Fill::None => PlotlyFill::None,
    }
}

pub(crate) fn convert_bar_mode(bar_mode: &BarMode) -> BarModePlotly {
    match bar_mode {
        BarMode::Stack => BarModePlotly::Stack,
        BarMode::Group => BarModePlotly::Group,
        BarMode::Overlay => BarModePlotly::Overlay,
        BarMode::Relative => BarModePlotly::Relative,
    }
}

pub(crate) fn convert_coloring(coloring: &Coloring) -> ColoringPlotly {
    match coloring {
        Coloring::Fill => ColoringPlotly::Fill,
        Coloring::HeatMap => ColoringPlotly::HeatMap,
        Coloring::Lines => ColoringPlotly::Lines,
        Coloring::None => ColoringPlotly::None,
    }
}

pub(crate) fn convert_exponent(exponent: &ValueExponent) -> ExponentFormat {
    match exponent {
        ValueExponent::None => ExponentFormat::None,
        ValueExponent::SmallE => ExponentFormat::SmallE,
        ValueExponent::CapitalE => ExponentFormat::CapitalE,
        ValueExponent::Power => ExponentFormat::Power,
        ValueExponent::SI => ExponentFormat::SI,
        ValueExponent::B => ExponentFormat::B,
    }
}

pub(crate) fn convert_intensity_mode(mode: &IntensityMode) -> PlotlyIntensityMode {
    match mode {
        IntensityMode::Vertex => PlotlyIntensityMode::Vertex,
        IntensityMode::Cell => PlotlyIntensityMode::Cell,
    }
}

pub(crate) fn convert_arrangement(arrangement: &Arrangement) -> ArrangementPlotly {
    match arrangement {
        Arrangement::Snap => ArrangementPlotly::Snap,
        Arrangement::Perpendicular => ArrangementPlotly::Perpendicular,
        Arrangement::Freeform => ArrangementPlotly::Freeform,
        Arrangement::Fixed => ArrangementPlotly::Fixed,
    }
}

pub(crate) fn convert_shape(shape: &Shape) -> MarkerSymbol {
    match shape {
        Shape::Circle => MarkerSymbol::Circle,
        Shape::CircleOpen => MarkerSymbol::CircleOpen,
        Shape::CircleDot => MarkerSymbol::CircleDot,
        Shape::CircleOpenDot => MarkerSymbol::CircleOpenDot,
        Shape::Square => MarkerSymbol::Square,
        Shape::SquareOpen => MarkerSymbol::SquareOpen,
        Shape::SquareDot => MarkerSymbol::SquareDot,
        Shape::SquareOpenDot => MarkerSymbol::SquareOpenDot,
        Shape::Diamond => MarkerSymbol::Diamond,
        Shape::DiamondOpen => MarkerSymbol::DiamondOpen,
        Shape::DiamondDot => MarkerSymbol::DiamondDot,
        Shape::DiamondOpenDot => MarkerSymbol::DiamondOpenDot,
        Shape::Cross => MarkerSymbol::Cross,
        Shape::CrossOpen => MarkerSymbol::CrossOpen,
        Shape::CrossDot => MarkerSymbol::CrossDot,
        Shape::CrossOpenDot => MarkerSymbol::CrossOpenDot,
        Shape::X => MarkerSymbol::X,
        Shape::XOpen => MarkerSymbol::XOpen,
        Shape::XDot => MarkerSymbol::XDot,
        Shape::XOpenDot => MarkerSymbol::XOpenDot,
        Shape::TriangleUp => MarkerSymbol::TriangleUp,
        Shape::TriangleUpOpen => MarkerSymbol::TriangleUpOpen,
        Shape::TriangleUpDot => MarkerSymbol::TriangleUpDot,
        Shape::TriangleUpOpenDot => MarkerSymbol::TriangleUpOpenDot,
        Shape::TriangleDown => MarkerSymbol::TriangleDown,
        Shape::TriangleDownOpen => MarkerSymbol::TriangleDownOpen,
        Shape::TriangleDownDot => MarkerSymbol::TriangleDownDot,
        Shape::TriangleDownOpenDot => MarkerSymbol::TriangleDownOpenDot,
        Shape::TriangleLeft => MarkerSymbol::TriangleLeft,
        Shape::TriangleLeftOpen => MarkerSymbol::TriangleLeftOpen,
        Shape::TriangleLeftDot => MarkerSymbol::TriangleLeftDot,
        Shape::TriangleLeftOpenDot => MarkerSymbol::TriangleLeftOpenDot,
        Shape::TriangleRight => MarkerSymbol::TriangleRight,
        Shape::TriangleRightOpen => MarkerSymbol::TriangleRightOpen,
        Shape::TriangleRightDot => MarkerSymbol::TriangleRightDot,
        Shape::TriangleRightOpenDot => MarkerSymbol::TriangleRightOpenDot,
        Shape::TriangleNE => MarkerSymbol::TriangleNE,
        Shape::TriangleNEOpen => MarkerSymbol::TriangleNEOpen,
        Shape::TriangleNEDot => MarkerSymbol::TriangleNEDot,
        Shape::TriangleNEOpenDot => MarkerSymbol::TriangleNEOpenDot,
        Shape::TriangleSE => MarkerSymbol::TriangleSE,
        Shape::TriangleSEOpen => MarkerSymbol::TriangleSEOpen,
        Shape::TriangleSEDot => MarkerSymbol::TriangleSEDot,
        Shape::TriangleSEOpenDot => MarkerSymbol::TriangleSEOpenDot,
        Shape::TriangleSW => MarkerSymbol::TriangleSW,
        Shape::TriangleSWOpen => MarkerSymbol::TriangleSWOpen,
        Shape::TriangleSWDot => MarkerSymbol::TriangleSWDot,
        Shape::TriangleSWOpenDot => MarkerSymbol::TriangleSWOpenDot,
        Shape::TriangleNW => MarkerSymbol::TriangleNW,
        Shape::TriangleNWOpen => MarkerSymbol::TriangleNWOpen,
        Shape::TriangleNWDot => MarkerSymbol::TriangleNWDot,
        Shape::TriangleNWOpenDot => MarkerSymbol::TriangleNWOpenDot,
        Shape::Pentagon => MarkerSymbol::Pentagon,
        Shape::PentagonOpen => MarkerSymbol::PentagonOpen,
        Shape::PentagonDot => MarkerSymbol::PentagonDot,
        Shape::PentagonOpenDot => MarkerSymbol::PentagonOpenDot,
        Shape::Hexagon => MarkerSymbol::Hexagon,
        Shape::HexagonOpen => MarkerSymbol::HexagonOpen,
        Shape::HexagonDot => MarkerSymbol::HexagonDot,
        Shape::HexagonOpenDot => MarkerSymbol::HexagonOpenDot,
        Shape::Hexagon2 => MarkerSymbol::Hexagon2,
        Shape::Hexagon2Open => MarkerSymbol::Hexagon2Open,
        Shape::Hexagon2Dot => MarkerSymbol::Hexagon2Dot,
        Shape::Hexagon2OpenDot => MarkerSymbol::Hexagon2OpenDot,
        Shape::Octagon => MarkerSymbol::Octagon,
        Shape::OctagonOpen => MarkerSymbol::OctagonOpen,
        Shape::OctagonDot => MarkerSymbol::OctagonDot,
        Shape::OctagonOpenDot => MarkerSymbol::OctagonOpenDot,
        Shape::Star => MarkerSymbol::Star,
        Shape::StarOpen => MarkerSymbol::StarOpen,
        Shape::StarDot => MarkerSymbol::StarDot,
        Shape::StarOpenDot => MarkerSymbol::StarOpenDot,
        Shape::Hexagram => MarkerSymbol::Hexagram,
        Shape::HexagramOpen => MarkerSymbol::HexagramOpen,
        Shape::HexagramDot => MarkerSymbol::HexagramDot,
        Shape::HexagramOpenDot => MarkerSymbol::HexagramOpenDot,
        Shape::StarTriangleUp => MarkerSymbol::StarTriangleUp,
        Shape::StarTriangleUpOpen => MarkerSymbol::StarTriangleUpOpen,
        Shape::StarTriangleUpDot => MarkerSymbol::StarTriangleUpDot,
        Shape::StarTriangleUpOpenDot => MarkerSymbol::StarTriangleUpOpenDot,
        Shape::StarTriangleDown => MarkerSymbol::StarTriangleDown,
        Shape::StarTriangleDownOpen => MarkerSymbol::StarTriangleDownOpen,
        Shape::StarTriangleDownDot => MarkerSymbol::StarTriangleDownDot,
        Shape::StarTriangleDownOpenDot => MarkerSymbol::StarTriangleDownOpenDot,
        Shape::StarSquare => MarkerSymbol::StarSquare,
        Shape::StarSquareOpen => MarkerSymbol::StarSquareOpen,
        Shape::StarSquareDot => MarkerSymbol::StarSquareDot,
        Shape::StarSquareOpenDot => MarkerSymbol::StarSquareOpenDot,
        Shape::StarDiamond => MarkerSymbol::StarDiamond,
        Shape::StarDiamondOpen => MarkerSymbol::StarDiamondOpen,
        Shape::StarDiamondDot => MarkerSymbol::StarDiamondDot,
        Shape::StarDiamondOpenDot => MarkerSymbol::StarDiamondOpenDot,
        Shape::DiamondTall => MarkerSymbol::DiamondTall,
        Shape::DiamondTallOpen => MarkerSymbol::DiamondTallOpen,
        Shape::DiamondTallDot => MarkerSymbol::DiamondTallDot,
        Shape::DiamondTallOpenDot => MarkerSymbol::DiamondTallOpenDot,
        Shape::DiamondWide => MarkerSymbol::DiamondWide,
        Shape::DiamondWideOpen => MarkerSymbol::DiamondWideOpen,
        Shape::DiamondWideDot => MarkerSymbol::DiamondWideDot,
        Shape::DiamondWideOpenDot => MarkerSymbol::DiamondWideOpenDot,
        Shape::Hourglass => MarkerSymbol::Hourglass,
        Shape::HourglassOpen => MarkerSymbol::HourglassOpen,
        Shape::BowTie => MarkerSymbol::BowTie,
        Shape::BowTieOpen => MarkerSymbol::BowTieOpen,
        Shape::CircleCross => MarkerSymbol::CircleCross,
        Shape::CircleCrossOpen => MarkerSymbol::CircleCrossOpen,
        Shape::CircleX => MarkerSymbol::CircleX,
        Shape::CircleXOpen => MarkerSymbol::CircleXOpen,
        Shape::SquareCross => MarkerSymbol::SquareCross,
        Shape::SquareCrossOpen => MarkerSymbol::SquareCrossOpen,
        Shape::SquareX => MarkerSymbol::SquareX,
        Shape::SquareXOpen => MarkerSymbol::SquareXOpen,
        Shape::DiamondCross => MarkerSymbol::DiamondCross,
        Shape::DiamondCrossOpen => MarkerSymbol::DiamondCrossOpen,
        Shape::DiamondX => MarkerSymbol::DiamondX,
        Shape::DiamondXOpen => MarkerSymbol::DiamondXOpen,
        Shape::CrossThin => MarkerSymbol::CrossThin,
        Shape::CrossThinOpen => MarkerSymbol::CrossThinOpen,
        Shape::XThin => MarkerSymbol::XThin,
        Shape::XThinOpen => MarkerSymbol::XThinOpen,
        Shape::Asterisk => MarkerSymbol::Asterisk,
        Shape::AsteriskOpen => MarkerSymbol::AsteriskOpen,
        Shape::Hash => MarkerSymbol::Hash,
        Shape::HashOpen => MarkerSymbol::HashOpen,
        Shape::HashDot => MarkerSymbol::HashDot,
        Shape::HashOpenDot => MarkerSymbol::HashOpenDot,
        Shape::YUp => MarkerSymbol::YUp,
        Shape::YUpOpen => MarkerSymbol::YUpOpen,
        Shape::YDown => MarkerSymbol::YDown,
        Shape::YDownOpen => MarkerSymbol::YDownOpen,
        Shape::YLeft => MarkerSymbol::YLeft,
        Shape::YLeftOpen => MarkerSymbol::YLeftOpen,
        Shape::YRight => MarkerSymbol::YRight,
        Shape::YRightOpen => MarkerSymbol::YRightOpen,
        Shape::LineEW => MarkerSymbol::LineEW,
        Shape::LineEWOpen => MarkerSymbol::LineEWOpen,
        Shape::LineNS => MarkerSymbol::LineNS,
        Shape::LineNSOpen => MarkerSymbol::LineNSOpen,
        Shape::LineNE => MarkerSymbol::LineNE,
        Shape::LineNEOpen => MarkerSymbol::LineNEOpen,
        Shape::LineNW => MarkerSymbol::LineNW,
        Shape::LineNWOpen => MarkerSymbol::LineNWOpen,
    }
}

pub(crate) fn convert_palette(palette: &Palette) -> ColorScale {
    match palette {
        Palette::Greys => ColorScale::Palette(ColorScalePalette::Greys),
        Palette::YlGnBu => ColorScale::Palette(ColorScalePalette::YlGnBu),
        Palette::Greens => ColorScale::Palette(ColorScalePalette::Greens),
        Palette::YlOrRd => ColorScale::Palette(ColorScalePalette::YlOrRd),
        Palette::Bluered => ColorScale::Palette(ColorScalePalette::Bluered),
        Palette::RdBu => ColorScale::Palette(ColorScalePalette::RdBu),
        Palette::Reds => ColorScale::Palette(ColorScalePalette::Reds),
        Palette::Blues => ColorScale::Palette(ColorScalePalette::Blues),
        Palette::Picnic => ColorScale::Palette(ColorScalePalette::Picnic),
        Palette::Rainbow => ColorScale::Palette(ColorScalePalette::Rainbow),
        Palette::Portland => ColorScale::Palette(ColorScalePalette::Portland),
        Palette::Jet => ColorScale::Palette(ColorScalePalette::Jet),
        Palette::Hot => ColorScale::Palette(ColorScalePalette::Hot),
        Palette::Blackbody => ColorScale::Palette(ColorScalePalette::Blackbody),
        Palette::Earth => ColorScale::Palette(ColorScalePalette::Earth),
        Palette::Electric => ColorScale::Palette(ColorScalePalette::Electric),
        Palette::Viridis => ColorScale::Palette(ColorScalePalette::Viridis),
        Palette::Cividis => ColorScale::Palette(ColorScalePalette::Cividis),
    }
}

// ── Tier 2: Components depending on Rgb/Text ─────────────────────────────

pub(crate) fn convert_tick_direction(tick: &TickDirection) -> TicksDirection {
    match tick {
        TickDirection::OutSide => TicksDirection::Outside,
        TickDirection::InSide => TicksDirection::Inside,
        TickDirection::None => TicksDirection::Outside,
    }
}

pub(crate) fn convert_tick_ticks(tick: &TickDirection) -> Ticks {
    match tick {
        TickDirection::OutSide => Ticks::Outside,
        TickDirection::InSide => Ticks::Inside,
        TickDirection::None => Ticks::None,
    }
}

pub(crate) fn convert_direction_increasing(direction: &Direction) -> DirectionPlotly {
    let mut line = LinePlotly::new();

    if let Some(line_color) = &direction.line_color {
        line = line.color(convert_rgb(line_color));
    }

    if let Some(width) = direction.line_width {
        line = line.width(width);
    }

    DirectionPlotly::Increasing { line }
}

pub(crate) fn convert_direction_decreasing(direction: &Direction) -> DirectionPlotly {
    let mut line = LinePlotly::new();

    if let Some(line_color) = &direction.line_color {
        line = line.color(convert_rgb(line_color));
    }

    if let Some(width) = direction.line_width {
        line = line.width(width);
    }

    DirectionPlotly::Decreasing { line }
}

pub(crate) fn convert_colorbar(colorbar: &ColorBar) -> ColorBarPlotly {
    let mut color_bar = ColorBarPlotly::new();

    if let Some(color) = &colorbar.background_color {
        color_bar = color_bar.background_color(convert_rgb(color));
    }

    if let Some(color) = &colorbar.border_color {
        color_bar = color_bar.border_color(convert_rgb(color));
    }

    if let Some(width) = colorbar.border_width {
        color_bar = color_bar.border_width(width);
    }

    if let Some(step) = colorbar.tick_step {
        color_bar = color_bar.dtick(step);
    }

    if let Some(value_exponent) = &colorbar.value_exponent {
        color_bar = color_bar.exponent_format(convert_exponent(value_exponent));
    }

    // NOTE: length (len) is NOT set here to avoid plotly.rs's usize limitation.
    // Instead, it will be injected as f64 via JSON post-processing using patch_trace_json().

    if let Some(n_ticks) = colorbar.n_ticks {
        color_bar = color_bar.n_ticks(n_ticks);
    }

    if let Some(orientation) = &colorbar.orientation {
        color_bar = color_bar.orientation(convert_orientation(orientation));
    }

    if let Some(color) = colorbar.outline_color {
        color_bar = color_bar.outline_color(convert_rgb(&color));
    }

    if let Some(width) = colorbar.outline_width {
        color_bar = color_bar.outline_width(width);
    }

    if let Some(separate_thousands) = colorbar.separate_thousands {
        color_bar = color_bar.separate_thousands(separate_thousands);
    }

    // NOTE: width (thickness) is NOT set here to avoid plotly.rs's usize limitation.
    // Instead, it will be injected as f64 via JSON post-processing using patch_trace_json().

    if let Some(angle) = colorbar.tick_angle {
        color_bar = color_bar.tick_angle(angle);
    }

    if let Some(color) = colorbar.tick_color {
        color_bar = color_bar.tick_color(convert_rgb(&color));
    }

    if let Some(font) = &colorbar.tick_font {
        color_bar = color_bar.tick_font(Font::new().family(font.as_str()));
    }

    if let Some(length) = colorbar.tick_length {
        color_bar = color_bar.tick_len(length);
    }

    if let Some(labels) = &colorbar.tick_labels {
        color_bar = color_bar.tick_text(labels.to_owned())
    }

    if let Some(values) = &colorbar.tick_values {
        color_bar = color_bar.tick_vals(values.to_owned());
    }

    if let Some(width) = colorbar.tick_width {
        color_bar = color_bar.tick_width(width);
    }

    if let Some(tick_direction) = &colorbar.tick_direction {
        color_bar = color_bar.ticks(convert_tick_ticks(tick_direction));
    }

    if let Some(title) = &colorbar.title {
        color_bar = color_bar.title(convert_text_to_title(title));
    }

    if let Some(x) = colorbar.x {
        color_bar = color_bar.x(x);
    }

    if let Some(y) = colorbar.y {
        color_bar = color_bar.y(y);
    }

    color_bar
}

pub(crate) fn convert_header<T>(header: &Header, default_values: Vec<T>) -> HeaderPlotly<T>
where
    T: serde::Serialize + Clone + Default + 'static,
{
    let mut h = HeaderPlotly::new(default_values);

    if let Some(height) = header.height {
        h = h.height(height);
    }

    if let Some(align) = &header.align {
        let align_enum = match align.to_lowercase().as_str() {
            "left" => Align::Left,
            "right" => Align::Right,
            _ => Align::Center,
        };
        h = h.align(align_enum);
    }

    if let Some(font) = &header.font {
        h = h.font(TableFont::new().family(font.as_str()));
    }

    if let Some(fill) = &header.fill {
        h = h.fill(TableFill::new().color(convert_rgb(fill)));
    }

    h
}

pub(crate) fn convert_cell<T>(cell: &Cell, default_values: Vec<Vec<T>>) -> CellsPlotly<T>
where
    T: serde::Serialize + Clone + Default + 'static,
{
    let mut cells = CellsPlotly::new(default_values);

    if let Some(height) = cell.height {
        cells = cells.height(height);
    }

    if let Some(align) = &cell.align {
        let align_enum = match align.to_lowercase().as_str() {
            "left" => Align::Left,
            "right" => Align::Right,
            _ => Align::Center,
        };
        cells = cells.align(align_enum);
    }

    if let Some(fill) = &cell.fill {
        cells = cells.fill(TableFill::new().color(convert_rgb(fill)));
    }

    cells
}

// ── Tier 3: Complex components ───────────────────────────────────────────

pub(crate) fn convert_axis_side(side: &AxisSide) -> AxisSidePlotly {
    match side {
        AxisSide::Top => AxisSidePlotly::Top,
        AxisSide::Bottom => AxisSidePlotly::Bottom,
        AxisSide::Left => AxisSidePlotly::Left,
        AxisSide::Right => AxisSidePlotly::Right,
    }
}

pub(crate) fn convert_axis_type(axis_type: &AxisType) -> AxisTypePlotly {
    match axis_type {
        AxisType::Default => AxisTypePlotly::Default,
        AxisType::Linear => AxisTypePlotly::Linear,
        AxisType::Log => AxisTypePlotly::Log,
        AxisType::Date => AxisTypePlotly::Date,
        AxisType::Category => AxisTypePlotly::Category,
        AxisType::MultiCategory => AxisTypePlotly::MultiCategory,
    }
}

pub(crate) fn set_axis(title: Option<Text>, format: &Axis, overlaying: Option<&str>) -> AxisPlotly {
    let mut axis = AxisPlotly::new();

    if let Some(title) = title {
        axis = axis.title(convert_text_to_title(&title));
    }
    axis = set_axis_format(axis, format, overlaying);

    axis
}

fn set_axis_format(mut axis: AxisPlotly, format: &Axis, overlaying: Option<&str>) -> AxisPlotly {
    if let Some(overlaying) = overlaying {
        axis = axis.overlaying(overlaying);
    }

    if let Some(visible) = format.show_axis {
        axis = axis.visible(visible.to_owned());
    }

    if let Some(axis_position) = &format.axis_side {
        axis = axis.side(convert_axis_side(axis_position));
    }

    if let Some(axis_type) = &format.axis_type {
        axis = axis.type_(convert_axis_type(axis_type));
    }

    if let Some(color) = format.value_color {
        axis = axis.color(convert_rgb(&color));
    }

    if let Some(range) = &format.value_range {
        axis = axis.range(range.to_owned());
    }

    if let Some(thousands) = format.value_thousands {
        axis = axis.separate_thousands(thousands.to_owned());
    }

    if let Some(exponent) = &format.value_exponent {
        axis = axis.exponent_format(convert_exponent(exponent));
    }

    if let Some(range_values) = &format.tick_values {
        axis = axis.tick_values(range_values.to_owned());
    }

    if let Some(tick_text) = &format.tick_labels {
        axis = axis.tick_text(tick_text.to_owned());
    }

    if let Some(tick_direction) = &format.tick_direction {
        axis = axis.ticks(convert_tick_direction(tick_direction));
    }

    if let Some(tick_length) = format.tick_length {
        axis = axis.tick_length(tick_length.to_owned());
    }

    if let Some(tick_width) = format.tick_width {
        axis = axis.tick_width(tick_width.to_owned());
    }

    if let Some(color) = format.tick_color {
        axis = axis.tick_color(convert_rgb(&color));
    }

    if let Some(tick_angle) = format.tick_angle {
        axis = axis.tick_angle(tick_angle.to_owned());
    }

    if let Some(font) = &format.tick_font {
        axis = axis.tick_font(Font::new().family(font.as_str()));
    }

    if let Some(show_line) = format.show_line {
        axis = axis.show_line(show_line.to_owned());
    }

    if let Some(color) = format.line_color {
        axis = axis.line_color(convert_rgb(&color));
    }

    if let Some(line_width) = format.line_width {
        axis = axis.line_width(line_width.to_owned());
    }

    if let Some(show_grid) = format.show_grid {
        axis = axis.show_grid(show_grid.to_owned());
    }

    if let Some(color) = format.grid_color {
        axis = axis.grid_color(convert_rgb(&color));
    }

    if let Some(grid_width) = format.grid_width {
        axis = axis.grid_width(grid_width.to_owned());
    }

    if let Some(show_zero_line) = format.show_zero_line {
        axis = axis.zero_line(show_zero_line.to_owned());
    }

    if let Some(color) = format.zero_line_color {
        axis = axis.zero_line_color(convert_rgb(&color));
    }

    if let Some(zero_line_width) = format.zero_line_width {
        axis = axis.zero_line_width(zero_line_width.to_owned());
    }

    if let Some(axis_position) = format.axis_position {
        axis = axis.position(axis_position.to_owned());
    }

    axis
}

pub(crate) fn set_legend(title: Option<Text>, format: Option<&Legend>) -> LegendPlotly {
    let mut legend = LegendPlotly::new();

    if let Some(title) = title {
        legend = legend.title(convert_text_to_title(&title));
    }

    if let Some(format) = format {
        legend = set_legend_format(legend, format);
    }

    legend
}

fn set_legend_format(mut legend: LegendPlotly, format: &Legend) -> LegendPlotly {
    if let Some(color) = format.background_color {
        legend = legend.background_color(convert_rgb(&color));
    }

    if let Some(color) = format.border_color {
        legend = legend.border_color(convert_rgb(&color));
    }

    if let Some(width) = format.border_width {
        legend = legend.border_width(width);
    }

    if let Some(font) = &format.font {
        legend = legend.font(Font::new().family(font.as_str()));
    }

    if let Some(orientation) = &format.orientation {
        legend = legend.orientation(convert_orientation(orientation));
    }

    if let Some(x) = format.x {
        legend = legend.x(x);
    }

    if let Some(y) = format.y {
        legend = legend.y(y);
    }

    legend
}

pub(crate) fn set_lighting(lighting: Option<&Lighting>) -> plotly::surface::Lighting {
    let mut lighting_plotly = plotly::surface::Lighting::new();

    if let Some(light) = lighting {
        if let Some(ambient) = light.ambient {
            lighting_plotly = lighting_plotly.ambient(ambient);
        }

        if let Some(diffuse) = light.diffuse {
            lighting_plotly = lighting_plotly.diffuse(diffuse);
        }

        if let Some(fresnel) = light.fresnel {
            lighting_plotly = lighting_plotly.fresnel(fresnel);
        }

        if let Some(roughness) = light.roughness {
            lighting_plotly = lighting_plotly.roughness(roughness);
        }

        if let Some(specular) = light.specular {
            lighting_plotly = lighting_plotly.specular(specular);
        }
    }

    lighting_plotly
}

// ── Tier 4: Text (depended on by axis, legend, colorbar, layout) ─────────

pub(crate) fn convert_text_to_title(text: &Text) -> Title {
    Title::with_text(&text.content)
        .font(
            Font::new()
                .family(text.font.as_str())
                .size(text.size)
                .color(convert_rgb(&text.color)),
        )
        .x(text.x)
        .y(text.y)
}

pub(crate) fn convert_text_to_font(text: &Text) -> Font {
    Font::new()
        .family(text.font.as_str())
        .size(text.size)
        .color(convert_rgb(&text.color))
}

pub(crate) fn convert_text_to_axis_annotation(
    text: &Text,
    is_x_axis: bool,
    axis_ref: &str,
    use_domain: bool,
) -> Annotation {
    let (x_ref, y_ref) = if use_domain {
        let axis_num = axis_ref.trim_start_matches(['x', 'y']);

        let x_axis = if axis_num.is_empty() {
            "x"
        } else {
            &format!("x{}", axis_num)
        };
        let y_axis = if axis_num.is_empty() {
            "y"
        } else {
            &format!("y{}", axis_num)
        };

        (format!("{} domain", x_axis), format!("{} domain", y_axis))
    } else {
        ("paper".to_string(), "paper".to_string())
    };

    let y_anchor = Anchor::Middle;
    let x_anchor = if is_x_axis {
        Anchor::Center
    } else {
        Anchor::Left
    };

    let effective_size = if text.size == 0 { 12 } else { text.size };

    let mut annotation = Annotation::new()
        .text(&text.content)
        .font(
            Font::new()
                .family(text.font.as_str())
                .size(effective_size)
                .color(convert_rgb(&text.color)),
        )
        .x_ref(&x_ref)
        .y_ref(&y_ref)
        .x(text.x)
        .y(text.y)
        .x_anchor(x_anchor)
        .y_anchor(y_anchor)
        .show_arrow(false);

    if !is_x_axis {
        annotation = annotation.text_angle(-90.0);
    }

    annotation
}
