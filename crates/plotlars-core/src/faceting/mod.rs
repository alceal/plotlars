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
