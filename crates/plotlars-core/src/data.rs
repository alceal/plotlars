use polars::{
    frame::DataFrame,
    prelude::{col, lit, DataType, IntoLazy},
};

use crate::io::PlotlarsError;

#[doc(hidden)]
pub fn get_unique_groups(
    data: &DataFrame,
    group_col: &str,
    sort_groups_by: Option<fn(&str, &str) -> std::cmp::Ordering>,
) -> Vec<String> {
    let unique_groups = data
        .column(group_col)
        .unwrap()
        .unique()
        .unwrap()
        .cast(&DataType::String)
        .unwrap();

    let mut groups: Vec<String> = unique_groups
        .str()
        .unwrap()
        .iter()
        .map(|x| x.unwrap().to_string())
        .collect();

    if let Some(sort_fn) = sort_groups_by {
        groups.sort_by(|a, b| sort_fn(a, b));
    } else {
        groups.sort();
    }

    groups
}

pub(crate) fn filter_data_by_group(
    data: &DataFrame,
    group_col: &str,
    group_name: &str,
) -> DataFrame {
    data.clone()
        .lazy()
        .filter(col(group_col).cast(DataType::String).eq(lit(group_name)))
        .collect()
        .unwrap()
}

pub(crate) fn get_numeric_column(data: &DataFrame, column_name: &str) -> Vec<Option<f32>> {
    try_get_numeric_column(data, column_name).unwrap()
}

pub(crate) fn get_string_column(data: &DataFrame, column_name: &str) -> Vec<Option<String>> {
    try_get_string_column(data, column_name).unwrap()
}

pub(crate) fn try_get_numeric_column(
    data: &DataFrame,
    column_name: &str,
) -> Result<Vec<Option<f32>>, PlotlarsError> {
    let column = data
        .column(column_name)
        .map_err(|_| PlotlarsError::ColumnNotFound {
            column: column_name.to_string(),
            available: data
                .get_column_names()
                .iter()
                .map(|s| s.to_string())
                .collect(),
        })?;

    let casted =
        column
            .clone()
            .cast(&DataType::Float32)
            .map_err(|_| PlotlarsError::TypeMismatch {
                column: column_name.to_string(),
                expected: "numeric".to_string(),
                actual: column.dtype().to_string(),
            })?;

    Ok(casted.f32().unwrap().to_vec())
}

pub(crate) fn try_get_string_column(
    data: &DataFrame,
    column_name: &str,
) -> Result<Vec<Option<String>>, PlotlarsError> {
    let column = data
        .column(column_name)
        .map_err(|_| PlotlarsError::ColumnNotFound {
            column: column_name.to_string(),
            available: data
                .get_column_names()
                .iter()
                .map(|s| s.to_string())
                .collect(),
        })?;

    let casted =
        column
            .clone()
            .cast(&DataType::String)
            .map_err(|_| PlotlarsError::TypeMismatch {
                column: column_name.to_string(),
                expected: "string-castable".to_string(),
                actual: column.dtype().to_string(),
            })?;

    Ok(casted
        .str()
        .unwrap()
        .iter()
        .map(|x| x.map(|s| s.to_string()))
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use polars::prelude::*;

    #[test]
    fn test_get_unique_groups_sorted() {
        let df = df!["g" => ["b", "a", "c", "a"]].unwrap();
        let result = get_unique_groups(&df, "g", None);
        assert_eq!(result, vec!["a", "b", "c"]);
    }

    #[test]
    fn test_get_unique_groups_custom_sort() {
        let df = df!["g" => ["b", "a", "c"]].unwrap();
        let result = get_unique_groups(&df, "g", Some(|a: &str, b: &str| b.cmp(a)));
        assert_eq!(result, vec!["c", "b", "a"]);
    }

    #[test]
    fn test_get_unique_groups_single_value() {
        let df = df!["g" => ["x", "x", "x"]].unwrap();
        let result = get_unique_groups(&df, "g", None);
        assert_eq!(result, vec!["x"]);
    }

    #[test]
    fn test_get_unique_groups_numeric_cast() {
        let df = df!["g" => [1i32, 2, 1]].unwrap();
        let result = get_unique_groups(&df, "g", None);
        assert_eq!(result, vec!["1", "2"]);
    }

    #[test]
    fn test_filter_matching() {
        let df = df!["g" => ["a", "b", "a"], "v" => [1, 2, 3]].unwrap();
        let filtered = filter_data_by_group(&df, "g", "a");
        assert_eq!(filtered.height(), 2);
    }

    #[test]
    fn test_filter_no_match() {
        let df = df!["g" => ["a", "b"], "v" => [1, 2]].unwrap();
        let filtered = filter_data_by_group(&df, "g", "z");
        assert_eq!(filtered.height(), 0);
    }

    #[test]
    fn test_filter_numeric_cast() {
        let df = df!["g" => [1i32, 2, 1], "v" => [10, 20, 30]].unwrap();
        let filtered = filter_data_by_group(&df, "g", "1");
        assert_eq!(filtered.height(), 2);
    }

    #[test]
    fn test_get_numeric_integers() {
        let df = df!["x" => [1i32, 2, 3]].unwrap();
        let result = get_numeric_column(&df, "x");
        assert_eq!(result, vec![Some(1.0f32), Some(2.0), Some(3.0)]);
    }

    #[test]
    fn test_get_numeric_with_nulls() {
        let s = Series::new("x".into(), &[Some(1.0f64), None, Some(3.0)]);
        let df = DataFrame::new(3, vec![s.into()]).unwrap();
        let result = get_numeric_column(&df, "x");
        assert_eq!(result.len(), 3);
        assert!(result[0].is_some());
        assert!(result[1].is_none());
        assert!(result[2].is_some());
    }

    #[test]
    fn test_get_numeric_floats() {
        let df = df!["x" => [1.5f64, 2.5]].unwrap();
        let result = get_numeric_column(&df, "x");
        assert!((result[0].unwrap() - 1.5).abs() < 0.01);
        assert!((result[1].unwrap() - 2.5).abs() < 0.01);
    }

    #[test]
    fn test_get_string_basic() {
        let df = df!["s" => ["a", "b"]].unwrap();
        let result = get_string_column(&df, "s");
        assert_eq!(result, vec![Some("a".to_string()), Some("b".to_string())]);
    }

    #[test]
    fn test_get_string_with_nulls() {
        let s = Series::new("s".into(), &[Some("a"), None::<&str>]);
        let df = DataFrame::new(2, vec![s.into()]).unwrap();
        let result = get_string_column(&df, "s");
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], Some("a".to_string()));
        assert!(result[1].is_none());
    }
}
