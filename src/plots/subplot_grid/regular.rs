use plotly::{
    layout::{Annotation, Axis as AxisPlotly, GridPattern, Layout as LayoutPlotly, LayoutGrid},
    Trace,
};
use serde_json::Value;

use crate::common::PlotHelper;
use crate::components::{Rgb, Text};

use super::custom_legend::CustomLegend;
use super::shared::{
    detect_plot_type, determine_bar_mode, determine_box_mode, JsonTrace, PlotType,
};
use super::SubplotGrid;

#[derive(Clone)]
struct AxisConfig {
    title: Option<Text>,
    axis_json: Value,
}

fn extract_axis_title_from_annotations(layout_json: &Value, is_x_axis: bool) -> Option<Text> {
    let annotations = layout_json.get("annotations")?.as_array()?;

    for ann in annotations {
        let xref = ann.get("xref")?.as_str()?;
        let yref = ann.get("yref")?.as_str()?;
        let yanchor = ann.get("yanchor").and_then(|v| v.as_str());
        let xanchor = ann.get("xanchor").and_then(|v| v.as_str());

        let is_axis_annotation = if is_x_axis {
            (xref == "paper" || xref.ends_with(" domain"))
                && (yref == "paper" || yref.ends_with(" domain"))
                && yanchor == Some("middle")
                && xanchor == Some("center")
        } else {
            (xref == "paper" || xref.ends_with(" domain"))
                && (yref == "paper" || yref.ends_with(" domain"))
                && yanchor == Some("middle")
                && xanchor == Some("left")
        };

        if !is_axis_annotation {
            continue;
        }

        let text_content = ann.get("text")?.as_str()?.to_string();
        let mut text = Text::from(text_content);

        if let Some(x) = ann.get("x").and_then(|v| v.as_f64()) {
            text = text.x(x);
        }

        if let Some(y) = ann.get("y").and_then(|v| v.as_f64()) {
            text = text.y(y);
        }

        if let Some(font_obj) = ann.get("font") {
            if let Some(size) = font_obj.get("size").and_then(|s| s.as_u64()) {
                if size > 0 {
                    text = text.size(size as usize);
                }
            }

            if let Some(family) = font_obj.get("family").and_then(|f| f.as_str()) {
                if !family.is_empty() {
                    text = text.font(family);
                }
            }

            if let Some(color_str) = font_obj.get("color").and_then(|c| c.as_str()) {
                if let Some(rgb) = parse_color(color_str) {
                    text = text.color(rgb);
                }
            }
        }

        return Some(text);
    }

    None
}

fn parse_color(color_str: &str) -> Option<Rgb> {
    if color_str.starts_with("rgb(") || color_str.starts_with("rgba(") {
        let start = color_str.find('(')?;
        let end = color_str.find(')')?;
        let values = &color_str[start + 1..end];
        let parts: Vec<&str> = values.split(',').map(|s| s.trim()).collect();

        if parts.len() >= 3 {
            let r = parts[0].parse::<u8>().ok()?;
            let g = parts[1].parse::<u8>().ok()?;
            let b = parts[2].parse::<u8>().ok()?;
            return Some(Rgb(r, g, b));
        }
    }

    None
}

struct GridConfig {
    rows: usize,
    cols: usize,
    h_gap: f64,
    v_gap: f64,
}

