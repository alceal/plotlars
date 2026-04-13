use std::process::Command;

use plotly::{Plot as Plotly, Trace};
use serde_json::Value;

use plotlars_core::Plot;

/// Intermediate result from IR conversion. Contains the plotly `Plot` plus
/// optional JSON overrides for layout fields that plotly.rs cannot represent
/// (scene domains for 3D plots, polar domains for polar plots).
pub(crate) struct PlotlyResult {
    pub(crate) plot: Plotly,
    pub(crate) layout_overrides: Option<Value>,
}

pub(crate) fn build_plotly_result(data: &impl Plot) -> PlotlyResult {
    let (layout, layout_overrides) = crate::converters::layout::convert_layout_ir(data.ir_layout());
    let traces: Vec<Box<dyn Trace + 'static>> = data
        .ir_traces()
        .iter()
        .map(crate::converters::trace::convert)
        .collect();
    let mut plot = Plotly::new();
    plot.set_layout(layout);
    plot.add_traces(traces);
    PlotlyResult {
        plot,
        layout_overrides,
    }
}

/// Render plot JSON (possibly with overrides) to a standalone HTML document.
pub(crate) fn render_html_from_json(plot_json: &str) -> String {
    format!(
        r#"<html>
<head><meta charset="utf-8" /></head>
<body>
<div id="plotly-div" style="height:100%; width:100%;"></div>
<script src="https://cdn.plot.ly/plotly-3.0.1.min.js"></script>
<script type="text/javascript">
  var plotData = {plot_json};
  Plotly.newPlot("plotly-div", plotData.traces, plotData.layout, {{}});
</script>
</body>
</html>"#,
        plot_json = plot_json
    )
}

/// Render plot JSON to an inline HTML snippet.
pub(crate) fn render_inline_html_from_json(plot_json: &str, div_id: &str) -> String {
    format!(
        r#"<div id="{div_id}" class="plotly-graph-div" style="height:100%; width:100%;"></div>
<script type="text/javascript">
  var plotData = {plot_json};
  Plotly.newPlot("{div_id}", plotData.traces, plotData.layout, {{}});
</script>"#,
        div_id = div_id,
        plot_json = plot_json
    )
}

pub(crate) fn ir_to_json(data: &impl Plot) -> Result<String, serde_json::Error> {
    let (layout, layout_overrides) = crate::converters::layout::convert_layout_ir(data.ir_layout());
    let traces: Vec<Box<dyn Trace + 'static>> = data
        .ir_traces()
        .iter()
        .map(crate::converters::trace::convert)
        .collect();

    let mut layout_json = serde_json::to_value(&layout)?;

    // Merge layout overrides (scene/polar domain entries) into the layout JSON
    if let Some(ref overrides) = layout_overrides {
        if let (Some(layout_map), Some(overrides_obj)) =
            (layout_json.as_object_mut(), overrides.as_object())
        {
            for (key, value) in overrides_obj {
                layout_map.insert(key.clone(), value.clone());
            }
        }
    }

    let traces_json: Vec<Value> = traces
        .iter()
        .map(|t| {
            let serialized = t.to_json();
            serde_json::from_str(&serialized).unwrap_or(Value::Null)
        })
        .collect();

    let output = serde_json::json!({
        "traces": traces_json,
        "layout": layout_json,
    });
    serde_json::to_string(&output)
}

/// Helper function to open an HTML file in the default browser
pub(crate) fn open_html_file(path: &std::path::Path) {
    #[cfg(target_os = "macos")]
    {
        let _ = Command::new("open").arg(path).spawn().map(|mut child| {
            let _ = std::thread::spawn(move || {
                let _ = child.wait();
            });
        });
    }

    #[cfg(target_os = "linux")]
    {
        let _ = Command::new("xdg-open").arg(path).spawn().map(|mut child| {
            let _ = std::thread::spawn(move || {
                let _ = child.wait();
            });
        });
    }

    #[cfg(target_os = "windows")]
    {
        let _ = Command::new("cmd")
            .args(&["/C", "start", "", path.to_str().unwrap()])
            .spawn()
            .map(|mut child| {
                let _ = std::thread::spawn(move || {
                    let _ = child.wait();
                });
            });
    }
}
