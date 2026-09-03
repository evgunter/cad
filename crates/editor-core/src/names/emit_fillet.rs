//! **The fillet naming emitter** — the verb's thin door over the
//! shared blend translation, mirroring [`super::emit_chamfer`]'s: the
//! one implementation is [`super::emit_blend::name_blend`], and what
//! this door contributes is the `node` argument — the minting id
//! every segment is stamped with, which is the whole discrimination
//! between a fillet's names and a chamfer's (RECIPE-DOORS D3).

use std::sync::Arc;

use sweep::blend::naming::BlendNaming;
use topo::Body;

use super::emit::NamingError;
use super::table::NameTable;
use crate::node::RecipeNodeId;

/// Names one fillet result.
///
/// `target` is the fillet's single operand's table (body index 0 —
/// `body_operand` admits only single-body values), `body` the fillet
/// output, `rec` its birth records.
///
/// # Errors
///
/// [`NamingError::MissingUpstream`] when a record (or a survivor) names
/// a source entity the target's table does not carry — a wiring bug;
/// [`NamingError::Duplicate`] on aliasing at insertion;
/// [`NamingError::Unnamed`] if the result is not covered.
pub(crate) fn name_fillet<T: geom_core::Real>(
    node: RecipeNodeId,
    target_node: RecipeNodeId,
    target: &NameTable,
    body: &Body<T>,
    rec: &BlendNaming,
) -> Result<Arc<NameTable>, NamingError> {
    super::emit_blend::name_blend(node, target_node, target, body, rec)
}
