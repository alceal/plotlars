use plotlars_core::Plot;

/// Plotters rendering extension trait. Provides static image output methods.
///
/// Available on all types implementing the core `Plot` trait via blanket impl.
pub trait PlottersExt: Plot {
    /// Render and display the plot.
    ///
    /// In Jupyter/evcxr: displays inline PNG.
    /// Otherwise: writes a temp PNG and opens the OS image viewer.
    fn plot(&self);

    /// Save the plot to a file. Format inferred from extension:
    /// - `.png` -> BitMapBackend
    /// - `.svg` -> SVGBackend
    fn save(&self, path: &str);

    /// Render the plot to an SVG string (in-memory, no file I/O).
    fn to_svg(&self) -> String;
}

impl<T: Plot> PlottersExt for T {
    fn plot(&self) {
        crate::orchestrator::plot_interactive(self);
    }

    fn save(&self, path: &str) {
        crate::orchestrator::save_to_file(self, path);
    }

    fn to_svg(&self) -> String {
        crate::orchestrator::render_to_svg_string(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use plotlars_core::components::Rgb;
    use plotlars_core::plots::barplot::BarPlot;
    use plotlars_core::plots::histogram::Histogram;
    use plotlars_core::plots::lineplot::LinePlot;
    use plotlars_core::plots::scatterplot::ScatterPlot;
    use polars::prelude::*;

    #[test]
    fn scatter_plot_renders_to_svg() {
        let df = df![
            "x" => [1.0, 2.0, 3.0, 4.0],
            "y" => [10.0, 20.0, 15.0, 25.0]
        ]
        .unwrap();
        let plot = ScatterPlot::builder().data(&df).x("x").y("y").build();
        let svg = plot.to_svg();
        assert!(!svg.is_empty());
        assert!(svg.contains("<svg"));
    }

    #[test]
    fn scatter_plot_grouped_renders() {
        let df = df![
            "x" => [1.0, 2.0, 3.0, 4.0],
            "y" => [10.0, 20.0, 15.0, 25.0],
            "g" => ["a", "a", "b", "b"]
        ]
        .unwrap();
        let plot = ScatterPlot::builder()
            .data(&df)
            .x("x")
            .y("y")
            .group("g")
            .build();
        let svg = plot.to_svg();
        assert!(!svg.is_empty());
        assert!(svg.contains("<svg"));
    }

    #[test]
    fn scatter_plot_styled_renders() {
        let df = df![
            "x" => [1.0, 2.0, 3.0],
            "y" => [4.0, 5.0, 6.0]
        ]
        .unwrap();
        let plot = ScatterPlot::builder()
            .data(&df)
            .x("x")
            .y("y")
            .color(Rgb(255, 0, 0))
            .opacity(0.7)
            .size(10)
            .build();
        let svg = plot.to_svg();
        assert!(!svg.is_empty());
    }

    #[test]
    fn scatter_plot_with_title_renders() {
        let df = df![
            "x" => [1.0, 2.0, 3.0],
            "y" => [4.0, 5.0, 6.0]
        ]
        .unwrap();
        let plot = ScatterPlot::builder()
            .data(&df)
            .x("x")
            .y("y")
            .plot_title("My Plot")
            .x_title("X Axis")
            .y_title("Y Axis")
            .build();
        let svg = plot.to_svg();
        assert!(svg.contains("My Plot"));
    }

    #[test]
    fn horizontal_legend_border_debug() {
        use plotlars_core::components::{Legend, Orientation};
        let df = df![
            "x" => [1.0, 2.0, 3.0],
            "y" => [4.0, 5.0, 6.0],
            "g" => ["a", "b", "c"]
        ]
        .unwrap();
        let plot = ScatterPlot::builder()
            .data(&df)
            .x("x")
            .y("y")
            .group("g")
            .legend_title("test")
            .legend(
                &Legend::new()
                    .orientation(Orientation::Horizontal)
                    .border_width(50),
            )
            .build();
        let svg = plot.to_svg();
        assert!(
            svg.contains("stroke-width=\"50\""),
            "SVG should contain stroke-width=50"
        );
    }

    #[test]
    fn line_plot_renders_to_svg() {
        let df = df![
            "x" => [1.0, 2.0, 3.0, 4.0],
            "y" => [10.0, 20.0, 15.0, 25.0]
        ]
        .unwrap();
        let plot = LinePlot::builder().data(&df).x("x").y("y").build();
        let svg = plot.to_svg();
        assert!(!svg.is_empty());
        assert!(svg.contains("<svg"));
    }

    #[test]
    fn line_plot_additional_lines_renders() {
        let df = df![
            "x" => [1.0, 2.0, 3.0],
            "y1" => [4.0, 5.0, 6.0],
            "y2" => [7.0, 8.0, 9.0]
        ]
        .unwrap();
        let plot = LinePlot::builder()
            .data(&df)
            .x("x")
            .y("y1")
            .additional_lines(vec!["y2"])
            .build();
        let svg = plot.to_svg();
        assert!(!svg.is_empty());
    }

    #[test]
    fn bar_plot_renders_to_svg() {
        let df = df![
            "labels" => ["a", "b", "c"],
            "values" => [10.0, 20.0, 30.0]
        ]
        .unwrap();
        let plot = BarPlot::builder()
            .data(&df)
            .labels("labels")
            .values("values")
            .build();
        let svg = plot.to_svg();
        assert!(!svg.is_empty());
        assert!(svg.contains("<svg"));
    }

    #[test]
    fn bar_plot_grouped_renders() {
        let df = df![
            "labels" => ["a", "b", "a", "b"],
            "values" => [10.0, 20.0, 30.0, 40.0],
            "g" => ["x", "x", "y", "y"]
        ]
        .unwrap();
        let plot = BarPlot::builder()
            .data(&df)
            .labels("labels")
            .values("values")
            .group("g")
            .build();
        let svg = plot.to_svg();
        assert!(!svg.is_empty());
    }

    #[test]
    fn bar_plot_horizontal_renders() {
        let df = df![
            "labels" => ["a", "b", "c"],
            "values" => [10.0, 20.0, 30.0]
        ]
        .unwrap();
        let plot = BarPlot::builder()
            .data(&df)
            .labels("labels")
            .values("values")
            .orientation(plotlars_core::components::Orientation::Horizontal)
            .build();
        let svg = plot.to_svg();
        assert!(!svg.is_empty());
    }

    #[test]
    fn histogram_renders_to_svg() {
        let df = df!["x" => [1.0, 2.0, 2.0, 3.0, 3.0, 3.0]].unwrap();
        let plot = Histogram::builder().data(&df).x("x").build();
        let svg = plot.to_svg();
        assert!(!svg.is_empty());
        assert!(svg.contains("<svg"));
    }

    #[test]
    fn histogram_grouped_renders() {
        let df = df![
            "x" => [1.0, 2.0, 3.0, 4.0, 5.0, 6.0],
            "g" => ["a", "a", "a", "b", "b", "b"]
        ]
        .unwrap();
        let plot = Histogram::builder().data(&df).x("x").group("g").build();
        let svg = plot.to_svg();
        assert!(!svg.is_empty());
    }
}
