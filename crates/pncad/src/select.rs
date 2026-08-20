//! **Structural selectors, and the name doors they need**.
//!
//! A selection is a set of [`StableName`](editor_core::StableName)s:
//! the naming table, the four whole-body materializers, a pattern
//! language over the SHAPE of a name, the geometric filters that
//! narrow one, and the doors from a name back to geometry.
//!
//! Everything here is re-exported from `editor_core::names`; the
//! vocabulary lives beside the role enum it mirrors, which is what
//! makes its exhaustive match a compile-time tripwire when the role
//! enum grows. The façade's job is to make it reachable in one
//! import.
//!
//! Three invariants govern the surface, and each is worked through
//! in [`crate::guide::selecting`] — the page this module's examples
//! live on:
//!
//! 1. **A selector MATERIALIZES.** [`select`] answers as of one
//!    evaluation and hands back a `Vec<StableName>` you store. There
//!    is no live query in a recipe; a stored "all edges" would
//!    silently grow under an upstream edit.
//! 2. **Patterns are structural; geometry is a second stage.**
//!    [`Selector`] speaks role paths only. Geometry is a filter at
//!    the materializer — [`select_where`] over a conjunction of
//!    [`GeomPred`] atoms, split into EXACT tag reads that cannot
//!    refuse and DECIDED comparisons that are funnel sites and
//!    refuse in-band (`docs/SELECT-DESIGN.md` §§1-2).
//! 3. **A name answers with values, never keys.** [`face_frame`],
//!    [`edge_frame`], [`vertex_position`] and [`denotation`] speak
//!    names and hand back a [`Pose`] or a count. Arena keys are
//!    body-lineage-scoped and do not leave `editor-core`; a NURBS
//!    face has no canonical frame, so `face_frame` refuses it rather
//!    than nominating one.
//!
//! Detection and declaration of flush contact are separate doors on
//! purpose (the ruled no-fusion boundary): [`find_flush_candidates`]
//! REPORTS [`FlushFinding`]s — the contact verifier in
//! candidate-generation mode, so a finding cannot disagree with the
//! boolean's own verify-at-use — and [`declare`]/[`declare_all`]
//! turn findings the caller has INSPECTED into `Node::Declare`.

pub use editor_core::{
    ALL_SURFACE_KINDS, CONTACT_RECOURSE, CapEnd, Cmp, ContactClass, ContactRefusal, ContactVerdict,
    CurveKind, CurveKindSet, DeclareError, DeclaredContact, Denotation, DuplicateName, EntityKind,
    FIT_DEFERRAL, FlushEvidence, FlushFinding, FlushRung, GeomPred, InterrogateError, MeridianEnd,
    NamePat, NameTable, OpGroup, ProfileEdgeRef, ProfileVertexRef, RimSupport, RolePath, RoleSeg,
    SEL_DATUM_DISTANCE, SegPat, SegTag, SelectRefusal, Selector, Side, SplitHalf, SurfaceKindSet,
    TagPat, all_bodies, all_edges, all_faces, all_vertices, declare, declare_all, declare_node,
    denotation, edge_frame, edge_name, face_frame, face_name, find_flush_candidates, select,
    select_where, vertex_position,
};
/// The frame type the geometry doors answer with, and its refusal —
/// re-exported from the kernel's read-back module so a façade user
/// names one crate, not two.
pub use topo::readback::{Pose, ReadbackError};
