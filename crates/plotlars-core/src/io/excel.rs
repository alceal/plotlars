use std::path::{Path, PathBuf};

use calamine::{open_workbook, Data, Reader, Xlsx};
use polars::frame::DataFrame;
use polars::prelude::*;

use super::PlotlarsError;

/// An Excel file reader supporting `.xlsx` format.
///
/// Uses the `calamine` crate for parsing. Available when the `format-excel`
/// feature is enabled.
///
/// # Example
///
/// ```rust,no_run
/// use plotlars_core::io::ExcelReader;
///
/// let df = ExcelReader::new("data/report.xlsx")
///     .sheet("Q1 Sales")
///     .finish()
///     .unwrap();
/// ```
#[derive(Clone)]
pub struct ExcelReader {
    path: PathBuf,
    sheet: Option<String>,
    has_header: Option<bool>,
    skip_rows: Option<usize>,
}

impl ExcelReader {
    /// Create a new Excel reader for the given file path.
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
            sheet: None,
            has_header: None,
            skip_rows: None,
        }
    }

    /// Select the sheet to read by name. Defaults to the first sheet.
    pub fn sheet(mut self, sheet: &str) -> Self {
        self.sheet = Some(sheet.to_string());
        self
    }

    /// Set whether the first row is a header row. Defaults to `true`.
    pub fn has_header(mut self, has_header: bool) -> Self {
        self.has_header = Some(has_header);
        self
    }

    /// Set the number of rows to skip before reading data.
    pub fn skip_rows(mut self, skip_rows: usize) -> Self {
        self.skip_rows = Some(skip_rows);
        self
    }

    /// Execute the read and return a [`DataFrame`].
    ///
    /// # Errors
    ///
    /// Returns [`PlotlarsError::ExcelParse`] if the file cannot be read or parsed.
    pub fn finish(self) -> Result<DataFrame, PlotlarsError> {
        let path_str = self.path.display().to_string();
        let has_header = self.has_header.unwrap_or(true);
        let skip_rows = self.skip_rows.unwrap_or(0);

        let mut workbook: Xlsx<_> =
            open_workbook(&self.path).map_err(|e| PlotlarsError::ExcelParse {
                path: path_str.clone(),
                source: Box::new(e),
            })?;

        let sheet_name = match &self.sheet {
            Some(name) => name.clone(),
            None => workbook.sheet_names().first().cloned().ok_or_else(|| {
                PlotlarsError::ExcelParse {
                    path: path_str.clone(),
                    source: "workbook contains no sheets".into(),
                }
            })?,
        };

        let range =
            workbook
                .worksheet_range(&sheet_name)
                .map_err(|e| PlotlarsError::ExcelParse {
                    path: path_str.clone(),
                    source: Box::new(e),
                })?;

        let rows: Vec<_> = range.rows().skip(skip_rows).collect();

        if rows.is_empty() {
            return Ok(DataFrame::empty());
        }

        let (header_row, data_rows) = if has_header {
            let headers: Vec<String> = rows[0]
                .iter()
                .enumerate()
                .map(|(i, cell)| match cell {
                    Data::String(s) => s.clone(),
                    _ => format!("column_{}", i),
                })
                .collect();
            (headers, &rows[1..])
        } else {
            let headers: Vec<String> = (0..rows[0].len())
                .map(|i| format!("column_{}", i))
                .collect();
            (headers, &rows[..])
        };

        let num_cols = header_row.len();
        let series: Vec<Column> = (0..num_cols)
            .map(|col_idx| {
                let col_name = &header_row[col_idx];
                let values: Vec<AnyValue> = data_rows
                    .iter()
                    .map(|row| {
                        if col_idx >= row.len() {
                            return AnyValue::Null;
                        }
                        match &row[col_idx] {
                            Data::Int(i) => AnyValue::Float64(*i as f64),
                            Data::Float(f) => AnyValue::Float64(*f),
                            Data::String(s) => AnyValue::StringOwned(s.clone().into()),
                            Data::Bool(b) => AnyValue::Boolean(*b),
                            Data::Empty => AnyValue::Null,
                            _ => AnyValue::Null,
                        }
                    })
                    .collect();
                Series::from_any_values(col_name.into(), &values, false)
                    .unwrap()
                    .into()
            })
            .collect();

        let n_rows = data_rows.len();
        DataFrame::new(n_rows, series).map_err(|e| PlotlarsError::ExcelParse {
            path: path_str,
            source: Box::new(e),
        })
    }
}
