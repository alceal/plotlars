use plotly::{
    layout::{Annotation, Axis as AxisPlotly, Layout as LayoutPlotly},
    Trace,
};
use serde_json::Value;

use crate::common::PlotHelper;
use crate::components::Text;

use super::custom_legend::CustomLegend;
use super::shared::{
    build_axis_from_config, calculate_spanning_domain, detect_axis_type_from_traces,
    detect_plot_type, determine_bar_mode, determine_box_mode, extract_axis_title_from_annotations,
    validate_irregular_grid, AxisConfig, JsonTrace, PlotType,
};
use super::SubplotGrid;

type PlotConfig = (
    (usize, usize, usize, usize),
    Option<Text>,
    AxisConfig,
    AxisConfig,
    Option<String>,
    Option<String>,
);

pub(super) fn build_irregular(
    plots: Vec<(&dyn PlotHelper, usize, usize, usize, usize)>,
    rows: Option<usize>,
    cols: Option<usize>,
    title: Option<Text>,
    h_gap: Option<f64>,
    v_gap: Option<f64>,
) -> SubplotGrid {
    let rows = rows.unwrap_or(1);
    let cols = cols.unwrap_or(1);
    let h_gap = h_gap.unwrap_or(0.1);
    let v_gap = v_gap.unwrap_or(0.1);

    validate_irregular_grid(&plots, rows, cols);

    let mut all_traces: Vec<Box<dyn Trace + 'static>> = Vec::new();
    let mut plot_configs: Vec<PlotConfig> = Vec::new();

    for (plot, row, col, rowspan, colspan) in &plots {
        let traces = plot.get_traces();
        let plot_title = plot.get_main_title_text();

        let layout_json = serde_json::to_value(plot.get_layout()).unwrap_or(Value::Null);
        let x_axis_json = layout_json.get("xaxis").cloned().unwrap_or(Value::Null);
        let y_axis_json = layout_json.get("yaxis").cloned().unwrap_or(Value::Null);

        let x_title = plot
            .get_x_title_text()
            .or_else(|| extract_axis_title_from_annotations(&layout_json, true));

        let y_title = plot
            .get_y_title_text()
            .or_else(|| extract_axis_title_from_annotations(&layout_json, false));

        let x_axis_type = detect_axis_type_from_traces(traces);
        let y_axis_type = None;

        plot_configs.push((
            (*row, *col, *rowspan, *colspan),
            plot_title,
            AxisConfig {
                title: x_title,
                axis_json: x_axis_json,
            },
            AxisConfig {
                title: y_title,
                axis_json: y_axis_json,
            },
            x_axis_type,
            y_axis_type,
        ));

        if let Some(serialized_traces) = plot.get_serialized_traces() {
            for trace_value in serialized_traces {
                let json_trace = JsonTrace::from_value(trace_value);
                all_traces.push(Box::new(json_trace));
            }
        } else {
            for trace in traces {
                all_traces.push(trace.clone());
            }
        }
    }

    assign_axis_references(&mut all_traces, &plots);

    let (layout, layout_json) =
        create_irregular_layout(rows, cols, h_gap, v_gap, title, &plot_configs, &plots);

    scale_colorbars_for_subplots(
        &mut all_traces,
        &plot_configs,
        &plots,
        rows,
        cols,
        h_gap,
        v_gap,
    );

    SubplotGrid {
        traces: all_traces,
        layout,
        layout_json: Some(layout_json),
    }
}

