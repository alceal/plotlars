use bon::bon;

use plotly::{Layout as LayoutPlotly, Trace};

use polars::frame::DataFrame;
use serde::Serialize;

use crate::{
    common::{Layout, PlotHelper, Polar},
    components::{Cell, Header, Text},
    ir::layout::LayoutIR,
    ir::trace::{TableIR, TraceIR},
};

/// A structure representing a table plot.
///
/// The `Table` struct allows for the creation and customization of tables with support
/// for custom headers, cell formatting, column widths, and various styling options.
///
/// # Arguments
///
/// * `data` - A reference to the `DataFrame` containing the data to be displayed.
/// * `columns` - A vector of column names to be displayed in the table.
/// * `header` - An optional `Header` component for custom header values and formatting.
/// * `cell` - An optional `Cell` component for cell formatting.
/// * `column_width` - An optional column width ratio. Columns fill the available width in proportion.
/// * `plot_title` - An optional `Text` struct specifying the title of the plot.
///
/// # Example
///
/// ```rust
/// use polars::prelude::*;
/// use plotlars::{Table, Header, Cell, Plot, Text, Rgb};
///
/// let dataset = LazyCsvReader::new(PlRefPath::new("data/employee_data.csv"))
///     .finish()
///     .unwrap()
///     .collect()
///     .unwrap();
///
/// let header = Header::new()
///     .values(vec![
///          "Employee Name",
///          "Department",
///          "Annual Salary ($)",
///          "Years of Service",
///     ])
///     .align("center")
///     .font("Arial Bold")
///     .fill(Rgb(70, 130, 180));
///
/// let cell = Cell::new()
///     .align("center")
///     .height(25.0)
///     .font("Arial")
///     .fill(Rgb(240, 248, 255));
///
/// Table::builder()
///     .data(&dataset)
///     .columns(vec![
///         "name",
///         "department",
///         "salary",
///         "years",
///     ])
///     .header(&header)
///     .cell(&cell)
///     .plot_title(
///         Text::from("Table")
///             .font("Arial")
///             .size(20)
///             .color(Rgb(25, 25, 112))
///     )
///     .build()
///     .plot();
/// ```
///
/// ![Example](https://imgur.com/QDKTeFX.png)
#[derive(Clone, Serialize)]
#[allow(dead_code)]
pub struct Table {
    #[serde(skip)]
    ir_traces: Vec<TraceIR>,
    #[serde(skip)]
    ir_layout: LayoutIR,
    traces: Vec<Box<dyn Trace + 'static>>,
    layout: LayoutPlotly,
}

#[bon]
impl Table {
    #[builder(on(String, into), on(Text, into))]
    pub fn new(
        data: &DataFrame,
        columns: Vec<&str>,
        header: Option<&Header>,
        cell: Option<&Cell>,
        column_width: Option<f64>,
        plot_title: Option<Text>,
    ) -> Self {
        let x_title = None;
        let y_title = None;
        let y2_title = None;
        let z_title = None;
        let legend_title = None;
        let x_axis = None;
        let y_axis = None;
        let y2_axis = None;
        let z_axis = None;
        let legend = None;

        // Determine column names
        let column_names: Vec<String> = if let Some(h) = header {
            if let Some(custom_values) = &h.values {
                custom_values.clone()
            } else {
                columns.iter().map(|&c| c.to_string()).collect()
            }
        } else {
            columns.iter().map(|&c| c.to_string()).collect()
        };

        // Extract cell values from DataFrame
        let mut column_data: Vec<Vec<String>> = Vec::new();
        for column_name in &columns {
            let col_data = Self::get_string_column(data, column_name);
            let col_strings: Vec<String> = col_data
                .iter()
                .map(|opt| opt.clone().unwrap_or_default())
                .collect();
            column_data.push(col_strings);
        }

        // Build IR
        let ir_trace = TraceIR::Table(TableIR {
            header: header.cloned(),
            cell: cell.cloned(),
            column_names,
            column_data,
            column_width,
        });
        let ir_traces = vec![ir_trace];
        let ir_layout = LayoutIR {
            title: plot_title.clone(),
            x_title: None,
            y_title: None,
            y2_title: None,
            z_title: None,
            legend_title: None,
            legend: None,
            dimensions: None,
            bar_mode: None,
            axes_2d: None,
            scene_3d: None,
            polar: None,
            mapbox: None,
            grid: None,
            annotations: vec![],
        };

        // Build plotly types from IR
        let plotly_traces: Vec<Box<dyn Trace + 'static>> = ir_traces
            .iter()
            .map(crate::plotly_conversions::trace::convert)
            .collect();

        let layout = Self::create_layout(
            plot_title,
            x_title,
            y_title,
            y2_title,
            z_title,
            legend_title,
            x_axis,
            y_axis,
            y2_axis,
            z_axis,
            legend,
            None,
        );

        Self {
            ir_traces,
            ir_layout,
            traces: plotly_traces,
            layout,
        }
    }
}

impl Layout for Table {}
impl Polar for Table {}

impl PlotHelper for Table {
    fn get_layout(&self) -> &LayoutPlotly {
        &self.layout
    }

    fn get_traces(&self) -> &Vec<Box<dyn Trace + 'static>> {
        &self.traces
    }
}