fn build_axis_from_config(config: &AxisConfig) -> Option<AxisPlotly> {
    let axis_obj = config.axis_json.as_object()?;

    let mut axis = AxisPlotly::new();

    if let Some(title_text) = &config.title {
        if !title_text.has_custom_position() {
            let title = title_text.to_plotly();
            axis = axis.title(title);
        }
    }

    if let Some(show_line) = axis_obj.get("showline").and_then(|v| v.as_bool()) {
        axis = axis.show_line(show_line);
    }

    if let Some(show_grid) = axis_obj.get("showgrid").and_then(|v| v.as_bool()) {
        axis = axis.show_grid(show_grid);
    }

    if let Some(zero_line) = axis_obj.get("zeroline").and_then(|v| v.as_bool()) {
        axis = axis.zero_line(zero_line);
    }

    if let Some(range) = axis_obj.get("range").and_then(|v| v.as_array()) {
        if range.len() == 2 {
            if let (Some(min), Some(max)) = (range[0].as_f64(), range[1].as_f64()) {
                axis = axis.range(vec![min, max]);
            }
        }
    }

    if let Some(separators) = axis_obj.get("separatethousands").and_then(|v| v.as_bool()) {
        axis = axis.separate_thousands(separators);
    }

    if let Some(tick_direction) = axis_obj.get("ticks").and_then(|v| v.as_str()) {
        use plotly::layout::TicksDirection;
        let dir = match tick_direction {
            "outside" => TicksDirection::Outside,
            "inside" => TicksDirection::Inside,
            _ => TicksDirection::Outside,
        };
        axis = axis.ticks(dir);
    }

    Some(axis)
}

fn validate_regular_grid(plots: &[&dyn PlotHelper], rows: usize, cols: usize) {
    if plots.is_empty() {
        panic!(
            "SubplotGrid validation error: plots vector cannot be empty.\n\
            \n\
            Problem: You provided an empty plots vector.\n\
            Solution: Create at least one plot and add it to the plots vector.\n\
            \n\
            Example:\n\
              let plot1 = ScatterPlot::builder().data(&df).x(\"x\").y(\"y\").build();\n\
              SubplotGrid::regular().plots(vec![&plot1])\n\
                .build();"
        );
    }

    if rows == 0 {
        panic!(
            "SubplotGrid validation error: rows must be greater than 0.\n\
            \n\
            Problem: You specified rows = 0, but rows must be at least 1.\n\
            Solution: Set rows to a positive integer (e.g., 1, 2, or 3).\n\
            \n\
            Example:\n\
              SubplotGrid::regular()\n\
                  .plots(vec![&plot1])\n\
                  .rows(2)  // Use positive integer\n\
                  .cols(2)\n\
                  .build();"
        );
    }

    if cols == 0 {
        panic!(
            "SubplotGrid validation error: cols must be greater than 0.\n\
            \n\
            Problem: You specified cols = 0, but cols must be at least 1.\n\
            Solution: Set cols to a positive integer (e.g., 1, 2, or 3).\n\
            \n\
            Example:\n\
              SubplotGrid::regular()\n\
                  .plots(vec![&plot1])\n\
                  .rows(2)\n\
                  .cols(2)  // Use positive integer\n\
                  .build();"
        );
    }

    let grid_capacity = rows * cols;
    let n_plots = plots.len();

    if n_plots > grid_capacity {
        panic!(
            "SubplotGrid validation error: too many plots for grid size.\n\
            \n\
            Problem: You provided {} plot(s) but the grid only has {} cells ({}x{} = {}).\n\
            Solution: Either reduce the number of plots or increase the grid size.\n\
            \n\
            Option 1 - Reduce plots:\n\
              Use {} plots instead of {}\n\
            \n\
            Option 2 - Increase grid size:\n\
              Example calculations:\n\
              - For {} plots: {}x{} grid works\n\
              - For {} plots: {}x{} grid works",
            n_plots,
            grid_capacity,
            rows,
            cols,
            grid_capacity,
            grid_capacity,
            n_plots,
            n_plots,
            (n_plots as f64).sqrt().ceil() as usize,
            ((n_plots as f64) / 2.0).ceil() as usize,
            n_plots,
            ((n_plots + 1) as f64).sqrt().ceil() as usize,
            ((n_plots + 1) as f64 / 2.0).ceil() as usize
        );
    }

    for (idx, plot) in plots.iter().enumerate() {
        let traces = plot.get_traces();
        let plot_type = detect_plot_type(traces[0].as_ref());
        if plot_type != PlotType::Cartesian2D {
            panic!(
                "SubplotGrid validation error: unsupported plot type.\n\
                \n\
                Problem: Plot at index {} is a {:?} plot, but SubplotGrid currently only supports 2D Cartesian plots.\n\
                Solution: Use only 2D plot types (ScatterPlot, LinePlot, BarPlot, BoxPlot, Histogram, etc.).",
                idx, plot_type
            );
        }
    }
}

