use plotly::Layout as LayoutPlotly;

use crate::components::{Axis, Legend, Text};

#[allow(clippy::too_many_arguments)]
pub(crate) trait Layout {
    fn create_layout(
        plot_title: Option<Text>,
        x_title: Option<Text>,
        y_title: Option<Text>,
        y2_title: Option<Text>,
        z_title: Option<Text>,
        legend_title: Option<Text>,
        x_axis: Option<&Axis>,
        y_axis: Option<&Axis>,
        y2_axis: Option<&Axis>,
        z_axis: Option<&Axis>,
        legend: Option<&Legend>,
    ) -> LayoutPlotly {
        let mut layout = LayoutPlotly::new();

        if let Some(title) = plot_title {
            layout = layout.title(title.to_plotly());
        }

        match (x_axis, x_title) {
            (Some(axis), title) => {
                layout = layout.x_axis(Axis::set_axis(title, axis, None));
            }
            (None, Some(title)) => {
                let default_axis = Axis::default();
                layout = layout.x_axis(Axis::set_axis(Some(title), &default_axis, None));
            }
            _ => {}
        }

        match (y_axis, y_title) {
            (Some(axis), title) => {
                layout = layout.y_axis(Axis::set_axis(title, axis, None));
            }
            (None, Some(title)) => {
                let default_axis = Axis::default();
                layout = layout.y_axis(Axis::set_axis(Some(title), &default_axis, None));
            }
            _ => {}
        }

        // Handle y2-axis
        if let Some(y2_axis) = y2_axis {
            layout = layout.y_axis2(Axis::set_axis(y2_title, y2_axis, Some("y")));
        }

        // Handle z-axis: use provided axis or create default with title if only title exists
        match (z_axis, z_title) {
            (Some(axis), title) => {
                layout = layout.z_axis(Axis::set_axis(title, axis, None));
            }
            (None, Some(title)) => {
                let default_axis = Axis::default();
                layout = layout.z_axis(Axis::set_axis(Some(title), &default_axis, None));
            }
            _ => {}
        }

        layout = layout.legend(Legend::set_legend(legend_title, legend));
        layout
    }
}
