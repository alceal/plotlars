use bon::bon;

use plotly::{
    layout::{Center, Layout as LayoutPlotly, Mapbox, MapboxStyle, Margin},
    Trace,
};

use polars::frame::DataFrame;
use serde::Serialize;

use crate::{
    common::{Layout, PlotHelper, Polar},
    components::{Legend, Text},
    ir::data::ColumnData,
    ir::layout::LayoutIR,
    ir::trace::{DensityMapboxIR, TraceIR},
};

/// A structure representing a density mapbox visualization.
///
/// The `DensityMapbox` struct enables the creation of geographic density visualizations on an interactive map.
/// It displays density or intensity values at geographic locations using latitude and longitude coordinates,
/// with a third dimension (z) representing the intensity at each point. This is useful for visualizing
/// population density, heat maps of activity, or any geographic concentration of values.
///
/// # Arguments
///
/// * `data` - A reference to the `DataFrame` containing the data to be plotted.
/// * `lat` - A string slice specifying the column name containing latitude values.
/// * `lon` - A string slice specifying the column name containing longitude values.
/// * `z` - A string slice specifying the column name containing intensity/density values.
/// * `center` - An optional array `[f64; 2]` specifying the initial center point of the map ([latitude, longitude]).
/// * `zoom` - An optional `u8` specifying the initial zoom level of the map.
/// * `radius` - An optional `u8` specifying the radius of influence for each point.
/// * `opacity` - An optional `f64` value between `0.0` and `1.0` specifying the opacity of the density layer.
/// * `z_min` - An optional `f64` specifying the minimum value for the color scale.
/// * `z_max` - An optional `f64` specifying the maximum value for the color scale.
/// * `z_mid` - An optional `f64` specifying the midpoint value for the color scale.
/// * `plot_title` - An optional `Text` struct specifying the title of the plot.
/// * `legend_title` - An optional `Text` struct specifying the title of the legend.
/// * `legend` - An optional reference to a `Legend` struct for customizing the legend.
///
/// # Example
///
/// ```rust
/// use plotlars::{DensityMapbox, Plot, Text};
/// use polars::prelude::*;
///
/// let data = LazyCsvReader::new(PlRefPath::new("data/us_city_density.csv"))
///     .finish()
///     .unwrap()
///     .collect()
///     .unwrap();
///
/// DensityMapbox::builder()
///     .data(&data)
///     .lat("city_lat")
///     .lon("city_lon")
///     .z("population_density")
///     .center([39.8283, -98.5795])
///     .zoom(3)
///     .plot_title(
///         Text::from("Density Mapbox")
///             .font("Arial")
///             .size(20)
///     )
///     .build()
///     .plot();
/// ```
///
/// ![Example](https://imgur.com/82eLyBm.png)
#[derive(Clone, Serialize)]
#[allow(dead_code)]
pub struct DensityMapbox {
    #[serde(skip)]
    ir_traces: Vec<TraceIR>,
    #[serde(skip)]
    ir_layout: LayoutIR,
    traces: Vec<Box<dyn Trace + 'static>>,
    layout: LayoutPlotly,
}

#[bon]
impl DensityMapbox {
    #[builder(on(String, into), on(Text, into))]
    pub fn new(
        data: &DataFrame,
        lat: &str,
        lon: &str,
        z: &str,
        center: Option<[f64; 2]>,
        zoom: Option<u8>,
        radius: Option<u8>,
        opacity: Option<f64>,
        z_min: Option<f64>,
        z_max: Option<f64>,
        z_mid: Option<f64>,
        plot_title: Option<Text>,
        legend_title: Option<Text>,
        legend: Option<&Legend>,
    ) -> Self {
        let x_title = None;
        let y_title = None;
        let z_title = None;
        let x_axis = None;
        let y_axis = None;
        let z_axis = None;
        let y2_title = None;
        let y2_axis = None;

        let ir_title = plot_title.clone();
        let ir_legend_title = legend_title.clone();

        let mut layout = Self::create_layout(
            plot_title,
            x_title,
            y_title,
            y2_title,
            z_title,
            legend_title,
            x_axis,
            y_axis,
            y2_axis,
            z_axis,
            legend,
            None,
        )
        .margin(Margin::new().bottom(0));

        let mut map_box = Mapbox::new().style(MapboxStyle::OpenStreetMap);

        if let Some(center) = center {
            map_box = map_box.center(Center::new(center[0], center[1]));
        }

        if let Some(zoom) = zoom {
            map_box = map_box.zoom(zoom);
        } else {
            map_box = map_box.zoom(1);
        }

        layout = layout.mapbox(map_box);

        // Build IR
        let lat_data = ColumnData::Numeric(Self::get_numeric_column(data, lat));
        let lon_data = ColumnData::Numeric(Self::get_numeric_column(data, lon));
        let z_data = ColumnData::Numeric(Self::get_numeric_column(data, z));

        let ir_trace = TraceIR::DensityMapbox(DensityMapboxIR {
            lat: lat_data,
            lon: lon_data,
            z: z_data,
            radius,
            opacity,
            z_min,
            z_max,
            z_mid,
        });
        let ir_traces = vec![ir_trace];

        let ir_layout = LayoutIR {
            title: ir_title,
            x_title: None,
            y_title: None,
            y2_title: None,
            z_title: None,
            legend_title: ir_legend_title,
            legend: legend.cloned(),
            dimensions: None,
            bar_mode: None,
            axes_2d: None,
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

        Self {
            ir_traces,
            ir_layout,
            traces: plotly_traces,
            layout,
        }
    }
}

impl Layout for DensityMapbox {}
impl Polar for DensityMapbox {}

impl PlotHelper for DensityMapbox {
    fn get_layout(&self) -> &LayoutPlotly {
        &self.layout
    }

    fn get_traces(&self) -> &Vec<Box<dyn Trace + 'static>> {
        &self.traces
    }

    #[allow(private_interfaces)]
    fn get_ir_layout(&self) -> Option<&LayoutIR> {
        Some(&self.ir_layout)
    }

    #[allow(private_interfaces)]
    fn get_ir_traces(&self) -> Option<&[TraceIR]> {
        Some(&self.ir_traces)
    }
}