pub(super) fn build_regular(
    plots: Vec<&dyn PlotHelper>,
    rows: Option<usize>,
    cols: Option<usize>,
    title: Option<Text>,
    h_gap: Option<f64>,
    v_gap: Option<f64>,
    legends: Option<Vec<Option<&CustomLegend>>>,
) -> SubplotGrid {
    let rows = rows.unwrap_or(1);
    let cols = cols.unwrap_or(1);
    let h_gap = h_gap.unwrap_or(0.1);
    let v_gap = v_gap.unwrap_or(0.1);

    validate_regular_grid(&plots, rows, cols);

    let mut all_traces: Vec<Box<dyn Trace + 'static>> = Vec::new();
    let mut plot_titles: Vec<Option<Text>> = Vec::new();
    let mut axis_configs: Vec<(AxisConfig, AxisConfig)> = Vec::new();

    for (plot_idx, plot) in plots.iter().enumerate() {
        let traces = plot.get_traces();
        let plot_type = detect_plot_type(traces[0].as_ref());

        plot_titles.push(plot.get_main_title_text());

        let layout_json = serde_json::to_value(plot.get_layout()).unwrap_or(Value::Null);
        let x_axis_json = layout_json.get("xaxis").cloned().unwrap_or(Value::Null);
        let y_axis_json = layout_json.get("yaxis").cloned().unwrap_or(Value::Null);

        let x_title = plot
            .get_x_title_text()
            .or_else(|| extract_axis_title_from_annotations(&layout_json, true));

        let y_title = plot
            .get_y_title_text()
            .or_else(|| extract_axis_title_from_annotations(&layout_json, false));

        let x_config = AxisConfig {
            title: x_title,
            axis_json: x_axis_json,
        };

        let y_config = AxisConfig {
            title: y_title,
            axis_json: y_axis_json,
        };

        axis_configs.push((x_config, y_config));

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

        if let Some(serialized_traces) = plot.get_serialized_traces() {
            for trace_value in serialized_traces {
                let modified_trace = match plot_type {
                    PlotType::Cartesian2D => {
                        let mut json_trace = JsonTrace::from_value(trace_value);
                        json_trace.set_axis_references(&x_axis, &y_axis);
                        Box::new(json_trace)
                    }
                    _ => {
                        let json_trace = JsonTrace::from_value(trace_value);
                        Box::new(json_trace)
                    }
                };
                all_traces.push(modified_trace);
            }
        } else {
            for trace in traces {
                let modified_trace = match plot_type {
                    PlotType::Cartesian2D => {
                        let mut json_trace = JsonTrace::new(trace.clone());
                        json_trace.set_axis_references(&x_axis, &y_axis);
                        Box::new(json_trace)
                    }
                    _ => trace.clone(),
                };
                all_traces.push(modified_trace);
            }
        }
    }

    let grid_config = GridConfig {
        rows,
        cols,
        h_gap,
        v_gap,
    };

    let (layout, layout_json) = create_regular_layout(
        &grid_config,
        title,
        &plot_titles,
        &axis_configs,
        legends,
        &plots,
    );

    SubplotGrid {
        traces: all_traces,
        layout,
        layout_json: Some(layout_json),
    }
}

