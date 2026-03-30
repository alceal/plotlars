use crate::components::facet::FacetScales;
use crate::components::{Axis, Legend, Text};

/// Distinguishes the layout strategy a faceted plot requires.
#[derive(Clone)]
#[doc(hidden)]
pub enum FacetKind {
    /// 2D Cartesian: BarPlot, BoxPlot, ScatterPlot, LinePlot, etc.
    Axis,
    /// 3D: Scatter3dPlot, SurfacePlot, Mesh3D
    Scene,
    /// ScatterPolar
    Polar,
    /// PieChart, SankeyDiagram
    Domain,
}

#[derive(Clone)]
#[doc(hidden)]
pub struct GridSpec {
    pub kind: FacetKind,
    pub rows: usize,
    pub cols: usize,
    pub h_gap: Option<f64>,
    pub v_gap: Option<f64>,
    pub scales: FacetScales,
    pub n_facets: usize,
    pub facet_categories: Vec<String>,
    pub title_style: Option<Text>,
    pub x_title: Option<Text>,
    pub y_title: Option<Text>,
    pub x_axis: Option<Axis>,
    pub y_axis: Option<Axis>,
    pub legend_title: Option<Text>,
    pub legend: Option<Legend>,
}

#[derive(Clone)]
#[doc(hidden)]
pub struct AnnotationIR {
    pub text: String,
    pub x: f64,
    pub y: f64,
    pub style: Option<Text>,
}
