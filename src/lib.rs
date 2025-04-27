#![doc = include_str!("../README.md")]
#![allow(clippy::needless_doctest_main)]

mod common;
mod components;
mod plots;

pub use crate::{
    common::plot::Plot,
    components::{
        axis::{Axis, AxisSide, AxisType},
        color::Rgb,
        colorbar::ColorBar,
        coloring::Coloring,
        exponent::ValueExponent,
        legend::Legend,
        lighting::Lighting,
        line::Line,
        orientation::Orientation,
        palette::Palette,
        shape::Shape,
        text::Text,
        tick::TickDirection,
    },
    plots::{
        array2dplot::Array2dPlot,
        barplot::BarPlot,
        boxplot::BoxPlot,
        contourplot::ContourPlot,
        heatmap::HeatMap,
        histogram::Histogram,
        image::Image,
        lineplot::LinePlot,
        piechart::PieChart,
        sankeydiagram::SankeyDiagram,
        scatter3dplot::Scatter3dPlot,
        scattermap::ScatterMap,
        scatterplot::ScatterPlot,
        surfaceplot::SurfacePlot,
        timeseriesplot::TimeSeriesPlot,
    },
};
