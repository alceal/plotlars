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

        if let Some(x_axis) = x_axis {
            layout = layout.x_axis(Axis::set_axis(x_title, x_axis, None));
        }

        if let Some(y_axis ) = y_axis {
            layout = layout.y_axis(Axis::set_axis(y_title, y_axis, None));
        }

        if let Some(y2_axis) = y2_axis {
            layout = layout.y_axis2(Axis::set_axis(y2_title, y2_axis, Some("y")));
        }

        if let Some(z_axis) = z_axis {
            layout = layout.z_axis(Axis::set_axis(z_title, z_axis, None));
        }

        layout = layout.legend(Legend::set_legend(legend_title, legend));
        layout
    }
}
