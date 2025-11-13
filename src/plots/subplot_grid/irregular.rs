use crate::common::PlotHelper;
use crate::components::Text;

use super::SubplotGrid;

#[allow(dead_code)]
pub(crate) fn build_irregular(
    _plots: Vec<(&dyn PlotHelper, usize, usize, usize, usize)>,
    _rows: Option<usize>,
    _cols: Option<usize>,
    _title: Option<Text>,
) -> SubplotGrid {
    panic!("SubplotGrid::irregular() is not yet implemented. Use regular() for now.");
}
