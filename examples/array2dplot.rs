use plotlars::{Array2dPlot, Plot, Text};

fn main() {
    let data = vec![
        vec![[255, 0, 0], [0, 255, 0], [0, 0, 255]],
        vec![[0, 0, 255], [255, 0, 0], [0, 255, 0]],
        vec![[0, 255, 0], [0, 0, 255], [255, 0, 0]],
    ];

    Array2dPlot::builder()
        .data(&data)
        .plot_title(Text::from("Array 2D Plot").font("Arial").size(18))
        .build()
        .plot();
}
