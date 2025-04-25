use bon::bon;

use plotly::{
    // Plot struct here,
    Layout as LayoutPlotly,
    Trace,
};

use polars::frame::DataFrame;
use serde::Serialize;

use crate::{
    common::{Layout, PlotHelper, Polar},
    components::{Axis, Text},
};

#[derive(Clone, Serialize)]
pub struct TemplatePlot {
    traces: Vec<Box<dyn Trace + 'static>>,
    layout: LayoutPlotly,
}

#[bon]
impl TemplatePlot {
    #[builder(on(String, into), on(Text, into))]
    pub fn new(
        data: &DataFrame,
        x: &str,
        y: &str,
        plot_title: Option<Text>,
        x_title: Option<Text>,
        y_title: Option<Text>,
        x_axis: Option<&Axis>,
        y_axis: Option<&Axis>,
    ) -> Self {
        let legend = None;
        let legend_title = None;
        let z_title = None;
        let z_axis = None;

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

        let traces = Self::create_traces(data, x, y);

        Self { traces, layout }
    }

    #[allow(clippy::too_many_arguments)]
    fn create_traces(
        data: &DataFrame,
        x: &str,
        y: &str,
    ) -> Vec<Box<dyn Trace + 'static>> {
        let mut traces: Vec<Box<dyn Trace + 'static>> = Vec::new();

        let trace = Self::create_trace(data, x, y);

        traces.push(trace);
        traces
    }

    #[allow(clippy::too_many_arguments)]
    fn create_trace(
        data: &DataFrame,
        x: &str,
        y: &str,
    ) -> Box<dyn Trace + 'static> {
        let x = Self::get_string_column(data, x);
        let y = Self::get_numeric_column(data, y);

        let mut trace = Plot here::new(x, y);

        trace = Self::set_something(trace, something);

        trace
    }

    fn set_something<X, Y, Z>(
        mut trace: Box<Plot here<X, Y, Z>>,
        something: Option<&something>,
    ) -> Box<Plot here<X, Y, Z>>
    where
        X: Serialize + Clone,
        Y: Serialize + Clone,
        Z: Serialize + Clone,
    {
        if let Some(something) = something {
            trace = trace.something(something.to_plotly())
        }

        trace
    }
}

impl Layout for TemplatePlot {}
impl Polar for TemplatePlot {}

impl PlotHelper for TemplatePlot {
    fn get_layout(&self) -> &LayoutPlotly {
        &self.layout
    }

    fn get_traces(&self) -> &Vec<Box<dyn Trace + 'static>> {
        &self.traces
    }
}
