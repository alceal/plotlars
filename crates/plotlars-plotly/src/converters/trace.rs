#![allow(dead_code)]

use plotly::{
    box_plot::BoxPoints,
    color::Rgb as RgbPlotly,
    common::{
        Domain, ErrorData, ErrorType, Line as LinePlotly, Marker as MarkerPlotly,
        Mode as ModePlotly,
    },
    contour::Contours as ContoursPlotly,
    histogram::{Bins, HistFunc},
    image::ColorModel,
    mesh3d::{Contour, DelaunayAxis, LightPosition, Lighting as LightingMesh3D},
    sankey::{Link, Node},
    traces::table::{Cells as CellsPlotly, Header as HeaderPlotly},
    Bar, BoxPlot as BoxPlotPlotly, Candlestick as CandlestickPlotly, Contour as ContourPlotly,
    DensityMapbox as DensityMapboxPlotly, HeatMap as HeatMapPlotly, Histogram as HistogramPlotly,
    Image as ImagePlotly, Mesh3D as Mesh3DPlotly, Ohlc as OhlcPlotly, Pie, Sankey, Scatter,
    Scatter3D, ScatterGeo as ScatterGeoPlotly, ScatterMapbox, ScatterPolar as ScatterPolarPlotly,
    Surface, Table as TablePlotly, Trace,
};

use crate::converters::components as conv;
use plotlars_core::components::Orientation;
use plotlars_core::ir::data::ColumnData;
use plotlars_core::ir::line::LineIR;
use plotlars_core::ir::marker::MarkerIR;
use plotlars_core::ir::trace::{
    Array2dPlotIR, BarPlotIR, BoxPlotIR, CandlestickPlotIR, ContourPlotIR, DensityMapboxIR,
    HeatMapIR, HistogramIR, ImageIR, LinePlotIR, Mesh3DIR, OhlcPlotIR, PieChartIR, SankeyDiagramIR,
    Scatter3dPlotIR, ScatterGeoIR, ScatterMapIR, ScatterPlotIR, ScatterPolarIR, SurfacePlotIR,
    TableIR, TimeSeriesPlotIR, TraceIR,
};

/// Wrapper around a serialized trace that injects a `scene` key.
/// Used for Surface plots where plotly.rs doesn't expose `.scene()`.
#[derive(Clone, serde::Serialize)]
struct TraceWithScene {
    #[serde(skip)]
    json: String,
}

impl Trace for TraceWithScene {
    fn to_json(&self) -> String {
        self.json.clone()
    }
}

fn wrap_trace_with_scene(inner: &dyn Trace, scene_ref: &str) -> Box<dyn Trace + 'static> {
    let mut json: serde_json::Value =
        serde_json::from_str(&inner.to_json()).unwrap_or(serde_json::Value::Null);
    if let serde_json::Value::Object(ref mut map) = json {
        map.insert(
            "scene".to_string(),
            serde_json::Value::String(scene_ref.to_string()),
        );
    }
    Box::new(TraceWithScene {
        json: serde_json::to_string(&json).unwrap_or_default(),
    })
}

pub(crate) fn convert(trace: &TraceIR) -> Box<dyn Trace + 'static> {
    match trace {
        TraceIR::Image(ir) => convert_image(ir),
        TraceIR::Array2dPlot(ir) => convert_array2d(ir),
        TraceIR::OhlcPlot(ir) => convert_ohlc(ir),
        TraceIR::CandlestickPlot(ir) => convert_candlestick(ir),
        TraceIR::Table(ir) => convert_table(ir),
        TraceIR::ScatterGeo(ir) => convert_scatter_geo(ir),
        TraceIR::ScatterMap(ir) => convert_scatter_map(ir),
        TraceIR::DensityMapbox(ir) => convert_density_mapbox(ir),
        TraceIR::ScatterPlot(ir) => convert_scatter_plot(ir),
        TraceIR::LinePlot(ir) => convert_line_plot(ir),
        TraceIR::TimeSeriesPlot(ir) => convert_time_series_plot(ir),
        TraceIR::ScatterPolar(ir) => convert_scatter_polar(ir),
        TraceIR::Scatter3dPlot(ir) => convert_scatter_3d(ir),
        TraceIR::SurfacePlot(ir) => convert_surface_plot(ir),
        TraceIR::Mesh3D(ir) => convert_mesh3d(ir),
        TraceIR::BarPlot(ir) => convert_bar_plot(ir),
        TraceIR::BoxPlot(ir) => convert_box_plot(ir),
        TraceIR::Histogram(ir) => convert_histogram(ir),
        TraceIR::ContourPlot(ir) => convert_contour_plot(ir),
        TraceIR::HeatMap(ir) => convert_heat_map(ir),
        TraceIR::PieChart(ir) => convert_pie_chart(ir),
        TraceIR::SankeyDiagram(ir) => convert_sankey_diagram(ir),
        _ => unimplemented!("TraceIR variant not yet implemented for plotly conversion"),
    }
}

