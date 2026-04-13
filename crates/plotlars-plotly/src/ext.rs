use std::env;

use plotlars_core::Plot;

use crate::render::{
    build_plotly_result, ir_to_json, open_html_file, render_html_from_json,
    render_inline_html_from_json,
};

/// Plotly rendering extension trait. Provides all visualization methods.
pub trait PlotlyExt: Plot {
    fn plot(&self);
    fn write_html(&self, path: impl Into<String>);
    fn to_json(&self) -> Result<String, serde_json::Error>;
    fn to_html(&self) -> String;
    fn to_inline_html(&self, plot_div_id: Option<&str>) -> String;

    #[cfg(any(
        feature = "export-chrome",
        feature = "export-firefox",
        feature = "export-default"
    ))]
    fn write_image(
        &self,
        path: impl Into<String>,
        width: usize,
        height: usize,
        scale: f64,
    ) -> Result<(), Box<dyn std::error::Error + 'static>>;
}

impl<T: Plot> PlotlyExt for T {
    fn plot(&self) {
        let result = build_plotly_result(self);
        if result.layout_overrides.is_some() {
            // For plots with layout overrides (scene/polar domains),
            // we must serialize traces via Trace::to_json() to capture
            // injected keys like "scene" on Surface traces.
            let json = ir_to_json(self).unwrap_or_default();
            let html = render_html_from_json(&json);
            let temp_dir = std::env::temp_dir();
            let timestamp = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos();
            let temp_path = temp_dir.join(format!(
                "plotlars_{}_{}.html",
                std::process::id(),
                timestamp
            ));
            std::fs::write(&temp_path, html).expect("failed to write temp html");
            open_html_file(&temp_path);
        } else {
            match env::var("EVCXR_IS_RUNTIME") {
                Ok(_) => result.plot.evcxr_display(),
                _ => result.plot.show(),
            }
        }
    }

    fn write_html(&self, path: impl Into<String>) {
        let result = build_plotly_result(self);
        if result.layout_overrides.is_some() {
            let json = ir_to_json(self).unwrap_or_default();
            let html = render_html_from_json(&json);
            std::fs::write(path.into(), html).expect("failed to write html output");
        } else {
            result.plot.write_html(path.into());
        }
    }

    fn to_json(&self) -> Result<String, serde_json::Error> {
        ir_to_json(self)
    }

    fn to_html(&self) -> String {
        let result = build_plotly_result(self);
        if result.layout_overrides.is_some() {
            let json = ir_to_json(self).unwrap_or_default();
            render_html_from_json(&json)
        } else {
            result.plot.to_html()
        }
    }

    fn to_inline_html(&self, plot_div_id: Option<&str>) -> String {
        let result = build_plotly_result(self);
        let div_id = plot_div_id.unwrap_or("plotly-div");
        if result.layout_overrides.is_some() {
            let json = ir_to_json(self).unwrap_or_default();
            render_inline_html_from_json(&json, div_id)
        } else {
            result.plot.to_inline_html(plot_div_id)
        }
    }

