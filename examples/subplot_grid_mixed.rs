use plotlars::{
    Arrangement, Mode, Orientation, Plot, Rgb, SankeyDiagram, Scatter3dPlot, ScatterGeo,
    ScatterMap, ScatterPolar, ScatterPlot, Shape, SubplotGrid, Text,
};
use polars::prelude::*;

fn main() {
    // 2D cartesian scatter (baseline)
    let penguins = LazyCsvReader::new(PlPath::new("data/penguins.csv"))
        .finish()
        .unwrap()
        .collect()
        .unwrap()
        .lazy()
        .select([
            col("species"),
            col("bill_length_mm"),
            col("flipper_length_mm"),
            col("body_mass_g"),
        ])
        .collect()
        .unwrap();

    let scatter_2d = ScatterPlot::builder()
        .data(&penguins)
        .x("bill_length_mm")
        .y("flipper_length_mm")
        .group("species")
        .opacity(0.65)
        .size(10)
        .plot_title(Text::from("Penguins 2D"))
        .build();

    // 3D scene subplot
    let scatter_3d = Scatter3dPlot::builder()
        .data(&penguins)
        .x("bill_length_mm")
        .y("flipper_length_mm")
        .z("body_mass_g")
        .group("species")
        .opacity(0.35)
        .size(6)
        .plot_title(Text::from("Penguins 3D"))
        .build();

    // Polar subplot
    let polar_df = LazyCsvReader::new(PlPath::new("data/product_comparison_polar.csv"))
        .finish()
        .unwrap()
        .collect()
        .unwrap();

    let polar = ScatterPolar::builder()
        .data(&polar_df)
        .theta("angle")
        .r("score")
        .group("product")
        .mode(Mode::LinesMarkers)
        .size(10)
        .plot_title(Text::from("Product Comparison (Polar)"))
        .build();

    // Domain-based subplot (Sankey)
    let sankey_df = LazyCsvReader::new(PlPath::new("data/energy_transition.csv"))
        .finish()
        .unwrap()
        .collect()
        .unwrap();

    let sankey = SankeyDiagram::builder()
        .data(&sankey_df)
        .sources("source")
        .targets("target")
        .values("value")
        .orientation(Orientation::Horizontal)
        .arrangement(Arrangement::Freeform)
        .plot_title(Text::from("Energy Flow"))
        .build();

    // Mapbox subplot
    let map_df = LazyCsvReader::new(PlPath::new("data/cities.csv"))
        .finish()
        .unwrap()
        .collect()
        .unwrap();

    let scatter_map = ScatterMap::builder()
        .data(&map_df)
        .latitude("latitude")
        .longitude("longitude")
        .group("city")
        .zoom(4)
        .center([50.0, 5.0])
        .opacity(0.8)
        .plot_title(Text::from("Cities (Mapbox)"))
        .build();

    // Geo subplot
    let geo_df = LazyCsvReader::new(PlPath::new("data/world_cities.csv"))
        .finish()
        .unwrap()
        .collect()
        .unwrap();

    let scatter_geo = ScatterGeo::builder()
        .data(&geo_df)
        .lat("lat")
        .lon("lon")
        .group("continent")
        .mode(Mode::Markers)
        .size(10)
        .color(Rgb(255, 140, 0))
        .shape(Shape::Circle)
        .plot_title(Text::from("Global Cities (Geo)"))
        .build();

    SubplotGrid::regular()
        .plots(vec![
            &scatter_2d,
            &scatter_3d,
            &polar,
            &sankey,
            &scatter_map,
            &scatter_geo,
        ])
        .rows(2)
        .cols(3)
        .h_gap(0.12)
        .v_gap(0.22)
        .title(Text::from("Mixed Subplot Grid").size(18))
        .build()
        .plot();
}