fn convert_image(ir: &ImageIR) -> Box<dyn Trace + 'static> {
    let pixels: Vec<Vec<RgbPlotly>> = ir
        .pixels
        .iter()
        .map(|row| {
            row.iter()
                .map(|p| RgbPlotly::new(p[0], p[1], p[2]))
                .collect()
        })
        .collect();

    ImagePlotly::new(pixels).color_model(ColorModel::RGB)
}

fn convert_array2d(ir: &Array2dPlotIR) -> Box<dyn Trace + 'static> {
    let pixels: Vec<Vec<RgbPlotly>> = ir
        .data
        .iter()
        .map(|row| {
            row.iter()
                .map(|rgb| RgbPlotly::new(rgb[0], rgb[1], rgb[2]))
                .collect()
        })
        .collect();

    ImagePlotly::new(pixels).color_model(ColorModel::RGB)
}

fn extract_numeric(col: &ColumnData) -> Vec<f32> {
    match col {
        ColumnData::Numeric(data) => data.iter().map(|v| v.unwrap_or(0.0)).collect(),
        _ => panic!("expected numeric column data"),
    }
}

fn extract_strings(col: &ColumnData) -> Vec<String> {
    match col {
        ColumnData::String(data) => data.iter().map(|v| v.clone().unwrap_or_default()).collect(),
        _ => panic!("expected string column data"),
    }
}

fn extract_numeric_options(col: &ColumnData) -> Vec<Option<f32>> {
    match col {
        ColumnData::Numeric(data) => data.clone(),
        _ => panic!("expected numeric column data"),
    }
}

fn build_marker(marker_ir: &MarkerIR) -> MarkerPlotly {
    let mut marker = MarkerPlotly::new();

    if let Some(opacity) = marker_ir.opacity {
        marker = marker.opacity(opacity);
    }

    if let Some(size) = marker_ir.size {
        marker = marker.size(size);
    }

    if let Some(ref rgb) = marker_ir.color {
        marker = marker.color(RgbPlotly::new(rgb.0, rgb.1, rgb.2));
    }

    if let Some(ref shape) = marker_ir.shape {
        marker = marker.symbol(conv::convert_shape(shape));
    }

    marker
}

fn build_line(line_ir: &LineIR) -> LinePlotly {
    let mut line = LinePlotly::new();

    if let Some(width) = line_ir.width {
        line = line.width(width);
    }

    if let Some(ref color) = line_ir.color {
        line = line.color(RgbPlotly::new(color.0, color.1, color.2));
    }

    if let Some(ref style) = line_ir.style {
        line = line.dash(conv::convert_line(style));
    }

    line
}

fn convert_ohlc(ir: &OhlcPlotIR) -> Box<dyn Trace + 'static> {
    let dates_values = extract_strings(&ir.dates);
    let open_values = extract_numeric(&ir.open);
    let high_values = extract_numeric(&ir.high);
    let low_values = extract_numeric(&ir.low);
    let close_values = extract_numeric(&ir.close);

    let mut trace = *OhlcPlotly::new(
        dates_values,
        open_values,
        high_values,
        low_values,
        close_values,
    );

    if let Some(tick_w) = ir.tick_width {
        trace = trace.tick_width(tick_w);
    }

    Box::new(trace)
}

fn convert_candlestick(ir: &CandlestickPlotIR) -> Box<dyn Trace + 'static> {
    let dates_values = extract_strings(&ir.dates);
    let open_values = extract_numeric(&ir.open);
    let high_values = extract_numeric(&ir.high);
    let low_values = extract_numeric(&ir.low);
    let close_values = extract_numeric(&ir.close);

    let mut trace = *CandlestickPlotly::new(
        dates_values,
        open_values,
        high_values,
        low_values,
        close_values,
    );

    if let Some(ref inc) = ir.increasing {
        trace = trace.increasing(conv::convert_direction_increasing(inc));
    }

    if let Some(ref dec) = ir.decreasing {
        trace = trace.decreasing(conv::convert_direction_decreasing(dec));
    }

    if let Some(whisker_w) = ir.whisker_width {
        trace = trace.whisker_width(whisker_w);
    }

    Box::new(trace)
}

