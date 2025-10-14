use plotlars::*;
use polars::prelude::*;

fn main() {
    // Create a sample dataset with multiple categories
    let dataset = df![
        "species" => ["Adelie", "Adelie", "Adelie", "Adelie", "Adelie", "Adelie",
                      "Chinstrap", "Chinstrap", "Chinstrap", "Chinstrap", "Chinstrap", "Chinstrap",
                      "Gentoo", "Gentoo", "Gentoo", "Gentoo", "Gentoo", "Gentoo"],
        "island" => ["Torgersen", "Torgersen", "Torgersen", "Biscoe", "Biscoe", "Biscoe",
                     "Dream", "Dream", "Dream", "Dream", "Dream", "Dream",
                     "Biscoe", "Biscoe", "Biscoe", "Biscoe", "Biscoe", "Biscoe"],
        "sex" => ["Male", "Female", "Male", "Female", "Male", "Female",
                  "Male", "Female", "Male", "Female", "Male", "Female",
                  "Male", "Female", "Male", "Female", "Male", "Female"],
        "body_mass_g" => [3750, 3800, 3850, 3900, 4000, 3950,
                          3500, 3550, 3600, 3650, 3700, 3750,
                          5000, 5050, 5100, 5150, 5200, 5250],
    ]
    .unwrap();

    // Example 1: Basic faceting by species
    BoxPlot::builder()
        .data(&dataset)
        .labels("island")
        .values("body_mass_g")
        .facet("species")
        .plot_title("Body Mass by Island (Faceted by Species)")
        .x_title("Island")
        .y_title("Body Mass (g)")
        .build()
        .plot();

    // Example 2: Faceting with grouping (sex within each species facet)
    BoxPlot::builder()
        .data(&dataset)
        .labels("island")
        .values("body_mass_g")
        .group("sex")
        .facet("species")
        .colors(vec![Rgb(100, 150, 200), Rgb(200, 150, 100)])
        .plot_title("Body Mass by Island and Sex (Faceted by Species)")
        .x_title("Island")
        .y_title("Body Mass (g)")
        .legend_title("Sex")
        .build()
        .plot();

    // Example 3: Custom FacetConfig
    let facet_config = FacetConfig::new()
        .ncol(3)
        .x_gap(0.05)
        .y_gap(0.08)
        .title_style(Text::from("").size(14).color(Rgb(50, 50, 50)));

    BoxPlot::builder()
        .data(&dataset)
        .labels("sex")
        .values("body_mass_g")
        .facet("species")
        .facet_config(&facet_config)
        .plot_title("Body Mass by Sex (Faceted by Species)")
        .x_title("Sex")
        .y_title("Body Mass (g)")
        .opacity(0.8)
        .color(Rgb(100, 150, 200))
        .build()
        .plot();

    // Example 4: Free scales
    let facet_config_free = FacetConfig::new()
        .ncol(2)
        .scales(FacetScales::FreeY);

    BoxPlot::builder()
        .data(&dataset)
        .labels("island")
        .values("body_mass_g")
        .facet("species")
        .facet_config(&facet_config_free)
        .plot_title("Body Mass by Island (Free Y Scales)")
        .x_title("Island")
        .y_title("Body Mass (g)")
        .build()
        .plot();

    // Example 5: Per-facet colors (Scenario 2)
    BoxPlot::builder()
        .data(&dataset)
        .labels("sex")
        .values("body_mass_g")
        .facet("species")
        .colors(vec![
            Rgb(178, 34, 34),   // Adelie
            Rgb(65, 105, 225),  // Chinstrap
            Rgb(255, 140, 0),   // Gentoo
        ])
        .plot_title("Body Mass by Sex (Per-facet Colors)")
        .x_title("Sex")
        .y_title("Body Mass (g)")
        .build()
        .plot();

    // Example 6: Box points with faceting
    BoxPlot::builder()
        .data(&dataset)
        .labels("island")
        .values("body_mass_g")
        .facet("species")
        .box_points(true)
        .jitter(0.1)
        .point_offset(-0.3)
        .opacity(0.3)
        .plot_title("Body Mass Distribution with Points")
        .x_title("Island")
        .y_title("Body Mass (g)")
        .build()
        .plot();

    // Example 7: Horizontal orientation with faceting
    BoxPlot::builder()
        .data(&dataset)
        .labels("sex")
        .values("body_mass_g")
        .orientation(Orientation::Horizontal)
        .facet("species")
        .plot_title("Horizontal Box Plot (Faceted)")
        .x_title("Body Mass (g)")
        .y_title("Sex")
        .build()
        .plot();
}