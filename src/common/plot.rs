use std::env;

use plotly::{Layout, Plot as Plotly, Trace};

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

// Private helper trait containing methods not exposed publicly.
pub(crate) trait PlotHelper {
    fn get_layout(&self) -> &Layout;
    fn get_traces(&self) -> &Vec<Box<dyn Trace + 'static>>;

    #[cfg(any(
        feature = "static_export_chromedriver",
        feature = "static_export_geckodriver",
        feature = "static_export_default"
    ))]
    fn get_image_format(&self, extension: &str) -> ImageFormat {
        match extension {
            "png" => ImageFormat::PNG,
            "jpg" => ImageFormat::JPEG,
            "jpeg" => ImageFormat::JPEG,
            "webp" => ImageFormat::WEBP,
            "svg" => ImageFormat::SVG,
            _ => panic!("no image extension provided"),
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

        match env::var("EVCXR_IS_RUNTIME") {
            Ok(_) => plot.notebook_display(),
            _ => plot.show(),
        }
    }

    fn write_html(&self, path: impl Into<String>) {
        let mut plot = Plotly::new();
        plot.set_layout(self.get_layout().to_owned());
        plot.add_traces(self.get_traces().to_owned());
        plot.write_html(path.into());
    }

    fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    fn to_html(&self) -> String {
        let mut plot = Plotly::new();
        plot.set_layout(self.get_layout().to_owned());
        plot.add_traces(self.get_traces().to_owned());
        plot.to_html()
    }

    fn to_inline_html(&self, plot_div_id: Option<&str>) -> String {
        let mut plot = Plotly::new();
        plot.set_layout(self.get_layout().to_owned());
        plot.add_traces(self.get_traces().to_owned());
        plot.to_inline_html(plot_div_id)
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
            let format = self.get_image_format(extension);
            plot.write_image(filename, format, width, height, scale)?;
        }

        Ok(())
    }
}
