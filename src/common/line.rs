use plotly::common::Line as LinePlotly;

use crate::components::Line as LineStyle;

pub(crate) trait Line {
    fn create_line(
        index: usize,
        width: Option<f64>,
        style: Option<LineStyle>,
        styles: Option<Vec<LineStyle>>,
    ) -> LinePlotly {
        let mut line = LinePlotly::new();
        line = Self::set_width(line, width);
        line = Self::set_style(line, style, styles, index);
        line
    }

    fn set_width(mut line: LinePlotly, width: Option<f64>) -> LinePlotly {
        if let Some(width) = width {
            line = line.width(width);
        }

        line
    }

    fn set_style(
        mut line: LinePlotly,
        style: Option<LineStyle>,
        styles: Option<Vec<LineStyle>>,
        index: usize,
    ) -> LinePlotly {
        if let Some(style) = style {
            line = line.dash(style.to_plotly());
            return line;
        }

        if let Some(styles) = styles {
            if let Some(style) = styles.get(index) {
                line = line.dash(style.to_plotly());
            }
        }

        line
    }
}
