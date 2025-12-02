use plotly::common::Anchor;
use plotly::layout::Annotation;
use plotly::Trace;
use serde::Serialize;
use serde_json::Value;

use crate::common::PlotHelper;
use crate::components::{Orientation, Rgb};

/// Plotly's default color sequence used when traces don't have explicit colors
const PLOTLY_COLORS: &[(u8, u8, u8)] = &[
    (31, 119, 180),  // blue
    (255, 127, 14),  // orange
    (44, 160, 44),   // green
    (214, 39, 40),   // red
    (148, 103, 189), // purple
    (140, 86, 75),   // brown
    (227, 119, 194), // pink
    (127, 127, 127), // gray
    (188, 189, 34),  // olive
    (23, 190, 207),  // cyan
];

#[derive(Clone, Debug, Serialize)]
pub(crate) enum MarkerType {
    Circle,
    Square,
    Diamond,
    Triangle,
    Cross,
    Plus,
    Line,
    #[allow(dead_code)]
    None,
}

impl MarkerType {
    fn to_html(&self, color: &Rgb) -> String {
        let color_str = format!("rgb({},{},{})", color.0, color.1, color.2);

        match self {
            MarkerType::Circle => format!(
                "<span style='color:{};font-size:16px;'>&#9679;</span>",
                color_str
            ),
            MarkerType::Square => format!(
                "<span style='color:{};font-size:16px;'>&#9632;</span>",
                color_str
            ),
            MarkerType::Diamond => format!(
                "<span style='color:{};font-size:16px;'>&#9670;</span>",
                color_str
            ),
            MarkerType::Triangle => format!(
                "<span style='color:{};font-size:16px;'>&#9650;</span>",
                color_str
            ),
            MarkerType::Cross => format!(
                "<span style='color:{};font-size:16px;'>&#10005;</span>",
                color_str
            ),
            MarkerType::Plus => format!(
                "<span style='color:{};font-size:16px;'>&#10010;</span>",
                color_str
            ),
            MarkerType::Line => format!(
                "<span style='color:{};font-size:16px;font-weight:bold;'>&#9644;</span>",
                color_str
            ),
            MarkerType::None => String::new(),
        }
    }
}

#[derive(Clone)]
pub(crate) struct LegendEntry {
    pub(crate) marker_type: MarkerType,
    pub(crate) marker_color: Rgb,
    pub(crate) label: String,
}

impl LegendEntry {
    pub(crate) fn new(marker_type: MarkerType, marker_color: Rgb, label: &str) -> Self {
        Self {
            marker_type,
            marker_color,
            label: label.to_string(),
        }
    }

    fn to_html(&self, font_size: usize, font_color: &Rgb) -> String {
        let marker_html = self.marker_type.to_html(&self.marker_color);
        let label_color = format!("rgb({},{},{})", font_color.0, font_color.1, font_color.2);

        if marker_html.is_empty() {
            format!(
                "    <span style='color:{};font-size:{}px;'>{}</span>",
                label_color, font_size, self.label
            )
        } else {
            format!(
                "    {} <span style='color:{};font-size:{}px;'>{}</span>",
                marker_html, label_color, font_size, self.label
            )
        }
    }
}

#[derive(Clone)]
pub(crate) struct CustomLegend {
    entries: Vec<LegendEntry>,
    x: f64,
    y: f64,
    x_anchor: Anchor,
    y_anchor: Anchor,
    background_color: Option<Rgb>,
    border_color: Option<Rgb>,
    border_width: f64,
    font_family: String,
    font_size: usize,
    font_color: Rgb,
    padding: f64,
    #[allow(dead_code)]
    line_spacing: f64,
    visible: bool,
    title: Option<String>,
    title_font_size: Option<usize>,
    orientation: Orientation,
}

impl Default for CustomLegend {
    fn default() -> Self {
        Self {
            entries: Vec::new(),
            x: 1.02,
            y: 1.0,
            x_anchor: Anchor::Left,
            y_anchor: Anchor::Top,
            background_color: Some(Rgb(255, 255, 255)),
            border_color: None,
            border_width: 0.0,
            font_family: "Arial".to_string(),
            font_size: 12,
            font_color: Rgb(0, 0, 0),
            padding: 5.0,
            line_spacing: 1.5,
            visible: true,
            title: None,
            title_font_size: None,
            orientation: Orientation::Vertical,
        }
    }
}

