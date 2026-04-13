#![allow(dead_code)]

use plotly::common::Anchor;
use plotly::layout::{Annotation, BoxMode, Center, Mapbox, MapboxStyle, Margin};
use plotly::Layout as LayoutPlotly;

use crate::converters::components as conv;
use plotlars_core::components::{Axis, Dimensions, Legend, Text};
use plotlars_core::ir::layout::{BoxModeIR, LayoutIR};

#[allow(clippy::too_many_arguments)]
pub(crate) fn create_layout(
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
    let mut layout = LayoutPlotly::new();
    let mut annotations = Vec::new();

    if let Some(title) = plot_title {
        layout = layout.title(conv::convert_text_to_title(&title));
    }

    let (x_title_for_axis, x_annotation) = if let Some(text) = x_title {
        if text.has_custom_position() {
            let text_with_defaults = text.with_x_title_defaults_for_annotation();
            let ann = conv::convert_text_to_axis_annotation(&text_with_defaults, true, "x", false);
            (None, Some(ann))
        } else {
            (Some(text.with_x_title_defaults()), None)
        }
    } else {
        (None, None)
    };

    match (x_axis, x_title_for_axis) {
        (Some(axis), title) => {
            layout = layout.x_axis(conv::set_axis(title, axis, None));
        }
        (None, Some(title)) => {
            let default_axis = Axis::default();
            layout = layout.x_axis(conv::set_axis(Some(title), &default_axis, None));
        }
        _ => {}
    }

    if let Some(ann) = x_annotation {
        annotations.push(ann);
    }

    let (y_title_for_axis, y_annotation) = if let Some(text) = y_title {
        if text.has_custom_position() {
            let text_with_defaults = text.with_y_title_defaults_for_annotation();
            let ann = conv::convert_text_to_axis_annotation(&text_with_defaults, false, "y", false);
            (None, Some(ann))
        } else {
            (Some(text.with_y_title_defaults()), None)
        }
    } else {
        (None, None)
    };

    match (y_axis, y_title_for_axis) {
        (Some(axis), title) => {
            layout = layout.y_axis(conv::set_axis(title, axis, None));
        }
        (None, Some(title)) => {
            let default_axis = Axis::default();
            layout = layout.y_axis(conv::set_axis(Some(title), &default_axis, None));
        }
        _ => {}
    }

    if let Some(ann) = y_annotation {
        annotations.push(ann);
    }

    if let Some(y2_axis) = y2_axis {
        layout = layout.y_axis2(conv::set_axis(y2_title, y2_axis, Some("y")));
    }

    match (z_axis, z_title) {
        (Some(axis), title) => {
            layout = layout.z_axis(conv::set_axis(title, axis, None));
        }
        (None, Some(title)) => {
            let default_axis = Axis::default();
            layout = layout.z_axis(conv::set_axis(Some(title), &default_axis, None));
        }
        _ => {}
    }

    layout = layout.legend(conv::set_legend(legend_title, legend));

    if !annotations.is_empty() {
        layout = layout.annotations(annotations);
    }

    if let Some(dims) = dimensions {
        if let Some(width) = dims.width {
            layout = layout.width(width);
        }
        if let Some(height) = dims.height {
            layout = layout.height(height);
        }
        if let Some(auto_size) = dims.auto_size {
            layout = layout.auto_size(auto_size);
        }
    }

    layout
}

/// Converts a `LayoutIR` to a plotly `Layout` plus optional JSON overrides.
///
/// The second element of the tuple, when `Some`, contains extra JSON keys
/// (e.g. `scene`, `scene2`, `polar`, `polar2`) that must be merged into
/// the serialized layout because plotly.rs's `Layout` type does not expose
/// these fields directly.
pub(crate) fn convert_layout_ir(ir: &LayoutIR) -> (LayoutPlotly, Option<serde_json::Value>) {
    // If a grid spec is present, build a full faceted layout
    if let Some(ref grid_spec) = ir.grid {
        return convert_faceted_layout_ir(ir, grid_spec);
    }

    let (x_axis, y_axis, y2_axis) = match &ir.axes_2d {
        Some(axes) => (
            axes.x_axis.as_ref(),
            axes.y_axis.as_ref(),
            axes.y2_axis.as_ref(),
        ),
        None => (None, None, None),
    };

    let z_axis = ir.scene_3d.as_ref().and_then(|scene| scene.z_axis.as_ref());

    let mut layout = create_layout(
        ir.title.clone(),
        ir.x_title.clone(),
        ir.y_title.clone(),
        ir.y2_title.clone(),
        ir.z_title.clone(),
        ir.legend_title.clone(),
        x_axis,
        y_axis,
        y2_axis,
        z_axis,
        ir.legend.as_ref(),
        ir.dimensions.as_ref(),
    );

    if let Some(ref bar_mode) = ir.bar_mode {
        layout = layout.bar_mode(conv::convert_bar_mode(bar_mode));
    }

    if let Some(ref box_mode) = ir.box_mode {
        layout = layout.box_mode(match box_mode {
            BoxModeIR::Group => BoxMode::Group,
        });
    }

    if let Some(gap) = ir.box_gap {
        layout = layout.box_gap(gap);
    }

    if let Some(ref mapbox_ir) = ir.mapbox {
        let mut map_box = Mapbox::new().style(MapboxStyle::OpenStreetMap);
        if let Some((lat, lon)) = mapbox_ir.center {
            map_box = map_box.center(Center::new(lat, lon));
        }
        if let Some(zoom) = mapbox_ir.zoom {
            map_box = map_box.zoom(zoom as u8);
        } else {
            map_box = map_box.zoom(0);
        }
        layout = layout.mapbox(map_box);
    }

    if let Some(bottom) = ir.margin_bottom {
        layout = layout.margin(Margin::new().bottom(bottom));
    }

    (layout, None)
}

