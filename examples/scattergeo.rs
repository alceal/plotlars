use plotlars::{Mode, Plot, Rgb, ScatterGeo, Shape, Text};
use polars::prelude::*;

fn main() {
    let cities = LazyCsvReader::new(PlPath::new("data/us_cities_regions.csv"))
        .finish()
        .unwrap()
        .select([col("city"), col("lat"), col("lon")])
        .limit(5)
        .collect()
        .unwrap();

    ScatterGeo::builder()
        .data(&cities)
        .lat("lat")
        .lon("lon")
        .text("city")
        .plot_title(Text::from("US Major Cities").font("Arial").size(20))
        .build()
        .plot();

    let cities_with_regions = LazyCsvReader::new(PlPath::new("data/us_cities_regions.csv"))
        .finish()
        .unwrap()
        .collect()
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
    let flight_path = LazyCsvReader::new(PlPath::new("data/flight_path.csv"))
        .finish()
        .unwrap()
        .collect()
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
    let world_cities = LazyCsvReader::new(PlPath::new("data/world_cities.csv"))
        .finish()
        .unwrap()
        .collect()
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
