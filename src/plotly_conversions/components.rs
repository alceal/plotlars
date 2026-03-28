#![allow(dead_code)]

use plotly::color::Rgb as RgbPlotly;
use plotly::common::{
    AxisSide as AxisSidePlotly, ColorBar as ColorBarPlotly, ColorScale, DashType, Direction as DirectionPlotly,
    ExponentFormat, Fill as PlotlyFill, Font, MarkerSymbol, Mode as PlotlyMode,
    Orientation as OrientationPlotly, Ticks, Title,
};
use plotly::contour::Coloring as ColoringPlotly;
use plotly::layout::{
    Annotation, Axis as AxisPlotly, AxisType as AxisTypePlotly,
    BarMode as BarModePlotly, Legend as LegendPlotly, TicksDirection,
};
use plotly::sankey::Arrangement as ArrangementPlotly;
use plotly::traces::mesh3d::IntensityMode as PlotlyIntensityMode;

use crate::components::{
    Arrangement, BarMode, Coloring, Fill, IntensityMode, Line, Mode,
    Orientation, Palette, Shape, ValueExponent,
};
use crate::components::axis::{Axis, AxisSide, AxisType};
use crate::components::colorbar::ColorBar;
use crate::components::direction::Direction;
use crate::components::header::Header;
use crate::components::cell::Cell;
use crate::components::legend::Legend;
use crate::components::lighting::Lighting;
use crate::components::text::Text;
use crate::components::tick::TickDirection;
use crate::components::Rgb;

// ── Leaf component conversions (Tier 1) ──────────────────────────────────

pub(crate) fn convert_rgb(rgb: &Rgb) -> RgbPlotly {
    rgb.to_plotly()
}

pub(crate) fn convert_line(line: &Line) -> DashType {
    line.to_plotly()
}

pub(crate) fn convert_mode(mode: &Mode) -> PlotlyMode {
    mode.to_plotly()
}

pub(crate) fn convert_orientation(orientation: &Orientation) -> OrientationPlotly {
    orientation.to_plotly()
}

pub(crate) fn convert_fill(fill: &Fill) -> PlotlyFill {
    fill.to_plotly()
}

pub(crate) fn convert_bar_mode(bar_mode: &BarMode) -> BarModePlotly {
    bar_mode.to_plotly()
}

pub(crate) fn convert_coloring(coloring: Coloring) -> ColoringPlotly {
    coloring.to_plotly()
}

pub(crate) fn convert_exponent(exponent: &ValueExponent) -> ExponentFormat {
    exponent.to_plotly()
}

pub(crate) fn convert_intensity_mode(mode: &IntensityMode) -> PlotlyIntensityMode {
    mode.to_plotly()
}

pub(crate) fn convert_arrangement(arrangement: &Arrangement) -> ArrangementPlotly {
    arrangement.to_plotly()
}

pub(crate) fn convert_shape(shape: &Shape) -> MarkerSymbol {
    shape.to_plotly()
}

pub(crate) fn convert_palette(palette: &Palette) -> ColorScale {
    palette.to_plotly()
}

// ── Tier 2: Components depending on Rgb/Text ─────────────────────────────

pub(crate) fn convert_tick_direction(tick: &TickDirection) -> TicksDirection {
    tick.to_plotly_tickdirection()
}

pub(crate) fn convert_tick_ticks(tick: &TickDirection) -> Ticks {
    tick.to_plotly_ticks()
}

pub(crate) fn convert_direction_increasing(direction: &Direction) -> DirectionPlotly {
    direction.to_plotly_increasing()
}

pub(crate) fn convert_direction_decreasing(direction: &Direction) -> DirectionPlotly {
    direction.to_plotly_decreasing()
}

pub(crate) fn convert_colorbar(colorbar: &ColorBar) -> ColorBarPlotly {
    colorbar.to_plotly()
}

pub(crate) fn convert_header<T>(header: &Header, default_values: Vec<T>) -> plotly::traces::table::Header<T>
where
    T: serde::Serialize + Clone + Default + 'static,
{
    header.to_plotly(default_values)
}

pub(crate) fn convert_cell<T>(cell: &Cell, default_values: Vec<Vec<T>>) -> plotly::traces::table::Cells<T>
where
    T: serde::Serialize + Clone + Default + 'static,
{
    cell.to_plotly(default_values)
}

// ── Tier 3: Complex components ───────────────────────────────────────────

pub(crate) fn convert_axis_side(side: &AxisSide) -> AxisSidePlotly {
    side.to_plotly()
}

pub(crate) fn convert_axis_type(axis_type: &AxisType) -> AxisTypePlotly {
    axis_type.to_plotly()
}

pub(crate) fn set_axis(
    title: Option<Text>,
    format: &Axis,
    overlaying: Option<&str>,
) -> AxisPlotly {
    Axis::set_axis(title, format, overlaying)
}

pub(crate) fn set_legend(title: Option<Text>, format: Option<&Legend>) -> LegendPlotly {
    Legend::set_legend(title, format)
}

pub(crate) fn set_lighting(lighting: Option<&Lighting>) -> plotly::surface::Lighting {
    Lighting::set_lighting(lighting)
}

// ── Tier 4: Text (depended on by axis, legend, colorbar, layout) ─────────

pub(crate) fn convert_text_to_title(text: &Text) -> Title {
    text.to_plotly()
}

pub(crate) fn convert_text_to_font(text: &Text) -> Font {
    text.to_font()
}

pub(crate) fn convert_text_to_axis_annotation(
    text: &Text,
    is_x_axis: bool,
    axis_ref: &str,
    use_domain: bool,
) -> Annotation {
    text.to_axis_annotation(is_x_axis, axis_ref, use_domain)
}
