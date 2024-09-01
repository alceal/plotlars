use plotly::{
    color::Rgb,
    common::{Font, Title},
    layout::{Axis as AxisPlotly, BarMode, BoxMode, Legend},
    Layout,
};

use crate::{Axis, Text};

pub(crate) trait LayoutPlotly {
    fn create_layout(
        bar_mode: Option<BarMode>,
        plot_title: Option<Text>,
        x_title: Option<Text>,
        x_axis: Option<&Axis>,
        y_title: Option<Text>,
        y_axis: Option<&Axis>,
        legend_title: Option<Text>,
    ) -> Layout {
        let mut layout = Layout::new().box_mode(BoxMode::Group);
        let mut x_axis_format = AxisPlotly::new();
        let mut y_axis_format = AxisPlotly::new();

        if let Some(mode) = bar_mode {
            layout = layout.bar_mode(mode);
        }

        if let Some(title) = plot_title {
            layout = layout.title(Self::set_title(title));
        }

        if let Some(title) = x_title {
            x_axis_format = x_axis_format.title(Self::set_title(title));
        }

        if let Some(axis_details) = x_axis {
            x_axis_format = Self::set_axis_format(x_axis_format, axis_details.into());
        }

        layout = layout.x_axis(x_axis_format);

        if let Some(title) = y_title {
            y_axis_format = y_axis_format.title(Self::set_title(title));
        }

        if let Some(axis_details) = y_axis {
            y_axis_format = Self::set_axis_format(y_axis_format, axis_details.clone());
        }

        layout = layout.y_axis(y_axis_format);

        if let Some(title) = legend_title {
            layout = layout.legend(Legend::new().title(Self::set_title(title)));
        }

        layout
    }

    fn set_axis_format(mut x_axis_format: AxisPlotly, axis_details: Axis) -> AxisPlotly {
        if let Some(visible) = axis_details.show_axis {
            x_axis_format = x_axis_format.visible(visible);
        }

        if let Some(axis_position) = axis_details.axis_position {
            x_axis_format = x_axis_format.side(axis_position.get_position());
        }

        if let Some(axis_type) = axis_details.axis_type {
            x_axis_format = x_axis_format.type_(axis_type.get_type());
        }

        if let Some(color) = axis_details.value_color {
            x_axis_format = x_axis_format.color(Rgb::new(color.0, color.1, color.2));
        }

        if let Some(range) = axis_details.value_range {
            x_axis_format = x_axis_format.range(range);
        }

        if let Some(thousands) = axis_details.value_thousands {
            x_axis_format = x_axis_format.separate_thousands(thousands);
        }

        if let Some(exponent) = axis_details.value_exponent {
            x_axis_format = x_axis_format.exponent_format(exponent.get_exponent());
        }

        if let Some(range_values) = axis_details.tick_values {
            x_axis_format = x_axis_format.tick_values(range_values);
        }

        if let Some(tick_text) = axis_details.tick_labels {
            x_axis_format = x_axis_format.tick_text(tick_text);
        }

        if let Some(tick_direction) = axis_details.tick_direction {
            x_axis_format = x_axis_format.ticks(tick_direction.get_direction());
        }

        if let Some(tick_length) = axis_details.tick_length {
            x_axis_format = x_axis_format.tick_length(tick_length);
        }

        if let Some(tick_width) = axis_details.tick_width {
            x_axis_format = x_axis_format.tick_width(tick_width);
        }

        if let Some(tick_color) = axis_details.tick_color {
            x_axis_format =
                x_axis_format.tick_color(Rgb::new(tick_color.0, tick_color.1, tick_color.2));
        }

        if let Some(tick_angle) = axis_details.tick_angle {
            x_axis_format = x_axis_format.tick_angle(tick_angle);
        }

        if let Some(font) = axis_details.tick_font {
            x_axis_format = x_axis_format.tick_font(Font::new().family(font.as_str()));
        }

        if let Some(show_line) = axis_details.show_line {
            x_axis_format = x_axis_format.show_line(show_line);
        }

        if let Some(line_color) = axis_details.line_color {
            x_axis_format =
                x_axis_format.line_color(Rgb::new(line_color.0, line_color.1, line_color.2));
        }

        if let Some(line_width) = axis_details.line_width {
            x_axis_format = x_axis_format.line_width(line_width);
        }

        if let Some(show_grid) = axis_details.show_grid {
            x_axis_format = x_axis_format.show_grid(show_grid);
        }

        if let Some(grid_color) = axis_details.grid_color {
            x_axis_format =
                x_axis_format.grid_color(Rgb::new(grid_color.0, grid_color.1, grid_color.2));
        }

        if let Some(grid_width) = axis_details.grid_width {
            x_axis_format = x_axis_format.grid_width(grid_width);
        }

        if let Some(show_zero_line) = axis_details.show_zero_line {
            x_axis_format = x_axis_format.zero_line(show_zero_line);
        }

        if let Some(zero_line_color) = axis_details.zero_line_color {
            x_axis_format = x_axis_format.zero_line_color(Rgb::new(
                zero_line_color.0,
                zero_line_color.1,
                zero_line_color.2,
            ));
        }

        if let Some(zero_line_width) = axis_details.zero_line_width {
            x_axis_format = x_axis_format.zero_line_width(zero_line_width);
        }

        x_axis_format
    }

    fn set_title(title_details: Text) -> Title {
        Title::with_text(title_details.content)
            .font(
                Font::new()
                    .family(title_details.font.as_str())
                    .size(title_details.size)
                    .color(Rgb::new(
                        title_details.color.0,
                        title_details.color.1,
                        title_details.color.2,
                    )),
            )
            .x(title_details.x)
            .y(title_details.y)
    }
}
