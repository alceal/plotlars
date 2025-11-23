// This example requires one of the export features to be enabled:
// cargo run --example export_image --features export-default
// cargo run --example export_image --features export-chrome
// cargo run --example export_image --features export-firefox

#[cfg(any(
    feature = "export-chrome",
    feature = "export-firefox",
    feature = "export-default"
))]
use plotlars::{Axis, BoxPlot, Legend, Orientation, Plot, Rgb, Text};

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
    // Load penguins dataset
    let dataset = LazyCsvReader::new(PlPath::new("data/penguins.csv"))
        .finish()?
        .select([
            col("species"),
            col("sex").alias("gender"),
            col("flipper_length_mm").cast(DataType::Int16),
            col("body_mass_g").cast(DataType::Int16),
        ])
        .collect()?;

    // Create a box plot
    let plot = BoxPlot::builder()
        .data(&dataset)
        .labels("species")
        .values("body_mass_g")
        .orientation(Orientation::Vertical)
        .group("gender")
        .box_points(true)
        .point_offset(-1.5)
        .jitter(0.01)
        .opacity(0.1)
        .colors(vec![Rgb(0, 191, 255), Rgb(57, 255, 20), Rgb(255, 105, 180)])
        .plot_title(
            Text::from("Box Plot - Export Example")
                .font("Arial")
                .size(18),
        )
        .x_title(Text::from("species").font("Arial").size(15))
        .y_title(Text::from("body mass (g)").font("Arial").size(15).x(-0.04))
        .legend_title(Text::from("gender").font("Arial").size(15))
        .y_axis(&Axis::new().value_thousands(true))
        .legend(&Legend::new().border_width(1).x(0.9))
        .build();

    // Export in different formats
    println!("Exporting PNG...");
    plot.write_image("output_boxplot.png", 1200, 800, 2.0)?;

    println!("Exporting JPEG...");
    plot.write_image("output_boxplot.jpg", 1200, 800, 1.0)?;

    println!("Exporting SVG...");
    plot.write_image("output_boxplot.svg", 1200, 800, 1.0)?;

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