/// Build a faceted layout from GridSpec. Dispatches to the appropriate layout
/// builder depending on the facet kind (Axis, Scene, Polar, Domain).
fn convert_faceted_layout_ir(
    ir: &LayoutIR,
    grid_spec: &plotlars_core::ir::facet::GridSpec,
) -> (LayoutPlotly, Option<serde_json::Value>) {
    use plotlars_core::ir::facet::FacetKind;

    let (mut layout, json_overrides) = match &grid_spec.kind {
        FacetKind::Axis => (
            crate::faceting::build_faceted_layout_from_grid_spec(ir, grid_spec),
            None,
        ),
        FacetKind::Scene => crate::faceting::build_scene_faceted_layout(ir, grid_spec),
        FacetKind::Polar => crate::faceting::build_polar_faceted_layout(ir, grid_spec),
        FacetKind::Domain => (
            crate::faceting::build_domain_faceted_layout(ir, grid_spec),
            None,
        ),
    };

    if let Some(ref bar_mode) = ir.bar_mode {
        layout = layout.bar_mode(conv::convert_bar_mode(bar_mode));
    }

    if let Some(ref box_mode) = ir.box_mode {
        layout = layout.box_mode(match box_mode {
            BoxModeIR::Group => BoxMode::Group,
        });
    }

    if let Some(gap) = ir.box_gap {
        layout = layout.box_gap(gap);
    }

    (layout, json_overrides)
}

pub(crate) fn create_facet_annotations(
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
                ann = ann.font(conv::convert_text_to_font(style));
            }

            ann
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use plotlars_core::ir::layout::LayoutIR;

    fn make_default_layout_ir() -> LayoutIR {
        LayoutIR {
            title: None,
            x_title: None,
            y_title: None,
            y2_title: None,
            z_title: None,
            legend_title: None,
            legend: None,
            dimensions: None,
            bar_mode: None,
            box_mode: None,
            box_gap: None,
            margin_bottom: None,
            axes_2d: None,
            scene_3d: None,
            polar: None,
            mapbox: None,
            grid: None,
            annotations: vec![],
        }
    }

    // -----------------------------------------------------------------------
    // convert_layout_ir
    // -----------------------------------------------------------------------

    #[test]
    fn test_no_grid_returns_none_overrides() {
        let ir = make_default_layout_ir();
        let (_layout, overrides) = convert_layout_ir(&ir);
        assert!(overrides.is_none());
    }

    #[test]
    fn test_axis_grid_returns_none_overrides() {
        use plotlars_core::components::facet::FacetScales;
        use plotlars_core::ir::facet::{FacetKind, GridSpec};

        let mut ir = make_default_layout_ir();
        ir.grid = Some(GridSpec {
            kind: FacetKind::Axis,
            rows: 1,
            cols: 2,
            h_gap: None,
            v_gap: None,
            scales: FacetScales::Fixed,
            n_facets: 2,
            facet_categories: vec!["A".to_string(), "B".to_string()],
            title_style: None,
            x_title: None,
            y_title: None,
            x_axis: None,
            y_axis: None,
            legend_title: None,
            legend: None,
        });
        let (_layout, overrides) = convert_layout_ir(&ir);
        assert!(overrides.is_none());
    }

    #[test]
    fn test_scene_grid_returns_some_overrides() {
        use plotlars_core::components::facet::FacetScales;
        use plotlars_core::ir::facet::{FacetKind, GridSpec};

        let mut ir = make_default_layout_ir();
        ir.grid = Some(GridSpec {
            kind: FacetKind::Scene,
            rows: 1,
            cols: 2,
            h_gap: None,
            v_gap: None,
            scales: FacetScales::Fixed,
            n_facets: 2,
            facet_categories: vec!["A".to_string(), "B".to_string()],
            title_style: None,
            x_title: None,
            y_title: None,
            x_axis: None,
            y_axis: None,
            legend_title: None,
            legend: None,
        });
        let (_layout, overrides) = convert_layout_ir(&ir);
        assert!(overrides.is_some());
    }

    // -----------------------------------------------------------------------
    // create_facet_annotations
    // -----------------------------------------------------------------------

    #[test]
    fn test_annotations_count() {
        let cats = vec!["A".to_string(), "B".to_string(), "C".to_string()];
        let anns = create_facet_annotations(&cats, None);
        assert_eq!(anns.len(), 3);
    }

    #[test]
    fn test_annotations_text() {
        let cats = vec!["Alpha".to_string(), "Beta".to_string()];
        let anns = create_facet_annotations(&cats, None);
        let json = serde_json::to_string(&anns).unwrap();
        assert!(json.contains("Alpha"));
        assert!(json.contains("Beta"));
    }

    #[test]
    fn test_annotations_with_style() {
        let cats = vec!["X".to_string()];
        let style = Text::from("X").size(20);
        let anns = create_facet_annotations(&cats, Some(&style));
        let json = serde_json::to_string(&anns).unwrap();
        assert!(json.contains("font"));
    }
}
