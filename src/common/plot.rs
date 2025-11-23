use std::env;
use std::fs;
use std::process::Command;

use plotly::{Layout, Plot as Plotly, Trace};
use serde_json::Value;

use crate::components::{Rgb, Text};

use serde::Serialize;

/// A trait representing a generic plot that can be displayed or rendered.
pub trait Plot {
    fn plot(&self);

    fn write_html(&self, path: impl Into<String>);

    fn to_json(&self) -> Result<String, serde_json::Error>;

    fn to_html(&self) -> String;

    fn to_inline_html(&self, plot_div_id: Option<&str>) -> String; // We need it?

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
    ) -> Result<(), std::boxed::Box<dyn std::error::Error + 'static>>;
}

/// Helper trait for internal use by the `Plot` trait implementation.
/// Can be used to get the underlying layout and traces of a plot (for example, to create a subplot).
pub trait PlotHelper {
    #[doc(hidden)]
    fn get_layout(&self) -> &Layout;
    #[doc(hidden)]
    fn get_traces(&self) -> &Vec<Box<dyn Trace + 'static>>;

    #[doc(hidden)]
    fn get_layout_override(&self) -> Option<&Value> {
        None
    }

    #[doc(hidden)]
    fn get_serialized_traces(&self) -> Option<Vec<Value>> {
        None
    }

    #[doc(hidden)]
    fn get_main_title(&self) -> Option<String> {
        let layout_json = serde_json::to_value(self.get_layout()).ok()?;
        layout_json
            .get("title")
            .and_then(|t| t.get("text"))
            .and_then(|t| t.as_str())
            .map(|s| s.to_string())
    }

    #[doc(hidden)]
    fn get_x_title(&self) -> Option<String> {
        let layout_json = serde_json::to_value(self.get_layout()).ok()?;
        layout_json
            .get("xaxis")
            .and_then(|axis| axis.get("title"))
            .and_then(|title| title.get("text"))
            .and_then(|text| text.as_str())
            .map(|s| s.to_string())
    }

    #[doc(hidden)]
    fn get_y_title(&self) -> Option<String> {
        let layout_json = serde_json::to_value(self.get_layout()).ok()?;
        layout_json
            .get("yaxis")
            .and_then(|axis| axis.get("title"))
            .and_then(|title| title.get("text"))
            .and_then(|text| text.as_str())
            .map(|s| s.to_string())
    }

    #[doc(hidden)]
    fn get_main_title_text(&self) -> Option<Text> {
        let layout_json = serde_json::to_value(self.get_layout()).ok()?;
        let title_obj = layout_json.get("title")?;

        let content = title_obj
            .get("text")
            .and_then(|t| t.as_str())
            .map(|s| s.to_string())?;

        let mut text = Text::from(content);

        if let Some(font_obj) = title_obj.get("font") {
            if let Some(family) = font_obj.get("family").and_then(|f| f.as_str()) {
                if !family.is_empty() {
                    text = text.font(family);
                }
            }

            if let Some(size) = font_obj.get("size").and_then(|s| s.as_u64()) {
                if size > 0 {
                    text = text.size(size as usize);
                }
            }

            if let Some(color) = font_obj.get("color").and_then(|c| c.as_str()) {
                if let Some(rgb) = parse_color(color) {
                    text = text.color(rgb);
                }
            }
        }

        if let Some(x) = title_obj.get("x").and_then(|v| v.as_f64()) {
            text = text.x(x);
        }

        if let Some(y) = title_obj.get("y").and_then(|v| v.as_f64()) {
            text = text.y(y);
        }

        Some(text)
    }

    #[doc(hidden)]
    fn get_x_title_text(&self) -> Option<Text> {
        let layout_json = serde_json::to_value(self.get_layout()).ok()?;
        let title_obj = layout_json
            .get("xaxis")
            .and_then(|axis| axis.get("title"))?;

        let content = title_obj
            .get("text")
            .and_then(|t| t.as_str())
            .map(|s| s.to_string())?;

        let mut text = Text::from(content);

        if let Some(font_obj) = title_obj.get("font") {
            if let Some(family) = font_obj.get("family").and_then(|f| f.as_str()) {
                if !family.is_empty() {
                    text = text.font(family);
                }
            }

            if let Some(size) = font_obj.get("size").and_then(|s| s.as_u64()) {
                if size > 0 {
                    text = text.size(size as usize);
                }
            }

            if let Some(color) = font_obj.get("color").and_then(|c| c.as_str()) {
                if let Some(rgb) = parse_color(color) {
                    text = text.color(rgb);
                }
            }
        }

        if let Some(x) = title_obj.get("x").and_then(|v| v.as_f64()) {
            text = text.x(x);
        }

        if let Some(y) = title_obj.get("y").and_then(|v| v.as_f64()) {
            text = text.y(y);
        }

        Some(text)
    }

    #[doc(hidden)]
    fn get_y_title_text(&self) -> Option<Text> {
        let layout_json = serde_json::to_value(self.get_layout()).ok()?;
        let title_obj = layout_json
            .get("yaxis")
            .and_then(|axis| axis.get("title"))?;

        let content = title_obj
            .get("text")
            .and_then(|t| t.as_str())
            .map(|s| s.to_string())?;

        let mut text = Text::from(content);

        if let Some(font_obj) = title_obj.get("font") {
            if let Some(family) = font_obj.get("family").and_then(|f| f.as_str()) {
                if !family.is_empty() {
                    text = text.font(family);
                }
            }

            if let Some(size) = font_obj.get("size").and_then(|s| s.as_u64()) {
                if size > 0 {
                    text = text.size(size as usize);
                }
            }

            if let Some(color) = font_obj.get("color").and_then(|c| c.as_str()) {
                if let Some(rgb) = parse_color(color) {
                    text = text.color(rgb);
                }
            }
        }

        if let Some(x) = title_obj.get("x").and_then(|v| v.as_f64()) {
            text = text.x(x);
        }

        if let Some(y) = title_obj.get("y").and_then(|v| v.as_f64()) {
            text = text.y(y);
        }

        Some(text)
    }

