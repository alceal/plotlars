use plotly::{layout::Axis as AxisPlotly, Trace};
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};
use std::collections::HashMap;

use crate::common::PlotHelper;
use crate::components::{Rgb, Text};

pub(super) const DEFAULT_COLORWAY: &[(u8, u8, u8)] = &[
    (31, 119, 180),
    (255, 127, 14),
    (44, 160, 44),
    (214, 39, 40),
    (148, 103, 189),
    (140, 86, 75),
    (227, 119, 194),
    (127, 127, 127),
    (188, 189, 34),
    (23, 190, 207),
];

#[derive(Debug, Clone, PartialEq)]
pub(super) enum PlotType {
    Cartesian2D,
    Cartesian3D,
    Polar,
    Domain,
    Mapbox,
    Geo,
}

#[derive(Clone)]
pub(super) struct NonCartesianLayout {
    pub plot_type: PlotType,
    pub domain_x: [f64; 2],
    pub domain_y: [f64; 2],
    pub layout_fragment: Option<Value>,
    pub subplot_ref: String,
}

#[derive(Clone)]
pub(super) struct AxisConfig {
    pub(super) title: Option<Text>,
    pub(super) axis_json: Value,
}

pub(super) fn detect_plot_type(trace: &(dyn Trace + 'static)) -> PlotType {
    let json_str = trace.to_json();
    if let Ok(json) = serde_json::from_str::<Value>(&json_str) {
        if let Some(trace_type) = json.get("type").and_then(|v| v.as_str()) {
            match trace_type {
                "scatter3d" | "mesh3d" | "surface" | "isosurface" | "volume" | "streamtube"
                | "cone" => PlotType::Cartesian3D,
                "scatterpolar" | "scatterpolargl" | "barpolar" => PlotType::Polar,
                "pie" | "sankey" | "table" => PlotType::Domain,
                "scattermapbox" | "densitymapbox" => PlotType::Mapbox,
                "scattergeo" => PlotType::Geo,
                _ => PlotType::Cartesian2D,
            }
        } else {
            PlotType::Cartesian2D
        }
    } else {
        PlotType::Cartesian2D
    }
}

pub(super) fn adjust_domain_for_type(
    plot_type: PlotType,
    x_start: f64,
    x_end: f64,
    y_start: f64,
    y_end: f64,
) -> ([f64; 2], [f64; 2]) {
    let height = y_end - y_start;
    let padding_ratio = match plot_type {
        PlotType::Polar => 0.18, // extra room for polar titles
        PlotType::Cartesian3D => 0.12,
        PlotType::Domain => 0.08,
        PlotType::Mapbox | PlotType::Geo => 0.06,
        PlotType::Cartesian2D => 0.0,
    };

    let padding = height * padding_ratio;
    let adjusted_y = [y_start + padding / 2.0, y_end - padding / 2.0];
    ([x_start, x_end], adjusted_y)
}

pub(super) fn inject_non_cartesian_domains(
    layout_json: &mut Value,
    configs: &[NonCartesianLayout],
) {
    for info in configs {
        match info.plot_type {
            PlotType::Cartesian3D => {
                let scene_key = info.subplot_ref.clone();
                let mut scene_obj = info
                    .layout_fragment
                    .clone()
                    .unwrap_or_else(|| Value::Object(Map::new()));
                scene_obj["domain"] = json!({
                    "x": info.domain_x,
                    "y": info.domain_y
                });
                layout_json[scene_key] = scene_obj;
            }
            PlotType::Polar => {
                let polar_key = info.subplot_ref.clone();
                let mut polar_obj = info
                    .layout_fragment
                    .clone()
                    .unwrap_or_else(|| Value::Object(Map::new()));
                polar_obj["domain"] = json!({
                    "x": info.domain_x,
                    "y": info.domain_y
                });
                layout_json[polar_key] = polar_obj;
            }
            PlotType::Mapbox => {
                let mapbox_key = info.subplot_ref.clone();
                let mut mapbox_obj = info
                    .layout_fragment
                    .clone()
                    .unwrap_or_else(|| Value::Object(Map::new()));
                mapbox_obj["domain"] = json!({
                    "x": info.domain_x,
                    "y": info.domain_y
                });
                layout_json[mapbox_key] = mapbox_obj;
            }
            PlotType::Geo => {
                let geo_key = info.subplot_ref.clone();
                let mut geo_obj = info
                    .layout_fragment
                    .clone()
                    .unwrap_or_else(|| Value::Object(Map::new()));
                geo_obj["domain"] = json!({
                    "x": info.domain_x,
                    "y": info.domain_y
                });
                layout_json[geo_key] = geo_obj;
            }
            PlotType::Domain | PlotType::Cartesian2D => {}
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub(super) struct JsonTrace {
    #[serde(flatten)]
    data: Value,
}

impl JsonTrace {
    pub(super) fn new(trace: Box<dyn Trace + 'static>) -> Self {
        let data = serde_json::to_value(&trace)
            .expect("Failed to serialize trace with custom Serialize implementation");
        Self { data }
    }

    pub(super) fn from_value(value: Value) -> Self {
        Self { data: value }
    }

    pub(super) fn data(&self) -> &Value {
        &self.data
    }

    pub(super) fn set_axis_references(&mut self, x_axis: &str, y_axis: &str) {
        if let Some(obj) = self.data.as_object_mut() {
            obj.insert("xaxis".to_string(), json!(x_axis));
            obj.insert("yaxis".to_string(), json!(y_axis));
        }
    }

    pub(super) fn set_domain(&mut self, domain_x: [f64; 2], domain_y: [f64; 2]) {
        use serde_json::Map;

        let mut domain_obj = self
            .data
            .get_mut("domain")
            .and_then(|d| d.as_object_mut())
            .cloned()
            .unwrap_or_else(Map::new);

        domain_obj.insert("x".to_string(), json!(domain_x));
        domain_obj.insert("y".to_string(), json!(domain_y));

        if let Some(obj) = self.data.as_object_mut() {
            obj.insert("domain".to_string(), Value::Object(domain_obj));
        }
    }

    pub(super) fn set_scene_reference(&mut self, scene: &str) {
        if let Some(obj) = self.data.as_object_mut() {
            obj.insert("scene".to_string(), json!(scene));
        }
    }

    pub(super) fn set_subplot_reference(&mut self, subplot: &str) {
        if let Some(obj) = self.data.as_object_mut() {
            obj.insert("subplot".to_string(), json!(subplot));
        }
    }

    pub(super) fn ensure_color(&mut self, global_index: usize) {
        let trace_type = self
            .data
            .get("type")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        let default_color = DEFAULT_COLORWAY[global_index % DEFAULT_COLORWAY.len()];
        let color_str = format!("rgb({},{},{})", default_color.0, default_color.1, default_color.2);

        match trace_type {
            "scatter" | "scattergl" | "scatter3d" | "scatterpolar" | "scattergeo"
            | "scattermapbox" => {
                let mode = self
                    .data
                    .get("mode")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");

                let has_marker_color = self
                    .data
                    .get("marker")
                    .and_then(|m| m.get("color"))
                    .is_some();

                let has_line_color = self
                    .data
                    .get("line")
                    .and_then(|l| l.get("color"))
                    .is_some();

                if mode.contains("markers") && !has_marker_color {
                    if self.data.get("marker").is_none() {
                        self.data["marker"] = Value::Object(Map::new());
                    }
                    if let Some(obj) = self.data.get_mut("marker").and_then(|m| m.as_object_mut()) {
                        obj.insert("color".to_string(), json!(color_str));
                    }
                } else if mode.contains("lines") && !has_line_color {
                    if self.data.get("line").is_none() {
                        self.data["line"] = Value::Object(Map::new());
                    }
                    if let Some(obj) = self.data.get_mut("line").and_then(|l| l.as_object_mut()) {
                        obj.insert("color".to_string(), json!(color_str));
                    }
                } else if !has_marker_color && !has_line_color {
                    if self.data.get("marker").is_none() {
                        self.data["marker"] = Value::Object(Map::new());
                    }
                    if let Some(obj) = self.data.get_mut("marker").and_then(|m| m.as_object_mut()) {
                        obj.insert("color".to_string(), json!(color_str));
                    }
                }
            }
            "bar" | "box" | "histogram" | "barpolar" => {
                let has_color = self
                    .data
                    .get("marker")
                    .and_then(|m| m.get("color"))
                    .is_some();
                if !has_color {
                    if self.data.get("marker").is_none() {
                        self.data["marker"] = Value::Object(Map::new());
                    }
                    if let Some(obj) = self.data.get_mut("marker").and_then(|m| m.as_object_mut()) {
                        obj.insert("color".to_string(), json!(color_str));
                    }
                }
            }
            _ => {}
        }
    }
}

impl Trace for JsonTrace {
    fn to_json(&self) -> String {
        serde_json::to_string(&self.data).unwrap()
    }
}

pub(super) fn extract_axis_title_from_annotations(
    layout_json: &Value,
    is_x_axis: bool,
) -> Option<Text> {
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

pub(super) fn parse_color(color_str: &str) -> Option<Rgb> {
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

pub(super) fn build_axis_from_config(config: &AxisConfig) -> AxisPlotly {
    let mut axis = AxisPlotly::new();

    // Note: Titles are now handled as annotations for better grid positioning
    // See annotation creation loop in create_irregular_layout

    if let Some(axis_obj) = config.axis_json.as_object() {
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
    }

    axis
}

pub(super) fn validate_irregular_grid(
    plots: &[(&dyn PlotHelper, usize, usize, usize, usize)],
    rows: usize,
    cols: usize,
) {
    validate_not_empty(plots);
    validate_dimensions(rows, cols);
    validate_individual_plots(plots, rows, cols);
    validate_no_overlaps(plots);
    let plot_refs: Vec<&dyn PlotHelper> = plots.iter().map(|(p, _, _, _, _)| *p).collect();
    validate_plot_types(&plot_refs);
}

fn validate_not_empty(plots: &[(&dyn PlotHelper, usize, usize, usize, usize)]) {
    if plots.is_empty() {
        panic!(
            "SubplotGrid validation error: plots vector cannot be empty.\n\
            \n\
            Problem: You provided an empty plots vector.\n\
            Solution: Create at least one plot and add it to the plots vector.\n\
            \n\
            Example:\n\
              let plot1 = ScatterPlot::builder().data(&df).x(\"x\").y(\"y\").build();\n\
              SubplotGrid::irregular()\n\
                .plots(vec![(&plot1, 0, 0, 1, 1)])\n\
                .rows(2).cols(2)\n\
                .build();"
        );
    }
}

fn validate_dimensions(rows: usize, cols: usize) {
    if rows == 0 {
        panic!(
            "SubplotGrid validation error: rows must be greater than 0.\n\
            \n\
            Problem: You specified rows = 0, but rows must be at least 1.\n\
            Solution: Set rows to a positive integer (e.g., 1, 2, or 3)."
        );
    }

    if cols == 0 {
        panic!(
            "SubplotGrid validation error: cols must be greater than 0.\n\
            \n\
            Problem: You specified cols = 0, but cols must be at least 1.\n\
            Solution: Set cols to a positive integer (e.g., 1, 2, or 3)."
        );
    }
}

fn validate_individual_plots(
    plots: &[(&dyn PlotHelper, usize, usize, usize, usize)],
    rows: usize,
    cols: usize,
) {
    for (i, (_, row, col, rowspan, colspan)) in plots.iter().enumerate() {
        if *rowspan == 0 || *colspan == 0 {
            panic!(
                "SubplotGrid validation error: rowspan and colspan must be at least 1.\n\
                \n\
                Problem: Plot {} has rowspan={}, colspan={}\n\
                Solution: Use positive integers for spans (e.g., 1, 2, 3).\n\
                \n\
                Example:\n\
                  .plots(vec![(&plot1, {}, {}, 1, 1)])  // Valid spans",
                i, rowspan, colspan, row, col
            );
        }

        if *row >= rows {
            panic!(
                "SubplotGrid validation error: plot position out of bounds.\n\
                \n\
                Problem: Plot {} is positioned at row {}, but the grid only has {} rows (0-indexed: 0-{}).\n\
                Solution: Use a row index between 0 and {}, or increase the grid rows.",
                i, row, rows, rows - 1, rows - 1
            );
        }

        if *col >= cols {
            panic!(
                "SubplotGrid validation error: plot position out of bounds.\n\
                \n\
                Problem: Plot {} is positioned at column {}, but the grid only has {} columns (0-indexed: 0-{}).\n\
                Solution: Use a column index between 0 and {}, or increase the grid columns.",
                i, col, cols, cols - 1, cols - 1
            );
        }

        if row + rowspan > rows {
            panic!(
                "SubplotGrid validation error: plot span exceeds grid boundary.\n\
                \n\
                Problem: Plot {} at row {} with rowspan {} extends beyond the grid (ends at row {}, but grid has {} rows).\n\
                Solution: Either reduce rowspan to {} or increase grid rows to {}.",
                i, row, rowspan, row + rowspan, rows, rows - row, row + rowspan
            );
        }

        if col + colspan > cols {
            panic!(
                "SubplotGrid validation error: plot span exceeds grid boundary.\n\
                \n\
                Problem: Plot {} at column {} with colspan {} extends beyond the grid (ends at col {}, but grid has {} cols).\n\
                Solution: Either reduce colspan to {} or increase grid columns to {}.",
                i, col, colspan, col + colspan, cols, cols - col, col + colspan
            );
        }
    }
}

fn validate_no_overlaps(plots: &[(&dyn PlotHelper, usize, usize, usize, usize)]) {
    let mut cell_owners: HashMap<(usize, usize), usize> = HashMap::new();

    for (plot_idx, (_plot, row, col, rowspan, colspan)) in plots.iter().enumerate() {
        for r in *row..(*row + *rowspan) {
            for c in *col..(*col + *colspan) {
                if let Some(existing_plot_idx) = cell_owners.insert((r, c), plot_idx) {
                    let (_, row1, col1, rs1, cs1) = plots[existing_plot_idx];
                    let (_, row2, col2, rs2, cs2) = plots[plot_idx];

                    panic!(
                        "SubplotGrid validation error: overlapping plots detected.\n\
                        \n\
                        Problem: Plots {} and {} both occupy cell ({}, {}).\n\
                        \n\
                        Plot {} position:\n\
                          - row={}, col={}, rowspan={}, colspan={}\n\
                          - Occupies cells: ({},{}) to ({},{})\n\
                        \n\
                        Plot {} position:\n\
                          - row={}, col={}, rowspan={}, colspan={}\n\
                          - Occupies cells: ({},{}) to ({},{})\n\
                        \n\
                        Solution: Adjust plot positions or spans to avoid overlaps.",
                        existing_plot_idx,
                        plot_idx,
                        r,
                        c,
                        existing_plot_idx,
                        row1,
                        col1,
                        rs1,
                        cs1,
                        row1,
                        col1,
                        row1 + rs1 - 1,
                        col1 + cs1 - 1,
                        plot_idx,
                        row2,
                        col2,
                        rs2,
                        cs2,
                        row2,
                        col2,
                        row2 + rs2 - 1,
                        col2 + cs2 - 1
                    );
                }
            }
        }
    }
}

pub(super) fn validate_plot_types(plots: &[&dyn PlotHelper]) {
    for (idx, plot) in plots.iter().enumerate() {
        let traces = plot.get_traces();
        if !traces.is_empty() {
            let plot_type = detect_plot_type(traces[0].as_ref());
            let supported = matches!(
                plot_type,
                PlotType::Cartesian2D
                    | PlotType::Cartesian3D
                    | PlotType::Polar
                    | PlotType::Domain
                    | PlotType::Mapbox
                    | PlotType::Geo
            );
            if !supported {
                panic!(
                    "SubplotGrid validation error: unsupported plot type.\n\
                    Problem: Plot {} uses an unsupported trace family ({:?}).\n\
                    Solution: Use cartesian, polar, 3D scene, domain-based (pie/sankey/table), geo, or mapbox traces.",
                    idx, plot_type
                );
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub(super) fn calculate_spanning_domain(
    row: usize,
    col: usize,
    rowspan: usize,
    colspan: usize,
    rows: usize,
    cols: usize,
    h_gap: f64,
    v_gap: f64,
) -> (f64, f64, f64, f64) {
    let col_width = (1.0 - h_gap * (cols - 1) as f64) / cols as f64;
    let row_height = (1.0 - v_gap * (rows - 1) as f64) / rows as f64;

    let x_start = col as f64 * (col_width + h_gap);
    let x_end = x_start + (col_width * colspan as f64) + (h_gap * (colspan - 1) as f64);

    let y_from_top_start = row as f64 * (row_height + v_gap);
    let y_from_top_end =
        y_from_top_start + (row_height * rowspan as f64) + (v_gap * (rowspan - 1) as f64);

    let y_start = 1.0 - y_from_top_end;
    let y_end = 1.0 - y_from_top_start;

    (x_start, x_end, y_start, y_end)
}

pub(super) fn detect_axis_type_from_traces(traces: &[Box<dyn Trace + 'static>]) -> Option<String> {
    if traces.is_empty() {
        return None;
    }

    let first_trace_json = traces[0].to_json();
    let trace_value: Value = serde_json::from_str(&first_trace_json).ok()?;

    if let Some(x_array) = trace_value.get("x").and_then(|v| v.as_array()) {
        if let Some(first_x) = x_array.first() {
            if let Some(x_str) = first_x.as_str() {
                if x_str.len() >= 10
                    && x_str.chars().nth(4) == Some('-')
                    && x_str.chars().nth(7) == Some('-')
                {
                    return Some("date".to_string());
                }
                return Some("category".to_string());
            }
            if first_x.is_number() {
                return Some("linear".to_string());
            }
        }
    }

    None
}

pub(super) fn determine_bar_mode(plots: &[&dyn PlotHelper]) -> Option<plotly::layout::BarMode> {
    let mut has_histogram = false;
    let mut has_barplot = false;

    for plot in plots {
        let traces = plot.get_traces();

        for trace in traces {
            let json_str = trace.to_json();
            if let Ok(json) = serde_json::from_str::<Value>(&json_str) {
                if let Some(trace_type) = json.get("type").and_then(|v| v.as_str()) {
                    match trace_type {
                        "histogram" => has_histogram = true,
                        "bar" => has_barplot = true,
                        _ => {}
                    }
                }
            }
        }
    }

    if has_histogram {
        Some(plotly::layout::BarMode::Overlay)
    } else if has_barplot {
        Some(plotly::layout::BarMode::Group)
    } else {
        None
    }
}

pub(super) fn determine_box_mode(plots: &[&dyn PlotHelper]) -> Option<plotly::layout::BoxMode> {
    let mut has_grouped_box = false;

    for plot in plots {
        let traces = plot.get_traces();

        if traces.len() > 1 {
            for trace in traces {
                let json_str = trace.to_json();
                if let Ok(json) = serde_json::from_str::<Value>(&json_str) {
                    if let Some(trace_type) = json.get("type").and_then(|v| v.as_str()) {
                        if trace_type == "box" {
                            has_grouped_box = true;
                            break;
                        }
                    }
                }
            }
        }

        if has_grouped_box {
            break;
        }
    }

    if has_grouped_box {
        Some(plotly::layout::BoxMode::Group)
    } else {
        None
    }
}
