#![allow(dead_code)]

use plotly::common::Marker as MarkerPlotly;

use crate::converters::components as conv;
use plotlars_core::components::{Rgb, Shape};

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
        marker = marker.symbol(conv::convert_shape(&shape));
        return marker;
    }

    if let Some(shapes) = shapes {
        if let Some(shape) = shapes.get(index) {
            marker = marker.symbol(conv::convert_shape(shape));
        }
    }
    marker
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_singular_color_priority() {
        let marker = create_marker(
            0,
            None,
            None,
            Some(Rgb(255, 0, 0)),
            Some(vec![Rgb(0, 0, 255)]),
            None,
            None,
        );
        let json = serde_json::to_value(&marker).unwrap();
        assert_eq!(json["color"], "rgb(255, 0, 0)");
    }

    #[test]
    fn test_color_from_vec() {
        let marker = create_marker(
            1,
            None,
            None,
            None,
            Some(vec![Rgb(255, 0, 0), Rgb(0, 255, 0), Rgb(0, 0, 255)]),
            None,
            None,
        );
        let json = serde_json::to_value(&marker).unwrap();
        assert_eq!(json["color"], "rgb(0, 255, 0)");
    }

    #[test]
    fn test_color_out_of_bounds() {
        let marker = create_marker(5, None, None, None, Some(vec![Rgb(255, 0, 0)]), None, None);
        let json = serde_json::to_value(&marker).unwrap();
        assert!(json.get("color").is_none());
    }

    #[test]
    fn test_singular_shape_priority() {
        let marker = create_marker(
            0,
            None,
            None,
            None,
            None,
            Some(Shape::Circle),
            Some(vec![Shape::Square]),
        );
        let json = serde_json::to_value(&marker).unwrap();
        assert_eq!(json["symbol"], "circle");
    }

    #[test]
    fn test_opacity_and_size() {
        let marker = create_marker(0, Some(0.7), Some(12), None, None, None, None);
        let json = serde_json::to_value(&marker).unwrap();
        assert_eq!(json["opacity"], 0.7);
        assert_eq!(json["size"], 12);
    }
}
