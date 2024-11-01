use plotly::{
    common::Font,
    layout::{Axis as AxisPlotly, Legend as LegendPlotly},
    Layout as LayoutPlotly,
};

use crate::components::{Axis, Legend, Text};

#[allow(clippy::too_many_arguments)]
pub(crate) trait Layout {
    fn create_layout(
        plot_title: Option<Text>,
        x_title: Option<Text>,
        y_title: Option<Text>,
        legend_title: Option<Text>,
        x_axis: Option<&Axis>,
        y_axis: Option<&Axis>,
        legend: Option<&Legend>,
    ) -> LayoutPlotly {
        let mut layout = LayoutPlotly::new();

        if let Some(title) = plot_title {
            layout = layout.title(title.to_plotly());
        }

        layout = layout.x_axis(Self::set_axis(x_title, x_axis));
        layout = layout.y_axis(Self::set_axis(y_title, y_axis));
        layout = layout.legend(Self::set_legend(legend_title, legend));
        layout
    }

    // TODO:  Move axis functions to Axis struct like colorbar
    fn set_axis(title: Option<Text>, format: Option<&Axis>) -> AxisPlotly {
        let mut axis = AxisPlotly::new();

        if let Some(title) = title {
            axis = axis.title(title.to_plotly());
        }

        if let Some(format) = format {
            axis = Self::set_axis_format(axis, format);
        }

        axis
    }

    // TODO:  Move legend functions to Axis struct like colorbar
    fn set_legend(title: Option<Text>, format: Option<&Legend>) -> LegendPlotly {
        let mut legend = LegendPlotly::new();

        if let Some(title) = title {
            legend = legend.title(title.to_plotly());
        }

        if let Some(format) = format {
            legend = Self::set_legend_format(legend, format);
        }

        legend
    }

    fn set_axis_format(mut axis: AxisPlotly, format: &Axis) -> AxisPlotly {
        if let Some(visible) = format.show_axis {
            axis = axis.visible(visible.to_owned());
        }

        if let Some(axis_position) = &format.axis_side {
            axis = axis.side(axis_position.to_plotly());
        }

        if let Some(axis_type) = &format.axis_type {
            axis = axis.type_(axis_type.to_plotly());
        }

        if let Some(color) = format.value_color {
            axis = axis.color(color.to_plotly());
        }

        if let Some(range) = &format.value_range {
            axis = axis.range(range.to_owned());
        }

        if let Some(thousands) = format.value_thousands {
            axis = axis.separate_thousands(thousands.to_owned());
        }

        if let Some(exponent) = &format.value_exponent {
            axis = axis.exponent_format(exponent.to_plotly());
        }

        if let Some(range_values) = &format.tick_values {
            axis = axis.tick_values(range_values.to_owned());
        }

        if let Some(tick_text) = &format.tick_labels {
            axis = axis.tick_text(tick_text.to_owned());
        }

        if let Some(tick_direction) = &format.tick_direction {
            axis = axis.ticks(tick_direction.to_plotly_tickdirection());
        }

        if let Some(tick_length) = format.tick_length {
            axis = axis.tick_length(tick_length.to_owned());
        }

        if let Some(tick_width) = format.tick_width {
            axis = axis.tick_width(tick_width.to_owned());
        }

        if let Some(color) = format.tick_color {
            axis = axis.tick_color(color.to_plotly());
        }

        if let Some(tick_angle) = format.tick_angle {
            axis = axis.tick_angle(tick_angle.to_owned());
        }

        if let Some(font) = &format.tick_font {
            axis = axis.tick_font(Font::new().family(font.as_str()));
        }

        if let Some(show_line) = format.show_line {
            axis = axis.show_line(show_line.to_owned());
        }

        if let Some(color) = format.line_color {
            axis = axis.line_color(color.to_plotly());
        }

        if let Some(line_width) = format.line_width {
            axis = axis.line_width(line_width.to_owned());
        }

        if let Some(show_grid) = format.show_grid {
            axis = axis.show_grid(show_grid.to_owned());
        }

        if let Some(color) = format.grid_color {
            axis = axis.grid_color(color.to_plotly());
        }

        if let Some(grid_width) = format.grid_width {
            axis = axis.grid_width(grid_width.to_owned());
        }

        if let Some(show_zero_line) = format.show_zero_line {
            axis = axis.zero_line(show_zero_line.to_owned());
        }

        if let Some(color) = format.zero_line_color {
            axis = axis.zero_line_color(color.to_plotly());
        }

        if let Some(zero_line_width) = format.zero_line_width {
            axis = axis.zero_line_width(zero_line_width.to_owned());
        }

        if let Some(axis_position) = format.axis_position {
            axis = axis.position(axis_position.to_owned());
        }

        axis
    }

    fn set_legend_format(mut legend: LegendPlotly, format: &Legend) -> LegendPlotly {
        if let Some(color) = format.background_color {
            legend = legend.background_color(color.to_plotly());
        }

        if let Some(color) = format.border_color {
            legend = legend.border_color(color.to_plotly());
        }

        if let Some(width) = format.border_width {
            legend = legend.border_width(width);
        }

        if let Some(font) = &format.font {
            legend = legend.font(Font::new().family(font.as_str()));
        }

        if let Some(orientation) = &format.orientation {
            legend = legend.orientation(orientation.to_plotly());
        }

        if let Some(x) = format.x {
            legend = legend.x(x);
        }

        if let Some(y) = format.y {
            legend = legend.y(y);
        }

        legend
    }
}
