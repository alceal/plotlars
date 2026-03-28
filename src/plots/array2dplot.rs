use bon::bon;

use plotly::{Layout as LayoutPlotly, Trace};

use serde::Serialize;

use crate::{
    common::{Layout, PlotHelper},
    components::{Axis, Text},
    ir::layout::LayoutIR,
    ir::trace::{Array2dPlotIR, TraceIR},
};

/// A structure representing a 2D array plot.
///
/// The `Array2dPlot` struct allows for visualizing 2D arrays of RGB color values as images or heatmaps.
/// Each element in the 2D array corresponds to a pixel, with its color defined by an `[u8; 3]` RGB triplet.
/// This struct supports customizable titles, axis labels, and axis configurations for better presentation.
///
/// # Arguments
///
/// * `data` - A 2D vector of RGB triplets (`&[Vec<[u8; 3]>]`) representing pixel colors for the plot.
/// * `plot_title` - An optional `Text` struct specifying the title of the plot.
/// * `x_title` - An optional `Text` struct specifying the title of the x-axis.
/// * `y_title` - An optional `Text` struct specifying the title of the y-axis.
/// * `x_axis` - An optional reference to an `Axis` struct for customizing the x-axis.
/// * `y_axis` - An optional reference to an `Axis` struct for customizing the y-axis.
///
/// # Example
///
/// ## Basic 2D Array Plot
///
/// ```rust
/// use plotlars::{Array2dPlot, Plot, Text};
///
/// let data = vec![
///     vec![[255, 0, 0], [0, 255, 0], [0, 0, 255]],
///     vec![[0, 0, 255], [255, 0, 0], [0, 255, 0]],
///     vec![[0, 255, 0], [0, 0, 255], [255, 0, 0]],
/// ];
///
/// Array2dPlot::builder()
///     .data(&data)
///     .plot_title(
///         Text::from("Array2D Plot")
///             .font("Arial")
///             .size(18)
///     )
///     .build()
///     .plot();
/// ```
///
/// ![Example](https://imgur.com/LMrqAaT.png)
#[allow(dead_code)]
#[derive(Clone, Serialize)]
pub struct Array2dPlot {
    #[serde(skip)]
    ir_traces: Vec<TraceIR>,
    #[serde(skip)]
    ir_layout: LayoutIR,
    traces: Vec<Box<dyn Trace + 'static>>,
    layout: LayoutPlotly,
}

#[bon]
impl Array2dPlot {
    #[builder(on(String, into), on(Text, into))]
    pub fn new(
        data: &[Vec<[u8; 3]>],
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
        let ir_trace = TraceIR::Array2dPlot(Array2dPlotIR {
            data: data.to_vec(),
        });
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

        // Build plotly types from IR
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
}

impl Layout for Array2dPlot {}

impl PlotHelper for Array2dPlot {
    fn get_layout(&self) -> &LayoutPlotly {
        &self.layout
    }

    fn get_traces(&self) -> &Vec<Box<dyn Trace + 'static>> {
        &self.traces
    }
}
