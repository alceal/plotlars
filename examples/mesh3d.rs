use plotlars::{ColorBar, IntensityMode, Lighting, Mesh3D, Palette, Plot, Rgb, Text};
use polars::prelude::*;

fn main() {
    example_basic_mesh();
    example_with_indices();
    example_with_intensity();
    example_with_lighting();
}

fn example_basic_mesh() {
    let x = vec![0.0, 1.0, 2.0, 0.0, 1.0, 2.0];
    let y = vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0];
    let z = vec![0.0, 0.5, 0.0, 0.0, 0.8, 0.0];

    let dataset = DataFrame::new(vec![
        Column::new("x".into(), x),
        Column::new("y".into(), y),
        Column::new("z".into(), z),
    ])
    .unwrap();

    Mesh3D::builder()
        .data(&dataset)
        .x("x")
        .y("y")
        .z("z")
        .color(Rgb(100, 150, 200))
        .opacity(0.8)
        .plot_title("Basic Mesh3D")
        .build()
        .plot();
}

fn example_with_indices() {
    let x = vec![0.0, 1.0, 0.5, 0.5];
    let y = vec![0.0, 0.0, 0.866, 0.289];
    let z = vec![0.0, 0.0, 0.0, 0.816];
    let i = vec![0, 0, 0, 1];
    let j = vec![1, 2, 3, 2];
    let k = vec![2, 3, 1, 3];

    let dataset = DataFrame::new(vec![
        Column::new("x".into(), x),
        Column::new("y".into(), y),
        Column::new("z".into(), z),
        Column::new("i".into(), i),
        Column::new("j".into(), j),
        Column::new("k".into(), k),
    ])
    .unwrap();

    Mesh3D::builder()
        .data(&dataset)
        .x("x")
        .y("y")
        .z("z")
        .i("i")
        .j("j")
        .k("k")
        .color(Rgb(255, 100, 100))
        .opacity(0.9)
        .flat_shading(true)
        .plot_title("Tetrahedron with Explicit Indices")
        .build()
        .plot();
}

fn example_with_intensity() {
    let mut x = Vec::new();
    let mut y = Vec::new();
    let mut z = Vec::new();
    let mut intensity = Vec::new();

    for i in 0..10 {
        for j in 0..10 {
            let xi = i as f64 * 0.1;
            let yj = j as f64 * 0.1;
            x.push(xi);
            y.push(yj);
            z.push(
                (xi * 2.0 * std::f64::consts::PI).sin()
                    * (yj * 2.0 * std::f64::consts::PI).cos()
                    * 0.3,
            );
            intensity.push(xi * yj);
        }
    }

    let dataset = DataFrame::new(vec![
        Column::new("x".into(), x),
        Column::new("y".into(), y),
        Column::new("z".into(), z),
        Column::new("intensity".into(), intensity),
    ])
    .unwrap();

    Mesh3D::builder()
        .data(&dataset)
        .x("x")
        .y("y")
        .z("z")
        .intensity("intensity")
        .intensity_mode(IntensityMode::Vertex)
        .color_scale(Palette::Viridis)
        .reverse_scale(false)
        .show_scale(true)
        .color_bar(
            &ColorBar::new()
                .x(0.85)  // Move color bar very close to the plot
                .title("Intensity")
        )
        .opacity(0.95)
        .plot_title(
            Text::from("Mesh3D with Intensity Coloring")
                .font("Arial")
                .size(20),
        )
        .build()
        .plot();
}

fn example_with_lighting() {
    // Create a simple wavy surface mesh without explicit indices
    // The mesh will be auto-triangulated
    let mut x = Vec::new();
    let mut y = Vec::new();
    let mut z = Vec::new();

    let n = 20;
    for i in 0..n {
        for j in 0..n {
            let xi = (i as f64 / (n - 1) as f64) * 2.0 - 1.0;
            let yj = (j as f64 / (n - 1) as f64) * 2.0 - 1.0;
            x.push(xi);
            y.push(yj);
            // Create a wavy surface
            z.push(0.3 * ((xi * 3.0).sin() + (yj * 3.0).cos()));
        }
    }

    let dataset = DataFrame::new(vec![
        Column::new("x".into(), x),
        Column::new("y".into(), y),
        Column::new("z".into(), z),
    ])
    .unwrap();

    Mesh3D::builder()
        .data(&dataset)
        .x("x")
        .y("y")
        .z("z")
        .color(Rgb(200, 200, 255))
        .lighting(
            &Lighting::new()
                .ambient(0.5)
                .diffuse(0.8)
                .specular(0.5)
                .roughness(0.2)
                .fresnel(0.2),
        )
        .light_position((1, 1, 2))
        .opacity(1.0)
        .flat_shading(false)
        .contour(true)
        .plot_title(
            Text::from("Wavy Surface with Custom Lighting")
                .font("Arial")
                .size(22),
        )
        .build()
        .plot();
}
