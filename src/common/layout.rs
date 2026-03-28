use plotly::layout::Annotation;
use plotly::Layout as LayoutPlotly;

use crate::components::{Axis, Dimensions, Legend, Text};

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
        dimensions: Option<&Dimensions>,
    ) -> LayoutPlotly {
        crate::plotly_conversions::layout::create_layout(
            plot_title,
            x_title,
            y_title,
            y2_title,
            z_title,
            legend_title,
            x_axis,
            y_axis,
            y2_axis,
            z_axis,
            legend,
            dimensions,
        )
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
        crate::plotly_conversions::layout::create_facet_annotations(categories, title_style)
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
}
