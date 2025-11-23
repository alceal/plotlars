// This example requires one of the export features to be enabled:
// cargo run --example export_image --features export-default
// cargo run --example export_image --features export-chrome
// cargo run --example export_image --features export-firefox

#[cfg(any(
    feature = "export-chrome",
    feature = "export-firefox",
    feature = "export-default"
))]
use plotlars::{Plot, ScatterPlot};

#[cfg(any(
    feature = "export-chrome",
    feature = "export-firefox",
    feature = "export-default"
))]
use polars::prelude::*;

#[cfg(any(
    feature = "export-chrome",
    feature = "export-firefox",
    feature = "export-default"
))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create sample data
    let dataset = df! {
        "x" => &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
        "y" => &[2, 4, 6, 8, 10, 12, 14, 16, 18, 20],
        "category" => &["A", "A", "B", "B", "A", "B", "A", "B", "A", "B"],
    }?;

    // Create a scatter plot
    let plot = ScatterPlot::builder()
        .data(&dataset)
        .x("x")
        .y("y")
        .group("category")
        .plot_title("Export Image Example")
        .x_title("X Values")
        .y_title("Y Values")
        .build();

    // Export in different formats
    println!("Exporting PNG...");
    plot.write_image("output_scatter.png", 1200, 800, 2.0)?;

    println!("Exporting JPEG...");
    plot.write_image("output_scatter.jpg", 1200, 800, 1.0)?;

    println!("Exporting SVG...");
    plot.write_image("output_scatter.svg", 1200, 800, 1.0)?;

    println!("All images exported successfully!");

    Ok(())
}

#[cfg(not(any(
    feature = "export-chrome",
    feature = "export-firefox",
    feature = "export-default"
)))]
fn main() {
    eprintln!("This example requires one of the export features to be enabled.");
    eprintln!("Run with: cargo run --example export_image --features export-default");
}
