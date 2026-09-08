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
//! same reason: both speak names, so they belong beside the vocabulary
//! rather than one crate deeper. Each is cut to what a consumer
//! actually needs to ask, not to the whole of what `editor-core`
//! exports about the subject; the stanzas below say what was left out
//! and why.
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
//!    names and hand back a [`Pose`] or a count, and
//!    [`face_carrier_kind`] hands back the face's stored
//!    [`SurfaceKind`](crate::prelude::SurfaceKind) tag — a value too, the one "is this face
//!    planar" is a comparison of. Arena keys are
//!    body-lineage-scoped and do not leave `editor-core`; a NURBS
//!    face has no canonical frame, so `face_frame` refuses it rather
//!    than nominating one (its kind is still readable).
//!
//! **A name also says which node MADE the entity.** [`attribute`]
//! walks a name's carry-through segments — `FromTarget`, `FromA`,
//! `Instance` and their siblings — down to the role that minted it,
//! so "which feature is this face's" is answered by the name rather
//! than by whichever node happens to draw the body.
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
    NameOrigin, NamePat, NameTable, OpGroup, ProfileEdgeRef, ProfileVertexRef, RimSupport,
    RolePath, RoleSeg, SEL_DATUM_DISTANCE, SegPat, SegTag, SelectRefusal, Selector, Side,
    SplitHalf, SurfaceKindSet, TagPat, all_bodies, all_edges, all_faces, all_vertices, attribute,
    declare, declare_all, declare_node, denotation, edge_frame, edge_name, face_carrier_kind,
    face_frame, face_name, find_flush_candidates, select, select_where, vertex_position,
};
/// The frame type the geometry doors answer with, its refusal, and
/// the refusal's own payload — re-exported from the kernel's
/// read-back module so a façade user names one crate, not two.
///
/// [`DanglingRef`] rides beside [`ReadbackError`] because it is that
/// refusal's MATCHABLE payload — the same convention the prelude's
/// `SurfaceKind` follows for `BooleanError`, and the curated half of
/// the crate contract's closure over error payloads. The contract's
/// crate-level half is already met by the whole re-export of `topo`;
/// what a curated list owes on top of it is that a refusal it names
/// is matchable THROUGH it, and `Dangling`'s two arms are different
/// facts about the model: a topological key that does not resolve is
/// a stale or foreign handle, while a geometry key reached from a
/// live entity that does not resolve is a dangling reference inside
/// the body. Carrying the carrier alone leaves that distinction
/// readable only out of the message prose.
pub use topo::readback::{DanglingRef, Pose, ReadbackError};

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
// reason.
//
// **The raw-assembly lane is NOT carried, and its absence is
// structural.** `MeshPick` stays interior, so a façade consumer
// cannot build one — and `PickTarget`'s `pick` field is a
// `&MeshPick`, so the target whose contract warns of a confidently
// wrong name has no constructor here. The type is carried only
// because `pick_face`'s signature names it. `NodePick` is therefore
// not merely the door to prefer: through this façade it is the only
// one.
//
// **`MeshPickError` is carried and its index is not, and the two
// facts do not pull against each other.** The absence above is about
// CONSTRUCTION: an index a consumer cannot build is a target a
// consumer cannot mis-pair. A refusal is not constructed — it arrives
// as `NodePickError::Index`'s payload, out of a door that is carried
// — and what a curated list owes about a refusal it names is that the
// refusal is MATCHABLE through it. Every other arm of `NodePickError`
// is: `Standing` carries a curated `HitTestError`, `Tessellate` a
// prelude-curated `TessellateError`, the two the door owns carry a
// `RecipeNodeId` and a `u32`. Carrying the payload alone leaves the
// index unbuildable and closes that one exception.
pub use editor_core::{
    HitTestError, MeshPickError, NodePick, NodePickError, PickHit, PickTarget, Ray, pick_face,
};

// **The resolution verdict a stored name gets at the next
// evaluation** — the machinery the ratified resolution-failure
// semantics are expressed in. A consumer that stores names (a
// selection, a tool's pick) needs to ask, each run, whether each one
// still denotes something: `resolve` answers `Resolution`, and the
// failure arms carry the typed N5 diagnosis rather than a message.
//
// WHAT THIS SEAL IS, STATED CORRECTLY. The façade's document rule is
// that arena keys — `EntityRef`, `EntityKey`, `Entry` — are not
// nameable through `pncad`, and this list does not name them.
//
// **That is a naming barrier, not a capability barrier, and the
// difference matters.** An earlier version of this comment claimed a
// consumer "cannot store one in its own state"; that is false as
// compiled code. `Resolution::Resolved(r)` binds a value whose type is
// unnameable here, and a generic field — `struct Stash<T>(T)` — stores
// it anyway; the payload derives `PartialEq`, so two arena keys minted
// by two DIFFERENT evaluations can be compared, which is exactly the
// body-lineage-scoped comparison G1's rule exists to forbid. Both
// reviewers of the unit that opened this door demonstrated it with
// compiling code that names only `pncad`.
//
// So what the seal buys is precise and worth having: **no consumer
// reaches an arena key by accident**, because every route to one is a
// deliberate contortion that a reader of the code can see. It does not
// make the reach impossible, and narrowing the payloads' own derives
// to make it so was assessed and declined — `Resolved`'s `PartialEq`
// is `Resolution`'s, which `editor-core`'s own resolution suites
// compare, so removing it is a cascade through the kernel's tests
// rather than a small façade edit. The cheaper narrowing was taken
// instead: `Resolved`, `Tombstone` and `MeshPatchKey` are not carried
// at all, so nothing here names a key-bearing payload as a TYPE.
//
// What is carried is the verdict a consumer that stores names must
// read on every re-evaluation, plus the three types its arms are —
// `ResolutionFailure`, its `ResolveError`, and `ResolveIndeterminate`
// — so that a consumer can branch on WHICH failure it got and not
// only on THAT it failed. `vanished`, `ambiguous` and `node_gone` ask
// for three different repairs, and `target_failed` /
// `target_poisoned` / `target_not_evaluated` name three different
// nodes to look at; without the types those six facts arrive as two
// words and a sentence.
//
// The richer-diagnosis vocabulary underneath them stays interior:
// `Diagnosis`, `Tombstone`, `TieWitness`, `RecipeEditRef` and
// `resolve_with_prior` are the telemetry and key-bearing half, and
// `Resolved` is the arm that holds an `EntityRef` outright. A caller
// BINDS those and branches on the discriminant the arm gives it,
// which is what a curated list owes; nothing here names one as a
// type.
pub use editor_core::{
    Resolution, ResolutionFailure, ResolveError, ResolveIndeterminate, RunCtx, resolve,
};
