
use crate::components::facet::FacetScales;
use crate::components::Text;

#[derive(Clone)]
pub(crate) struct GridSpec {
    pub rows: usize,
    pub cols: usize,
    pub h_gap: f64,
    pub v_gap: f64,
    pub scales: FacetScales,
    pub cell_annotations: Vec<AnnotationIR>,
}

#[derive(Clone)]
pub(crate) struct AnnotationIR {
    pub text: String,
    pub x: f64,
    pub y: f64,
    pub style: Option<Text>,
}
