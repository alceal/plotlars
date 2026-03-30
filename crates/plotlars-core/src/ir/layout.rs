use crate::components::{Axis, BarMode, Dimensions, Legend, Text};
use crate::ir::facet::{AnnotationIR, GridSpec};

#[derive(Clone)]
#[doc(hidden)]
pub struct LayoutIR {
    pub title: Option<Text>,
    pub x_title: Option<Text>,
    pub y_title: Option<Text>,
    pub y2_title: Option<Text>,
    pub z_title: Option<Text>,
    pub legend_title: Option<Text>,
    pub legend: Option<Legend>,
    pub dimensions: Option<Dimensions>,
    pub bar_mode: Option<BarMode>,
    pub box_mode: Option<BoxModeIR>,
    pub box_gap: Option<f64>,
    pub margin_bottom: Option<usize>,
    pub axes_2d: Option<Axes2dIR>,
    pub scene_3d: Option<Scene3dIR>,
    pub polar: Option<PolarAxisIR>,
    pub mapbox: Option<MapboxIR>,
    pub grid: Option<GridSpec>,
    pub annotations: Vec<AnnotationIR>,
}

#[derive(Clone)]
#[doc(hidden)]
pub enum BoxModeIR {
    Group,
}

#[derive(Clone)]
#[doc(hidden)]
pub struct Axes2dIR {
    pub x_axis: Option<Axis>,
    pub y_axis: Option<Axis>,
    pub y2_axis: Option<Axis>,
}

#[derive(Clone)]
#[doc(hidden)]
pub struct Scene3dIR {
    pub x_axis: Option<Axis>,
    pub y_axis: Option<Axis>,
    pub z_axis: Option<Axis>,
    pub domain: Option<DomainIR>,
}

#[derive(Clone)]
#[doc(hidden)]
pub struct PolarAxisIR {
    pub radial_axis: Option<Axis>,
    pub angular_axis: Option<Axis>,
    pub domain: Option<DomainIR>,
}

#[derive(Clone)]
#[doc(hidden)]
pub struct MapboxIR {
    pub center: Option<(f64, f64)>,
    pub zoom: Option<f64>,
    pub style: Option<String>,
}

#[derive(Clone)]
#[doc(hidden)]
pub struct DomainIR {
    pub x: (f64, f64),
    pub y: (f64, f64),
}