fn convert_table(ir: &TableIR) -> Box<dyn Trace + 'static> {
    let plotly_header = if let Some(ref h) = ir.header {
        conv::convert_header(h, ir.column_names.clone())
    } else {
        HeaderPlotly::new(ir.column_names.clone())
    };

    let plotly_cells = if let Some(ref c) = ir.cell {
        conv::convert_cell(c, ir.column_data.clone())
    } else {
        CellsPlotly::new(ir.column_data.clone())
    };

    let mut table = TablePlotly::new(plotly_header, plotly_cells);

    if let Some(width) = ir.column_width {
        table = table.column_width(width);
    }

    table
}

fn convert_scatter_geo(ir: &ScatterGeoIR) -> Box<dyn Trace + 'static> {
    let lat_data = extract_numeric_options(&ir.lat);
    let lon_data = extract_numeric_options(&ir.lon);

    let mut trace = ScatterGeoPlotly::new(lat_data, lon_data);

    if let Some(ref mode) = ir.mode {
        trace = trace.mode(conv::convert_mode(mode));
    } else {
        trace = trace.mode(plotly::common::Mode::Markers);
    }

    if let Some(ref marker_ir) = ir.marker {
        trace = trace.marker(build_marker(marker_ir));
    }

    if let Some(ref line_ir) = ir.line {
        trace = trace.line(build_line(line_ir));
    }

    if let Some(ref text_data) = ir.text {
        let text_strings = extract_strings(text_data);
        trace = trace.text_array(text_strings);
    }

    if let Some(ref name) = ir.name {
        trace = trace.name(name.as_str());
    }

    if let Some(show) = ir.show_legend {
        trace = trace.show_legend(show);
    }

    trace
}

fn convert_scatter_map(ir: &ScatterMapIR) -> Box<dyn Trace + 'static> {
    let lat_data = extract_numeric_options(&ir.lat);
    let lon_data = extract_numeric_options(&ir.lon);

    let mut trace = ScatterMapbox::default()
        .lat(lat_data)
        .lon(lon_data)
        .mode(plotly::common::Mode::Markers);

    if let Some(ref marker_ir) = ir.marker {
        trace = trace.marker(build_marker(marker_ir));
    }

    if let Some(ref name) = ir.name {
        trace = trace.name(name.as_str());
    }

    if let Some(show) = ir.show_legend {
        trace = trace.show_legend(show);
    }

    trace
}

fn convert_density_mapbox(ir: &DensityMapboxIR) -> Box<dyn Trace + 'static> {
    let lat_data = extract_numeric_options(&ir.lat);
    let lon_data = extract_numeric_options(&ir.lon);
    let z_data = extract_numeric_options(&ir.z);

    let mut trace = DensityMapboxPlotly::new(lat_data, lon_data, z_data);

    if let Some(radius) = ir.radius {
        trace = trace.radius(radius);
    }

    if let Some(opacity) = ir.opacity {
        trace = trace.opacity(opacity);
    }

    if let Some(z_min) = ir.z_min {
        trace = trace.zmin(Some(z_min as f32));
    }

    if let Some(z_max) = ir.z_max {
        trace = trace.zmax(Some(z_max as f32));
    }

    if let Some(z_mid) = ir.z_mid {
        trace = trace.zmid(Some(z_mid as f32));
    }

    trace
}

fn convert_scatter_plot(ir: &ScatterPlotIR) -> Box<dyn Trace + 'static> {
    let x_data = extract_numeric_options(&ir.x);
    let y_data = extract_numeric_options(&ir.y);

    let mut trace = Scatter::default()
        .x(x_data)
        .y(y_data)
        .mode(ModePlotly::Markers);

    if let Some(ref marker_ir) = ir.marker {
        trace = trace.marker(build_marker(marker_ir));
    }

    if let Some(ref fill) = ir.fill {
        trace = trace.fill(conv::convert_fill(fill));
    }

    if let Some(ref name) = ir.name {
        trace = trace.name(name.as_str());
    }

    if let Some(ref subplot_ref) = ir.subplot_ref {
        let (x_axis, y_axis) = parse_subplot_axes(subplot_ref);
        trace = trace.x_axis(&x_axis).y_axis(&y_axis);
    }

    if let Some(ref group) = ir.legend_group {
        trace = trace.legend_group(group);
    }

    if let Some(show) = ir.show_legend {
        trace = trace.show_legend(show);
    }

    trace
}

