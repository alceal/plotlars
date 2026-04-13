use std::path::{Path, PathBuf};

use polars::frame::DataFrame;
use polars::prelude::*;

use super::PlotlarsError;

/// An NDJSON (newline-delimited JSON) file reader.
///
/// Reads files where each line is a valid JSON object, which maps naturally
/// to tabular data. Available when the `format-json` feature is enabled.
///
/// # Example
///
/// ```rust,no_run
/// use plotlars_core::io::JsonReader;
///
/// let df = JsonReader::new("data/events.jsonl").finish().unwrap();
/// ```
#[derive(Clone)]
pub struct JsonReader {
    path: PathBuf,
    n_rows: Option<usize>,
}

impl JsonReader {
    /// Create a new JSON reader for the given file path.
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
            n_rows: None,
        }
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
    /// Returns [`PlotlarsError::JsonParse`] if the file cannot be read or parsed.
    pub fn finish(self) -> Result<DataFrame, PlotlarsError> {
        let path_str = self.path.display().to_string();

        let reader = LazyJsonLineReader::new(PlRefPath::new(&path_str)).with_n_rows(self.n_rows);

        let lf = reader.finish().map_err(|e| PlotlarsError::JsonParse {
            path: path_str.clone(),
            source: Box::new(e),
        })?;

        lf.collect().map_err(|e| PlotlarsError::JsonParse {
            path: path_str,
            source: Box::new(e),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_jsonl() -> PathBuf {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../target/test_data.jsonl");

        std::fs::write(
            &path,
            "{\"a\":1,\"b\":\"x\"}\n{\"a\":2,\"b\":\"y\"}\n{\"a\":3,\"b\":\"z\"}\n",
        )
        .unwrap();
        path
    }

    #[test]
    fn read_json_default() {
        let path = create_test_jsonl();
        let df = JsonReader::new(&path).finish().unwrap();
        assert_eq!(df.height(), 3);
        assert_eq!(df.width(), 2);
    }

    #[test]
    fn read_json_n_rows() {
        let path = create_test_jsonl();
        let df = JsonReader::new(&path).n_rows(2).finish().unwrap();
        assert_eq!(df.height(), 2);
    }

    #[test]
    fn read_json_file_not_found() {
        let result = JsonReader::new("nonexistent.jsonl").finish();
        assert!(result.is_err());
    }
}