    #[doc(hidden)]
    #[cfg(any(
        feature = "export-chrome",
        feature = "export-firefox",
        feature = "export-default"
    ))]
    fn get_image_format(
        &self,
        extension: &str,
    ) -> Result<plotly::ImageFormat, std::boxed::Box<dyn std::error::Error + 'static>> {
        match extension {
            "png" => Ok(plotly::ImageFormat::PNG),
            "jpg" => Ok(plotly::ImageFormat::JPEG),
            "jpeg" => Ok(plotly::ImageFormat::JPEG),
            "webp" => Ok(plotly::ImageFormat::WEBP),
            "svg" => Ok(plotly::ImageFormat::SVG),
            _ => Err(format!("Unsupported image format: {extension}").into()),
        }
    }
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

// Implement the public trait `Plot` for any type that implements `PlotHelper`.
impl<T> Plot for T
where
    T: PlotHelper + Serialize + Clone,
{
    fn plot(&self) {
        if self.get_layout_override().is_some() {
            let html = self.to_html();

            match env::var("EVCXR_IS_RUNTIME") {
                Ok(_) => {
                    // For Jupyter/evcxr, print the HTML directly
                    println!("HTML");
                    println!("{}", html);
                }
                _ => {
                    // Write HTML to temp file and open in browser
                    let temp_dir = env::temp_dir();
                    let timestamp = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_nanos();
                    let temp_file = temp_dir.join(format!(
                        "plotlars_{}_{}.html",
                        std::process::id(),
                        timestamp
                    ));
                    fs::write(&temp_file, html).expect("Failed to write HTML file");

                    // Open the file in default browser
                    open_html_file(&temp_file);
                }
            }
        } else {
            let mut plot = Plotly::new();
            plot.set_layout(self.get_layout().to_owned());
            plot.add_traces(self.get_traces().to_owned());

            match env::var("EVCXR_IS_RUNTIME") {
                Ok(_) => plot.evcxr_display(),
                _ => plot.show(),
            }
        }
    }

    fn write_html(&self, path: impl Into<String>) {
        if self.get_layout_override().is_some() {
            let html = self.to_html();
            fs::write(path.into(), html).expect("Failed to write HTML file");
        } else {
            let mut plot = Plotly::new();
            plot.set_layout(self.get_layout().to_owned());
            plot.add_traces(self.get_traces().to_owned());
            plot.write_html(path.into());
        }
    }

    fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    fn to_html(&self) -> String {
        if self.get_layout_override().is_some() {
            let plot_json = serde_json::to_string(self).unwrap();
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
    <script src="https://cdn.plot.ly/plotly-2.18.0.min.js"></script>
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
        } else {
            let mut plot = Plotly::new();
            plot.set_layout(self.get_layout().to_owned());
            plot.add_traces(self.get_traces().to_owned());
            plot.to_html()
        }
    }

    fn to_inline_html(&self, plot_div_id: Option<&str>) -> String {
        let div_id = plot_div_id.unwrap_or("plotly-div");

        if self.get_layout_override().is_some() {
            let plot_json = serde_json::to_string(self).unwrap();
            let escaped_json = plot_json
                .replace('\\', "\\\\")
                .replace('\'', "\\'")
                .replace('\n', "\\n")
                .replace('\r', "\\r");

            format!(
                r#"<div id="{}" style="width:100%;height:100%;"></div>
<script type="text/javascript">
    var plotData = JSON.parse('{}');
    Plotly.newPlot('{}', plotData.traces, plotData.layout, {{responsive: true}});
</script>"#,
                div_id, escaped_json, div_id
            )
        } else {
            let mut plot = Plotly::new();
            plot.set_layout(self.get_layout().to_owned());
            plot.add_traces(self.get_traces().to_owned());
            plot.to_inline_html(plot_div_id)
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
    ) -> Result<(), std::boxed::Box<dyn std::error::Error + 'static>> {
        let path_string = path.into();

        let mut plot = Plotly::new();
        plot.set_layout(self.get_layout().to_owned());
        plot.add_traces(self.get_traces().to_owned());

        if let Some((filename, extension)) = path_string.rsplit_once('.') {
            let format = self.get_image_format(extension)?;
            plot.write_image(filename, format, width, height, scale)?;
        } else {
            return Err("No extension provided for image.".into());
        }

        Ok(())
    }
}

/// Helper function to open an HTML file in the default browser
fn open_html_file(path: &std::path::Path) {
    #[cfg(target_os = "macos")]
    {
        let _ = Command::new("open").arg(path).spawn().map(|mut child| {
            // Spawn browser process and detach - we don't want to wait for it
            let _ = std::thread::spawn(move || {
                let _ = child.wait();
            });
        });
    }

    #[cfg(target_os = "linux")]
    {
        let _ = Command::new("xdg-open").arg(path).spawn().map(|mut child| {
            // Spawn browser process and detach - we don't want to wait for it
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
                // Spawn browser process and detach - we don't want to wait for it
                let _ = std::thread::spawn(move || {
                    let _ = child.wait();
                });
            });
    }

    #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
    {
        eprintln!("Cannot automatically open browser on this platform. Please open the file manually: {:?}", path);
    }
}
