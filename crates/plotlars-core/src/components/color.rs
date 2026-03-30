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

pub fn parse_color(color_str: &str) -> Option<Rgb> {
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

pub const DEFAULT_PLOTLY_COLORS: [Rgb; 10] = [
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
    fn test_parse_hex6() {
        assert_rgb(parse_color("#ff8000"), 255, 128, 0);
    }

    #[test]
    fn test_parse_hex6_uppercase() {
        assert_rgb(parse_color("#FF0000"), 255, 0, 0);
    }

    #[test]
    fn test_parse_hex3() {
        assert_rgb(parse_color("#f00"), 255, 0, 0);
    }

    #[test]
    fn test_parse_hex_invalid_length() {
        assert!(parse_color("#abcd").is_none());
    }

    #[test]
    fn test_parse_hex_invalid_chars() {
        assert!(parse_color("#xyz").is_none());
    }

    #[test]
    fn test_parse_named_colors() {
        assert_rgb(parse_color("black"), 0, 0, 0);
        assert_rgb(parse_color("white"), 255, 255, 255);
        assert_rgb(parse_color("red"), 255, 0, 0);
    }

    #[test]
    fn test_parse_named_case_insensitive() {
        assert_rgb(parse_color("BLUE"), 0, 0, 255);
        assert_rgb(parse_color("Blue"), 0, 0, 255);
    }

    #[test]
    fn test_parse_grey_alias() {
        let grey = parse_color("grey").expect("expected Some");
        let gray = parse_color("gray").expect("expected Some");
        assert_eq!(grey.0, gray.0);
        assert_eq!(grey.1, gray.1);
        assert_eq!(grey.2, gray.2);
        assert_rgb(parse_color("grey"), 128, 128, 128);
    }

    #[test]
    fn test_parse_unknown_name() {
        assert!(parse_color("chartreuse").is_none());
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
