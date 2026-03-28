#![allow(dead_code)]

use plotly::{color::Rgb as RgbPlotly, image::ColorModel, Image as ImagePlotly, Trace};

use crate::ir::trace::{Array2dPlotIR, ImageIR, TraceIR};

pub(crate) fn convert(trace: &TraceIR) -> Box<dyn Trace + 'static> {
    match trace {
        TraceIR::Image(ir) => convert_image(ir),
        TraceIR::Array2dPlot(ir) => convert_array2d(ir),
        _ => unimplemented!("TraceIR variant not yet implemented for plotly conversion"),
    }
}

fn convert_image(ir: &ImageIR) -> Box<dyn Trace + 'static> {
    let pixels: Vec<Vec<RgbPlotly>> = ir
        .pixels
        .iter()
        .map(|row| {
            row.iter()
                .map(|p| RgbPlotly::new(p[0], p[1], p[2]))
                .collect()
        })
        .collect();

    ImagePlotly::new(pixels).color_model(ColorModel::RGB)
}

fn convert_array2d(ir: &Array2dPlotIR) -> Box<dyn Trace + 'static> {
    let pixels: Vec<Vec<RgbPlotly>> = ir
        .data
        .iter()
        .map(|row| {
            row.iter()
                .map(|rgb| RgbPlotly::new(rgb[0], rgb[1], rgb[2]))
                .collect()
        })
        .collect();

    ImagePlotly::new(pixels).color_model(ColorModel::RGB)
}
