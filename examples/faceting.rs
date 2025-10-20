use ndarray::Array;
use plotlars::{
    Arrangement, BarPlot, BoxPlot, ContourPlot, FacetConfig, FacetScales, HeatMap, Histogram, Line,
    LinePlot, Mode, PieChart, Plot, Rgb, SankeyDiagram, ScatterPlot, ScatterPolar, Shape, Text,
    TimeSeriesPlot,
};
use polars::prelude::*;

fn main() {
    barplot_example();
    boxplot_example();
    contourplot_example();
    heatmap_example();
    histogram_example();
    lineplot_example();
    piechart_example();
    sankeydiagram_example();
    scatterplot_example();
    scatterpolar_example();
    timeseriesplot_example();
}

fn barplot_example() {
    let regional_data = df![
        "region" => &["North", "North", "North", "South", "South", "South",
                     "East", "East", "East", "West", "West", "West",
                     "Southwest", "Southwest", "Southwest", "Northeast", "Northeast", "Northeast",
                     "Southeast", "Southeast", "Southeast", "Northwest", "Northwest", "Northwest"],
        "product" => &["A", "B", "C", "A", "B", "C",
                      "A", "B", "C", "A", "B", "C",
                      "A", "B", "C", "A", "B", "C",
                      "A", "B", "C", "A", "B", "C"],
        "sales" => &[180.0f32, 250.0, 210.0, 55.0, 85.0, 65.0,
                    140.0, 175.0, 160.0, 35.0, 60.0, 48.0,
                    95.0, 115.0, 105.0, 230.0, 280.0, 255.0,
                    70.0, 95.0, 80.0, 45.0, 195.0, 120.0],
    ]
    .unwrap();

    let facet_config = FacetConfig::new().ncol(4).nrow(2).x_gap(0.05).y_gap(0.30);

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
        .facet_config(&FacetConfig::new().ncol(3))
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
        .facet_config(&FacetConfig::new().nrow(2).ncol(3))
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
        .facet_config(&FacetConfig::new().nrow(2).ncol(2))
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

    let facet_config = FacetConfig::new().nrow(2).ncol(3);

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
    let x_values = Array::linspace(0.0, 2.0 * std::f64::consts::PI, 200).to_vec();

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

fn piechart_example() {
    let dataset = df![
        "category" => vec![
            "Tech", "Tech", "Tech", "Finance", "Finance", "Healthcare", "Retail",
            "Tech", "Finance", "Finance", "Finance", "Healthcare", "Healthcare", "Retail",
            "Tech", "Finance", "Healthcare", "Healthcare", "Healthcare", "Retail", "Retail", "Retail",
        ],
        "region" => vec![
            "North", "North", "North", "North", "North", "North", "North",
            "South", "South", "South", "South", "South", "South", "South",
            "West", "West", "West", "West", "West", "West", "West", "West",
        ],
    ]
    .unwrap();

    let facet_config = FacetConfig::new()
        .ncol(3)
        .scales(FacetScales::Free)
        .x_gap(0.08)
        .y_gap(0.12)
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
    let dataset = df![
        "year" => [
            "2020", "2020", "2020", "2020", "2020", "2020",
            "2021", "2021", "2021", "2021", "2021", "2021",
            "2022", "2022", "2022", "2022", "2022", "2022",
            "2023", "2023", "2023", "2023", "2023", "2023"
        ],
        "source" => [
            "Coal", "Natural Gas", "Oil", "Solar", "Wind", "Hydro",
            "Coal", "Natural Gas", "Oil", "Solar", "Wind", "Hydro",
            "Coal", "Natural Gas", "Oil", "Solar", "Wind", "Hydro",
            "Coal", "Natural Gas", "Oil", "Solar", "Wind", "Hydro"
        ],
        "target" => [
            "Fossil Energy", "Fossil Energy", "Fossil Energy", "Renewable Energy", "Renewable Energy", "Renewable Energy",
            "Fossil Energy", "Fossil Energy", "Fossil Energy", "Renewable Energy", "Renewable Energy", "Renewable Energy",
            "Fossil Energy", "Fossil Energy", "Fossil Energy", "Renewable Energy", "Renewable Energy", "Renewable Energy",
            "Fossil Energy", "Fossil Energy", "Fossil Energy", "Renewable Energy", "Renewable Energy", "Renewable Energy"
        ],
        "value" => [
            45, 55, 30, 25, 30, 35,
            40, 50, 25, 30, 35, 40,
            35, 45, 20, 35, 40, 45,
            30, 40, 15, 40, 45, 50
        ],
    ]
    .unwrap();

    let facet_config = FacetConfig::new()
        .ncol(4)
        .x_gap(0.06)
        .title_style(Text::from("").size(11).color(Rgb(50, 50, 50)));

    let node_colors = vec![
        Rgb(64, 64, 64),    // Coal - Dark Gray
        Rgb(100, 149, 237), // Natural Gas - Cornflower Blue
        Rgb(139, 69, 19),   // Oil - Saddle Brown
        Rgb(255, 195, 0),   // Solar - Yellow
        Rgb(135, 206, 250), // Wind - Sky Blue
        Rgb(65, 105, 225),  // Hydro - Royal Blue
        Rgb(220, 20, 60),   // Fossil Energy - Crimson
        Rgb(34, 139, 34),   // Renewable Energy - Forest Green
    ];

    let link_colors = vec![
        Rgb(220, 220, 220), // Coal links - Light Gray
        Rgb(200, 220, 245), // Gas links - Light Blue
        Rgb(220, 200, 180), // Oil links - Light Brown
        Rgb(255, 240, 200), // Solar links - Light Yellow
        Rgb(220, 240, 255), // Wind links - Very Light Blue
        Rgb(200, 220, 240), // Hydro links - Light Royal Blue
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

fn scatterpolar_example() {
    let dataset = create_scatterpolar_wind_data();

    let facet_config = FacetConfig::new()
        .highlight_facet(true)
        .unhighlighted_color(Rgb(220, 220, 220))
        .ncol(3);

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
    let dataset = create_timeseriesplot_dataset();

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

fn create_timeseriesplot_dataset() -> DataFrame {
    let months = [
        "2023-01", "2023-02", "2023-03", "2023-04", "2023-05", "2023-06", "2023-07", "2023-08",
        "2023-09", "2023-10", "2023-11", "2023-12",
    ];

    let mut date = Vec::new();
    let mut region = Vec::new();
    let mut revenue = Vec::new();
    let mut costs = Vec::new();

    for reg in ["North", "South", "West", "East"].iter() {
        let (base_revenue, base_cost, growth) = match *reg {
            "North" => (100000.0_f64, 60000.0_f64, 1.08_f64),
            "South" => (150000.0_f64, 90000.0_f64, 1.05_f64),
            "West" => (120000.0_f64, 70000.0_f64, 1.10_f64),
            "East" => (130000.0_f64, 75000.0_f64, 1.06_f64),
            _ => (100000.0_f64, 60000.0_f64, 1.05_f64),
        };

        for (i, month) in months.iter().enumerate() {
            date.push(month.to_string());
            region.push(reg.to_string());

            let month_revenue = base_revenue * growth.powi(i as i32);
            let month_cost = base_cost * 1.03_f64.powi(i as i32);

            revenue.push(month_revenue);
            costs.push(month_cost);
        }
    }

    df![
        "date" => &date,
        "region" => &region,
        "revenue" => &revenue,
        "costs" => &costs,
    ]
    .unwrap()
}

fn create_scatterpolar_wind_data() -> DataFrame {
    let mut angles = Vec::new();
    let mut speeds = Vec::new();
    let mut seasons = Vec::new();
    let mut times = Vec::new();

    let season_list = ["Spring", "Summer", "Fall"];
    let time_list = ["Morning", "Evening"];

    for season in &season_list {
        for time in &time_list {
            for angle in (0..=360).step_by(30) {
                let base_speed = match *season {
                    "Spring" => 15.0,
                    "Summer" => 10.0,
                    "Fall" => 20.0,
                    _ => 15.0,
                };

                let time_modifier = match *time {
                    "Morning" => 0.8,
                    "Evening" => 1.2,
                    _ => 1.0,
                };

                let angle_rad = (angle as f64).to_radians();
                let variation = (angle_rad * 2.0).sin() * 5.0;
                let speed = base_speed * time_modifier + variation;

                angles.push(angle as f64);
                speeds.push(speed.abs());
                seasons.push(*season);
                times.push(*time);
            }
        }
    }

    df![
        "angle" => angles,
        "speed" => speeds,
        "season" => seasons,
        "time" => times,
    ]
    .unwrap()
}
