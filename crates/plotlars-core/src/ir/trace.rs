use crate::components::{
    Arrangement, Coloring, Direction, Fill, IntensityMode, Lighting, Mode, Orientation, Palette,
    Rgb,
};
use crate::ir::data::ColumnData;
use crate::ir::line::LineIR;
use crate::ir::marker::MarkerIR;

use crate::components::cell::Cell;
use crate::components::colorbar::ColorBar;
use crate::components::header::Header;

#[derive(Clone)]
#[doc(hidden)]
pub enum TraceIR {
    ScatterPlot(ScatterPlotIR),
    BarPlot(BarPlotIR),
    BoxPlot(BoxPlotIR),
    LinePlot(LinePlotIR),
    TimeSeriesPlot(TimeSeriesPlotIR),
    Histogram(HistogramIR),
    HeatMap(HeatMapIR),
    ContourPlot(ContourPlotIR),
    PieChart(PieChartIR),
    SankeyDiagram(SankeyDiagramIR),
    CandlestickPlot(CandlestickPlotIR),
    OhlcPlot(OhlcPlotIR),
    ScatterPolar(ScatterPolarIR),
    Scatter3dPlot(Scatter3dPlotIR),
    SurfacePlot(SurfacePlotIR),
    Mesh3D(Mesh3DIR),
    ScatterGeo(ScatterGeoIR),
    ScatterMap(ScatterMapIR),
    DensityMapbox(DensityMapboxIR),
    Table(TableIR),
    Image(ImageIR),
    Array2dPlot(Array2dPlotIR),
    SubplotGrid(SubplotGridIR),
}

// ── Per-variant IR structs ───────────────────────────────────────────────

#[derive(Clone)]
#[doc(hidden)]
pub struct ScatterPlotIR {
    pub x: ColumnData,
    pub y: ColumnData,
    pub name: Option<String>,
    pub marker: Option<MarkerIR>,
    pub fill: Option<Fill>,
    pub show_legend: Option<bool>,
    pub legend_group: Option<String>,
    pub subplot_ref: Option<String>,
}

#[derive(Clone)]
#[doc(hidden)]
pub struct BarPlotIR {
    pub labels: ColumnData,
    pub values: ColumnData,
    pub name: Option<String>,
    pub orientation: Option<Orientation>,
    pub marker: Option<MarkerIR>,
    pub error: Option<ColumnData>,
    pub show_legend: Option<bool>,
    pub legend_group: Option<String>,
    pub subplot_ref: Option<String>,
}

#[derive(Clone)]
#[doc(hidden)]
pub struct BoxPlotIR {
    pub labels: ColumnData,
    pub values: ColumnData,
    pub name: Option<String>,
    pub orientation: Option<Orientation>,
    pub marker: Option<MarkerIR>,
    pub box_points: Option<bool>,
    pub point_offset: Option<f64>,
    pub jitter: Option<f64>,
    pub show_legend: Option<bool>,
    pub legend_group: Option<String>,
    pub subplot_ref: Option<String>,
}

#[derive(Clone)]
#[doc(hidden)]
pub struct LinePlotIR {
    pub x: ColumnData,
    pub y: ColumnData,
    pub name: Option<String>,
    pub marker: Option<MarkerIR>,
    pub line: Option<LineIR>,
    pub mode: Option<Mode>,
    pub show_legend: Option<bool>,
    pub legend_group: Option<String>,
    pub subplot_ref: Option<String>,
}

#[derive(Clone)]
#[doc(hidden)]
pub struct TimeSeriesPlotIR {
    pub x: ColumnData,
    pub y: ColumnData,
    pub name: Option<String>,
    pub marker: Option<MarkerIR>,
    pub line: Option<LineIR>,
    pub mode: Option<Mode>,
    pub show_legend: Option<bool>,
    pub legend_group: Option<String>,
    pub y_axis_ref: Option<String>,
    pub subplot_ref: Option<String>,
}

#[derive(Clone)]
#[doc(hidden)]
pub struct HistogramIR {
    pub x: ColumnData,
    pub name: Option<String>,
    pub marker: Option<MarkerIR>,
    pub bins: Option<BinsIR>,
    pub show_legend: Option<bool>,
    pub legend_group: Option<String>,
    pub subplot_ref: Option<String>,
}

#[derive(Clone)]
#[doc(hidden)]
pub struct BinsIR {
    pub start: f64,
    pub end: f64,
    pub size: f64,
}

#[derive(Clone)]
#[doc(hidden)]
pub struct HeatMapIR {
    pub x: ColumnData,
    pub y: ColumnData,
    pub z: ColumnData,
    pub color_scale: Option<Palette>,
    pub color_bar: Option<ColorBar>,
    pub auto_color_scale: Option<bool>,
    pub reverse_scale: Option<bool>,
    pub show_scale: Option<bool>,
    pub z_min: Option<f64>,
    pub z_max: Option<f64>,
    pub subplot_ref: Option<String>,
}

#[derive(Clone)]
#[doc(hidden)]
pub struct ContourPlotIR {
    pub x: ColumnData,
    pub y: ColumnData,
    pub z: ColumnData,
    pub color_scale: Option<Palette>,
    pub color_bar: Option<ColorBar>,
    pub coloring: Option<Coloring>,
    pub show_lines: Option<bool>,
    pub show_labels: Option<bool>,
    pub n_contours: Option<usize>,
    pub reverse_scale: Option<bool>,
    pub show_scale: Option<bool>,
    pub z_min: Option<f64>,
    pub z_max: Option<f64>,
    pub subplot_ref: Option<String>,
}

