use plotlars::{DensityMapbox, Plot, Text};
use polars::prelude::*;

fn main() {
    // Create sample data with US city population density
    let data = df![
        "city_lat" => [40.7128, 34.0522, 41.8781, 29.7604, 33.4484, 37.7749, 47.6062, 42.3601,
                       32.7767, 39.9526, 38.9072, 35.2271, 30.2672, 36.1699, 39.7392],
        "city_lon" => [-74.0060, -118.2437, -87.6298, -95.3698, -112.0740, -122.4194, -122.3321, -71.0589,
                       -79.9309, -75.1652, -77.0369, -80.8431, -97.7431, -115.1398, -104.9903],
        "population_density" => [27000.0, 8092.0, 11841.0, 3540.0, 3165.0, 18581.0, 8386.0, 13321.0,
                                 4707.0, 11379.0, 9856.0, 2457.0, 1386.0, 4525.0, 4193.0],
        "city_name" => ["New York", "Los Angeles", "Chicago", "Houston", "Phoenix", "San Francisco",
                       "Seattle", "Boston", "Charleston", "Philadelphia", "Washington DC",
                       "Charlotte", "Austin", "Las Vegas", "Denver"]
    ].unwrap();

    DensityMapbox::builder()
        .data(&data)
        .lat("city_lat")
        .lon("city_lon")
        .z("population_density")
        .center([39.8283, -98.5795]) // Center of USA
        .zoom(3)
        .plot_title(
            Text::from("US City Population Density")
                .font("Arial")
                .size(20),
        )
        .build()
        .plot();

    println!("Density map plotted!");
}
