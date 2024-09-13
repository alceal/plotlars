use plotly::common::Marker;

use crate::{Rgb, Shape};

pub(crate) trait Mark {
    fn create_marker(mut opacity: Option<f64>, mut size: Option<usize>) -> Marker {
        if opacity.is_none() {
            opacity = Some(1.0);
        }

        if size.is_none() {
            size = Some(10);
        }

        marker!(opacity, size)
    }

    fn set_color(
        marker: &Marker,
        color: &Option<Rgb>,
        colors: &Option<Vec<Rgb>>,
        index: usize,
    ) -> Marker {
        let mut updated_marker = marker.clone();

        match color {
            Some(rgb) => {
                let group_color = plotly::color::Rgb::new(rgb.0, rgb.1, rgb.2);
                updated_marker = updated_marker.color(group_color);
            }
            None => {
                if let Some(colors) = colors {
                    if let Some(rgb) = colors.get(index) {
                        let group_color = plotly::color::Rgb::new(rgb.0, rgb.1, rgb.2);
                        updated_marker = updated_marker.color(group_color);
                    }
                }
            }
        }

        updated_marker
    }

    fn set_shape(
        marker: &Marker,
        shape: &Option<Shape>,
        shapes: &Option<Vec<Shape>>,
        index: usize,
    ) -> Marker {
        let mut updated_marker = marker.clone();

        match shape {
            Some(shape) => {
                updated_marker = updated_marker.symbol(shape.get_shape());
            }
            None => {
                if let Some(shapes) = shapes {
                    if let Some(shape) = shapes.get(index) {
                        updated_marker = updated_marker.symbol(shape.get_shape());
                    }
                }
            }
        }

        updated_marker
    }
}