fn convert_line_plot(ir: &LinePlotIR) -> Box<dyn Trace + 'static> {
    let x_data = extract_numeric_options(&ir.x);
    let y_data = extract_numeric_options(&ir.y);

    let mut trace = Scatter::default().x(x_data).y(y_data);

    if let Some(ref mode) = ir.mode {
        trace = trace.mode(conv::convert_mode(mode));
    }

    if let Some(ref marker_ir) = ir.marker {
        trace = trace.marker(build_marker(marker_ir));
    }

    if let Some(ref line_ir) = ir.line {
        trace = trace.line(build_line(line_ir));
    }

    if let Some(ref name) = ir.name {
        trace = trace.name(name.as_str());
    }

    if let Some(ref subplot_ref) = ir.subplot_ref {
        let (x_axis, y_axis) = parse_subplot_axes(subplot_ref);
        trace = trace.x_axis(&x_axis).y_axis(&y_axis);
    }

    if let Some(show) = ir.show_legend {
        trace = trace.show_legend(show);
    }

    if let Some(ref group) = ir.legend_group {
        trace = trace.legend_group(group);
    }

    trace
}

fn convert_time_series_plot(ir: &TimeSeriesPlotIR) -> Box<dyn Trace + 'static> {
    let x_data = extract_strings(&ir.x);
    let y_data = extract_numeric_options(&ir.y);

    let mut trace = Scatter::default().x(x_data).y(y_data);

    if let Some(ref mode) = ir.mode {
        trace = trace.mode(conv::convert_mode(mode));
    }

    if let Some(ref marker_ir) = ir.marker {
        trace = trace.marker(build_marker(marker_ir));
    }

    if let Some(ref line_ir) = ir.line {
        trace = trace.line(build_line(line_ir));
    }

    if let Some(ref name) = ir.name {
        trace = trace.name(name.as_str());
    }

    if let Some(ref y_ref) = ir.y_axis_ref {
        trace = trace.y_axis(y_ref);
    }

    if let Some(ref subplot_ref) = ir.subplot_ref {
        let (x_axis, y_axis) = parse_subplot_axes(subplot_ref);
        trace = trace.x_axis(&x_axis).y_axis(&y_axis);
    }

    if let Some(show) = ir.show_legend {
        trace = trace.show_legend(show);
    }

    if let Some(ref group) = ir.legend_group {
        trace = trace.legend_group(group);
    }

    trace
}

fn parse_subplot_axes(subplot_ref: &str) -> (String, String) {
    if let Some(pos) = subplot_ref.find('y') {
        let x_axis = &subplot_ref[..pos];
        let y_axis = &subplot_ref[pos..];
        (x_axis.to_string(), y_axis.to_string())
    } else {
        (subplot_ref.to_string(), String::new())
    }
}

fn extract_numeric_as_usize(col: &ColumnData) -> Vec<usize> {
    match col {
        ColumnData::Numeric(v) => v.iter().map(|o| o.unwrap_or(0.0) as usize).collect(),
        _ => vec![],
    }
}

fn convert_scatter_polar(ir: &ScatterPolarIR) -> Box<dyn Trace + 'static> {
    let theta_data = extract_numeric_options(&ir.theta);
    let r_data = extract_numeric_options(&ir.r);

    let mut trace = ScatterPolarPlotly::default().theta(theta_data).r(r_data);

    if let Some(ref mode) = ir.mode {
        trace = trace.mode(conv::convert_mode(mode));
    }
    if let Some(ref marker_ir) = ir.marker {
        trace = trace.marker(build_marker(marker_ir));
    }
    if let Some(ref line_ir) = ir.line {
        trace = trace.line(build_line(line_ir));
    }
    if let Some(ref fill) = ir.fill {
        trace = trace.fill(conv::convert_fill(fill));
    }
    if let Some(ref name) = ir.name {
        trace = trace.name(name.as_str());
    }
    if let Some(ref subplot_ref) = ir.subplot_ref {
        trace = trace.subplot(subplot_ref);
    }
    if let Some(ref group) = ir.legend_group {
        trace = trace.legend_group(group);
    }
    if let Some(show) = ir.show_legend {
        trace = trace.show_legend(show);
    }

    trace
}

fn convert_scatter_3d(ir: &Scatter3dPlotIR) -> Box<dyn Trace + 'static> {
    let x_data = extract_numeric_options(&ir.x);
    let y_data = extract_numeric_options(&ir.y);
    let z_data = extract_numeric_options(&ir.z);

    let mut trace = Scatter3D::default()
        .x(x_data)
        .y(y_data)
        .z(z_data)
        .mode(ModePlotly::Markers);

    if let Some(ref mode) = ir.mode {
        trace = trace.mode(conv::convert_mode(mode));
    }
    if let Some(ref marker_ir) = ir.marker {
        trace = trace.marker(build_marker(marker_ir));
    }
    if let Some(ref name) = ir.name {
        trace = trace.name(name.as_str());
    }
    if let Some(ref scene_ref) = ir.scene_ref {
        trace = trace.scene(scene_ref);
    }
    if let Some(ref group) = ir.legend_group {
        trace = trace.legend_group(group);
    }
    if let Some(show) = ir.show_legend {
        trace = trace.show_legend(show);
    }

    trace
}

