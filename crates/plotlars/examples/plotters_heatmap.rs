use plotlars::{CsvReader, HeatMap, Palette, Plot};

fn main() {
    let dataset = CsvReader::new("data/heatmap.csv").finish().unwrap();

    HeatMap::builder()
        .data(&dataset)
        .x("x")
        .y("y")
        .z("z")
        .color_scale(Palette::Viridis)
        .plot_title("Heat Map")
        .x_title("X")
        .y_title("Y")
        .build()
        .plot();
}
