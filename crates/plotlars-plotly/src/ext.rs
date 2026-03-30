use std::env;
use std::process::Command;

use plotly::{Plot as Plotly, Trace};
use serde_json::Value;

use plotlars_core::Plot;

/// Plotly rendering extension trait. Provides all visualization methods.
pub trait PlotlyExt: Plot {
    fn plot(&self);
    fn write_html(&self, path: impl Into<String>);
    fn to_json(&self) -> Result<String, serde_json::Error>;
    fn to_html(&self) -> String;
    fn to_inline_html(&self, plot_div_id: Option<&str>) -> String;

    #[cfg(any(
        feature = "export-chrome",
        feature = "export-firefox",
        feature = "export-default"
    ))]
    fn write_image(
        &self,
        path: impl Into<String>,
        width: usize,
        height: usize,
        scale: f64,
    ) -> Result<(), Box<dyn std::error::Error + 'static>>;
}

/// Intermediate result from IR conversion. Contains the plotly `Plot` plus
/// optional JSON overrides for layout fields that plotly.rs cannot represent
/// (scene domains for 3D plots, polar domains for polar plots).
struct PlotlyResult {
    plot: Plotly,
    layout_overrides: Option<Value>,
}

fn build_plotly_result(data: &impl Plot) -> PlotlyResult {
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
fn render_html_from_json(plot_json: &str) -> String {
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
fn render_inline_html_from_json(plot_json: &str, div_id: &str) -> String {
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

fn ir_to_json(data: &impl Plot) -> Result<String, serde_json::Error> {
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

impl<T: Plot> PlotlyExt for T {
    fn plot(&self) {
        let result = build_plotly_result(self);
        if result.layout_overrides.is_some() {
            // For plots with layout overrides (scene/polar domains),
            // we must serialize traces via Trace::to_json() to capture
            // injected keys like "scene" on Surface traces.
            let json = ir_to_json(self).unwrap_or_default();
            let html = render_html_from_json(&json);
            let temp_dir = std::env::temp_dir();
            let timestamp = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos();
            let temp_path = temp_dir.join(format!(
                "plotlars_{}_{}.html",
                std::process::id(),
                timestamp
            ));
            std::fs::write(&temp_path, html).expect("failed to write temp html");
            open_html_file(&temp_path);
        } else {
            match env::var("EVCXR_IS_RUNTIME") {
                Ok(_) => result.plot.evcxr_display(),
                _ => result.plot.show(),
            }
        }
    }

    fn write_html(&self, path: impl Into<String>) {
        let result = build_plotly_result(self);
        if result.layout_overrides.is_some() {
            let json = ir_to_json(self).unwrap_or_default();
            let html = render_html_from_json(&json);
            std::fs::write(path.into(), html).expect("failed to write html output");
        } else {
            result.plot.write_html(path.into());
        }
    }

    fn to_json(&self) -> Result<String, serde_json::Error> {
        ir_to_json(self)
    }

    fn to_html(&self) -> String {
        let result = build_plotly_result(self);
        if result.layout_overrides.is_some() {
            let json = ir_to_json(self).unwrap_or_default();
            render_html_from_json(&json)
        } else {
            result.plot.to_html()
        }
    }

    fn to_inline_html(&self, plot_div_id: Option<&str>) -> String {
        let result = build_plotly_result(self);
        let div_id = plot_div_id.unwrap_or("plotly-div");
        if result.layout_overrides.is_some() {
            let json = ir_to_json(self).unwrap_or_default();
            render_inline_html_from_json(&json, div_id)
        } else {
            result.plot.to_inline_html(plot_div_id)
        }
    }

    #[cfg(any(
        feature = "export-chrome",
        feature = "export-firefox",
        feature = "export-default"
    ))]
    fn write_image(
        &self,
        path: impl Into<String>,
        width: usize,
        height: usize,
        scale: f64,
    ) -> Result<(), Box<dyn std::error::Error + 'static>> {
        let path_string = path.into();
        let result = build_plotly_result(self);

        // Image export uses plotly.rs directly; layout overrides are not
        // applicable here because the plotly.js static exporter only reads
        // the standard Layout fields. For scene/polar faceted plots the
        // JSON override path should be used for HTML output only.
        if let Some((filename, extension)) = path_string.rsplit_once('.') {
            let format = match extension {
                "png" => plotly::ImageFormat::PNG,
                "jpg" | "jpeg" => plotly::ImageFormat::JPEG,
                "webp" => plotly::ImageFormat::WEBP,
                "svg" => plotly::ImageFormat::SVG,
                _ => return Err(format!("Unsupported image format: {extension}").into()),
            };
            result
                .plot
                .write_image(filename, format, width, height, scale)?;
        } else {
            return Err("No extension provided for image.".into());
        }

        Ok(())
    }
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
