use plotlars::{Coloring, ContourPlot, CsvReader, Palette, Plot, Text};

fn main() {
    let dataset = CsvReader::new("data/contour_surface.csv").finish().unwrap();

    ContourPlot::builder()
        .data(&dataset)
        .x("x")
        .y("y")
        .z("z")
        .color_scale(Palette::Viridis)
        .reverse_scale(true)
        .coloring(Coloring::Fill)
        .show_lines(false)
        .plot_title(Text::from("Contour Plot").font("Arial").size(18))
        .build()
        .plot();
}
