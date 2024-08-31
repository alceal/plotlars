use plotly::{
    color::Rgb,
    common::{Font, Title},
    layout::{Axis, BarMode, BoxMode, Legend},
    Layout,
};

use crate::Text;

pub(crate) trait LayoutPlotly {
    fn create_layout(
        bar_mode: Option<BarMode>,
        plot_title: Option<Text>,
        x_title: Option<Text>,
        y_title: Option<Text>,
        legend_title: Option<Text>,
    ) -> Layout {
        let mut layout = Layout::new().box_mode(BoxMode::Group);

        if let Some(mode) = bar_mode {
            layout = layout.bar_mode(mode);
        }

        if let Some(title) = plot_title {
            layout = layout.title(Self::create_title(title));
        }

        if let Some(title) = x_title {
            layout = layout.x_axis(Axis::new().title(Self::create_title(title)));
        }

        if let Some(title) = y_title {
            layout = layout.y_axis(Axis::new().title(Self::create_title(title)));
        }

        if let Some(title) = legend_title {
            layout = layout.legend(Legend::new().title(Self::create_title(title)));
        }

        layout
    }

    fn create_title(title_details: Text) -> Title {
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
