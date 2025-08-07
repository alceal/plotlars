use bon::bon;

use plotly::{
    DensityMapbox as DensityMapboxPlotly, Trace,
    layout::{Center, Layout as LayoutPlotly, Mapbox, MapboxStyle, Margin},
};

use polars::frame::DataFrame;
use serde::Serialize;

use crate::{
    common::{Layout, PlotHelper, Polar},
    components::{Legend, Text},
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
/// // Create sample data with US city population density
/// let data = df![
///     "city_lat" => [40.7128, 34.0522, 41.8781, 29.7604, 33.4484, 37.7749, 47.6062, 42.3601,
///                    32.7767, 39.9526, 38.9072, 35.2271, 30.2672, 36.1699, 39.7392],
///     "city_lon" => [-74.0060, -118.2437, -87.6298, -95.3698, -112.0740, -122.4194, -122.3321, -71.0589,
///                    -79.9309, -75.1652, -77.0369, -80.8431, -97.7431, -115.1398, -104.9903],
///     "population_density" => [27000.0, 8092.0, 11841.0, 3540.0, 3165.0, 18581.0, 8386.0, 13321.0,
///                              4707.0, 11379.0, 9856.0, 2457.0, 1386.0, 4525.0, 4193.0],
///     "city_name" => ["New York", "Los Angeles", "Chicago", "Houston", "Phoenix", "San Francisco",
///                     "Seattle", "Boston", "Charleston", "Philadelphia", "Washington DC",
///                     "Charlotte", "Austin", "Las Vegas", "Denver"]
/// ].unwrap();
///
/// DensityMapbox::builder()
///     .data(&data)
///     .lat("city_lat")
///     .lon("city_lon")
///     .z("population_density")
///     .center([39.8283, -98.5795])
///     .zoom(3)
///     .plot_title(
///         Text::from("US City Population Density")
///             .font("Arial")
///             .size(20)
///     )
///     .build()
///     .plot();
/// ```
///
/// ![Example](https://imgur.com/6sHke2h.png)
#[derive(Clone, Serialize)]
pub struct DensityMapbox {
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

        let traces = Self::create_traces(data, lat, lon, z, radius, opacity, z_min, z_max, z_mid);

        Self { traces, layout }
    }

    #[allow(clippy::too_many_arguments)]
    fn create_traces(
        data: &DataFrame,
        lat: &str,
        lon: &str,
        z: &str,
        radius: Option<u8>,
        opacity: Option<f64>,
        z_min: Option<f64>,
        z_max: Option<f64>,
        z_mid: Option<f64>,
    ) -> Vec<Box<dyn Trace + 'static>> {
        let mut traces: Vec<Box<dyn Trace + 'static>> = Vec::new();

        let trace = Self::create_trace(data, lat, lon, z, radius, opacity, z_min, z_max, z_mid);

        traces.push(trace);
        traces
    }

    #[allow(clippy::too_many_arguments)]
    fn create_trace(
        data: &DataFrame,
        lat: &str,
        lon: &str,
        z: &str,
        radius: Option<u8>,
        opacity: Option<f64>,
        z_min: Option<f64>,
        z_max: Option<f64>,
        z_mid: Option<f64>,
    ) -> Box<dyn Trace + 'static> {
        let lat_data = Self::get_numeric_column(data, lat);
        let lon_data = Self::get_numeric_column(data, lon);
        let z_data = Self::get_numeric_column(data, z);

        let mut trace = DensityMapboxPlotly::new(lat_data, lon_data, z_data);

        if let Some(radius) = radius {
            trace = trace.radius(radius);
        }

        if let Some(opacity) = opacity {
            trace = trace.opacity(opacity);
        }

        if let Some(z_min) = z_min {
            trace = trace.zmin(Some(z_min as f32));
        }

        if let Some(z_max) = z_max {
            trace = trace.zmax(Some(z_max as f32));
        }

        if let Some(z_mid) = z_mid {
            trace = trace.zmid(Some(z_mid as f32));
        }

        trace
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
}