fn convert_surface_plot(ir: &SurfacePlotIR) -> Box<dyn Trace + 'static> {
    let x_data: Vec<f64> = match &ir.x {
        ColumnData::Numeric(v) => v.iter().map(|o| o.unwrap_or(0.0) as f64).collect(),
        _ => vec![],
    };
    let y_data: Vec<f64> = match &ir.y {
        ColumnData::Numeric(v) => v.iter().map(|o| o.unwrap_or(0.0) as f64).collect(),
        _ => vec![],
    };

    let mut trace = Surface::new(ir.z.clone()).x(x_data).y(y_data);

    if let Some(ref cs) = ir.color_scale {
        trace = trace.color_scale(conv::convert_palette(cs));
    }
    if let Some(ref cb) = ir.color_bar {
        trace = trace.color_bar(conv::convert_colorbar(cb));
    }
    if let Some(rs) = ir.reverse_scale {
        trace = trace.reverse_scale(rs);
    }
    if let Some(ss) = ir.show_scale {
        trace = trace.show_scale(ss);
    }
    trace = trace.lighting(conv::set_lighting(ir.lighting.as_ref()));
    if let Some(ref lighting) = ir.lighting {
        if let Some(position) = lighting.position {
            let position = plotly::surface::Position::new(position[0], position[1], position[2]);
            trace = trace.light_position(position);
        }
    }
    if let Some(opacity) = ir.opacity {
        trace = trace.opacity(opacity);
    }
    if let Some(ref scene_ref) = ir.scene_ref {
        let boxed: Box<dyn Trace + 'static> = trace;
        return wrap_trace_with_scene(boxed.as_ref(), scene_ref);
    }

    trace
}

fn convert_mesh3d(ir: &Mesh3DIR) -> Box<dyn Trace + 'static> {
    let x_data = extract_numeric_options(&ir.x);
    let y_data = extract_numeric_options(&ir.y);
    let z_data = extract_numeric_options(&ir.z);

    let mut trace = Mesh3DPlotly::new(x_data, y_data, z_data, None, None, None);

    if let Some(ref i) = ir.i {
        trace = trace.i(extract_numeric_as_usize(i));
    }
    if let Some(ref j) = ir.j {
        trace = trace.j(extract_numeric_as_usize(j));
    }
    if let Some(ref k) = ir.k {
        trace = trace.k(extract_numeric_as_usize(k));
    }
    if let Some(ref intensity) = ir.intensity {
        let intensity_data: Vec<f64> = match intensity {
            ColumnData::Numeric(v) => v.iter().map(|o| o.unwrap_or(0.0) as f64).collect(),
            _ => vec![],
        };
        trace = trace.intensity(intensity_data);
    }
    if let Some(ref im) = ir.intensity_mode {
        trace = trace.intensity_mode(conv::convert_intensity_mode(im));
    }
    if let Some(ref cs) = ir.color_scale {
        trace = trace.color_scale(conv::convert_palette(cs));
    }
    if let Some(ref cb) = ir.color_bar {
        trace = trace.color_bar(conv::convert_colorbar(cb));
    }
    if let Some(ref lighting) = ir.lighting {
        let mut lighting_mesh3d = LightingMesh3D::new();
        if let Some(ambient) = lighting.ambient {
            lighting_mesh3d = lighting_mesh3d.ambient(ambient);
        }
        if let Some(diffuse) = lighting.diffuse {
            lighting_mesh3d = lighting_mesh3d.diffuse(diffuse);
        }
        if let Some(fresnel) = lighting.fresnel {
            lighting_mesh3d = lighting_mesh3d.fresnel(fresnel);
        }
        if let Some(roughness) = lighting.roughness {
            lighting_mesh3d = lighting_mesh3d.roughness(roughness);
        }
        if let Some(specular) = lighting.specular {
            lighting_mesh3d = lighting_mesh3d.specular(specular);
        }
        trace = trace.lighting(lighting_mesh3d);
    }
    if let Some(opacity) = ir.opacity {
        trace = trace.opacity(opacity);
    }
    if let Some(ref color) = ir.color {
        trace = trace.color(RgbPlotly::new(color.0, color.1, color.2));
    }
    if let Some(flat_shading) = ir.flat_shading {
        trace = trace.flat_shading(flat_shading);
    }
    if let Some((x, y, z)) = ir.light_position {
        let position = LightPosition::new()
            .x(vec![x as f64])
            .y(vec![y as f64])
            .z(vec![z as f64]);
        trace = trace.light_position(position);
    }
    if let Some(ref axis) = ir.delaunay_axis {
        let axis = match axis.to_lowercase().as_str() {
            "x" => DelaunayAxis::X,
            "y" => DelaunayAxis::Y,
            "z" => DelaunayAxis::Z,
            _ => DelaunayAxis::Z,
        };
        trace = trace.delaunay_axis(axis);
    }
    if let Some(true) = ir.contour {
        trace = trace.contour(Contour::new().show(true).width(2));
    }
    if let Some(ref scene_ref) = ir.scene_ref {
        trace = trace.scene(scene_ref);
    }

    trace
}

