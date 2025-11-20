use plotly::{
    layout::{Annotation, Axis as AxisPlotly, Layout as LayoutPlotly},
    Trace,
};
use serde_json::Value;

use crate::common::PlotHelper;
use crate::components::{Dimensions, Text};

use super::custom_legend::CustomLegend;
use super::shared::{
    adjust_domain_for_type, build_axis_from_config, calculate_spanning_domain,
    detect_axis_type_from_traces, detect_plot_type, determine_bar_mode, determine_box_mode,
    extract_axis_title_from_annotations, inject_non_cartesian_domains, validate_irregular_grid,
    AxisConfig, JsonTrace, NonCartesianLayout, PlotType,
};
use super::SubplotGrid;

struct PlotConfig {
    _position: (usize, usize, usize, usize),
    title: Option<Text>,
    x_config: AxisConfig,
    y_config: AxisConfig,
    x_axis_type: Option<String>,
    y_axis_type: Option<String>,
    plot_type: PlotType,
    layout_fragment: Option<Value>,
    domain_x: [f64; 2],
    domain_y: [f64; 2],
    subplot_ref: Option<String>,
}

pub(super) fn build_irregular(
    plots: Vec<(&dyn PlotHelper, usize, usize, usize, usize)>,
    rows: Option<usize>,
    cols: Option<usize>,
    title: Option<Text>,
    h_gap: Option<f64>,
    v_gap: Option<f64>,
    dimensions: Option<&Dimensions>,
) -> SubplotGrid {
    let rows = rows.unwrap_or(1);
    let cols = cols.unwrap_or(1);
    let h_gap = h_gap.unwrap_or(0.1);
    let v_gap = v_gap.unwrap_or(0.1);

    validate_irregular_grid(&plots, rows, cols);

    let mut all_traces: Vec<Box<dyn Trace + 'static>> = Vec::new();
    let mut plot_configs: Vec<PlotConfig> = Vec::new();
    let mut legend_sources: Vec<Vec<JsonTrace>> = Vec::new();
    let mut scene_count = 0;
    let mut polar_count = 0;
    let mut mapbox_count = 0;
    let mut geo_count = 0;

    for (plot, row, col, rowspan, colspan) in plots.iter() {
        let traces = plot.get_traces();
        let plot_type = detect_plot_type(traces[0].as_ref());
        let plot_title = plot.get_main_title_text();

        let layout_json = plot
            .get_layout_override()
            .cloned()
            .unwrap_or_else(|| serde_json::to_value(plot.get_layout()).unwrap_or(Value::Null));
        let x_axis_json = layout_json.get("xaxis").cloned().unwrap_or(Value::Null);
        let y_axis_json = layout_json.get("yaxis").cloned().unwrap_or(Value::Null);

        let x_title = plot
            .get_x_title_text()
            .or_else(|| extract_axis_title_from_annotations(&layout_json, true));

        let y_title = plot
            .get_y_title_text()
            .or_else(|| extract_axis_title_from_annotations(&layout_json, false));

        let x_axis_type = if plot_type == PlotType::Cartesian2D {
            detect_axis_type_from_traces(traces)
        } else {
            None
        };
        let y_axis_type = None;

        let layout_fragment = match plot_type {
            PlotType::Cartesian3D => layout_json.get("scene").cloned(),
            PlotType::Polar => layout_json.get("polar").cloned(),
            PlotType::Mapbox => layout_json.get("mapbox").cloned(),
            PlotType::Geo => layout_json.get("geo").cloned(),
            _ => None,
        };

        let (x_start, x_end, y_start, y_end) =
            calculate_spanning_domain(*row, *col, *rowspan, *colspan, rows, cols, h_gap, v_gap);
        let (domain_x, domain_y) =
            adjust_domain_for_type(plot_type.clone(), x_start, x_end, y_start, y_end);

        let subplot_ref = match plot_type {
            PlotType::Cartesian3D => {
                let name = if scene_count == 0 {
                    "scene".to_string()
                } else {
                    format!("scene{}", scene_count + 1)
                };
                scene_count += 1;
                Some(name)
            }
            PlotType::Polar => {
                let name = if polar_count == 0 {
                    "polar".to_string()
                } else {
                    format!("polar{}", polar_count + 1)
                };
                polar_count += 1;
                Some(name)
            }
            PlotType::Mapbox => {
                let name = if mapbox_count == 0 {
                    "mapbox".to_string()
                } else {
                    format!("mapbox{}", mapbox_count + 1)
                };
                mapbox_count += 1;
                Some(name)
            }
            PlotType::Geo => {
                let name = if geo_count == 0 {
                    "geo".to_string()
                } else {
                    format!("geo{}", geo_count + 1)
                };
                geo_count += 1;
                Some(name)
            }
            PlotType::Cartesian2D | PlotType::Domain => None,
        };

        plot_configs.push(PlotConfig {
            _position: (*row, *col, *rowspan, *colspan),
            title: plot_title,
            x_config: AxisConfig {
                title: x_title,
                axis_json: x_axis_json,
            },
            y_config: AxisConfig {
                title: y_title,
                axis_json: y_axis_json,
            },
            x_axis_type,
            y_axis_type,
            plot_type,
            layout_fragment,
            domain_x,
            domain_y,
            subplot_ref,
        });

        let mut legend_traces: Vec<JsonTrace> = Vec::new();

        if let Some(serialized_traces) = plot.get_serialized_traces() {
            for trace_value in serialized_traces {
                let mut json_trace = JsonTrace::from_value(trace_value);
                json_trace.ensure_color(all_traces.len());
                legend_traces.push(json_trace.clone());
                all_traces.push(Box::new(json_trace));
            }
        } else {
            for trace in traces {
                let mut json_trace = JsonTrace::new(trace.clone());
                json_trace.ensure_color(all_traces.len());
                legend_traces.push(json_trace.clone());
                all_traces.push(Box::new(json_trace));
            }
        }

        legend_sources.push(legend_traces);
    }

    assign_axis_references(
        &mut all_traces,
        &plots,
        &plot_configs,
    );

    let (layout, layout_json) = create_irregular_layout(
        rows,
        cols,
        h_gap,
        v_gap,
        title,
        &plot_configs,
        &plots,
        dimensions,
    );

    scale_colorbars_for_subplots(&mut all_traces, &plot_configs, &plots, cols, h_gap);

    SubplotGrid {
        traces: all_traces,
        layout,
        layout_json: Some(layout_json),
    }
}

