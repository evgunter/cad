//! The validation harness: [`validate`] (tier 1), [`validate_closed`]
//! (tier 2), [`validate_geometric`] (tier 3 — M2 PR 3), and
//! [`ValidationError`].
//!
//! # The two validity tiers (ratified via the M1-PLAN conversation)
//!
//! **Tier 1 — "euler-valid"** ([`validate`]): the referential and
//! combinatorial soundness of the arena store, checkable without any
//! geometry evaluation. Tier 1 accepts every Euler-reachable state,
//! construction scaffolding included (empty loops, struts, self-loop
//! edges, laminae): those are mandatory intermediates, and each Euler
//! operator debug-asserts tier 1 as its postcondition (D1).
//! Watertightness in the half-edge representation is *structural* and
//! therefore tier 1: every edge has exactly two antiparallel half-edges
//! (passes 3–4) and every vertex orbit is a single cycle (pass 6).
//!
//! **Tier 2 — "closed solid"** ([`validate_closed`]): tier 1 plus the
//! at-rest bans on construction scaffolding — no empty loops
//! ([`ValidationError::ScaffoldingEmptyLoop`]), no valence-1 vertices
//! (struts, [`ValidationError::ScaffoldingStrutVertex`]), every
//! shell's incidence complex connected (c = 1,
//! [`ValidationError::ShellDisconnected`]), and — since M3 PR 1 — no
//! null entities ([`ValidationError::NullEdgeAtRest`] /
//! [`ValidationError::NullFaceAtRest`]; see `crate::null`). A solid
//! may hold **multiple shells** (M3 multi-shell results — voids,
//! disjoint unions); tier 2 constrains each shell, never the count.
//! Finished bodies must pass
//! tier 2; tier-1-only states are visible solely inside operation
//! sequences. Two deliberate boundaries of the tier:
//!
//! - **Laminae are NOT banned.** Two faces glued along their entire
//!   shared boundary is exactly the incidence structure of a legitimate
//!   two-hemisphere ball, so a zero-volume lamina is a *geometric*
//!   defect — killed at the M2+ geometric tier by the material
//!   wedge-angle predicate (D4 ¶3), never by a topological rule here.
//! - **c = 1 is not implied by the other two bans.** Promoting a
//!   detached cycle ring (`mfkrh` on a ring that never plugged a
//!   handle) disconnects the shell's surface with no empty loops and no
//!   struts anywhere — the PR 4 finding — so the connectivity ban is
//!   its own rule.
//!
//! # The tier-1 check set (each check names its error variants):
//!
//! 1. **Reference resolution.** Every topology key held by an entity
//!    resolves in its arena ([`ValidationError::DanglingTopology`]), and
//!    every geometry key resolves — a face's surface, an edge's curve, a
//!    vertex's point ([`ValidationError::DanglingGeometry`]).
//! 2. **Half-edge chain coherence.** `next`/`prev` are mutual inverses
//!    ([`ValidationError::NextPrevMismatch`]); every
//!    [`LoopBoundary::Cycle`] closes within the arena bound
//!    ([`ValidationError::LoopCycleOverrun`]); every half-edge reached
//!    by a loop's cycle points back to that loop
//!    ([`ValidationError::ParentLoopMismatch`]); every half-edge is
//!    reached by its own parent's cycle
//!    ([`ValidationError::UnreachableHalfEdge`] — also fired when the
//!    claimed parent is an `Empty` loop, which reaches nothing).
//! 3. **Edge ↔ half-edge bijection.** An edge's halves are distinct
//!    ([`ValidationError::EdgeHalvesIdentical`]), both point back via
//!    `.edge` ([`ValidationError::EdgeSlotBackpointerMismatch`]), and
//!    every half-edge is claimed by exactly one edge slot overall
//!    ([`ValidationError::HalfEdgeUnclaimed`] /
//!    [`ValidationError::HalfEdgeMultiplyClaimed`]).
//! 4. **Antiparallelism.** The two halves traverse the edge in opposite
//!    directions: `end(he_plus) = start(he_minus)` and vice versa
//!    ([`ValidationError::EdgeNotAntiparallel`]).
//! 5. **Vertex anchoring.** `emanating = Some(he)` ⇒ `he` starts at the
//!    vertex ([`ValidationError::EmanatingStartMismatch`]) and the
//!    vertex is no empty loop's lone vertex
//!    ([`ValidationError::EmptyLoopVertexWithEmanating`]);
//!    `emanating = None` ⇒ no half-edge starts at the vertex
//!    ([`ValidationError::LoneVertexWithIncidence`]) and the vertex is
//!    the lone vertex of *exactly one* empty loop — zero is
//!    [`ValidationError::OrphanEntity`], two or more is
//!    [`ValidationError::MultiplyOwned`]. (This restates M0's
//!    orphan-vertex rule in half-edge terms: every vertex is anchored by
//!    incident half-edges XOR by one empty loop.)
//! 6. **Vertex-orbit closure (manifoldness).** The bounded orbit walk
//!    from `emanating` returns to its start
//!    ([`ValidationError::VertexOrbitOverrun`] on non-closure) and
//!    visits exactly the half-edges starting at the vertex — no foreign
//!    members ([`ValidationError::OrbitForeignMember`]), no split orbits
//!    ([`ValidationError::SplitVertexOrbit`], the classic non-manifold
//!    "bowtie" catch).
//! 7. **Ownership and back-pointers.** `outer ∉ rings`
//!    ([`ValidationError::OuterListedAsRing`]); every shell, face, and
//!    loop is owned exactly once by its parent kind, counting
//!    multiplicity (zero is [`ValidationError::OrphanEntity`], two or
//!    more [`ValidationError::MultiplyOwned`] — duplicate rings surface
//!    here); the upward back-pointers `loop.face`, `face.shell`,
//!    `shell.solid` match the actual owner
//!    ([`ValidationError::BackPointerMismatch`]).
//! 8. **Orphan geometry.** Every point, curve, and surface is referenced
//!    by at least one entity ([`ValidationError::OrphanGeometry`]) — an
//!    **error**, not a warning: bodies are values built by operators,
//!    and nothing should leak.
//! 9. **Arity floors.** Every solid has at least one shell
//!    ([`ValidationError::SolidWithoutShells`]) and every shell at least
//!    one face ([`ValidationError::ShellWithoutFaces`]). These are
//!    defined by the operator set (the M0 deferral, now discharged):
//!    `mvfs` births solid + shell + face together and `kvfs` retires
//!    them together — no operator ever strands a bare solid or shell.
//!    (Face ≥ 1 loop and the loop shapes are already structural:
//!    `Face::outer` is non-optional and [`LoopBoundary`] is total.)
//! 10. **Shell-partition/edge-adjacency coherence.** For every edge, the
//!     faces of its two halves' loops belong to the same shell
//!     ([`ValidationError::EdgeAcrossShells`]). No Euler operator crosses
//!     shells, so a violation is operator-unreachable — a tier-1
//!     invariant (the gap named by the PR 1 review: the ownership tree
//!     alone never compares an edge's two shells).
//! 11. **Component-aware per-shell Euler–Poincaré** (the PR 4
//!     correction — the naive per-body form is wrong for tier-1 bodies,
//!     where `mfkrh` on a detached ring disconnects a shell's surface
//!     while one shell entity remains). Per shell, the incidence complex
//!     is partitioned into connected components: faces glue all their
//!     loops (outer and rings); a cycle loop glues its edges' two sides
//!     via mate; an empty loop glues its lone vertex; a dartless
//!     empty-outer face is its own component with its vertex. Each
//!     component is a closed oriented surface piece and must satisfy
//!     `v − e + f − r = 2(1 − g)` with genus `g` a non-negative integer
//!     — parity and the `g ≥ 0` bound are checked per component
//!     ([`ValidationError::ComponentEulerViolation`], which carries the
//!     counts). The per-shell sum identity `v − e + f − r = 2(c − Σgᵢ)`
//!     follows; tier 2 additionally requires c = 1 per shell.
//! 12. **Bidirectional D5 provenance.** Every live entity in all seven
//!     topology arenas has a provenance record
//!     ([`ValidationError::MissingProvenance`]), and every provenance
//!     entry's key resolves to a live entity
//!     ([`ValidationError::LeakedProvenance`] — the `SecondaryMap` leak
//!     a kill-side operator would cause by removing an entity without
//!     its record).
//! 13. **Null-entity referential coherence (M3 PR 1).** A null-scaffold
//!     curve entry is referenced by at most one edge
//!     ([`ValidationError::NullScaffoldShared`]), a null-face record
//!     never outlives its face
//!     ([`ValidationError::LeakedNullFaceRecord`]), and every loop key
//!     a record names resolves
//!     ([`ValidationError::StaleNullFaceLoop`]). Deliberately
//!     minimal and referential-only — attribute semantics are the
//!     surgery ops' contract, and tier 2 bans null entities at rest
//!     outright (see `crate::null`).
//!
//! The harness is deliberately a plain function plus an error enum,
//! **not a trait**: there is exactly one notion of body validity per
//! milestone, and speculative abstraction would only blur it.
//!
//! # Tier 3 — "geometric" ([`validate_geometric`], M2 PR 3)
//!
//! Tier 2 plus the geometric re-checks at rest: D4 ¶2 residual
//! certification (every edge carrier re-certified against its
//! intensional description and endpoints), planar-face plane-equation
//! residuals, description-adjacency coherence, the dihedral
//! classification pass (the material wedge-angle predicate's first
//! arrival: every edge classifies definitely — corner or smooth seam,
//! never sliver) with its prefer-intrinsic enforcement
//! (definitely-transverse edges must carry `Intersection` — D2,
//! ratified 2026-07-19), and planar-boundary containment (edge carrier
//! samples against adjacent planar faces' planes — M2 PR 3 fix
//! pass). The full
//! check list, gate, and the honest not-yet-checked list live on
//! [`validate_geometric`].
//!
//! # All failures, not the first
//!
//! [`validate`] collects **every** failure before returning: a validator
//! that stops at the first defect is a bad debugging tool, and fail-loud
//! (D4) means report everything. The `Err` vector is never empty.
//!
//! # Cascade discipline
//!
//! One defect must not drown the report in echoes, so downstream checks
//! are *gated* on their prerequisites, and every gate is chosen so that
//! skipping is only possible when an earlier pass already reported the
//! cause: a walk that hits a stale link or a broken mate stays silent
//! (pass 1/3 reported it); reachability is only judged against loops
//! whose cycle closed; antiparallelism needs both halves distinct and
//! both ends derivable; orbit checks need a resolving `emanating` that
//! starts at its vertex; back-pointer comparisons need an unambiguous
//! (exactly-one) owner and a resolving stored key; the edge-adjacency
//! comparison (pass 10) derives each half's shell through the
//! *ownership partition* — never the stored back-pointers, whose
//! defects are pass 7's report — and needs both ownership steps
//! unambiguous. Pass 11 (component Euler–Poincaré) interprets the
//! *counts* of a structurally coherent complex, so it has one coarse
//! gate: it runs only when passes 1–7 reported nothing — any earlier
//! structural defect voids the counting and already carries its own
//! report (passes 8–10 do not gate it: orphan geometry and arity floors
//! cannot corrupt the counts, and a cross-shell edge is exactly what the
//! per-shell count must still be computed *through* — the moved-face
//! scenario is reported by pass 10 AND pass 11). Tier 2's connectivity
//! check shares pass 11's gate and its component enumeration. Its other
//! two scans are ungated — the empty-loop scan reads single loops, but
//! the strut scan is an AGGREGATE (it counts resolving `start`
//! references over the half-edge arena), so on a corrupt in-crate body
//! a pass-1 dangling `start` deflates its true vertex's derived valence
//! and can echo as a spurious `ScaffoldingStrutVertex` alongside the
//! pass-1 report. Accepted and pinned (see the promoted PR 5 review
//! probe): such bodies are unreachable through the public API, whose
//! every product is tier-1-valid, and gating the aggregate would cost
//! genuine strut reports on bodies with unrelated defects. Genuinely
//! independent facts still report independently — a corruption that
//! breaks two invariants yields two errors.
//!
//! # Deterministic report order (D9)
//!
//! Errors arrive in a fixed, documented order — twelve tier-1 passes,
//! each walking its arenas in slot-index order, checking an entity's
//! references in field-declaration order:
//!
//! 1. reference resolution, walking solids → shells → faces → loops →
//!    half-edges → edges → vertices;
//! 2. chain coherence: next/prev inverses (half-edges), then cycle walks
//!    (loops; members in walk order), then unreachable half-edges
//!    (half-edges);
//! 3. bijection: per-edge checks (halves identical, then plus/minus
//!    back-pointers), then per-half-edge claim counts;
//! 4. antiparallelism, sweeping edges;
//! 5. vertex anchoring, sweeping vertices;
//! 6. orbit closure, sweeping vertices (foreign members in walk order);
//! 7. ownership/back-pointers: outer-vs-rings (faces), then shells,
//!    faces, loops;
//! 8. orphan geometry, sweeping points → curves → surfaces;
//! 9. arity floors, sweeping solids then shells;
//! 10. edge-adjacency shell coherence, sweeping edges;
//! 11. component Euler–Poincaré: shells in arena order; within a shell,
//!     components in seed order, where a component's seed is its first
//!     face in **face-arena order** (no hashing anywhere — D9);
//! 12. provenance: missing records (entities in arena order, kinds in
//!     the pass-1 order solids → … → vertices), then leaked records
//!     (`SecondaryMap` entries in slot order, same kind order).
//!
//! [`validate_closed`] appends the tier-2 failures after all tier-1
//! errors, in this order: empty loops (loop-arena order), valence-1
//! vertices (vertex-arena order), disconnected shells (shell-arena
//! order), null edges at rest (edge-arena order), null-face records at
//! rest (record slot order) — the last two added at M3 PR 1 (null
//! entities are surgery transients, banned at rest by name like the
//! other scaffolding shapes).

use core::fmt;

use geom::Surface;
use geom_brep::{CertifyError, DihedralClass, classify_dihedral};
use geom_core::{Band, BandError, Decide, Indeterminate, Margin, Real, Sign};
use slotmap::{Key, SecondaryMap};

use crate::body::{Body, Walk};
use crate::contact::DeclaredContact;
use crate::geometry::CurveKey;
use crate::null::CurveGeom;

use crate::entity::{
    EdgeKey, EntityId, FaceKey, GeomRef, HalfEdgeKey, LoopBoundary, LoopKey, ShellKey, SolidKey,
    VertexKey,
};

/// The one classification funnel of this crate (the `geom-brep`
/// pattern): delegates to the unified recorder funnel
/// [`geom_core::k_stats::decide`] (M2 PR 7), which names the predicate
/// for the margin-telemetry recorder, classifies through the
/// sanctioned [`Decide`] door, and tags any escalation.
pub(crate) fn decide<T: Decide>(
    name: &'static str,
    margin: Margin<T>,
    band: Band,
) -> Result<Sign, Indeterminate> {
    geom_core::k_stats::decide(name, margin, band)
}

/// **The tier-3 contact MARK** (OQ7's two-level shape, level (i);
/// M5 PR 9): the per-edge dihedral/jet verdict the tier-3 pass
/// derives, KEPT as a named recorded classification instead of
/// discarded — this is what stops the prefer-intrinsic preference
/// from drifting, at zero enforcement risk. Obtainable at rest
/// through [`contact_marks`]; the must-carry
/// ([`ValidationError::TangentNotIntrinsic`]) is level (ii) and
/// fires only on [`ContactMark::Tangent`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ContactMark {
    /// Every interior sample definitely transverse (the
    /// `Intersection` regime).
    Transverse,
    /// Definitely tangent AND jet-determinate: every interior sample
    /// definitely smooth at first order, second-order separation
    /// (`tangent_second_order`) definitely positive — the surfaces
    /// determine the locus (`TangentIntersection`'s regime).
    Tangent,
    /// Definitely smooth at first order with a zero-side second-order
    /// margin at some sample: the surfaces UNDER-determine the locus
    /// (a G2 conventional join, a same-surface split) — exempt from
    /// the must-carry by this very verdict.
    SmoothUnderdetermined,
    /// A `Seam`-described edge — exempt by kind, as always.
    Seam,
    /// No definite whole-edge verdict: mixed samples, or an
    /// escalation already reported as its own error.
    Unmarked,
}

