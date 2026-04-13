use std::fmt;

/// Unified error type for plotlars I/O and plot construction failures.
///
/// This enum is `#[non_exhaustive]` -- new variants may be added in minor
/// releases without breaking downstream `match` arms that include a wildcard.
///
/// # Example
///
/// ```rust,no_run
/// use plotlars_core::io::{CsvReader, PlotlarsError};
///
/// fn main() -> Result<(), PlotlarsError> {
///     let df = CsvReader::new("data/penguins.csv").finish()?;
///     Ok(())
/// }
/// ```
#[derive(Debug)]
#[non_exhaustive]
pub enum PlotlarsError {
    /// File not found, permission denied, or other I/O error.
    Io {
        path: String,
        source: std::io::Error,
    },

    /// CSV parsing failure.
    CsvParse {
        path: String,
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    /// Parquet parsing failure.
    ParquetParse {
        path: String,
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    /// JSON (NDJSON) parsing failure.
    #[cfg(feature = "format-json")]
    JsonParse {
        path: String,
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    /// Excel parsing failure.
    #[cfg(feature = "format-excel")]
    ExcelParse {
        path: String,
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    /// A required column was not found in the DataFrame.
    ColumnNotFound {
        column: String,
        available: Vec<String>,
    },

    /// A column's data type did not match what the plot expected.
    TypeMismatch {
        column: String,
        expected: String,
        actual: String,
    },

    /// Plot construction failure.
    PlotBuild { message: String },
}

impl fmt::Display for PlotlarsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io { path, source } => {
                write!(f, "I/O error reading '{}': {}", path, source)
            }
            Self::CsvParse { path, source } => {
                write!(f, "failed to parse CSV '{}': {}", path, source)
            }
            Self::ParquetParse { path, source } => {
                write!(f, "failed to parse Parquet '{}': {}", path, source)
            }
            #[cfg(feature = "format-json")]
            Self::JsonParse { path, source } => {
                write!(f, "failed to parse JSON '{}': {}", path, source)
            }
            #[cfg(feature = "format-excel")]
            Self::ExcelParse { path, source } => {
                write!(f, "failed to parse Excel '{}': {}", path, source)
            }
            Self::ColumnNotFound { column, available } => {
                write!(
                    f,
                    "column '{}' not found; available columns: [{}]",
                    column,
                    available.join(", ")
                )
            }
            Self::TypeMismatch {
                column,
                expected,
                actual,
            } => {
                write!(
                    f,
                    "column '{}': expected type {}, found {}",
                    column, expected, actual
                )
            }
            Self::PlotBuild { message } => {
                write!(f, "plot construction error: {}", message)
            }
        }
    }
}

impl std::error::Error for PlotlarsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io { source, .. } => Some(source),
            Self::CsvParse { source, .. } => Some(source.as_ref()),
            Self::ParquetParse { source, .. } => Some(source.as_ref()),
            #[cfg(feature = "format-json")]
            Self::JsonParse { source, .. } => Some(source.as_ref()),
            #[cfg(feature = "format-excel")]
            Self::ExcelParse { source, .. } => Some(source.as_ref()),
            Self::ColumnNotFound { .. } | Self::TypeMismatch { .. } | Self::PlotBuild { .. } => {
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_io_error() {
        let err = PlotlarsError::Io {
            path: "test.csv".to_string(),
            source: std::io::Error::new(std::io::ErrorKind::NotFound, "not found"),
        };
        assert!(err.to_string().contains("test.csv"));
        assert!(err.to_string().contains("not found"));
    }

    #[test]
    fn display_csv_parse_error() {
        let err = PlotlarsError::CsvParse {
            path: "bad.csv".to_string(),
            source: "invalid data".into(),
        };
        assert!(err.to_string().contains("bad.csv"));
    }

    #[test]
    fn display_column_not_found() {
        let err = PlotlarsError::ColumnNotFound {
            column: "missing".to_string(),
            available: vec!["a".to_string(), "b".to_string()],
        };
        let msg = err.to_string();
        assert!(msg.contains("missing"));
        assert!(msg.contains("a, b"));
    }

    #[test]
    fn display_type_mismatch() {
        let err = PlotlarsError::TypeMismatch {
            column: "x".to_string(),
            expected: "Float64".to_string(),
            actual: "String".to_string(),
        };
        let msg = err.to_string();
        assert!(msg.contains("Float64"));
        assert!(msg.contains("String"));
    }

    #[test]
    fn error_source_chain() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "gone");
        let err = PlotlarsError::Io {
            path: "x".to_string(),
            source: io_err,
        };
        assert!(std::error::Error::source(&err).is_some());

        let err = PlotlarsError::ColumnNotFound {
            column: "x".to_string(),
            available: vec![],
        };
        assert!(std::error::Error::source(&err).is_none());
    }
}