fn assign_axis_references(
    all_traces: &mut [Box<dyn Trace + 'static>],
    plots: &[(&dyn PlotHelper, usize, usize, usize, usize)],
    plot_configs: &[PlotConfig],
) {
    let mut trace_idx = 0;
    for (plot_idx, ((plot, _, _, _, _), config)) in plots.iter().zip(plot_configs.iter()).enumerate()
    {
        let x_axis = if plot_idx == 0 {
            "x".to_string()
        } else {
            format!("x{}", plot_idx + 1)
        };
        let y_axis = if plot_idx == 0 {
            "y".to_string()
        } else {
            format!("y{}", plot_idx + 1)
        };

        let traces = plot.get_traces();

        for _ in 0..traces.len() {
            let mut json_trace = JsonTrace::new(all_traces[trace_idx].clone());

            match config.plot_type {
                PlotType::Cartesian2D => json_trace.set_axis_references(&x_axis, &y_axis),
                PlotType::Cartesian3D => {
                    if let Some(ref scene_ref) = config.subplot_ref {
                        json_trace.set_scene_reference(scene_ref);
                    }
                }
                PlotType::Polar => {
                    if let Some(ref polar_ref) = config.subplot_ref {
                        json_trace.set_subplot_reference(polar_ref);
                    }
                }
                PlotType::Domain => json_trace.set_domain(config.domain_x, config.domain_y),
                PlotType::Mapbox => {
                    if let Some(ref mapbox_ref) = config.subplot_ref {
                        json_trace.set_subplot_reference(mapbox_ref);
                    }
                }
                PlotType::Geo => {
                    if let Some(ref geo_ref) = config.subplot_ref {
                        json_trace.set_subplot_reference(geo_ref);
                    }
                }
            }

            all_traces[trace_idx] = Box::new(json_trace);
            trace_idx += 1;
        }
    }
}

fn scale_colorbars_for_subplots(
    all_traces: &mut [Box<dyn Trace + 'static>],
    plot_configs: &[PlotConfig],
    plots: &[(&dyn PlotHelper, usize, usize, usize, usize)],
    cols: usize,
    h_gap: f64,
) {
    let mut trace_idx = 0;

    for (config, (plot, _, _, _, _)) in plot_configs.iter().zip(plots.iter()) {
        let (_, col, _, colspan) = config._position;
        let x_start = config.domain_x[0];
        let x_end = config.domain_x[1];
        let y_start = config.domain_y[0];
        let y_end = config.domain_y[1];

        // Calculate the rightmost column this subplot occupies
        let end_col = col + colspan - 1;
        let is_rightmost_col = end_col == cols - 1;

        let domain_width = x_end - x_start;
        let domain_height = y_end - y_start;
        let traces = plot.get_traces();
        let num_traces = traces.len();

        for _ in 0..num_traces {
            if trace_idx >= all_traces.len() {
                break;
            }

            let trace_json = serde_json::to_value(&all_traces[trace_idx]).ok();

            if let Some(mut trace_value) = trace_json {
                // Check if trace shows a colorbar (has colorbar object or showscale is not false)
                let has_colorbar = trace_value.get("colorbar").is_some();
                let shows_scale = trace_value
                    .get("showscale")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(true); // Default is true for heatmaps/contours

                // Create colorbar object if trace shows scale but doesn't have explicit colorbar
                if !has_colorbar && shows_scale {
                    if let Some(trace_type) = trace_value.get("type").and_then(|v| v.as_str()) {
                        if matches!(trace_type, "heatmap" | "contour" | "surface") {
                            trace_value["colorbar"] = serde_json::json!({});
                        }
                    }
                }

                if let Some(colorbar) = trace_value.get_mut("colorbar") {
                    let current_len = colorbar.get("len").and_then(|v| v.as_f64());

                    match current_len {
                        Some(len) => {
                            if let Some(lenmode) = colorbar.get("lenmode").and_then(|v| v.as_str())
                            {
                                if lenmode == "fraction" && len > domain_height {
                                    let scaled_len = len * domain_height;
                                    colorbar["len"] = serde_json::json!(scaled_len);
                                }
                            }
                        }
                        None => {
                            colorbar["len"] = serde_json::json!(domain_height);
                            colorbar["lenmode"] = serde_json::json!("fraction");
                        }
                    }

                    let user_y_domain = colorbar.get("y").and_then(|v| v.as_f64()).unwrap_or(0.5);

                    if colorbar.get("yanchor").is_none() {
                        let yanchor = if user_y_domain >= 0.8 {
                            "top"
                        } else if user_y_domain <= 0.2 {
                            "bottom"
                        } else {
                            "middle"
                        };
                        colorbar["yanchor"] = serde_json::json!(yanchor);
                    }

                    if colorbar.get("yref").is_none() {
                        colorbar["yref"] = serde_json::json!("paper");
                    }

                    let paper_y = y_start + user_y_domain * (y_end - y_start);
                    colorbar["y"] = serde_json::json!(paper_y);

                    // Position colorbar - convert user's x value from subplot domain to paper coordinates
                    if colorbar.get("xref").is_none() {
                        colorbar["xref"] = serde_json::json!("paper");
                    }

                    if let Some(user_x) = colorbar.get("x").and_then(|v| v.as_f64()) {
                        // User specified x value - interpret as subplot domain and convert to paper
                        let paper_x = x_start + user_x * domain_width;
                        colorbar["x"] = serde_json::json!(paper_x);
                    } else {
                        // No user x value - use automatic positioning in the gap
                        if is_rightmost_col {
                            // Rightmost column: position just past the right edge
                            if colorbar.get("xanchor").is_none() {
                                colorbar["xanchor"] = serde_json::json!("left");
                            }
                            let paper_x = x_end + 0.01;
                            colorbar["x"] = serde_json::json!(paper_x);
                        } else {
                            // Non-rightmost columns: center colorbar in the gap
                            if colorbar.get("xanchor").is_none() {
                                colorbar["xanchor"] = serde_json::json!("center");
                            }
                            // Position at center of the gap between this subplot and the next
                            let gap_center = x_end + (h_gap / 2.0);
                            colorbar["x"] = serde_json::json!(gap_center);
                        }
                    }

                    let scaled_trace = JsonTrace::from_value(trace_value);
                    all_traces[trace_idx] = Box::new(scaled_trace);
                }
            }

            trace_idx += 1;
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn create_irregular_layout(
    _rows: usize,
    _cols: usize,
    _h_gap: f64,
    _v_gap: f64,
    plot_title: Option<Text>,
    plot_configs: &[PlotConfig],
    plots: &[(&dyn PlotHelper, usize, usize, usize, usize)],
    dimensions: Option<&Dimensions>,
) -> (LayoutPlotly, Value) {
    let mut layout = LayoutPlotly::new().show_legend(false);

    let plot_refs: Vec<&dyn PlotHelper> = plots.iter().map(|(p, _, _, _, _)| *p).collect();
    if let Some(bar_mode) = determine_bar_mode(&plot_refs) {
        layout = layout.bar_mode(bar_mode);
    }

    if let Some(box_mode) = determine_box_mode(&plot_refs) {
        layout = layout.box_mode(box_mode);
    }

    if let Some(title) = plot_title {
        layout = layout.title(title.with_plot_title_defaults().to_plotly());
    }

    for (idx, config) in plot_configs.iter().enumerate() {
        if !matches!(config.plot_type, PlotType::Cartesian2D) {
            continue;
        }

        let mut x_axis = build_axis_from_config(&config.x_config).domain(&config.domain_x);
        if let Some(axis_type_str) = &config.x_axis_type {
            use plotly::layout::AxisType;
            let axis_type = match axis_type_str.as_str() {
                "date" => AxisType::Date,
                "category" => AxisType::Category,
                "linear" => AxisType::Linear,
                _ => AxisType::Default,
            };
            x_axis = x_axis.type_(axis_type);
        }

        let y_anchor = if idx == 0 {
            "y"
        } else {
            &format!("y{}", idx + 1)
        };
        x_axis = x_axis.anchor(y_anchor);

        layout = assign_x_axis(layout, idx, x_axis);

        let mut y_axis = build_axis_from_config(&config.y_config).domain(&config.domain_y);
        if let Some(axis_type_str) = &config.y_axis_type {
            use plotly::layout::AxisType;
            let axis_type = match axis_type_str.as_str() {
                "date" => AxisType::Date,
                "category" => AxisType::Category,
                "linear" => AxisType::Linear,
                _ => AxisType::Default,
            };
            y_axis = y_axis.type_(axis_type);
        }

        let x_anchor = if idx == 0 {
            "x"
        } else {
            &format!("x{}", idx + 1)
        };
        y_axis = y_axis.anchor(x_anchor);

        layout = assign_y_axis(layout, idx, y_axis);
    }

    let annotations = collect_annotations(plot_configs, plots);
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

    let mut layout_json = serde_json::to_value(&layout).unwrap();

    let non_cartesian: Vec<NonCartesianLayout> = plot_configs
        .iter()
        .map(|config| NonCartesianLayout {
            plot_type: config.plot_type.clone(),
            domain_x: config.domain_x,
            domain_y: config.domain_y,
            layout_fragment: config.layout_fragment.clone(),
            subplot_ref: config.subplot_ref.clone().unwrap_or_default(),
        })
        .collect();

    inject_non_cartesian_domains(&mut layout_json, &non_cartesian);

    (layout, layout_json)
}

fn assign_x_axis(mut layout: LayoutPlotly, idx: usize, axis: AxisPlotly) -> LayoutPlotly {
    layout = match idx {
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
    layout
}

fn assign_y_axis(mut layout: LayoutPlotly, idx: usize, axis: AxisPlotly) -> LayoutPlotly {
    layout = match idx {
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
    layout
}

fn collect_annotations(
    plot_configs: &[PlotConfig],
    plots: &[(&dyn PlotHelper, usize, usize, usize, usize)],
) -> Vec<Annotation> {
    let mut annotations = Vec::new();

    // Convert ALL axis titles to annotations for grid-aware positioning
    // This ensures titles work correctly regardless of how they were specified
    for (idx, config) in plot_configs.iter().enumerate() {
        if !matches!(config.plot_type, PlotType::Cartesian2D) {
            continue;
        }

        if let Some(ref x_title) = config.x_config.title {
            let axis_ref = if idx == 0 {
                "x".to_string()
            } else {
                format!("x{}", idx + 1)
            };

            // Apply defaults to ensure proper positioning (fills in any unset coordinates)
            let x_title_with_defaults = x_title.clone().with_x_title_defaults();
            annotations.push(x_title_with_defaults.to_axis_annotation(true, &axis_ref, true));
        }

        if let Some(ref y_title) = config.y_config.title {
            let axis_ref = if idx == 0 {
                "y".to_string()
            } else {
                format!("y{}", idx + 1)
            };

            // Apply defaults to ensure proper positioning (fills in any unset coordinates)
            let y_title_with_defaults = y_title.clone().with_y_title_defaults();
            annotations.push(y_title_with_defaults.to_axis_annotation(false, &axis_ref, true));
        }
    }

    for (idx, config) in plot_configs.iter().enumerate() {
        if let Some(title_text) = &config.title {
            let title = title_text.clone().with_subplot_title_defaults();

            if matches!(config.plot_type, PlotType::Cartesian2D) {
                let x_ref = if idx == 0 {
                    "x domain".to_string()
                } else {
                    format!("x{} domain", idx + 1)
                };
                let y_ref = if idx == 0 {
                    "y domain".to_string()
                } else {
                    format!("y{} domain", idx + 1)
                };

                annotations.push(
                    Annotation::new()
                        .text(&title.content)
                        .font(title.to_font())
                        .x_ref(&x_ref)
                        .y_ref(&y_ref)
                        .x(title.x)
                        .y(title.y)
                        .show_arrow(false),
                );
            } else {
                let width = config.domain_x[1] - config.domain_x[0];
                let height = config.domain_y[1] - config.domain_y[0];
                let x_pos = config.domain_x[0] + width * title.x;
                let y_pos = if matches!(config.plot_type, PlotType::Polar) {
                    config.domain_y[1] + height * 0.20
                } else {
                    config.domain_y[0] + height * title.y
                };

                annotations.push(
                    Annotation::new()
                        .text(&title.content)
                        .font(title.to_font())
                        .x_ref("paper")
                        .y_ref("paper")
                        .x(x_pos)
                        .y(y_pos)
                        .show_arrow(false),
                );
            }
        }
    }

    for (subplot_idx, (plot, _, _, _, _)) in plots.iter().enumerate() {
        if let Some(auto_legend) = CustomLegend::from_plot(*plot) {
            let domain = plot_configs.get(subplot_idx).and_then(|config| {
                if matches!(config.plot_type, PlotType::Cartesian2D) {
                    None
                } else {
                    Some((config.domain_x, config.domain_y))
                }
            });

            if let Some(legend_annotation) = auto_legend.to_annotation(subplot_idx, domain) {
                annotations.push(legend_annotation);
            }
        }
    }

    annotations
}
