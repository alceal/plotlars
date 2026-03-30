use crate::components::Line as LineStyle;
use crate::components::Rgb;

#[derive(Clone)]
#[doc(hidden)]
pub struct LineIR {
    pub width: Option<f64>,
    pub style: Option<LineStyle>,
    pub color: Option<Rgb>,
}
