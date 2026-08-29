//! **The chamfer naming emitter** — the composition surgery's output,
//! named from BIRTH data, under the chamfering node's id.
//!
//! # Why this is a door and not a copy
//!
//! The chamfer surgery IS the fillet surgery: `chamfer_edges` runs the
//! same battery and the same `blend_surgery`, and `Chamfered<T>` is
//! `Filleted<T>` — the same [`FilletNaming`] rows, written for the
//! same reasons. RECIPE-DOORS D3 then rules that the ROLE vocabulary
//! is shared too: a chamfer strip off a source edge and a fillet blend
//! off a source edge are the same shape in the same place, a
//! `StableName` already carries the minting node, and growing the role
//! vocabulary would be schema-visible for no added discrimination.
//!
//! Both halves of the translation are therefore identical, and this
//! module says so by CALLING the fillet's rather than restating it —
//! [`super::emit_fillet::name_blend`] is the one implementation. A
//! transcription would be two things to keep in step, and the thing
//! they would drift on is exactly the deferral #708 recorded.
//!
//! What differs is the `node` argument, and that difference is the
//! whole discrimination: every segment this emits is minted under the
//! chamfer node's id, so a selector distinguishes a chamfer's strip
//! from a fillet's blend by asking which node minted it — never by
//! reading the role.
//!
//! # What a chamfer's records actually carry
//!
//! `blends` are the flat strips, `corners` the planar patches, and
//! `bands` is empty: a chamfer has no closed-chain band, so the rim
//! roles (`BandFace`, `BandTrim`, `BandFoot`, `BandCross`, `BandCut`,
//! `BandSlit`) simply have no rows to translate. That is a fact about
//! the surgery's output, not a branch here — the shared translation
//! iterates whatever rows it is given.
//!
//! # Ties
//!
//! Inherited from the shared implementation, which defers every
//! tie-descended row through [`super::defer::TieRows`]. This emitter
//! is on that shape from birth; the fillet's was moved onto it in the
//! same change, so #708 has no remaining site.

use std::sync::Arc;

use sweep::fillet::naming::FilletNaming;
use topo::Body;

use super::emit::NamingError;
use super::table::NameTable;
use crate::node::RecipeNodeId;

/// Names one chamfer result.
///
/// `target` is the chamfer's single operand's table (body index 0),
/// `body` the chamfer output, `rec` its birth records.
///
/// # Errors
///
/// [`NamingError::MissingUpstream`] when a record (or a survivor) names
/// a source entity the target's table does not carry — a wiring bug;
/// [`NamingError::Duplicate`] on aliasing at insertion;
/// [`NamingError::Unnamed`] if the result is not covered.
pub(crate) fn name_chamfer<T: geom_core::Real>(
    node: RecipeNodeId,
    target_node: RecipeNodeId,
    target: &NameTable,
    body: &Body<T>,
    rec: &FilletNaming,
) -> Result<Arc<NameTable>, NamingError> {
    super::emit_fillet::name_blend(node, target_node, target, body, rec)
}