fn convert_bar_plot(ir: &BarPlotIR) -> Box<dyn Trace + 'static> {
    let labels = extract_strings(&ir.labels);
    let values = extract_numeric_options(&ir.values);

    let is_horizontal = ir
        .orientation
        .as_ref()
        .is_some_and(|o| matches!(o, Orientation::Horizontal));

    if is_horizontal {
        let mut trace = Bar::default()
            .y(labels)
            .x(values)
            .orientation(plotly::common::Orientation::Horizontal);

        if let Some(ref marker_ir) = ir.marker {
            trace = trace.marker(build_marker(marker_ir));
        }
        if let Some(ref name) = ir.name {
            trace = trace.name(name.as_str());
        }
        if let Some(ref error) = ir.error {
            let error_values: Vec<f64> = match error {
                ColumnData::Numeric(data) => data.iter().map(|v| v.unwrap_or(0.0) as f64).collect(),
                _ => vec![],
            };
            trace = trace.error_x(ErrorData::new(ErrorType::Data).array(error_values));
        }
        if let Some(ref subplot_ref) = ir.subplot_ref {
            let (x_axis, y_axis) = parse_subplot_axes(subplot_ref);
            trace = trace.x_axis(&x_axis).y_axis(&y_axis);
        }
        if let Some(ref group) = ir.legend_group {
            trace = trace.legend_group(group);
        }
        if let Some(show) = ir.show_legend {
            trace = trace.show_legend(show);
        }
        trace
    } else {
        let mut trace = Bar::default()
            .x(labels)
            .y(values)
            .orientation(plotly::common::Orientation::Vertical);

        if let Some(ref marker_ir) = ir.marker {
            trace = trace.marker(build_marker(marker_ir));
        }
        if let Some(ref name) = ir.name {
            trace = trace.name(name.as_str());
        }
        if let Some(ref error) = ir.error {
            let error_values: Vec<f64> = match error {
                ColumnData::Numeric(data) => data.iter().map(|v| v.unwrap_or(0.0) as f64).collect(),
                _ => vec![],
            };
            trace = trace.error_y(ErrorData::new(ErrorType::Data).array(error_values));
        }
        if let Some(ref subplot_ref) = ir.subplot_ref {
            let (x_axis, y_axis) = parse_subplot_axes(subplot_ref);
            trace = trace.x_axis(&x_axis).y_axis(&y_axis);
        }
        if let Some(ref group) = ir.legend_group {
            trace = trace.legend_group(group);
        }
        if let Some(show) = ir.show_legend {
            trace = trace.show_legend(show);
        }
        trace
    }
}

fn convert_box_plot(ir: &BoxPlotIR) -> Box<dyn Trace + 'static> {
    let labels = extract_strings(&ir.labels);
    let values = extract_numeric_options(&ir.values);

    let is_horizontal = ir
        .orientation
        .as_ref()
        .is_some_and(|o| matches!(o, Orientation::Horizontal));

    if is_horizontal {
        apply_box_common_h(*BoxPlotPlotly::default().x(values).y(labels), ir)
    } else {
        apply_box_common_v(*BoxPlotPlotly::default().x(labels).y(values), ir)
    }
}