fn create_regular_layout(
    grid_config: &GridConfig,
    plot_title: Option<Text>,
    subplot_titles: &[Option<Text>],
    axis_configs: &[(AxisConfig, AxisConfig)],
    legends: Option<Vec<Option<&CustomLegend>>>,
    plots: &[&dyn PlotHelper],
) -> (LayoutPlotly, Value) {
    let mut layout = LayoutPlotly::new().show_legend(false);

    if let Some(bar_mode) = determine_bar_mode(plots) {
        layout = layout.bar_mode(bar_mode);
    }

    if let Some(box_mode) = determine_box_mode(plots) {
        layout = layout.box_mode(box_mode);
    }

    if let Some(title) = plot_title {
        layout = layout.title(title.to_plotly());
    }

    let grid = LayoutGrid::new()
        .rows(grid_config.rows)
        .columns(grid_config.cols)
        .pattern(GridPattern::Independent)
        .x_gap(grid_config.h_gap)
        .y_gap(grid_config.v_gap);

    layout = layout.grid(grid);

    for (idx, (x_config, y_config)) in axis_configs.iter().enumerate() {
        if let Some(x_axis) = build_axis_from_config(x_config) {
            layout = match idx {
                0 => layout.x_axis(x_axis),
                1 => layout.x_axis2(x_axis),
                2 => layout.x_axis3(x_axis),
                3 => layout.x_axis4(x_axis),
                4 => layout.x_axis5(x_axis),
                5 => layout.x_axis6(x_axis),
                6 => layout.x_axis7(x_axis),
                7 => layout.x_axis8(x_axis),
                _ => layout,
            };
        }

        if let Some(y_axis) = build_axis_from_config(y_config) {
            layout = match idx {
                0 => layout.y_axis(y_axis),
                1 => layout.y_axis2(y_axis),
                2 => layout.y_axis3(y_axis),
                3 => layout.y_axis4(y_axis),
                4 => layout.y_axis5(y_axis),
                5 => layout.y_axis6(y_axis),
                6 => layout.y_axis7(y_axis),
                7 => layout.y_axis8(y_axis),
                _ => layout,
            };
        }
    }

    let mut annotations = Vec::new();

    for (idx, (x_config, y_config)) in axis_configs.iter().enumerate() {
        if let Some(ref x_title) = x_config.title {
            if x_title.has_custom_position() {
                let axis_ref = if idx == 0 {
                    "x".to_string()
                } else {
                    format!("x{}", idx + 1)
                };
                let annotation = x_title.to_axis_annotation(true, &axis_ref, true);
                annotations.push(annotation);
            }
        }

        if let Some(ref y_title) = y_config.title {
            if y_title.has_custom_position() {
                let axis_ref = if idx == 0 {
                    "y".to_string()
                } else {
                    format!("y{}", idx + 1)
                };
                let annotation = y_title.to_axis_annotation(false, &axis_ref, true);
                annotations.push(annotation);
            }
        }
    }

    for (idx, title_opt) in subplot_titles.iter().enumerate() {
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

            let ann = Annotation::new()
                .text(&title_text.content)
                .font(title_text.to_font())
                .x_ref(&x_ref)
                .y_ref(&y_ref)
                .x(title_text.x)
                .y(title_text.y)
                .show_arrow(false);

            annotations.push(ann);
        }
    }

    if let Some(legend_configs) = legends {
        for (subplot_idx, legend_opt) in legend_configs.iter().enumerate() {
            if let Some(legend) = legend_opt {
                if let Some(legend_annotation) = legend.to_annotation(subplot_idx) {
                    annotations.push(legend_annotation);
                }
            }
        }
    } else {
        for (subplot_idx, plot) in plots.iter().enumerate() {
            if let Some(auto_legend) = CustomLegend::from_plot(*plot) {
                if let Some(legend_annotation) = auto_legend.to_annotation(subplot_idx) {
                    annotations.push(legend_annotation);
                }
            }
        }
    }

    if !annotations.is_empty() {
        layout = layout.annotations(annotations);
    }

    let layout_json = serde_json::to_value(&layout).unwrap();

    (layout, layout_json)
}
