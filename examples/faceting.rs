use plotlars::{
    Arrangement, BarPlot, BoxPlot, ContourPlot, FacetConfig, FacetScales, HeatMap, Histogram,
    Lighting, Line, LinePlot, Mesh3D, Mode, Palette, PieChart, Plot, Rgb, SankeyDiagram,
    Scatter3dPlot, ScatterPlot, ScatterPolar, Shape, SurfacePlot, Text, TimeSeriesPlot,
};
use polars::prelude::*;

fn main() {
    barplot_example();
    boxplot_example();
    contourplot_example();
    heatmap_example();
    histogram_example();
    lineplot_example();
    mesh3d_example();
    piechart_example();
    sankeydiagram_example();
    scatter3d_example();
    scatterplot_example();
    scatterpolar_example();
    surfaceplot_example();
    timeseriesplot_example();
}

fn barplot_example() {
    let regional_data = CsvReadOptions::default()
        .with_has_header(true)
        .try_into_reader_with_file_path(Some("data/regional_sales.csv".into()))
        .unwrap()
        .finish()
        .unwrap();

    let facet_config = FacetConfig::new().cols(4).rows(2).h_gap(0.05).v_gap(0.30);

    BarPlot::builder()
        .data(&regional_data)
        .labels("product")
        .values("sales")
        .facet("region")
        .facet_config(&facet_config)
        .color(Rgb(70, 130, 180))
        .plot_title(Text::from("8-Region Sales Facet Grid"))
        .x_title("Product")
        .y_title("Sales")
        .build()
        .plot();
}

fn boxplot_example() {
    let dataset = LazyCsvReader::new(PlPath::new("data/penguins.csv"))
        .finish()
        .unwrap()
        .select([
            col("species"),
            col("island"),
            col("sex"),
            col("body_mass_g").cast(DataType::Int16),
        ])
        .collect()
        .unwrap();

    BoxPlot::builder()
        .data(&dataset)
        .labels("island")
        .values("body_mass_g")
        .group("sex")
        .colors(vec![Rgb(0, 119, 182), Rgb(0, 180, 216), Rgb(144, 224, 239)])
        .facet("species")
        .facet_config(&FacetConfig::new().cols(3))
        .plot_title(Text::from("Body Mass by Island, Sex and Species").size(16))
        .x_title(Text::from("Island"))
        .y_title(Text::from("Body Mass (g)"))
        .legend_title(Text::from("Sex"))
        .build()
        .plot();
}

