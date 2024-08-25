use polars::{
    frame::DataFrame,
    prelude::{col, lit, DataType, IntoLazy},
};

pub(crate) trait Polar {
    fn get_unique_groups(data: &DataFrame, group_col: &str) -> Vec<String> {
        let unique_groups = data
            .column(group_col)
            .unwrap()
            .unique()
            .unwrap()
            .cast(&DataType::String)
            .unwrap();

        unique_groups
            .str()
            .unwrap()
            .iter()
            .map(|x| x.unwrap().to_string())
            .collect::<Vec<String>>()
    }

    fn filter_data_by_group(data: &DataFrame, group_col: &str, group_name: &str) -> DataFrame {
        data.clone()
            .lazy()
            .filter(col(group_col).eq(lit(group_name)))
            .collect()
            .unwrap()
    }

    fn get_numeric_column(data: &DataFrame, column_name: &str) -> Vec<Option<f32>> {
        data.column(column_name)
            .unwrap()
            .clone()
            .cast(&DataType::Float32)
            .unwrap()
            .f32()
            .unwrap()
            .to_vec()
    }

    fn get_string_column(data: &DataFrame, column_name: &str) -> Vec<Option<String>> {
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
}
