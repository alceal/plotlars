use plotly::color::{Color, Rgb as RgbPlotly};

use serde::Serialize;

/// A structure representing an RGB color with red, green, and blue components.
///
/// # Example
///
/// ```rust
/// use polars::prelude::*;
/// use plotlars::{Axis, BarPlot, Legend, Orientation, Plot, Rgb};
///
/// let dataset = df![
///         "label" => &["", "", ""],
///         "color" => &["red", "green", "blue"],
///         "value" => &[1, 1, 1],
///     ]
///     .unwrap();
///
/// let axis = Axis::new()
///     .show_axis(false);
///
/// let legend = Legend::new()
///     .orientation(Orientation::Horizontal)
///     .x(0.3);
///
/// BarPlot::builder()
///     .data(&dataset)
///     .labels("label")
///     .values("value")
///     .group("color")
///     .colors(vec![
///         Rgb(255, 0, 0),
///         Rgb(0, 255, 0),
///         Rgb(0, 0, 255),
///     ])
///     .x_axis(&axis)
///     .y_axis(&axis)
///     .legend(&legend)
///     .build()
///     .plot();
/// ```
/// ![example](https://imgur.com/HPmtj9I.png)
#[derive(Debug, Default, Clone, Copy, Serialize)]
pub struct Rgb(
    /// Red component
    pub u8,
    /// Green component
    pub u8,
    /// Blue component
    pub u8,
);

impl Rgb {
    #[allow(clippy::wrong_self_convention)]
    pub(crate) fn to_plotly(&self) -> RgbPlotly {
        RgbPlotly::new(self.0, self.1, self.2)
    }
}

impl Color for Rgb {}

pub(crate) fn parse_color(color_str: &str) -> Option<Rgb> {
    if color_str.starts_with("rgb(") || color_str.starts_with("rgba(") {
        let start = color_str.find('(')?;
        let end = color_str.find(')')?;
        let values = &color_str[start + 1..end];
        let parts: Vec<&str> = values.split(',').map(|s| s.trim()).collect();

        if parts.len() >= 3 {
            let r = parts[0].parse::<u8>().ok()?;
            let g = parts[1].parse::<u8>().ok()?;
            let b = parts[2].parse::<u8>().ok()?;
            return Some(Rgb(r, g, b));
        }
    }

    if let Some(hex) = color_str.strip_prefix('#') {
        if hex.len() == 6 {
            let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
            return Some(Rgb(r, g, b));
        } else if hex.len() == 3 {
            let r = u8::from_str_radix(&hex[0..1], 16).ok()? * 17;
            let g = u8::from_str_radix(&hex[1..2], 16).ok()? * 17;
            let b = u8::from_str_radix(&hex[2..3], 16).ok()? * 17;
            return Some(Rgb(r, g, b));
        }
    }

    match color_str.to_lowercase().as_str() {
        "black" => Some(Rgb(0, 0, 0)),
        "white" => Some(Rgb(255, 255, 255)),
        "red" => Some(Rgb(255, 0, 0)),
        "green" => Some(Rgb(0, 128, 0)),
        "blue" => Some(Rgb(0, 0, 255)),
        "yellow" => Some(Rgb(255, 255, 0)),
        "cyan" => Some(Rgb(0, 255, 255)),
        "magenta" => Some(Rgb(255, 0, 255)),
        "gray" | "grey" => Some(Rgb(128, 128, 128)),
        "orange" => Some(Rgb(255, 165, 0)),
        "purple" => Some(Rgb(128, 0, 128)),
        "pink" => Some(Rgb(255, 192, 203)),
        "brown" => Some(Rgb(165, 42, 42)),
        "lime" => Some(Rgb(0, 255, 0)),
        "navy" => Some(Rgb(0, 0, 128)),
        "teal" => Some(Rgb(0, 128, 128)),
        "silver" => Some(Rgb(192, 192, 192)),
        "maroon" => Some(Rgb(128, 0, 0)),
        "olive" => Some(Rgb(128, 128, 0)),
        _ => None,
    }
}

pub(crate) const DEFAULT_PLOTLY_COLORS: [Rgb; 10] = [
    Rgb(99, 110, 250),
    Rgb(239, 85, 59),
    Rgb(0, 204, 150),
    Rgb(171, 99, 250),
    Rgb(255, 161, 90),
    Rgb(25, 211, 243),
    Rgb(255, 102, 146),
    Rgb(182, 232, 128),
    Rgb(255, 151, 255),
    Rgb(254, 203, 82),
];