    #[cfg(any(
        feature = "export-chrome",
        feature = "export-firefox",
        feature = "export-default"
    ))]
    fn write_image(
        &self,
        path: impl Into<String>,
        width: usize,
        height: usize,
        scale: f64,
    ) -> Result<(), Box<dyn std::error::Error + 'static>> {
        let path_string = path.into();
        let result = build_plotly_result(self);

        // Image export uses plotly.rs directly; layout overrides are not
        // applicable here because the plotly.js static exporter only reads
        // the standard Layout fields. For scene/polar faceted plots the
        // JSON override path should be used for HTML output only.
        if let Some((filename, extension)) = path_string.rsplit_once('.') {
            let format = match extension {
                "png" => plotly::ImageFormat::PNG,
                "jpg" | "jpeg" => plotly::ImageFormat::JPEG,
                "webp" => plotly::ImageFormat::WEBP,
                "svg" => plotly::ImageFormat::SVG,
                _ => return Err(format!("Unsupported image format: {extension}").into()),
            };
            result
                .plot
                .write_image(filename, format, width, height, scale)?;
        } else {
            return Err("No extension provided for image.".into());
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use plotlars_core::components::orientation::Orientation;
    use plotlars_core::components::Rgb;
    use plotlars_core::plots::array2dplot::Array2dPlot;
    use plotlars_core::plots::barplot::BarPlot;
    use plotlars_core::plots::boxplot::BoxPlot;
    use plotlars_core::plots::candlestick::CandlestickPlot;
    use plotlars_core::plots::contourplot::ContourPlot;
    use plotlars_core::plots::density_mapbox::DensityMapbox;
    use plotlars_core::plots::heatmap::HeatMap;
    use plotlars_core::plots::histogram::Histogram;
    use plotlars_core::plots::lineplot::LinePlot;
    use plotlars_core::plots::mesh3d::Mesh3D;
    use plotlars_core::plots::ohlc::OhlcPlot;
    use plotlars_core::plots::piechart::PieChart;
    use plotlars_core::plots::sankeydiagram::SankeyDiagram;
    use plotlars_core::plots::scatter3dplot::Scatter3dPlot;
    use plotlars_core::plots::scattergeo::ScatterGeo;
    use plotlars_core::plots::scattermap::ScatterMap;
    use plotlars_core::plots::scatterplot::ScatterPlot;
    use plotlars_core::plots::scatterpolar::ScatterPolar;
    use plotlars_core::plots::surfaceplot::SurfacePlot;
    use plotlars_core::plots::table::Table;
    use plotlars_core::plots::timeseriesplot::TimeSeriesPlot;
    use plotlars_core::Plot;
    use polars::prelude::*;

    fn to_json_value(plot: &impl Plot) -> serde_json::Value {
        let json_str = ir_to_json(plot).unwrap();
        serde_json::from_str(&json_str).unwrap()
    }

    #[test]
    fn test_scatter_to_json_has_traces() {
        let df = df!["x" => [1.0, 2.0, 3.0], "y" => [4.0, 5.0, 6.0]].unwrap();
        let plot = ScatterPlot::builder().data(&df).x("x").y("y").build();
        let json = to_json_value(&plot);
        assert!(json["traces"].is_array());
        assert_eq!(json["traces"].as_array().unwrap().len(), 1);
    }

    #[test]
    fn test_scatter_to_json_has_layout() {
        let df = df!["x" => [1.0, 2.0, 3.0], "y" => [4.0, 5.0, 6.0]].unwrap();
        let plot = ScatterPlot::builder().data(&df).x("x").y("y").build();
        let json = to_json_value(&plot);
        assert!(json["layout"].is_object());
    }

    #[test]
    fn test_scatter_to_json_with_title() {
        let df = df!["x" => [1.0, 2.0, 3.0], "y" => [4.0, 5.0, 6.0]].unwrap();
        let plot = ScatterPlot::builder()
            .data(&df)
            .x("x")
            .y("y")
            .plot_title("My Plot")
            .build();
        let json = to_json_value(&plot);
        let layout_str = serde_json::to_string(&json["layout"]).unwrap();
        assert!(layout_str.contains("My Plot"));
    }

    #[test]
    fn test_bar_to_json_has_traces() {
        let df = df!["labels" => ["a", "b", "c"], "values" => [10.0, 20.0, 30.0]].unwrap();
        let plot = BarPlot::builder()
            .data(&df)
            .labels("labels")
            .values("values")
            .build();
        let json = to_json_value(&plot);
        assert_eq!(json["traces"].as_array().unwrap().len(), 1);
    }

    #[test]
    fn test_bar_to_json_trace_type() {
        let df = df!["labels" => ["a", "b", "c"], "values" => [10.0, 20.0, 30.0]].unwrap();
        let plot = BarPlot::builder()
            .data(&df)
            .labels("labels")
            .values("values")
            .build();
        let json = to_json_value(&plot);
        assert_eq!(json["traces"][0]["type"], "bar");
    }

    #[test]
    fn test_pie_to_json_trace_type() {
        let df = df!["labels" => ["a", "b", "c", "a", "b"]].unwrap();
        let plot = PieChart::builder().data(&df).labels("labels").build();
        let json = to_json_value(&plot);
        assert_eq!(json["traces"][0]["type"], "pie");
    }

    #[test]
    fn test_scatter_grouped_to_json() {
        let df = df![
            "x" => [1.0, 2.0, 3.0, 4.0],
            "y" => [10.0, 20.0, 30.0, 40.0],
            "g" => ["a", "b", "a", "b"]
        ]
        .unwrap();
        let plot = ScatterPlot::builder()
            .data(&df)
            .x("x")
            .y("y")
            .group("g")
            .build();
        let json = to_json_value(&plot);
        assert_eq!(json["traces"].as_array().unwrap().len(), 2);
    }

    #[test]
    fn test_scatter_trace_has_x_and_y() {
        let df = df!["x" => [1.0, 2.0, 3.0], "y" => [4.0, 5.0, 6.0]].unwrap();
        let plot = ScatterPlot::builder().data(&df).x("x").y("y").build();
        let json = to_json_value(&plot);
        let trace = &json["traces"][0];
        assert!(trace["x"].is_array());
        assert!(trace["y"].is_array());
        assert_eq!(trace["x"].as_array().unwrap().len(), 3);
    }

    #[test]
    fn test_histogram_to_json() {
        let df = df!["x" => [1.0, 2.0, 2.0, 3.0, 3.0, 3.0]].unwrap();
        let plot = Histogram::builder().data(&df).x("x").build();
        let json = to_json_value(&plot);
        assert_eq!(json["traces"].as_array().unwrap().len(), 1);
        assert_eq!(json["traces"][0]["type"], "histogram");
    }

    #[test]
    fn test_line_to_json() {
        let df = df!["x" => [1.0, 2.0, 3.0], "y" => [4.0, 5.0, 6.0]].unwrap();
        let plot = LinePlot::builder().data(&df).x("x").y("y").build();
        let json = to_json_value(&plot);
        assert_eq!(json["traces"].as_array().unwrap().len(), 1);
        assert_eq!(json["traces"][0]["type"], "scatter");
    }

    // ---- E2E scatter tests ----

    #[test]
    fn test_e2e_scatter() {
        let df = df!["x" => [1.0, 2.0, 3.0], "y" => [4.0, 5.0, 6.0]].unwrap();
        let plot = ScatterPlot::builder().data(&df).x("x").y("y").build();
        let json = to_json_value(&plot);
        let trace = &json["traces"][0];
        assert_eq!(trace["type"], "scatter");
        assert_eq!(trace["mode"], "markers");
        assert_eq!(trace["x"].as_array().unwrap().len(), 3);
        assert_eq!(trace["y"].as_array().unwrap().len(), 3);
    }

    #[test]
    fn test_e2e_scatter_styled() {
        let df = df!["x" => [1.0, 2.0, 3.0], "y" => [4.0, 5.0, 6.0]].unwrap();
        let plot = ScatterPlot::builder()
            .data(&df)
            .x("x")
            .y("y")
            .opacity(0.7)
            .size(12)
            .color(Rgb(255, 0, 0))
            .build();
        let json = to_json_value(&plot);
        let marker = &json["traces"][0]["marker"];
        assert_eq!(marker["opacity"], 0.7);
        assert_eq!(marker["size"], 12);
    }

    #[test]
    fn test_e2e_scatter_grouped() {
        let df = df![
            "x" => [1.0, 2.0, 3.0, 4.0],
            "y" => [10.0, 20.0, 30.0, 40.0],
            "g" => ["a", "b", "a", "b"]
        ]
        .unwrap();
        let plot = ScatterPlot::builder()
            .data(&df)
            .x("x")
            .y("y")
            .group("g")
            .build();
        let json = to_json_value(&plot);
        let traces = json["traces"].as_array().unwrap();
        assert_eq!(traces.len(), 2);
        assert!(traces[0]["name"].is_string());
        assert!(traces[1]["name"].is_string());
    }

    // ---- E2E bar tests ----

    #[test]
    fn test_e2e_bar() {
        let df = df!["labels" => ["a", "b", "c"], "values" => [10.0, 20.0, 30.0]].unwrap();
        let plot = BarPlot::builder()
            .data(&df)
            .labels("labels")
            .values("values")
            .build();
        let json = to_json_value(&plot);
        let trace = &json["traces"][0];
        assert_eq!(trace["type"], "bar");
        assert!(trace["x"].is_array() || trace["y"].is_array());
    }

    #[test]
    fn test_e2e_bar_horizontal() {
        let df = df!["labels" => ["a", "b", "c"], "values" => [10.0, 20.0, 30.0]].unwrap();
        let plot = BarPlot::builder()
            .data(&df)
            .labels("labels")
            .values("values")
            .orientation(Orientation::Horizontal)
            .build();
        let json = to_json_value(&plot);
        let trace = &json["traces"][0];
        assert_eq!(trace["type"], "bar");
        assert_eq!(trace["orientation"], "h");
    }

    #[test]
    fn test_e2e_bar_grouped() {
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
        let json = to_json_value(&plot);
        assert_eq!(json["traces"].as_array().unwrap().len(), 2);
    }

    // ---- E2E boxplot test ----

    #[test]
    fn test_e2e_boxplot() {
        let df = df![
            "labels" => ["a", "a", "b", "b"],
            "values" => [1.0, 2.0, 3.0, 4.0]
        ]
        .unwrap();
        let plot = BoxPlot::builder()
            .data(&df)
            .labels("labels")
            .values("values")
            .build();
        let json = to_json_value(&plot);
        assert_eq!(json["traces"][0]["type"], "box");
    }

    // ---- E2E histogram test ----

    #[test]
    fn test_e2e_histogram() {
        let df = df!["x" => [1.0, 2.0, 2.0, 3.0, 3.0, 3.0]].unwrap();
        let plot = Histogram::builder().data(&df).x("x").build();
        let json = to_json_value(&plot);
        assert_eq!(json["traces"][0]["type"], "histogram");
        assert!(json["traces"][0]["x"].is_array());
    }

    // ---- E2E line plot tests ----

    #[test]
    fn test_e2e_lineplot() {
        let df = df!["x" => [1.0, 2.0, 3.0], "y" => [4.0, 5.0, 6.0]].unwrap();
        let plot = LinePlot::builder().data(&df).x("x").y("y").build();
        let json = to_json_value(&plot);
        let trace = &json["traces"][0];
        assert_eq!(trace["type"], "scatter");
        assert!(trace["x"].is_array());
        assert!(trace["y"].is_array());
    }

    #[test]
    fn test_e2e_lineplot_additional_lines() {
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
        let json = to_json_value(&plot);
        assert!(json["traces"].as_array().unwrap().len() >= 2);
    }

    // ---- E2E time series test ----

    #[test]
    fn test_e2e_timeseries() {
        let df = df![
            "date" => ["2024-01", "2024-02", "2024-03"],
            "val" => [10.0, 20.0, 30.0]
        ]
        .unwrap();
        let plot = TimeSeriesPlot::builder()
            .data(&df)
            .x("date")
            .y("val")
            .build();
        let json = to_json_value(&plot);
        let trace = &json["traces"][0];
        assert_eq!(trace["type"], "scatter");
        assert_eq!(trace["x"].as_array().unwrap().len(), 3);
    }

    // ---- E2E heatmap test ----

    #[test]
    fn test_e2e_heatmap() {
        let df = df![
            "x" => ["a", "b", "c"],
            "y" => ["d", "e", "f"],
            "z" => [1.0, 2.0, 3.0]
        ]
        .unwrap();
        let plot = HeatMap::builder().data(&df).x("x").y("y").z("z").build();
        let json = to_json_value(&plot);
        assert_eq!(json["traces"][0]["type"], "heatmap");
    }

    // ---- E2E contour test ----

    #[test]
    fn test_e2e_contour() {
        let df = df![
            "x" => ["a", "b", "c"],
            "y" => ["d", "e", "f"],
            "z" => [1.0, 2.0, 3.0]
        ]
        .unwrap();
        let plot = ContourPlot::builder()
            .data(&df)
            .x("x")
            .y("y")
            .z("z")
            .build();
        let json = to_json_value(&plot);
        assert_eq!(json["traces"][0]["type"], "contour");
    }

    // ---- E2E pie chart test ----

    #[test]
    fn test_e2e_piechart() {
        let df = df!["labels" => ["a", "b", "c", "a", "b"]].unwrap();
        let plot = PieChart::builder().data(&df).labels("labels").build();
        let json = to_json_value(&plot);
        assert_eq!(json["traces"][0]["type"], "pie");
        assert!(json["traces"][0]["labels"].is_array());
    }

    // ---- E2E sankey test ----

    #[test]
    fn test_e2e_sankey() {
        let df = df![
            "source" => ["A", "A", "B"],
            "target" => ["B", "C", "C"],
            "value" => [10.0, 20.0, 30.0]
        ]
        .unwrap();
        let plot = SankeyDiagram::builder()
            .data(&df)
            .sources("source")
            .targets("target")
            .values("value")
            .build();
        let json = to_json_value(&plot);
        assert_eq!(json["traces"][0]["type"], "sankey");
        assert!(json["traces"][0]["node"].is_object());
        assert!(json["traces"][0]["link"].is_object());
    }

    // ---- E2E candlestick test ----

    #[test]
    fn test_e2e_candlestick() {
        let df = df![
            "date"  => ["2024-01-01", "2024-01-02", "2024-01-03"],
            "open"  => [100.0, 105.0, 102.0],
            "high"  => [110.0, 108.0, 107.0],
            "low"   => [ 95.0, 100.0,  98.0],
            "close" => [105.0, 102.0, 106.0]
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
        let json = to_json_value(&plot);
        assert_eq!(json["traces"][0]["type"], "candlestick");
    }

    // ---- E2E OHLC test ----

    #[test]
    fn test_e2e_ohlc() {
        let df = df![
            "date"  => ["2024-01-01", "2024-01-02", "2024-01-03"],
            "open"  => [100.0, 105.0, 102.0],
            "high"  => [110.0, 108.0, 107.0],
            "low"   => [ 95.0, 100.0,  98.0],
            "close" => [105.0, 102.0, 106.0]
        ]
        .unwrap();
        let plot = OhlcPlot::builder()
            .data(&df)
            .dates("date")
            .open("open")
            .high("high")
            .low("low")
            .close("close")
            .build();
        let json = to_json_value(&plot);
        assert_eq!(json["traces"][0]["type"], "ohlc");
    }

    // ---- E2E scatter polar test ----

    #[test]
    fn test_e2e_scatter_polar() {
        let df = df![
            "angle" => [0.0, 90.0, 180.0, 270.0],
            "radius" => [1.0, 2.0, 3.0, 4.0]
        ]
        .unwrap();
        let plot = ScatterPolar::builder()
            .data(&df)
            .theta("angle")
            .r("radius")
            .build();
        let json = to_json_value(&plot);
        assert_eq!(json["traces"][0]["type"], "scatterpolar");
        assert!(json["traces"][0]["theta"].is_array());
        assert!(json["traces"][0]["r"].is_array());
    }

    // ---- E2E scatter 3d test ----

    #[test]
    fn test_e2e_scatter3d() {
        let df = df![
            "x" => [1.0, 2.0, 3.0],
            "y" => [4.0, 5.0, 6.0],
            "z" => [7.0, 8.0, 9.0]
        ]
        .unwrap();
        let plot = Scatter3dPlot::builder()
            .data(&df)
            .x("x")
            .y("y")
            .z("z")
            .build();
        let json = to_json_value(&plot);
        assert_eq!(json["traces"][0]["type"], "scatter3d");
        assert!(json["traces"][0]["x"].is_array());
        assert!(json["traces"][0]["y"].is_array());
        assert!(json["traces"][0]["z"].is_array());
    }

    // ---- E2E surface test ----

    #[test]
    fn test_e2e_surface() {
        let df = df![
            "x" => [1.0, 1.0, 2.0, 2.0],
            "y" => [1.0, 2.0, 1.0, 2.0],
            "z" => [5.0, 6.0, 7.0, 8.0]
        ]
        .unwrap();
        let plot = SurfacePlot::builder()
            .data(&df)
            .x("x")
            .y("y")
            .z("z")
            .build();
        let json = to_json_value(&plot);
        assert_eq!(json["traces"][0]["type"], "surface");
        assert!(json["traces"][0]["z"].is_array());
    }

    // ---- E2E mesh3d test ----

    #[test]
    fn test_e2e_mesh3d() {
        let df = df![
            "x" => [0.0, 1.0, 0.5, 0.5],
            "y" => [0.0, 0.0, 1.0, 0.5],
            "z" => [0.0, 0.0, 0.0, 1.0]
        ]
        .unwrap();
        let plot = Mesh3D::builder().data(&df).x("x").y("y").z("z").build();
        let json = to_json_value(&plot);
        assert_eq!(json["traces"][0]["type"], "mesh3d");
    }

    // ---- E2E scatter geo test ----

    #[test]
    fn test_e2e_scattergeo() {
        let df = df![
            "lat" => [40.7, 34.0, 41.8],
            "lon" => [-74.0, -118.2, -87.6]
        ]
        .unwrap();
        let plot = ScatterGeo::builder()
            .data(&df)
            .lat("lat")
            .lon("lon")
            .build();
        let json = to_json_value(&plot);
        assert_eq!(json["traces"][0]["type"], "scattergeo");
        assert!(json["traces"][0]["lat"].is_array());
        assert!(json["traces"][0]["lon"].is_array());
    }

    // ---- E2E scatter map test ----

    #[test]
    fn test_e2e_scattermap() {
        let df = df![
            "latitude" => [48.8, 51.5, 40.7],
            "longitude" => [2.3, -0.1, -74.0]
        ]
        .unwrap();
        let plot = ScatterMap::builder()
            .data(&df)
            .latitude("latitude")
            .longitude("longitude")
            .build();
        let json = to_json_value(&plot);
        assert_eq!(json["traces"][0]["type"], "scattermapbox");
        assert!(json["traces"][0]["lat"].is_array());
        assert!(json["traces"][0]["lon"].is_array());
        let layout_str = serde_json::to_string(&json["layout"]).unwrap();
        assert!(layout_str.contains("mapbox"));
    }

    // ---- E2E density mapbox test ----

    #[test]
    fn test_e2e_density_mapbox() {
        let df = df![
            "lat" => [40.7, 34.0, 41.8],
            "lon" => [-74.0, -118.2, -87.6],
            "density" => [100.0, 200.0, 150.0]
        ]
        .unwrap();
        let plot = DensityMapbox::builder()
            .data(&df)
            .lat("lat")
            .lon("lon")
            .z("density")
            .build();
        let json = to_json_value(&plot);
        assert_eq!(json["traces"][0]["type"], "densitymapbox");
        let layout_str = serde_json::to_string(&json["layout"]).unwrap();
        assert!(layout_str.contains("mapbox"));
    }

    // ---- E2E table test ----

    #[test]
    fn test_e2e_table() {
        let df = df![
            "name" => ["Alice", "Bob", "Carol"],
            "score" => [90, 85, 95]
        ]
        .unwrap();
        let plot = Table::builder()
            .data(&df)
            .columns(vec!["name", "score"])
            .build();
        let json = to_json_value(&plot);
        assert_eq!(json["traces"][0]["type"], "table");
        assert!(json["traces"][0]["header"].is_object());
        assert!(json["traces"][0]["cells"].is_object());
    }

    // ---- E2E array2d test ----

    #[test]
    fn test_e2e_array2d() {
        let data = vec![
            vec![[255, 0, 0], [0, 255, 0], [0, 0, 255]],
            vec![[0, 0, 255], [255, 0, 0], [0, 255, 0]],
        ];
        let plot = Array2dPlot::builder().data(&data).build();
        let json = to_json_value(&plot);
        assert_eq!(json["traces"][0]["type"], "image");
    }

    // ---- E2E layout titles test ----

    #[test]
    fn test_e2e_with_all_titles() {
        let df = df!["x" => [1.0, 2.0, 3.0], "y" => [4.0, 5.0, 6.0]].unwrap();
        let plot = ScatterPlot::builder()
            .data(&df)
            .x("x")
            .y("y")
            .plot_title("Main Title")
            .x_title("X Axis")
            .y_title("Y Axis")
            .legend_title("Groups")
            .build();
        let json = to_json_value(&plot);
        let layout = &json["layout"];
        let layout_str = serde_json::to_string(layout).unwrap();
        assert!(layout_str.contains("Main Title"));
        assert!(layout_str.contains("X Axis"));
        assert!(layout_str.contains("Y Axis"));
        assert!(layout_str.contains("Groups"));
    }

    // ---- E2E faceted test ----

    #[test]
    fn test_e2e_scatter_faceted() {
        let df = df![
            "x" => [1.0, 2.0, 3.0, 4.0, 5.0, 6.0],
            "y" => [10.0, 20.0, 30.0, 40.0, 50.0, 60.0],
            "f" => ["a", "a", "b", "b", "c", "c"]
        ]
        .unwrap();
        let plot = ScatterPlot::builder()
            .data(&df)
            .x("x")
            .y("y")
            .facet("f")
            .build();
        let json = to_json_value(&plot);
        let traces = json["traces"].as_array().unwrap();
        assert_eq!(traces.len(), 3);
        let layout_str = serde_json::to_string(&json["layout"]).unwrap();
        assert!(layout_str.contains("xaxis2") || layout_str.contains("yaxis2"));
    }
}