/// A structural or geometric defect found by the validators. Closed
/// enum, D3 style: each tier's PRs add variants as compiler-guided
/// extensions.
///
/// **The obligation that closedness buys falls on the sites that
/// CLASSIFY**, and it is not stylistic. A match that maps this enum
/// onto a SMALLER vocabulary — a caller's verdict, a recourse, a tag —
/// must say what it does with each new failure kind, because a
/// wildcard there answers for the new kind silently and no existing
/// row can notice: the rows that exist exercise the variants the named
/// arms already handle. `editor_core`'s `attribute` is the one such
/// site outside this crate, and what it decides is whether an assembly
/// was refused or merely could not be certified.
///
/// A site that EXTRACTS carries no such obligation — a
/// `find_map`/`filter_map` picking one variant out and answering
/// `None` to the rest is asking a question, not giving an answer, and
/// its `_` arm IS the question. Several rows in this crate's own tests
/// are that shape and are meant to be.
///
/// (`Eq` dropped at M2 PR 3: the tier-3 variants carry margin
/// diagnostics with `f64` payloads.)
#[derive(Clone, Debug, PartialEq)]
pub enum ValidationError {
    /// Tier 3: the run's tolerance could not produce a valid
    /// classification band (absurd ε — see `Band::linear`). Reported
    /// alone; no geometric check can run without a band.
    Band {
        /// The band-construction failure.
        error: BandError,
    },
    /// An edge-curve *description* references a surface key that does
    /// not resolve — the geometry-to-geometry half of referential
    /// integrity (tier 1, pass 1; M2 PR 3: `Intersection`/`Seam`
    /// descriptions name surfaces by arena key).
    DanglingDescription {
        /// The curve whose description holds the dangling reference.
        from: GeomRef,
        /// The dangling surface reference.
        to: GeomRef,
    },
    /// Tier 3: a face's surface is the `Nurbs` **placeholder** at rest
    /// — `mvfs`'s all-poison "no description yet" seed state, which
    /// must be replaced via `Body::set_face_surface` before rest.
    /// Nothing can be certified against poison.
    ///
    /// Since M6-3 this names ONLY the placeholder: a **described**
    /// NURBS surface (finite control net) is real geometry and passes
    /// check 1 — its seams certify through the `IsoCurve` lane and its
    /// volume flux through the quadrature door. The two states used to
    /// be conflated here; `NurbsSurface::is_placeholder` is the one
    /// shared discriminator.
    UncertifiableSurface {
        /// The face whose surface is the placeholder.
        face: FaceKey,
    },
    /// Tier 3: an edge's carrier re-certification failed at rest — the
    /// stored cache no longer satisfies D4 ¶2's `residual ≤ ε` against
    /// its description and endpoints (or its certification is not
    /// runnable: unimplemented kinds, unresolved described surfaces).
    EdgeCertification {
        /// The edge whose certification failed.
        edge: EdgeKey,
        /// The re-certification failure.
        error: CertifyError,
    },
    /// Tier 3: an `Intersection`/`Seam` description's surface keys are
    /// not the edge's two adjacent faces' surfaces (D2 adjacency
    /// coherence — an intersection edge's surfaces are its faces').
    DescriptionNotAdjacent {
        /// The edge whose description is incoherent with its faces.
        edge: EdgeKey,
    },
    /// Tier 3: a vertex of a planar face lies definitely off the face's
    /// stored plane (the Newell-certified plane equation is a derived
    /// cache; its residuals are re-checked at rest, D4 ¶2).
    PlanarFaceResidual {
        /// The planar face.
        face: FaceKey,
        /// The off-plane vertex.
        vertex: VertexKey,
    },
    /// Tier 3: a planar-face vertex residual escalated (in the sliver
    /// band, or poisoned) — indeterminate geometry at rest is a defect
    /// (D4 ¶3's escalate-never-guess, applied by the validator).
    PlanarFaceEscalated {
        /// The planar face.
        face: FaceKey,
        /// The vertex whose residual escalated.
        vertex: VertexKey,
        /// The classifier's diagnostic.
        cause: Indeterminate,
    },
    /// Tier 3: an edge bounding a **planar** face has carrier samples
    /// definitely off that face's plane (M2 PR 3 fix pass, S3): the
    /// planar-face pass checks vertices, and this check gives the same
    /// teeth to the boundary *between* vertices — an honestly certified
    /// arc bulging off its face's plane is a face-boundary defect even
    /// though every vertex residual passes. (Curved-face boundary
    /// containment is not checked: [`validate_geometric`]'s
    /// not-yet-checked list, tracked at #638.)
    PlanarBoundaryResidual {
        /// The planar face whose boundary leaves its plane.
        face: FaceKey,
        /// The edge whose carrier samples lie off the plane.
        edge: EdgeKey,
    },
    /// Tier 3: a planar-boundary sample residual escalated (in the
    /// sliver band, or poisoned) — the escalation counterpart of
    /// [`ValidationError::PlanarBoundaryResidual`].
    PlanarBoundaryEscalated {
        /// The planar face.
        face: FaceKey,
        /// The edge whose sample residual escalated.
        edge: EdgeKey,
        /// The classifier's diagnostic (first failing sample).
        cause: Indeterminate,
    },
    /// Tier 3: the dihedral classification at an edge escalated — the
    /// wedge between its two faces' tangent planes is in the sliver
    /// band (or unclassifiable: poison at a surface singularity, an
    /// unimplemented kind). The material wedge-angle predicate's
    /// at-rest form: every edge must classify definitely (transverse
    /// corner or smooth seam), never sliver.
    SliverDihedral {
        /// The edge whose wedge cannot be classified definitely.
        edge: EdgeKey,
        /// The classifier's diagnostic (from the first failing interior
        /// sample).
        cause: Indeterminate,
    },
    /// Tier 3: a **definitely-transverse** edge carries a conventional
    /// (`MappedCurve`) description at rest — the prefer-intrinsic rule
    /// with teeth (D2; ratified with Evan 2026-07-19, M2 PR 4 fix
    /// pass). Enforced only when the dihedral pass classified **every**
    /// interior sample definitely Transverse: definitely-smooth edges
    /// keep their conventional descriptions (the D2 conventional
    /// split), `Seam` edges are exempt by kind, and escalated dihedrals
    /// already report [`ValidationError::SliverDihedral`] — so
    /// ε-tightening can escalate an edge but never flips a valid body
    /// to invalid through this check. (A mixed transverse/smooth sample
    /// set — a tangency-crossing edge — is neither definitely
    /// transverse nor definitely smooth as a whole, so it takes
    /// `ContactMark::Unmarked` and NEITHER must-carry attaches: an
    /// exemption by the predicate, like every other one here.)
    TransverseNotIntrinsic {
        /// The definitely-transverse edge whose description is
        /// conventional.
        edge: EdgeKey,
    },
    /// Tier 3, the symmetric must-carry (OQ7's two-level shape, level
    /// (ii); M5 PR 9): a **jet-determinate tangency** — every interior
    /// sample definitely Smooth at first order AND the second-order
    /// separation (`tangent_second_order`, the relative transverse
    /// normal curvature at the folded lever arm) definitely positive
    /// at every sample — carries a conventional (`MappedCurve`)
    /// description at rest. The surfaces DETERMINE the locus, so the
    /// intrinsic `TangentIntersection` description must be stored
    /// (prefer-intrinsic, one differential order up).
    ///
    /// Exemptions are **by the predicate, never a list**: a G2
    /// conventional join's second-order margin is zero-side (the
    /// surfaces under-determine the locus — κ_rel exactly zero, e.g.
    /// any same-surface or same-value smooth split), so it never
    /// classifies jet-determinate; `Seam` edges are exempt by kind
    /// exactly as today; in-band second order escalates as
    /// [`ValidationError::SliverDihedral`] with the
    /// `tangent_second_order` cause (F6 — an osculating pair is a
    /// sliver at this ε), which exempts the edge here — so
    /// ε-tightening can escalate but never flips a valid body to
    /// invalid THROUGH THIS CHECK. Enforcement is scoped by
    /// [`geom_brep::tangent_certificate_lane`] — the ONE home of the
    /// jet certificate's per-class boundary (C12.1: `Line` carriers
    /// on `Plane`/`Cylinder`/`Sphere` pairs), so the demanded set and
    /// the certifiable set are the same set by construction; a
    /// tangency outside the lane is neither certifiable nor demanded.
    TangentNotIntrinsic {
        /// The jet-determinate tangent edge whose description is
        /// conventional.
        edge: EdgeKey,
    },
    /// Tier 3 (check 6): a planar face's loop ROLES disagree with its
    /// windings — the outer loop winds **definitely negatively** (or a
    /// cycle ring definitely positively) around the face's outward
    /// normal; the region-bounding statement's planar half, previously
    /// a documented deferral (M5 S1 fix pass). A role inversion is
    /// invisible to every volume gate (they are role-invariant) but
    /// silently corrupts tessellation/export — exactly the defect
    /// class this check closes structurally.
    ///
    /// **Since M5 S10 this is also the sense gate.** The outward normal
    /// is `Face::sense_sign() · chart_normal`, so the comparison is
    /// between the face's two independent encodings of one fact: the
    /// stored `sense` bit and the loops' stored winding (interior-left
    /// ⇒ the outer loop winds CCW about the outward normal). A body
    /// whose face sense disagrees with its winding is INSIDE-OUT, and
    /// this is the check that refuses it — nothing else can, since the
    /// signed volume is itself winding-derived and therefore blind to
    /// a lone `sense` flip. (The analytic curved kinds get the same
    /// statement from this check's curved arm,
    /// [`Self::CurvedSenseInverted`] — M6-6.) `Zero` and escalated
    /// windings are exempt (the check-7 posture: an orientation probe,
    /// not a thinness gate — degenerate pillow fixtures stay legal and
    /// ε-tightening never flips valid → invalid; a genuinely
    /// positive-area loop never classifies `Negative` under a
    /// tighter ε).
    LoopRoleInverted {
        /// The face whose loop roles disagree with the windings.
        face: FaceKey,
        /// The loop whose winding disagrees with its role.
        r#loop: LoopKey,
    },
    /// Tier 3 (check 6, curved arm — M6-6): a curved face's stored
    /// [`crate::Face::sense`] bit **definitely disagrees** with the
    /// material side its own boundary encodes — the
    /// [`Self::LoopRoleInverted`] sibling for the analytic curved kinds
    /// (cylinder, cone, rim-bearing sphere, torus). The face's two
    /// orientation encodings are the S10 `sense` bit and the stored
    /// outer-loop traversal (interior-left): the flux derivations read
    /// the material side off the latter
    /// (`geom_brep::props::boundary_material_sign` — the rim-side /
    /// meridian-orientation decides), and a face where the two
    /// encodings disagree is INSIDE-OUT at that face. Tier 3 refuses
    /// it here, per face, and no volume gate can: the flux is
    /// traversal-derived, so a lone `sense` flip on a rim-bearing
    /// curved face is BIT-IDENTICAL in volume (executed truth table,
    /// M6-6 substrate — washer walls, cone laterals, sphere zones,
    /// torus bands all certified green before this arm).
    ///
    /// The comparison is **combinatorial** — two exact ±1s, no
    /// comparand, no new margin: the derived side reuses the flux
    /// lanes' already-length-metered named decides. Posture inherited
    /// from check 7: only a DEFINITE disagreement refuses; an
    /// escalated/degenerate/out-of-inventory derivation is exempt.
    ///
    /// **What is exempt, in full**, because this set is larger than a
    /// reader expects and it grew:
    ///
    /// * the **rimless** sphere band — its boundary encodes no side at
    ///   all (`s_f` IS the bit), the documented residual;
    /// * a face whose **domain is not an iso-parameter rectangle**.
    ///   `props`' side derivation runs the `props_rim_level` predicate
    ///   before answering, on all four curved kinds, because
    ///   `lo + hi − 2v` reads *which extreme is this rim at* and that
    ///   is a material side only on a rectangle. Such a face returns
    ///   `Err` and is exempt HERE, which is correct — without the
    ///   premise it returned a definite ±1 that depended on where
    ///   `loop_edges` started the cycle, and this arm turned that into
    ///   a refusal that also suppressed check 7's honest
    ///   `VolumeUncomputable { NotIsoRectangle }` through the
    ///   `errors.is_empty()` gate. **The coverage is real and is
    ///   given up on purpose**: the corpus's `cross.step` and
    ///   `tee.step` walls are exempt here now, and the flux lane
    ///   refuses those bodies anyway;
    /// * conic-trimmed faces the quadrature lane owns, and NURBS.
    ///
    /// S11's honest `sense: false` faces (a washer's bore, a die's
    /// dimples) PASS: their traversals already place the material on
    /// the anti-chart-normal side — the gate refuses lone bit flips,
    /// not concave walls.
    CurvedSenseInverted {
        /// The inside-out face: its `sense` bit contradicts the
        /// material side its boundary traversal encodes.
        face: FaceKey,
    },
    /// Tier 3: the body's exact-B-rep signed volume is **definitely
    /// negative** — global orientation corruption (the +V invariant,
    /// M2 PR 7). The margin is `V / A_total` (a length: the mean
    /// boundary displacement of the volume defect), classified against
    /// the run's linear band; `Zero` and escalated margins are exempt
    /// (escalation never flips valid → invalid).
    NegativeVolume,
    /// Tier 3: the exact-B-rep volume for the +V invariant could not
    /// be computed — a face's boundary fell outside the M2
    /// iso-rectangle inventory or its closed-form classification
    /// failed.
    ///
    /// **Which D2-addendum row this is depends on `source`, and the
    /// variant spans both.** The doc used to read *"at rest every
    /// M2-constructible body computes; this is corruption surfaced
    /// loudly"* — row 1 for every arm — and #649 falsified that with
    /// an executed counterexample. It is not row 2 for every arm
    /// either. Per source ([`crate::props::MassPropsError`]):
    ///
    /// * `Face` — **row 2**, and the reachable one. The body carries a
    ///   face whose measurement lane the kernel has not built: today
    ///   the closed forms need an iso-parameter rectangle
    ///   (`geom_brep::props`' `props_rim_level`, S58) and the
    ///   certified-quadrature lane consumes only conic/NURBS trims.
    ///   #649's `cross.step` — a real manifold closed keyed shaft
    ///   whose cylindrical walls have a cross-shaped iso domain — and
    ///   the same solid produced from rectangular sub-faces by the
    ///   public `Body::merge_coplanar_faces` are both perfectly valid
    ///   and both land here.
    /// * `Corrupt`, `NullScaffoldEdge`, `RingOnCurvedFace` — **row 1**
    ///   by their own docs: unresolvable structure, a mid-surgery
    ///   body carrying M3 null-edge scaffolding, and a curved face
    ///   with interior rings no M2 construction produces.
    /// * `Band` — neither: a misconfigured ambient tolerance, a
    ///   configuration failure of the run rather than a statement
    ///   about the body. It is also the arm this file's own
    ///   every-variant fixture builds the variant with.
    ///
    /// What keeps the row-1 arms **near**-unreachable here is not a
    /// property of this site but the `if errors.is_empty()` gate on
    /// check 7's `mass_properties_with` call: it runs only after every
    /// structural check has passed, so a corrupt body has normally
    /// already been refused by the check that names its corruption.
    /// "Normally" is the honest word — that gate is a sequencing fact
    /// elsewhere in this file, not an invariant this variant enforces,
    /// and `mass_properties` called directly has no such guard.
    VolumeUncomputable {
        /// The mass-properties failure.
        source: crate::props::MassPropsError,
    },
    /// **Tier 3, check 8 (M5 PR 6).** A stored pcurve cache failed its
    /// at-rest pass: missing on a chart that mints, failed
    /// re-certification (the meters-through-the-map residual, the
    /// closed-form envelope, or trim containment), or broke its
    /// face loop's one-branch chart continuity. The typed finding is
    /// nested whole.
    Pcurve {
        /// The pcurve pass's typed finding.
        finding: crate::pcurves::PcurveMintError,
    },
    /// Tier 3′ (M3 PR 6a): the global coincidence census found a
    /// position coincidence between distinct entities that no declared
    /// contact record backs (directly, or via the D3 segment
    /// reconstruction) — an **undeclared contact** is a hard error,
    /// never blessed (F1/F2(iii): discovery ≠ declaration, ever).
    UndeclaredContact {
        /// The coinciding entity pair, typed by census kind.
        contact: CensusContact,
        /// A debug rendering of the witnessing position.
        witness: String,
    },
    /// Tier 3′: a declared contact record has no geometric witness —
    /// the named entities do not coincide (or no longer resolve).
    /// Silent tolerance of unconfirmed declarations is forbidden
    /// (F1/F2(iii), the other direction of the certification diff).
    StaleContactDeclaration {
        /// The unconfirmed declaration.
        declaration: StaleDeclaration,
    },
    /// Tier 3′ (C4): a declared contact meets DEFINITE counter-evidence
    /// — the declaration is a lie where it meets the geometry.
    ///
    /// Distinct from [`Self::StaleContactDeclaration`], which is the
    /// *absence* of a witness: this is the presence of a
    /// contradiction, and it fires at the AT-REST gate as well as at
    /// use, because a false `Rest` must never be silent once it meets
    /// the zip (S1's deviation-2 strictness). Every definite verdict
    /// wins over every declaration.
    ContactContradicted {
        /// The face pair and class that were declared.
        declaration: DeclaredContact,
        /// A debug rendering of the witnessing site.
        witness: String,
        /// The margin that decided, and its predicate — the named
        /// number the message renders, never a bare "contradicted".
        margin: Indeterminate,
        /// Extra recourse steering when the counter-evidence has a
        /// named remedy (AQ6's designed-clearance arm).
        steer: Option<&'static str>,
    },
    /// Tier 3′: a census predicate escalated (sliver band or poison) —
    /// indeterminate coincidence geometry at rest is a defect (the
    /// standard trilean discipline: typed escalation, never a silent
    /// skip).
    CensusEscalated {
        /// The classifier's diagnostic.
        cause: Indeterminate,
    },
    /// Tier 3′: an entity or record outside the census's CERTIFIABLE
    /// inventory (M9-2 — the blanket exact-on-planar refusal retired
    /// with the census arms that replaced it). The census admits
    /// every carrier kind; this refusal now names the residue: a
    /// record or conformal candidate whose certifier lane refuses
    /// typed (no exact-constant-arm chart, seam-branch divergence,
    /// non-planar trims, a carrier kind outside the Rest ladder, a
    /// scalar with no certified lane). Refused loudly rather than
    /// sampled, exactly as before — only the inventory statement
    /// moved.
    CensusUnsupported {
        /// The unsupported entity.
        entity: EntityId,
    },
    /// Tier 3′ (M9-2 union fix, the conservative loudness backstop):
    /// a cross-solid candidate pair the census can neither examine
    /// nor definitely clear — refused loudly as UNDECIDABLE instead
    /// of silently not looked at (A5's letter: decide or refuse,
    /// never silently not-examine). Two classes fire it today: a
    /// cross-solid curved face pair within reach of each other (the
    /// C9-ring conformal-rest/proximity class — the exclusion ring
    /// is the certified excluder this backstop stands in for), and
    /// one instance's vertex hull inside another's REACH box (C6's
    /// interference class — representable only through recorded
    /// gate-skips, which do not exist yet; the containing side must be
    /// a superset of its locus or a nested body clears silently).
    CensusUndecidable {
        /// One side of the pair the census cannot clear.
        a: EntityId,
        /// The other side.
        b: EntityId,
        /// The not-yet-supported class, named.
        what: &'static str,
    },
    /// An entity holds a topology key that does not resolve in its arena.
    /// Reported once per occurrence (a parent listing the same dangling
    /// key twice yields two errors).
    DanglingTopology {
        /// The entity holding the dangling reference.
        from: EntityId,
        /// The dangling key (wrapped with its kind; it resolves to
        /// nothing).
        to: EntityId,
    },
    /// An entity holds a geometry key that does not resolve in its arena.
    DanglingGeometry {
        /// The entity holding the dangling reference.
        from: EntityId,
        /// The dangling geometry key.
        to: GeomRef,
    },
    /// `next(half_edge).prev != half_edge` — the doubly-linked cycle is
    /// torn between this half-edge and its successor.
    NextPrevMismatch {
        /// The half-edge whose successor does not point back.
        half_edge: HalfEdgeKey,
    },
    /// Walking `next` from the loop's `Cycle::first` failed to return to
    /// it within the arena bound (the walk wandered into a cycle not
    /// containing its start — always accompanied by the
    /// [`NextPrevMismatch`](Self::NextPrevMismatch) that made `next`
    /// non-injective).
    LoopCycleOverrun {
        /// The loop whose cycle does not close.
        loop_: LoopKey,
    },
    /// A half-edge reached by a loop's cycle has `parent_loop` pointing
    /// elsewhere.
    ParentLoopMismatch {
        /// The half-edge with the wrong back-pointer.
        half_edge: HalfEdgeKey,
        /// The loop whose cycle actually reaches it.
        owner: LoopKey,
    },
    /// A half-edge that its own `parent_loop`'s boundary does not reach:
    /// the parent's cycle closed without it, or the parent is an
    /// [`LoopBoundary::Empty`] loop (which reaches nothing).
    UnreachableHalfEdge {
        /// The unreachable half-edge.
        half_edge: HalfEdgeKey,
    },
    /// An edge whose two half-edge slots hold the same key — an edge
    /// must have two distinct halves.
    EdgeHalvesIdentical {
        /// The degenerate edge.
        edge: EdgeKey,
    },
    /// An edge slot's half-edge does not point back to the edge via
    /// `.edge`.
    EdgeSlotBackpointerMismatch {
        /// The claiming edge.
        edge: EdgeKey,
        /// The claimed half-edge whose `.edge` points elsewhere.
        half_edge: HalfEdgeKey,
    },
    /// A half-edge no edge slot claims (every half-edge must be exactly
    /// one edge's plus or minus half).
    HalfEdgeUnclaimed {
        /// The unclaimed half-edge.
        half_edge: HalfEdgeKey,
    },
    /// A half-edge claimed by more than one edge slot, counting
    /// multiplicity (an edge claiming it in both slots reports 2).
    HalfEdgeMultiplyClaimed {
        /// The multiply-claimed half-edge.
        half_edge: HalfEdgeKey,
        /// How many edge slots claim it (≥ 2).
        claims: usize,
    },
    /// An edge whose halves do not traverse it in opposite directions:
    /// `end(he_plus) != start(he_minus)` or
    /// `end(he_minus) != start(he_plus)`.
    EdgeNotAntiparallel {
        /// The misoriented edge.
        edge: EdgeKey,
    },
    /// A vertex's `emanating` half-edge does not start at the vertex.
    EmanatingStartMismatch {
        /// The vertex.
        vertex: VertexKey,
        /// Its `emanating` half-edge (which starts elsewhere).
        emanating: HalfEdgeKey,
    },
    /// A vertex with an `emanating` half-edge that is also the lone
    /// vertex of one or more empty loops — a lone vertex must have no
    /// half-edges (`emanating: None`).
    EmptyLoopVertexWithEmanating {
        /// The doubly-anchored vertex.
        vertex: VertexKey,
        /// How many empty loops hold it (≥ 1).
        empty_loops: usize,
    },
    /// A vertex with `emanating: None` at which half-edges nevertheless
    /// start — a lone vertex must have none.
    LoneVertexWithIncidence {
        /// The vertex claiming to be lone.
        vertex: VertexKey,
        /// How many half-edges start at it (≥ 1).
        incident: usize,
    },
    /// Walking the vertex orbit (`next(mate(he))`) from `emanating`
    /// failed to return to it within the arena bound (always accompanied
    /// by the chain/bijection error that broke the orbit permutation).
    VertexOrbitOverrun {
        /// The vertex whose orbit does not close.
        vertex: VertexKey,
    },
    /// The vertex orbit visited a half-edge that does not start at the
    /// vertex — the orbit permutation is crossing between vertices.
    OrbitForeignMember {
        /// The vertex whose orbit was walked.
        vertex: VertexKey,
        /// The visited half-edge that starts elsewhere.
        half_edge: HalfEdgeKey,
    },
    /// The vertex orbit closed but did not visit every half-edge
    /// starting at the vertex: the incident half-edges fall into more
    /// than one orbit — a non-manifold vertex (the "bowtie" case).
    SplitVertexOrbit {
        /// The non-manifold vertex.
        vertex: VertexKey,
        /// How many half-edges the closed orbit visited.
        orbit: usize,
        /// How many half-edges start at the vertex (> `orbit`).
        incident: usize,
    },
    /// A face listing its outer loop among its rings — `outer` is
    /// excluded from `rings` by construction (see [`crate::Face`]).
    OuterListedAsRing {
        /// The offending face.
        face: FaceKey,
    },
    /// A spine back-pointer (`loop.face`, `face.shell`, `shell.solid`)
    /// that does not match the entity's actual (unique) owner.
    BackPointerMismatch {
        /// The entity with the wrong back-pointer.
        child: EntityId,
        /// What the back-pointer stores.
        stored: EntityId,
        /// The parent that actually owns the child.
        owner: EntityId,
    },
    /// A spine entity nothing anchors: a shell in no solid, a face in no
    /// shell, a loop in no face (outer or ring), or a vertex at which no
    /// half-edge starts and which no empty loop holds.
    OrphanEntity {
        /// The unreferenced entity.
        entity: EntityId,
    },
    /// A shell, face, or loop referenced more than once by its parent
    /// kind (the containment spine is a tree of ownership down to
    /// loops), or an empty-loop vertex held by more than one empty loop.
    /// Multiplicity counts: one parent listing a child twice reports 2.
    MultiplyOwned {
        /// The multiply-referenced child.
        child: EntityId,
        /// How many owning references were found (≥ 2).
        owners: usize,
    },
    /// A geometry-arena entry no entity references. An error, not a
    /// warning: bodies are values built by operators, and nothing should
    /// leak (see the [module docs](self)).
    OrphanGeometry {
        /// The unreferenced geometry entry.
        geometry: GeomRef,
    },
    /// A solid with no shells. Defined by the operator set: `mvfs`
    /// births solid + shell + face together and `kvfs` retires them
    /// together, so a bare solid is operator-unreachable (module docs,
    /// check 9).
    SolidWithoutShells {
        /// The shell-less solid.
        solid: SolidKey,
    },
    /// A shell with no faces. Operator-unreachable, like
    /// [`SolidWithoutShells`](Self::SolidWithoutShells).
    ShellWithoutFaces {
        /// The face-less shell.
        shell: ShellKey,
    },
    /// An edge whose two halves' loops belong to faces in *different
    /// shells*. No Euler operator crosses shells, so this is
    /// operator-unreachable — the shell-partition/edge-adjacency
    /// coherence gap named by the PR 1 review (module docs, check 10).
    EdgeAcrossShells {
        /// The shell-crossing edge.
        edge: EdgeKey,
        /// The shell of the plus half's face.
        shell_plus: ShellKey,
        /// The shell of the minus half's face (≠ `shell_plus`).
        shell_minus: ShellKey,
    },
    /// A connected component of a shell's incidence complex that fails
    /// the Euler–Poincaré identity `v − e + f − r = 2(1 − g)` for every
    /// non-negative integer genus `g` — i.e. its `v − e + f − r` is odd
    /// or exceeds 2 (module docs, check 11). Carries the component's
    /// counts so the report is self-contained.
    ComponentEulerViolation {
        /// The shell whose complex contains the component.
        shell: ShellKey,
        /// The component's seed: its first face in face-arena order
        /// (the deterministic component identity — module docs).
        seed: FaceKey,
        /// Vertices in the component.
        vertices: usize,
        /// Edges in the component.
        edges: usize,
        /// Faces in the component.
        faces: usize,
        /// Rings (interior loops) across the component's faces.
        rings: usize,
    },
    /// A live topology entity with no D5 provenance record — every
    /// entity records its birth, so a missing record means the
    /// `SecondaryMap` was corrupted (module docs, check 12).
    MissingProvenance {
        /// The record-less entity.
        entity: EntityId,
    },
    /// A D5 provenance record whose entity is no longer live — the
    /// `SecondaryMap` leak a kill would cause by removing an entity
    /// without removing its record (module docs, check 12).
    LeakedProvenance {
        /// The dead entity the leaked record still names.
        entity: EntityId,
    },
    /// **Tier 2 only.** An empty loop on a body validated as a closed
    /// solid — construction scaffolding (the `mvfs`/`kemr` intermediate
    /// state) that must be resolved before a body is finished.
    ScaffoldingEmptyLoop {
        /// The empty loop.
        loop_: LoopKey,
    },
    /// **Tier 2 only.** A valence-1 vertex (the tip of a strut edge) on
    /// a body validated as a closed solid — construction scaffolding.
    ScaffoldingStrutVertex {
        /// The valence-1 vertex.
        vertex: VertexKey,
    },
    /// **Tier 2 only.** A shell whose incidence complex has more than
    /// one connected component (c ≠ 1) on a body validated as a closed
    /// solid. Not implied by the other two tier-2 bans: promoting a
    /// detached cycle ring disconnects a shell with no empty loops and
    /// no struts anywhere (module docs).
    ShellDisconnected {
        /// The disconnected shell.
        shell: ShellKey,
        /// How many components its complex has (≥ 2).
        components: usize,
    },
    /// **Tier 1, pass 13 (M3 PR 1).** A null-scaffold curve entry
    /// (`crate::CurveGeom::NullScaffold`) is referenced by more than
    /// one edge: the F9 attribute is per-edge data, so sharing is a
    /// corrupt state (each null edge mints its own entry; zero
    /// references is pass 8's `OrphanGeometry` report).
    NullScaffoldShared {
        /// The multiply-referenced scaffolding entry.
        curve: CurveKey,
        /// How many edges reference it (≥ 2).
        edges: usize,
    },
    /// **Tier 1, pass 13 (M3 PR 1).** A null-face record's face key
    /// does not resolve — the `SecondaryMap` leak a face-killing
    /// operator would cause by removing a marked face without its
    /// record (the provenance-leak rule applied to F9 records).
    LeakedNullFaceRecord {
        /// The dead face key still carrying a record.
        face: FaceKey,
    },
    /// **Tier 1, pass 13 (M3 PR 1, review flag c).** A null-face
    /// record names a loop key that does not resolve in the loop
    /// arena — a loop-killing operator ran without scrubbing the
    /// record (the same leak rule as `LeakedNullFaceRecord`, applied
    /// to the record's named loops). Referential-only by the ratified
    /// posture: *which* loops the record names is semantics, not
    /// checked at tier 1.
    StaleNullFaceLoop {
        /// The face whose record is stale.
        face: FaceKey,
        /// The named loop key that no longer resolves.
        named_loop: LoopKey,
    },
    /// **Tier 2 (M3 PR 1).** A null edge at rest: the edge's curve
    /// entry is `crate::CurveGeom::NullScaffold` — mid-surgery
    /// scaffolding (ch. 14/15 splitting/boolean transients) that must
    /// be consumed before a body rests.
    NullEdgeAtRest {
        /// The scaffolding edge.
        edge: EdgeKey,
    },
    /// **Tier 2 (M3 PR 1).** A null face at rest: the face carries an
    /// F9 loop-role record (`crate::NullFacePair`) — mid-surgery
    /// scaffolding that must be consumed before a body rests.
    NullFaceAtRest {
        /// The marked face.
        face: FaceKey,
    },
}

/// A census-discovered coincidence between **distinct** entities (the
/// tier-3′ injectivity pass's finding kinds, M3 PR 6a). Interior means
/// strictly interior (endpoint/boundary coincidences surface through
/// the vertex-level kinds instead — one finding per configuration
/// class, deterministically).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CensusContact {
    /// Two distinct vertices at one position.
    VertexVertex {
        /// The lower-arena-order vertex.
        a: VertexKey,
        /// The other vertex.
        b: VertexKey,
    },
    /// A vertex on a face's interior (off the face's boundary).
    VertexOnFace {
        /// The resting vertex.
        vertex: VertexKey,
        /// The face.
        face: FaceKey,
    },
    /// A vertex on an edge's interior — never declarable: reduction
    /// refines every such contact into v-v records before records are
    /// emitted (module docs, D4), so at rest this is always a defect.
    VertexOnEdge {
        /// The resting vertex.
        vertex: VertexKey,
        /// The edge.
        edge: EdgeKey,
    },
    /// An edge piercing a face transversally at both interiors — a
    /// proper crossing, categorically undeclarable (3′ allows touching,
    /// never crossing).
    EdgeFacePierce {
        /// The piercing edge.
        edge: EdgeKey,
        /// The pierced face.
        face: FaceKey,
    },
    /// Two edges crossing at both interiors (coplanar or skew-with-
    /// contact) — a proper crossing, categorically undeclarable.
    EdgeEdgeCross {
        /// The lower-arena-order edge.
        a: EdgeKey,
        /// The other edge.
        b: EdgeKey,
    },
    /// Two collinear edges overlapping on a positive-length segment
    /// (certifiable via the D3 bounding-record reconstruction).
    EdgeEdgeOverlap {
        /// The lower-arena-order edge.
        a: EdgeKey,
        /// The other edge.
        b: EdgeKey,
    },
    /// An edge lying in a face's plane with a positive-length
    /// sub-segment inside the face region (certifiable via D3).
    EdgeFaceOverlap {
        /// The resting edge.
        edge: EdgeKey,
        /// The face.
        face: FaceKey,
    },
    /// Two curved faces on ONE carrier with opposed senses whose trim
    /// regions definitely overlap in the shared chart (M9-2's
    /// conformal face-pair arm) — carried as the kernel contact
    /// FINDING, so the refusal quotes exactly the declaration that
    /// would verify (M9-1's layering ruling: one vocabulary
    /// end-to-end).
    ConformalPatch {
        /// What was found: the face pair, its class (`Rest`), and the
        /// definite verdict the arm's doors reported.
        finding: crate::contact::ContactFinding,
    },
}

