use plotlars::{Mode, Plot, Rgb, ScatterGeo, Shape, Text};
use polars::prelude::*;

fn main() {
    // Example 1: Basic ScatterGeo with markers only
    let cities = df![
        "city" => ["New York", "Los Angeles", "Chicago", "Houston", "Phoenix"],
        "lat" => [40.7128, 34.0522, 41.8781, 29.7604, 33.4484],
        "lon" => [-74.0060, -118.2437, -87.6298, -95.3698, -112.0740],
    ]
    .unwrap();

    ScatterGeo::builder()
        .data(&cities)
        .lat("lat")
        .lon("lon")
        .text("city")
        .plot_title(Text::from("US Major Cities").font("Arial").size(20))
        .build()
        .plot();

    // Example 2: ScatterGeo with grouping by region
    let cities_with_regions = df![
        "city" => ["New York", "Los Angeles", "Chicago", "Houston", "Phoenix", "Philadelphia", "San Antonio", "San Diego", "Dallas", "San Jose"],
        "lat" => [40.7128, 34.0522, 41.8781, 29.7604, 33.4484, 39.9526, 29.4241, 32.7157, 32.7767, 37.3382],
        "lon" => [-74.0060, -118.2437, -87.6298, -95.3698, -112.0740, -75.1652, -98.4936, -117.1611, -96.7970, -121.8863],
        "population" => [8336817, 3979576, 2693976, 2320268, 1680992, 1584064, 1547253, 1423851, 1343573, 1021795],
        "region" => ["Northeast", "West", "Midwest", "South", "West", "Northeast", "South", "West", "South", "West"]
    ]
    .unwrap();

    ScatterGeo::builder()
        .data(&cities_with_regions)
        .lat("lat")
        .lon("lon")
        .mode(Mode::Markers)
        .text("city")
        .group("region")
        .size(20)
        .colors(vec![
            Rgb(255, 0, 0),
            Rgb(0, 255, 0),
            Rgb(0, 0, 255),
            Rgb(255, 165, 0),
        ])
        .shapes(vec![
            Shape::Circle,
            Shape::Square,
            Shape::Diamond,
            Shape::Cross,
        ])
        .plot_title(
            Text::from("US Cities by Region")
                .font("Arial")
                .size(24)
                .x(0.5),
        )
        .legend_title(Text::from("Region").size(14))
        .build()
        .plot();

    // Example 3: ScatterGeo with lines connecting cities (flight paths)
    let flight_path = df![
        "city" => ["New York", "Chicago", "Denver", "Los Angeles"],
        "lat" => [40.7128, 41.8781, 39.7392, 34.0522],
        "lon" => [-74.0060, -87.6298, -104.9903, -118.2437],
    ]
    .unwrap();

    ScatterGeo::builder()
        .data(&flight_path)
        .lat("lat")
        .lon("lon")
        .mode(Mode::LinesMarkers)
        .text("city")
        .size(15)
        .color(Rgb(0, 123, 255))
        .line_width(2.0)
        .line_color(Rgb(255, 123, 0))
        .opacity(0.8)
        .plot_title(Text::from("Flight Path: NY to LA").font("Arial").size(20))
        .build()
        .plot();

    // Example 4: World cities with custom styling
    let world_cities = df![
        "city" => ["London", "Paris", "Tokyo", "Sydney", "Cairo", "Mumbai", "Beijing", "Rio de Janeiro", "Toronto"],
        "lat" => [51.5074, 48.8566, 35.6762, -33.8688, 30.0444, 19.0760, 39.9042, -22.9068, 43.6532],
        "lon" => [-0.1278, 2.3522, 139.6503, 151.2093, 31.2357, 72.8777, 116.4074, -43.1729, -79.3832],
        "continent" => ["Europe", "Europe", "Asia", "Oceania", "Africa", "Asia", "Asia", "South America", "North America"],
        "population_millions" => [9.0, 2.2, 13.9, 5.3, 9.5, 12.4, 21.5, 6.7, 2.9]
    ]
    .unwrap();

    ScatterGeo::builder()
        .data(&world_cities)
        .lat("lat")
        .lon("lon")
        .mode(Mode::Markers)
        .text("city")
        .group("continent")
        .size(25)
        .opacity(0.7)
        .colors(vec![
            Rgb(255, 0, 0),
            Rgb(0, 255, 0),
            Rgb(0, 0, 255),
            Rgb(255, 255, 0),
            Rgb(255, 0, 255),
            Rgb(0, 255, 255),
        ])
        .plot_title(
            Text::from("Major World Cities by Continent")
                .font("Arial")
                .size(24),
        )
        .legend_title(Text::from("Continent").size(16))
        .build()
        .plot();
}
