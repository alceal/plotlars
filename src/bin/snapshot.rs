use std::fs;
use std::path::Path;

use plotlars::{
    Array2dPlot, Axis, BarPlot, BoxPlot, CandlestickPlot, ColorBar, ContourPlot, DensityMapbox,
    HeatMap, Histogram, Image, Legend, LinePlot, Mesh3D, Mode, OhlcPlot, Orientation, Palette,
    PieChart, Plot, Rgb, SankeyDiagram, Scatter3dPlot, ScatterGeo, ScatterMap, ScatterPlot,
    ScatterPolar, Shape, SurfacePlot, Table, Text, TimeSeriesPlot,
};
use polars::prelude::*;

fn main() {
    let out_dir = Path::new("snapshots");
    fs::create_dir_all(out_dir).expect("Failed to create snapshots directory");

    snapshot("barplot", build_barplot().to_json(), out_dir);
    snapshot("boxplot", build_boxplot().to_json(), out_dir);
    snapshot("scatterplot", build_scatterplot().to_json(), out_dir);
    snapshot("lineplot", build_lineplot().to_json(), out_dir);
    snapshot("histogram", build_histogram().to_json(), out_dir);
    snapshot("timeseriesplot", build_timeseriesplot().to_json(), out_dir);
    snapshot("contourplot", build_contourplot().to_json(), out_dir);
    snapshot("heatmap", build_heatmap().to_json(), out_dir);
    snapshot("scatter3dplot", build_scatter3dplot().to_json(), out_dir);
    snapshot("surfaceplot", build_surfaceplot().to_json(), out_dir);
    snapshot("mesh3d", build_mesh3d().to_json(), out_dir);
    snapshot("scatterpolar", build_scatterpolar().to_json(), out_dir);
    snapshot("piechart", build_piechart().to_json(), out_dir);
    snapshot("sankeydiagram", build_sankeydiagram().to_json(), out_dir);
    snapshot("scattergeo", build_scattergeo().to_json(), out_dir);
    snapshot("scattermap", build_scattermap().to_json(), out_dir);
    snapshot("density_mapbox", build_density_mapbox().to_json(), out_dir);
    snapshot("candlestick", build_candlestick().to_json(), out_dir);
    snapshot("ohlc", build_ohlc().to_json(), out_dir);
    snapshot("table", build_table().to_json(), out_dir);
    snapshot("image", build_image().to_json(), out_dir);
    snapshot("array2dplot", build_array2dplot().to_json(), out_dir);

    println!("All 22 snapshots written to {}", out_dir.display());
}

fn snapshot(name: &str, result: Result<String, serde_json::Error>, out_dir: &Path) {
    let json = result.unwrap_or_else(|e| panic!("Failed to serialize {}: {}", name, e));

    let pretty: serde_json::Value =
        serde_json::from_str(&json).unwrap_or_else(|e| panic!("Failed to parse {}: {}", name, e));
    let pretty_json = serde_json::to_string_pretty(&pretty)
        .unwrap_or_else(|e| panic!("Failed to prettify {}: {}", name, e));

    let path = out_dir.join(format!("{}.json", name));
    fs::write(&path, &pretty_json)
        .unwrap_or_else(|e| panic!("Failed to write {}: {}", name, e));
    println!("  {} ({} bytes)", path.display(), pretty_json.len());
}

fn load_csv(path: &str) -> DataFrame {
    LazyCsvReader::new(PlRefPath::new(path))
        .finish()
        .unwrap()
        .collect()
        .unwrap()
}

fn build_barplot() -> BarPlot {
    let dataset = load_csv("data/animal_statistics.csv");
    BarPlot::builder()
        .data(&dataset)
        .labels("animal")
        .values("value")
        .orientation(Orientation::Vertical)
        .group("gender")
        .error("error")
        .colors(vec![Rgb(255, 127, 80), Rgb(64, 224, 208)])
        .plot_title(Text::from("Bar Plot").font("Arial").size(18))
        .x_title(Text::from("animal").font("Arial").size(15))
        .y_title(Text::from("value").font("Arial").size(15))
        .legend_title(Text::from("gender").font("Arial").size(15))
        .legend(
            &Legend::new()
                .orientation(Orientation::Horizontal)
                .y(1.0)
                .x(0.43),
        )
        .build()
}

fn build_boxplot() -> BoxPlot {
    let dataset = load_csv("data/penguins.csv");
    BoxPlot::builder()
        .data(&dataset)
        .labels("species")
        .values("body_mass_g")
        .group("sex")
        .colors(vec![Rgb(255, 127, 80), Rgb(64, 224, 208)])
        .plot_title(Text::from("Box Plot").font("Arial").size(18))
        .x_title("species")
        .y_title("body mass (g)")
        .legend_title("sex")
        .build()
}

