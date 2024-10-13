//! This module provides implementations for heatmaps using the Plotly library.
//!
//! The `HeatMap` struct allow for the creation and customization of heatmaps
//! with various options for data, layout, and aesthetics.

use bon::bon;

use plotly::{
    common::{Line as LinePlotly, Marker},
    HeatMap as HeatMapPlotly, Layout as LayoutPlotly, Trace as TracePlotly,
};

use polars::frame::DataFrame;

use crate::{
    aesthetics::{line::Line, mark::Mark},
    legends::colorbar::ColorBar,
    traits::{layout::Layout, polar::Polar, trace::Trace},
    Axis, Legend, Orientation, Plot, Text,
};

/// A structure representing a bar plot.
pub struct HeatMap {
    traces: Vec<Box<dyn TracePlotly + 'static>>,
    layout: LayoutPlotly,
}

#[bon]
impl HeatMap {
    #[builder(on(String, into), on(Text, into))]
    pub fn new(
        data: &DataFrame,
        x: String,
        y: String,
        z: String,
        // Layout
        plot_title: Option<Text>,
        x_title: Option<Text>,
        y_title: Option<Text>,
        legend_title: Option<Text>,
        x_axis: Option<&Axis>,
        y_axis: Option<&Axis>,
        legend: Option<&Legend>,
        color_bar: Option<&ColorBar>,
    ) -> Self {
        let x_col = x.as_str();
        let y_col = y.as_str();
        let z_col = z.as_str();

        // Layout
        let bar_mode = None;

        let layout = Self::create_layout(
            bar_mode,
            plot_title,
            x_title,
            x_axis,
            y_title,
            y_axis,
            legend_title,
            legend,
        );

        // Trace
        let orientation = None;
        let error = None;
        let box_points = None;
        let point_offset = None;
        let jitter = None;
        let additional_series = None;
        let line_types = None;
        let with_shape = None;
        let line_width = None;

        let group = None;
        let opacity = None;
        let size = None;
        let color = None;
        let colors = None;
        let shape = None;
        let shapes = None;

        let traces = Self::create_traces(
            data,
            x_col,
            y_col,
            z_col,
            orientation,
            group,
            error,
            box_points,
            point_offset,
            jitter,
            additional_series,
            opacity,
            size,
            color,
            colors,
            with_shape,
            shape,
            shapes,
            line_types,
            line_width,
            color_bar,
        );

        Self { traces, layout }
    }
}

impl Layout for HeatMap {}
impl Polar for HeatMap {}
impl Mark for HeatMap {}
impl Line for HeatMap {}

impl Trace for HeatMap {
    fn create_trace(
        data: &DataFrame,
        x_col: &str,
        y_col: &str,
        z_col: &str,
        #[allow(unused_variables)] orientation: Option<Orientation>,
        #[allow(unused_variables)] group_name: Option<&str>,
        #[allow(unused_variables)] error: Option<String>,
        #[allow(unused_variables)] box_points: Option<bool>,
        #[allow(unused_variables)] point_offset: Option<f64>,
        #[allow(unused_variables)] jitter: Option<f64>,
        #[allow(unused_variables)] with_shape: Option<bool>,
        #[allow(unused_variables)] marker: Marker,
        #[allow(unused_variables)] line: LinePlotly,
        color_bar: Option<&ColorBar>,
    ) -> Box<dyn TracePlotly + 'static> {
        let x_data = Self::get_string_column(data, x_col);
        let y_data = Self::get_string_column(data, y_col);
        let z_data = Self::get_numeric_column(data, z_col);

        let mut heat_map = HeatMapPlotly::default().x(x_data).y(y_data).z(z_data);
        // .auto_color_scale(true)

        if let Some(color_bar) = color_bar {
            heat_map = heat_map.color_bar(color_bar.to_plotly())
        }

        heat_map
    }
}

impl Plot for HeatMap {
    fn get_layout(&self) -> &LayoutPlotly {
        &self.layout
    }

    fn get_traces(&self) -> &Vec<Box<dyn TracePlotly + 'static>> {
        &self.traces
    }
}
