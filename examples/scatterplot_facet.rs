use polars::prelude::*;

use plotlars::{FacetConfig, FacetScales, Plot, Rgb, ScatterPlot, Shape, Text};

fn main() {
    let dataset = LazyCsvReader::new(PlPath::new("data/penguins.csv"))
        .finish()
        .unwrap()
        .select([
            col("species"),
            col("sex").alias("gender"),
            col("bill_length_mm"),
            col("bill_depth_mm"),
            col("flipper_length_mm").cast(DataType::Int16),
            col("body_mass_g").cast(DataType::Int16),
        ])
        .collect()
        .unwrap();

    example_1_basic_faceting(&dataset);
    example_2_faceting_with_grouping(&dataset);
    example_3_faceting_with_custom_config(&dataset);
    example_4_faceting_free_scales(&dataset);
    example_5_faceting_with_highlighting(&dataset);
    example_6_faceting_with_highlighting_and_grouping(&dataset);
}

fn example_1_basic_faceting(dataset: &DataFrame) {
    ScatterPlot::builder()
        .data(dataset)
        .x("body_mass_g")
        .y("flipper_length_mm")
        .facet("species")
        .plot_title(Text::from("Example 1: Basic Faceting by Species"))
        .x_title("body mass (g)")
        .y_title("flipper length (mm)")
        .opacity(0.6)
        .size(8)
        .color(Rgb(65, 105, 225))
        .build()
        .plot();
}

fn example_2_faceting_with_grouping(dataset: &DataFrame) {
    ScatterPlot::builder()
        .data(dataset)
        .x("body_mass_g")
        .y("flipper_length_mm")
        .group("gender")
        .facet("species")
        .plot_title(Text::from(
            "Example 2: Faceting by Species with Gender Groups",
        ))
        .x_title("body mass (g)")
        .y_title("flipper length (mm)")
        .opacity(0.6)
        .size(8)
        .colors(vec![Rgb(255, 105, 180), Rgb(30, 144, 255)])
        .shapes(vec![Shape::Circle, Shape::Square])
        .legend_title("gender")
        .build()
        .plot();
}

fn example_3_faceting_with_custom_config(dataset: &DataFrame) {
    let facet_config = FacetConfig::new()
        .ncol(3)
        .x_gap(0.08)
        .y_gap(0.12)
        .title_style(Text::from("").size(14).color(Rgb(50, 50, 50)));

    ScatterPlot::builder()
        .data(dataset)
        .x("bill_length_mm")
        .y("bill_depth_mm")
        .facet("species")
        .facet_config(&facet_config)
        .plot_title(Text::from("Example 3: Custom Facet Configuration"))
        .x_title("bill length (mm)")
        .y_title("bill depth (mm)")
        .opacity(0.7)
        .size(10)
        .color(Rgb(178, 34, 34))
        .build()
        .plot();
}

fn example_4_faceting_free_scales(dataset: &DataFrame) {
    let facet_config = FacetConfig::new()
        .scales(FacetScales::Free)
        .title_style(Text::from("").size(12));

    ScatterPlot::builder()
        .data(dataset)
        .x("bill_length_mm")
        .y("bill_depth_mm")
        .group("gender")
        .facet("species")
        .facet_config(&facet_config)
        .plot_title(Text::from("Example 4: Faceting with Free Scales"))
        .x_title("bill length (mm)")
        .y_title("bill depth (mm)")
        .opacity(0.6)
        .size(8)
        .colors(vec![Rgb(255, 140, 0), Rgb(60, 179, 113)])
        .legend_title("gender")
        .build()
        .plot();
}

fn example_5_faceting_with_highlighting(dataset: &DataFrame) {
    let facet_config = FacetConfig::new().highlight_facet(true);

    ScatterPlot::builder()
        .data(dataset)
        .x("bill_length_mm")
        .y("bill_depth_mm")
        .facet("species")
        .facet_config(&facet_config)
        .plot_title(Text::from(
            "Example 5: Faceting with Highlighting (Default Grey)",
        ))
        .x_title("bill length (mm)")
        .y_title("bill depth (mm)")
        .opacity(0.6)
        .size(8)
        .color(Rgb(65, 105, 225))
        .build()
        .plot();
}

fn example_6_faceting_with_highlighting_and_grouping(dataset: &DataFrame) {
    let facet_config = FacetConfig::new()
        .highlight_facet(true)
        .unhighlighted_color(Rgb(220, 220, 220));

    ScatterPlot::builder()
        .data(dataset)
        .x("bill_length_mm")
        .y("bill_depth_mm")
        .group("gender")
        .facet("species")
        .facet_config(&facet_config)
        .plot_title(Text::from(
            "Example 6: Faceting with Highlighting and Grouping",
        ))
        .x_title("bill length (mm)")
        .y_title("bill depth (mm)")
        .opacity(0.6)
        .size(8)
        .colors(vec![Rgb(255, 105, 180), Rgb(30, 144, 255)])
        .shapes(vec![Shape::Circle, Shape::Square])
        .legend_title("gender")
        .build()
        .plot();
}
