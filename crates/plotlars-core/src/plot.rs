use crate::ir::layout::LayoutIR;
use crate::ir::trace::TraceIR;

/// Core trait implemented by all plot types.
/// Provides access to the intermediate representation (IR) data.
pub trait Plot {
    fn ir_traces(&self) -> &[TraceIR];
    fn ir_layout(&self) -> &LayoutIR;
}
