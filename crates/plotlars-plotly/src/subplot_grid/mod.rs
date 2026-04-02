use bon::bon;
use plotly::{layout::Layout as LayoutPlotly, Trace};
use serde::ser::{Serialize, SerializeStruct, Serializer};
use serde_json::Value;

use plotlars_core::components::{Dimensions, Text};
use plotlars_core::Plot;

mod custom_legend;
mod irregular;
mod regular;
mod shared;

/// A structure representing a subplot grid layout.
///
/// The `SubplotGrid` struct facilitates the creation of multi-plot layouts arranged in a grid configuration.
/// Plots are automatically arranged in rows and columns in row-major order (left-to-right, top-to-bottom).
/// Each subplot retains its own title, axis labels, and legend, providing flexibility for displaying
/// multiple related visualizations in a single figure.
///
/// # Features
///
/// - Automatic grid layout with configurable rows and columns
/// - Individual subplot titles (extracted from plot titles)
/// - Independent axis labels for each subplot
/// - Configurable horizontal and vertical spacing
/// - Overall figure title
/// - Sparse grid support (fewer plots than grid capacity)
///
#[derive(Clone)]
pub struct SubplotGrid {
    traces: Vec<Box<dyn Trace + 'static>>,
    layout: LayoutPlotly,
    layout_json: Option<Value>,
}

impl Serialize for SubplotGrid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("SubplotGrid", 2)?;
        state.serialize_field("traces", &self.traces)?;

        if let Some(ref layout_json) = self.layout_json {
            state.serialize_field("layout", layout_json)?;
        } else {
            state.serialize_field("layout", &self.layout)?;
        }

        state.end()
    }
}

#[bon]
impl SubplotGrid {
    /// Creates a subplot grid layout.
    ///
    /// Arranges plots in a row * column grid with automatic positioning. Plots are placed
    /// in row-major order (left-to-right, top-to-bottom). Each subplot retains its individual title
    /// (from the plot's `plot_title`), axis labels, and legend.
    ///
    /// # Arguments
    ///
    /// * `plots` - Vector of plot references to arrange in the grid. Plots are positioned in row-major order.
    /// * `rows` - Number of rows in the grid (default: 1).
    /// * `cols` - Number of columns in the grid (default: 1).
    /// * `title` - Overall title for the entire subplot figure (optional).
    /// * `h_gap` - Horizontal spacing between columns (range: 0.0 to 1.0, default: 0.1).
    /// * `v_gap` - Vertical spacing between rows (range: 0.0 to 1.0, default: 0.1).
    ///
    /// ![Example](https://imgur.com/q0K7cyP.png)
    #[builder(on(String, into), on(Text, into), finish_fn = build)]
    pub fn regular(
        plots: Vec<&dyn Plot>,
        rows: Option<usize>,
        cols: Option<usize>,
        title: Option<Text>,
        h_gap: Option<f64>,
        v_gap: Option<f64>,
        dimensions: Option<&Dimensions>,
    ) -> Self {
        regular::build_regular(plots, rows, cols, title, h_gap, v_gap, None, dimensions)
    }

    /// Creates an irregular grid subplot layout with custom row/column spanning.
    ///
    /// Allows plots to span multiple rows and/or columns, enabling dashboard-style
    /// layouts and asymmetric grid arrangements. Each plot explicitly specifies its
    /// position and span.
    ///
    /// # Arguments
    ///
    /// * `plots` - Vector of tuples `(plot, row, col, rowspan, colspan)` where:
    ///   - `plot`: Reference to the plot
    ///   - `row`: Starting row (0-indexed)
    ///   - `col`: Starting column (0-indexed)
    ///   - `rowspan`: Number of rows to span (minimum 1)
    ///   - `colspan`: Number of columns to span (minimum 1)
    /// * `rows` - Total number of rows in the grid (default: 1).
    /// * `cols` - Total number of columns in the grid (default: 1).
    /// * `title` - Overall title for the subplot figure (optional).
    /// * `h_gap` - Horizontal spacing between columns (range: 0.0 to 1.0, default: 0.1).
    /// * `v_gap` - Vertical spacing between rows (range: 0.0 to 1.0, default: 0.1).
    ///
    /// ![Example](https://imgur.com/RvZwv3O.png)
    #[builder(on(String, into), on(Text, into), finish_fn = build)]
    pub fn irregular(
        plots: Vec<(&dyn Plot, usize, usize, usize, usize)>,
        rows: Option<usize>,
        cols: Option<usize>,
        title: Option<Text>,
        h_gap: Option<f64>,
        v_gap: Option<f64>,
        dimensions: Option<&Dimensions>,
    ) -> Self {
        irregular::build_irregular(plots, rows, cols, title, h_gap, v_gap, dimensions)
    }
}

