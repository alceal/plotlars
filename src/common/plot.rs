use std::env;

use plotly::{Layout, Plot as Plotly, Trace};
use serde_json::Value;

#[cfg(any(
    feature = "static_export_chromedriver",
    feature = "static_export_geckodriver",
    feature = "static_export_default"
))]
use plotly_static::ImageFormat;

use serde::Serialize;

/// A trait representing a generic plot that can be displayed or rendered.
pub trait Plot {
    fn plot(&self);

    fn write_html(&self, path: impl Into<String>);

    fn to_json(&self) -> Result<String, serde_json::Error>;

    fn to_html(&self) -> String;

    fn to_inline_html(&self, plot_div_id: Option<&str>) -> String;

    #[cfg(any(
        feature = "static_export_chromedriver",
        feature = "static_export_geckodriver",
        feature = "static_export_default"
    ))]
    fn write_image(
        &self,
        path: impl Into<String>,
        width: usize,
        height: usize,
        scale: f64,
    ) -> Result<(), std::boxed::Box<(dyn std::error::Error + 'static)>>;
}

/// Helper trait for internal use by the `Plot` trait implementation.
/// Can be used to get the underlying layout and traces of a plot (for example, to create a subplot).
pub trait PlotHelper {
    fn get_layout(&self) -> &Layout;
    fn get_traces(&self) -> &Vec<Box<dyn Trace + 'static>>;

    fn get_layout_override(&self) -> Option<&Value> {
        None
    }

    #[cfg(any(
        feature = "static_export_chromedriver",
        feature = "static_export_geckodriver",
        feature = "static_export_default"
    ))]
    fn get_image_format(
        &self,
        extension: &str,
    ) -> Result<ImageFormat, std::boxed::Box<(dyn std::error::Error + 'static)>> {
        match extension {
            "png" => Ok(ImageFormat::PNG),
            "jpg" => Ok(ImageFormat::JPEG),
            "jpeg" => Ok(ImageFormat::JPEG),
            "webp" => Ok(ImageFormat::WEBP),
            "svg" => Ok(ImageFormat::SVG),
            _ => Err(format!("Unsupported image format: {extension}").into()),
        }
    }
}

// Implement the public trait `Plot` for any type that implements `PlotHelper`.
impl<T> Plot for T
where
    T: PlotHelper + Serialize + Clone,
{
    fn plot(&self) {
        let mut plot = Plotly::new();
        plot.set_layout(self.get_layout().to_owned());
        plot.add_traces(self.get_traces().to_owned());

        let spec_json = build_spec_json(self, &plot);

        match env::var("EVCXR_IS_RUNTIME") {
            Ok(_) => {
                let html = render_jupyter_notebook_html(&spec_json);
                println!("EVCXR_BEGIN_CONTENT text/html\n{html}\nEVCXR_END_CONTENT");
            }
            _ => {
                let mut html = plot.to_html();
                replace_plot_spec(&mut html, &spec_json);
                plot_show(html);
            }
        }
    }

    fn write_html(&self, path: impl Into<String>) {
        let mut plot = Plotly::new();
        plot.set_layout(self.get_layout().to_owned());
        plot.add_traces(self.get_traces().to_owned());

        let spec_json = build_spec_json(self, &plot);
        let mut html = plot.to_html();
        replace_plot_spec(&mut html, &spec_json);

        std::fs::write(path.into(), html).expect("failed to write html output");
    }

    fn to_json(&self) -> Result<String, serde_json::Error> {
        let mut plot = Plotly::new();
        plot.set_layout(self.get_layout().to_owned());
        plot.add_traces(self.get_traces().to_owned());

        let spec_value = build_spec_value(self, &plot);
        serde_json::to_string(&spec_value)
    }

    fn to_html(&self) -> String {
        let mut plot = Plotly::new();
        plot.set_layout(self.get_layout().to_owned());
        plot.add_traces(self.get_traces().to_owned());

        let spec_json = build_spec_json(self, &plot);
        let mut html = plot.to_html();
        replace_plot_spec(&mut html, &spec_json);
        html
    }

    fn to_inline_html(&self, plot_div_id: Option<&str>) -> String {
        let mut plot = Plotly::new();
        plot.set_layout(self.get_layout().to_owned());
        plot.add_traces(self.get_traces().to_owned());

        let spec_json = build_spec_json(self, &plot);
        let mut html = plot.to_inline_html(plot_div_id);
        replace_plot_spec(&mut html, &spec_json);
        html
    }

    #[cfg(any(
        feature = "static_export_chromedriver",
        feature = "static_export_geckodriver",
        feature = "static_export_default"
    ))]
    fn write_image(
        &self,
        path: impl Into<String>,
        width: usize,
        height: usize,
        scale: f64,
    ) -> Result<(), std::boxed::Box<(dyn std::error::Error + 'static)>> {
        let mut plot = Plotly::new();
        plot.set_layout(self.get_layout().to_owned());
        plot.add_traces(self.get_traces().to_owned());

        if let Some((filename, extension)) = path.into().rsplit_once('.') {
            let format = self.get_image_format(extension)?;
            plot.write_image(filename, format, width, height, scale)?;
            Ok(())
        } else {
            Err("No extension provided for image.".into())
        }
    }
}

