use plotlars::{CsvReader, DensityMapbox, Plot, Text};

fn main() {
    let data = CsvReader::new("data/us_city_density.csv").finish().unwrap();

    DensityMapbox::builder()
        .data(&data)
        .lat("city_lat")
        .lon("city_lon")
        .z("population_density")
        .center([39.8283, -98.5795])
        .zoom(3)
        .plot_title(Text::from("Density Mapbox").font("Arial").size(20))
        .build()
        .plot();
}
