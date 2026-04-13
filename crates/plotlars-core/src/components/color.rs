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

#[doc(hidden)]
pub fn parse_color(color_str: &str) -> Option<Rgb> {
    let start = color_str.find('(')?;
    let end = color_str.find(')')?;
    let parts: Vec<&str> = color_str[start + 1..end]
        .split(',')
        .map(|s| s.trim())
        .collect();

    if parts.len() >= 3 {
        let r = parts[0].parse::<u8>().ok()?;
        let g = parts[1].parse::<u8>().ok()?;
        let b = parts[2].parse::<u8>().ok()?;
        return Some(Rgb(r, g, b));
    }

    None
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

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_rgb(actual: Option<Rgb>, r: u8, g: u8, b: u8) {
        let c = actual.expect("expected Some(Rgb)");
        assert_eq!(c.0, r);
        assert_eq!(c.1, g);
        assert_eq!(c.2, b);
    }

    #[test]
    fn test_parse_rgb_basic() {
        assert_rgb(parse_color("rgb(255, 0, 128)"), 255, 0, 128);
    }

    #[test]
    fn test_parse_rgb_whitespace() {
        assert_rgb(parse_color("rgb( 128 , 64 , 32 )"), 128, 64, 32);
    }

    #[test]
    fn test_parse_rgba_ignores_alpha() {
        assert_rgb(parse_color("rgba(10, 20, 30, 0.5)"), 10, 20, 30);
    }

    #[test]
    fn test_parse_rgb_overflow() {
        assert!(parse_color("rgb(256, 0, 0)").is_none());
    }

    #[test]
    fn test_parse_rgb_too_few_parts() {
        assert!(parse_color("rgb(0, 0)").is_none());
    }

    #[test]
    fn test_parse_no_parens() {
        assert!(parse_color("black").is_none());
    }

    #[test]
    fn test_parse_empty_string() {
        assert!(parse_color("").is_none());
    }

    #[test]
    fn test_rgb_default() {
        let d = Rgb::default();
        assert_eq!(d.0, 0);
        assert_eq!(d.1, 0);
        assert_eq!(d.2, 0);
    }

    #[test]
    fn test_rgb_copy() {
        let a = Rgb(1, 2, 3);
        let b = a;
        assert_eq!(a.0, b.0);
        assert_eq!(a.1, b.1);
        assert_eq!(a.2, b.2);
    }

    #[test]
    fn test_default_plotly_colors_len() {
        assert_eq!(DEFAULT_PLOTLY_COLORS.len(), 10);
    }
}