fn build_spec_value<T>(plot_helper: &T, plot: &Plotly) -> Value
where
    T: PlotHelper,
{
    let mut spec = serde_json::to_value(plot).unwrap();
    if let Some(layout_override) = plot_helper.get_layout_override() {
        spec["layout"] = layout_override.clone();
    }
    spec
}

fn build_spec_json<T>(plot_helper: &T, plot: &Plotly) -> String
where
    T: PlotHelper,
{
    serde_json::to_string(&build_spec_value(plot_helper, plot)).unwrap()
}

fn replace_plot_spec(html: &mut String, spec_json: &str) {
    const PREFIX: &str = "await Plotly.newPlot(graph_div, ";
    if let Some(start) = html.find(PREFIX) {
        let json_start = start + PREFIX.len();
        if let Some(end) = html[json_start..].find(");") {
            html.replace_range(json_start..json_start + end, spec_json);
        }
    }
}

fn plot_show(html: String) {
    use std::fs::File;
    use std::io::Write;
    use std::time::{SystemTime, UNIX_EPOCH};

    let mut temp = std::env::temp_dir();
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    let pid = std::process::id();
    temp.push(format!("plotlars_{pid}_{timestamp}.html"));

    let temp_path = temp.to_str().unwrap().to_string();
    {
        let mut file = File::create(&temp_path).expect("failed to create temp html file");
        file.write_all(html.as_bytes())
            .expect("failed to write html output");
        file.flush().expect("failed to flush html output");
    }

    open_with_default_app(&temp_path);
}

#[cfg(all(not(target_family = "wasm"), not(target_os = "android")))]
fn open_with_default_app(temp_path: &str) {
    #[cfg(all(unix, not(target_os = "macos")))]
    {
        use std::process::Command;
        Command::new("xdg-open")
            .args([temp_path])
            .output()
            .expect("Could not open HTML file with default application.");
    }

    #[cfg(target_os = "macos")]
    {
        use std::process::Command;
        Command::new("open")
            .args([temp_path])
            .output()
            .expect("Could not open HTML file with default application.");
    }

    #[cfg(target_os = "windows")]
    {
        use std::process::Command;
        Command::new("explorer")
            .arg(temp_path)
            .spawn()
            .expect("Could not open HTML file with default application.");
    }
}

#[cfg(any(target_family = "wasm", target_os = "android"))]
fn open_with_default_app(_: &str) {
    // Opening a browser is not supported in these environments.
}

fn render_jupyter_notebook_html(spec_json: &str) -> String {
    use std::time::{SystemTime, UNIX_EPOCH};

    let div_id = format!(
        "plotlars_div_{}_{}",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos()
    );

    format!(
        r#"<div>
    <div id="{div_id}" class="plotly-graph-div" style="height:100%; width:100%;"></div>
    <script type="text/javascript">
        require(['https://cdn.plot.ly/plotly-3.0.1.min.js'], function(Plotly) {{
            Plotly.newPlot(
                "{div_id}",
                {spec_json}
        ).then(function () {{
            var gd = document.getElementById('{div_id}');
            var x = new MutationObserver(function (mutations, observer) {{
                var display = window.getComputedStyle(gd).display;
                if (!display || display === 'none') {{
                    Plotly.purge(gd);
                    observer.disconnect();
                }}
            }});

            var notebookContainer = gd.closest('#notebook-container');
            if (notebookContainer) {{
                x.observe(notebookContainer, {{ childList: true }});
            }}

            var outputEl = gd.closest('.output');
            if (outputEl) {{
                x.observe(outputEl, {{ childList: true }});
            }}
        }});
        }});
    </script>
</div>"#,
        div_id = div_id,
        spec_json = spec_json
    )
}