/// A declared contact the census could not confirm (tier 3′).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StaleDeclaration {
    /// A v-v record whose vertices are dead, equal, or not coincident.
    VertexVertex {
        /// The record's A-side vertex.
        a: VertexKey,
        /// The record's B-side vertex.
        b: VertexKey,
    },
    /// A v-on-f record whose vertex is dead / face dead / vertex not
    /// resting on the face interior.
    VertexOnFace {
        /// The record's vertex.
        vertex: VertexKey,
        /// The record's face.
        face: FaceKey,
    },
    /// A curve-granularity record whose faces or witness edge no
    /// longer resolve — the locus that certified it is gone.
    CurveLocus {
        /// The record's A-side face.
        face_a: FaceKey,
        /// The record's B-side face.
        face_b: FaceKey,
        /// The record's witness edge.
        witness: crate::entity::EdgeKey,
    },
    /// A patch-granularity record whose faces are dead or whose trim
    /// regions definitely no longer overlap in the shared chart
    /// (overlap Empty ⇒ stale, C3's letter).
    Patch {
        /// The record's A-side face.
        face_a: FaceKey,
        /// The record's B-side face.
        face_b: FaceKey,
    },
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Band { error } => write!(f, "tier 3: {error}"),
            Self::DanglingDescription { from, to } => {
                write!(
                    f,
                    "{from}'s description references {to}, which does not resolve"
                )
            }
            Self::UncertifiableSurface { face } => write!(
                f,
                "face {face:?}'s surface is the Nurbs PLACEHOLDER (mvfs's all-poison \
                 'no description yet' state) — uncertifiable at rest; attach the real \
                 surface. A described NURBS surface passes this check"
            ),
            Self::EdgeCertification { edge, error } => {
                write!(f, "edge {edge:?} failed re-certification at rest: {error}")
            }
            Self::DescriptionNotAdjacent { edge } => write!(
                f,
                "edge {edge:?}'s intrinsic/seam description names surfaces that are not \
                 its adjacent faces' surfaces (D2 adjacency coherence)"
            ),
            Self::PlanarFaceResidual { face, vertex } => write!(
                f,
                "vertex {vertex:?} lies definitely off planar face {face:?}'s stored \
                 plane (D4 \u{b6}2 residual)"
            ),
            Self::PlanarFaceEscalated {
                face,
                vertex,
                cause,
            } => write!(
                f,
                "vertex {vertex:?}'s residual against planar face {face:?}'s plane \
                 escalated: {cause}"
            ),
            Self::PlanarBoundaryResidual { face, edge } => write!(
                f,
                "edge {edge:?}'s carrier lies definitely off planar face {face:?}'s \
                 stored plane between its vertices (face-boundary containment, D4 \u{b6}2)"
            ),
            Self::PlanarBoundaryEscalated { face, edge, cause } => write!(
                f,
                "edge {edge:?}'s carrier residual against planar face {face:?}'s plane \
                 escalated: {cause}"
            ),
            Self::SliverDihedral { edge, cause } => write!(
                f,
                "edge {edge:?}'s dihedral wedge cannot be classified definitely \
                 (sliver, D4 \u{b6}3): {cause}"
            ),
            Self::TransverseNotIntrinsic { edge } => write!(
                f,
                "edge {edge:?} is definitely transverse at every interior sample but \
                 carries a conventional MappedCurve description — transverse edges must \
                 be described intrinsically as the Intersection of their faces' surfaces \
                 (prefer-intrinsic, D2)"
            ),
            Self::TangentNotIntrinsic { edge } => write!(
                f,
                "edge {edge:?} is a jet-determinate tangency (definitely smooth at \
                 every interior sample, second-order separation definitely positive — \
                 the surfaces DETERMINE the locus) but carries a conventional \
                 MappedCurve description — such edges must be described intrinsically \
                 as the TangentIntersection of their faces' surfaces (prefer-intrinsic \
                 one order up; a G2 join is exempt by its zero-side \
                 second-order margin, never by a list)"
            ),
            Self::LoopRoleInverted { face, r#loop } => write!(
                f,
                "face {face:?}: loop {loop:?}'s winding disagrees with its outer/ring role \
                 (the outer loop must wind positively around the outward normal, rings \
                 negatively — the planar region-bounding statement)"
            ),
            Self::CurvedSenseInverted { face } => write!(
                f,
                "face {face:?}: the stored sense bit disagrees with the material side \
                 the face's own boundary traversal encodes (rim side / meridian \
                 orientation) — the face is inside-out; the two orientation encodings \
                 of a curved face must state the same side"
            ),
            Self::NegativeVolume => f.write_str(
                "the body's exact-B-rep signed volume is definitely negative — global \
                 orientation corruption (+V invariant; every closed body's outward-oriented \
                 boundary encloses positive volume)",
            ),
            Self::VolumeUncomputable { source } => write!(
                f,
                "the exact-B-rep volume for the +V invariant could not be computed: {source}"
            ),
            // The TWO-arm menu (SELECT-DESIGN §3d, ratified), not the
            // three-arm decidability sentence: a contact refusal is
            // about intent nobody recorded, and lowering ε cannot
            // supply intent — it can only hide its absence.
            Self::UndeclaredContact { contact, witness } => write!(
                f,
                "tier-3′ census: undeclared contact {contact:?} at {witness} — \
                 touching must be backed by a declared-contact record, never \
                 blessed from discovery; {}",
                crate::contact::CONTACT_RECOURSE
            ),
            Self::ContactContradicted {
                declaration,
                witness,
                margin,
                steer,
            } => write!(
                f,
                "tier-3′ census: the declared {} contact between faces {:?} and {:?} is \
                 contradicted at {witness} by {} — every definite verdict wins over every \
                 declaration; {}{}",
                declaration.class.name(),
                declaration.a,
                declaration.b,
                margin.payload(),
                crate::contact::CONTACT_RECOURSE,
                steer.map(|s| format!(" — {s}")).unwrap_or_default(),
            ),
            Self::StaleContactDeclaration { declaration } => write!(
                f,
                "tier-3′ census: declaration {declaration:?} has no geometric \
                 witness — stale contact records are defects, not noise"
            ),
            // `{cause}` (Display), NOT `{cause:?}`: the S6 sweep fixed a
            // Debug-format bug here that dropped the carrier's recourse
            // sentence from the user-facing message entirely.
            Self::CensusEscalated { cause } => write!(
                f,
                "tier-3′ census predicate escalated: {cause} — indeterminate \
                 coincidence geometry at rest is a defect"
            ),
            Self::CensusUnsupported { entity } => write!(
                f,
                "tier-3′ census: {entity} is outside the census's certifiable \
                 inventory — the census admits every carrier kind, but \
                 this record or conformal candidate has no certifier lane \
                 (exact-constant-arm charts, the Rest carrier ladder and the \
                 jet schedule are the certified set; the inf-stretch-bounds \
                 extension is the named follow-up). Refused rather than \
                 sampled; declare and certify through a supported lane, or \
                 separate the geometry"
            ),
            Self::CensusUndecidable { a, b, what } => write!(
                f,
                "tier-3′ census: the pair {a} / {b} cannot be examined or definitely \
                 cleared by any census arm ({what}) — refused as undecidable rather \
                 than silently not looked at; separate the bodies, or wait for the \
                 named lane (the exclusion ring for curved proximity; the \
                 recorded gate-skips for declared interference)"
            ),
            Self::DanglingTopology { from, to } => {
                write!(f, "{from} references {to}, which does not resolve")
            }
            Self::DanglingGeometry { from, to } => {
                write!(f, "{from} references {to}, which does not resolve")
            }
            Self::NextPrevMismatch { half_edge } => write!(
                f,
                "half-edge {half_edge:?}'s next half-edge does not point back \
                 to it via prev"
            ),
            Self::LoopCycleOverrun { loop_ } => write!(
                f,
                "loop {loop_:?}'s cycle does not return to its first half-edge \
                 within the arena bound"
            ),
            Self::ParentLoopMismatch { half_edge, owner } => write!(
                f,
                "half-edge {half_edge:?} is in loop {owner:?}'s cycle but its \
                 parent_loop points elsewhere"
            ),
            Self::UnreachableHalfEdge { half_edge } => write!(
                f,
                "half-edge {half_edge:?} is not reached by its parent loop's \
                 boundary"
            ),
            Self::EdgeHalvesIdentical { edge } => write!(
                f,
                "edge {edge:?}'s two half-edge slots hold the same half-edge"
            ),
            Self::EdgeSlotBackpointerMismatch { edge, half_edge } => write!(
                f,
                "edge {edge:?} claims half-edge {half_edge:?}, which points at \
                 a different edge"
            ),
            Self::HalfEdgeUnclaimed { half_edge } => {
                write!(f, "half-edge {half_edge:?} is claimed by no edge slot")
            }
            Self::HalfEdgeMultiplyClaimed { half_edge, claims } => write!(
                f,
                "half-edge {half_edge:?} is claimed by {claims} edge slots \
                 (exactly one required)"
            ),
            Self::EdgeNotAntiparallel { edge } => write!(
                f,
                "edge {edge:?}'s half-edges do not traverse it in opposite \
                 directions"
            ),
            Self::EmanatingStartMismatch { vertex, emanating } => write!(
                f,
                "vertex {vertex:?}'s emanating half-edge {emanating:?} does \
                 not start at it"
            ),
            Self::EmptyLoopVertexWithEmanating {
                vertex,
                empty_loops,
            } => write!(
                f,
                "vertex {vertex:?} has an emanating half-edge but is the lone \
                 vertex of {empty_loops} empty loop(s)"
            ),
            Self::LoneVertexWithIncidence { vertex, incident } => write!(
                f,
                "vertex {vertex:?} has no emanating half-edge but {incident} \
                 half-edge(s) start at it"
            ),
            Self::VertexOrbitOverrun { vertex } => write!(
                f,
                "vertex {vertex:?}'s orbit does not return to its emanating \
                 half-edge within the arena bound"
            ),
            Self::OrbitForeignMember { vertex, half_edge } => write!(
                f,
                "vertex {vertex:?}'s orbit visits half-edge {half_edge:?}, \
                 which starts at a different vertex"
            ),
            Self::SplitVertexOrbit {
                vertex,
                orbit,
                incident,
            } => write!(
                f,
                "vertex {vertex:?} is non-manifold: its orbit closes over \
                 {orbit} half-edge(s) but {incident} start at it"
            ),
            Self::OuterListedAsRing { face } => {
                write!(f, "face {face:?} lists its outer loop among its rings")
            }
            Self::BackPointerMismatch {
                child,
                stored,
                owner,
            } => write!(f, "{child} points back at {stored} but is owned by {owner}"),
            Self::OrphanEntity { entity } => {
                write!(f, "{entity} is anchored by no parent entity")
            }
            Self::MultiplyOwned { child, owners } => write!(
                f,
                "{child} has {owners} owning references (exactly one required)"
            ),
            Self::OrphanGeometry { geometry } => {
                write!(f, "{geometry} is referenced by no entity")
            }
            Self::SolidWithoutShells { solid } => {
                write!(f, "solid {solid:?} has no shells (every solid has ≥ 1)")
            }
            Self::ShellWithoutFaces { shell } => {
                write!(f, "shell {shell:?} has no faces (every shell has ≥ 1)")
            }
            Self::EdgeAcrossShells {
                edge,
                shell_plus,
                shell_minus,
            } => write!(
                f,
                "edge {edge:?}'s two halves lie in different shells \
                 ({shell_plus:?} vs {shell_minus:?})"
            ),
            Self::ComponentEulerViolation {
                shell,
                seed,
                vertices,
                edges,
                faces,
                rings,
            } => {
                let chi = *vertices as i64 - *edges as i64 + *faces as i64 - *rings as i64;
                write!(
                    f,
                    "shell {shell:?}'s component seeded at face {seed:?} violates \
                     Euler–Poincaré: v − e + f − r = {vertices} − {edges} + {faces} \
                     − {rings} = {chi}, which is not 2(1 − g) for any integer g ≥ 0"
                )
            }
            Self::MissingProvenance { entity } => {
                write!(f, "{entity} is live but has no D5 provenance record")
            }
            Self::LeakedProvenance { entity } => write!(
                f,
                "a D5 provenance record exists for {entity}, which is not live \
                 (leaked past its entity's death)"
            ),
            Self::ScaffoldingEmptyLoop { loop_ } => write!(
                f,
                "loop {loop_:?} is an empty loop — construction scaffolding, \
                 banned on a closed solid (tier 2)"
            ),
            Self::ScaffoldingStrutVertex { vertex } => write!(
                f,
                "vertex {vertex:?} has valence 1 (a strut tip) — construction \
                 scaffolding, banned on a closed solid (tier 2)"
            ),
            Self::ShellDisconnected { shell, components } => write!(
                f,
                "shell {shell:?}'s incidence complex has {components} connected \
                 components (a closed solid requires exactly 1, tier 2)"
            ),
            Self::NullScaffoldShared { curve, edges } => write!(
                f,
                "null-scaffold curve entry {curve:?} is referenced by {edges} \
                 edges (the null-scaffold attribute is per-edge data — sharing is corrupt)"
            ),
            Self::LeakedNullFaceRecord { face } => write!(
                f,
                "null-face record outlives its face {face:?} (record leak — \
                 face kills must remove the record)"
            ),
            Self::StaleNullFaceLoop { face, named_loop } => write!(
                f,
                "null-face record on face {face:?} names loop {named_loop:?}, \
                 which does not resolve (record leak — loop kills must \
                 scrub records naming the loop)"
            ),
            Self::NullEdgeAtRest { edge } => write!(
                f,
                "edge {edge:?} is null-edge scaffolding at rest (tier 2 bans \
                 unconsumed surgery transients)"
            ),
            Self::NullFaceAtRest { face } => write!(
                f,
                "face {face:?} carries a null-face record at rest (tier 2 bans \
                 unconsumed surgery transients)"
            ),
            Self::Pcurve { finding } => write!(f, "tier 3: {finding}"),
        }
    }
}

impl std::error::Error for ValidationError {}

/// Counts one resolved reference to `key` (dangling keys never reach
/// here). `SecondaryMap` rather than a hash map: typed per key kind and
/// deterministic to sweep, per the crate's D9 iteration invariant.
fn count_ref<K: Key>(counts: &mut SecondaryMap<K, usize>, key: K) {
    let n = counts.get(key).copied().unwrap_or(0);
    counts.insert(key, n + 1);
}

/// Counts one owning reference to `key` and remembers the owner. Only
/// the count is meaningful when it ends up ≠ 1; the remembered owner is
/// consulted for back-pointer comparison exactly when the count is 1
/// (in which case it is *the* owner).
fn count_owner<K: Key, O: Copy>(counts: &mut SecondaryMap<K, (usize, O)>, key: K, owner: O) {
    let n = counts.get(key).map_or(0, |(n, _)| *n);
    counts.insert(key, (n + 1, owner));
}

/// The counts of one connected component of a shell's incidence complex
/// (pass 11 — see the module docs for the glue rules).
struct ComponentCounts {
    vertices: usize,
    edges: usize,
    faces: usize,
    rings: usize,
}

impl ComponentCounts {
    /// Whether `v − e + f − r = 2(1 − g)` holds for some integer
    /// `g ≥ 0`: the characteristic must be even and at most 2.
    ///
    /// The `chi > 2` (negative-genus) arm is believed
    /// defensive-unreachable: the PR 5 review could not construct it
    /// even with gate-passing corruption — passes 3/4/6 force each
    /// component to be a closed oriented surface (χ ≤ 2 automatic
    /// within a coherent shell), and per-shell *cutting* (the
    /// moved-face family) only ever produced odd χ. Kept as defense in
    /// depth; untested until someone falsifies that derivation.
    fn satisfies_euler_poincare(&self) -> bool {
        let chi = self.vertices as i64 - self.edges as i64 + self.faces as i64 - self.rings as i64;
        chi % 2 == 0 && chi <= 2
    }
}

/// Tier 1's full result: the error vector plus the per-shell component
/// counts when the component pass ran (`None` when its gate — passes
/// 1–7 clean — did not hold). Tier 2 reuses the counts for its c = 1
/// check instead of re-enumerating.
struct Tier1Report {
    errors: Vec<ValidationError>,
    /// `(shell, component count)` in shell-arena order.
    shell_components: Option<Vec<(ShellKey, usize)>>,
}

/// Validates a body's **tier-1** ("euler-valid") structural coherence,
/// collecting **all** failures.
///
/// The check set, the report order, the cascade discipline, and the two
/// validity tiers are documented in the [module docs](self). The empty
/// body validates vacuously. Every Euler-reachable state passes;
/// construction scaffolding is banned only by tier 2
/// ([`validate_closed`]).
///
/// # Errors
///
/// A non-empty vector of every [`ValidationError`] found, in the
/// documented deterministic order.
pub fn validate<T: Real>(body: &Body<T>) -> Result<(), Vec<ValidationError>> {
    let report = tier1(body);
    if report.errors.is_empty() {
        Ok(())
    } else {
        Err(report.errors)
    }
}