#[derive(Clone)]
#[doc(hidden)]
pub struct PieChartIR {
    pub labels: ColumnData,
    pub values: Option<ColumnData>,
    pub name: Option<String>,
    pub hole: Option<f64>,
    pub pull: Option<f64>,
    pub rotation: Option<f64>,
    pub colors: Option<Vec<Rgb>>,
    pub domain_x: Option<(f64, f64)>,
    pub domain_y: Option<(f64, f64)>,
}

#[derive(Clone)]
#[doc(hidden)]
pub struct SankeyDiagramIR {
    pub sources: Vec<usize>,
    pub targets: Vec<usize>,
    pub values: ColumnData,
    pub node_labels: Vec<String>,
    pub orientation: Option<Orientation>,
    pub arrangement: Option<Arrangement>,
    pub pad: Option<usize>,
    pub thickness: Option<usize>,
    pub node_colors: Option<Vec<Rgb>>,
    pub link_colors: Option<Vec<Rgb>>,
    pub domain_x: Option<(f64, f64)>,
    pub domain_y: Option<(f64, f64)>,
}

#[derive(Clone)]
#[doc(hidden)]
pub struct CandlestickPlotIR {
    pub dates: ColumnData,
    pub open: ColumnData,
    pub high: ColumnData,
    pub low: ColumnData,
    pub close: ColumnData,
    pub increasing: Option<Direction>,
    pub decreasing: Option<Direction>,
    pub whisker_width: Option<f64>,
}

#[derive(Clone)]
#[doc(hidden)]
pub struct OhlcPlotIR {
    pub dates: ColumnData,
    pub open: ColumnData,
    pub high: ColumnData,
    pub low: ColumnData,
    pub close: ColumnData,
    pub tick_width: Option<f64>,
}

#[derive(Clone)]
#[doc(hidden)]
pub struct ScatterPolarIR {
    pub theta: ColumnData,
    pub r: ColumnData,
    pub name: Option<String>,
    pub mode: Option<Mode>,
    pub marker: Option<MarkerIR>,
    pub line: Option<LineIR>,
    pub fill: Option<Fill>,
    pub show_legend: Option<bool>,
    pub legend_group: Option<String>,
    pub subplot_ref: Option<String>,
}

#[derive(Clone)]
#[doc(hidden)]
pub struct Scatter3dPlotIR {
    pub x: ColumnData,
    pub y: ColumnData,
    pub z: ColumnData,
    pub name: Option<String>,
    pub mode: Option<Mode>,
    pub marker: Option<MarkerIR>,
    pub show_legend: Option<bool>,
    pub legend_group: Option<String>,
    pub scene_ref: Option<String>,
}

#[derive(Clone)]
#[doc(hidden)]
pub struct SurfacePlotIR {
    pub x: ColumnData,
    pub y: ColumnData,
    pub z: Vec<Vec<f64>>,
    pub color_scale: Option<Palette>,
    pub color_bar: Option<ColorBar>,
    pub reverse_scale: Option<bool>,
    pub show_scale: Option<bool>,
    pub lighting: Option<Lighting>,
    pub opacity: Option<f64>,
    pub scene_ref: Option<String>,
}

#[derive(Clone)]
#[doc(hidden)]
pub struct Mesh3DIR {
    pub x: ColumnData,
    pub y: ColumnData,
    pub z: ColumnData,
    pub i: Option<ColumnData>,
    pub j: Option<ColumnData>,
    pub k: Option<ColumnData>,
    pub intensity: Option<ColumnData>,
    pub intensity_mode: Option<IntensityMode>,
    pub color_scale: Option<Palette>,
    pub color_bar: Option<ColorBar>,
    pub lighting: Option<Lighting>,
    pub opacity: Option<f64>,
    pub color: Option<Rgb>,
    pub flat_shading: Option<bool>,
    pub light_position: Option<(i32, i32, i32)>,
    pub delaunay_axis: Option<String>,
    pub contour: Option<bool>,
    pub scene_ref: Option<String>,
}

#[derive(Clone)]
#[doc(hidden)]
pub struct ScatterGeoIR {
    pub lat: ColumnData,
    pub lon: ColumnData,
    pub name: Option<String>,
    pub text: Option<ColumnData>,
    pub mode: Option<Mode>,
    pub marker: Option<MarkerIR>,
    pub line: Option<LineIR>,
    pub show_legend: Option<bool>,
}

#[derive(Clone)]
#[doc(hidden)]
pub struct ScatterMapIR {
    pub lat: ColumnData,
    pub lon: ColumnData,
    pub name: Option<String>,
    pub marker: Option<MarkerIR>,
    pub show_legend: Option<bool>,
}

#[derive(Clone)]
#[doc(hidden)]
pub struct DensityMapboxIR {
    pub lat: ColumnData,
    pub lon: ColumnData,
    pub z: ColumnData,
    pub radius: Option<u8>,
    pub opacity: Option<f64>,
    pub z_min: Option<f64>,
    pub z_max: Option<f64>,
    pub z_mid: Option<f64>,
}

#[derive(Clone)]
#[doc(hidden)]
pub struct TableIR {
    pub header: Option<Header>,
    pub cell: Option<Cell>,
    pub column_names: Vec<String>,
    pub column_data: Vec<Vec<String>>,
    pub column_width: Option<f64>,
}

#[derive(Clone)]
#[doc(hidden)]
pub struct ImageIR {
    pub pixels: Vec<Vec<[u8; 3]>>,
}

#[derive(Clone)]
#[doc(hidden)]
pub struct Array2dPlotIR {
    pub data: Vec<Vec<[u8; 3]>>,
}

#[derive(Clone)]
#[doc(hidden)]
pub struct SubplotGridIR {
    pub geometry: GridGeometry,
}

#[derive(Clone)]
#[doc(hidden)]
pub struct GridGeometry {
    pub rows: usize,
    pub cols: usize,
    pub h_gap: f64,
    pub v_gap: f64,
}