fn build_scatterplot() -> ScatterPlot {
    let dataset = LazyCsvReader::new(PlRefPath::new("data/penguins.csv"))
        .finish()
        .unwrap()
        .select([
            col("species"),
            col("flipper_length_mm").cast(DataType::Int16),
            col("body_mass_g").cast(DataType::Int16),
        ])
        .collect()
        .unwrap();

    ScatterPlot::builder()
        .data(&dataset)
        .x("body_mass_g")
        .y("flipper_length_mm")
        .group("species")
        .opacity(0.5)
        .size(12)
        .colors(vec![Rgb(178, 34, 34), Rgb(65, 105, 225), Rgb(255, 140, 0)])
        .shapes(vec![Shape::Circle, Shape::Square, Shape::Diamond])
        .plot_title(Text::from("Scatter Plot").font("Arial").size(20))
        .x_title("body mass (g)")
        .y_title("flipper length (mm)")
        .legend_title("species")
        .legend(&Legend::new().x(0.85).y(0.15))
        .build()
}

fn build_lineplot() -> LinePlot {
    let x_values: Vec<f64> = (0..100).map(|i| i as f64 * 0.1).collect();
    let sine_values: Vec<f64> = x_values.iter().map(|x| x.sin()).collect();
    let cosine_values: Vec<f64> = x_values.iter().map(|x| x.cos()).collect();

    let dataset = df![
        "x" => &x_values,
        "sine" => &sine_values,
        "cosine" => &cosine_values,
    ]
    .unwrap();

    LinePlot::builder()
        .data(&dataset)
        .x("x")
        .y("sine")
        .additional_lines(vec!["cosine"])
        .colors(vec![Rgb(255, 0, 0), Rgb(0, 255, 0)])
        .plot_title(Text::from("Line Plot").font("Arial").size(18))
        .build()
}

fn build_histogram() -> Histogram {
    let dataset = load_csv("data/penguins.csv");
    Histogram::builder()
        .data(&dataset)
        .x("body_mass_g")
        .group("species")
        .colors(vec![
            Rgb(178, 34, 34),
            Rgb(65, 105, 225),
            Rgb(255, 140, 0),
        ])
        .plot_title(Text::from("Histogram").font("Arial").size(18))
        .x_title("body mass (g)")
        .y_title("count")
        .legend_title("species")
        .build()
}

fn build_timeseriesplot() -> TimeSeriesPlot {
    let dataset = LazyCsvReader::new(PlRefPath::new("data/revenue_and_cost.csv"))
        .finish()
        .unwrap()
        .select([
            col("Date").cast(DataType::String),
            col("Revenue").cast(DataType::Int32),
            col("Cost").cast(DataType::Int32),
        ])
        .collect()
        .unwrap();

    TimeSeriesPlot::builder()
        .data(&dataset)
        .x("Date")
        .y("Revenue")
        .additional_series(vec!["Cost"])
        .colors(vec![Rgb(0, 0, 255), Rgb(255, 0, 0)])
        .plot_title(Text::from("Time Series Plot").font("Arial").size(18))
        .build()
}

fn build_contourplot() -> ContourPlot {
    let dataset = load_csv("data/contour_surface.csv");
    ContourPlot::builder()
        .data(&dataset)
        .x("x")
        .y("y")
        .z("z")
        .plot_title(Text::from("Contour Plot").font("Arial").size(18))
        .build()
}

fn build_heatmap() -> HeatMap {
    let dataset = load_csv("data/heatmap.csv");
    HeatMap::builder()
        .data(&dataset)
        .x("x")
        .y("y")
        .z("z")
        .plot_title(Text::from("Heat Map").font("Arial").size(18))
        .build()
}

fn build_scatter3dplot() -> Scatter3dPlot {
    let dataset = LazyCsvReader::new(PlRefPath::new("data/penguins.csv"))
        .finish()
        .unwrap()
        .select([
            col("species"),
            col("bill_length_mm").cast(DataType::Float32),
            col("flipper_length_mm").cast(DataType::Int16),
            col("body_mass_g").cast(DataType::Int16),
        ])
        .collect()
        .unwrap();

    Scatter3dPlot::builder()
        .data(&dataset)
        .x("body_mass_g")
        .y("flipper_length_mm")
        .z("bill_length_mm")
        .group("species")
        .opacity(0.25)
        .size(8)
        .colors(vec![Rgb(178, 34, 34), Rgb(65, 105, 225), Rgb(255, 140, 0)])
        .shapes(vec![Shape::Circle, Shape::Square, Shape::Diamond])
        .plot_title("Scatter 3D Plot")
        .legend(&Legend::new().x(0.6))
        .build()
}

fn build_surfaceplot() -> SurfacePlot {
    let dataset = load_csv("data/contour_surface.csv");
    SurfacePlot::builder()
        .data(&dataset)
        .x("x")
        .y("y")
        .z("z")
        .color_scale(Palette::Viridis)
        .color_bar(&ColorBar::new().length(0.5))
        .plot_title(Text::from("Surface Plot").font("Arial").size(18))
        .build()
}

