use std::env;

use plotly::{Layout, Plot as Plotly, Trace};
use serde::Serialize;

/// A trait representing a generic plot that can be displayed or rendered.
pub trait Plot {
    fn plot(&self);

    fn write_html(&self, path: impl Into<String>);

    fn to_json(&self) -> Result<String, serde_json::Error>;

    // fn write_image(
    //     &self,
    //     path: impl Into<String>,
    //     width: usize,
    //     height: usize,
    //     scale: f64,
    // );
}

// Private helper trait containing methods not exposed publicly.
pub(crate) trait PlotHelper {
    fn get_layout(&self) -> &Layout;
    fn get_traces(&self) -> &Vec<Box<dyn Trace + 'static>>;

    // fn get_image_format(&self, extension: &str) -> ImageFormat {
    //     match extension {
    //         "png" => ImageFormat::PNG,
    //         _ => panic!("no image extension provided")
    //     }
    // }
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

    // fn write_image(
    //     &self,
    //     path: impl Into<String>,
    //     width: usize,
    //     height: usize,
    //     scale: f64,
    // ) {
    //     let mut plot = Plotly::new();
    //     plot.set_layout(self.get_layout().to_owned());
    //     plot.add_traces(self.get_traces().to_owned());

    //     if let Some((filename, extension)) = path.into().rsplit_once('.') {
    //         let format = self.get_image_format(extension);
    //         plot.write_image(filename, format, width, height, scale);
    //     }
    // }
}