impl CustomLegend {
    pub(crate) fn to_annotation(
        &self,
        subplot_idx: usize,
        domain: Option<([f64; 2], [f64; 2])>,
    ) -> Option<Annotation> {
        if !self.visible || self.entries.is_empty() {
            return None;
        }

        let entries_html: Vec<String> = self
            .entries
            .iter()
            .map(|entry| entry.to_html(self.font_size, &self.font_color))
            .collect();

        let separator = match self.orientation {
            Orientation::Horizontal => "&nbsp;&nbsp;",
            Orientation::Vertical => "<br>",
        };

        let legend_text = if let Some(title) = &self.title {
            let title_size = self
                .title_font_size
                .filter(|&size| size > 0)
                .unwrap_or(self.font_size + 2);

            let title_color = format!(
                "rgb({},{},{})",
                self.font_color.0, self.font_color.1, self.font_color.2
            );
            let title_html = format!(
                "<span style='font-size:{}px;color:{};'>{}</span>",
                title_size, title_color, title
            );

            let title_separator = match self.orientation {
                Orientation::Horizontal => "&nbsp;&nbsp;",
                Orientation::Vertical => "<br>",
            };

            format!(
                "{}{}{}",
                title_html,
                title_separator,
                entries_html.join(separator)
            )
        } else {
            entries_html.join(separator)
        };

        let (x_ref, y_ref, x_pos, y_pos) = if let Some((domain_x, domain_y)) = domain {
            let width = domain_x[1] - domain_x[0];
            let height = domain_y[1] - domain_y[0];
            let x = domain_x[0] + self.x * width;
            let y = domain_y[0] + self.y * height;
            ("paper".to_string(), "paper".to_string(), x, y)
        } else {
            let xr = if subplot_idx == 0 {
                "x domain".to_string()
            } else {
                format!("x{} domain", subplot_idx + 1)
            };

            let yr = if subplot_idx == 0 {
                "y domain".to_string()
            } else {
                format!("y{} domain", subplot_idx + 1)
            };
            (xr, yr, self.x, self.y)
        };

        let mut annotation = Annotation::new()
            .text(&legend_text)
            .x_ref(&x_ref)
            .y_ref(&y_ref)
            .x(x_pos)
            .y(y_pos)
            .x_anchor(self.x_anchor.clone())
            .y_anchor(self.y_anchor.clone())
            .show_arrow(false)
            .align(plotly::layout::HAlign::Left);

        annotation = annotation.font(
            plotly::common::Font::new()
                .family(&self.font_family)
                .size(self.font_size)
                .color(self.font_color.to_plotly()),
        );

        if let Some(bg_color) = &self.background_color {
            annotation = annotation.background_color(bg_color.to_plotly());
        }

        if let Some(border_color) = &self.border_color {
            annotation = annotation.border_color(border_color.to_plotly());
        } else if self.border_width > 0.0 {
            annotation = annotation.border_color(Rgb(0, 0, 0).to_plotly());
        }

        if self.border_width > 0.0 {
            annotation = annotation.border_width(self.border_width);
        }

        annotation = annotation.border_pad(self.padding);

        Some(annotation)
    }

    pub(crate) fn from_plot(plot: &dyn PlotHelper) -> Option<Self> {
        let traces = plot.get_traces();
        let layout = plot.get_layout();

        let layout_json = serde_json::to_value(layout).ok()?;
        let legend_json = layout_json.get("legend");

        let show_legend = legend_json
            .and_then(|l| l.get("visible"))
            .and_then(|v| v.as_bool())
            .unwrap_or(true);

        if !show_legend {
            return None;
        }

        let mut entries = Vec::new();

        for (trace_index, trace) in traces.iter().enumerate() {
            if let Some(entry) = Self::extract_legend_entry(trace.as_ref(), trace_index) {
                entries.push(entry);
            }
        }

        if entries.is_empty() {
            return None;
        }

        let mut legend = Self {
            entries,
            ..Default::default()
        };

        if let Some(legend_json) = legend_json {
            if let Some(x) = legend_json.get("x").and_then(|v| v.as_f64()) {
                legend.x = x;
            }
            if let Some(y) = legend_json.get("y").and_then(|v| v.as_f64()) {
                legend.y = y;
            }
            if let Some(x_anchor) = legend_json.get("xanchor").and_then(|v| v.as_str()) {
                legend.x_anchor = match x_anchor {
                    "left" => Anchor::Left,
                    "center" => Anchor::Center,
                    "right" => Anchor::Right,
                    _ => Anchor::Left,
                };
            }
            if let Some(y_anchor) = legend_json.get("yanchor").and_then(|v| v.as_str()) {
                legend.y_anchor = match y_anchor {
                    "top" => Anchor::Top,
                    "middle" => Anchor::Middle,
                    "bottom" => Anchor::Bottom,
                    _ => Anchor::Top,
                };
            }

            legend.orientation = legend_json
                .get("orientation")
                .and_then(|v| v.as_str())
                .map(
                    |orientation_str| match orientation_str.to_lowercase().as_str() {
                        "h" | "horizontal" => Orientation::Horizontal,
                        "v" | "vertical" => Orientation::Vertical,
                        _ => Orientation::Vertical,
                    },
                )
                .unwrap_or(Orientation::Vertical);

            if let Some(bg_color) = legend_json
                .get("bgcolor")
                .and_then(|v| v.as_str())
                .and_then(parse_color)
            {
                legend.background_color = Some(bg_color);
            }
            if let Some(border_color) = legend_json
                .get("bordercolor")
                .and_then(|v| v.as_str())
                .and_then(parse_color)
            {
                legend.border_color = Some(border_color);
            }
            if let Some(border_width) = legend_json.get("borderwidth").and_then(|v| v.as_f64()) {
                legend.border_width = border_width;
            }
            if let Some(font_obj) = legend_json.get("font") {
                if let Some(family) = font_obj.get("family").and_then(|v| v.as_str()) {
                    legend.font_family = family.to_string();
                }
                if let Some(size) = font_obj.get("size").and_then(|v| v.as_u64()) {
                    legend.font_size = size as usize;
                }
                if let Some(color) = font_obj
                    .get("color")
                    .and_then(|v| v.as_str())
                    .and_then(parse_color)
                {
                    legend.font_color = color;
                }
            }
            if let Some(title_obj) = legend_json.get("title") {
                if let Some(title_text) = title_obj.get("text").and_then(|v| v.as_str()) {
                    legend.title = Some(title_text.to_string());
                }
                if let Some(title_font) = title_obj.get("font") {
                    if let Some(size) = title_font.get("size").and_then(|v| v.as_u64()) {
                        legend.title_font_size = Some(size as usize);
                    }
                }
            }
        }

        Some(legend)
    }

