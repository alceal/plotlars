use plotly::common::Marker;

use crate::Rgb;

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

    fn set_color(marker: &Marker, colors: &Option<Vec<Rgb>>, index: usize) -> Marker {
        let mut updated_marker = marker.clone();

        if let Some(colors) = colors {
            if let Some(rgb) = colors.get(index) {
                let group_color = plotly::color::Rgb::new(rgb.0, rgb.1, rgb.2);
                updated_marker = updated_marker.color(group_color);
            }
        }

        updated_marker
    }
}