fn contourplot_example() {
    let mut x_vals = Vec::new();
    let mut y_vals = Vec::new();
    let mut z_vals = Vec::new();
    let mut patterns = Vec::new();

    let grid_size = 25;

    // Pattern 1: Gaussian Peak
    for i in 0..grid_size {
        for j in 0..grid_size {
            let x = (i as f64 - 12.0) / 3.0;
            let y = (j as f64 - 12.0) / 3.0;
            let z = (-x * x - y * y).exp();
            x_vals.push(x);
            y_vals.push(y);
            z_vals.push(z);
            patterns.push("Gaussian");
        }
    }

    // Pattern 2: Saddle Point
    for i in 0..grid_size {
        for j in 0..grid_size {
            let x = (i as f64 - 12.0) / 3.0;
            let y = (j as f64 - 12.0) / 3.0;
            let z = x * x - y * y;
            x_vals.push(x);
            y_vals.push(y);
            z_vals.push(z);
            patterns.push("Saddle");
        }
    }

    // Pattern 3: Ripple Effect
    for i in 0..grid_size {
        for j in 0..grid_size {
            let x = (i as f64 - 12.0) / 3.0;
            let y = (j as f64 - 12.0) / 3.0;
            let r = (x * x + y * y).sqrt();
            let z = (r * 2.0).sin() / (r + 0.1);
            x_vals.push(x);
            y_vals.push(y);
            z_vals.push(z);
            patterns.push("Ripple");
        }
    }

    // Pattern 4: Paraboloid
    for i in 0..grid_size {
        for j in 0..grid_size {
            let x = (i as f64 - 12.0) / 3.0;
            let y = (j as f64 - 12.0) / 3.0;
            let z = x * x + y * y;
            x_vals.push(x);
            y_vals.push(y);
            z_vals.push(z);
            patterns.push("Paraboloid");
        }
    }

    // Pattern 5: Wave Interference
    for i in 0..grid_size {
        for j in 0..grid_size {
            let x = (i as f64 - 12.0) / 3.0;
            let y = (j as f64 - 12.0) / 3.0;
            let z = (x * 2.0).sin() * (y * 2.0).cos();
            x_vals.push(x);
            y_vals.push(y);
            z_vals.push(z);
            patterns.push("Wave");
        }
    }

    // Pattern 6: Diagonal Waves
    for i in 0..grid_size {
        for j in 0..grid_size {
            let x = (i as f64 - 12.0) / 3.0;
            let y = (j as f64 - 12.0) / 3.0;
            let z = ((x + y) * 2.0).sin();
            x_vals.push(x);
            y_vals.push(y);
            z_vals.push(z);
            patterns.push("Diagonal");
        }
    }

    let contour_data = df! {
        "x" => x_vals,
        "y" => y_vals,
        "z" => z_vals,
        "pattern" => patterns,
    }
    .unwrap();

    ContourPlot::builder()
        .data(&contour_data)
        .x("x")
        .y("y")
        .z("z")
        .facet("pattern")
        .facet_config(&FacetConfig::new().rows(2).cols(3))
        .plot_title(Text::from("Mathematical Surface Patterns").size(16))
        .x_title(Text::from("X Axis"))
        .y_title(Text::from("Y Axis"))
        .build()
        .plot();
}

fn heatmap_example() {
    let mut regions = Vec::new();
    let mut x_coords = Vec::new();
    let mut y_coords = Vec::new();
    let mut intensities = Vec::new();

    let region_names = ["North", "South", "East", "West"];
    let x_labels = ["X0", "X1", "X2", "X3", "X4"];
    let y_labels = ["Y0", "Y1", "Y2", "Y3", "Y4"];

    for (region_idx, region_name) in region_names.iter().enumerate() {
        for (y_idx, y_label) in y_labels.iter().enumerate() {
            for (x_idx, x_label) in x_labels.iter().enumerate() {
                regions.push(*region_name);
                x_coords.push(*x_label);
                y_coords.push(*y_label);

                let intensity = match region_idx {
                    0 => (x_idx + y_idx * 5) as f64 * 4.0,
                    1 => {
                        let dx = x_idx as f64 - 2.0;
                        let dy = y_idx as f64 - 2.0;
                        100.0 - (dx * dx + dy * dy) * 4.0
                    }
                    2 => ((x_idx * x_idx + y_idx * y_idx) as f64).sqrt() * 10.0,
                    3 => x_idx.max(y_idx) as f64 * 20.0,
                    _ => 0.0,
                };

                intensities.push(intensity);
            }
        }
    }

    let heatmap_data = df! {
        "region" => regions,
        "x" => x_coords,
        "y" => y_coords,
        "intensity" => intensities,
    }
    .unwrap();

    HeatMap::builder()
        .data(&heatmap_data)
        .x("x")
        .y("y")
        .z("intensity")
        .facet("region")
        .facet_config(&FacetConfig::new().rows(2).cols(2))
        .plot_title(Text::from("Regional Heat Intensity Patterns").size(16))
        .x_title(Text::from("X Coordinate"))
        .y_title(Text::from("Y Coordinate"))
        .build()
        .plot();
}

fn histogram_example() {
    let csv_path = "data/temperature_seasonal.csv";

    let df = CsvReadOptions::default()
        .with_has_header(true)
        .try_into_reader_with_file_path(Some(csv_path.into()))
        .unwrap()
        .finish()
        .unwrap();

    let facet_config = FacetConfig::new().rows(2).cols(3);

    Histogram::builder()
        .data(&df)
        .x("temperature")
        .group("season")
        .facet("city")
        .facet_config(&facet_config)
        .opacity(0.5)
        .plot_title("Seasonal Temperature Distribution by City")
        .x_title("Temperature (°C)")
        .y_title("Frequency")
        .build()
        .plot();
}

