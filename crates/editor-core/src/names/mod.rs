//! Persistent naming (M4 PR 3; NAMING-DESIGN N1–N4 made concrete —
//! ratified #74, binding; spec `docs/M4-PR3-SPEC.md` D1–D6).
//!
//! A [`StableName`] is a **derivation path**: the minting recipe node
//! plus an op-typed [`RolePath`] of closed-enum [`RoleSeg`]s (N1).
//! Names contain no floats, no arena keys, no bare enumeration
//! indices — geometry enters only as margined predicate VERDICTS
//! recorded in [`Qualifier`]s (N2), and the only integer payloads are
//! recipe-structural data (pattern `Instance(i)`) or the profile
//! crate's own canonical combinatorial identities (locators — see
//! [`role`](self)).
//!
//! The per-node [`NameTable`] (N4) is emitted EAGERLY by the wire
//! layer during evaluation — a mechanical, linear pass over each op's
//! output driven by kernel birth data (`Extruded`/`Revolved` maps,
//! `SplitNaming`, `BooleanNaming`, D5 provenance); nothing is ever
//! reconstructed by matching. Resolution (PR 4) is a table lookup.
//!
//! Layering (D1, G1): the kernel never sees a `StableName` — ops emit
//! birth facts; THIS module (editor-core) names things.

mod discriminate;
mod emit;
mod emit_fillet;
mod emit_sweep;
mod emit_topo;
mod role;
mod table;

pub use emit::NamingError;
pub(crate) use emit::{empty, name_pattern};
pub(crate) use emit_fillet::name_fillet;
pub(crate) use emit_sweep::{name_extrude, name_loft, name_revolve};
pub(crate) use emit_topo::{OperandCtx, name_boolean, name_split};
pub use role::{
    CapEnd, EntityKind, MeridianEnd, ProfileEdgeRef, ProfileVertexRef, Qualifier, RimSupport,
    RolePath, RoleSeg, SideVerdict, SplitHalf, StableName,
};
pub use table::{EntityKey, EntityRef, Entry, NameTable};

/// **Every edge name of a node's output body, as of THIS evaluation**
/// — the materializer for a whole-body fillet selection (M6-5, the
/// F-a ruling).
///
/// [`crate::Node::Fillet`] has no "all edges" variant on purpose: a
/// selection is a frozen commitment, and a live "all" would silently
/// grow when an upstream edit adds an edge. This helper closes the gap
/// the honest way — it hands back the set as it stands, the caller
/// STORES it, and from then on it behaves like any other selection
/// (the growth path is [`crate::DocEdit::Rebind`]).
///
/// Returns the names in canonical order, ready for
/// [`crate::Node::fillet`]. Empty if `node` has no value, no table, or
/// no edges — the fillet node itself refuses an empty selection, so
/// the emptiness surfaces there rather than here.
pub fn all_edges<T: geom_core::Decide>(
    ev: &crate::eval::Evaluation<T>,
    node: crate::node::RecipeNodeId,
) -> Vec<StableName> {
    let Some(crate::eval::NodeResult::Ok(value)) = ev.nodes.get(&node) else {
        return Vec::new();
    };
    let mut out: Vec<StableName> = value
        .name_table
        .iter()
        .filter(|(name, _)| name.kind == EntityKind::Edge)
        .map(|(name, _)| name.clone())
        .collect();
    out.sort();
    out.dedup();
    out
}
