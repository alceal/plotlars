use bon::bon;

use polars::frame::DataFrame;

use crate::{
    components::{Legend, Text},
    ir::data::ColumnData,
    ir::layout::{LayoutIR, MapboxIR},
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
#[derive(Clone)]
#[allow(dead_code)]
pub struct DensityMapbox {
    traces: Vec<TraceIR>,
    layout: LayoutIR,
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
        let traces = vec![TraceIR::DensityMapbox(DensityMapboxIR {
            lat: ColumnData::Numeric(crate::data::get_numeric_column(data, lat)),
            lon: ColumnData::Numeric(crate::data::get_numeric_column(data, lon)),
            z: ColumnData::Numeric(crate::data::get_numeric_column(data, z)),
            radius,
            opacity,
            z_min,
            z_max,
            z_mid,
        })];

        let layout = LayoutIR {
            title: plot_title,
            x_title: None,
            y_title: None,
            y2_title: None,
            z_title: None,
            legend_title,
            legend: legend.cloned(),
            dimensions: None,
            bar_mode: None,
            box_mode: None,
            box_gap: None,
            margin_bottom: Some(0),
            axes_2d: None,
            scene_3d: None,
            polar: None,
            mapbox: Some(MapboxIR {
                center: center.map(|c| (c[0], c[1])),
                zoom: Some(zoom.map(|z| z as f64).unwrap_or(1.0)),
                style: None,
            }),
            grid: None,
            annotations: vec![],
        };

        Self { traces, layout }
    }
}

impl crate::Plot for DensityMapbox {
    fn ir_traces(&self) -> &[TraceIR] {
        &self.traces
    }

    fn ir_layout(&self) -> &LayoutIR {
        &self.layout
    }
}
