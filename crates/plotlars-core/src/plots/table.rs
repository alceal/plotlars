use bon::bon;

use polars::frame::DataFrame;

use crate::{
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
///     .font("Arial Black")
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
#[derive(Clone)]
#[allow(dead_code)]
pub struct Table {
    traces: Vec<TraceIR>,
    layout: LayoutIR,
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
            let col_data = crate::data::get_string_column(data, column_name);
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
        let traces = vec![ir_trace];
        let layout = LayoutIR {
            title: plot_title.clone(),
            x_title: None,
            y_title: None,
            y2_title: None,
            z_title: None,
            legend_title: None,
            legend: None,
            dimensions: None,
            bar_mode: None,
            box_mode: None,
            box_gap: None,
            margin_bottom: None,
            axes_2d: None,
            scene_3d: None,
            polar: None,
            mapbox: None,
            grid: None,
            annotations: vec![],
        };
        Self { traces, layout }
    }
}

impl crate::Plot for Table {
    fn ir_traces(&self) -> &[TraceIR] {
        &self.traces
    }

    fn ir_layout(&self) -> &LayoutIR {
        &self.layout
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Plot;
    use polars::prelude::*;

    #[test]
    fn test_basic_one_trace() {
        let df = df![
            "name" => ["Alice", "Bob"],
            "age" => [30, 25]
        ]
        .unwrap();
        let plot = Table::builder()
            .data(&df)
            .columns(vec!["name", "age"])
            .build();
        assert_eq!(plot.ir_traces().len(), 1);
    }

    #[test]
    fn test_trace_variant() {
        let df = df![
            "col1" => ["a", "b"],
            "col2" => ["c", "d"]
        ]
        .unwrap();
        let plot = Table::builder()
            .data(&df)
            .columns(vec!["col1", "col2"])
            .build();
        assert!(matches!(plot.ir_traces()[0], TraceIR::Table(_)));
    }

    #[test]
    fn test_layout_no_axes() {
        let df = df![
            "col1" => ["a"]
        ]
        .unwrap();
        let plot = Table::builder().data(&df).columns(vec!["col1"]).build();
        let layout = plot.ir_layout();
        assert!(layout.axes_2d.is_none());
        assert!(layout.scene_3d.is_none());
        assert!(layout.polar.is_none());
    }
}