fn apply_box_common_v(
    trace: BoxPlotPlotly<String, Option<f32>>,
    ir: &BoxPlotIR,
) -> Box<dyn Trace + 'static> {
    let mut t = *trace.orientation(plotly::common::Orientation::Vertical);
    if let Some(ref marker_ir) = ir.marker {
        t = *t.marker(build_marker(marker_ir));
    }
    if let Some(ref name) = ir.name {
        t = *t.name(name.as_str()).offset_group(name.as_str());
    }
    if let Some(bp) = ir.box_points {
        t = *t.box_points(if bp { BoxPoints::All } else { BoxPoints::False });
    }
    if let Some(po) = ir.point_offset {
        t = *t.point_pos(po);
    }
    if let Some(j) = ir.jitter {
        t = *t.jitter(j);
    }
    if let Some(ref subplot_ref) = ir.subplot_ref {
        let (x_axis, y_axis) = parse_subplot_axes(subplot_ref);
        t = *t.x_axis(&x_axis).y_axis(&y_axis);
    }
    if let Some(ref group) = ir.legend_group {
        t = *t.legend_group(group);
    }
    if let Some(show) = ir.show_legend {
        t = *t.show_legend(show);
    }
    Box::new(t)
}

fn apply_box_common_h(
    trace: BoxPlotPlotly<Option<f32>, String>,
    ir: &BoxPlotIR,
) -> Box<dyn Trace + 'static> {
    let mut t = *trace.orientation(plotly::common::Orientation::Horizontal);
    if let Some(ref marker_ir) = ir.marker {
        t = *t.marker(build_marker(marker_ir));
    }
    if let Some(ref name) = ir.name {
        t = *t.name(name.as_str()).offset_group(name.as_str());
    }
    if let Some(bp) = ir.box_points {
        t = *t.box_points(if bp { BoxPoints::All } else { BoxPoints::False });
    }
    if let Some(po) = ir.point_offset {
        t = *t.point_pos(po);
    }
    if let Some(j) = ir.jitter {
        t = *t.jitter(j);
    }
    if let Some(ref subplot_ref) = ir.subplot_ref {
        let (x_axis, y_axis) = parse_subplot_axes(subplot_ref);
        t = *t.x_axis(&x_axis).y_axis(&y_axis);
    }
    if let Some(ref group) = ir.legend_group {
        t = *t.legend_group(group);
    }
    if let Some(show) = ir.show_legend {
        t = *t.show_legend(show);
    }
    Box::new(t)
}

fn convert_histogram(ir: &HistogramIR) -> Box<dyn Trace + 'static> {
    let x_data = extract_numeric_options(&ir.x);

    let mut trace = HistogramPlotly::default()
        .x(x_data)
        .hist_func(HistFunc::Count);

    if let Some(ref marker_ir) = ir.marker {
        trace = trace.marker(build_marker(marker_ir));
    }

    if let Some(ref bins_ir) = ir.bins {
        trace = trace.x_bins(Bins::new(bins_ir.start, bins_ir.end, bins_ir.size));
    }

    if let Some(ref name) = ir.name {
        trace = trace.name(name.as_str());
    }

    if let Some(ref subplot_ref) = ir.subplot_ref {
        let (x_axis, y_axis) = parse_subplot_axes(subplot_ref);
        trace = trace.x_axis(&x_axis).y_axis(&y_axis);
    }

    if let Some(ref group) = ir.legend_group {
        trace = trace.legend_group(group);
    }

    if let Some(show) = ir.show_legend {
        trace = trace.show_legend(show);
    }

    trace
}

fn extract_numeric_as_f64(col: &ColumnData) -> Vec<f64> {
    match col {
        ColumnData::Numeric(v) => v.iter().map(|o| o.unwrap_or(0.0) as f64).collect(),
        _ => vec![],
    }
}

fn convert_contour_plot(ir: &ContourPlotIR) -> Box<dyn Trace + 'static> {
    let x_data = extract_numeric_options(&ir.x);
    let y_data = extract_numeric_options(&ir.y);
    let z_data = extract_numeric_options(&ir.z);

    let mut trace = ContourPlotly::new(x_data, y_data, z_data);

    if let Some(ref cs) = ir.color_scale {
        trace = trace.color_scale(conv::convert_palette(cs));
    }
    if let Some(ref cb) = ir.color_bar {
        trace = trace.color_bar(conv::convert_colorbar(cb));
    }
    if let Some(rs) = ir.reverse_scale {
        trace = trace.reverse_scale(rs);
    }
    if let Some(ss) = ir.show_scale {
        trace = trace.show_scale(ss);
    }

    let mut contours = ContoursPlotly::new();
    if let Some(ref coloring) = ir.coloring {
        contours = contours.coloring(conv::convert_coloring(coloring));
    }
    if let Some(show) = ir.show_lines {
        contours = contours.show_lines(show);
    }
    trace = trace.contours(contours);

    if let Some(z_min) = ir.z_min {
        trace = trace.zmin(Some(z_min as f32));
    }
    if let Some(z_max) = ir.z_max {
        trace = trace.zmax(Some(z_max as f32));
    }

    if let Some(ref subplot_ref) = ir.subplot_ref {
        let (x_axis, y_axis) = parse_subplot_axes(subplot_ref);
        trace = trace.x_axis(&x_axis).y_axis(&y_axis);
    }

    trace
}

