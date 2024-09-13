use plotly::{
    common::{Line as LinePlotly, Marker},
    Trace as TracePlotly,
};

use polars::frame::DataFrame;

use crate::{
    aesthetics::{line::Line, mark::Mark},
    LineType, Orientation, Rgb, Shape,
};

use crate::traits::polar::Polar;

pub(crate) trait Trace: Polar + Mark + Line {
    #[allow(clippy::too_many_arguments)]
    fn create_trace(
        data: &DataFrame,
        x_col: &str,
        y_col: &str,
        orientation: Option<Orientation>,
        group_name: Option<&str>,
        error: Option<String>,
        box_points: Option<bool>,
        point_offset: Option<f64>,
        jitter: Option<f64>,
        with_shape: Option<bool>,
        marker: Marker,
        line: LinePlotly,
    ) -> Box<dyn TracePlotly + 'static>;

    #[allow(clippy::too_many_arguments)]
    fn create_traces(
        data: &DataFrame,
        x_col: &str,
        y_col: &str,
        orientation: Option<Orientation>,
        group: Option<String>,
        error: Option<String>,
        box_points: Option<bool>,
        point_offset: Option<f64>,
        jitter: Option<f64>,
        #[allow(unused_variables)] additional_series: Option<Vec<&str>>,
        opacity: Option<f64>,
        size: Option<usize>,
        color: Option<Rgb>,
        colors: Option<Vec<Rgb>>,
        with_shape: Option<bool>,
        shape: Option<Shape>,
        shapes: Option<Vec<Shape>>,
        line_types: Option<Vec<LineType>>,
        line_width: Option<f64>,
    ) -> Vec<Box<dyn TracePlotly + 'static>> {
        let mark = Self::create_marker(opacity, size);
        let mut line = Self::create_line();

        let mut traces: Vec<Box<dyn TracePlotly + 'static>> = Vec::new();

        match group {
            Some(group) => {
                let group_col = group.as_str();

                let unique_groups = Self::get_unique_groups(data, group_col);

                let groups = unique_groups.iter().map(|s| s.as_str());

                for (i, group_name) in groups.enumerate() {
                    let group_mark = Self::set_color(&mark, &color, &colors, i);
                    let group_mark = Self::set_shape(&group_mark, &shape, &shapes, i);

                    line = Self::set_line_type(&line, &line_types, line_width, i);

                    let subset = Self::filter_data_by_group(data, group_col, group_name);

                    let trace = Self::create_trace(
                        &subset,
                        x_col,
                        y_col,
                        orientation.clone(),
                        Some(group_name),
                        error.clone(),
                        box_points,
                        point_offset,
                        jitter,
                        with_shape,
                        group_mark,
                        line.clone(),
                    );
                    traces.push(trace);
                }
            }
            None => {
                let group_name = None;
                let mut mark = mark.clone();

                mark = Self::set_color(&mark, &color, &colors, 0);
                mark = Self::set_shape(&mark, &shape, &shapes, 0);

                let trace = Self::create_trace(
                    data,
                    x_col,
                    y_col,
                    orientation,
                    group_name,
                    error,
                    box_points,
                    point_offset,
                    jitter,
                    with_shape,
                    mark,
                    line,
                );

                traces.push(trace);
            }
        }

        traces
    }
}
