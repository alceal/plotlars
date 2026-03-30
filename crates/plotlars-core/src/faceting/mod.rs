#[doc(hidden)]
pub fn calculate_grid_dimensions(
    n_facets: usize,
    cols: Option<usize>,
    rows: Option<usize>,
) -> (usize, usize) {
    match (cols, rows) {
        (Some(c), Some(r)) => {
            if c * r < n_facets {
                panic!("Grid dimensions {}x{} cannot fit {} facets", c, r, n_facets);
            }
            (c, r)
        }
        (Some(c), None) => {
            let r = n_facets.div_ceil(c);
            (c, r)
        }
        (None, Some(r)) => {
            let c = n_facets.div_ceil(r);
            (c, r)
        }
        (None, None) => {
            let c = (n_facets as f64).sqrt().ceil() as usize;
            let r = n_facets.div_ceil(c);
            (c, r)
        }
    }
}

#[doc(hidden)]
pub fn get_axis_reference(subplot_index: usize, axis_type: &str) -> String {
    if subplot_index == 0 {
        axis_type.to_string()
    } else {
        format!("{}{}", axis_type, subplot_index + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_both_exact() {
        assert_eq!(calculate_grid_dimensions(4, Some(2), Some(2)), (2, 2));
    }

    #[test]
    fn test_grid_both_extra() {
        assert_eq!(calculate_grid_dimensions(3, Some(2), Some(2)), (2, 2));
    }

    #[test]
    #[should_panic(expected = "cannot fit")]
    fn test_grid_both_too_small() {
        calculate_grid_dimensions(5, Some(2), Some(2));
    }

    #[test]
    fn test_grid_cols_only() {
        assert_eq!(calculate_grid_dimensions(5, Some(3), None), (3, 2));
    }

    #[test]
    fn test_grid_rows_only() {
        assert_eq!(calculate_grid_dimensions(5, None, Some(2)), (3, 2));
    }

    #[test]
    fn test_grid_auto_1() {
        assert_eq!(calculate_grid_dimensions(1, None, None), (1, 1));
    }

    #[test]
    fn test_grid_auto_4() {
        assert_eq!(calculate_grid_dimensions(4, None, None), (2, 2));
    }

    #[test]
    fn test_grid_auto_5() {
        assert_eq!(calculate_grid_dimensions(5, None, None), (3, 2));
    }

    #[test]
    fn test_grid_auto_9() {
        assert_eq!(calculate_grid_dimensions(9, None, None), (3, 3));
    }

    #[test]
    fn test_axis_ref_index_zero() {
        assert_eq!(get_axis_reference(0, "x"), "x");
    }

    #[test]
    fn test_axis_ref_index_one() {
        assert_eq!(get_axis_reference(1, "x"), "x2");
    }

    #[test]
    fn test_axis_ref_index_seven() {
        assert_eq!(get_axis_reference(7, "y"), "y8");
    }
}
