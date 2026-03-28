#![allow(dead_code)]

use plotly::common::Marker as MarkerPlotly;

use crate::components::{Rgb, Shape};

pub(crate) fn create_marker(
    index: usize,
    opacity: Option<f64>,
    size: Option<usize>,
    color: Option<Rgb>,
    colors: Option<Vec<Rgb>>,
    shape: Option<Shape>,
    shapes: Option<Vec<Shape>>,
) -> MarkerPlotly {
    let mut marker = MarkerPlotly::new();
    marker = set_opacity(marker, opacity);
    marker = set_size(marker, size);
    marker = set_color(marker, color, colors, index);
    marker = set_shape(marker, shape, shapes, index);
    marker
}

pub(crate) fn set_opacity(mut marker: MarkerPlotly, opacity: Option<f64>) -> MarkerPlotly {
    if let Some(opacity) = opacity {
        marker = marker.opacity(opacity);
    }
    marker
}

pub(crate) fn set_size(mut marker: MarkerPlotly, size: Option<usize>) -> MarkerPlotly {
    if let Some(size) = size {
        marker = marker.size(size);
    }
    marker
}

pub(crate) fn set_color(
    mut marker: MarkerPlotly,
    color: Option<Rgb>,
    colors: Option<Vec<Rgb>>,
    index: usize,
) -> MarkerPlotly {
    if let Some(rgb) = color {
        let color = plotly::color::Rgb::new(rgb.0, rgb.1, rgb.2);
        marker = marker.color(color);
        return marker;
    }

    if let Some(colors) = colors {
        if let Some(rgb) = colors.get(index) {
            let group_color = plotly::color::Rgb::new(rgb.0, rgb.1, rgb.2);
            marker = marker.color(group_color);
        }
    }
    marker
}

pub(crate) fn set_shape(
    mut marker: MarkerPlotly,
    shape: Option<Shape>,
    shapes: Option<Vec<Shape>>,
    index: usize,
) -> MarkerPlotly {
    if let Some(shape) = shape {
        marker = marker.symbol(shape.to_plotly());
        return marker;
    }

    if let Some(shapes) = shapes {
        if let Some(shape) = shapes.get(index) {
            marker = marker.symbol(shape.to_plotly());
        }
    }
    marker
}
