use crate::components::{Rgb, Shape};

#[derive(Clone)]
#[doc(hidden)]
pub struct MarkerIR {
    pub opacity: Option<f64>,
    pub size: Option<usize>,
    pub color: Option<Rgb>,
    pub shape: Option<Shape>,
}
