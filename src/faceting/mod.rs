use plotly::layout::{Axis as AxisPlotly, GridPattern, Layout as LayoutPlotly, LayoutGrid};

use crate::common::{Layout, Polar};
use crate::components::facet::{FacetConfig, FacetScales};
use crate::components::{Axis, Legend, Text};

use polars::frame::DataFrame;

fn is_bottom_row(subplot_index: usize, ncols: usize, nrows: usize, n_facets: usize) -> bool {
    let row = subplot_index / ncols;
    let index_below = subplot_index + ncols;
    row == nrows - 1 || index_below >= n_facets
}

fn is_left_column(subplot_index: usize, ncols: usize) -> bool {
    subplot_index % ncols == 0
}

/// Shared axis-based faceted layout builder for 2D Cartesian plots.
///
/// Returns a layout with grid, axis matching, axis titles, annotations, and legend.
/// The caller can apply additional post-layout modifications (e.g., bar_mode, box_mode)
/// to the returned layout.
#[allow(clippy::too_many_arguments)]
pub(crate) fn create_axis_faceted_layout<T: Layout + Polar>(
    data: &DataFrame,
    facet_column: &str,
    config: &FacetConfig,
    plot_title: Option<Text>,
    x_title: Option<Text>,
    y_title: Option<Text>,
    legend_title: Option<Text>,
    x_axis: Option<&Axis>,
    y_axis: Option<&Axis>,
    legend: Option<&Legend>,
) -> LayoutPlotly {
    let facet_categories = T::get_unique_groups(data, facet_column, config.sorter);
    let n_facets = facet_categories.len();

    let (ncols, nrows) = T::calculate_grid_dimensions(n_facets, config.cols, config.rows);

    let mut grid = LayoutGrid::new()
        .rows(nrows)
        .columns(ncols)
        .pattern(GridPattern::Independent);

    if let Some(x_gap) = config.h_gap {
        grid = grid.x_gap(x_gap);
    }
    if let Some(y_gap) = config.v_gap {
        grid = grid.y_gap(y_gap);
    }

    let mut layout = LayoutPlotly::new().grid(grid);

    if let Some(title) = plot_title {
        layout = layout.title(title.to_plotly());
    }

    layout = apply_axis_matching(layout, n_facets, &config.scales);

    layout = apply_facet_axis_titles(
        layout, n_facets, ncols, nrows, x_title, y_title, x_axis, y_axis,
    );

    let annotations = T::create_facet_annotations(&facet_categories, config.title_style.as_ref());
    layout = layout.annotations(annotations);

    layout = layout.legend(Legend::set_legend(legend_title, legend));

    layout
}

/// Applies axis scale matching based on FacetScales configuration.
///
/// For Fixed: all subplot axes match the first subplot's axes.
/// For FreeX: only y-axes match (x-axes are independent).
/// For FreeY: only x-axes match (y-axes are independent).
/// For Free: no matching (all axes independent).
pub(crate) fn apply_axis_matching(
    mut layout: LayoutPlotly,
    n_facets: usize,
    scales: &FacetScales,
) -> LayoutPlotly {
    match scales {
        FacetScales::Fixed => {
            for i in 1..n_facets {
                let x_axis = AxisPlotly::new().matches("x");
                let y_axis = AxisPlotly::new().matches("y");
                layout = match i {
                    1 => layout.x_axis2(x_axis).y_axis2(y_axis),
                    2 => layout.x_axis3(x_axis).y_axis3(y_axis),
                    3 => layout.x_axis4(x_axis).y_axis4(y_axis),
                    4 => layout.x_axis5(x_axis).y_axis5(y_axis),
                    5 => layout.x_axis6(x_axis).y_axis6(y_axis),
                    6 => layout.x_axis7(x_axis).y_axis7(y_axis),
                    7 => layout.x_axis8(x_axis).y_axis8(y_axis),
                    _ => layout,
                };
            }
        }
        FacetScales::FreeX => {
            for i in 1..n_facets {
                let axis = AxisPlotly::new().matches("y");
                layout = match i {
                    1 => layout.y_axis2(axis),
                    2 => layout.y_axis3(axis),
                    3 => layout.y_axis4(axis),
                    4 => layout.y_axis5(axis),
                    5 => layout.y_axis6(axis),
                    6 => layout.y_axis7(axis),
                    7 => layout.y_axis8(axis),
                    _ => layout,
                };
            }
        }
        FacetScales::FreeY => {
            for i in 1..n_facets {
                let axis = AxisPlotly::new().matches("x");
                layout = match i {
                    1 => layout.x_axis2(axis),
                    2 => layout.x_axis3(axis),
                    3 => layout.x_axis4(axis),
                    4 => layout.x_axis5(axis),
                    5 => layout.x_axis6(axis),
                    6 => layout.x_axis7(axis),
                    7 => layout.x_axis8(axis),
                    _ => layout,
                };
            }
        }
        FacetScales::Free => {}
    }

    layout
}

/// Applies axis titles and configurations to each subplot in the facet grid.
///
/// X-axis titles only appear on bottom row subplots.
/// Y-axis titles only appear on left column subplots.
#[allow(clippy::too_many_arguments)]
pub(crate) fn apply_facet_axis_titles(
    mut layout: LayoutPlotly,
    n_facets: usize,
    ncols: usize,
    nrows: usize,
    x_title: Option<Text>,
    y_title: Option<Text>,
    x_axis_config: Option<&Axis>,
    y_axis_config: Option<&Axis>,
) -> LayoutPlotly {
    for i in 0..n_facets {
        let is_bottom = is_bottom_row(i, ncols, nrows, n_facets);
        let is_left = is_left_column(i, ncols);

        let x_title_for_subplot = if is_bottom { x_title.clone() } else { None };
        let y_title_for_subplot = if is_left { y_title.clone() } else { None };

        if x_title_for_subplot.is_some() || x_axis_config.is_some() {
            let axis = match x_axis_config {
                Some(config) => Axis::set_axis(x_title_for_subplot, config, None),
                None => {
                    if let Some(title) = x_title_for_subplot {
                        Axis::set_axis(Some(title), &Axis::default(), None)
                    } else {
                        continue;
                    }
                }
            };

            layout = match i {
                0 => layout.x_axis(axis),
                1 => layout.x_axis2(axis),
                2 => layout.x_axis3(axis),
                3 => layout.x_axis4(axis),
                4 => layout.x_axis5(axis),
                5 => layout.x_axis6(axis),
                6 => layout.x_axis7(axis),
                7 => layout.x_axis8(axis),
                _ => layout,
            };
        }

        if y_title_for_subplot.is_some() || y_axis_config.is_some() {
            let axis = match y_axis_config {
                Some(config) => Axis::set_axis(y_title_for_subplot, config, None),
                None => {
                    if let Some(title) = y_title_for_subplot {
                        Axis::set_axis(Some(title), &Axis::default(), None)
                    } else {
                        continue;
                    }
                }
            };

            layout = match i {
                0 => layout.y_axis(axis),
                1 => layout.y_axis2(axis),
                2 => layout.y_axis3(axis),
                3 => layout.y_axis4(axis),
                4 => layout.y_axis5(axis),
                5 => layout.y_axis6(axis),
                6 => layout.y_axis7(axis),
                7 => layout.y_axis8(axis),
                _ => layout,
            };
        }
    }

    layout
}
