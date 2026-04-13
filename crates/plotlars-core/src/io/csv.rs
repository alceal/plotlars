use std::path::{Path, PathBuf};

use polars::frame::DataFrame;
use polars::prelude::*;

use super::PlotlarsError;

/// A CSV file reader with configurable parsing options.
///
/// Uses a fluent builder pattern: construct with [`CsvReader::new`], chain
/// optional configuration methods, then call [`CsvReader::finish`] to load
/// the data into a [`DataFrame`].
///
/// # Example
///
/// ```rust,no_run
/// use plotlars_core::io::CsvReader;
///
/// let df = CsvReader::new("data/penguins.csv")
///     .has_header(true)
///     .try_parse_dates(true)
///     .finish()
///     .unwrap();
/// ```
#[derive(Clone)]
pub struct CsvReader {
    path: PathBuf,
    delimiter: Option<u8>,
    has_header: Option<bool>,
    skip_rows: Option<usize>,
    null_values: Option<Vec<String>>,
    try_parse_dates: Option<bool>,
}

impl CsvReader {
    /// Create a new CSV reader for the given file path.
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
            delimiter: None,
            has_header: None,
            skip_rows: None,
            null_values: None,
            try_parse_dates: None,
        }
    }

    /// Set the column delimiter byte. Defaults to `b','`.
    pub fn delimiter(mut self, delimiter: u8) -> Self {
        self.delimiter = Some(delimiter);
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

    /// Set strings that should be interpreted as null values.
    pub fn null_values(mut self, null_values: Vec<&str>) -> Self {
        self.null_values = Some(null_values.into_iter().map(|s| s.to_string()).collect());
        self
    }

    /// Attempt to automatically parse date and datetime columns.
    pub fn try_parse_dates(mut self, try_parse_dates: bool) -> Self {
        self.try_parse_dates = Some(try_parse_dates);
        self
    }

    /// Execute the read and return a [`DataFrame`].
    ///
    /// # Errors
    ///
    /// Returns [`PlotlarsError::Io`] if the file cannot be opened, or
    /// [`PlotlarsError::CsvParse`] if the CSV data cannot be parsed.
    pub fn finish(self) -> Result<DataFrame, PlotlarsError> {
        let path_str = self.path.display().to_string();

        let mut options =
            CsvReadOptions::default().with_has_header(self.has_header.unwrap_or(true));

        if let Some(skip) = self.skip_rows {
            options = options.with_skip_rows(skip);
        }

        let mut parse_options = CsvParseOptions::default();

        if let Some(delim) = self.delimiter {
            parse_options = parse_options.with_separator(delim);
        }

        if let Some(nulls) = self.null_values {
            let nulls: Vec<PlSmallStr> = nulls.into_iter().map(PlSmallStr::from).collect();
            parse_options = parse_options.with_null_values(Some(NullValues::AllColumns(nulls)));
        }

        if let Some(try_dates) = self.try_parse_dates {
            parse_options = parse_options.with_try_parse_dates(try_dates);
        }

        options = options.with_parse_options(parse_options);

        options
            .try_into_reader_with_file_path(Some(self.path))
            .map_err(|e| PlotlarsError::CsvParse {
                path: path_str.clone(),
                source: Box::new(e),
            })?
            .finish()
            .map_err(|e| PlotlarsError::CsvParse {
                path: path_str,
                source: Box::new(e),
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn data_path(name: &str) -> String {
        format!("{}/../../data/{}", env!("CARGO_MANIFEST_DIR"), name)
    }

    #[test]
    fn read_csv_default() {
        let df = CsvReader::new(data_path("penguins.csv")).finish().unwrap();
        assert!(df.height() > 0);
        assert!(df.width() > 0);
    }

    #[test]
    fn read_csv_with_options() {
        let df = CsvReader::new(data_path("penguins.csv"))
            .has_header(true)
            .try_parse_dates(false)
            .finish()
            .unwrap();
        assert!(df.height() > 0);
    }

    #[test]
    fn read_csv_file_not_found() {
        let result = CsvReader::new("nonexistent.csv").finish();
        assert!(result.is_err());
    }

    #[test]
    fn read_csv_custom_delimiter() {
        let df = CsvReader::new(data_path("penguins.csv"))
            .delimiter(b',')
            .finish()
            .unwrap();
        assert!(df.height() > 0);
    }

    #[test]
    fn read_csv_skip_rows() {
        let df_full = CsvReader::new(data_path("animal_statistics.csv"))
            .finish()
            .unwrap();
        let df_skip = CsvReader::new(data_path("animal_statistics.csv"))
            .skip_rows(2)
            .finish()
            .unwrap();
        assert_eq!(df_full.height() - 2, df_skip.height());
    }

    #[test]
    fn read_csv_null_values() {
        let df = CsvReader::new(data_path("penguins.csv"))
            .null_values(vec!["NA", ""])
            .finish()
            .unwrap();
        assert!(df.height() > 0);
    }
}
