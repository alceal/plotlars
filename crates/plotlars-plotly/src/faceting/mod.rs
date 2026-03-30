use plotly::common::Anchor;
use plotly::layout::{
    Annotation, Axis as AxisPlotly, GridPattern, Layout as LayoutPlotly, LayoutGrid,
};

use crate::converters::components as conv;
use plotlars_core::components::facet::FacetScales;
use plotlars_core::components::{Axis, Text};
use plotlars_core::ir::facet::GridSpec;
use plotlars_core::ir::layout::LayoutIR;

/// Build a faceted layout from an IR GridSpec. Called by convert_layout_ir
/// when the layout contains grid information.
pub fn build_faceted_layout_from_grid_spec(ir: &LayoutIR, grid_spec: &GridSpec) -> LayoutPlotly {
    let mut grid = LayoutGrid::new()
        .rows(grid_spec.rows)
        .columns(grid_spec.cols)
        .pattern(GridPattern::Independent);

    if let Some(x_gap) = grid_spec.h_gap {
        grid = grid.x_gap(x_gap);
    }
    if let Some(y_gap) = grid_spec.v_gap {
        grid = grid.y_gap(y_gap);
    }

    let mut layout = LayoutPlotly::new().grid(grid);

    if let Some(ref title) = ir.title {
        layout = layout.title(conv::convert_text_to_title(title));
    }

    layout = apply_axis_matching(layout, grid_spec.n_facets, &grid_spec.scales);

    layout = apply_facet_axis_titles(
        layout,
        grid_spec.n_facets,
        grid_spec.cols,
        grid_spec.rows,
        grid_spec.x_title.clone(),
        grid_spec.y_title.clone(),
        grid_spec.x_axis.as_ref(),
        grid_spec.y_axis.as_ref(),
    );

    let annotations = crate::converters::layout::create_facet_annotations(
        &grid_spec.facet_categories,
        grid_spec.title_style.as_ref(),
    );
    layout = layout.annotations(annotations);

    layout = layout.legend(conv::set_legend(
        grid_spec.legend_title.clone(),
        grid_spec.legend.as_ref(),
    ));

    layout
}

fn is_bottom_row(subplot_index: usize, ncols: usize, nrows: usize, n_facets: usize) -> bool {
    let row = subplot_index / ncols;
    let index_below = subplot_index + ncols;
    row == nrows - 1 || index_below >= n_facets
}

fn is_left_column(subplot_index: usize, ncols: usize) -> bool {
    subplot_index % ncols == 0
}

