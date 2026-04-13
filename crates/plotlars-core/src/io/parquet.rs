use std::path::{Path, PathBuf};

use polars::frame::DataFrame;
use polars::prelude::*;

use super::PlotlarsError;

/// A Parquet file reader.
///
/// Uses a fluent builder pattern: construct with [`ParquetReader::new`], chain
/// optional configuration methods, then call [`ParquetReader::finish`] to load
/// the data into a [`DataFrame`].
///
/// # Example
///
/// ```rust,no_run
/// use plotlars_core::io::ParquetReader;
///
/// let df = ParquetReader::new("data/sales.parquet").finish().unwrap();
/// ```
#[derive(Clone)]
pub struct ParquetReader {
    path: PathBuf,
    columns: Option<Vec<String>>,
    n_rows: Option<usize>,
}

impl ParquetReader {
    /// Create a new Parquet reader for the given file path.
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
            columns: None,
            n_rows: None,
        }
    }

    /// Select specific columns to load (projection pushdown).
    pub fn columns(mut self, columns: Vec<&str>) -> Self {
        self.columns = Some(columns.into_iter().map(|s| s.to_string()).collect());
        self
    }

    /// Limit the number of rows to read.
    pub fn n_rows(mut self, n_rows: usize) -> Self {
        self.n_rows = Some(n_rows);
        self
    }

    /// Execute the read and return a [`DataFrame`].
    ///
    /// # Errors
    ///
    /// Returns [`PlotlarsError::ParquetParse`] if the file cannot be read or parsed.
    pub fn finish(self) -> Result<DataFrame, PlotlarsError> {
        let path_str = self.path.display().to_string();

        let mut args = ScanArgsParquet::default();

        if let Some(n) = self.n_rows {
            args.n_rows = Some(n);
        }

        let mut lf = LazyFrame::scan_parquet(PlRefPath::new(&path_str), args).map_err(|e| {
            PlotlarsError::ParquetParse {
                path: path_str.clone(),
                source: Box::new(e),
            }
        })?;

        if let Some(cols) = self.columns {
            let exprs: Vec<Expr> = cols.iter().map(col).collect();
            lf = lf.select(exprs);
        }

        lf.collect().map_err(|e| PlotlarsError::ParquetParse {
            path: path_str,
            source: Box::new(e),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_parquet() -> PathBuf {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../target/test_data.parquet");

        let mut df = df!(
            "a" => [1, 2, 3],
            "b" => ["x", "y", "z"]
        )
        .unwrap();

        let file = std::fs::File::create(&path).unwrap();
        ParquetWriter::new(file).finish(&mut df).unwrap();
        path
    }

    #[test]
    fn read_parquet_default() {
        let path = create_test_parquet();
        let df = ParquetReader::new(&path).finish().unwrap();
        assert_eq!(df.height(), 3);
        assert_eq!(df.width(), 2);
    }

    #[test]
    fn read_parquet_select_columns() {
        let path = create_test_parquet();
        let df = ParquetReader::new(&path)
            .columns(vec!["a"])
            .finish()
            .unwrap();
        assert_eq!(df.width(), 1);
    }

    #[test]
    fn read_parquet_n_rows() {
        let path = create_test_parquet();
        let df = ParquetReader::new(&path).n_rows(2).finish().unwrap();
        assert_eq!(df.height(), 2);
    }

    #[test]
    fn read_parquet_file_not_found() {
        let result = ParquetReader::new("nonexistent.parquet").finish();
        assert!(result.is_err());
    }
}
