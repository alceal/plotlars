use polars::{
    frame::DataFrame,
    prelude::{col, lit, DataType, IntoLazy},
};

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

#[doc(hidden)]
pub fn filter_data_by_group(data: &DataFrame, group_col: &str, group_name: &str) -> DataFrame {
    data.clone()
        .lazy()
        .filter(col(group_col).cast(DataType::String).eq(lit(group_name)))
        .collect()
        .unwrap()
}

#[doc(hidden)]
pub fn get_numeric_column(data: &DataFrame, column_name: &str) -> Vec<Option<f32>> {
    data.column(column_name)
        .unwrap()
        .clone()
        .cast(&DataType::Float32)
        .unwrap()
        .f32()
        .unwrap()
        .to_vec()
}

#[doc(hidden)]
pub fn get_string_column(data: &DataFrame, column_name: &str) -> Vec<Option<String>> {
    data.column(column_name)
        .unwrap()
        .clone()
        .cast(&DataType::String)
        .unwrap()
        .str()
        .unwrap()
        .iter()
        .map(|x| x.map(|s| s.to_string()))
        .collect::<Vec<Option<String>>>()
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