fn lineplot_example() {
    let dataset = create_lineplot_dataset();

    let facet_config = FacetConfig::new().highlight_facet(true);

    LinePlot::builder()
        .data(&dataset)
        .x("x")
        .y("sine")
        .facet("category")
        .facet_config(&facet_config)
        .plot_title(Text::from("Sine Wave Patterns by Amplitude Level"))
        .x_title("x")
        .y_title("sin(x)")
        .width(2.5)
        .with_shape(false)
        .color(Rgb(255, 69, 0))
        .build()
        .plot();
}

fn create_lineplot_dataset() -> DataFrame {
    let x_values: Vec<f64> = (0..200)
        .map(|i| {
            let step = (2.0 * std::f64::consts::PI - 0.0) / 199.0;
            0.0 + step * i as f64
        })
        .collect();

    let mut category = Vec::new();
    let mut amplitude = Vec::new();
    let mut sine_values = Vec::new();

    for cat in ["Low", "Medium", "High"].iter() {
        let amp = match *cat {
            "Low" => 0.5,
            "Medium" => 1.0,
            "High" => 1.5,
            _ => 1.0,
        };

        for &x in &x_values {
            category.push(cat.to_string());
            amplitude.push(amp);
            sine_values.push(amp * x.sin());
        }
    }

    let x_repeated: Vec<f64> = x_values
        .iter()
        .cycle()
        .take(x_values.len() * 3)
        .copied()
        .collect();

    df![
        "x" => &x_repeated,
        "category" => &category,
        "amplitude" => &amplitude,
        "sine" => &sine_values,
    ]
    .unwrap()
}

fn mesh3d_example() {
    let mut x_vals = Vec::new();
    let mut y_vals = Vec::new();
    let mut z_vals = Vec::new();
    let mut surface_type = Vec::new();

    let n = 25;

    for surface in ["Gaussian", "Saddle", "Ripple"].iter() {
        for i in 0..n {
            for j in 0..n {
                let x = (i as f64 / (n - 1) as f64) * 4.0 - 2.0;
                let y = (j as f64 / (n - 1) as f64) * 4.0 - 2.0;

                let z = match *surface {
                    "Gaussian" => (-0.5 * (x * x + y * y)).exp(),
                    "Saddle" => 0.3 * (x * x - y * y),
                    "Ripple" => 0.4 * ((x * 3.0).sin() + (y * 3.0).cos()),
                    _ => 0.0,
                };

                x_vals.push(x);
                y_vals.push(y);
                z_vals.push(z);
                surface_type.push(surface.to_string());
            }
        }
    }

    let dataset = DataFrame::new(vec![
        Column::new("x".into(), x_vals),
        Column::new("y".into(), y_vals),
        Column::new("z".into(), z_vals),
        Column::new("surface_type".into(), surface_type),
    ])
    .unwrap();

    let config = FacetConfig::new().cols(3).rows(1);

    let lighting = Lighting::new().ambient(0.6).diffuse(0.8).specular(0.4);

    Mesh3D::builder()
        .data(&dataset)
        .x("x")
        .y("y")
        .z("z")
        .facet("surface_type")
        .facet_config(&config)
        .color(Rgb(100, 150, 200))
        .lighting(&lighting)
        .plot_title(
            Text::from("Mathematical Surfaces Comparison")
                .font("Arial")
                .size(20),
        )
        .build()
        .plot();
}

fn piechart_example() {
    let dataset = CsvReadOptions::default()
        .with_has_header(true)
        .try_into_reader_with_file_path(Some("data/industry_region.csv".into()))
        .unwrap()
        .finish()
        .unwrap();

    let facet_config = FacetConfig::new()
        .cols(3)
        .scales(FacetScales::Free)
        .h_gap(0.08)
        .v_gap(0.12)
        .title_style(Text::from("").size(13).color(Rgb(60, 60, 60)));

    PieChart::builder()
        .data(&dataset)
        .labels("category")
        .facet("region")
        .facet_config(&facet_config)
        .colors(vec![
            Rgb(192, 57, 43),
            Rgb(39, 174, 96),
            Rgb(41, 128, 185),
            Rgb(243, 156, 18),
        ])
        .rotation(25.0)
        .pull(0.02)
        .plot_title(Text::from("Industry Analysis").size(18))
        .build()
        .plot();
}

