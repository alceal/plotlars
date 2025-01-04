use bon::bon;

use plotly::{
    color::Rgb as RgbPlotly, image::ColorModel, Image as ImagePlotly, Layout as LayoutPlotly, Trace,
};

use serde::Serialize;

use crate::{
    common::{Layout, PlotHelper},
    components::{Axis, Rgb, Text},
};

/// A structure representing an image plot.
///
/// The `Image` struct allows for the integration of image data into plots, enabling visualization of raster data
/// or standalone images within a plotting context. It supports customizable titles, axis labels, legend configuration,
/// and layout adjustments for better presentation.
///
/// # Arguments
///
/// * `path` - A string slice specifying the file path of the image to be displayed.
/// * `plot_title` - An optional `Text` struct specifying the title of the plot.
/// * `x_title` - An optional `Text` struct specifying the title of the x-axis.
/// * `y_title` - An optional `Text` struct specifying the title of the y-axis.
/// * `x_axis` - An optional reference to an `Axis` struct for customizing the x-axis.
/// * `y_axis` - An optional reference to an `Axis` struct for customizing the y-axis.
///
/// # Example
///
/// ```rust
/// use plotlars::{Axis, Image, Plot};
///
/// let axis = Axis::new()
///     .show_axis(false);
///
/// Image::builder()
///     .path("data/image.png")
///     .x_axis(&axis)
///     .y_axis(&axis)
///     .plot_title("Image Plot")
///     .build()
///     .plot();
/// ```
///
/// ![Example](https://imgur.com/PAtdaHj.png)
#[derive(Clone, Serialize)]
pub struct Image {
    traces: Vec<Box<dyn Trace + 'static>>,
    layout: LayoutPlotly,
}

#[bon]
impl Image {
    #[builder(on(String, into), on(Text, into))]
    pub fn new(
        path: &str,
        plot_title: Option<Text>,
        x_title: Option<Text>,
        y_title: Option<Text>,
        x_axis: Option<&Axis>,
        y_axis: Option<&Axis>,
    ) -> Self {
        let z_title = None;
        let legend_title = None;
        let z_axis = None;
        let legend = None;

        let layout = Self::create_layout(
            plot_title,
            x_title,
            y_title,
            z_title,
            legend_title,
            x_axis,
            y_axis,
            z_axis,
            legend,
        );

        let mut traces = vec![];

        let trace = Self::create_trace(path);

        traces.push(trace);

        Self { traces, layout }
    }

    fn create_trace(path: &str) -> Box<dyn Trace + 'static> {
        let im: image::ImageBuffer<image::Rgb<u8>, Vec<u8>> =
            image::open(path).unwrap().into_rgb8();

        let (width, height) = im.dimensions();
        let mut pixels = vec![vec![RgbPlotly::new(0, 0, 0); width as usize]; height as usize];

        for (x, y, pixel) in im.enumerate_pixels() {
            let rgb = Rgb(pixel[0], pixel[1], pixel[2]);
            pixels[y as usize][x as usize] = rgb.to_plotly();
        }

        ImagePlotly::new(pixels).color_model(ColorModel::RGB)
    }
}

impl Layout for Image {}

impl PlotHelper for Image {
    fn get_layout(&self) -> &LayoutPlotly {
        &self.layout
    }

    fn get_traces(&self) -> &Vec<Box<dyn Trace + 'static>> {
        &self.traces
    }
}
