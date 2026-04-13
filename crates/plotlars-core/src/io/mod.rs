mod error;
pub use error::PlotlarsError;

mod csv;
pub use self::csv::CsvReader;

mod parquet;
pub use parquet::ParquetReader;

#[cfg(feature = "format-json")]
mod json;
#[cfg(feature = "format-json")]
pub use json::JsonReader;

#[cfg(feature = "format-excel")]
mod excel;
#[cfg(feature = "format-excel")]
pub use excel::ExcelReader;