fn sankeydiagram_example() {
    let dataset = CsvReadOptions::default()
        .with_has_header(true)
        .try_into_reader_with_file_path(Some("data/energy_transition.csv".into()))
        .unwrap()
        .finish()
        .unwrap();

    let facet_config = FacetConfig::new()
        .cols(4)
        .h_gap(0.06)
        .title_style(Text::from("").size(11).color(Rgb(50, 50, 50)));

    let node_colors = vec![
        Rgb(64, 64, 64),
        Rgb(100, 149, 237),
        Rgb(139, 69, 19),
        Rgb(255, 195, 0),
        Rgb(135, 206, 250),
        Rgb(65, 105, 225),
        Rgb(220, 20, 60),
        Rgb(34, 139, 34),
    ];

    let link_colors = vec![
        Rgb(220, 220, 220),
        Rgb(200, 220, 245),
        Rgb(220, 200, 180),
        Rgb(255, 240, 200),
        Rgb(220, 240, 255),
        Rgb(200, 220, 240),
    ];

    SankeyDiagram::builder()
        .data(&dataset)
        .sources("source")
        .targets("target")
        .values("value")
        .facet("year")
        .facet_config(&facet_config)
        .node_colors(node_colors)
        .link_colors(link_colors)
        .arrangement(Arrangement::Perpendicular)
        .plot_title(
            Text::from("Energy Transition Timeline (2020-2023)")
                .font("Arial")
                .size(16),
        )
        .pad(18)
        .thickness(22)
        .build()
        .plot();
}

fn scatterplot_example() {
    let dataset = LazyCsvReader::new(PlPath::new("data/penguins.csv"))
        .finish()
        .unwrap()
        .select([
            col("species"),
            col("sex").alias("gender"),
            col("bill_length_mm"),
            col("bill_depth_mm"),
        ])
        .collect()
        .unwrap();

    let facet_config = FacetConfig::new()
        .highlight_facet(true)
        .unhighlighted_color(Rgb(220, 220, 220));

    ScatterPlot::builder()
        .data(&dataset)
        .x("bill_length_mm")
        .y("bill_depth_mm")
        .group("gender")
        .facet("species")
        .facet_config(&facet_config)
        .plot_title(Text::from("Penguin Bill Morphology with Gender Comparison"))
        .x_title("bill length (mm)")
        .y_title("bill depth (mm)")
        .opacity(0.6)
        .size(8)
        .colors(vec![Rgb(128, 128, 128), Rgb(255, 0, 255), Rgb(0, 255, 255)])
        .shapes(vec![Shape::Diamond, Shape::Circle, Shape::Square])
        .legend_title("gender")
        .build()
        .plot();
}

fn scatter3d_example() {
    let dataset = LazyCsvReader::new(PlPath::new("data/penguins.csv"))
        .finish()
        .unwrap()
        .select([
            col("species"),
            col("sex").alias("gender"),
            col("bill_length_mm").cast(DataType::Float32),
            col("flipper_length_mm").cast(DataType::Int16),
            col("body_mass_g").cast(DataType::Int16),
        ])
        .collect()
        .unwrap();

    let facet_config = FacetConfig::new()
        .cols(3)
        .highlight_facet(true)
        .unhighlighted_color(Rgb(220, 220, 220));

    Scatter3dPlot::builder()
        .data(&dataset)
        .x("body_mass_g")
        .y("flipper_length_mm")
        .z("bill_length_mm")
        .facet("species")
        .facet_config(&facet_config)
        .opacity(0.6)
        .size(6)
        .colors(vec![Rgb(178, 34, 34), Rgb(65, 105, 225), Rgb(255, 140, 0)])
        .plot_title("Penguin Morphological Traits - 3D Faceted Analysis")
        .build()
        .plot();
}

