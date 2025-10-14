use ndarray::Array;
use polars::prelude::*;
use plotlars::{FacetConfig, FacetScales, Line, LinePlot, Plot, Rgb, Text};

fn main() {
    let dataset = create_dataset();

    example_1_basic_faceting(&dataset);
    example_2_faceting_with_additional_lines(&dataset);
    example_3_custom_facet_config(&dataset);
    example_4_faceting_free_scales(&dataset);
    example_5_faceting_with_highlighting(&dataset);
    example_6_faceting_with_highlighting_and_lines(&dataset);
    example_7_faceting_with_colors(&dataset);
}

fn create_dataset() -> DataFrame {
    let x_values = Array::linspace(0.0, 2.0 * std::f64::consts::PI, 200).to_vec();

    let mut category = Vec::new();
    let mut amplitude = Vec::new();
    let mut sine_values = Vec::new();
    let mut cosine_values = Vec::new();

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
            cosine_values.push(amp * x.cos());
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
        "cosine" => &cosine_values,
    ]
    .unwrap()
}

fn example_1_basic_faceting(dataset: &DataFrame) {
    LinePlot::builder()
        .data(dataset)
        .x("x")
        .y("sine")
        .facet("category")
        .plot_title(Text::from("Example 1: Basic Faceting by Category"))
        .x_title("x")
        .y_title("sin(x)")
        .width(2.5)
        .with_shape(false)
        .color(Rgb(65, 105, 225))
        .build()
        .plot();
}

fn example_2_faceting_with_additional_lines(dataset: &DataFrame) {
    LinePlot::builder()
        .data(dataset)
        .x("x")
        .y("sine")
        .additional_lines(vec!["cosine"])
        .facet("category")
        .plot_title(Text::from("Example 2: Faceting with Multiple Lines"))
        .x_title("x")
        .y_title("value")
        .legend_title("function")
        .width(2.0)
        .with_shape(false)
        .colors(vec![Rgb(255, 69, 0), Rgb(50, 205, 50)])
        .lines(vec![Line::Solid, Line::Dash])
        .build()
        .plot();
}

fn example_3_custom_facet_config(dataset: &DataFrame) {
    let facet_config = FacetConfig::new()
        .ncol(3)
        .x_gap(0.08)
        .y_gap(0.12)
        .title_style(Text::from("").size(14).color(Rgb(50, 50, 50)));

    LinePlot::builder()
        .data(dataset)
        .x("x")
        .y("sine")
        .facet("category")
        .facet_config(&facet_config)
        .plot_title(Text::from("Example 3: Custom Facet Configuration"))
        .x_title("x")
        .y_title("sine(x)")
        .width(3.0)
        .with_shape(false)
        .color(Rgb(178, 34, 34))
        .build()
        .plot();
}

fn example_4_faceting_free_scales(dataset: &DataFrame) {
    let facet_config = FacetConfig::new()
        .scales(FacetScales::FreeY)
        .title_style(Text::from("").size(12));

    LinePlot::builder()
        .data(dataset)
        .x("x")
        .y("sine")
        .additional_lines(vec!["cosine"])
        .facet("category")
        .facet_config(&facet_config)
        .plot_title(Text::from("Example 4: Faceting with Free Y Scales"))
        .x_title("x")
        .y_title("value")
        .legend_title("function")
        .width(2.0)
        .with_shape(false)
        .colors(vec![Rgb(178, 34, 34), Rgb(60, 179, 113)])
        .build()
        .plot();
}

fn example_5_faceting_with_highlighting(dataset: &DataFrame) {
    let facet_config = FacetConfig::new().highlight_facet(true);

    LinePlot::builder()
        .data(dataset)
        .x("x")
        .y("sine")
        .facet("category")
        .facet_config(&facet_config)
        .plot_title(Text::from("Example 5: Faceting with Highlighting (Default Grey)"))
        .x_title("x")
        .y_title("sin(x)")
        .width(2.5)
        .with_shape(false)
        .color(Rgb(255, 69, 0))
        .build()
        .plot();
}

fn example_6_faceting_with_highlighting_and_lines(dataset: &DataFrame) {
    let facet_config = FacetConfig::new()
        .highlight_facet(true)
        .unhighlighted_color(Rgb(220, 220, 220));

    LinePlot::builder()
        .data(dataset)
        .x("x")
        .y("sine")
        .additional_lines(vec!["cosine"])
        .facet("category")
        .facet_config(&facet_config)
        .plot_title(Text::from(
            "Example 6: Faceting with Highlighting and Multiple Lines",
        ))
        .x_title("x")
        .y_title("value")
        .legend_title("function")
        .width(2.0)
        .with_shape(false)
        .colors(vec![Rgb(255, 105, 180), Rgb(30, 144, 255)])
        .lines(vec![Line::Solid, Line::Dash])
        .build()
        .plot();
}

fn example_7_faceting_with_colors(dataset: &DataFrame) {
    LinePlot::builder()
        .data(dataset)
        .x("x")
        .y("sine")
        .facet("category")
        .plot_title(Text::from(
            "Example 7: Faceting with Colors (Scenario 2: colors + facet)",
        ))
        .x_title("x")
        .y_title("sin(x)")
        .width(2.5)
        .with_shape(false)
        .colors(vec![Rgb(255, 99, 71), Rgb(50, 205, 50), Rgb(30, 144, 255)])
        .build()
        .plot();
}