    fn extract_legend_entry(trace: &dyn Trace, trace_index: usize) -> Option<LegendEntry> {
        let json_str = trace.to_json();
        let trace_json: Value = serde_json::from_str(&json_str).ok()?;
        extract_legend_entry_value(&trace_json, trace_index)
    }
}

fn extract_legend_entry_value(trace_json: &Value, trace_index: usize) -> Option<LegendEntry> {
        let show_legend = trace_json
            .get("showlegend")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);

        if !show_legend {
            return None;
        }

        let name = trace_json
            .get("name")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        if name.is_empty() {
            return None;
        }

    let trace_type = trace_json.get("type").and_then(|v| v.as_str())?;

    let marker_type = map_trace_type_to_marker(trace_type, trace_json);

    let marker_color = extract_trace_color(trace_json, trace_type, trace_index);

    Some(LegendEntry::new(marker_type, marker_color, name))
}

fn map_trace_type_to_marker(trace_type: &str, trace_json: &Value) -> MarkerType {
    match trace_type {
        "scatter" | "scattergl" => {
            let mode = trace_json
                .get("mode")
                .and_then(|v| v.as_str())
                .unwrap_or("");

            if mode.contains("lines") && !mode.contains("markers") {
                MarkerType::Line
            } else {
                extract_marker_symbol(trace_json)
            }
        }
        "bar" => MarkerType::Square,
        "box" => MarkerType::Square,
        "violin" => MarkerType::Diamond,
        "histogram" => MarkerType::Square,
        "pie" => MarkerType::Diamond,
        "heatmap" => MarkerType::Square,
        "contour" => MarkerType::Line,
        "scatter3d" => extract_marker_symbol(trace_json),
        "surface" => MarkerType::Line,
        "mesh3d" => MarkerType::Triangle,
        "scatterpolar" => {
            let mode = trace_json
                .get("mode")
                .and_then(|v| v.as_str())
                .unwrap_or("");

            if mode.contains("lines") && !mode.contains("markers") {
                MarkerType::Line
            } else {
                extract_marker_symbol(trace_json)
            }
        }
        "barpolar" => MarkerType::Square,
        "scattergeo" => extract_marker_symbol(trace_json),
        "scattermapbox" => extract_marker_symbol(trace_json),
        "densitymapbox" => MarkerType::Circle,
        "candlestick" => MarkerType::Line,
        "ohlc" => MarkerType::Line,
        "sankey" => MarkerType::Line,
        "table" => MarkerType::Square,
        _ => MarkerType::Circle,
    }
}

/// Extracts the marker symbol from trace JSON and maps it to MarkerType
fn extract_marker_symbol(trace_json: &Value) -> MarkerType {
    if let Some(symbol) = trace_json
        .get("marker")
        .and_then(|m| m.get("symbol"))
        .and_then(|s| s.as_str())
    {
        map_symbol_to_marker_type(symbol)
    } else {
        MarkerType::Circle
    }
}

