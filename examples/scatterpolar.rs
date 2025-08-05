use plotlars::{Fill, Legend, Line, Mode, Plot, Rgb, ScatterPolar, Shape, Text};
use polars::prelude::*;

fn main() {
    // Example 1: Basic scatter polar plot with markers only
    basic_scatter_polar();

    // Example 2: Lines and markers with custom styling
    styled_scatter_polar();

    // Example 3: Grouped data with multiple traces
    grouped_scatter_polar();

    // Example 4: Filled area polar plot
    filled_scatter_polar();
}

fn basic_scatter_polar() {
    // Create sample data - wind direction and speed
    let directions = vec![0., 45., 90., 135., 180., 225., 270., 315., 360.];
    let speeds = vec![5.0, 7.5, 10.0, 8.5, 6.0, 4.5, 3.0, 2.5, 5.0];

    let dataset = DataFrame::new(vec![
        Column::new("direction".into(), directions),
        Column::new("speed".into(), speeds),
    ])
    .unwrap();

    ScatterPolar::builder()
        .data(&dataset)
        .theta("direction")
        .r("speed")
        .mode(Mode::Markers)
        .color(Rgb(65, 105, 225))
        .shape(Shape::Circle)
        .size(10)
        .plot_title(Text::from("Wind Speed by Direction").font("Arial").size(20))
        .build()
        .plot();
}

fn styled_scatter_polar() {
    // Create sample data - radar chart style
    let categories = vec![0., 72., 144., 216., 288., 360.];
    let performance = vec![8.0, 6.5, 7.0, 9.0, 5.5, 8.0];

    let dataset = DataFrame::new(vec![
        Column::new("category".into(), categories),
        Column::new("performance".into(), performance),
    ])
    .unwrap();

    ScatterPolar::builder()
        .data(&dataset)
        .theta("category")
        .r("performance")
        .mode(Mode::LinesMarkers)
        .color(Rgb(255, 0, 0))
        .shape(Shape::Diamond)
        .line(Line::Solid)
        .width(3.0)
        .size(12)
        .opacity(0.8)
        .plot_title(
            Text::from("Performance Radar Chart")
                .font("Arial")
                .size(22)
                .x(0.5),
        )
        .build()
        .plot();
}

fn grouped_scatter_polar() {
    // Create sample data - comparing two products across multiple metrics
    let angles = vec![
        0., 60., 120., 180., 240., 300., 360., // Product A
        0., 60., 120., 180., 240., 300., 360., // Product B
    ];
    let values = vec![
        7.0, 8.5, 6.0, 5.5, 9.0, 8.0, 7.0, // Product A values
        6.0, 7.0, 8.0, 9.0, 6.5, 7.5, 6.0, // Product B values
    ];
    let products = vec![
        "Product A",
        "Product A",
        "Product A",
        "Product A",
        "Product A",
        "Product A",
        "Product A",
        "Product B",
        "Product B",
        "Product B",
        "Product B",
        "Product B",
        "Product B",
        "Product B",
    ];

    let dataset = DataFrame::new(vec![
        Column::new("angle".into(), angles),
        Column::new("score".into(), values),
        Column::new("product".into(), products),
    ])
    .unwrap();

    ScatterPolar::builder()
        .data(&dataset)
        .theta("angle")
        .r("score")
        .group("product")
        .mode(Mode::LinesMarkers)
        .colors(vec![
            Rgb(255, 99, 71),  // Tomato red
            Rgb(60, 179, 113), // Medium sea green
        ])
        .shapes(vec![Shape::Circle, Shape::Square])
        .lines(vec![Line::Solid, Line::Dash])
        .width(2.5)
        .size(8)
        .plot_title(Text::from("Product Comparison").font("Arial").size(24))
        .legend_title(Text::from("Products").font("Arial").size(14))
        .legend(&Legend::new().x(0.85).y(0.95))
        .build()
        .plot();
}

fn filled_scatter_polar() {
    // Create sample data - filled area chart
    let angles: Vec<f64> = (0..=360).step_by(10).map(|x| x as f64).collect();
    let radii: Vec<f64> = angles
        .iter()
        .map(|&angle| 5.0 + 3.0 * (angle * std::f64::consts::PI / 180.0).sin())
        .collect();

    let dataset = DataFrame::new(vec![
        Column::new("angle".into(), angles),
        Column::new("radius".into(), radii),
    ])
    .unwrap();

    ScatterPolar::builder()
        .data(&dataset)
        .theta("angle")
        .r("radius")
        .mode(Mode::Lines)
        .fill(Fill::ToSelf)
        .color(Rgb(135, 206, 250))
        .line(Line::Solid)
        .width(2.0)
        .opacity(0.6)
        .plot_title(Text::from("Filled Polar Area Chart").font("Arial").size(20))
        .build()
        .plot();
}