/// Validates a body as a **tier-2 "closed solid"**: all of tier 1
/// ([`validate`]) plus the at-rest bans on construction scaffolding —
/// no empty loops, no valence-1 vertices (struts), and every shell's
/// incidence complex connected (c = 1). See the [module docs](self) for
/// the ratified tier semantics (laminae are deliberately *not* banned).
///
/// Tier-2 failures are appended after all tier-1 errors, in the
/// documented order: empty loops (loop-arena order), valence-1 vertices
/// (vertex-arena order), disconnected shells (shell-arena order). The
/// connectivity check shares pass 11's gate (structural soundness of
/// passes 1–7) and its component enumeration; a face-less shell is
/// reported by the tier-1 arity floor alone (c = 0 is not double-
/// reported here). The empty body validates vacuously.
///
/// # Errors
///
/// A non-empty vector of every [`ValidationError`] found, tier 1 first,
/// in the documented deterministic order.
pub fn validate_closed<T: Real>(body: &Body<T>) -> Result<(), Vec<ValidationError>> {
    let Tier1Report {
        mut errors,
        shell_components,
    } = tier1(body);

    // Tier 2, check 1: no empty loops (loop-arena order).
    for (loop_key, loop_) in body.loops.iter() {
        if matches!(loop_.boundary, LoopBoundary::Empty { .. }) {
            errors.push(ValidationError::ScaffoldingEmptyLoop { loop_: loop_key });
        }
    }

    // Tier 2, check 2: no valence-1 vertices (vertex-arena order).
    // Valence = the number of half-edges starting at the vertex (a
    // self-loop contributes two). Re-derived here rather than threaded
    // out of pass 1: the count only reads resolving `start` references,
    // so it is meaningful even on bodies with unrelated tier-1 errors —
    // with one documented echo: a dangling `start` (a pass-1 error)
    // deflates its true vertex's count and can surface here as a
    // spurious strut (module docs, cascade discipline; pinned by the
    // promoted review probe; unreachable through the public API).
    let mut incidence: SecondaryMap<VertexKey, usize> = SecondaryMap::new();
    for (_, he) in body.half_edges.iter() {
        if body.vertices.contains_key(he.start) {
            count_ref(&mut incidence, he.start);
        }
    }
    for (vertex_key, _) in body.vertices.iter() {
        if incidence.get(vertex_key).copied().unwrap_or(0) == 1 {
            errors.push(ValidationError::ScaffoldingStrutVertex { vertex: vertex_key });
        }
    }

    // Tier 2, check 3: c = 1 per shell (shell-arena order), from the
    // tier-1 component enumeration. c = 0 (a face-less shell) is the
    // tier-1 arity floor's report, not a disconnection.
    if let Some(per_shell) = shell_components {
        for (shell, components) in per_shell {
            if components >= 2 {
                errors.push(ValidationError::ShellDisconnected { shell, components });
            }
        }
    }

    // Tier 2, check 4 (M3 PR 1): no null entities at rest — null edges
    // (edge-arena order) and null-face records (record slot order) are
    // ch. 14/15 surgery transients, refused by name exactly like the
    // other scaffolding shapes (see `crate::null`).
    for (edge_key, edge) in body.edges.iter() {
        if matches!(
            body.curves.get(edge.curve),
            Some(CurveGeom::NullScaffold(_))
        ) {
            errors.push(ValidationError::NullEdgeAtRest { edge: edge_key });
        }
    }
    for (face_key, _) in body.null_faces.iter() {
        if body.faces.contains_key(face_key) {
            errors.push(ValidationError::NullFaceAtRest { face: face_key });
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Validates a body as a **tier-3 "geometric" solid** (M2 PR 3 — the
/// tier's start): all of tier 2 ([`validate_closed`]), then the
/// geometric re-checks at rest, in documented order:
///
/// 1. **Surface implementedness** (faces, arena order): no face's
///    surface is the `Nurbs` representable-unimplemented placeholder
///    ([`ValidationError::UncertifiableSurface`]) — nothing can be
///    certified against it at M2.
/// 2. **Carrier re-certification** (edges, arena order): every edge's
///    stored [`geom_brep::EdgeCurve`] re-runs its full D4 ¶2
///    certification — endpoint pinning against the edge's own vertices'
///    points (`he_plus` forward order), description residuals at the
///    fixed schedule, transversality for `Intersection` — with the same
///    deterministic sampling as at attachment
///    ([`ValidationError::EdgeCertification`]); then the
///    **description-adjacency coherence** check: an
///    `Intersection`/`Seam` description's surfaces must be exactly the
///    edge's two faces' surfaces
///    ([`ValidationError::DescriptionNotAdjacent`]).
/// 3. **Planar-face residuals** (faces, arena order; per face the outer
///    loop then rings in list order, vertices in cycle order): every
///    vertex of a face whose surface is a plane lies within ε of that
///    plane ([`ValidationError::PlanarFaceResidual`] /
///    [`ValidationError::PlanarFaceEscalated`]) — the Newell-certified
///    plane equations re-checked at rest.
/// 4. **Dihedral classification** (edges, arena order): at the interior
///    schedule samples (i = 1…7 — endpoints excluded: vertices may sit
///    on surface singularities like the cone apex, where angular
///    classification honestly poisons), the wedge between the edge's
///    two faces' surfaces classifies **definitely** — transverse corner
///    or smooth seam, never sliver
///    ([`ValidationError::SliverDihedral`]; first failing sample
///    reported, one error per edge). Implicit-form gradients only —
///    chart normals are never sampled. The extent lever arm is the
///    edge's honest spatial extent (`geom_brep::edge_extent`: the chord
///    for open edges, the carrier diameter at closure for circle
///    carriers — so self-loop/full-period edges classify through a
///    real arm, never vacuously; M2 PR 3 fix pass). The same per-edge
///    classifications feed the **prefer-intrinsic enforcement** (D2;
///    ratified 2026-07-19, M2 PR 4 fix pass): an edge classifying
///    definitely Transverse at every interior sample must carry an
///    `Intersection` description
///    ([`ValidationError::TransverseNotIntrinsic`]); definitely-smooth
///    edges keep conventional descriptions, `Seam` edges are exempt by
///    kind, and escalated edges report only their `SliverDihedral`.
/// 5. **Planar-boundary containment** (same edge sweep, same samples;
///    M2 PR 3 fix pass): each interior carrier sample is checked
///    against each **adjacent planar** face's plane — the
///    between-vertices counterpart of check 3, so an honestly certified
///    carrier that bulges off its planar face's surface is reported
///    ([`ValidationError::PlanarBoundaryResidual`] /
///    [`ValidationError::PlanarBoundaryEscalated`]; plus face first,
///    then the minus face when distinct; first failing sample, one
///    error per edge–face pair). Curved-face containment is NOT
///    checked (the not-yet-checked list below; #638).
///
/// **Coarse gate** (the pass-11 philosophy): the geometric passes run
/// only when tiers 1–2 are clean — structural defects void geometric
/// interpretation and already carry their own reports. Requires
/// `T: Decide` (geometric validation classifies; the structural tiers
/// never do).
///
/// # What tier 3 does NOT yet check (deferred, named)
///
/// - **Global self-intersection / minimum clearance** — M3 partial
///   (via booleans); the interval-based whole-body check is scheduled
///   rather than dropped. `DESIGN.md` carries it twice — in the
///   tier-3 bullet of the validation-tier list, and in the milestone
///   list under the entry containing *"interval-based
///   self-intersection / minimum-clearance checks over the parameter
///   box"*. Quoted rather than dated on purpose: a milestone copied
///   into this list is the rot the list keeps producing, and a quote
///   is what survives the milestone being renumbered.
/// - **The material wedge side** (lamina/zero-volume detection, wedge
///   0 vs 2π vs the legal π): distinguishing them needs the faces'
///   material sense **at the edge** — which side of each surface the
///   solid occupies *there*. Since M5 S10 the per-face half of that is
///   no longer missing: [`crate::Face::sense`] states it globally per
///   face (outward normal = `sense_sign · chart normal`), and check 6
///   below certifies the statement against the loop windings on planar
///   faces. What is still absent is the *edge-local* pairing —
///   orienting each face's tangent plane at a sample of the shared
///   carrier and measuring the wedge the two material sides subtend,
///   which wants the curved-face pcurve machinery. At present the
///   dihedral pass (check 4) classifies the *tangent-plane* wedge
///   only: unsigned, so it sees a sliver but not a lamina.
/// - **Face-boundary containment on curved surfaces** (a face's loops
///   actually bounding a region of its surface) — tracked at #638.
///   The **planar** case is now covered between vertices by check 5
///   (sample containment against adjacent planar faces), and its
///   orientation half by check 6 (loop-role winding against the
///   outward normal, line-bounded loops). The **curved analytic**
///   kinds' orientation half is covered by check 6's curved arm
///   (M6-6: boundary material side vs the sense bit); what remains
///   deferred is containment against curved surfaces and the
///   region-bounding statement for arc-bounded planar faces and
///   curved faces, plus the curved arm's documented residuals (the
///   rimless sphere band; NURBS faces; the quadrature-owned
///   conic-trimmed walls, whose boundary parse refuses typed and is
///   therefore exempt — such a body's flips, single-face AND
///   whole-body, certify green today; executed on the tilted-section
///   cylinder and pinned as residual).
/// - **Curve conventional-invariant certification** (unit `dir`/`axis`,
///   `u_ref ⊥ axis`): partially implied by the residual checks (a
///   non-unit frame breaks the carrier-vs-description comparisons),
///   not independently certified yet.
///
/// # Errors
///
/// A non-empty vector of every failure found: tiers 1–2 verbatim if
/// any, else the tier-3 failures in the documented order.
pub fn validate_geometric<T: crate::props::PropsQuadLane>(
    body: &Body<T>,
) -> Result<(), Vec<ValidationError>> {
    // Coarse gate: structural tiers first, verbatim.
    validate_closed(body)?;

    let band = match Band::linear() {
        Ok(band) => band,
        Err(error) => return Err(vec![ValidationError::Band { error }]),
    };
    let errors = tier3_local_checks(body, band);
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Tier 3's local check battery (checks 1–6 + the +V invariant, check
/// 7), shared verbatim between [`validate_geometric`] and
/// [`validate_pseudomanifold`] (M3 PR 6a: the tier-3′ validator runs
/// the SAME local passes — extraction, not copy-paste; behavior under
/// `validate_geometric` is identical to the pre-extraction code).
/// Assumes the tier-1/2 coarse gate already passed.
pub(crate) fn tier3_local_checks<T: crate::props::PropsQuadLane>(
    body: &Body<T>,
    band: Band,
) -> Vec<ValidationError> {
    let mut marks = slotmap::SecondaryMap::new();
    tier3_local_checks_marked(body, band, &mut marks)
}

/// The per-edge tier-3 contact MARKS at rest (OQ7 level (i), M5 PR 9):
/// runs the structural coarse gate, then the tier-3 battery, and
/// returns the recorded per-edge [`ContactMark`]s. `Err` carries every
/// validation failure exactly as [`validate_geometric`] would (marks
/// are only meaningful on a valid body).
///
/// # Errors
///
/// As [`validate_geometric`].
pub fn contact_marks<T: crate::props::PropsQuadLane>(
    body: &Body<T>,
) -> Result<slotmap::SecondaryMap<EdgeKey, ContactMark>, Vec<ValidationError>> {
    validate_closed(body)?;
    let band = match Band::linear() {
        Ok(band) => band,
        Err(error) => return Err(vec![ValidationError::Band { error }]),
    };
    let mut marks = slotmap::SecondaryMap::new();
    let errors = tier3_local_checks_marked(body, band, &mut marks);
    if errors.is_empty() {
        Ok(marks)
    } else {
        Err(errors)
    }
}

/// [`tier3_local_checks`] with the check-4 contact marks KEPT (the
/// same pass — never classifying twice; the mark is the verdict the
/// dihedral/jet loop derives anyway).
pub(crate) fn tier3_local_checks_marked<T: crate::props::PropsQuadLane>(
    body: &Body<T>,
    band: Band,
    marks: &mut slotmap::SecondaryMap<EdgeKey, ContactMark>,
) -> Vec<ValidationError> {
    let mut errors = Vec::new();

    // ------------------------------------------------------------------
    // Tier 3, check 1: surface implementedness (face-arena order).
    //
    // M6-3 flip A: a DESCRIBED NURBS surface (finite control net) is
    // real geometry — the loft/sweep assembly mints faces on it, its
    // seams certify through the IsoCurve lane, and its volume flux
    // goes through the quadrature door — so it passes here. What keeps
    // refusing is the mvfs PLACEHOLDER (all-poison control points),
    // which is a mid-surgery "no description yet" fact, never a
    // certifiable surface. One discriminator, shared:
    // `NurbsSurface::is_placeholder`.
    // ------------------------------------------------------------------
    for (face_key, face) in body.faces.iter() {
        if let Some(Surface::Nurbs(payload)) = body.surfaces.get(face.surface)
            && payload.is_placeholder()
        {
            errors.push(ValidationError::UncertifiableSurface { face: face_key });
        }
    }

    // ------------------------------------------------------------------
    // Tier 3, check 2: carrier re-certification + description adjacency
    // (edge-arena order). Tier 1 guarantees every key below resolves;
    // the `else` arms are unreachable and stay silent (cascade
    // discipline — the structural report already fired, and the gate
    // above means we never get here in that case).
    // ------------------------------------------------------------------
    for (edge_key, edge) in body.edges.iter() {
        // Null scaffolding cannot reach the tier-3 passes: the coarse
        // gate ran tier 2, which refuses null entities at rest.
        let Some(curve) = body.curves.get(edge.curve).and_then(CurveGeom::certified) else {
            continue;
        };
        let Some((p_start, p_end)) = edge_endpoints(body, edge.he_plus) else {
            continue;
        };
        // The at-rest pass takes the plane × NURBS DOOR (M7-8): this
        // tier already requires a CERTIFYING scalar (`PropsQuadLane` —
        // not merely a bracket-carrying one, which since D1
        // (2026-08-19) includes `Dual`), so the lane that certifies a described
        // NURBS operand is available here — and an imported body of
        // that class must re-derive its certificate at rest exactly as
        // it did at attach time. Re-certification re-derives; it never
        // trusts the stored certificate.
        if let Err(error) =
            curve.recertify_nurbs_lane(p_start, p_end, |k| body.surfaces.get(k).cloned(), band)
        {
            errors.push(ValidationError::EdgeCertification {
                edge: edge_key,
                error,
            });
        }
        let Some((fs_plus, fs_minus)) = edge_face_surfaces(body, edge.he_plus, edge.he_minus)
        else {
            continue;
        };
        let adjacent = match *curve.description() {
            geom_brep::EdgeGeometry::Intersection { s1, s2, .. }
            | geom_brep::EdgeGeometry::TangentIntersection { s1, s2, .. } => {
                (s1 == fs_plus && s2 == fs_minus) || (s1 == fs_minus && s2 == fs_plus)
            }
            geom_brep::EdgeGeometry::Seam { surface } => surface == fs_plus && surface == fs_minus,
            // Iso adjacency (M6-3, the M5-LOG item 6(iii) rule): the
            // described chart is ONE of the edge's two adjacent faces'
            // surfaces — a wall–wall seam is the u-boundary iso of
            // either wall, and the minted convention names one.
            geom_brep::EdgeGeometry::IsoCurve { surface, .. } => {
                surface == fs_plus || surface == fs_minus
            }
            geom_brep::EdgeGeometry::MappedCurve(_) => true,
        };
        if !adjacent {
            errors.push(ValidationError::DescriptionNotAdjacent { edge: edge_key });
        }
    }

    // ------------------------------------------------------------------
    // Tier 3, check 3: planar-face vertex residuals (face-arena order;
    // outer loop then rings; vertices in cycle order).
    //
    // S10 CATEGORY C (orientation-free): the margin `(p − origin)·n` is
    // tested against `Zero`, and `Zero` is invariant under negating
    // `n` — the plane as a POINT SET does not depend on which side the
    // material is. Threading `sense_sign` here would flip
    // Positive↔Negative in a decision that never distinguishes them.
    // ------------------------------------------------------------------
    for (face_key, face) in body.faces.iter() {
        let Some(&Surface::Plane { origin, normal, .. }) = body.surfaces.get(face.surface) else {
            continue;
        };
        for &loop_key in core::iter::once(&face.outer).chain(&face.rings) {
            let Some(loop_) = body.loops.get(loop_key) else {
                continue;
            };
            let LoopBoundary::Cycle { first } = loop_.boundary else {
                continue; // tier 2 banned empty loops; unreachable
            };
            let Some(cycle) = body.loop_cycle(first) else {
                continue;
            };
            for he in cycle {
                let Some(he_data) = body.half_edges.get(he) else {
                    continue;
                };
                let vertex = he_data.start;
                let Some(vertex_data) = body.vertices.get(vertex) else {
                    continue;
                };
                let Some(&point) = body.points.get(vertex_data.point) else {
                    continue;
                };
                let residual = (point - origin).dot(normal);
                match decide("planar_face_residual", Margin::of(residual), band) {
                    Ok(Sign::Zero) => {}
                    Ok(Sign::Positive | Sign::Negative) => {
                        errors.push(ValidationError::PlanarFaceResidual {
                            face: face_key,
                            vertex,
                        });
                    }
                    Err(cause) => {
                        errors.push(ValidationError::PlanarFaceEscalated {
                            face: face_key,
                            vertex,
                            cause,
                        });
                    }
                }
            }
        }
    }

    // ------------------------------------------------------------------
    // Tier 3, checks 4–5 (one edge sweep, samples shared):
    //
    // 4. Dihedral classification at interior samples (edge-arena order;
    //    first failing sample, one error per edge), metered through the
    //    edge's honest extent (`geom_brep::edge_extent` — the carrier
    //    diameter for closed circle carriers, whose chord collapses; M2
    //    PR 3 fix pass, B2 — self-loop edges no longer classify
    //    vacuously Smooth). Exempt BY KIND: `Seam`-described edges
    //    (as always) and Nurbs-ADJACENT edges (M6-3 flip B — see the
    //    in-loop note: implicit-form gradients are poison on NURBS,
    //    and the wall junction's contact class is declared, Q8/C11).
    // 5. Planar-boundary containment (M2 PR 3 fix pass, S3): the same
    //    interior carrier samples are checked against each ADJACENT
    //    face's surface when that surface is a plane — the
    //    between-vertices counterpart of check 3 (plus face first, then
    //    the minus face when distinct; first failing sample, one error
    //    per edge-face pair). Curved-face containment is NOT checked
    //    (`validate_geometric`'s not-yet-checked list; #638).
    //
    // S10 CATEGORY C, both: neither reads a face orientation at all.
    // Check 4 takes the two SURFACES (never the faces) and classifies
    // the UNSIGNED tangent-plane wedge from implicit-form gradients —
    // it distinguishes transverse from smooth, never which side the
    // material is on (that is the deferred material-wedge check, named
    // in the header). Check 5 tests `(p − origin)·n` against `Zero`,
    // sign-invariant for the same reason as check 3. `sense_sign` is
    // deliberately absent from both.
    // ------------------------------------------------------------------
    for (edge_key, edge) in body.edges.iter() {
        // Null scaffolding cannot reach the tier-3 passes: the coarse
        // gate ran tier 2, which refuses null entities at rest.
        let Some(curve) = body.curves.get(edge.curve).and_then(CurveGeom::certified) else {
            continue;
        };
        let Some((p_start, p_end)) = edge_endpoints(body, edge.he_plus) else {
            continue;
        };
        let Some(((f_plus, fs_plus), (f_minus, fs_minus))) =
            edge_adjacent_faces(body, edge.he_plus, edge.he_minus)
        else {
            continue;
        };
        let (Some(s_plus), Some(s_minus)) =
            (body.surfaces.get(fs_plus), body.surfaces.get(fs_minus))
        else {
            continue;
        };
        let chord = p_start.distance(p_end);
        let (t0, t1) = curve.params();
        let extent = geom_brep::edge_extent(curve.carrier(), t0, t1, chord);
        // The interior schedule samples, evaluated once (D9: the same
        // parameters certification used) and shared by checks 4 and 5.
        let samples: Vec<_> = (1..(geom_brep::CERT_SAMPLES - 1))
            .map(|i| curve.carrier().eval(curve.sample_param(i)))
            .collect();
        // Check 4: dihedral — plus the prefer-intrinsic enforcement
        // (D2; ratified 2026-07-19): reusing the SAME per-sample
        // classifications (never classifying twice), an edge whose
        // every interior sample is definitely Transverse must carry an
        // `Intersection` description at rest. `Seam` is exempt by
        // kind, definitely-smooth keeps `MappedCurve` (the D2
        // conventional split), an escalation reports `SliverDihedral`
        // and exempts the edge (ε-tightening escalates, it never flips
        // valid → invalid), and a mixed sample set is enforced as
        // neither.
        //
        // **Nurbs-adjacent edges are exempt BY KIND** (M6-3 flip B —
        // the `Seam` exemption idiom, one shelf over): implicit-form
        // gradients are poison on a NURBS surface, so
        // `classify_dihedral` cannot run there, and no derived contact
        // class exists to enforce. That is not a gap being papered
        // over — a definitional wall junction's contact class is the
        // profile's DECLARED corner structure (Q8/C11), carried by the
        // conventional `IsoCurve`/`MappedCurve` descriptions the
        // loft/sweep builder mints; the mark stays `Unmarked` (no
        // derived verdict, exactly the escalation posture).
        let nurbs_adjacent =
            matches!(s_plus, Surface::Nurbs(_)) || matches!(s_minus, Surface::Nurbs(_));
        let mut escalated = false;
        let mut all_transverse = true;
        let mut all_smooth = true;
        if !nurbs_adjacent {
            for &p in &samples {
                match classify_dihedral(s_plus, s_minus, p, extent, band) {
                    Ok(DihedralClass::Transverse) => all_smooth = false,
                    Ok(DihedralClass::Smooth) => all_transverse = false,
                    Err(cause) => {
                        errors.push(ValidationError::SliverDihedral {
                            edge: edge_key,
                            cause,
                        });
                        escalated = true;
                        break;
                    }
                }
            }
            if !escalated
                && all_transverse
                && matches!(curve.description(), geom_brep::EdgeGeometry::MappedCurve(_))
            {
                errors.push(ValidationError::TransverseNotIntrinsic { edge: edge_key });
            }
        }
        // The contact MARK (OQ7 level (i), M5 PR 9) and the symmetric
        // must-carry (level (ii)). A definitely-smooth edge descends
        // one order — the jet's second-order margin at the SAME
        // schedule samples: definite at every sample ⇒ a
        // jet-determinate tangency (the surfaces determine the locus);
        // a zero-side sample ⇒ under-determined (G2 conventional —
        // exempt BY THE PREDICATE); in-band ⇒ SliverDihedral with the
        // `tangent_second_order` cause (F6), which exempts the edge
        // here so ε-tightening never flips valid→invalid through the
        // must-carry. `Seam` edges are exempt by kind, as always —
        // and Nurbs-adjacent edges likewise (flip B above): no jet is
        // derivable from a poison implicit form, so they carry
        // `Unmarked`.
        // The two `Unmarked` arms are deliberately separate branches
        // (not `nurbs_adjacent || escalated`): one is an exemption BY
        // KIND, the other an escalation already reported — same mark,
        // different reasons, and the reader should see both.
        #[allow(clippy::if_same_then_else)]
        let mark = if nurbs_adjacent {
            ContactMark::Unmarked
        } else if escalated {
            ContactMark::Unmarked
        } else if matches!(curve.description(), geom_brep::EdgeGeometry::Seam { .. }) {
            ContactMark::Seam
        } else if all_transverse {
            ContactMark::Transverse
        } else if all_smooth {
            let mut jet_determinate = true;
            let mut jet_escalated = false;
            for i in 1..(geom_brep::CERT_SAMPLES - 1) {
                let t = curve.sample_param(i);
                let p = curve.carrier().eval(t);
                let jet = geom_brep::tangent_jet(s_plus, s_minus, p, curve.carrier().deriv(t));
                let arm = geom_brep::curvature_lever_arm(s_plus, p)
                    .min(geom_brep::curvature_lever_arm(s_minus, p))
                    .min(extent);
                let margin = Margin::sagitta(jet.kappa_rel.abs(), arm);
                match decide("tangent_second_order", margin, band) {
                    Ok(Sign::Positive) => {}
                    Ok(Sign::Zero | Sign::Negative) => {
                        jet_determinate = false;
                        break;
                    }
                    Err(cause) => {
                        errors.push(ValidationError::SliverDihedral {
                            edge: edge_key,
                            cause,
                        });
                        jet_escalated = true;
                        break;
                    }
                }
            }
            if jet_escalated {
                ContactMark::Unmarked
            } else if jet_determinate {
                ContactMark::Tangent
            } else {
                ContactMark::SmoothUnderdetermined
            }
        } else {
            ContactMark::Unmarked
        };
        if mark == ContactMark::Tangent
            && matches!(curve.description(), geom_brep::EdgeGeometry::MappedCurve(_))
            && geom_brep::tangent_certificate_lane(curve.carrier(), s_plus, s_minus)
        {
            // The lane condition IS the jet certificate's per-class
            // boundary, consulted from its one home
            // (`geom_brep::tangent_certificate_lane`, C12.1): the
            // demanded set equals the certifiable set by construction
            // — a Line-on-Cone tangency, say, is neither certifiable
            // nor demanded (fix pass, dev 7).
            errors.push(ValidationError::TangentNotIntrinsic { edge: edge_key });
        }
        marks.insert(edge_key, mark);
        // Check 5: planar-boundary containment against each distinct
        // adjacent planar face.
        let mut adjacent = vec![(f_plus, fs_plus)];
        if f_minus != f_plus {
            adjacent.push((f_minus, fs_minus));
        }
        for (face_key, surface_key) in adjacent {
            let Some(&Surface::Plane { origin, normal, .. }) = body.surfaces.get(surface_key)
            else {
                continue;
            };
            for &p in &samples {
                let residual = (p - origin).dot(normal);
                match decide("planar_boundary_residual", Margin::of(residual), band) {
                    Ok(Sign::Zero) => continue,
                    Ok(Sign::Positive | Sign::Negative) => {
                        errors.push(ValidationError::PlanarBoundaryResidual {
                            face: face_key,
                            edge: edge_key,
                        });
                    }
                    Err(cause) => {
                        errors.push(ValidationError::PlanarBoundaryEscalated {
                            face: face_key,
                            edge: edge_key,
                            cause,
                        });
                    }
                }
                break;
            }
        }
    }

    // ------------------------------------------------------------------
    // Tier 3, check 6: planar loop-role winding (M5 S1 fix pass) — the
    // region-bounding statement's PLANAR half, previously a documented
    // deferral of this battery: on every planar face, the outer loop
    // must wind positively around the outward normal and every cycle
    // ring negatively (empty rings bound no area and are exempt;
    // Zero/escalated windings are exempt — the check-7 posture). The
    // margin is the Newell functional `n · Σ (pᵢ−p₀)×(pᵢ₊₁−p₀)` —
    // twice the loop's signed enclosed area — through the reified
    // `bool_ring_run_winding` predicate (the same margin the boolean
    // join's ring lane and the merge role normalization decide on),
    // metered to a LENGTH by the loop's perimeter: 2A/P, the region's
    // mean width (audit F4; derivation at `boolean::join::ring_run_ccw`,
    // the same discipline as check 7's V/A below).
    // A role inversion passes every volume gate (they are
    // role-invariant) but silently corrupts tessellation/export;
    // this closes that class structurally. Scope: LINE-BOUNDED loops
    // only — the vertex-chord Newell functional IS the enclosed area
    // exactly for straight boundaries; a planar face bounded by arcs
    // (a revolve's annular sector) has chord windings that can
    // legitimately disagree with the region's (a 270° sector's chord
    // quad self-crosses), so curved-bounded faces stay in the
    // documented deferral (M5 pcurves) along with curved faces.
    //
    // **The S10 sense gate.** Since M5 S10 a face's outward normal is
    // `sense_sign · chart_normal`, so the winding is compared against
    // the OUTWARD normal — CATEGORY A: the chart normal alone is not
    // the face's orientation any more, and reading it raw would make
    // this check blind to exactly the corruption it exists to catch.
    // With the sign threaded, check 6 *is* the "the two encodings
    // agree" gate: `Face::sense` and the loops' stored winding are two
    // encodings of one fact (interior-left ⇒ the outer loop winds CCW
    // about the outward normal ⇒ CCW about the chart normal iff
    // `sense`), and a body where they disagree is INSIDE-OUT. Tier 3
    // refuses it here, per face, and no volume gate can: check 7 is
    // computed from the same loop windings, so an inside-out face is
    // invisible to it (flipping `sense` alone changes no winding and
    // therefore no volume). Check 6 — this planar arm and the curved
    // arm below (M6-6) — is where the sense bit is read as a claim to
    // be falsified rather than as a sign to be honored.
    // ------------------------------------------------------------------
    for (face_key, face) in body.faces.iter() {
        let Some(&Surface::Plane { normal, .. }) = body.surfaces.get(face.surface) else {
            continue;
        };
        // The face's outward normal (S10). Exact structure: a `bool`
        // selects `±1`, no predicate, no band, and `· 1` is bitwise
        // identity — every sense-true body decides exactly as before.
        let outward = normal * face.sense_sign::<T>();
        for (l, is_outer) in
            core::iter::once((face.outer, true)).chain(face.rings.iter().map(|&r| (r, false)))
        {
            let Some(loop_data) = body.get_loop(l) else {
                continue; // unreachable on tier-1 input
            };
            let crate::entity::LoopBoundary::Cycle { first } = loop_data.boundary else {
                continue; // empty ring: bounds no area
            };
            let Some(cycle) = body.loop_cycle(first) else {
                continue; // unreachable on tier-1 input
            };
            // Line-bounded only (banner): an arc's vertex chord is not
            // the boundary, and its winding is not the region's.
            let all_lines = cycle.iter().all(|&he| {
                body.get_half_edge(he)
                    .and_then(|hd| body.get_edge(hd.edge))
                    .and_then(|e| body.get_curve_geom(e.curve))
                    .and_then(crate::null::CurveGeom::certified)
                    .is_some_and(|c| matches!(c.carrier(), geom::Curve3::Line { .. }))
            });
            if !all_lines {
                continue;
            }
            let point_of = |he| {
                body.get_half_edge(he)
                    .and_then(|hd| body.get_vertex(hd.start))
                    .and_then(|vd| body.get_point(vd.point).copied())
            };
            let Some(p0) = point_of(cycle[0]) else {
                continue; // unreachable on tier-1 input
            };
            let mut newell = geom_core::Vec3::new(T::zero(), T::zero(), T::zero());
            // The F4 metering lever (audit; the derivation and the
            // three-site coherence statement live at
            // `boolean::join::ring_run_ccw`): this loop's perimeter,
            // accumulated with the area. Line-bounded only, so the
            // chords ARE the boundary.
            let mut perimeter = T::zero();
            let mut prev = p0;
            for &he in &cycle[1..] {
                let Some(p) = point_of(he) else {
                    continue;
                };
                newell = newell + (prev - p0).cross(p - p0);
                perimeter = perimeter + (p - prev).norm();
                prev = p;
            }
            perimeter = perimeter + (p0 - prev).norm();
            // Only a DEFINITE wrong sign refuses (doc on the variant:
            // the check-7 posture — Zero and escalated windings are
            // exempt, so degenerate pillows stay legal and
            // ε-tightening never flips valid → invalid).
            let wrong = if is_outer {
                Sign::Negative
            } else {
                Sign::Positive
            };
            if decide(
                "bool_ring_run_winding",
                Margin::over_lever(outward.dot(newell), perimeter),
                band,
            ) == Ok(wrong)
            {
                errors.push(ValidationError::LoopRoleInverted {
                    face: face_key,
                    r#loop: l,
                });
            }
        }
    }

    // ------------------------------------------------------------------
    // Tier 3, check 6, CURVED arm (M6-6): the sense-vs-boundary
    // material-side gate for the analytic curved kinds. The same
    // statement as the planar arm — the face's two orientation
    // encodings must agree — but the boundary's encoding is read the
    // way the flux lanes read it:
    // `geom_brep::props::boundary_material_sign` re-runs the rim-side
    // / meridian-orientation sub-derivations (`props_rim_side`,
    // `props_rim_level`, `props_circle_axis_class`,
    // `props_meridian_orient` — already length-metered named decides).
    // No new comparand exists: the final comparison is two exact ±1s,
    // genuinely combinatorial.
    //
    // `props_rim_level` — the iso-rectangle premise — is on that list
    // because the rim-side derivation rests on it: `lo + hi − 2v` is a
    // material side only on a domain whose rims all sit at an extreme,
    // and without it a plus-shaped face answered a definite ±1 that
    // depended on where `loop_edges` started the cycle. Such a face now
    // arrives here as `Err`, i.e. EXEMPT by the posture below, which is
    // what it always should have been: this arm firing on it pushed a
    // `CurvedSenseInverted` that suppressed check 7's honest
    // `VolumeUncomputable { NotIsoRectangle }` through the
    // `errors.is_empty()` gate.
    // Fires BEFORE check 7, which cannot see this defect — the flux is
    // traversal-derived, so a lone sense flip on a rim-bearing curved
    // face leaves the volume BIT-IDENTICAL (M6-6 substrate truth
    // table: washer walls, cone laterals, sphere zones, torus bands
    // all certified green before this arm), and a whole-body-inverted
    // washer/cone/donut even keeps its POSITIVE volume (their planar
    // caps are arc-bounded, exempt from the Newell arm above — this
    // arm is what refuses those bodies).
    //
    // Posture inherited (check 7): only a DEFINITE disagreement
    // refuses. A failed derivation (escalated classification,
    // degenerate/out-of-inventory boundary, conic-trimmed faces the
    // quadrature lane owns, an uncertifiable loop) is EXEMPT here —
    // corrupt structure is tier 1/2's job, an uncomputable volume is
    // check 7's. The conic-trim exemption is a RECORDED residual, not
    // a covered class: an ellipse-trimmed wall has a winding-derived
    // (bit-free) quadrature flux AND an exempt parse, so its flips —
    // single-face and whole-body — certify green today (executed on
    // the tilted-section cylinder, pinned as residual; closes when the
    // ellipse-rim material-side encoding lands). The rimless sphere
    // band returns `Unencoded` (its
    // boundary encodes no side; the bit is the ONLY encoding) and
    // stays exempt: the documented residual — a half-flipped rimless
    // ball meters V = 0, Zero-exempt by the ratified posture, while
    // the fully-inverted ball is check 7's NegativeVolume.
    // ------------------------------------------------------------------
    for (face_key, face) in body.faces.iter() {
        let Some(surface) = body.surfaces.get(face.surface) else {
            continue; // unreachable on tier-1 input
        };
        if matches!(surface, Surface::Plane { .. } | Surface::Nurbs(_)) {
            // Planar: the Newell arm above. NURBS: the quadrature lane
            // is winding-derived end to end, bit-free by design (S10
            // module docs) — recorded residual, out of this arm's
            // scope.
            continue;
        }
        let Ok((outer, _hes)) = crate::props::loop_edges(body, face.outer) else {
            continue; // derivation exempt (posture above)
        };
        match geom_brep::props::boundary_material_sign(surface, &outer, band) {
            Ok(geom_brep::props::MaterialSign::Encoded(side)) => {
                // Two exact ±1s: the derived side is definite by
                // construction, the sense bit is stored — a discrete
                // comparison, no scalar, no band.
                if (side == Sign::Positive) != face.sense {
                    errors.push(ValidationError::CurvedSenseInverted { face: face_key });
                }
            }
            Ok(geom_brep::props::MaterialSign::Unencoded) | Err(_) => {}
        }
    }

    // ------------------------------------------------------------------
    // Tier 3, check 7: the +V global orientation invariant (M2 PR 7).
    // Exact-B-rep signed volume, classified through the dimensionally
    // honest margin V / A_total — a *length*: the mean boundary
    // displacement the volume defect corresponds to (raw ε against a
    // volume in m³ would be dimensionally wrong; the total surface
    // area is the lever arm, since displacing the whole boundary
    // inward by ε changes V by ≈ ε·A). Definitely negative ⇒
    // orientation corruption. `Zero` and escalated margins are exempt
    // (the PR 4 posture: ε-tightening can escalate but never flips a
    // valid body to invalid — this check is an orientation probe, not
    // a thinness gate; V is monotone in the margin, so a genuinely
    // positive volume never classifies `Negative` under a tighter ε).
    // Gated on a clean report: on a geometrically corrupt body the
    // volume is meaningless cascade noise (same discipline as the
    // tier-2 gate above).
    //
    // S10: the sense handling is INHERITED, not repeated here.
    // `crate::props` owns it and applies `Face::sense_sign` at exactly
    // one site (the rimless sphere band, the sole flux sign no
    // boundary traversal encodes); every other term of the flux is
    // derived from the stored loop windings and is therefore
    // sense-invariant by derivation. Multiplying anything at this call
    // site would double-count. Note the consequence, which is why
    // check 6 exists (both arms): a lone `sense` flip changes no
    // winding, so the volume this check reads is BLIND to it — check 7
    // cannot detect an inside-out planar OR rim-bearing curved face
    // and does not claim to; its one sense-visible catch is the
    // rimless sphere band (the s_f the bit supplies), which is exactly
    // the face the check-6 curved arm must exempt as Unencoded.
    // ------------------------------------------------------------------
    if errors.is_empty() {
        match crate::props::mass_properties_with(body, band) {
            Ok(props) => {
                // The margin consumes the CERTIFIED bound (M5 PR 11):
                // for quadrature faces `volume` is an enclosure
                // midpoint with half-width `volume_pad`, so the honest
                // "definitely negative" statement is about the UPPER
                // end `volume + pad` — a thin positive volume inside a
                // wide bracket must never refuse. Closed-form bodies
                // have pad = 0.0 and the margin is bit-identical to
                // the pre-PR-11 one.
                let v_hi = props.volume + T::from_f64(props.volume_pad);
                if let Ok(Sign::Negative) = decide(
                    "positive_volume",
                    Margin::over_lever(v_hi, props.surface_area),
                    band,
                ) {
                    errors.push(ValidationError::NegativeVolume);
                }
            }
            Err(source) => errors.push(ValidationError::VolumeUncomputable { source }),
        }
    }

    // ------------------------------------------------------------------
    // Tier 3, check 8 (M5 PR 6, C4): the stored pcurve caches — the
    // certificate must be PRESENT on every half-edge of a minting
    // chart's face, the certification must REPLAY at rest (re-derived,
    // never trusted), and domain validity (trim containment + the
    // loop's one-branch continuity) must hold. Bodies carrying no
    // stored pcurves — every all-planar body — contribute nothing.
    // Ungated on the volume check: a pcurve defect is local evidence
    // about a specific half-edge, not cascade noise.
    // ------------------------------------------------------------------
    for finding in crate::pcurves::validate_pcurves(body, band) {
        errors.push(ValidationError::Pcurve { finding });
    }

    errors
}

/// **Tier 3′** (M3 PR 6a, F1/F2): the pseudomanifold at-rest validator
/// for declared-contact bodies — tier 3's full local battery **plus**
/// the global coincidence census tier 3 defers, certified against the
/// body's declared-contact records.
///
/// Structure (D1):
/// 1. Coarse-gate on tiers 1–2 (as [`validate_geometric`]).
/// 2. All of tier 3's local checks, shared verbatim
///    ([`tier3_local_checks`]).
/// 3. The **global coincidence census** (only when the local checks are
///    clean — census geometry on a locally corrupt body is cascade
///    noise, the check-7 discipline): every cross-entity position
///    coincidence among distinct entities — vertex-vertex,
///    vertex-on-face (interior), vertex-on-edge (interior), proper
///    edge-face / edge-edge crossings, and the segment-granularity
///    overlaps (edge-edge collinear, edge-on-face) reconstructed per
///    the D3 rule (see [`crate::census`] module docs). Quadratic
///    sweeps, documented — the boolean edge×face convention.
/// 4. The certification diff, both directions: every census finding
///    must be backed by a declaration
///    ([`ValidationError::UndeclaredContact`] otherwise — the
///    validator never blesses discovered contacts; F1: no
///    scan-to-bless) and every declaration must be geometrically
///    confirmed ([`ValidationError::StaleContactDeclaration`]
///    otherwise).
///
/// With `contacts` empty the census must find nothing, and 3′ ≡ tier 3
/// plus the census actually run (pinned by the acceptance suite).
///
/// Since M9-2 the census reaches beyond the exact planar sweeps: the
/// conformal face-pair arm (shared-carrier opposed-sense curved pairs
/// through the chart-region predicate), face-granularity backing from
/// curve/patch records, and the record certifiers (jet schedule for
/// `CurveContact`, chart-region overlap for `PatchContact`). ASM R2-b
/// consumes THIS gate (ASM-R2-SPEC-DRAFT:39-58): mate declarations
/// land in the product body's [`crate::boolean::ContactRecords`] —
/// same currency, no adapter — and validate here.
///
/// # Errors
///
/// A non-empty vector of every failure found: tiers 1–2 verbatim if
/// any, else tier-3 local failures, else census/certification
/// failures in deterministic sweep order.
pub fn validate_pseudomanifold<T: crate::props::PropsQuadLane>(
    body: &Body<T>,
    contacts: &crate::boolean::ContactRecords,
) -> Result<(), Vec<ValidationError>> {
    validate_closed(body)?;
    let band = match Band::linear() {
        Ok(band) => band,
        Err(error) => return Err(vec![ValidationError::Band { error }]),
    };
    let mut errors = tier3_local_checks(body, band);
    if errors.is_empty() {
        errors.extend(crate::census::census_and_certify(body, contacts, band));
    }
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// The endpoint points of an edge in `he_plus` forward order, or `None`
/// on (tier-1-impossible) unresolvable links.
fn edge_endpoints<T: Real>(
    body: &Body<T>,
    he_plus: HalfEdgeKey,
) -> Option<(geom_core::Point3<T>, geom_core::Point3<T>)> {
    let plus = body.half_edges.get(he_plus)?;
    let end_vertex = body.half_edge_end(he_plus)?;
    let p_start = *body.points.get(body.vertices.get(plus.start)?.point)?;
    let p_end = *body.points.get(body.vertices.get(end_vertex)?.point)?;
    Some((p_start, p_end))
}

/// The surface keys of an edge's two adjacent faces (`he_plus`'s side,
/// `he_minus`'s side), or `None` on unresolvable links.
fn edge_face_surfaces<T: Real>(
    body: &Body<T>,
    he_plus: HalfEdgeKey,
    he_minus: HalfEdgeKey,
) -> Option<(crate::geometry::SurfaceKey, crate::geometry::SurfaceKey)> {
    let ((_, s_plus), (_, s_minus)) = edge_adjacent_faces(body, he_plus, he_minus)?;
    Some((s_plus, s_minus))
}

/// An edge's two adjacent faces with their surface keys (`he_plus`'s
/// side, `he_minus`'s side), or `None` on unresolvable links.
fn edge_adjacent_faces<T: Real>(
    body: &Body<T>,
    he_plus: HalfEdgeKey,
    he_minus: HalfEdgeKey,
) -> Option<(
    (FaceKey, crate::geometry::SurfaceKey),
    (FaceKey, crate::geometry::SurfaceKey),
)> {
    let face_of = |he: HalfEdgeKey| {
        let he_data = body.half_edges.get(he)?;
        let loop_data = body.loops.get(he_data.parent_loop)?;
        let face_data = body.faces.get(loop_data.face)?;
        Some((loop_data.face, face_data.surface))
    };
    Some((face_of(he_plus)?, face_of(he_minus)?))
}

/// The tier-1 pass pipeline (see [`validate`] and the module docs).
fn tier1<T: Real>(body: &Body<T>) -> Tier1Report {
    let mut errors = Vec::new();

    // Reference/ownership counters, filled by pass 1 and consumed by the
    // later passes. Only keys that resolve are counted; dangling keys are
    // reported in pass 1 and count toward nothing.
    let mut shell_owners: SecondaryMap<_, (usize, _)> = SecondaryMap::new();
    let mut face_owners: SecondaryMap<_, (usize, _)> = SecondaryMap::new();
    let mut loop_owners: SecondaryMap<_, (usize, _)> = SecondaryMap::new();
    let mut vertex_incidence: SecondaryMap<VertexKey, usize> = SecondaryMap::new();
    let mut vertex_empty_loops: SecondaryMap<VertexKey, usize> = SecondaryMap::new();
    let mut point_refs: SecondaryMap<_, usize> = SecondaryMap::new();
    let mut curve_refs: SecondaryMap<_, usize> = SecondaryMap::new();
    let mut surface_refs: SecondaryMap<_, usize> = SecondaryMap::new();

    // ------------------------------------------------------------------
    // Pass 1: reference resolution, solids → shells → faces → loops →
    // half-edges → edges → vertices; within an entity, field-declaration
    // order.
    // ------------------------------------------------------------------
    for (solid_key, solid) in body.solids.iter() {
        for &shell in &solid.shells {
            if body.shells.contains_key(shell) {
                count_owner(&mut shell_owners, shell, solid_key);
            } else {
                errors.push(ValidationError::DanglingTopology {
                    from: EntityId::Solid(solid_key),
                    to: EntityId::Shell(shell),
                });
            }
        }
    }
    for (shell_key, shell) in body.shells.iter() {
        for &face in &shell.faces {
            if body.faces.contains_key(face) {
                count_owner(&mut face_owners, face, shell_key);
            } else {
                errors.push(ValidationError::DanglingTopology {
                    from: EntityId::Shell(shell_key),
                    to: EntityId::Face(face),
                });
            }
        }
        if !body.solids.contains_key(shell.solid) {
            errors.push(ValidationError::DanglingTopology {
                from: EntityId::Shell(shell_key),
                to: EntityId::Solid(shell.solid),
            });
        }
    }
    for (face_key, face) in body.faces.iter() {
        if body.surfaces.contains_key(face.surface) {
            count_ref(&mut surface_refs, face.surface);
        } else {
            errors.push(ValidationError::DanglingGeometry {
                from: EntityId::Face(face_key),
                to: GeomRef::Surface(face.surface),
            });
        }
        for &loop_ in core::iter::once(&face.outer).chain(&face.rings) {
            if body.loops.contains_key(loop_) {
                count_owner(&mut loop_owners, loop_, face_key);
            } else {
                errors.push(ValidationError::DanglingTopology {
                    from: EntityId::Face(face_key),
                    to: EntityId::Loop(loop_),
                });
            }
        }
        if !body.shells.contains_key(face.shell) {
            errors.push(ValidationError::DanglingTopology {
                from: EntityId::Face(face_key),
                to: EntityId::Shell(face.shell),
            });
        }
    }
    for (loop_key, loop_) in body.loops.iter() {
        match loop_.boundary {
            LoopBoundary::Empty { vertex } => {
                if body.vertices.contains_key(vertex) {
                    count_ref(&mut vertex_empty_loops, vertex);
                } else {
                    errors.push(ValidationError::DanglingTopology {
                        from: EntityId::Loop(loop_key),
                        to: EntityId::Vertex(vertex),
                    });
                }
            }
            LoopBoundary::Cycle { first } => {
                if !body.half_edges.contains_key(first) {
                    errors.push(ValidationError::DanglingTopology {
                        from: EntityId::Loop(loop_key),
                        to: EntityId::HalfEdge(first),
                    });
                }
            }
        }
        if !body.faces.contains_key(loop_.face) {
            errors.push(ValidationError::DanglingTopology {
                from: EntityId::Loop(loop_key),
                to: EntityId::Face(loop_.face),
            });
        }
    }
    for (he_key, he) in body.half_edges.iter() {
        if !body.edges.contains_key(he.edge) {
            errors.push(ValidationError::DanglingTopology {
                from: EntityId::HalfEdge(he_key),
                to: EntityId::Edge(he.edge),
            });
        }
        if body.vertices.contains_key(he.start) {
            count_ref(&mut vertex_incidence, he.start);
        } else {
            errors.push(ValidationError::DanglingTopology {
                from: EntityId::HalfEdge(he_key),
                to: EntityId::Vertex(he.start),
            });
        }
        if !body.loops.contains_key(he.parent_loop) {
            errors.push(ValidationError::DanglingTopology {
                from: EntityId::HalfEdge(he_key),
                to: EntityId::Loop(he.parent_loop),
            });
        }
        for link in [he.next, he.prev] {
            if !body.half_edges.contains_key(link) {
                errors.push(ValidationError::DanglingTopology {
                    from: EntityId::HalfEdge(he_key),
                    to: EntityId::HalfEdge(link),
                });
            }
        }
    }
    for (edge_key, edge) in body.edges.iter() {
        for slot in [edge.he_plus, edge.he_minus] {
            if !body.half_edges.contains_key(slot) {
                errors.push(ValidationError::DanglingTopology {
                    from: EntityId::Edge(edge_key),
                    to: EntityId::HalfEdge(slot),
                });
            }
        }
        if body.curves.contains_key(edge.curve) {
            count_ref(&mut curve_refs, edge.curve);
        } else {
            errors.push(ValidationError::DanglingGeometry {
                from: EntityId::Edge(edge_key),
                to: GeomRef::Curve(edge.curve),
            });
        }
    }
    // Geometry-to-geometry references (M2 PR 3): an edge curve's
    // description names surfaces by key (`Intersection`/`Seam`); those
    // references must resolve, and they anchor their surfaces for the
    // pass-8 orphan count exactly like a face's reference does (the
    // removal guard `remove_surface_if_orphaned` honors the same rule).
    for (curve_key, curve) in body.curves.iter() {
        for surface in Body::description_surfaces(curve) {
            if body.surfaces.contains_key(surface) {
                count_ref(&mut surface_refs, surface);
            } else {
                errors.push(ValidationError::DanglingDescription {
                    from: GeomRef::Curve(curve_key),
                    to: GeomRef::Surface(surface),
                });
            }
        }
    }
    for (vertex_key, vertex) in body.vertices.iter() {
        if body.points.contains_key(vertex.point) {
            count_ref(&mut point_refs, vertex.point);
        } else {
            errors.push(ValidationError::DanglingGeometry {
                from: EntityId::Vertex(vertex_key),
                to: GeomRef::Point(vertex.point),
            });
        }
        if let Some(emanating) = vertex.emanating
            && !body.half_edges.contains_key(emanating)
        {
            errors.push(ValidationError::DanglingTopology {
                from: EntityId::Vertex(vertex_key),
                to: EntityId::HalfEdge(emanating),
            });
        }
    }

    // ------------------------------------------------------------------
    // Pass 2: half-edge chain coherence — next/prev inverses, cycle
    // walks, reachability.
    // ------------------------------------------------------------------
    for (he_key, he) in body.half_edges.iter() {
        if let Some(next) = body.half_edges.get(he.next)
            && next.prev != he_key
        {
            errors.push(ValidationError::NextPrevMismatch { half_edge: he_key });
        }
    }
    // Which loops' cycles closed, and which half-edges were reached by
    // their own parent's cycle (gates for the reachability check).
    let mut loop_closed: SecondaryMap<LoopKey, ()> = SecondaryMap::new();
    let mut reached_by_parent: SecondaryMap<HalfEdgeKey, ()> = SecondaryMap::new();
    for (loop_key, loop_) in body.loops.iter() {
        let LoopBoundary::Cycle { first } = loop_.boundary else {
            continue;
        };
        match body.loop_walk(first) {
            Walk::Closed(members) => {
                loop_closed.insert(loop_key, ());
                for member in members {
                    // Walk members always resolve (the walk checked).
                    let Some(he) = body.half_edges.get(member) else {
                        continue;
                    };
                    if he.parent_loop == loop_key {
                        reached_by_parent.insert(member, ());
                    } else {
                        errors.push(ValidationError::ParentLoopMismatch {
                            half_edge: member,
                            owner: loop_key,
                        });
                    }
                }
            }
            // Broken: a stale link mid-cycle — pass 1 reported it.
            Walk::Broken => {}
            Walk::Overrun => {
                errors.push(ValidationError::LoopCycleOverrun { loop_: loop_key });
            }
        }
    }
    for (he_key, he) in body.half_edges.iter() {
        let Some(parent) = body.loops.get(he.parent_loop) else {
            continue; // dangling parent: pass 1 reported it
        };
        let unreachable = match parent.boundary {
            // An empty loop reaches no half-edge at all.
            LoopBoundary::Empty { .. } => true,
            // Only judge against cycles that actually closed; a broken or
            // overrun parent already carries its own report.
            LoopBoundary::Cycle { .. } => {
                loop_closed.contains_key(he.parent_loop) && !reached_by_parent.contains_key(he_key)
            }
        };
        if unreachable {
            errors.push(ValidationError::UnreachableHalfEdge { half_edge: he_key });
        }
    }

    // ------------------------------------------------------------------
    // Pass 3: edge ↔ half-edge bijection.
    // ------------------------------------------------------------------
    let mut he_claims: SecondaryMap<HalfEdgeKey, usize> = SecondaryMap::new();
    for (edge_key, edge) in body.edges.iter() {
        if edge.he_plus == edge.he_minus {
            errors.push(ValidationError::EdgeHalvesIdentical { edge: edge_key });
        }
        for slot in [edge.he_plus, edge.he_minus] {
            let Some(he) = body.half_edges.get(slot) else {
                continue; // dangling slot: pass 1 reported it
            };
            count_ref(&mut he_claims, slot);
            if he.edge != edge_key {
                errors.push(ValidationError::EdgeSlotBackpointerMismatch {
                    edge: edge_key,
                    half_edge: slot,
                });
            }
        }
    }
    for (he_key, _) in body.half_edges.iter() {
        match he_claims.get(he_key).copied().unwrap_or(0) {
            0 => errors.push(ValidationError::HalfEdgeUnclaimed { half_edge: he_key }),
            1 => {}
            claims => errors.push(ValidationError::HalfEdgeMultiplyClaimed {
                half_edge: he_key,
                claims,
            }),
        }
    }

    // ------------------------------------------------------------------
    // Pass 4: antiparallelism. Gated on distinct, resolving halves whose
    // ends are derivable (everything else was reported above).
    // ------------------------------------------------------------------
    for (edge_key, edge) in body.edges.iter() {
        if edge.he_plus == edge.he_minus {
            continue;
        }
        let (Some(plus), Some(minus)) = (
            body.half_edges.get(edge.he_plus),
            body.half_edges.get(edge.he_minus),
        ) else {
            continue;
        };
        let (Some(plus_next), Some(minus_next)) = (
            body.half_edges.get(plus.next),
            body.half_edges.get(minus.next),
        ) else {
            continue;
        };
        if plus_next.start != minus.start || minus_next.start != plus.start {
            errors.push(ValidationError::EdgeNotAntiparallel { edge: edge_key });
        }
    }

    // ------------------------------------------------------------------
    // Pass 5: vertex anchoring (see the module docs, check 5).
    // ------------------------------------------------------------------
    for (vertex_key, vertex) in body.vertices.iter() {
        let incident = vertex_incidence.get(vertex_key).copied().unwrap_or(0);
        let empty_loops = vertex_empty_loops.get(vertex_key).copied().unwrap_or(0);
        match vertex.emanating {
            Some(emanating) => {
                if let Some(he) = body.half_edges.get(emanating)
                    && he.start != vertex_key
                {
                    errors.push(ValidationError::EmanatingStartMismatch {
                        vertex: vertex_key,
                        emanating,
                    });
                }
                if empty_loops > 0 {
                    errors.push(ValidationError::EmptyLoopVertexWithEmanating {
                        vertex: vertex_key,
                        empty_loops,
                    });
                }
            }
            None => {
                if incident > 0 {
                    errors.push(ValidationError::LoneVertexWithIncidence {
                        vertex: vertex_key,
                        incident,
                    });
                }
                if incident == 0 && empty_loops == 0 {
                    errors.push(ValidationError::OrphanEntity {
                        entity: EntityId::Vertex(vertex_key),
                    });
                }
                if empty_loops >= 2 {
                    errors.push(ValidationError::MultiplyOwned {
                        child: EntityId::Vertex(vertex_key),
                        owners: empty_loops,
                    });
                }
            }
        }
    }

    // ------------------------------------------------------------------
    // Pass 6: vertex-orbit closure (manifoldness). Gated on a resolving
    // emanating half-edge that starts at the vertex.
    // ------------------------------------------------------------------
    for (vertex_key, vertex) in body.vertices.iter() {
        let Some(emanating) = vertex.emanating else {
            continue;
        };
        let Some(he) = body.half_edges.get(emanating) else {
            continue; // dangling: pass 1 reported it
        };
        if he.start != vertex_key {
            continue; // pass 5 reported the mismatch
        }
        match body.orbit_walk(emanating) {
            // Broken: a stale link or broken mate — passes 1/3 reported
            // the cause.
            Walk::Broken => {}
            Walk::Overrun => {
                errors.push(ValidationError::VertexOrbitOverrun { vertex: vertex_key });
            }
            Walk::Closed(members) => {
                let orbit = members.len();
                let mut foreign = false;
                for member in members {
                    let Some(member_he) = body.half_edges.get(member) else {
                        continue; // walk members always resolve
                    };
                    if member_he.start != vertex_key {
                        foreign = true;
                        errors.push(ValidationError::OrbitForeignMember {
                            vertex: vertex_key,
                            half_edge: member,
                        });
                    }
                }
                let incident = vertex_incidence.get(vertex_key).copied().unwrap_or(0);
                // With foreign members the two counts measure different
                // sets; the foreign reports carry the failure.
                if !foreign && orbit != incident {
                    errors.push(ValidationError::SplitVertexOrbit {
                        vertex: vertex_key,
                        orbit,
                        incident,
                    });
                }
            }
        }
    }

    // ------------------------------------------------------------------
    // Pass 7: ownership and back-pointers. Back-pointer comparison is
    // gated on an unambiguous owner (count exactly 1) and a resolving
    // stored key (a dangling one was reported in pass 1).
    // ------------------------------------------------------------------
    for (face_key, face) in body.faces.iter() {
        if face.rings.contains(&face.outer) {
            errors.push(ValidationError::OuterListedAsRing { face: face_key });
        }
    }
    for (shell_key, shell) in body.shells.iter() {
        match shell_owners.get(shell_key).copied() {
            None => errors.push(ValidationError::OrphanEntity {
                entity: EntityId::Shell(shell_key),
            }),
            Some((1, owner)) => {
                if body.solids.contains_key(shell.solid) && shell.solid != owner {
                    errors.push(ValidationError::BackPointerMismatch {
                        child: EntityId::Shell(shell_key),
                        stored: EntityId::Solid(shell.solid),
                        owner: EntityId::Solid(owner),
                    });
                }
            }
            Some((owners, _)) => errors.push(ValidationError::MultiplyOwned {
                child: EntityId::Shell(shell_key),
                owners,
            }),
        }
    }
    for (face_key, face) in body.faces.iter() {
        match face_owners.get(face_key).copied() {
            None => errors.push(ValidationError::OrphanEntity {
                entity: EntityId::Face(face_key),
            }),
            Some((1, owner)) => {
                if body.shells.contains_key(face.shell) && face.shell != owner {
                    errors.push(ValidationError::BackPointerMismatch {
                        child: EntityId::Face(face_key),
                        stored: EntityId::Shell(face.shell),
                        owner: EntityId::Shell(owner),
                    });
                }
            }
            Some((owners, _)) => errors.push(ValidationError::MultiplyOwned {
                child: EntityId::Face(face_key),
                owners,
            }),
        }
    }
    for (loop_key, loop_) in body.loops.iter() {
        match loop_owners.get(loop_key).copied() {
            None => errors.push(ValidationError::OrphanEntity {
                entity: EntityId::Loop(loop_key),
            }),
            Some((1, owner)) => {
                if body.faces.contains_key(loop_.face) && loop_.face != owner {
                    errors.push(ValidationError::BackPointerMismatch {
                        child: EntityId::Loop(loop_key),
                        stored: EntityId::Face(loop_.face),
                        owner: EntityId::Face(owner),
                    });
                }
            }
            Some((owners, _)) => errors.push(ValidationError::MultiplyOwned {
                child: EntityId::Loop(loop_key),
                owners,
            }),
        }
    }

    // Pass 11's coarse gate, captured HERE — after the structural passes
    // 1–7, before the passes that cannot corrupt the counts (module
    // docs, cascade discipline).
    let structurally_sound = errors.is_empty();

    // ------------------------------------------------------------------
    // Pass 8: orphan geometry, points → curves → surfaces.
    // ------------------------------------------------------------------
    for (key, _) in body.points.iter() {
        if point_refs.get(key).copied().unwrap_or(0) == 0 {
            errors.push(ValidationError::OrphanGeometry {
                geometry: GeomRef::Point(key),
            });
        }
    }
    for (key, _) in body.curves.iter() {
        if curve_refs.get(key).copied().unwrap_or(0) == 0 {
            errors.push(ValidationError::OrphanGeometry {
                geometry: GeomRef::Curve(key),
            });
        }
    }
    for (key, _) in body.surfaces.iter() {
        if surface_refs.get(key).copied().unwrap_or(0) == 0 {
            errors.push(ValidationError::OrphanGeometry {
                geometry: GeomRef::Surface(key),
            });
        }
    }

    // ------------------------------------------------------------------
    // Pass 9: arity floors, solids then shells (module docs, check 9).
    // ------------------------------------------------------------------
    for (solid_key, solid) in body.solids.iter() {
        if solid.shells.is_empty() {
            errors.push(ValidationError::SolidWithoutShells { solid: solid_key });
        }
    }
    for (shell_key, shell) in body.shells.iter() {
        if shell.faces.is_empty() {
            errors.push(ValidationError::ShellWithoutFaces { shell: shell_key });
        }
    }

    // ------------------------------------------------------------------
    // Pass 10: edge-adjacency shell coherence, sweeping edges. The
    // half's shell is derived through the OWNERSHIP partition (loop's
    // owning face, face's owning shell), not the stored back-pointers:
    // a wrong back-pointer is pass 7's report and must not echo here.
    // Gated per-edge on the halves resolving and both ownership steps
    // being unambiguous (exactly one owner — anything else was reported
    // in pass 1 or 7).
    // ------------------------------------------------------------------
    let shell_of = |he_key: HalfEdgeKey| -> Option<ShellKey> {
        let he = body.half_edges.get(he_key)?;
        let (1, face) = loop_owners.get(he.parent_loop).copied()? else {
            return None;
        };
        let (1, shell) = face_owners.get(face).copied()? else {
            return None;
        };
        Some(shell)
    };
    for (edge_key, edge) in body.edges.iter() {
        let (Some(shell_plus), Some(shell_minus)) =
            (shell_of(edge.he_plus), shell_of(edge.he_minus))
        else {
            continue;
        };
        if shell_plus != shell_minus {
            errors.push(ValidationError::EdgeAcrossShells {
                edge: edge_key,
                shell_plus,
                shell_minus,
            });
        }
    }

    // ------------------------------------------------------------------
    // Pass 11: component-aware per-shell Euler–Poincaré (module docs,
    // check 11), gated on passes 1–7 having reported nothing. Shells in
    // arena order; a shell's component seeds in face-arena order.
    // ------------------------------------------------------------------
    let mut shell_components = None;
    if structurally_sound {
        // Per-shell face lists in face-arena order. Membership comes
        // from the `face.shell` back-pointer, which the gate guarantees
        // matches the ownership lists.
        let mut shell_faces: SecondaryMap<ShellKey, Vec<FaceKey>> = SecondaryMap::new();
        for (face_key, face) in body.faces.iter() {
            if let Some(list) = shell_faces.get_mut(face.shell) {
                list.push(face_key);
            } else {
                shell_faces.insert(face.shell, vec![face_key]);
            }
        }
        let mut visited: SecondaryMap<FaceKey, ()> = SecondaryMap::new();
        let mut per_shell = Vec::new();
        for (shell_key, _) in body.shells.iter() {
            let mut components = 0_usize;
            let seeds = shell_faces.get(shell_key).map_or(&[][..], Vec::as_slice);
            for &seed in seeds {
                if visited.contains_key(seed) {
                    continue;
                }
                components += 1;
                // The walk is total but cannot fail under the gate; a
                // `None` would mean a resolution failure passes 1–7
                // somehow missed, and silence is then the safe echo rule.
                if let Some(counts) = shell_component(body, shell_key, seed, &mut visited)
                    && !counts.satisfies_euler_poincare()
                {
                    errors.push(ValidationError::ComponentEulerViolation {
                        shell: shell_key,
                        seed,
                        vertices: counts.vertices,
                        edges: counts.edges,
                        faces: counts.faces,
                        rings: counts.rings,
                    });
                }
            }
            per_shell.push((shell_key, components));
        }
        shell_components = Some(per_shell);
    }

    // ------------------------------------------------------------------
    // Pass 12: bidirectional D5 provenance (module docs, check 12).
    // Missing records first (entities in arena order, kinds in the
    // pass-1 order), then leaked records (SecondaryMap slot order, same
    // kind order).
    // ------------------------------------------------------------------
    for (k, _) in body.solids.iter() {
        if !body.solid_provenance.contains_key(k) {
            errors.push(ValidationError::MissingProvenance {
                entity: EntityId::Solid(k),
            });
        }
    }
    for (k, _) in body.shells.iter() {
        if !body.shell_provenance.contains_key(k) {
            errors.push(ValidationError::MissingProvenance {
                entity: EntityId::Shell(k),
            });
        }
    }
    for (k, _) in body.faces.iter() {
        if !body.face_provenance.contains_key(k) {
            errors.push(ValidationError::MissingProvenance {
                entity: EntityId::Face(k),
            });
        }
    }
    for (k, _) in body.loops.iter() {
        if !body.loop_provenance.contains_key(k) {
            errors.push(ValidationError::MissingProvenance {
                entity: EntityId::Loop(k),
            });
        }
    }
    for (k, _) in body.half_edges.iter() {
        if !body.half_edge_provenance.contains_key(k) {
            errors.push(ValidationError::MissingProvenance {
                entity: EntityId::HalfEdge(k),
            });
        }
    }
    for (k, _) in body.edges.iter() {
        if !body.edge_provenance.contains_key(k) {
            errors.push(ValidationError::MissingProvenance {
                entity: EntityId::Edge(k),
            });
        }
    }
    for (k, _) in body.vertices.iter() {
        if !body.vertex_provenance.contains_key(k) {
            errors.push(ValidationError::MissingProvenance {
                entity: EntityId::Vertex(k),
            });
        }
    }
    for (k, _) in body.solid_provenance.iter() {
        if !body.solids.contains_key(k) {
            errors.push(ValidationError::LeakedProvenance {
                entity: EntityId::Solid(k),
            });
        }
    }
    for (k, _) in body.shell_provenance.iter() {
        if !body.shells.contains_key(k) {
            errors.push(ValidationError::LeakedProvenance {
                entity: EntityId::Shell(k),
            });
        }
    }
    for (k, _) in body.face_provenance.iter() {
        if !body.faces.contains_key(k) {
            errors.push(ValidationError::LeakedProvenance {
                entity: EntityId::Face(k),
            });
        }
    }
    for (k, _) in body.loop_provenance.iter() {
        if !body.loops.contains_key(k) {
            errors.push(ValidationError::LeakedProvenance {
                entity: EntityId::Loop(k),
            });
        }
    }
    for (k, _) in body.half_edge_provenance.iter() {
        if !body.half_edges.contains_key(k) {
            errors.push(ValidationError::LeakedProvenance {
                entity: EntityId::HalfEdge(k),
            });
        }
    }
    for (k, _) in body.edge_provenance.iter() {
        if !body.edges.contains_key(k) {
            errors.push(ValidationError::LeakedProvenance {
                entity: EntityId::Edge(k),
            });
        }
    }
    for (k, _) in body.vertex_provenance.iter() {
        if !body.vertices.contains_key(k) {
            errors.push(ValidationError::LeakedProvenance {
                entity: EntityId::Vertex(k),
            });
        }
    }

    // Pass 13 (M3 PR 1): null-entity referential coherence — the
    // deliberately *minimal* tier-1 hooks (see `crate::null`): a
    // null-scaffold curve entry is per-edge data, so it is referenced
    // by at most one edge (curve-arena order; zero references is pass
    // 8's orphan report); a null-face record never outlives its face
    // and never names a dead loop (record slot order — the
    // provenance-leak rule applied to F9 records and their named
    // loops). Attribute *semantics* (which vertices/loops the
    // records name) are intentionally not tier-1 checks: Euler
    // surgery legitimately rewires neighborhoods mid-sequence, and
    // tier 2 refuses null entities at rest regardless.
    for (curve_key, entry) in body.curves.iter() {
        if matches!(entry, CurveGeom::NullScaffold(_)) {
            let referers = body
                .edges
                .values()
                .filter(|edge| edge.curve == curve_key)
                .count();
            if referers >= 2 {
                errors.push(ValidationError::NullScaffoldShared {
                    curve: curve_key,
                    edges: referers,
                });
            }
        }
    }
    for (face_key, record) in body.null_faces.iter() {
        if !body.faces.contains_key(face_key) {
            errors.push(ValidationError::LeakedNullFaceRecord { face: face_key });
        }
        // Review flag (c): the record's named loops must also resolve —
        // referential-only (a record naming killed loops is the same
        // leak as a record outliving its face); which loops they are
        // stays unexamined at tier 1.
        for named_loop in record.loops() {
            if !body.loops.contains_key(named_loop) {
                errors.push(ValidationError::StaleNullFaceLoop {
                    face: face_key,
                    named_loop,
                });
            }
        }
    }

    Tier1Report {
        errors,
        shell_components,
    }
}

/// Collects one connected component of `shell`'s incidence complex by a
/// bounded traversal from `seed`, marking every face it reaches in
/// `visited` and returning the component's counts.
///
/// Glue rules (the ratified pass-11 partition — module docs): a face
/// glues all its loops, outer and rings; a cycle loop contributes its
/// half-edges' start vertices and edges, and glues across each edge via
/// **mate** to the mate's face (followed only when that face is in the
/// same shell — the per-shell filter is the point of the pass); an
/// empty loop glues its lone vertex; a dartless empty-outer face is
/// therefore its own component with its vertex. Traversal order does
/// not affect the counts (they are set sizes); component *identity* is
/// fixed by seed order (D9).
///
/// Total, never panicking (D9): every lookup is checked and the cycle
/// walks are bounded. `None` is returned on any resolution failure —
/// unreachable when the pass-11 gate held, and silent by the cascade
/// rule (the cause was reported by passes 1–7).
fn shell_component<T: Real>(
    body: &Body<T>,
    shell: ShellKey,
    seed: FaceKey,
    visited: &mut SecondaryMap<FaceKey, ()>,
) -> Option<ComponentCounts> {
    let mut component_vertices: SecondaryMap<VertexKey, ()> = SecondaryMap::new();
    let mut component_edges: SecondaryMap<EdgeKey, ()> = SecondaryMap::new();
    let mut faces = 0_usize;
    let mut rings = 0_usize;
    let mut pending = vec![seed];
    visited.insert(seed, ());
    while let Some(face_key) = pending.pop() {
        let face = body.faces.get(face_key)?;
        faces += 1;
        rings += face.rings.len();
        for &loop_key in core::iter::once(&face.outer).chain(&face.rings) {
            let loop_ = body.loops.get(loop_key)?;
            match loop_.boundary {
                LoopBoundary::Empty { vertex } => {
                    if !body.vertices.contains_key(vertex) {
                        return None;
                    }
                    component_vertices.insert(vertex, ());
                }
                LoopBoundary::Cycle { first } => {
                    let Walk::Closed(members) = body.loop_walk(first) else {
                        return None;
                    };
                    for member in members {
                        let he = body.half_edges.get(member)?;
                        if !body.vertices.contains_key(he.start) {
                            return None;
                        }
                        component_vertices.insert(he.start, ());
                        if !body.edges.contains_key(he.edge) {
                            return None;
                        }
                        component_edges.insert(he.edge, ());
                        // Glue across the edge via mate.
                        let mate = body.mate(member)?;
                        let mate_he = body.half_edges.get(mate)?;
                        let mate_loop = body.loops.get(mate_he.parent_loop)?;
                        let mate_face = body.faces.get(mate_loop.face)?;
                        if mate_face.shell == shell && !visited.contains_key(mate_loop.face) {
                            visited.insert(mate_loop.face, ());
                            pending.push(mate_loop.face);
                        }
                    }
                }
            }
        }
    }
    Some(ComponentCounts {
        vertices: component_vertices.len(),
        edges: component_edges.len(),
        faces,
        rings,
    })
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use geom_core::Point3;
    use proptest::prelude::*;

    use super::*;
    use crate::body::Body;
    use crate::entity::{Face, Loop, Shell, Solid, Vertex};
    use crate::euler::{MefSite, MevSite};
    use crate::fixtures::{
        mvfs_state, ngon_pillow, ops_cube, ops_genus2, ops_holed_box, pillow, prism, prov,
    };
    use crate::seqgen;

    fn anchor() -> Point3<f64> {
        Point3::origin()
    }

    /// Adds a face bounded by one empty loop holding `vertex` to `shell`
    /// (the mvfs shape grafted onto an existing body), returning the new
    /// loop key. Used to manufacture empty-loop/vertex interactions.
    fn add_empty_loop_face(
        body: &mut Body<f64>,
        shell: crate::entity::ShellKey,
        vertex: VertexKey,
    ) -> LoopKey {
        let surface = body.add_surface(crate::fixtures::test_surface(anchor()));
        let lp = body.add_loop(
            Loop {
                boundary: LoopBoundary::Empty { vertex },
                face: FaceKey::default(),
            },
            prov(),
        );
        let f = body.add_face(
            Face {
                sense: true,
                surface,
                outer: lp,
                rings: vec![],
                shell,
            },
            prov(),
        );
        body.get_loop_mut(lp).unwrap().face = f;
        body.get_shell_mut(shell).unwrap().faces.push(f);
        lp
    }

    // ------------------------------------------------------------------
    // Well-formed fixtures validate cleanly.
    // ------------------------------------------------------------------

    #[test]
    fn empty_body_validates_vacuously() {
        assert_eq!(validate(&Body::<f64>::new()), Ok(()));
    }

    #[test]
    fn digon_pillow_validates_cleanly() {
        // The minimal closed body: 2 vertices, 2 edges, 2 faces —
        // Euler–Poincaré v − e + f = 2 − 2 + 2 = 2 = 2(s − h) + r with
        // s = 1, h = r = 0 (a sphere). Replaces M0's single-face tiny().
        let t = pillow();
        assert_eq!(validate(&t.body), Ok(()));
        assert_eq!(t.body.vertices().count(), 2);
        assert_eq!(t.body.edges().count(), 2);
        assert_eq!(t.body.faces().count(), 2);
        assert_eq!(t.body.half_edges().count(), 4);
    }

    #[test]
    fn self_loop_digon_validates_cleanly() {
        // n = 1: one vertex, one self-loop edge whose two halves live in
        // different faces' one-half-edge loops. A legal (tier-1 and
        // tier-2) closed manifold body: v − e + f = 1 − 1 + 2 = 2.
        let t = ngon_pillow(1);
        assert_eq!(validate(&t.body), Ok(()));
        assert_eq!(t.body.vertices().count(), 1);
        assert_eq!(t.body.edges().count(), 1);
        assert_eq!(t.body.faces().count(), 2);
    }

    #[test]
    fn mvfs_state_validates_cleanly() {
        // The skeletal mvfs state (empty outer loop + lone vertex) is
        // tier-1-legal BY DESIGN: it is the state every Euler
        // construction starts from. Tier 2 (validate_closed) bans it on
        // finished solids; tier 1 must accept it.
        let t = mvfs_state();
        assert_eq!(validate(&t.body), Ok(()));
    }

    #[test]
    fn prism_validates_cleanly() {
        let t = prism(4);
        assert_eq!(validate(&t.body), Ok(()));
        // v = 2n, e = 3n, f = n + 2: v − e + f = 8 − 12 + 6 = 2.
        assert_eq!(t.body.vertices().count(), 8);
        assert_eq!(t.body.edges().count(), 12);
        assert_eq!(t.body.faces().count(), 6);
    }

    // ------------------------------------------------------------------
    // Orbit-step direction: the ratified derivation, tested against a
    // hand-computed prism vertex (see the entity module docs).
    // ------------------------------------------------------------------

    #[test]
    fn orbit_step_next_mate_walks_clockwise_from_outside() {
        // At top-rim vertex t[i] of the prism (indices counterclockwise
        // viewed from above = from outside above the cap), the emanating
        // half-edges are: ht[i] (toward t[i+1]), s3[i] (down toward
        // u[i]), s2[i-1] (toward t[i-1]). Viewed from outside,
        // "toward t[i+1]" → "down" → "toward t[i-1]" is CLOCKWISE — and
        // that is exactly the next(mate(·)) order. (GWB states its orbit
        // idiom for the mirrored clockwise-loop convention; this test is
        // the transcription guard.)
        let t = prism(4);
        let i = 1;
        assert_eq!(
            t.body.vertex_orbit(t.ht[i]),
            Some(vec![t.ht[i], t.s3[i], t.s2[i - 1]])
        );
        // Bottom-rim vertex u[i]: s0[i] (toward u[i+1]), hb[i-1]
        // (toward u[i-1]... as start of the bottom cap's walk), s1[i-1]
        // (up toward t[i]).
        assert_eq!(
            t.body.vertex_orbit(t.s0[i]),
            Some(vec![t.s0[i], t.hb[i - 1], t.s1[i - 1]])
        );
        // The inverse step mate(prev(·)) walks the SAME orbit
        // counterclockwise: its first step from ht[i] is the CW orbit's
        // last member.
        let prev = t.body.get_half_edge(t.ht[i]).unwrap().prev;
        assert_eq!(prev, t.ht[i - 1]);
        assert_eq!(t.body.mate(prev), Some(t.s2[i - 1]));
    }

    #[test]
    fn orbit_steps_are_mutual_inverses_and_preserve_start() {
        let t = prism(3);
        for (he_key, he) in t.body.half_edges() {
            // cw(he) = next(mate(he)) starts at the same vertex...
            let mate = t.body.mate(he_key).unwrap();
            let cw = t.body.get_half_edge(mate).unwrap().next;
            assert_eq!(t.body.get_half_edge(cw).unwrap().start, he.start);
            // ...and ccw(cw(he)) = mate(prev(cw(he))) returns to he.
            let cw_prev = t.body.get_half_edge(cw).unwrap().prev;
            assert_eq!(t.body.mate(cw_prev), Some(he_key));
        }
    }

    // ------------------------------------------------------------------
    // One test per error variant, each asserting the EXACT error vector —
    // the corruption is surgical, so every reported error is reasoned
    // about in the test. Where a corruption necessarily breaks several
    // invariants at once (e.g. an overrun requires a torn next/prev
    // pair), the full expected vector is spelled out in documented pass
    // order — these double as report-order tests.
    //
    // Malformed bodies are built through the public raw API (including
    // the _mut patching accessors) plus direct pub(crate) arena access
    // for removals, which is exactly what unit tests are for.
    // ------------------------------------------------------------------

    #[test]
    fn dangling_topology_is_reported() {
        let mut t = pillow();
        // Mint a key that can never resolve again: insert, then remove
        // (the slot's version is bumped; even reuse cannot revive it).
        let dead = t.body.add_half_edge(
            crate::entity::HalfEdge {
                edge: t.edges[0],
                start: t.vertices[0],
                parent_loop: t.loop_a,
                next: t.hes_a[0],
                prev: t.hes_a[0],
            },
            prov(),
        );
        t.body.half_edges.remove(dead);
        // Raw arena removal leaves the provenance record behind; remove
        // it too so the dangling reference is this test's ONLY defect
        // (the leak variant has its own test).
        t.body.half_edge_provenance.remove(dead);
        t.body.get_half_edge_mut(t.hes_a[1]).unwrap().next = dead;
        // Loop A's walk breaks (silent: this very dangling is the cause),
        // e1's antiparallelism is underivable (skipped), v0's orbit
        // breaks (silent). Exactly one error.
        assert_eq!(
            validate(&t.body),
            Err(vec![ValidationError::DanglingTopology {
                from: EntityId::HalfEdge(t.hes_a[1]),
                to: EntityId::HalfEdge(dead),
            }])
        );
    }

    #[test]
    fn dangling_empty_loop_vertex_is_reported() {
        let mut t = mvfs_state();
        t.body.vertices.remove(t.vertex);
        // Provenance removed with the entity (leaks have their own test).
        t.body.vertex_provenance.remove(t.vertex);
        assert_eq!(
            validate(&t.body),
            Err(vec![
                ValidationError::DanglingTopology {
                    from: EntityId::Loop(t.lone_loop),
                    to: EntityId::Vertex(t.vertex),
                },
                ValidationError::OrphanGeometry {
                    geometry: GeomRef::Point(t.point),
                },
            ])
        );
    }

    #[test]
    fn dangling_geometry_is_reported() {
        let mut t = pillow();
        // Repoint an existing vertex at a removed point: the vertex's
        // reference dangles, and the abandoned point becomes an orphan.
        let dead = t.body.add_point(anchor());
        t.body.points.remove(dead);
        t.body.get_vertex_mut(t.vertices[0]).unwrap().point = dead;
        assert_eq!(
            validate(&t.body),
            Err(vec![
                ValidationError::DanglingGeometry {
                    from: EntityId::Vertex(t.vertices[0]),
                    to: GeomRef::Point(dead),
                },
                ValidationError::OrphanGeometry {
                    geometry: GeomRef::Point(t.points[0]),
                },
            ])
        );
    }

    #[test]
    fn dangling_back_pointer_reports_once() {
        // A dangling back-pointer is a pass-1 error only: pass 7's
        // mismatch comparison is gated on a RESOLVING stored key, so the
        // defect is reported exactly once.
        let mut t = mvfs_state();
        let dead = t.body.add_shell(
            Shell {
                faces: vec![],
                solid: t.solid,
            },
            prov(),
        );
        t.body.shells.remove(dead);
        // Provenance removed with the entity (leaks have their own test).
        t.body.shell_provenance.remove(dead);
        t.body.get_face_mut(t.face).unwrap().shell = dead;
        assert_eq!(
            validate(&t.body),
            Err(vec![ValidationError::DanglingTopology {
                from: EntityId::Face(t.face),
                to: EntityId::Shell(dead),
            }])
        );
    }

    #[test]
    fn next_prev_mismatch_is_reported() {
        let mut t = pillow();
        // Tear prev only: a1's true predecessor (via next) is a0, so the
        // check fires for a0 ("my successor does not point back") and
        // for nothing else — next itself is intact, so cycles and orbits
        // still close.
        let a1 = t.hes_a[1];
        t.body.get_half_edge_mut(a1).unwrap().prev = a1;
        assert_eq!(
            validate(&t.body),
            Err(vec![ValidationError::NextPrevMismatch {
                half_edge: t.hes_a[0],
            }])
        );
    }

    #[test]
    fn loop_cycle_overrun_is_reported() {
        let mut t = pillow();
        // Point a0's next into loop B's cycle. A pure overrun is
        // impossible: while next/prev stay mutual inverses next is a
        // permutation and every walk closes — so the expected vector
        // necessarily pairs the overrun with the NextPrevMismatch that
        // tore the permutation, plus the orbit overrun at v1 whose orbit
        // permutation is equally torn. Documented pass order: 2a, 2b, 6.
        t.body.get_half_edge_mut(t.hes_a[0]).unwrap().next = t.hes_b[0];
        assert_eq!(
            validate(&t.body),
            Err(vec![
                ValidationError::NextPrevMismatch {
                    half_edge: t.hes_a[0],
                },
                ValidationError::LoopCycleOverrun { loop_: t.loop_a },
                ValidationError::VertexOrbitOverrun {
                    vertex: t.vertices[1],
                },
            ])
        );
    }

    #[test]
    fn parent_loop_mismatch_is_reported() {
        let mut t = pillow();
        // b0 claims loop A as parent while sitting in loop B's cycle:
        // loop B's walk reports the mismatch, and b0 is simultaneously
        // unreachable from its claimed parent (whose cycle closed
        // without it) — two true statements, two errors.
        t.body.get_half_edge_mut(t.hes_b[0]).unwrap().parent_loop = t.loop_a;
        assert_eq!(
            validate(&t.body),
            Err(vec![
                ValidationError::ParentLoopMismatch {
                    half_edge: t.hes_b[0],
                    owner: t.loop_b,
                },
                ValidationError::UnreachableHalfEdge {
                    half_edge: t.hes_b[0],
                },
            ])
        );
    }

    #[test]
    fn half_edge_claiming_an_empty_parent_is_unreachable() {
        let mut t = pillow();
        // A fresh lone vertex in an empty loop (legal), then a1 claims
        // that empty loop as its parent: an empty loop reaches nothing.
        let p2 = t.body.add_point(anchor());
        let v2 = t.body.add_vertex(
            Vertex {
                point: p2,
                emanating: None,
            },
            prov(),
        );
        let empty = add_empty_loop_face(&mut t.body, t.shell, v2);
        t.body.get_half_edge_mut(t.hes_a[1]).unwrap().parent_loop = empty;
        assert_eq!(
            validate(&t.body),
            Err(vec![
                ValidationError::ParentLoopMismatch {
                    half_edge: t.hes_a[1],
                    owner: t.loop_a,
                },
                ValidationError::UnreachableHalfEdge {
                    half_edge: t.hes_a[1],
                },
            ])
        );
    }

    #[test]
    fn edge_halves_identical_is_reported() {
        let mut t = pillow();
        // e0 claims a0 in both slots: a0 is now claimed twice overall
        // and b0 (e0's real minus half) by nobody. Antiparallelism is
        // gated on distinct halves (skipped); both orbits break at the
        // mate of an unclaimed half-edge (silent — the claim errors are
        // the cause). Pass order: 3 per-edge, then 3 claim counts in
        // half-edge slot order.
        t.body.get_edge_mut(t.edges[0]).unwrap().he_minus = t.hes_a[0];
        assert_eq!(
            validate(&t.body),
            Err(vec![
                ValidationError::EdgeHalvesIdentical { edge: t.edges[0] },
                ValidationError::HalfEdgeMultiplyClaimed {
                    half_edge: t.hes_a[0],
                    claims: 2,
                },
                ValidationError::HalfEdgeUnclaimed {
                    half_edge: t.hes_b[0],
                },
            ])
        );
    }

    #[test]
    fn edge_slot_backpointer_mismatch_is_reported() {
        let mut t = pillow();
        // e0's minus slot claims b1 (whose .edge is e1): back-pointer
        // mismatch; b0 goes unclaimed, b1 doubly claimed; and e0's
        // halves (a0, b1) both run v0 → v1, so antiparallelism genuinely
        // fails too. Orbits break at b0's missing mate (silent).
        t.body.get_edge_mut(t.edges[0]).unwrap().he_minus = t.hes_b[1];
        assert_eq!(
            validate(&t.body),
            Err(vec![
                ValidationError::EdgeSlotBackpointerMismatch {
                    edge: t.edges[0],
                    half_edge: t.hes_b[1],
                },
                ValidationError::HalfEdgeUnclaimed {
                    half_edge: t.hes_b[0],
                },
                ValidationError::HalfEdgeMultiplyClaimed {
                    half_edge: t.hes_b[1],
                    claims: 2,
                },
                ValidationError::EdgeNotAntiparallel { edge: t.edges[0] },
            ])
        );
    }

    #[test]
    fn edge_not_antiparallel_is_reported() {
        let mut t = pillow();
        // Swap loop B's start vertices: both edges' halves now run the
        // same way (parallel, not antiparallel), and both orbits close
        // over a foreign member — the b half-edge that now starts at the
        // OTHER vertex. Incidence counts are unchanged (2 per vertex),
        // so no anchoring errors. Pass order: 4 (edges), then 6
        // (vertices, walk order).
        let v0 = t.vertices[0];
        let v1 = t.vertices[1];
        t.body.get_half_edge_mut(t.hes_b[0]).unwrap().start = v0;
        t.body.get_half_edge_mut(t.hes_b[1]).unwrap().start = v1;
        assert_eq!(
            validate(&t.body),
            Err(vec![
                ValidationError::EdgeNotAntiparallel { edge: t.edges[0] },
                ValidationError::EdgeNotAntiparallel { edge: t.edges[1] },
                ValidationError::OrbitForeignMember {
                    vertex: v0,
                    half_edge: t.hes_b[1],
                },
                ValidationError::OrbitForeignMember {
                    vertex: v1,
                    half_edge: t.hes_b[0],
                },
            ])
        );
    }

    #[test]
    fn emanating_start_mismatch_is_reported() {
        let mut t = pillow();
        // v0's emanating points at a half-edge starting at v1. The orbit
        // check is gated on a matching start (skipped for v0), so the
        // mismatch is the single report.
        t.body.get_vertex_mut(t.vertices[0]).unwrap().emanating = Some(t.hes_a[1]);
        assert_eq!(
            validate(&t.body),
            Err(vec![ValidationError::EmanatingStartMismatch {
                vertex: t.vertices[0],
                emanating: t.hes_a[1],
            }])
        );
    }

    #[test]
    fn empty_loop_vertex_with_emanating_is_reported() {
        let mut t = pillow();
        // An empty loop claiming v0, which has half-edges: a lone vertex
        // must have none.
        add_empty_loop_face(&mut t.body, t.shell, t.vertices[0]);
        assert_eq!(
            validate(&t.body),
            Err(vec![ValidationError::EmptyLoopVertexWithEmanating {
                vertex: t.vertices[0],
                empty_loops: 1,
            }])
        );
    }

    #[test]
    fn lone_vertex_with_incidence_is_reported() {
        let mut t = pillow();
        // v0 claims to be lone (emanating: None) while two half-edges
        // start at it. The orbit check needs an emanating (skipped).
        t.body.get_vertex_mut(t.vertices[0]).unwrap().emanating = None;
        assert_eq!(
            validate(&t.body),
            Err(vec![ValidationError::LoneVertexWithIncidence {
                vertex: t.vertices[0],
                incident: 2,
            }])
        );
    }

    #[test]
    fn orphan_vertex_is_reported() {
        let mut t = pillow();
        // A vertex no half-edge starts at and no empty loop holds — the
        // M0 orphan-vertex rule restated in half-edge terms. Its point
        // is NOT an orphan: geometry referenced by an orphan entity is
        // still referenced.
        let p = t.body.add_point(anchor());
        let v = t.body.add_vertex(
            Vertex {
                point: p,
                emanating: None,
            },
            prov(),
        );
        assert_eq!(
            validate(&t.body),
            Err(vec![ValidationError::OrphanEntity {
                entity: EntityId::Vertex(v),
            }])
        );
    }

    #[test]
    fn orphaned_empty_loop_vertex_is_reported() {
        let mut t = mvfs_state();
        // Repoint the empty loop at a fresh lone vertex: the original
        // lone vertex loses its only anchor (no half-edges start at it
        // and no empty loop holds it any more) — the mvfs-state
        // counterpart of orphaning a vertex.
        let p2 = t.body.add_point(anchor());
        let v2 = t.body.add_vertex(
            Vertex {
                point: p2,
                emanating: None,
            },
            prov(),
        );
        t.body.get_loop_mut(t.lone_loop).unwrap().boundary = LoopBoundary::Empty { vertex: v2 };
        assert_eq!(
            validate(&t.body),
            Err(vec![ValidationError::OrphanEntity {
                entity: EntityId::Vertex(t.vertex),
            }])
        );
    }

    #[test]
    fn vertex_in_two_empty_loops_is_multiply_owned() {
        let mut t = mvfs_state();
        // A second empty loop claiming the same lone vertex: empty-loop
        // ownership of a vertex is exclusive.
        add_empty_loop_face(&mut t.body, t.shell, t.vertex);
        assert_eq!(
            validate(&t.body),
            Err(vec![ValidationError::MultiplyOwned {
                child: EntityId::Vertex(t.vertex),
                owners: 2,
            }])
        );
    }

    #[test]
    fn split_vertex_orbit_is_reported() {
        // The classic non-manifold "bowtie": glue a second, self-loop
        // digon pillow onto v0. Every local invariant holds — cycles
        // close, mates pair, antiparallelism holds, emanating matches —
        // but v0's incident half-edges fall into TWO orbits, and the
        // orbit-closure check is exactly what catches it.
        let mut t = pillow();
        let v0 = t.vertices[0];
        let curve = t.body.add_curve(crate::fixtures::test_curve(anchor()));
        let e2 = t.body.add_edge(
            crate::entity::Edge {
                he_plus: HalfEdgeKey::default(),
                he_minus: HalfEdgeKey::default(),
                curve,
            },
            prov(),
        );
        let self_loop_face = |body: &mut Body<f64>| {
            let he = body.add_half_edge(
                crate::entity::HalfEdge {
                    edge: e2,
                    start: v0,
                    parent_loop: LoopKey::default(),
                    next: HalfEdgeKey::default(),
                    prev: HalfEdgeKey::default(),
                },
                prov(),
            );
            let surface = body.add_surface(crate::fixtures::test_surface(anchor()));
            let lp = body.add_loop(
                Loop {
                    boundary: LoopBoundary::Cycle { first: he },
                    face: FaceKey::default(),
                },
                prov(),
            );
            let f = body.add_face(
                Face {
                    sense: true,
                    surface,
                    outer: lp,
                    rings: vec![],
                    shell: t.shell,
                },
                prov(),
            );
            body.get_loop_mut(lp).unwrap().face = f;
            body.get_shell_mut(t.shell).unwrap().faces.push(f);
            let h = body.get_half_edge_mut(he).unwrap();
            h.parent_loop = lp;
            h.next = he;
            h.prev = he;
            he
        };
        let ap = self_loop_face(&mut t.body);
        let bp = self_loop_face(&mut t.body);
        let e = t.body.get_edge_mut(e2).unwrap();
        e.he_plus = ap;
        e.he_minus = bp;
        // v0 now has 4 incident half-edges but its orbit (from the
        // pillow's a0) closes over 2.
        assert_eq!(
            validate(&t.body),
            Err(vec![ValidationError::SplitVertexOrbit {
                vertex: v0,
                orbit: 2,
                incident: 4,
            }])
        );
    }

    #[test]
    fn outer_listed_as_ring_is_reported() {
        let mut t = pillow();
        // outer ∈ rings is both the designation error and a double
        // ownership (the loop is counted once as outer, once as ring) —
        // two true statements, two errors, in pass-7 order.
        t.body.get_face_mut(t.face_a).unwrap().rings = vec![t.loop_a];
        assert_eq!(
            validate(&t.body),
            Err(vec![
                ValidationError::OuterListedAsRing { face: t.face_a },
                ValidationError::MultiplyOwned {
                    child: EntityId::Loop(t.loop_a),
                    owners: 2,
                },
            ])
        );
    }

    #[test]
    fn loop_back_pointer_mismatch_is_reported() {
        let mut t = pillow();
        t.body.get_loop_mut(t.loop_b).unwrap().face = t.face_a;
        assert_eq!(
            validate(&t.body),
            Err(vec![ValidationError::BackPointerMismatch {
                child: EntityId::Loop(t.loop_b),
                stored: EntityId::Face(t.face_a),
                owner: EntityId::Face(t.face_b),
            }])
        );
    }

    #[test]
    fn face_and_shell_back_pointer_mismatches_are_reported() {
        let mut t = pillow();
        // A second (empty but owned) shell+solid to point at.
        let solid2 = t.body.add_solid(Solid { shells: vec![] }, prov());
        let shell2 = t.body.add_shell(
            Shell {
                faces: vec![],
                solid: solid2,
            },
            prov(),
        );
        t.body.get_solid_mut(solid2).unwrap().shells.push(shell2);
        t.body.get_shell_mut(t.shell).unwrap().solid = solid2;
        t.body.get_face_mut(t.face_b).unwrap().shell = shell2;
        // Pass 7 order: shells before faces. shell2's face list stayed
        // empty (only face_b's back-pointer moved), so the pass-9 arity
        // floor genuinely fires too.
        assert_eq!(
            validate(&t.body),
            Err(vec![
                ValidationError::BackPointerMismatch {
                    child: EntityId::Shell(t.shell),
                    stored: EntityId::Solid(solid2),
                    owner: EntityId::Solid(t.solid),
                },
                ValidationError::BackPointerMismatch {
                    child: EntityId::Face(t.face_b),
                    stored: EntityId::Shell(shell2),
                    owner: EntityId::Shell(t.shell),
                },
                ValidationError::ShellWithoutFaces { shell: shell2 },
            ])
        );
    }

    #[test]
    fn orphan_shell_is_reported() {
        let mut t = pillow();
        let sh2 = t.body.add_shell(
            Shell {
                faces: vec![],
                solid: t.solid,
            },
            prov(),
        );
        // sh2 is not in any solid's shell list (ownership counts, not
        // back-pointers, define anchoring — and with zero owners the
        // back-pointer comparison is skipped). Being face-less, it also
        // trips the pass-9 arity floor — two true statements, two
        // errors, in pass order.
        assert_eq!(
            validate(&t.body),
            Err(vec![
                ValidationError::OrphanEntity {
                    entity: EntityId::Shell(sh2),
                },
                ValidationError::ShellWithoutFaces { shell: sh2 },
            ])
        );
    }

    #[test]
    fn multiply_owned_shell_is_reported() {
        let mut t = pillow();
        let sh = t.shell;
        t.body.get_solid_mut(t.solid).unwrap().shells.push(sh);
        assert_eq!(
            validate(&t.body),
            Err(vec![ValidationError::MultiplyOwned {
                child: EntityId::Shell(sh),
                owners: 2,
            }])
        );
    }

    #[test]
    fn orphan_geometry_is_reported_for_all_three_arenas() {
        let mut t = pillow();
        let p = t.body.add_point(anchor());
        let c = t.body.add_curve(crate::fixtures::test_curve(anchor()));
        let s = t.body.add_surface(crate::fixtures::test_surface(anchor()));
        assert_eq!(
            validate(&t.body),
            Err(vec![
                ValidationError::OrphanGeometry {
                    geometry: GeomRef::Point(p),
                },
                ValidationError::OrphanGeometry {
                    geometry: GeomRef::Curve(c),
                },
                ValidationError::OrphanGeometry {
                    geometry: GeomRef::Surface(s),
                },
            ])
        );
    }

    #[test]
    fn errors_display_without_panicking() {
        // One sample per `ValidationError` variant.
        //
        // What the compiler enforces: `variant_index` matches the enum
        // with NO wildcard arm, so a new variant fails to build until an
        // arm exists for it, and the coverage assertion then names the
        // variant whose sample is missing.
        //
        // What it does NOT enforce: `VARIANTS` is hand-written, so a new
        // variant given an arm but no sample still passes. Closing that
        // needs the variant count from the compiler — `strum`'s
        // `EnumCount` derive or the workspace's first proc-macro crate —
        // and neither is bought here. When you add an arm, its index is
        // the new `VARIANTS - 1`.
        const VARIANTS: usize = 59;
        fn variant_index(e: &ValidationError) -> usize {
            match e {
                ValidationError::Band { .. } => 0,
                ValidationError::DanglingDescription { .. } => 1,
                ValidationError::UncertifiableSurface { .. } => 2,
                ValidationError::EdgeCertification { .. } => 3,
                ValidationError::DescriptionNotAdjacent { .. } => 4,
                ValidationError::PlanarFaceResidual { .. } => 5,
                ValidationError::PlanarFaceEscalated { .. } => 6,
                ValidationError::PlanarBoundaryResidual { .. } => 7,
                ValidationError::PlanarBoundaryEscalated { .. } => 8,
                ValidationError::SliverDihedral { .. } => 9,
                ValidationError::TransverseNotIntrinsic { .. } => 10,
                ValidationError::TangentNotIntrinsic { .. } => 11,
                ValidationError::LoopRoleInverted { .. } => 12,
                ValidationError::CurvedSenseInverted { .. } => 13,
                ValidationError::NegativeVolume => 14,
                ValidationError::VolumeUncomputable { .. } => 15,
                ValidationError::Pcurve { .. } => 16,
                ValidationError::UndeclaredContact { .. } => 17,
                ValidationError::StaleContactDeclaration { .. } => 18,
                ValidationError::ContactContradicted { .. } => 19,
                ValidationError::CensusEscalated { .. } => 20,
                ValidationError::CensusUnsupported { .. } => 21,
                ValidationError::CensusUndecidable { .. } => 22,
                ValidationError::DanglingTopology { .. } => 23,
                ValidationError::DanglingGeometry { .. } => 24,
                ValidationError::NextPrevMismatch { .. } => 25,
                ValidationError::LoopCycleOverrun { .. } => 26,
                ValidationError::ParentLoopMismatch { .. } => 27,
                ValidationError::UnreachableHalfEdge { .. } => 28,
                ValidationError::EdgeHalvesIdentical { .. } => 29,
                ValidationError::EdgeSlotBackpointerMismatch { .. } => 30,
                ValidationError::HalfEdgeUnclaimed { .. } => 31,
                ValidationError::HalfEdgeMultiplyClaimed { .. } => 32,
                ValidationError::EdgeNotAntiparallel { .. } => 33,
                ValidationError::EmanatingStartMismatch { .. } => 34,
                ValidationError::EmptyLoopVertexWithEmanating { .. } => 35,
                ValidationError::LoneVertexWithIncidence { .. } => 36,
                ValidationError::VertexOrbitOverrun { .. } => 37,
                ValidationError::OrbitForeignMember { .. } => 38,
                ValidationError::SplitVertexOrbit { .. } => 39,
                ValidationError::OuterListedAsRing { .. } => 40,
                ValidationError::BackPointerMismatch { .. } => 41,
                ValidationError::OrphanEntity { .. } => 42,
                ValidationError::MultiplyOwned { .. } => 43,
                ValidationError::OrphanGeometry { .. } => 44,
                ValidationError::SolidWithoutShells { .. } => 45,
                ValidationError::ShellWithoutFaces { .. } => 46,
                ValidationError::EdgeAcrossShells { .. } => 47,
                ValidationError::ComponentEulerViolation { .. } => 48,
                ValidationError::MissingProvenance { .. } => 49,
                ValidationError::LeakedProvenance { .. } => 50,
                ValidationError::ScaffoldingEmptyLoop { .. } => 51,
                ValidationError::ScaffoldingStrutVertex { .. } => 52,
                ValidationError::ShellDisconnected { .. } => 53,
                ValidationError::NullScaffoldShared { .. } => 54,
                ValidationError::LeakedNullFaceRecord { .. } => 55,
                ValidationError::StaleNullFaceLoop { .. } => 56,
                ValidationError::NullEdgeAtRest { .. } => 57,
                ValidationError::NullFaceAtRest { .. } => 58,
            }
        }
        fn band_error() -> geom_core::BandError {
            geom_core::Band::new(1.0, 0.0).unwrap_err()
        }
        fn indeterminate() -> Indeterminate {
            Indeterminate {
                margin: geom_core::MarginDiag::Value(5e-9),
                band: geom_core::Band::new(1e-9, 1e-8).unwrap(),
                predicate: Some("validate_probe"),
            }
        }
        let t = pillow();
        let he = t.hes_a[0];
        let v = t.vertices[0];
        let e = t.edges[0];
        let all: Vec<ValidationError> = vec![
            ValidationError::DanglingTopology {
                from: EntityId::Solid(t.solid),
                to: EntityId::Shell(t.shell),
            },
            ValidationError::DanglingGeometry {
                from: EntityId::Vertex(v),
                to: GeomRef::Point(t.points[0]),
            },
            ValidationError::NextPrevMismatch { half_edge: he },
            ValidationError::LoopCycleOverrun { loop_: t.loop_a },
            ValidationError::ParentLoopMismatch {
                half_edge: he,
                owner: t.loop_a,
            },
            ValidationError::UnreachableHalfEdge { half_edge: he },
            ValidationError::EdgeHalvesIdentical { edge: e },
            ValidationError::EdgeSlotBackpointerMismatch {
                edge: e,
                half_edge: he,
            },
            ValidationError::HalfEdgeUnclaimed { half_edge: he },
            ValidationError::HalfEdgeMultiplyClaimed {
                half_edge: he,
                claims: 2,
            },
            ValidationError::EdgeNotAntiparallel { edge: e },
            ValidationError::EmanatingStartMismatch {
                vertex: v,
                emanating: he,
            },
            ValidationError::EmptyLoopVertexWithEmanating {
                vertex: v,
                empty_loops: 1,
            },
            ValidationError::LoneVertexWithIncidence {
                vertex: v,
                incident: 2,
            },
            ValidationError::VertexOrbitOverrun { vertex: v },
            ValidationError::OrbitForeignMember {
                vertex: v,
                half_edge: he,
            },
            ValidationError::SplitVertexOrbit {
                vertex: v,
                orbit: 2,
                incident: 4,
            },
            ValidationError::OuterListedAsRing { face: t.face_a },
            ValidationError::BackPointerMismatch {
                child: EntityId::Loop(t.loop_a),
                stored: EntityId::Face(t.face_a),
                owner: EntityId::Face(t.face_b),
            },
            ValidationError::OrphanEntity {
                entity: EntityId::Vertex(v),
            },
            ValidationError::MultiplyOwned {
                child: EntityId::Shell(t.shell),
                owners: 2,
            },
            ValidationError::OrphanGeometry {
                geometry: GeomRef::Point(t.points[0]),
            },
            ValidationError::SolidWithoutShells { solid: t.solid },
            ValidationError::ShellWithoutFaces { shell: t.shell },
            ValidationError::EdgeAcrossShells {
                edge: e,
                shell_plus: t.shell,
                shell_minus: t.shell,
            },
            ValidationError::ComponentEulerViolation {
                shell: t.shell,
                seed: t.face_a,
                vertices: 2,
                edges: 2,
                faces: 1,
                rings: 0,
            },
            ValidationError::MissingProvenance {
                entity: EntityId::Face(t.face_a),
            },
            ValidationError::LeakedProvenance {
                entity: EntityId::Face(t.face_a),
            },
            ValidationError::ScaffoldingEmptyLoop { loop_: t.loop_a },
            ValidationError::ScaffoldingStrutVertex { vertex: v },
            ValidationError::ShellDisconnected {
                shell: t.shell,
                components: 2,
            },
            ValidationError::Band {
                error: band_error(),
            },
            ValidationError::DanglingDescription {
                from: GeomRef::Point(t.points[0]),
                to: GeomRef::Point(t.points[0]),
            },
            ValidationError::UncertifiableSurface { face: t.face_a },
            ValidationError::EdgeCertification {
                edge: e,
                error: CertifyError::Unimplemented,
            },
            ValidationError::DescriptionNotAdjacent { edge: e },
            ValidationError::PlanarFaceResidual {
                face: t.face_a,
                vertex: v,
            },
            ValidationError::PlanarFaceEscalated {
                face: t.face_a,
                vertex: v,
                cause: indeterminate(),
            },
            ValidationError::PlanarBoundaryResidual {
                face: t.face_a,
                edge: e,
            },
            ValidationError::PlanarBoundaryEscalated {
                face: t.face_a,
                edge: e,
                cause: indeterminate(),
            },
            ValidationError::SliverDihedral {
                edge: e,
                cause: indeterminate(),
            },
            ValidationError::TransverseNotIntrinsic { edge: e },
            ValidationError::TangentNotIntrinsic { edge: e },
            ValidationError::LoopRoleInverted {
                face: t.face_a,
                r#loop: t.loop_a,
            },
            ValidationError::CurvedSenseInverted { face: t.face_a },
            ValidationError::NegativeVolume,
            ValidationError::VolumeUncomputable {
                source: crate::props::MassPropsError::Band {
                    error: band_error(),
                },
            },
            ValidationError::Pcurve {
                finding: crate::pcurves::PcurveMintError::Corrupt,
            },
            ValidationError::UndeclaredContact {
                contact: CensusContact::VertexVertex { a: v, b: v },
                witness: "witness".to_string(),
            },
            ValidationError::StaleContactDeclaration {
                declaration: StaleDeclaration::VertexVertex { a: v, b: v },
            },
            ValidationError::ContactContradicted {
                declaration: DeclaredContact {
                    a: t.face_a,
                    b: t.face_b,
                    class: crate::ContactClass::Rest,
                },
                witness: "witness".to_string(),
                margin: indeterminate(),
                steer: None,
            },
            ValidationError::CensusEscalated {
                cause: indeterminate(),
            },
            ValidationError::CensusUnsupported {
                entity: EntityId::Face(t.face_a),
            },
            ValidationError::CensusUndecidable {
                a: EntityId::Face(t.face_a),
                b: EntityId::Face(t.face_a),
                what: "what",
            },
            ValidationError::NullScaffoldShared {
                curve: CurveKey::default(),
                edges: 2,
            },
            ValidationError::LeakedNullFaceRecord { face: t.face_a },
            ValidationError::StaleNullFaceLoop {
                face: t.face_a,
                named_loop: t.loop_a,
            },
            ValidationError::NullEdgeAtRest { edge: e },
            ValidationError::NullFaceAtRest { face: t.face_a },
        ];
        let mut covered = [false; VARIANTS];
        for err in &all {
            // Display and Error are wired up; content is human-oriented.
            assert!(!err.to_string().is_empty());
            let _: &dyn std::error::Error = err;
            covered[variant_index(err)] = true;
        }
        assert!(
            covered.iter().all(|&c| c),
            "every ValidationError variant needs a Display sample; missing index {:?}",
            covered.iter().position(|&c| !c),
        );
    }

    // ------------------------------------------------------------------
    // Pass 9: arity floors.
    // ------------------------------------------------------------------

    #[test]
    fn solid_without_shells_is_reported() {
        let mut t = pillow();
        // Solids are containment roots — nothing anchors them, so a bare
        // solid trips ONLY the arity floor.
        let bare = t.body.add_solid(Solid { shells: vec![] }, prov());
        assert_eq!(
            validate(&t.body),
            Err(vec![ValidationError::SolidWithoutShells { solid: bare }])
        );
    }

    #[test]
    fn shell_without_faces_is_reported() {
        let mut t = pillow();
        // Owned and back-pointed correctly — the missing faces are the
        // only defect (contrast the orphan-shell test, where BOTH fire).
        let sh2 = t.body.add_shell(
            Shell {
                faces: vec![],
                solid: t.solid,
            },
            prov(),
        );
        t.body.get_solid_mut(t.solid).unwrap().shells.push(sh2);
        assert_eq!(
            validate(&t.body),
            Err(vec![ValidationError::ShellWithoutFaces { shell: sh2 }])
        );
    }

    // ------------------------------------------------------------------
    // Passes 10 + 11: edge-adjacency shell coherence and the
    // component-aware per-shell Euler–Poincaré count.
    // ------------------------------------------------------------------

    #[test]
    fn face_moved_to_second_shell_is_caught_twice() {
        // The PR 1 review's named gap at pillow scale: move face B to a
        // fresh solid+shell with membership and back-pointers fully
        // self-consistent. Passes 1–7 accept it (they never compare an
        // edge's two shells); pass 10 reports both shared edges, and
        // pass 11 fails BOTH shells' components — a lone face cut from
        // its mates has v − e + f − r = 2 − 2 + 1 − 0 = 1, odd.
        // Documented order: pass 10 (edge arena order), then pass 11
        // (shell arena order).
        let mut t = pillow();
        let solid2 = t.body.add_solid(Solid { shells: vec![] }, prov());
        let shell2 = t.body.add_shell(
            Shell {
                faces: vec![t.face_b],
                solid: solid2,
            },
            prov(),
        );
        t.body.get_solid_mut(solid2).unwrap().shells.push(shell2);
        t.body.get_shell_mut(t.shell).unwrap().faces = vec![t.face_a];
        t.body.get_face_mut(t.face_b).unwrap().shell = shell2;
        assert_eq!(
            validate(&t.body),
            Err(vec![
                ValidationError::EdgeAcrossShells {
                    edge: t.edges[0],
                    shell_plus: t.shell,
                    shell_minus: shell2,
                },
                ValidationError::EdgeAcrossShells {
                    edge: t.edges[1],
                    shell_plus: t.shell,
                    shell_minus: shell2,
                },
                ValidationError::ComponentEulerViolation {
                    shell: t.shell,
                    seed: t.face_a,
                    vertices: 2,
                    edges: 2,
                    faces: 1,
                    rings: 0,
                },
                ValidationError::ComponentEulerViolation {
                    shell: shell2,
                    seed: t.face_b,
                    vertices: 2,
                    edges: 2,
                    faces: 1,
                    rings: 0,
                },
            ])
        );
    }

    #[test]
    fn cube_face_moved_to_second_shell_is_caught() {
        // The scenario exactly as the PR 1 review posed it: an
        // operator-built cube with one side face moved to a second
        // solid+shell. Four cross-shell edges (pass 10) plus two
        // component violations (pass 11): the 5-face side keeps all 8
        // vertices and all 12 edges (each moved edge still has its
        // other face here), χ = 8 − 12 + 5 = 1; the lone face has
        // χ = 4 − 4 + 1 = 1.
        let t = ops_cube();
        let mut body = t.body;
        let front = t.mefs[1].face;
        let old_shell = body.get_face(front).unwrap().shell;
        let solid2 = body.add_solid(Solid { shells: vec![] }, prov());
        let shell2 = body.add_shell(
            Shell {
                faces: vec![front],
                solid: solid2,
            },
            prov(),
        );
        body.get_solid_mut(solid2).unwrap().shells.push(shell2);
        body.get_shell_mut(old_shell)
            .unwrap()
            .faces
            .retain(|&f| f != front);
        body.get_face_mut(front).unwrap().shell = shell2;

        let errors = validate(&body).unwrap_err();
        let crossings = errors
            .iter()
            .filter(|e| matches!(e, ValidationError::EdgeAcrossShells { .. }))
            .count();
        assert_eq!(crossings, 4, "the moved face's four edges cross shells");
        let violations: Vec<_> = errors
            .iter()
            .filter_map(|e| match e {
                ValidationError::ComponentEulerViolation {
                    shell,
                    vertices,
                    edges,
                    faces,
                    rings,
                    ..
                } => Some((*shell, *vertices, *edges, *faces, *rings)),
                _ => None,
            })
            .collect();
        assert_eq!(
            violations,
            vec![(old_shell, 8, 12, 5, 0), (shell2, 4, 4, 1, 0)]
        );
        assert_eq!(errors.len(), 6, "nothing else fires");
    }

    // ------------------------------------------------------------------
    // Pass 12: bidirectional D5 provenance.
    // ------------------------------------------------------------------

    #[test]
    fn missing_provenance_is_reported() {
        let mut t = pillow();
        t.body.face_provenance.remove(t.face_a);
        assert_eq!(
            validate(&t.body),
            Err(vec![ValidationError::MissingProvenance {
                entity: EntityId::Face(t.face_a),
            }])
        );
    }

    #[test]
    fn leaked_provenance_is_reported() {
        // The provenance-leak fixture: remove an entity through the
        // pub(crate) arena WITHOUT removing its record — the exact bug
        // a kill-side operator would have if it forgot its kill-hygiene
        // duty, and the reason the bidirectional check exists.
        let mut t = pillow();
        let extra = t.body.add_solid(Solid { shells: vec![] }, prov());
        t.body.solids.remove(extra);
        assert_eq!(
            validate(&t.body),
            Err(vec![ValidationError::LeakedProvenance {
                entity: EntityId::Solid(extra),
            }])
        );
    }

    #[test]
    fn cross_pass_report_order_is_the_documented_one() {
        // One body with independent defects in passes 9, 10, 11, and 12:
        // the report arrives in exactly pass order. (The per-pass tests
        // above pin intra-pass order; this pins the pass sequence.)
        let mut t = pillow();
        let bare = t.body.add_solid(Solid { shells: vec![] }, prov()); // pass 9
        let solid2 = t.body.add_solid(Solid { shells: vec![] }, prov());
        let shell2 = t.body.add_shell(
            Shell {
                faces: vec![t.face_b],
                solid: solid2,
            },
            prov(),
        );
        t.body.get_solid_mut(solid2).unwrap().shells.push(shell2);
        t.body.get_shell_mut(t.shell).unwrap().faces = vec![t.face_a];
        t.body.get_face_mut(t.face_b).unwrap().shell = shell2; // passes 10+11
        let dead = t.body.add_solid(Solid { shells: vec![] }, prov());
        t.body.solids.remove(dead); // pass 12
        assert_eq!(
            validate(&t.body),
            Err(vec![
                ValidationError::SolidWithoutShells { solid: bare },
                ValidationError::EdgeAcrossShells {
                    edge: t.edges[0],
                    shell_plus: t.shell,
                    shell_minus: shell2,
                },
                ValidationError::EdgeAcrossShells {
                    edge: t.edges[1],
                    shell_plus: t.shell,
                    shell_minus: shell2,
                },
                ValidationError::ComponentEulerViolation {
                    shell: t.shell,
                    seed: t.face_a,
                    vertices: 2,
                    edges: 2,
                    faces: 1,
                    rings: 0,
                },
                ValidationError::ComponentEulerViolation {
                    shell: shell2,
                    seed: t.face_b,
                    vertices: 2,
                    edges: 2,
                    faces: 1,
                    rings: 0,
                },
                ValidationError::LeakedProvenance {
                    entity: EntityId::Solid(dead),
                },
            ])
        );
    }

    // ------------------------------------------------------------------
    // The PR 3 carry: two empty loops on ONE vertex, in one face — the
    // state `kemr`'s EmptyAnchorsCollide defends against. Pass 5's
    // exclusive empty-loop ownership is the rule that catches it.
    // ------------------------------------------------------------------

    #[test]
    fn two_empty_loops_on_one_vertex_in_one_face_are_multiply_owned() {
        let mut t = mvfs_state();
        let ring = t.body.add_loop(
            Loop {
                boundary: LoopBoundary::Empty { vertex: t.vertex },
                face: t.face,
            },
            prov(),
        );
        t.body.get_face_mut(t.face).unwrap().rings.push(ring);
        assert_eq!(
            validate(&t.body),
            Err(vec![ValidationError::MultiplyOwned {
                child: EntityId::Vertex(t.vertex),
                owners: 2,
            }])
        );
    }

    // ------------------------------------------------------------------
    // Tier 2: validate_closed. Closed bodies pass; every scaffolding
    // state fails with exactly the right variants, in the documented
    // tier-2 order.
    // ------------------------------------------------------------------

    fn p(x: f64) -> Point3<f64> {
        Point3::new(x, 0.0, 0.0)
    }

    #[test]
    fn closed_fixtures_pass_tier_two() {
        // Raw-built closed families…
        assert_eq!(validate_closed(&pillow().body), Ok(()));
        assert_eq!(validate_closed(&ngon_pillow(1).body), Ok(()));
        assert_eq!(validate_closed(&prism(4).body), Ok(()));
        // …and the operator-built acceptance bodies, genus 0 through 2.
        assert_eq!(validate_closed(&ops_cube().body), Ok(()));
        assert_eq!(validate_closed(&ops_holed_box().body), Ok(()));
        assert_eq!(validate_closed(&ops_genus2()), Ok(()));
    }

    /// Gives every face of `body` the Newell plane of its outer loop —
    /// the minimum needed to reach check 6 from [`ops_cube`], whose
    /// faces are raw `Nurbs` placeholders (check 6 only inspects
    /// `Surface::Plane` faces, so on the raw fixture it is vacuous).
    /// The loop order is the stored one, so the minted normal is the
    /// face's OUTWARD normal — which is what `sense: true` claims.
    fn plane_every_face(body: &mut Body<f64>) {
        let band = Band::linear().unwrap();
        let keys: Vec<FaceKey> = body.faces.keys().collect();
        for face_key in keys {
            let outer = body.get_face(face_key).unwrap().outer;
            let LoopBoundary::Cycle { first } = body.get_loop(outer).unwrap().boundary else {
                continue; // empty loop: no polygon to fit
            };
            let points: Vec<Point3<f64>> = body
                .loop_cycle(first)
                .unwrap()
                .into_iter()
                .map(|he| {
                    let v = body.get_half_edge(he).unwrap().start;
                    *body.get_point(body.get_vertex(v).unwrap().point).unwrap()
                })
                .collect();
            let plane = geom_brep::newell_plane(&points, band).unwrap();
            body.set_face_surface(face_key, crate::FaceSurface::New(plane))
                .unwrap();
        }
    }

    /// **M5 S10 acceptance row 1: tier 3 is the sense gate (check 6).**
    ///
    /// A face's outward normal is `Face::sense_sign()` times its
    /// surface's chart normal, and by the interior-left rule its outer
    /// loop winds CCW about that outward normal. `sense` and the
    /// stored winding are therefore two encodings of ONE fact, and
    /// check 6 — the loop's Newell functional against the OUTWARD
    /// normal — is exactly the gate that they agree.
    /// [`Body::flipped_face_sense_for_tests`] inverts one bit and
    /// nothing else, producing a body that is inside-out at that face;
    /// tier 3 must refuse it, by name.
    ///
    /// The row also pins why check 6 carries this alone: the +V
    /// invariant (check 7) is computed from the same loop windings,
    /// which a lone sense flip does not touch, so both bodies meter
    /// the identical volume — bit for bit. Nothing else in the at-rest
    /// battery can see the defect.
    ///
    /// Bit-identity: the honest body's report is unchanged by the S10
    /// threading (planar sweeps mint `sense: true` throughout — S11
    /// reverses only material-against-chart walls, none here — so the
    /// multiply is `· +1`) — pinned here as "no `LoopRoleInverted`
    /// before the flip". The fixture is [`ops_cube`] with real planes
    /// grafted on; its twelve chords stay conventional, so the honest
    /// report is the twelve `TransverseNotIntrinsic` complaints and
    /// nothing else. (The all-green variant of this row, on the fully
    /// certified cube, lives in `tests/geometric_cube.rs`.)
    #[test]
    fn tier_three_refuses_a_hand_flipped_face_sense() {
        let mut cube = ops_cube().body;
        plane_every_face(&mut cube);
        let honest = validate_geometric(&cube).unwrap_err();
        assert!(
            honest
                .iter()
                .all(|e| matches!(e, ValidationError::TransverseNotIntrinsic { .. })),
            "the grafted cube's only complaint is the conventional \
             chords; got {honest:?}"
        );

        let (face, f) = cube.faces.iter().next().unwrap();
        let outer = f.outer;
        let flipped = cube.flipped_face_sense_for_tests(face).unwrap();
        let errors = validate_geometric(&flipped).unwrap_err();
        assert!(
            errors.contains(&ValidationError::LoopRoleInverted {
                face,
                r#loop: outer,
            }),
            "check 6 must name the inverted face's outer loop; got {errors:?}"
        );

        // And why check 6 has to carry this alone: the +V invariant is
        // computed from the loop windings, which a lone sense flip does
        // not touch — the two bodies meter the SAME volume, so check 7
        // could not refuse the flipped one even if it ran.
        let volume = |b: &Body<f64>| crate::props::mass_properties(b).unwrap().volume;
        assert_eq!(
            volume(&cube).to_bits(),
            volume(&flipped).to_bits(),
            "check 7 is winding-derived and cannot see a lone sense flip"
        );
    }

    #[test]
    fn empty_body_passes_both_tiers_vacuously() {
        assert_eq!(validate(&Body::<f64>::new()), Ok(()));
        assert_eq!(validate_closed(&Body::<f64>::new()), Ok(()));
    }

    #[test]
    fn tier_two_rejects_the_skeletal_mvfs_state() {
        let mut body = Body::<f64>::new();
        let seed = body.mvfs(p(0.0)).unwrap();
        assert_eq!(validate(&body), Ok(()), "tier 1 accepts the seed state");
        // The lone vertex has valence 0, not 1, and the dartless
        // empty-outer face is one component — the empty loop is the
        // single tier-2 defect.
        assert_eq!(
            validate_closed(&body),
            Err(vec![ValidationError::ScaffoldingEmptyLoop {
                loop_: seed.r#loop,
            }])
        );
    }

    #[test]
    fn tier_two_rejects_struts() {
        // The segment body: BOTH endpoints have valence 1 (vertex-arena
        // order).
        let mut body = Body::<f64>::new();
        let seed = body.mvfs(p(0.0)).unwrap();
        let seg = body
            .mev_line(
                MevSite::Lone {
                    r#loop: seed.r#loop,
                },
                p(1.0),
            )
            .unwrap();
        assert_eq!(validate(&body), Ok(()));
        assert_eq!(
            validate_closed(&body),
            Err(vec![
                ValidationError::ScaffoldingStrutVertex {
                    vertex: seed.vertex,
                },
                ValidationError::ScaffoldingStrutVertex { vertex: seg.vertex },
            ])
        );

        // A strut hanging off a CLOSED pillow: exactly the tip (the
        // base has valence 3).
        let mut body = Body::<f64>::new();
        let seed = body.mvfs(p(0.0)).unwrap();
        let seg = body
            .mev_line(
                MevSite::Lone {
                    r#loop: seed.r#loop,
                },
                p(1.0),
            )
            .unwrap();
        body.mef_chord(MefSite::Chords {
            he1: seg.he_plus,
            he2: seg.he_minus,
        })
        .unwrap();
        let strut = body
            .mev_line(
                MevSite::Fan {
                    he1: seg.he_plus,
                    he2: seg.he_plus,
                },
                p(2.0),
            )
            .unwrap();
        assert_eq!(validate(&body), Ok(()));
        assert_eq!(
            validate_closed(&body),
            Err(vec![ValidationError::ScaffoldingStrutVertex {
                vertex: strut.vertex,
            }])
        );
    }

    #[test]
    fn tier_two_rejects_planted_empty_rings() {
        // Pillow + strut + kemr: the strut edge dies, its tip becomes
        // an empty ring's lone vertex (valence 0 — no strut report),
        // and the ring keeps the shell connected. One defect.
        let mut body = Body::<f64>::new();
        let seed = body.mvfs(p(0.0)).unwrap();
        let seg = body
            .mev_line(
                MevSite::Lone {
                    r#loop: seed.r#loop,
                },
                p(1.0),
            )
            .unwrap();
        body.mef_chord(MefSite::Chords {
            he1: seg.he_plus,
            he2: seg.he_minus,
        })
        .unwrap();
        let strut = body
            .mev_line(
                MevSite::Fan {
                    he1: seg.he_plus,
                    he2: seg.he_plus,
                },
                p(2.0),
            )
            .unwrap();
        let kill = body.kemr(strut.he_plus, strut.he_minus).unwrap();
        assert_eq!(validate(&body), Ok(()));
        assert_eq!(
            validate_closed(&body),
            Err(vec![ValidationError::ScaffoldingEmptyLoop {
                loop_: kill.ring,
            }])
        );
    }

    /// Pillow + a *detached digon* hanging on a promoted ring: plant an
    /// empty ring, grow it to a two-edge cycle (mev + mef), then
    /// `mfkrh` the ring into a face. The shell's surface is now TWO
    /// closed components (pillow; digon pair) with **no empty loops and
    /// no struts anywhere** — the PR 4 finding that made c = 1 its own
    /// tier-2 rule. Returns (body, shell).
    fn detached_digon_body() -> (Body<f64>, crate::entity::ShellKey) {
        let mut body = Body::<f64>::new();
        let seed = body.mvfs(p(0.0)).unwrap();
        let seg = body
            .mev_line(
                MevSite::Lone {
                    r#loop: seed.r#loop,
                },
                p(1.0),
            )
            .unwrap();
        body.mef_chord(MefSite::Chords {
            he1: seg.he_plus,
            he2: seg.he_minus,
        })
        .unwrap();
        let strut = body
            .mev_line(
                MevSite::Fan {
                    he1: seg.he_plus,
                    he2: seg.he_plus,
                },
                p(2.0),
            )
            .unwrap();
        let kill = body.kemr(strut.he_plus, strut.he_minus).unwrap();
        let grow = body
            .mev_line(MevSite::Lone { r#loop: kill.ring }, p(3.0))
            .unwrap();
        body.mef_chord(MefSite::Chords {
            he1: grow.he_plus,
            he2: grow.he_minus,
        })
        .unwrap();
        body.mfkrh_plug(kill.ring).unwrap();
        (body, seed.shell)
    }

    #[test]
    fn tier_two_rejects_the_promoted_detached_ring() {
        let (body, shell) = detached_digon_body();
        // Tier 1 holds — both components are closed genus-0 pieces
        // (χ = 2 each) — and neither of tier 2's other bans has
        // anything to say. Only the connectivity rule fires.
        assert_eq!(validate(&body), Ok(()));
        assert_eq!(
            validate_closed(&body),
            Err(vec![ValidationError::ShellDisconnected {
                shell,
                components: 2,
            }])
        );
    }

    #[test]
    fn tier_two_report_order_is_the_documented_one() {
        // All three tier-2 defects at once: empty loops (loop-arena
        // order), then struts (vertex-arena order), then disconnection
        // (shell-arena order).
        let (mut body, shell) = detached_digon_body();
        // A strut off the pillow rim…
        let anchor = body
            .half_edges()
            .map(|(k, _)| k)
            .next()
            .expect("pillow has half-edges");
        let tail = body
            .mev_line(
                MevSite::Fan {
                    he1: anchor,
                    he2: anchor,
                },
                p(4.0),
            )
            .unwrap();
        // …and a planted empty ring next to it.
        let plant = body
            .mev_line(
                MevSite::Fan {
                    he1: anchor,
                    he2: anchor,
                },
                p(5.0),
            )
            .unwrap();
        let ring2 = body.kemr(plant.he_plus, plant.he_minus).unwrap();
        assert_eq!(validate(&body), Ok(()));
        assert_eq!(
            validate_closed(&body),
            Err(vec![
                ValidationError::ScaffoldingEmptyLoop { loop_: ring2.ring },
                ValidationError::ScaffoldingStrutVertex {
                    vertex: tail.vertex,
                },
                ValidationError::ShellDisconnected {
                    shell,
                    components: 2,
                },
            ])
        );
    }

    // ------------------------------------------------------------------
    // Fuzz wiring: seqgen drives random valid operator sequences, and
    // the validator is asserted IN THE TEST — not via the operators'
    // debug postconditions — so the whole harness is exercised in
    // release builds too.
    // ------------------------------------------------------------------

    proptest! {
        #![proptest_config(ProptestConfig {
            cases: 32,
            ..ProptestConfig::default()
        })]

        #[test]
        fn random_op_sequences_stay_tier1_valid_at_every_step(
            decisions in proptest::collection::vec(
                (any::<u32>(), any::<u32>()),
                1..40,
            )
        ) {
            let mut body = Body::<f64>::new();
            let mut counter = 0_u32;
            for (d1, d2) in decisions {
                let choice = seqgen::choose_op(&body, d1, d2)
                    .expect("an op always applies");
                seqgen::apply(&mut body, choice, &mut counter);
                prop_assert_eq!(validate(&body), Ok(()), "after {:?}", choice);
            }
        }

        #[test]
        fn random_sequences_then_teardown_validate_vacuously(
            decisions in proptest::collection::vec(
                (any::<u32>(), any::<u32>()),
                1..40,
            )
        ) {
            let mut body = Body::<f64>::new();
            let mut counter = 0_u32;
            for (d1, d2) in decisions {
                let choice = seqgen::choose_op(&body, d1, d2)
                    .expect("an op always applies");
                seqgen::apply(&mut body, choice, &mut counter);
            }
            seqgen::teardown(&mut body);
            // The torn-down body is empty; both tiers hold vacuously.
            prop_assert_eq!(validate(&body), Ok(()));
            prop_assert_eq!(validate_closed(&body), Ok(()));
        }
    }

    // ------------------------------------------------------------------
    // Property tests: both fixture families validate cleanly at every
    // size, and clones stay independent (mutating the clone leaves the
    // original clean). Strategies are deterministic (proptest's seeded
    // RNG; no ambient state).
    // ------------------------------------------------------------------

    proptest! {
        #[test]
        fn ngon_pillows_validate_cleanly(n in 1usize..=8) {
            let t = ngon_pillow(n);
            prop_assert_eq!(validate(&t.body), Ok(()));
            prop_assert_eq!(validate_closed(&t.body), Ok(()));
            prop_assert_eq!(t.body.vertices().count(), n);
            prop_assert_eq!(t.body.edges().count(), n);
            prop_assert_eq!(t.body.faces().count(), 2);
            // Clone independence, propertywise.
            let mut cloned = t.body.clone();
            prop_assert_eq!(validate(&cloned), Ok(()));
            cloned.add_point(anchor()); // now malformed (orphan point)
            prop_assert!(validate(&cloned).is_err());
            prop_assert_eq!(validate(&t.body), Ok(()));
        }

        #[test]
        fn prisms_validate_cleanly(n in 2usize..=8) {
            let t = prism(n);
            prop_assert_eq!(validate(&t.body), Ok(()));
            prop_assert_eq!(validate_closed(&t.body), Ok(()));
            prop_assert_eq!(t.body.vertices().count(), 2 * n);
            prop_assert_eq!(t.body.edges().count(), 3 * n);
            prop_assert_eq!(t.body.faces().count(), n + 2);
            prop_assert_eq!(t.body.half_edges().count(), 6 * n);
            // Every vertex has valence 3: its orbit closes over 3
            // half-edges (nontrivial orbits are the point of this
            // family).
            for (_, vertex) in t.body.vertices() {
                let orbit = t.body.vertex_orbit(vertex.emanating.unwrap()).unwrap();
                prop_assert_eq!(orbit.len(), 3);
            }
            // Clone independence.
            let mut cloned = t.body.clone();
            prop_assert_eq!(validate(&cloned), Ok(()));
            cloned.add_point(anchor());
            prop_assert!(validate(&cloned).is_err());
            prop_assert_eq!(validate(&t.body), Ok(()));
        }
    }

    /// S6 (two-tolerance, D4 ¶1 addendum): the census pair —
    /// exactly-touching (`UndeclaredContact`) and in-band
    /// (`CensusEscalated`) — is one user situation, both arms carrying
    /// the shared recourse fragment. Also the regression net for the
    /// S6 `{:?}` bug: `CensusEscalated` used Debug formatting on its
    /// cause, which dropped the recourse sentence entirely.
    #[test]
    fn census_pair_carries_the_shared_recourse() {
        use crate::entity::VertexKey;

        let contact = ValidationError::UndeclaredContact {
            contact: CensusContact::VertexVertex {
                a: VertexKey::default(),
                b: VertexKey::default(),
            },
            witness: "(0e0, 0e0, 0e0)".to_string(),
        };
        let msg = contact.to_string();
        // The CONTACT tier carries the two-arm menu (SELECT-DESIGN
        // §3d, ratified): declare the named class / move the geometry.
        // The three-arm decidability sentence must not leak in — its
        // "lower the tolerance" arm cannot supply missing intent.
        assert_eq!(
            msg.matches(crate::contact::CONTACT_RECOURSE).count(),
            1,
            "{msg}"
        );
        assert!(!msg.contains("lower the tolerance"), "{msg}");

        let escalated = ValidationError::CensusEscalated {
            cause: Indeterminate {
                margin: geom_core::MarginDiag::Value(5e-9),
                band: Band::new(1e-9, 1e-8).unwrap(),
                predicate: Some("pm_census_vv_gap"),
            },
        };
        let msg = escalated.to_string();
        assert_eq!(
            msg.matches(geom_core::COINCIDENCE_RECOURSE).count(),
            1,
            "{msg}"
        );
        // Debug leakage (the `{:?}` bug) would print the enum shape,
        // not the carrier sentence.
        assert!(!msg.contains("MarginDiag"), "{msg}");
        assert!(!msg.contains("Value("), "{msg}");
    }
}
