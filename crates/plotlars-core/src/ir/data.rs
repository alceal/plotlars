#[derive(Clone)]
#[doc(hidden)]
pub enum ColumnData {
    Numeric(Vec<Option<f32>>),
    String(Vec<Option<String>>),
    DateTime(Vec<Option<i64>>),
}