fn build_mesh3d() -> Mesh3D {
    let dataset = df! {
        "x" => &[0.0_f32, 1.0, 2.0, 0.0],
        "y" => &[0.0_f32, 0.0, 1.0, 2.0],
        "z" => &[0.0_f32, 2.0, 0.0, 1.0],
        "i" => &[0_i32, 0, 0, 1],
        "j" => &[1_i32, 2, 3, 2],
        "k" => &[2_i32, 3, 1, 3],
    }
    .unwrap();

    Mesh3D::builder()
        .data(&dataset)
        .x("x")
        .y("y")
        .z("z")
        .i("i")
        .j("j")
        .k("k")
        .plot_title("Mesh 3D")
        .build()
}

fn build_scatterpolar() -> ScatterPolar {
    let dataset = load_csv("data/product_comparison_polar.csv");
    ScatterPolar::builder()
        .data(&dataset)
        .theta("angle")
        .r("score")
        .group("product")
        .mode(Mode::LinesMarkers)
        .colors(vec![Rgb(0, 191, 255), Rgb(250, 128, 114)])
        .plot_title(Text::from("Scatter Polar").font("Arial").size(18))
        .legend_title("product")
        .build()
}

fn build_piechart() -> PieChart {
    let dataset = LazyCsvReader::new(PlRefPath::new("data/penguins.csv"))
        .finish()
        .unwrap()
        .select([col("species")])
        .collect()
        .unwrap();

    PieChart::builder()
        .data(&dataset)
        .labels("species")
        .hole(0.4)
        .pull(0.01)
        .rotation(20.0)
        .plot_title(Text::from("Pie Chart").font("Arial").size(18))
        .build()
}

fn build_sankeydiagram() -> SankeyDiagram {
    let dataset = load_csv("data/sankey_flow.csv");
    SankeyDiagram::builder()
        .data(&dataset)
        .sources("source")
        .targets("target")
        .values("value")
        .plot_title(Text::from("Sankey Diagram").font("Arial").size(18))
        .build()
}

fn build_scattergeo() -> ScatterGeo {
    let dataset = load_csv("data/world_cities.csv");
    ScatterGeo::builder()
        .data(&dataset)
        .lat("lat")
        .lon("lon")
        .text("city")
        .size(10)
        .plot_title(Text::from("Scatter Geo").font("Arial").size(18))
        .build()
}

fn build_scattermap() -> ScatterMap {
    let dataset = load_csv("data/cities.csv");
    ScatterMap::builder()
        .data(&dataset)
        .latitude("latitude")
        .longitude("longitude")
        .size(10)
        .plot_title(Text::from("Scatter Map").font("Arial").size(18))
        .build()
}

fn build_density_mapbox() -> DensityMapbox {
    let dataset = load_csv("data/us_city_density.csv");
    DensityMapbox::builder()
        .data(&dataset)
        .lat("city_lat")
        .lon("city_lon")
        .z("population_density")
        .plot_title(Text::from("Density Mapbox").font("Arial").size(18))
        .build()
}

fn build_candlestick() -> CandlestickPlot {
    let dataset = load_csv("data/stock_prices.csv");
    CandlestickPlot::builder()
        .data(&dataset)
        .dates("date")
        .open("open")
        .high("high")
        .low("low")
        .close("close")
        .plot_title(Text::from("Candlestick Plot").font("Arial").size(18))
        .build()
}

fn build_ohlc() -> OhlcPlot {
    let dataset = load_csv("data/stock_prices.csv");
    OhlcPlot::builder()
        .data(&dataset)
        .dates("date")
        .open("open")
        .high("high")
        .low("low")
        .close("close")
        .plot_title(Text::from("OHLC Plot").font("Arial").size(18))
        .build()
}

fn build_table() -> Table {
    let dataset = load_csv("data/penguins.csv");
    Table::builder()
        .data(&dataset)
        .columns(vec!["species", "island", "body_mass_g"])
        .plot_title(Text::from("Table").font("Arial").size(18))
        .build()
}

fn build_image() -> Image {
    Image::builder()
        .path("data/image.png")
        .x_axis(&Axis::new().show_axis(false))
        .y_axis(&Axis::new().show_axis(false))
        .plot_title("Image Plot")
        .build()
}

fn build_array2dplot() -> Array2dPlot {
    let data = vec![
        vec![[255, 0, 0], [0, 255, 0], [0, 0, 255]],
        vec![[0, 0, 255], [255, 0, 0], [0, 255, 0]],
        vec![[0, 255, 0], [0, 0, 255], [255, 0, 0]],
    ];

    Array2dPlot::builder()
        .data(&data)
        .plot_title(Text::from("Array 2D Plot").font("Arial").size(18))
        .build()
}
