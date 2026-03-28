#![allow(dead_code)]

use plotly::{
    color::Rgb as RgbPlotly,
    common::{Line as LinePlotly, Marker as MarkerPlotly},
    image::ColorModel,
    traces::table::{Cells as CellsPlotly, Header as HeaderPlotly},
    Candlestick as CandlestickPlotly, DensityMapbox as DensityMapboxPlotly,
    Image as ImagePlotly, Ohlc as OhlcPlotly, ScatterGeo as ScatterGeoPlotly,
    ScatterMapbox, Table as TablePlotly, Trace,
};

use crate::ir::data::ColumnData;
use crate::ir::marker::MarkerIR;
use crate::ir::trace::{
    Array2dPlotIR, CandlestickPlotIR, DensityMapboxIR, ImageIR, OhlcPlotIR, ScatterGeoIR,
    ScatterMapIR, TableIR, TraceIR,
};

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
        marker = marker.symbol(shape.to_plotly());
    }

    marker
}

fn build_line(line_ir: &crate::ir::line::LineIR) -> LinePlotly {
    let mut line = LinePlotly::new();

    if let Some(width) = line_ir.width {
        line = line.width(width);
    }

    if let Some(ref color) = line_ir.color {
        line = line.color(RgbPlotly::new(color.0, color.1, color.2));
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
        trace = trace.increasing(inc.to_plotly_increasing());
    }

    if let Some(ref dec) = ir.decreasing {
        trace = trace.decreasing(dec.to_plotly_decreasing());
    }

    if let Some(whisker_w) = ir.whisker_width {
        trace = trace.whisker_width(whisker_w);
    }

    Box::new(trace)
}

fn convert_table(ir: &TableIR) -> Box<dyn Trace + 'static> {
    let plotly_header = if let Some(ref h) = ir.header {
        h.to_plotly(ir.column_names.clone())
    } else {
        HeaderPlotly::new(ir.column_names.clone())
    };

    let plotly_cells = if let Some(ref c) = ir.cell {
        c.to_plotly(ir.column_data.clone())
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
        trace = trace.mode(mode.to_plotly());
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
