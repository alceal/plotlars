#![allow(dead_code)]

use plotly::common::Line as LinePlotly;

use crate::converters::components as conv;
use plotlars_core::components::Line as LineStyle;

pub(crate) fn create_line(
    index: usize,
    width: Option<f64>,
    style: Option<LineStyle>,
    styles: Option<Vec<LineStyle>>,
) -> LinePlotly {
    let mut line = LinePlotly::new();
    line = set_width(line, width);
    line = set_style(line, style, styles, index);
    line
}

pub(crate) fn set_width(mut line: LinePlotly, width: Option<f64>) -> LinePlotly {
    if let Some(width) = width {
        line = line.width(width);
    }
    line
}

pub(crate) fn set_style(
    mut line: LinePlotly,
    style: Option<LineStyle>,
    styles: Option<Vec<LineStyle>>,
    index: usize,
) -> LinePlotly {
    if let Some(style) = style {
        line = line.dash(conv::convert_line(&style));
        return line;
    }

    if let Some(styles) = styles {
        if let Some(style) = styles.get(index) {
            line = line.dash(conv::convert_line(style));
        }
    }
    line
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_singular_style_priority() {
        let line = create_line(0, None, Some(LineStyle::Dash), Some(vec![LineStyle::Solid]));
        let json = serde_json::to_value(&line).unwrap();
        assert_eq!(json["dash"], "dash");
    }

    #[test]
    fn test_style_from_vec() {
        let line = create_line(1, None, None, Some(vec![LineStyle::Solid, LineStyle::Dot]));
        let json = serde_json::to_value(&line).unwrap();
        assert_eq!(json["dash"], "dot");
    }

    #[test]
    fn test_style_out_of_bounds() {
        let line = create_line(5, None, None, Some(vec![LineStyle::Solid]));
        let json = serde_json::to_value(&line).unwrap();
        assert!(json.get("dash").is_none());
    }

    #[test]
    fn test_width() {
        let line = create_line(0, Some(2.0), None, None);
        let json = serde_json::to_value(&line).unwrap();
        assert_eq!(json["width"], 2.0);
    }
}