// Manual PlotlyExt implementation for SubplotGrid.
// SubplotGrid is a composite (not an IR-based plot), so it cannot implement core::Plot.
// Instead it provides its own rendering methods.
impl SubplotGrid {
    /// Display the subplot grid in the default browser or Jupyter notebook.
    pub fn plot(&self) {
        use std::env;

        match env::var("EVCXR_IS_RUNTIME") {
            Ok(_) => {
                let mut plotly_plot = plotly::Plot::new();
                plotly_plot.set_layout(self.layout.clone());
                for trace in self.traces.clone() {
                    plotly_plot.add_trace(trace);
                }
                plotly_plot.evcxr_display();
            }
            _ => {
                let html = self.to_html();
                let dir = std::env::temp_dir();
                let path = dir.join("plotlars_subplot_grid.html");
                std::fs::write(&path, &html).expect("Failed to write HTML file");
                crate::ext::open_html_file(&path);
            }
        }
    }

    /// Write the subplot grid to an HTML file.
    pub fn write_html(&self, path: impl Into<String>) {
        let html = self.to_html();
        std::fs::write(path.into(), html).expect("Failed to write HTML file");
    }

    /// Serialize the subplot grid to a JSON string.
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        let layout_val = self
            .layout_json
            .as_ref()
            .cloned()
            .unwrap_or_else(|| serde_json::to_value(&self.layout).unwrap_or(Value::Null));
        let traces_json: Vec<Value> = self
            .traces
            .iter()
            .map(|t| {
                let s = t.to_json();
                serde_json::from_str(&s).unwrap_or(Value::Null)
            })
            .collect();
        let output = serde_json::json!({
            "traces": traces_json,
            "layout": layout_val,
        });
        serde_json::to_string(&output)
    }

    /// Render the subplot grid as a standalone HTML string.
    pub fn to_html(&self) -> String {
        let plot_json = self.to_json().unwrap();
        let escaped_json = plot_json
            .replace('\\', "\\\\")
            .replace('\'', "\\'")
            .replace('\n', "\\n")
            .replace('\r', "\\r");
        format!(
            r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8" />
    <script src="https://cdn.plot.ly/plotly-3.0.1.min.js"></script>
</head>
<body>
    <div id="plotly-div" style="width:100%;height:100%;"></div>
    <script type="text/javascript">
        var plotData = JSON.parse('{}');
        Plotly.newPlot('plotly-div', plotData.traces, plotData.layout, {{responsive: true}});
    </script>
</body>
</html>"#,
            escaped_json
        )
    }

    /// Render the subplot grid as an inline HTML snippet (no DOCTYPE/head).
    pub fn to_inline_html(&self, plot_div_id: Option<&str>) -> String {
        let div_id = plot_div_id.unwrap_or("plotly-div");
        let plot_json = self.to_json().unwrap();
        format!(
            r#"<div>
<script src="https://cdn.plot.ly/plotly-3.0.1.min.js"></script>
<div id="{div_id}" class="plotly-graph-div" style="height:100%; width:100%;"></div>
<script type="text/javascript">
  var plotData = {plot_json};
  Plotly.newPlot("{div_id}", plotData.traces, plotData.layout, {{responsive: true}});
</script>
</div>"#,
            div_id = div_id,
            plot_json = plot_json
        )
    }
}
