use crate::common::PlotHelper;
use crate::components::Text;

use super::Subplot2;

pub(crate) fn build_irregular(
    _plots: Vec<(&dyn PlotHelper, usize, usize, usize, usize)>,
    _rows: Option<usize>,
    _cols: Option<usize>,
    _title: Option<Text>,
) -> Subplot2 {
    panic!("Subplot2::irregular() is not yet implemented. Use regular() for now.");
}
