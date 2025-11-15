use bon::bon;

use plotly::{
    traces::table::{Cells as CellsPlotly, Header as HeaderPlotly},
    Layout as LayoutPlotly, Table as TablePlotly, Trace,
};

use polars::frame::DataFrame;
use serde::Serialize;

use crate::{
    common::{Layout, PlotHelper, Polar},
    components::{Cell, Header, Text},
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
/// let dataset = LazyCsvReader::new(PlPath::new("data/employee_data.csv"))
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
pub struct Table {
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

        let traces = Self::create_traces(data, &columns, header, cell, column_width);

        Self { traces, layout }
    }

    fn create_traces(
        data: &DataFrame,
        columns: &[&str],
        header: Option<&Header>,
        cell: Option<&Cell>,
        column_width: Option<f64>,
    ) -> Vec<Box<dyn Trace + 'static>> {
        let mut traces: Vec<Box<dyn Trace + 'static>> = Vec::new();

        let trace = Self::create_trace(data, columns, header, cell, column_width);

        traces.push(trace);
        traces
    }

    fn create_trace(
        data: &DataFrame,
        columns: &[&str],
        header: Option<&Header>,
        cell: Option<&Cell>,
        column_width: Option<f64>,
    ) -> Box<dyn Trace + 'static> {
        // Determine header values
        let header_values = if let Some(h) = header {
            if let Some(custom_values) = &h.values {
                custom_values.clone()
            } else {
                columns.iter().map(|&c| c.to_string()).collect()
            }
        } else {
            columns.iter().map(|&c| c.to_string()).collect()
        };

        // Extract cell values from DataFrame
        let mut cell_values: Vec<Vec<String>> = Vec::new();

        for column_name in columns {
            let column_data = Self::get_string_column(data, column_name);
            let column_strings: Vec<String> = column_data
                .iter()
                .map(|opt| opt.clone().unwrap_or_default())
                .collect();
            cell_values.push(column_strings);
        }

        // Create header
        let plotly_header = if let Some(h) = header {
            h.to_plotly(header_values)
        } else {
            HeaderPlotly::new(header_values)
        };

        // Create cells
        let plotly_cells = if let Some(c) = cell {
            c.to_plotly(cell_values)
        } else {
            CellsPlotly::new(cell_values)
        };

        // Create table
        let mut table = TablePlotly::new(plotly_header, plotly_cells);

        // Set column width if provided
        if let Some(width) = column_width {
            table = table.column_width(width);
        }

        table
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
