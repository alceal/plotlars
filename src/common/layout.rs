use plotly::common::Anchor;
use plotly::layout::Annotation;
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
        let mut annotations = Vec::new();

        if let Some(title) = plot_title {
            layout = layout.title(title.to_plotly());
        }

        let (x_title_for_axis, x_annotation) =
            Self::split_title_or_annotation(x_title, true, "x", false);

        match (x_axis, x_title_for_axis) {
            (Some(axis), title) => {
                layout = layout.x_axis(Axis::set_axis(title, axis, None));
            }
            (None, Some(title)) => {
                let default_axis = Axis::default();
                layout = layout.x_axis(Axis::set_axis(Some(title), &default_axis, None));
            }
            _ => {}
        }

        if let Some(ann) = x_annotation {
            annotations.push(ann);
        }

        let (y_title_for_axis, y_annotation) =
            Self::split_title_or_annotation(y_title, false, "y", false);

        match (y_axis, y_title_for_axis) {
            (Some(axis), title) => {
                layout = layout.y_axis(Axis::set_axis(title, axis, None));
            }
            (None, Some(title)) => {
                let default_axis = Axis::default();
                layout = layout.y_axis(Axis::set_axis(Some(title), &default_axis, None));
            }
            _ => {}
        }

        if let Some(ann) = y_annotation {
            annotations.push(ann);
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

        if !annotations.is_empty() {
            layout = layout.annotations(annotations);
        }

        layout
    }

    fn calculate_grid_dimensions(
        n_facets: usize,
        cols: Option<usize>,
        rows: Option<usize>,
    ) -> (usize, usize) {
        match (cols, rows) {
            (Some(c), Some(r)) => {
                if c * r < n_facets {
                    panic!("Grid dimensions {}x{} cannot fit {} facets", c, r, n_facets);
                }
                (c, r)
            }
            (Some(c), None) => {
                let r = n_facets.div_ceil(c);
                (c, r)
            }
            (None, Some(r)) => {
                let c = n_facets.div_ceil(r);
                (c, r)
            }
            (None, None) => {
                let c = (n_facets as f64).sqrt().ceil() as usize;
                let r = n_facets.div_ceil(c);
                (c, r)
            }
        }
    }

    fn create_facet_annotations(
        categories: &[String],
        title_style: Option<&Text>,
    ) -> Vec<Annotation> {
        categories
            .iter()
            .enumerate()
            .map(|(i, cat)| {
                let x_ref = if i == 0 {
                    "x domain".to_string()
                } else {
                    format!("x{} domain", i + 1)
                };
                let y_ref = if i == 0 {
                    "y domain".to_string()
                } else {
                    format!("y{} domain", i + 1)
                };

                let mut ann = Annotation::new()
                    .text(cat.as_str())
                    .x_ref(&x_ref)
                    .y_ref(&y_ref)
                    .x_anchor(Anchor::Center)
                    .y_anchor(Anchor::Bottom)
                    .x(0.5)
                    .y(1.0)
                    .show_arrow(false);

                if let Some(style) = title_style {
                    ann = ann.font(style.to_font());
                }

                ann
            })
            .collect()
    }

    fn get_axis_reference(subplot_index: usize, axis_type: &str) -> String {
        if subplot_index == 0 {
            axis_type.to_string()
        } else {
            format!("{}{}", axis_type, subplot_index + 1)
        }
    }

    fn is_bottom_row(subplot_index: usize, ncols: usize, nrows: usize, n_facets: usize) -> bool {
        let row = subplot_index / ncols;
        let index_below = subplot_index + ncols;

        row == nrows - 1 || index_below >= n_facets
    }

    fn is_left_column(subplot_index: usize, ncols: usize) -> bool {
        let col = subplot_index % ncols;
        col == 0
    }

    fn split_title_or_annotation(
        title: Option<Text>,
        is_x_axis: bool,
        axis_ref: &str,
        use_domain: bool,
    ) -> (Option<Text>, Option<Annotation>) {
        if let Some(text) = title {
            if text.has_custom_position() {
                let annotation = text.to_axis_annotation(is_x_axis, axis_ref, use_domain);
                (None, Some(annotation))
            } else {
                (Some(text), None)
            }
        } else {
            (None, None)
        }
    }
}
