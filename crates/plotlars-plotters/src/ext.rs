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
        crate::render::plot_interactive(self);
    }

    fn save(&self, path: &str) {
        crate::render::save_to_file(self, path);
    }

    fn to_svg(&self) -> String {
        crate::render::render_to_svg_string(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use plotlars_core::components::Rgb;
    use plotlars_core::plots::barplot::BarPlot;
    use plotlars_core::plots::boxplot::BoxPlot;
    use plotlars_core::plots::candlestick::CandlestickPlot;
    use plotlars_core::plots::heatmap::HeatMap;
    use plotlars_core::plots::histogram::Histogram;
    use plotlars_core::plots::lineplot::LinePlot;
    use plotlars_core::plots::scatterplot::ScatterPlot;
    use plotlars_core::plots::timeseriesplot::TimeSeriesPlot;
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
    fn boxplot_renders_to_svg() {
        let df = df![
            "species" => ["a", "a", "a", "a", "a", "b", "b", "b", "b", "b"],
            "value" => [1.0, 2.0, 3.0, 4.0, 5.0, 2.0, 3.0, 4.0, 5.0, 6.0]
        ]
        .unwrap();
        let plot = BoxPlot::builder()
            .data(&df)
            .labels("species")
            .values("value")
            .build();
        let svg = plot.to_svg();
        assert!(!svg.is_empty());
        assert!(svg.contains("<svg"));
    }

    #[test]
    fn boxplot_grouped_renders() {
        let df = df![
            "species" => ["a", "a", "a", "a", "b", "b", "b", "b"],
            "value" => [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0],
            "g" => ["x", "x", "y", "y", "x", "x", "y", "y"]
        ]
        .unwrap();
        let plot = BoxPlot::builder()
            .data(&df)
            .labels("species")
            .values("value")
            .group("g")
            .build();
        let svg = plot.to_svg();
        assert!(!svg.is_empty());
        assert!(svg.contains("<svg"));
    }

    #[test]
    fn heatmap_renders_to_svg() {
        let df = df![
            "x" => ["a", "b", "c", "a", "b", "c"],
            "y" => ["p", "p", "p", "q", "q", "q"],
            "z" => [1.0, 2.0, 3.0, 4.0, 5.0, 6.0]
        ]
        .unwrap();
        let plot = HeatMap::builder().data(&df).x("x").y("y").z("z").build();
        let svg = plot.to_svg();
        assert!(!svg.is_empty());
        assert!(svg.contains("<svg"));
    }

    #[test]
    fn heatmap_with_palette_renders() {
        let df = df![
            "x" => ["a", "b", "a", "b"],
            "y" => ["p", "p", "q", "q"],
            "z" => [10.0, 20.0, 30.0, 40.0]
        ]
        .unwrap();
        let plot = HeatMap::builder()
            .data(&df)
            .x("x")
            .y("y")
            .z("z")
            .color_scale(plotlars_core::components::Palette::Hot)
            .build();
        let svg = plot.to_svg();
        assert!(!svg.is_empty());
    }

    #[test]
    fn candlestick_renders_to_svg() {
        let df = df![
            "date" => ["2024-01-01", "2024-01-02", "2024-01-03"],
            "open" => [100.0, 102.5, 101.0],
            "high" => [103.0, 104.0, 103.5],
            "low" => [99.0, 101.5, 100.0],
            "close" => [102.5, 101.0, 103.5]
        ]
        .unwrap();
        let plot = CandlestickPlot::builder()
            .data(&df)
            .dates("date")
            .open("open")
            .high("high")
            .low("low")
            .close("close")
            .build();
        let svg = plot.to_svg();
        assert!(!svg.is_empty());
        assert!(svg.contains("<svg"));
    }

    #[test]
    fn candlestick_with_colors_renders() {
        use plotlars_core::components::Direction;
        let df = df![
            "date" => ["2024-01-01", "2024-01-02", "2024-01-03"],
            "open" => [100.0, 102.5, 101.0],
            "high" => [103.0, 104.0, 103.5],
            "low" => [99.0, 101.5, 100.0],
            "close" => [102.5, 101.0, 103.5]
        ]
        .unwrap();
        let inc = Direction::new().line_color(Rgb(0, 150, 255));
        let dec = Direction::new().line_color(Rgb(200, 0, 100));
        let plot = CandlestickPlot::builder()
            .data(&df)
            .dates("date")
            .open("open")
            .high("high")
            .low("low")
            .close("close")
            .increasing(&inc)
            .decreasing(&dec)
            .build();
        let svg = plot.to_svg();
        assert!(!svg.is_empty());
    }

    #[test]
    fn timeseries_renders_to_svg() {
        let df = df![
            "date" => ["2024-01", "2024-02", "2024-03", "2024-04"],
            "y" => [10.0, 20.0, 15.0, 25.0]
        ]
        .unwrap();
        let plot = TimeSeriesPlot::builder().data(&df).x("date").y("y").build();
        let svg = plot.to_svg();
        assert!(!svg.is_empty());
        assert!(svg.contains("<svg"));
    }

    #[test]
    fn timeseries_additional_series_renders() {
        let df = df![
            "date" => ["2024-01", "2024-02", "2024-03", "2024-04"],
            "y1" => [10.0, 20.0, 15.0, 25.0],
            "y2" => [5.0, 15.0, 10.0, 20.0],
            "y3" => [8.0, 18.0, 12.0, 22.0]
        ]
        .unwrap();
        let plot = TimeSeriesPlot::builder()
            .data(&df)
            .x("date")
            .y("y1")
            .additional_series(vec!["y2", "y3"])
            .colors(vec![Rgb(128, 128, 128), Rgb(0, 122, 255), Rgb(255, 128, 0)])
            .build();
        let svg = plot.to_svg();
        assert!(!svg.is_empty());
        assert!(svg.contains("<svg"));
        // Should contain line elements for all 3 series
        assert!(svg.contains("<polyline"));
    }

    #[test]
    fn timeseries_dual_y_axis_renders() {
        use plotlars_core::components::axis::AxisSide;
        use plotlars_core::components::Axis;
        let df = df![
            "date" => ["2024-01", "2024-02", "2024-03", "2024-04"],
            "revenue" => [1000.0, 2000.0, 3000.0, 4000.0],
            "cost" => [100.0, 200.0, 150.0, 250.0]
        ]
        .unwrap();
        let plot = TimeSeriesPlot::builder()
            .data(&df)
            .x("date")
            .y("revenue")
            .additional_series(vec!["cost"])
            .colors(vec![Rgb(0, 0, 255), Rgb(255, 0, 0)])
            .y_title("revenue")
            .y2_title("cost")
            .y_axis(&Axis::new().value_color(Rgb(0, 0, 255)))
            .y2_axis(
                &Axis::new()
                    .axis_side(AxisSide::Right)
                    .value_color(Rgb(255, 0, 0)),
            )
            .build();
        let svg = plot.to_svg();
        assert!(!svg.is_empty());
        assert!(svg.contains("<svg"));
        // Should not contain the unsupported warning text
        // y2 axis labels should appear on right side
    }

    #[test]
    fn timeseries_365_points_with_dashed_lines() {
        use plotlars_core::components::Line as LineStyle;
        // Simulate debilt 2023 temps: 365 days, 3 series with dashed lines
        let dates: Vec<String> = (0..365)
            .map(|i| format!("2023-{:02}-{:02}", i / 30 + 1, i % 30 + 1))
            .collect();
        let tavg: Vec<f64> = (0..365)
            .map(|i| 10.0 + 10.0 * (i as f64 * 0.017).sin())
            .collect();
        let tmin: Vec<f64> = tavg.iter().map(|t| t - 5.0).collect();
        let tmax: Vec<f64> = tavg.iter().map(|t| t + 5.0).collect();

        let df = df![
            "date" => dates,
            "tavg" => tavg,
            "tmin" => tmin,
            "tmax" => tmax
        ]
        .unwrap();

        let start = std::time::Instant::now();
        let plot = TimeSeriesPlot::builder()
            .data(&df)
            .x("date")
            .y("tavg")
            .additional_series(vec!["tmin", "tmax"])
            .colors(vec![Rgb(128, 128, 128), Rgb(0, 122, 255), Rgb(255, 128, 0)])
            .lines(vec![LineStyle::Solid, LineStyle::Dot, LineStyle::Dot])
            .build();
        let svg = plot.to_svg();
        let elapsed = start.elapsed();

        assert!(!svg.is_empty());
        assert!(svg.contains("<svg"));
        assert!(
            svg.contains("stroke-dasharray"),
            "Dashed lines should have stroke-dasharray in SVG"
        );
        assert!(
            elapsed.as_secs() < 5,
            "Rendering took too long: {:?}",
            elapsed
        );
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
