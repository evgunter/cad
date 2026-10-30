//! Persistent naming (M4 PR 3; NAMING-DESIGN N1–N4 made concrete —
//! ratified #74, binding).
//!
//! A [`StableName`] is a **derivation path**: the minting recipe node
//! plus an op-typed [`RolePath`] of closed-enum [`RoleSeg`]s (N1).
//! Names contain no floats, no arena keys, no bare enumeration
//! indices — geometry enters only as margined predicate VERDICTS
//! recorded in [`Qualifier`]s (N2), and the only integer payloads are
//! recipe-structural data (pattern `Instance(i)`) or a profile's own
//! combinatorial identities (locators — see [`role`](self), which
//! says which anchoring a published locator carries).
//!
//! The per-node [`NameTable`] (N4) is emitted EAGERLY by the wire
//! layer during evaluation — a mechanical, linear pass over each op's
//! output driven by kernel birth data (`Extruded`/`Revolved` maps,
//! `SplitNaming`, `BooleanNaming`, D5 provenance); nothing is ever
//! reconstructed by matching. Resolution (PR 4) is a table lookup.
//!
//! Layering (D1, G1): the kernel never sees a `StableName` — ops emit
//! birth facts; THIS module (editor-core) names things.

pub(crate) mod attribute;
mod borders;
mod canonical;
mod chase;
mod defer;
mod discriminate;
mod emit;
mod emit_blend;
mod emit_chamfer;
mod emit_fillet;
mod emit_shell;
mod emit_sweep;
mod emit_topo;
mod emit_union;
mod geompred;
mod groups;
pub(crate) mod interrogate;
mod join_names;
mod least_root;
pub(crate) mod merged;
mod nest;
#[cfg(test)]
mod nest_reference;
mod plane_pieces;
pub(crate) mod role;
mod seam_pair;
mod select;
mod table;
pub(crate) mod words;

pub use attribute::{NameOrigin, attribute};
pub(crate) use defer::CarriedRows;
pub(crate) use discriminate::{CHORD_ON_RIM, DECISION_WORDS, FAMILY, ON_MEMBER_EDGE};
pub use emit::{
    CrossingSensesShape, DisagreeShape, JoinShape, NamingError, RimShare, SeamSidesShape,
    SeamVertexShape, UnionNameShape, WayShape,
};
pub(crate) use emit::{check_total, empty, name_family, output_body, to_u32};
pub(crate) use emit::{name_assemble, name_copy};
pub(crate) use emit_chamfer::name_chamfer;
pub(crate) use emit_fillet::name_fillet;
pub(crate) use emit_shell::name_shell;
pub(crate) use emit_sweep::{name_extrude, name_loft, name_revolve};
pub(crate) use emit_topo::{OperandCtx, name_boolean, name_split};
pub(crate) use emit_union::{
    Fold as UnionFold, Links as UnionLinks, Member as UnionMember, collapse_table, member_cells,
    member_view, name_lone_member, name_union, repeated_member_view,
};
pub use geompred::{
    Cmp, CurveKind, CurveKindSet, GeomPred, SEL_PLANE_DISTANCE, SelectRefusal, SurfaceKindSet,
};
pub(crate) use groups::Emitted;
pub use groups::FragmentGroups;
pub use interrogate::{
    Denotation, InterrogateError, denotation, edge_carrier_kind, edge_frame, face_carrier_kind,
    face_frame, vertex_position,
};
pub(crate) use nest::HoldMut;
pub use nest::NameTextError;
pub(crate) use nest::{read_door, write_door};
pub use role::{
    CapEnd, EntityKind, FaceName, MemberAt, MeridianEnd, NameRef, NotAFaceName, PieceRole,
    PieceRun, ProfileEdgeRef, ProfileVertexRef, Qualifier, RimSupport, RolePath, RoleSeg, RunLine,
    SectionCircle, Sense, SplitHalf, StableName, band, band_pi, band_rim, band_rim_pi, carried,
    meridian_vertex,
};
pub(crate) use role::{Carry, SegRewrite, inert_seg, locator_seg};
pub(crate) use role::{FOLD_A, FOLD_B, read_edge};
pub(crate) use role::{VerbatimEdge, verbatim_edge};
pub(crate) use role::{edge_line, fragment_tail_start, name_free_seg, wrapped_edge};
pub use select::{NamePat, OpGroup, SegPat, SegTag, Selector, Side, TagPat, select, select_where};
pub use table::{
    DuplicateName, EntityKey, EntityNamed, EntityRef, Entry, KindMismatch, NameTable,
    SeatingRefusal, ShortTie,
};
/// The kernel's contact vocabulary, re-exported so a refusal the recipe
/// layer renders quotes the kernel's own words.
pub use topo::{CONTACT_RECOURSE, ContactRefusal, ContactVerdict, ContactPair};
pub use words::{LeafRole, leaf_role, role_leaf};

/// **Every edge name of a node's output body, as of THIS evaluation**
/// — the materializer for an every-edge fillet selection (M6-5, the
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
/// no edges — the fillet's insert refuses an empty selection, so the
/// emptiness surfaces there rather than here.
pub fn all_edges<T: geom_core::Decide>(
    ev: &crate::eval::Evaluation<T>,
    at: impl Into<crate::At>,
) -> Vec<StableName> {
    all_of_kind(ev, at.into(), EntityKind::Edge)
}

/// **Every face name of a node's output body, as of THIS evaluation**
/// — [`all_edges`]'s sibling (LIB-U7 deliverable 1), same contract:
/// filter, sort, dedup; empty for a node with no value or no table.
pub fn all_faces<T: geom_core::Decide>(
    ev: &crate::eval::Evaluation<T>,
    at: impl Into<crate::At>,
) -> Vec<StableName> {
    all_of_kind(ev, at.into(), EntityKind::Face)
}

/// **Every vertex name of a node's output body, as of THIS
/// evaluation** — [`all_edges`]'s sibling, same contract.
pub fn all_vertices<T: geom_core::Decide>(
    ev: &crate::eval::Evaluation<T>,
    at: impl Into<crate::At>,
) -> Vec<StableName> {
    all_of_kind(ev, at.into(), EntityKind::Vertex)
}

/// **Every body name a node's evaluation carries, as of THIS
/// evaluation** — [`all_edges`]'s sibling, same contract. Usually one
/// row (a body-producing op mints one [`RoleSeg::OutputBody`]); a
/// family's members are the plural case. A split's halves are two
/// reads with a row each, and a split named alone has none.
pub fn all_bodies<T: geom_core::Decide>(
    ev: &crate::eval::Evaluation<T>,
    at: impl Into<crate::At>,
) -> Vec<StableName> {
    all_of_kind(ev, at.into(), EntityKind::Body)
}

/// The one body of all four materializers (they differ only in the
/// kind they keep): the node's table, filtered, in canonical order.
fn all_of_kind<T: geom_core::Decide>(
    ev: &crate::eval::Evaluation<T>,
    at: crate::At,
    kind: EntityKind,
) -> Vec<StableName> {
    // No value, no names: `select::select`'s doc.
    let Some(table) = ev.table_at(at) else {
        return Vec::new();
    };
    let mut out: Vec<StableName> = table
        .iter()
        .filter(|(name, _)| name.kind == kind)
        .map(|(name, _)| name.clone())
        .collect();
    out.sort();
    out.dedup();
    out
}