fn convert_heat_map(ir: &HeatMapIR) -> Box<dyn Trace + 'static> {
    let x_data = extract_strings(&ir.x);
    let y_data = extract_strings(&ir.y);
    let z_data = extract_numeric_options(&ir.z);

    let mut trace = HeatMapPlotly::default().x(x_data).y(y_data).z(z_data);

    if let Some(auto_cs) = ir.auto_color_scale {
        trace = trace.auto_color_scale(auto_cs);
    }
    if let Some(ref cs) = ir.color_scale {
        trace = trace.color_scale(conv::convert_palette(cs));
    }
    if let Some(ref cb) = ir.color_bar {
        trace = trace.color_bar(conv::convert_colorbar(cb));
    }
    if let Some(rs) = ir.reverse_scale {
        trace = trace.reverse_scale(rs);
    }
    if let Some(ss) = ir.show_scale {
        trace = trace.show_scale(ss);
    }
    if let Some(z_min) = ir.z_min {
        trace = trace.zmin(z_min);
    }
    if let Some(z_max) = ir.z_max {
        trace = trace.zmax(z_max);
    }

    if let Some(ref subplot_ref) = ir.subplot_ref {
        let (x_axis, y_axis) = parse_subplot_axes(subplot_ref);
        trace = trace.x_axis(&x_axis).y_axis(&y_axis);
    }

    trace
}

fn convert_pie_chart(ir: &PieChartIR) -> Box<dyn Trace + 'static> {
    let labels: Vec<String> = match &ir.labels {
        ColumnData::String(v) => v.iter().filter_map(|s| s.clone()).collect(),
        _ => vec![],
    };

    let mut trace = Pie::<u32>::from_labels(&labels);

    if let Some(hole) = ir.hole {
        trace = trace.hole(hole);
    }
    if let Some(pull) = ir.pull {
        trace = trace.pull(pull);
    }
    if let Some(rotation) = ir.rotation {
        trace = trace.rotation(rotation);
    }

    if let Some((x0, x1)) = ir.domain_x {
        if let Some((y0, y1)) = ir.domain_y {
            trace = trace.domain(Domain::new().x(&[x0, x1]).y(&[y0, y1]));
        }
    }

    if let Some(ref colors) = ir.colors {
        let plotly_colors: Vec<RgbPlotly> = colors
            .iter()
            .map(|c| RgbPlotly::new(c.0, c.1, c.2))
            .collect();
        trace = trace.marker(MarkerPlotly::new().color_array(plotly_colors));
    }

    if let Some(ref name) = ir.name {
        trace = trace.name(name.as_str());
    }

    trace
}

fn convert_sankey_diagram(ir: &SankeyDiagramIR) -> Box<dyn Trace + 'static> {
    let label_refs: Vec<&str> = ir.node_labels.iter().map(|s| s.as_str()).collect();
    let mut node = Node::new().label(label_refs);

    if let Some(pad) = ir.pad {
        node = node.pad(pad);
    }
    if let Some(thickness) = ir.thickness {
        node = node.thickness(thickness);
    }
    if let Some(ref colors) = ir.node_colors {
        let c: Vec<RgbPlotly> = colors
            .iter()
            .map(|c| RgbPlotly::new(c.0, c.1, c.2))
            .collect();
        node = node.color_array(c);
    }

    let values = extract_numeric_as_f64(&ir.values);
    let mut link = Link::new()
        .source(ir.sources.clone())
        .target(ir.targets.clone())
        .value(values);

    if let Some(ref colors) = ir.link_colors {
        let c: Vec<RgbPlotly> = colors
            .iter()
            .map(|c| RgbPlotly::new(c.0, c.1, c.2))
            .collect();
        link = link.color_array(c);
    }

    let mut trace = Sankey::new().node(node).link(link);

    if let Some(ref orientation) = ir.orientation {
        trace = trace.orientation(conv::convert_orientation(orientation));
    }
    if let Some(ref arrangement) = ir.arrangement {
        trace = trace.arrangement(conv::convert_arrangement(arrangement));
    }

    if let Some((x0, x1)) = ir.domain_x {
        if let Some((y0, y1)) = ir.domain_y {
            trace = trace.domain(Domain::new().x(&[x0, x1]).y(&[y0, y1]));
        }
    }

    trace
}