fn scatterpolar_example() {
    let dataset = CsvReadOptions::default()
        .with_has_header(true)
        .try_into_reader_with_file_path(Some("data/wind_patterns.csv".into()))
        .unwrap()
        .finish()
        .unwrap();

    let facet_config = FacetConfig::new()
        .highlight_facet(true)
        .unhighlighted_color(Rgb(220, 220, 220))
        .cols(3);

    ScatterPolar::builder()
        .data(&dataset)
        .theta("angle")
        .r("speed")
        .group("time")
        .facet("season")
        .facet_config(&facet_config)
        .plot_title(Text::from("Wind Patterns by Season and Time of Day"))
        .mode(Mode::LinesMarkers)
        .opacity(0.7)
        .size(7)
        .width(2.5)
        .colors(vec![Rgb(255, 105, 180), Rgb(30, 144, 255)])
        .shapes(vec![Shape::Circle, Shape::Diamond])
        .lines(vec![Line::Solid, Line::DashDot])
        .legend_title("time of day")
        .build()
        .plot();
}

fn timeseriesplot_example() {
    let dataset = CsvReadOptions::default()
        .with_has_header(true)
        .try_into_reader_with_file_path(Some("data/financial_timeseries.csv".into()))
        .unwrap()
        .finish()
        .unwrap();

    let facet_config = FacetConfig::new()
        .highlight_facet(true)
        .unhighlighted_color(Rgb(220, 220, 220));

    TimeSeriesPlot::builder()
        .data(&dataset)
        .x("date")
        .y("revenue")
        .additional_series(vec!["costs"])
        .facet("region")
        .facet_config(&facet_config)
        .plot_title(Text::from("Regional Financial Metrics"))
        .x_title("Month")
        .y_title("Amount ($)")
        .legend_title("Metric")
        .width(2.0)
        .with_shape(false)
        .colors(vec![Rgb(255, 105, 180), Rgb(30, 144, 255)])
        .lines(vec![Line::Solid, Line::Dash])
        .build()
        .plot();
}

fn surfaceplot_example() {
    let n: usize = 50;
    let x_base: Vec<f64> = (0..n)
        .map(|i| {
            let step = (5.0 - (-5.0)) / (n - 1) as f64;
            -5.0 + step * i as f64
        })
        .collect();
    let y_base: Vec<f64> = (0..n)
        .map(|i| {
            let step = (5.0 - (-5.0)) / (n - 1) as f64;
            -5.0 + step * i as f64
        })
        .collect();

    let mut x_all = Vec::new();
    let mut y_all = Vec::new();
    let mut z_all = Vec::new();
    let mut category_all = Vec::new();

    type SurfaceFunction = Box<dyn Fn(f64, f64) -> f64>;
    let functions: Vec<(&str, SurfaceFunction)> = vec![
        (
            "Sine Wave",
            Box::new(|xi: f64, yj: f64| (xi * xi + yj * yj).sqrt().sin()),
        ),
        ("Saddle", Box::new(|xi: f64, yj: f64| xi * xi - yj * yj)),
        (
            "Gaussian",
            Box::new(|xi: f64, yj: f64| (-0.5 * (xi * xi + yj * yj)).exp()),
        ),
    ];

    for (name, func) in &functions {
        for &xi in x_base.iter() {
            for &yj in y_base.iter() {
                x_all.push(xi);
                y_all.push(yj);
                z_all.push(func(xi, yj));
                category_all.push(*name);
            }
        }
    }

    let dataset = df![
        "x" => &x_all,
        "y" => &y_all,
        "z" => &z_all,
        "function" => &category_all,
    ]
    .unwrap();

    SurfacePlot::builder()
        .data(&dataset)
        .x("x")
        .y("y")
        .z("z")
        .facet("function")
        .facet_config(&FacetConfig::new().cols(3).rows(1).h_gap(0.08).v_gap(0.12))
        .plot_title(
            Text::from("3D Mathematical Functions")
                .font("Arial")
                .size(20),
        )
        .color_scale(Palette::Viridis)
        .opacity(0.9)
        .build()
        .plot();
}
