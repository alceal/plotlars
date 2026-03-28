use plotly::common::Marker as MarkerPlotly;

use crate::components::{Rgb, Shape};

#[allow(dead_code)]
pub(crate) trait Marker {
    fn create_marker(
        index: usize,
        opacity: Option<f64>,
        size: Option<usize>,
        color: Option<Rgb>,
        colors: Option<Vec<Rgb>>,
        shape: Option<Shape>,
        shapes: Option<Vec<Shape>>,
    ) -> MarkerPlotly {
        crate::plotly_conversions::marker::create_marker(
            index, opacity, size, color, colors, shape, shapes,
        )
    }

    fn set_opacity(marker: MarkerPlotly, opacity: Option<f64>) -> MarkerPlotly {
        crate::plotly_conversions::marker::set_opacity(marker, opacity)
    }

    fn set_size(marker: MarkerPlotly, size: Option<usize>) -> MarkerPlotly {
        crate::plotly_conversions::marker::set_size(marker, size)
    }

    fn set_color(
        marker: MarkerPlotly,
        color: Option<Rgb>,
        colors: Option<Vec<Rgb>>,
        index: usize,
    ) -> MarkerPlotly {
        crate::plotly_conversions::marker::set_color(marker, color, colors, index)
    }

    fn set_shape(
        marker: MarkerPlotly,
        shape: Option<Shape>,
        shapes: Option<Vec<Shape>>,
        index: usize,
    ) -> MarkerPlotly {
        crate::plotly_conversions::marker::set_shape(marker, shape, shapes, index)
    }
}
