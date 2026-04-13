use bon::bon;

use crate::{
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
#[derive(Clone)]
#[allow(dead_code)]
pub struct Image {
    traces: Vec<TraceIR>,
    layout: LayoutIR,
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
        // Build IR
        let ir_trace = Self::create_ir_trace(path);
        let traces = vec![ir_trace];
        let layout = LayoutIR {
            title: plot_title.clone(),
            x_title: x_title.clone(),
            y_title: y_title.clone(),
            y2_title: None,
            z_title: None,
            legend_title: None,
            legend: None,
            dimensions: None,
            bar_mode: None,
            box_mode: None,
            box_gap: None,
            margin_bottom: None,
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
        Self { traces, layout }
    }
}

#[bon]
impl Image {
    #[builder(
        start_fn = try_builder,
        finish_fn = try_build,
        builder_type = ImageTryBuilder,
        on(String, into),
        on(Text, into),
    )]
    pub fn try_new(
        path: &str,
        plot_title: Option<Text>,
        x_title: Option<Text>,
        y_title: Option<Text>,
        x_axis: Option<&Axis>,
        y_axis: Option<&Axis>,
    ) -> Result<Self, crate::io::PlotlarsError> {
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            Self::__orig_new(path, plot_title, x_title, y_title, x_axis, y_axis)
        }))
        .map_err(|panic| {
            let msg = panic
                .downcast_ref::<String>()
                .cloned()
                .or_else(|| panic.downcast_ref::<&str>().map(|s| s.to_string()))
                .unwrap_or_else(|| "unknown error".to_string());
            crate::io::PlotlarsError::PlotBuild { message: msg }
        })
    }
}

impl Image {
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

impl crate::Plot for Image {
    fn ir_traces(&self) -> &[TraceIR] {
        &self.traces
    }

    fn ir_layout(&self) -> &LayoutIR {
        &self.layout
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Plot;

    fn image_path() -> String {
        let manifest = env!("CARGO_MANIFEST_DIR");
        format!("{}/../../data/image.png", manifest)
    }

    #[test]
    fn test_basic_one_trace() {
        let plot = Image::builder().path(&image_path()).build();
        assert_eq!(plot.ir_traces().len(), 1);
    }

    #[test]
    fn test_trace_variant() {
        let plot = Image::builder().path(&image_path()).build();
        assert!(matches!(plot.ir_traces()[0], TraceIR::Image(_)));
    }
}