fn assign_axis_references(
    all_traces: &mut [Box<dyn Trace + 'static>],
    plots: &[(&dyn PlotHelper, usize, usize, usize, usize)],
) {
    let mut trace_idx = 0;
    for (plot_idx, (plot, _, _, _, _)) in plots.iter().enumerate() {
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
        let plot_type = detect_plot_type(traces[0].as_ref());

        for _ in 0..traces.len() {
            if plot_type == PlotType::Cartesian2D {
                let mut json_trace = JsonTrace::new(all_traces[trace_idx].clone());
                json_trace.set_axis_references(&x_axis, &y_axis);
                all_traces[trace_idx] = Box::new(json_trace);
            }
            trace_idx += 1;
        }
    }
}

fn scale_colorbars_for_subplots(
    all_traces: &mut [Box<dyn Trace + 'static>],
    plot_configs: &[PlotConfig],
    plots: &[(&dyn PlotHelper, usize, usize, usize, usize)],
    rows: usize,
    cols: usize,
    h_gap: f64,
    v_gap: f64,
) {
    let mut trace_idx = 0;

    for (((row, col, rowspan, colspan), _, _, _, _, _), (plot, _, _, _, _)) in
        plot_configs.iter().zip(plots.iter())
    {
        let (_, _, y_start, y_end) =
            calculate_spanning_domain(*row, *col, *rowspan, *colspan, rows, cols, h_gap, v_gap);

        let domain_height = y_end - y_start;
        let traces = plot.get_traces();
        let num_traces = traces.len();

        for _ in 0..num_traces {
            if trace_idx >= all_traces.len() {
                break;
            }

            let trace_json = serde_json::to_value(&all_traces[trace_idx]).ok();

            if let Some(mut trace_value) = trace_json {
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

                    let scaled_trace = JsonTrace::from_value(trace_value);
                    all_traces[trace_idx] = Box::new(scaled_trace);
                }
            }

            trace_idx += 1;
        }
    }
}

fn create_irregular_layout(
    rows: usize,
    cols: usize,
    h_gap: f64,
    v_gap: f64,
    plot_title: Option<Text>,
    plot_configs: &[PlotConfig],
    plots: &[(&dyn PlotHelper, usize, usize, usize, usize)],
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
        layout = layout.title(title.to_plotly());
    }

    for (idx, ((row, col, rowspan, colspan), _, x_config, y_config, x_axis_type, y_axis_type)) in
        plot_configs.iter().enumerate()
    {
        let (x_start, x_end, y_start, y_end) =
            calculate_spanning_domain(*row, *col, *rowspan, *colspan, rows, cols, h_gap, v_gap);

        let mut x_axis = build_axis_from_config(x_config).domain(&[x_start, x_end]);
        if let Some(axis_type_str) = x_axis_type {
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

        let mut y_axis = build_axis_from_config(y_config).domain(&[y_start, y_end]);
        if let Some(axis_type_str) = y_axis_type {
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

    let layout_json = serde_json::to_value(&layout).unwrap();
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

    for (idx, (_, _, x_config, y_config, _, _)) in plot_configs.iter().enumerate() {
        if let Some(ref x_title) = x_config.title {
            if x_title.has_custom_position() {
                let axis_ref = if idx == 0 {
                    "x".to_string()
                } else {
                    format!("x{}", idx + 1)
                };
                annotations.push(x_title.to_axis_annotation(true, &axis_ref, true));
            }
        }

        if let Some(ref y_title) = y_config.title {
            if y_title.has_custom_position() {
                let axis_ref = if idx == 0 {
                    "y".to_string()
                } else {
                    format!("y{}", idx + 1)
                };
                annotations.push(y_title.to_axis_annotation(false, &axis_ref, true));
            }
        }
    }

    for (idx, (_, title_opt, _, _, _, _)) in plot_configs.iter().enumerate() {
        if let Some(title_text) = title_opt {
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
                    .text(&title_text.content)
                    .font(title_text.to_font())
                    .x_ref(&x_ref)
                    .y_ref(&y_ref)
                    .x(title_text.x)
                    .y(title_text.y)
                    .show_arrow(false),
            );
        }
    }

    for (subplot_idx, (plot, _, _, _, _)) in plots.iter().enumerate() {
        if let Some(auto_legend) = CustomLegend::from_plot(*plot) {
            if let Some(legend_annotation) = auto_legend.to_annotation(subplot_idx) {
                annotations.push(legend_annotation);
            }
        }
    }

    annotations
}
