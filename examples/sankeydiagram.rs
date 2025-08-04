use polars::prelude::*;

use plotlars::{Arrangement, Orientation, Plot, Rgb, SankeyDiagram, Text};

fn main() {
    let dataset = df![
        "source" => ["A1", "A2", "A1", "B1", "B2", "B2"],
        "target" => &["B1", "B2", "B2", "C1", "C1", "C2"],
        "value" => &[8, 4, 2, 8, 4, 2],
    ]
    .unwrap();

    SankeyDiagram::builder()
        .data(&dataset)
        .sources("source")
        .targets("target")
        .values("value")
        .orientation(Orientation::Horizontal)
        .arrangement(Arrangement::Freeform)
        .node_colors(vec![
            Rgb(222, 235, 247),
            Rgb(198, 219, 239),
            Rgb(158, 202, 225),
            Rgb(107, 174, 214),
            Rgb(66, 146, 198),
            Rgb(33, 113, 181),
        ])
        .link_colors(vec![
            Rgb(222, 235, 247),
            Rgb(198, 219, 239),
            Rgb(158, 202, 225),
            Rgb(107, 174, 214),
            Rgb(66, 146, 198),
            Rgb(33, 113, 181),
        ])
        .pad(20)
        .thickness(30)
        .plot_title(Text::from("Sankey Diagram").font("Arial").size(18))
        .build()
        .plot();
}
