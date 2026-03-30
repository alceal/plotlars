use plotly::common::Anchor;
use plotly::layout::Annotation;
use serde::Serialize;

use crate::converters::components as conv;
use plotlars_core::components::{Mode, Orientation, Rgb};
use plotlars_core::ir::layout::LayoutIR;
use plotlars_core::ir::trace::TraceIR;

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
    #[allow(dead_code)]
    Cross,
    #[allow(dead_code)]
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
                .color(conv::convert_rgb(&self.font_color)),
        );

        if let Some(bg_color) = &self.background_color {
            annotation = annotation.background_color(conv::convert_rgb(bg_color));
        }

        if let Some(border_color) = &self.border_color {
            annotation = annotation.border_color(conv::convert_rgb(border_color));
        } else if self.border_width > 0.0 {
            annotation = annotation.border_color(conv::convert_rgb(&Rgb(0, 0, 0)));
        }

        if self.border_width > 0.0 {
            annotation = annotation.border_width(self.border_width);
        }

        annotation = annotation.border_pad(self.padding);

        Some(annotation)
    }

    /// Build a CustomLegend directly from IR types, bypassing plotly serialization.
    /// This produces accurate colors because it reads from the IR structs directly.
    pub(crate) fn from_ir(ir_traces: &[TraceIR], ir_layout: &LayoutIR) -> Option<Self> {
        let mut entries = Vec::new();

        for (i, trace) in ir_traces.iter().enumerate() {
            if let Some(entry) = extract_legend_entry_from_ir(trace, i) {
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

        // Apply legend styling from IR layout
        if let Some(ref leg) = ir_layout.legend {
            if let Some(x) = leg.x {
                legend.x = x;
            }
            if let Some(y) = leg.y {
                legend.y = y;
            }
            if let Some(ref orient) = leg.orientation {
                legend.orientation = orient.clone();
            }
            if let Some(ref bg) = leg.background_color {
                legend.background_color = Some(*bg);
            }
            if let Some(ref bc) = leg.border_color {
                legend.border_color = Some(*bc);
            }
            if let Some(bw) = leg.border_width {
                legend.border_width = bw as f64;
            }
            if let Some(ref family) = leg.font {
                legend.font_family = family.clone();
            }
        }

        if let Some(ref legend_title) = ir_layout.legend_title {
            legend.title = Some(legend_title.content.clone());
            if legend_title.size > 0 {
                legend.title_font_size = Some(legend_title.size);
            }
        }

        Some(legend)
    }
}


// ---------------------------------------------------------------------------
// IR-based legend extraction
// ---------------------------------------------------------------------------

fn extract_legend_entry_from_ir(trace: &TraceIR, trace_index: usize) -> Option<LegendEntry> {
    let default_color = || {
        let (r, g, b) = PLOTLY_COLORS[trace_index % PLOTLY_COLORS.len()];
        Rgb(r, g, b)
    };

    match trace {
        TraceIR::ScatterPlot(ir) => {
            let name = ir.name.as_deref()?;
            if name.is_empty() { return None; }
            if ir.show_legend == Some(false) { return None; }
            let color = ir.marker.as_ref().and_then(|m| m.color).unwrap_or_else(default_color);
            let marker = ir.marker.as_ref().and_then(|m| m.shape.as_ref())
                .map(|_| MarkerType::Circle).unwrap_or(MarkerType::Circle);
            Some(LegendEntry::new(marker, color, name))
        }
        TraceIR::BarPlot(ir) => {
            let name = ir.name.as_deref()?;
            if name.is_empty() { return None; }
            if ir.show_legend == Some(false) { return None; }
            let color = ir.marker.as_ref().and_then(|m| m.color).unwrap_or_else(default_color);
            Some(LegendEntry::new(MarkerType::Square, color, name))
        }
        TraceIR::BoxPlot(ir) => {
            let name = ir.name.as_deref()?;
            if name.is_empty() { return None; }
            if ir.show_legend == Some(false) { return None; }
            let color = ir.marker.as_ref().and_then(|m| m.color).unwrap_or_else(default_color);
            Some(LegendEntry::new(MarkerType::Square, color, name))
        }
        TraceIR::LinePlot(ir) => {
            let name = ir.name.as_deref()?;
            if name.is_empty() { return None; }
            if ir.show_legend == Some(false) { return None; }
            let color = ir.line.as_ref().and_then(|l| l.color)
                .or_else(|| ir.marker.as_ref().and_then(|m| m.color))
                .unwrap_or_else(default_color);
            let is_line = ir.mode.is_none_or(|m| matches!(m, Mode::Lines | Mode::LinesMarkers | Mode::LinesText));
            let marker = if is_line { MarkerType::Line } else { MarkerType::Circle };
            Some(LegendEntry::new(marker, color, name))
        }
        TraceIR::TimeSeriesPlot(ir) => {
            let name = ir.name.as_deref()?;
            if name.is_empty() { return None; }
            if ir.show_legend == Some(false) { return None; }
            let color = ir.line.as_ref().and_then(|l| l.color)
                .or_else(|| ir.marker.as_ref().and_then(|m| m.color))
                .unwrap_or_else(default_color);
            let is_line = ir.mode.is_none_or(|m| matches!(m, Mode::Lines | Mode::LinesMarkers | Mode::LinesText));
            let marker = if is_line { MarkerType::Line } else { MarkerType::Circle };
            Some(LegendEntry::new(marker, color, name))
        }
        TraceIR::Histogram(ir) => {
            let name = ir.name.as_deref()?;
            if name.is_empty() { return None; }
            if ir.show_legend == Some(false) { return None; }
            let color = ir.marker.as_ref().and_then(|m| m.color).unwrap_or_else(default_color);
            Some(LegendEntry::new(MarkerType::Square, color, name))
        }
        TraceIR::ScatterPolar(ir) => {
            let name = ir.name.as_deref()?;
            if name.is_empty() { return None; }
            if ir.show_legend == Some(false) { return None; }
            let color = ir.marker.as_ref().and_then(|m| m.color)
                .or_else(|| ir.line.as_ref().and_then(|l| l.color))
                .unwrap_or_else(default_color);
            let is_line = ir.mode.is_some_and(|m| matches!(m, Mode::Lines | Mode::LinesMarkers | Mode::LinesText));
            let marker = if is_line { MarkerType::Line } else { MarkerType::Circle };
            Some(LegendEntry::new(marker, color, name))
        }
        TraceIR::Scatter3dPlot(ir) => {
            let name = ir.name.as_deref()?;
            if name.is_empty() { return None; }
            if ir.show_legend == Some(false) { return None; }
            let color = ir.marker.as_ref().and_then(|m| m.color).unwrap_or_else(default_color);
            Some(LegendEntry::new(MarkerType::Circle, color, name))
        }
        TraceIR::ScatterGeo(ir) => {
            let name = ir.name.as_deref()?;
            if name.is_empty() { return None; }
            if ir.show_legend == Some(false) { return None; }
            let color = ir.marker.as_ref().and_then(|m| m.color)
                .or_else(|| ir.line.as_ref().and_then(|l| l.color))
                .unwrap_or_else(default_color);
            Some(LegendEntry::new(MarkerType::Circle, color, name))
        }
        TraceIR::ScatterMap(ir) => {
            let name = ir.name.as_deref()?;
            if name.is_empty() { return None; }
            if ir.show_legend == Some(false) { return None; }
            let color = ir.marker.as_ref().and_then(|m| m.color).unwrap_or_else(default_color);
            Some(LegendEntry::new(MarkerType::Circle, color, name))
        }
        TraceIR::Mesh3D(ir) => {
            let color = ir.color.unwrap_or_else(default_color);
            Some(LegendEntry::new(MarkerType::Triangle, color, "mesh3d"))
        }
        TraceIR::SankeyDiagram(_) => {
            // Sankey diagrams have complex node/link colors; skip individual legend entries
            None
        }
        TraceIR::PieChart(ir) => {
            let name = ir.name.as_deref().unwrap_or("");
            if name.is_empty() { return None; }
            let color = ir.colors.as_ref().and_then(|c| c.first().cloned()).unwrap_or_else(default_color);
            Some(LegendEntry::new(MarkerType::Diamond, color, name))
        }
        TraceIR::CandlestickPlot(ir) => {
            let color = ir.increasing.as_ref().and_then(|d| d.line_color).unwrap_or_else(default_color);
            Some(LegendEntry::new(MarkerType::Line, color, "candlestick"))
        }
        TraceIR::OhlcPlot(_) => {
            Some(LegendEntry::new(MarkerType::Line, default_color(), "ohlc"))
        }
        // Plot types that typically don't have legend entries in subplots
        _ => None,
    }
}
