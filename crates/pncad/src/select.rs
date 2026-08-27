//! **Structural selectors, and the name doors they need**.
//!
//! A selection is a set of [`StableName`](editor_core::StableName)s:
//! the naming table, the four whole-body materializers, a pattern
//! language over the SHAPE of a name, the geometric filters that
//! narrow one, and the doors from a name back to geometry.
//!
//! The selector vocabulary is re-exported from `editor_core::names`;
//! it lives beside the role enum it mirrors, which is what
//! makes its exhaustive match a compile-time tripwire when the role
//! enum grows. The façade's job is to make it reachable in one
//! import. The two doors below it — picking (`ray → StableName`) and
//! resolution (`stored name → this run's verdict`) — are here for the
//! same reason: both speak names and nothing else, so they belong
//! beside the vocabulary rather than one crate deeper.
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

// **Picking: the fourth door onto a name.** The three above answer
// "which entities match this shape" (`select`), "where is this named
// entity" (`face_frame` and friends) and "how many does it denote"
// (`denotation`). This one answers **"what is under this ray"**: the
// hit-test service resolves a ray against tessellated bodies and hands
// back a `StableName`, which is the same currency every door here
// speaks. A picking consumer that had to reach `editor_core` directly
// would be reaching past the seal for a service whose whole public
// answer is a name.
//
// `Ray` is `bvh`'s, re-exported by `editor_core` for exactly this
// reason. `NodePick` is the door to prefer: it establishes the
// (node, body) ↔ mesh pairing by construction, where a hand-assembled
// `PickTarget` cannot be checked (its own contract says so).
pub use editor_core::{
    HitTestError, MeshPick, MeshPickError, NodePick, NodePickError, PickHit, PickTarget, Ray,
    pick_face,
};

// **The resolution verdict a stored name gets at the next
// evaluation** — the machinery the ratified resolution-failure
// semantics are expressed in. A consumer that stores names (a
// selection, a tool's pick) needs to ask, each run, whether each one
// still denotes something: `resolve` answers `Resolution`, and the
// failure arms carry the typed N5 diagnosis rather than a message.
//
// WHAT THIS DOES AND DOES NOT WIDEN. The façade's document rule is
// that arena keys — `EntityRef`, `EntityKey`, `Entry` — are not
// nameable through `pncad`, and this list does not name them. Two of
// the payloads below carry one in a FIELD (`Resolved::entity`, and
// `Tombstone::patch`'s `MeshPatchKey::entity`): a consumer can read
// and `Debug`-print those fields but cannot spell their type, so it
// cannot store one in its own state, which is what the seal is for.
// Holding a `Tombstone` beside a name is not a workaround either — it
// is the shipped selection discipline, stated in `Tombstone`'s own
// contract ("selection state holds name + tombstone, never a bare
// arena key").
pub use editor_core::{
    Diagnosis, MeshPatchKey, RecipeEditRef, Resolution, ResolutionFailure, ResolveError,
    ResolveIndeterminate, Resolved, RunCtx, TieWitness, Tombstone, resolve, resolve_with_prior,
};