/// Applies axis scale matching based on FacetScales configuration.
///
/// For Fixed: all subplot axes match the first subplot's axes.
/// For FreeX: only y-axes match (x-axes are independent).
/// For FreeY: only x-axes match (y-axes are independent).
/// For Free: no matching (all axes independent).
pub fn apply_axis_matching(
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
pub fn apply_facet_axis_titles(
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
                Some(config) => conv::set_axis(x_title_for_subplot, config, None),
                None => {
                    if let Some(title) = x_title_for_subplot {
                        conv::set_axis(Some(title), &Axis::default(), None)
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
                Some(config) => conv::set_axis(y_title_for_subplot, config, None),
                None => {
                    if let Some(title) = y_title_for_subplot {
                        conv::set_axis(Some(title), &Axis::default(), None)
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

// ---------------------------------------------------------------------------
// Scene-based faceted layout (Scatter3dPlot, SurfacePlot, Mesh3D)
// ---------------------------------------------------------------------------

/// Top margin reserved for the main title (fraction of paper height).
const SCENE_FACET_TOP_MARGIN: f64 = 0.08;
/// Space between annotation label and scene domain top.
const SCENE_FACET_LABEL_GAP: f64 = 0.03;

struct SceneFacetCell {
    annotation_x: f64,
    annotation_y: f64,
    domain_x: [f64; 2],
    domain_y: [f64; 2],
}

fn calculate_scene_facet_cell(
    subplot_index: usize,
    ncols: usize,
    nrows: usize,
    x_gap: Option<f64>,
    y_gap: Option<f64>,
) -> SceneFacetCell {
    let row = subplot_index / ncols;
    let col = subplot_index % ncols;

    let x_gap_val = x_gap.unwrap_or(0.08);
    let y_gap_val = y_gap.unwrap_or(0.12);

    // Usable vertical range (leave room for title at top)
    let usable_top = 1.0 - SCENE_FACET_TOP_MARGIN;
    let usable_height = usable_top;

    let cell_width = (1.0 - x_gap_val * (ncols - 1) as f64) / ncols as f64;
    let cell_height = (usable_height - y_gap_val * (nrows - 1) as f64) / nrows as f64;

    let cell_x_start = col as f64 * (cell_width + x_gap_val);
    let cell_y_top = usable_top - row as f64 * (cell_height + y_gap_val);
    let cell_y_bottom = cell_y_top - cell_height;

    let annotation_x = cell_x_start + cell_width / 2.0;
    let annotation_y = cell_y_top;

    let domain_y_top = cell_y_top - SCENE_FACET_LABEL_GAP;
    let domain_y_bottom = cell_y_bottom;

    SceneFacetCell {
        annotation_x,
        annotation_y,
        domain_x: [cell_x_start, cell_x_start + cell_width],
        domain_y: [domain_y_bottom, domain_y_top],
    }
}

/// Build a faceted layout for 3D scene-based plots (Scatter3dPlot, SurfacePlot, Mesh3D).
///
/// Returns a base layout plus JSON overrides containing `scene`/`scene2`/.../`sceneN`
/// keys with computed domains. The overrides must be merged into the serialized
/// layout JSON at render time because plotly.rs does not expose scene configuration.
pub fn build_scene_faceted_layout(
    ir: &LayoutIR,
    grid_spec: &GridSpec,
) -> (LayoutPlotly, Option<serde_json::Value>) {
    let ncols = grid_spec.cols;
    let nrows = grid_spec.rows;
    let x_gap = grid_spec.h_gap.unwrap_or(0.08);
    let y_gap = grid_spec.v_gap.unwrap_or(0.12);

    let mut layout = LayoutPlotly::new();

    if let Some(ref title) = ir.title {
        layout = layout.title(conv::convert_text_to_title(title));
    }

    let annotations = create_scene_facet_annotations(
        &grid_spec.facet_categories,
        ncols,
        nrows,
        grid_spec.title_style.as_ref(),
        grid_spec.h_gap,
        grid_spec.v_gap,
    );
    layout = layout.annotations(annotations);

    layout = layout.legend(conv::set_legend(
        grid_spec.legend_title.clone(),
        grid_spec.legend.as_ref(),
    ));

    layout = layout.height(500 * nrows);

    // Build JSON overrides for scene domain entries.
    let mut overrides = serde_json::Map::new();

    let total_cells = (ncols * nrows).clamp(1, 8);

    for i in 0..total_cells {
        let scene_key = if i == 0 {
            "scene".to_string()
        } else {
            format!("scene{}", i + 1)
        };

        let cell = calculate_scene_facet_cell(i, ncols, nrows, Some(x_gap), Some(y_gap));

        let mut scene_config = serde_json::json!({
            "domain": {
                "x": cell.domain_x,
                "y": cell.domain_y
            }
        });

        if i > 0 {
            match grid_spec.scales {
                FacetScales::Fixed => {
                    scene_config["xaxis"] = serde_json::json!({"matches": "x"});
                    scene_config["yaxis"] = serde_json::json!({"matches": "y"});
                    scene_config["zaxis"] = serde_json::json!({"matches": "z"});
                }
                FacetScales::FreeX => {
                    scene_config["yaxis"] = serde_json::json!({"matches": "y"});
                    scene_config["zaxis"] = serde_json::json!({"matches": "z"});
                }
                FacetScales::FreeY => {
                    scene_config["xaxis"] = serde_json::json!({"matches": "x"});
                    scene_config["zaxis"] = serde_json::json!({"matches": "z"});
                }
                FacetScales::Free => {}
            }
        }

        overrides.insert(scene_key, scene_config);
    }

    (layout, Some(serde_json::Value::Object(overrides)))
}

fn create_scene_facet_annotations(
    categories: &[String],
    ncols: usize,
    nrows: usize,
    title_style: Option<&Text>,
    x_gap: Option<f64>,
    y_gap: Option<f64>,
) -> Vec<Annotation> {
    categories
        .iter()
        .enumerate()
        .map(|(i, cat)| {
            let cell = calculate_scene_facet_cell(i, ncols, nrows, x_gap, y_gap);

            let mut ann = Annotation::new()
                .text(cat.as_str())
                .x_ref("paper")
                .y_ref("paper")
                .x_anchor(Anchor::Center)
                .y_anchor(Anchor::Bottom)
                .x(cell.annotation_x)
                .y(cell.annotation_y)
                .show_arrow(false);

            if let Some(style) = title_style {
                ann = ann.font(conv::convert_text_to_font(style));
            }

            ann
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Polar-based faceted layout (ScatterPolar)
// ---------------------------------------------------------------------------

/// Top margin reserved for the main title (fraction of paper height).
const POLAR_FACET_TOP_MARGIN: f64 = 0.08;
/// Space between annotation label and polar domain top.
const POLAR_FACET_LABEL_GAP: f64 = 0.03;

struct PolarFacetCell {
    annotation_x: f64,
    annotation_y: f64,
    domain_x: [f64; 2],
    domain_y: [f64; 2],
}

fn calculate_polar_facet_cell(
    subplot_index: usize,
    ncols: usize,
    nrows: usize,
    x_gap: Option<f64>,
    y_gap: Option<f64>,
) -> PolarFacetCell {
    let row = subplot_index / ncols;
    let col = subplot_index % ncols;

    let x_gap_val = x_gap.unwrap_or(0.08);
    let y_gap_val = y_gap.unwrap_or(0.12);

    let usable_top = 1.0 - POLAR_FACET_TOP_MARGIN;
    let usable_height = usable_top;

    let cell_width = (1.0 - x_gap_val * (ncols - 1) as f64) / ncols as f64;
    let cell_height = (usable_height - y_gap_val * (nrows - 1) as f64) / nrows as f64;

    let cell_x_start = col as f64 * (cell_width + x_gap_val);
    let cell_y_top = usable_top - row as f64 * (cell_height + y_gap_val);
    let cell_y_bottom = cell_y_top - cell_height;

    let annotation_x = cell_x_start + cell_width / 2.0;
    let annotation_y = cell_y_top;

    let domain_y_top = cell_y_top - POLAR_FACET_LABEL_GAP;
    let domain_y_bottom = cell_y_bottom;

    PolarFacetCell {
        annotation_x,
        annotation_y,
        domain_x: [cell_x_start, cell_x_start + cell_width],
        domain_y: [domain_y_bottom, domain_y_top],
    }
}

/// Build a faceted layout for polar-based plots (ScatterPolar).
///
/// Returns a base layout plus JSON overrides containing `polar`/`polar2`/.../`polarN`
/// domain entries. The overrides must be merged at render time.
pub fn build_polar_faceted_layout(
    ir: &LayoutIR,
    grid_spec: &GridSpec,
) -> (LayoutPlotly, Option<serde_json::Value>) {
    let ncols = grid_spec.cols;
    let nrows = grid_spec.rows;
    let x_gap = grid_spec.h_gap.unwrap_or(0.08);
    let y_gap = grid_spec.v_gap.unwrap_or(0.12);

    let mut layout = LayoutPlotly::new();

    if let Some(ref title) = ir.title {
        layout = layout.title(conv::convert_text_to_title(title));
    }

    let annotations = create_polar_facet_annotations(
        &grid_spec.facet_categories,
        ncols,
        nrows,
        grid_spec.title_style.as_ref(),
        grid_spec.h_gap,
        grid_spec.v_gap,
    );
    layout = layout.annotations(annotations);

    layout = layout.legend(conv::set_legend(
        grid_spec.legend_title.clone(),
        grid_spec.legend.as_ref(),
    ));

    layout = layout.height(500 * nrows);

    // Build JSON overrides for polar subplot domains.
    let mut overrides = serde_json::Map::new();

    let total_cells = (ncols * nrows).clamp(1, 8);

    for i in 0..total_cells {
        let polar_key = if i == 0 {
            "polar".to_string()
        } else {
            format!("polar{}", i + 1)
        };

        let cell = calculate_polar_facet_cell(i, ncols, nrows, Some(x_gap), Some(y_gap));

        // Compress domain height by 90% to leave breathing room around circular plots
        let compression_factor = 0.9;
        let domain_height = cell.domain_y[1] - cell.domain_y[0];
        let height_reduction = domain_height * (1.0 - compression_factor);
        let compressed_domain_y = [
            cell.domain_y[0] + height_reduction / 2.0,
            cell.domain_y[1] - height_reduction / 2.0,
        ];

        let polar_config = serde_json::json!({
            "domain": {
                "x": cell.domain_x,
                "y": compressed_domain_y
            }
        });

        overrides.insert(polar_key, polar_config);
    }

    (layout, Some(serde_json::Value::Object(overrides)))
}

fn create_polar_facet_annotations(
    categories: &[String],
    ncols: usize,
    nrows: usize,
    title_style: Option<&Text>,
    x_gap: Option<f64>,
    y_gap: Option<f64>,
) -> Vec<Annotation> {
    categories
        .iter()
        .enumerate()
        .map(|(i, cat)| {
            let cell = calculate_polar_facet_cell(i, ncols, nrows, x_gap, y_gap);

            let mut ann = Annotation::new()
                .text(cat.as_str())
                .x_ref("paper")
                .y_ref("paper")
                .x_anchor(Anchor::Center)
                .y_anchor(Anchor::Bottom)
                .x(cell.annotation_x)
                .y(cell.annotation_y)
                .show_arrow(false);

            if let Some(style) = title_style {
                ann = ann.font(conv::convert_text_to_font(style));
            }

            ann
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Domain-based faceted layout (PieChart, SankeyDiagram)
// ---------------------------------------------------------------------------

/// Title height ratio and padding matching the constants used in core's
/// `PieChart::calculate_facet_cell` and `SankeyDiagram::calculate_facet_cell`.
const DOMAIN_FACET_TITLE_HEIGHT_RATIO: f64 = 0.10;
const DOMAIN_FACET_TITLE_PADDING_RATIO: f64 = 0.35;

struct DomainFacetCell {
    annotation_x: f64,
    annotation_y: f64,
}

fn calculate_domain_facet_cell(
    subplot_index: usize,
    ncols: usize,
    nrows: usize,
    x_gap: Option<f64>,
    y_gap: Option<f64>,
) -> DomainFacetCell {
    let row = subplot_index / ncols;
    let col = subplot_index % ncols;

    let x_gap_val = x_gap.unwrap_or(0.05);
    let y_gap_val = y_gap.unwrap_or(0.10);

    let cell_width = (1.0 - x_gap_val * (ncols - 1) as f64) / ncols as f64;
    let cell_height = (1.0 - y_gap_val * (nrows - 1) as f64) / nrows as f64;

    let cell_x_start = col as f64 * (cell_width + x_gap_val);
    let cell_y_top = 1.0 - row as f64 * (cell_height + y_gap_val);

    let title_height = cell_height * DOMAIN_FACET_TITLE_HEIGHT_RATIO;
    let pie_y_top = cell_y_top - title_height;

    let padding_height = title_height * DOMAIN_FACET_TITLE_PADDING_RATIO;
    let actual_title_height = title_height - padding_height;
    let annotation_x = cell_x_start + cell_width / 2.0;
    let annotation_y = pie_y_top + padding_height + (actual_title_height / 2.0);

    DomainFacetCell {
        annotation_x,
        annotation_y,
    }
}

/// Build a faceted layout for domain-based plots (PieChart, SankeyDiagram).
///
/// The trace-level domain positioning is already computed in the IR traces
/// (domain_x/domain_y on PieChartIR and SankeyDiagramIR). This function
/// only needs to provide a layout with title, legend, and facet annotations
/// -- no LayoutGrid or axis matching is required.
pub fn build_domain_faceted_layout(ir: &LayoutIR, grid_spec: &GridSpec) -> LayoutPlotly {
    let mut layout = LayoutPlotly::new();

    if let Some(ref title) = ir.title {
        layout = layout.title(conv::convert_text_to_title(title));
    }

    let annotations = create_domain_facet_annotations(
        &grid_spec.facet_categories,
        grid_spec.cols,
        grid_spec.rows,
        grid_spec.title_style.as_ref(),
        grid_spec.h_gap,
        grid_spec.v_gap,
    );
    layout = layout.annotations(annotations);

    layout = layout.legend(conv::set_legend(
        grid_spec.legend_title.clone(),
        grid_spec.legend.as_ref(),
    ));

    layout
}

fn create_domain_facet_annotations(
    categories: &[String],
    ncols: usize,
    nrows: usize,
    title_style: Option<&Text>,
    x_gap: Option<f64>,
    y_gap: Option<f64>,
) -> Vec<Annotation> {
    categories
        .iter()
        .enumerate()
        .map(|(i, cat)| {
            let cell = calculate_domain_facet_cell(i, ncols, nrows, x_gap, y_gap);

            let mut ann = Annotation::new()
                .text(cat.as_str())
                .x_ref("paper")
                .y_ref("paper")
                .x_anchor(Anchor::Center)
                .y_anchor(Anchor::Middle)
                .x(cell.annotation_x)
                .y(cell.annotation_y)
                .show_arrow(false);

            if let Some(style) = title_style {
                ann = ann.font(conv::convert_text_to_font(style));
            }

            ann
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    // -----------------------------------------------------------------------
    // is_bottom_row
    // -----------------------------------------------------------------------

    #[test]
    fn test_is_bottom_row_last() {
        // 3x3 grid (9 facets), index 6 is row 2 (last row)
        assert!(is_bottom_row(6, 3, 3, 9));
    }

    #[test]
    fn test_is_bottom_row_not_last() {
        assert!(!is_bottom_row(0, 3, 3, 9));
    }

    #[test]
    fn test_is_bottom_row_partial() {
        // 2x3 grid with 5 facets, index 3 (row 1, col 1)
        // subplot below would be at index 5 which == n_facets, so true
        assert!(is_bottom_row(3, 2, 3, 5));
    }

    // -----------------------------------------------------------------------
    // is_left_column
    // -----------------------------------------------------------------------

    #[test]
    fn test_is_left_column_true() {
        assert!(is_left_column(0, 3));
    }

    #[test]
    fn test_is_left_column_false() {
        assert!(!is_left_column(1, 3));
    }

    #[test]
    fn test_is_left_column_second_row() {
        assert!(is_left_column(3, 3));
    }

    // -----------------------------------------------------------------------
    // calculate_scene_facet_cell
    // -----------------------------------------------------------------------

    #[test]
    fn test_scene_cell_first_in_2x2() {
        let cell = calculate_scene_facet_cell(0, 2, 2, Some(0.05), Some(0.05));
        assert!(cell.domain_x[0] >= 0.0);
        assert!(cell.domain_x[1] <= 1.0);
        assert!(cell.domain_y[0] >= 0.0);
        assert!(cell.domain_y[1] <= 1.0);
        assert!(cell.domain_x[0] < cell.domain_x[1]);
        assert!(cell.domain_y[0] < cell.domain_y[1]);
    }

    #[test]
    fn test_scene_cell_all_in_range() {
        for i in 0..4 {
            let cell = calculate_scene_facet_cell(i, 2, 2, Some(0.05), Some(0.05));
            assert!(cell.domain_x[0] >= 0.0 && cell.domain_x[0] <= 1.0);
            assert!(cell.domain_x[1] >= 0.0 && cell.domain_x[1] <= 1.0);
            assert!(cell.domain_y[0] >= 0.0 && cell.domain_y[0] <= 1.0);
            assert!(cell.domain_y[1] >= 0.0 && cell.domain_y[1] <= 1.0);
        }
    }

    #[test]
    fn test_scene_cell_different_gaps() {
        let cell_small = calculate_scene_facet_cell(1, 2, 2, Some(0.02), Some(0.02));
        let cell_large = calculate_scene_facet_cell(1, 2, 2, Some(0.10), Some(0.10));
        // Larger gap means narrower cells
        let width_small = cell_small.domain_x[1] - cell_small.domain_x[0];
        let width_large = cell_large.domain_x[1] - cell_large.domain_x[0];
        assert!(width_small > width_large);
    }

    // -----------------------------------------------------------------------
    // calculate_polar_facet_cell
    // -----------------------------------------------------------------------

    #[test]
    fn test_polar_cell_first_in_2x2() {
        let cell = calculate_polar_facet_cell(0, 2, 2, Some(0.05), Some(0.05));
        assert!(cell.domain_x[0] >= 0.0);
        assert!(cell.domain_x[1] <= 1.0);
        assert!(cell.domain_y[0] >= 0.0);
        assert!(cell.domain_y[1] <= 1.0);
        assert!(cell.domain_x[0] < cell.domain_x[1]);
        assert!(cell.domain_y[0] < cell.domain_y[1]);
    }

    // -----------------------------------------------------------------------
    // apply_axis_matching
    // -----------------------------------------------------------------------

    #[test]
    fn test_axis_matching_fixed() {
        let layout = LayoutPlotly::new();
        let result = apply_axis_matching(layout, 3, &FacetScales::Fixed);
        let json = serde_json::to_string(&result).unwrap();
        // Fixed: both xaxis2/yaxis2 and xaxis3/yaxis3 should have matches
        assert!(json.contains(r#""matches":"x""#));
        assert!(json.contains(r#""matches":"y""#));
    }

    #[test]
    fn test_axis_matching_free_x() {
        let layout = LayoutPlotly::new();
        let result = apply_axis_matching(layout, 3, &FacetScales::FreeX);
        let json = serde_json::to_string(&result).unwrap();
        // FreeX: only y-axes match
        assert!(!json.contains(r#""matches":"x""#));
        assert!(json.contains(r#""matches":"y""#));
    }

    #[test]
    fn test_axis_matching_free() {
        let layout = LayoutPlotly::new();
        let result = apply_axis_matching(layout, 3, &FacetScales::Free);
        let json = serde_json::to_string(&result).unwrap();
        // Free: no matching at all
        assert!(!json.contains("matches"));
    }
}
