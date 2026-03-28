use bon::bon;

use plotly::{Layout as LayoutPlotly, Trace};

use serde::Serialize;

use crate::{
    common::{Layout, PlotHelper},
    components::{Axis, Text},
    ir::layout::LayoutIR,
    ir::trace::{ImageIR, TraceIR},
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
#[allow(dead_code)]
pub struct Image {
    #[serde(skip)]
    ir_traces: Vec<TraceIR>,
    #[serde(skip)]
    ir_layout: LayoutIR,
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
        let y2_title = None;
        let y2_axis = None;

        // Build IR
        let ir_trace = Self::create_ir_trace(path);
        let ir_traces = vec![ir_trace];
        let ir_layout = LayoutIR {
            title: plot_title.clone(),
            x_title: x_title.clone(),
            y_title: y_title.clone(),
            y2_title: None,
            z_title: None,
            legend_title: None,
            legend: None,
            dimensions: None,
            bar_mode: None,
            axes_2d: Some(crate::ir::layout::Axes2dIR {
                x_axis: x_axis.cloned(),
                y_axis: y_axis.cloned(),
                y2_axis: None,
            }),
            scene_3d: None,
            polar: None,
            mapbox: None,
            grid: None,
            annotations: vec![],
        };

        // Build plotly types from IR for backward compatibility
        let plotly_traces: Vec<Box<dyn Trace + 'static>> = ir_traces
            .iter()
            .map(crate::plotly_conversions::trace::convert)
            .collect();

        let layout = Self::create_layout(
            plot_title, x_title, y_title, y2_title, z_title, legend_title, x_axis, y_axis,
            y2_axis, z_axis, legend, None,
        );

        Self {
            ir_traces,
            ir_layout,
            traces: plotly_traces,
            layout,
        }
    }

    fn create_ir_trace(path: &str) -> TraceIR {
        let im: image::ImageBuffer<image::Rgb<u8>, Vec<u8>> =
            image::open(path).unwrap().into_rgb8();

        let (width, height) = im.dimensions();
        let mut pixels = vec![vec![[0u8; 3]; width as usize]; height as usize];

        for (x, y, pixel) in im.enumerate_pixels() {
            pixels[y as usize][x as usize] = [pixel[0], pixel[1], pixel[2]];
        }

        TraceIR::Image(ImageIR { pixels })
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
