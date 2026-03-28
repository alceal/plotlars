use plotly::common::Line as LinePlotly;

use crate::components::Line as LineStyle;

#[allow(dead_code)]
pub(crate) trait Line {
    fn create_line(
        index: usize,
        width: Option<f64>,
        style: Option<LineStyle>,
        styles: Option<Vec<LineStyle>>,
    ) -> LinePlotly {
        crate::plotly_conversions::line::create_line(index, width, style, styles)
    }

    fn set_width(line: LinePlotly, width: Option<f64>) -> LinePlotly {
        crate::plotly_conversions::line::set_width(line, width)
    }

    fn set_style(
        line: LinePlotly,
        style: Option<LineStyle>,
        styles: Option<Vec<LineStyle>>,
        index: usize,
    ) -> LinePlotly {
        crate::plotly_conversions::line::set_style(line, style, styles, index)
    }
}