/// Maps plotly marker symbol strings to MarkerType enum
fn map_symbol_to_marker_type(symbol: &str) -> MarkerType {
    match symbol {
        s if s.starts_with("circle") => MarkerType::Circle,
        s if s.starts_with("square") => MarkerType::Square,
        s if s.starts_with("diamond") => MarkerType::Diamond,
        s if s.starts_with("triangle") => MarkerType::Triangle,
        s if s.starts_with("cross") => MarkerType::Cross,
        "x" | "x-open" | "x-thin" | "x-dot" => MarkerType::Cross,
        "+" | "plus" | "plus-open" => MarkerType::Plus,
        _ => MarkerType::Circle,
    }
}

fn extract_trace_color(trace_json: &Value, trace_type: &str, trace_index: usize) -> Rgb {
    match trace_type {
        "scatter" | "scattergl" | "scatter3d" | "scatterpolar" | "scattergeo" | "scattermapbox" => {
            if let Some(mode) = trace_json.get("mode").and_then(|v| v.as_str()) {
                if mode.contains("markers") {
                    if let Some(color) = trace_json
                        .get("marker")
                        .and_then(|m| m.get("color"))
                        .and_then(|c| c.as_str())
                        .and_then(parse_color)
                    {
                        return color;
                    }
                }
                if mode.contains("lines") {
                    if let Some(color) = trace_json
                        .get("line")
                        .and_then(|l| l.get("color"))
                        .and_then(|c| c.as_str())
                        .and_then(parse_color)
                    {
                        return color;
                    }
                }
            }
            if let Some(color) = trace_json
                .get("marker")
                .and_then(|m| m.get("color"))
                .and_then(|c| c.as_str())
                .and_then(parse_color)
            {
                return color;
            }
        }
        "bar" | "box" | "histogram" | "barpolar" => {
            if let Some(color) = trace_json
                .get("marker")
                .and_then(|m| m.get("color"))
                .and_then(|c| c.as_str())
                .and_then(parse_color)
            {
                return color;
            }
        }
        "pie" => {
            if let Some(colors) = trace_json.get("marker").and_then(|m| m.get("colors")) {
                if let Some(color_arr) = colors.as_array() {
                    if let Some(first_color) = color_arr
                        .first()
                        .and_then(|v| v.as_str())
                        .and_then(parse_color)
                    {
                        return first_color;
                    }
                }
            }
        }
        "candlestick" | "ohlc" => {
            if let Some(color) = trace_json
                .get("increasing")
                .and_then(|i| i.get("line"))
                .and_then(|l| l.get("color"))
                .and_then(|c| c.as_str())
                .and_then(parse_color)
            {
                return color;
            }
        }
        _ => {}
    }

    // No explicit color found - use plotly's default color sequence based on trace index
    let color_index = trace_index % PLOTLY_COLORS.len();
    let (r, g, b) = PLOTLY_COLORS[color_index];
    Rgb(r, g, b)
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

    if let Some(hex) = color_str.strip_prefix('#') {
        if hex.len() == 6 {
            let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
            return Some(Rgb(r, g, b));
        } else if hex.len() == 3 {
            let r = u8::from_str_radix(&hex[0..1], 16).ok()? * 17;
            let g = u8::from_str_radix(&hex[1..2], 16).ok()? * 17;
            let b = u8::from_str_radix(&hex[2..3], 16).ok()? * 17;
            return Some(Rgb(r, g, b));
        }
    }

    match color_str.to_lowercase().as_str() {
        "black" => Some(Rgb(0, 0, 0)),
        "white" => Some(Rgb(255, 255, 255)),
        "red" => Some(Rgb(255, 0, 0)),
        "green" => Some(Rgb(0, 128, 0)),
        "blue" => Some(Rgb(0, 0, 255)),
        "yellow" => Some(Rgb(255, 255, 0)),
        "cyan" => Some(Rgb(0, 255, 255)),
        "magenta" => Some(Rgb(255, 0, 255)),
        "gray" | "grey" => Some(Rgb(128, 128, 128)),
        "orange" => Some(Rgb(255, 165, 0)),
        "purple" => Some(Rgb(128, 0, 128)),
        "pink" => Some(Rgb(255, 192, 203)),
        "brown" => Some(Rgb(165, 42, 42)),
        "lime" => Some(Rgb(0, 255, 0)),
        "navy" => Some(Rgb(0, 0, 128)),
        "teal" => Some(Rgb(0, 128, 128)),
        "silver" => Some(Rgb(192, 192, 192)),
        "maroon" => Some(Rgb(128, 0, 0)),
        "olive" => Some(Rgb(128, 128, 0)),
        _ => None,
    }
}
