//! The validation harness: [`validate`] (tier 1), [`validate_closed`]
//! (tier 2), [`validate_geometric`] (tier 3 — M2 PR 3),
//! [`validate_pseudomanifold`] (tier 3′ — tier 3 over a body's
//! resolved contact declarations, the door a boolean result and a STEP
//! import are gated through), and [`ValidationError`].
//!
//! Each certifying tier has a **certificate form** beside it —
//! [`validate_geometric_certificate`], its `_declared` twin, and
//! [`validate_pseudomanifold_certificate`] — which runs the same pass
//! and returns the [`crate::SignCertificate`] check 7 decided on instead
//! of dropping it; [`validate_pseudomanifold_certificate_structural`] is
//! the same form holding no lane, whose check 7 is the closed form's.
//! Every tier-3 door that MAKES check 7 makes it the same way
//! (`plus_v_by_sign`, through the lane it holds —
//! [`validate_geometric_structural`] makes none): one walk per SOLID, check
//! 7's subject, each stopped at the round where that solid's sign is
//! certain, assembled into one body certificate over the face arena —
//! one read of each face, whatever the solid count. A caller that wants
//! the number continues the certificate
//! ([`crate::SignCertificate::refine_to_target`]) and pays only the
//! rounds the check did not. The `()`-returning doors above ARE those
//! calls with the value mapped away.
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
//! **The tier is two functions.** Eight of its nine checks are answerable
//! by any deciding scalar; the ninth — the +V global orientation
//! invariant — reads a volume enclosure, and deciding its sign is an act
//! of certification rather than a measurement. So
//! [`validate_geometric_structural`] runs the eight and
//! [`validate_geometric`] is that call followed by the certified one,
//! carrying the union of their bounds — a scalar without certification
//! rights takes the structural door, and cannot write the composed call
//! at all.
//!
//! **The structural half never judges orientation, at ANY scalar.** That
//! is the consequence to carry away, and it is stronger than "a dual
//! cannot certify": check 7's closed form computes a signed volume at
//! every scalar with a zero pad, so the pre-split door handed a dual a
//! real `+V` verdict on any planar body. The split moves the whole
//! check — both derivations — behind the certified bound, because the
//! sign is decided in exactly one place or the half that is supposed to
//! carry no certification arm grows one back. **An inverted body
//! therefore passes the structural half by design**, and a caller that
//! wants the sign at a non-certifying scalar goes to the `_structural`
//! passes, which make check 7 through the closed form:
//! [`validate_pseudomanifold_structural`], [`contact_marks_structural`],
//! [`crate::mass_properties_structural`].
//!
//! **What check 7 costs, and what it cannot refuse.** Deciding a sign
//! is cheaper than measuring a volume, and the tier pays only the
//! former: the certified quadrature is refined round by round until
//! the body's volume ENCLOSURE excludes zero, and stops there. So a
//! valid solid cannot fail check 7 on quadrature budget while its
//! sign is definite — the refusal that used to arrive when a fitted
//! rational wall could not reach the REPORTING target `1024·ε` is a
//! refusal of the caller who asks for the number, not of the body.
//! Check 7 still refuses `VolumeUncomputable` where the quadrature
//! produces no enclosure at all (an unsupported chart, a poisoned
//! bracket, a degenerate face, an escalated funnel decision) and where
//! the sign is still indefinite when the schedule runs out. The
//! number, when a caller wants it, is
//! [`validate_geometric_certificate`]'s continuation.
//!
//! **What it costs is not "less", and here is the bound.** The check
//! reads EVERY face at every round until the sign settles, because a
//! sum needs every term; the measurement door reads faces in arena
//! order and stops at the first one whose lane refuses. So on a body
//! whose sign settles early the check pays a fraction of the
//! measurement, and on a body with a refusing face it can pay MORE —
//! measured at about twice the measurement's quadrature verdicts on a
//! rational-walled body whose schedule runs out. The honest bound is
//! the schedule's own: at worst every face's whole schedule, which is
//! what the measurement pays for its own first face and no more than
//! it pays for all of them.
//!
//! **Gating and then measuring costs more than measuring**, and that
//! is worth knowing before a caller reaches for the continuation as a
//! saving. The piece evaluations compose exactly — the gate's rounds
//! plus the continuation's are the measurement's — but each entry into
//! a face's lane re-derives that face's per-round-independent SETUP
//! (the derivative grids, the block hulls, the last round's cut lists
//! and the bound taken from them), so the pair runs 1.3–1.8× one
//! measurement's wall time on the bodies measured. Reusing a face's
//! setup across windows is `work/perf/`'s
//! `quadrature-setup-is-re-derived-per-round-window`.
//!
//! **Tier 3′ is not this**, and the difference is visible from
//! outside: [`validate_pseudomanifold`] and [`contact_marks`] run
//! their check 7 through the certified quadrature at the reporting
//! target (their `_structural` twins through the closed form alone),
//! so a body tier 3 admits on a definite sign can still be refused
//! there on quadrature budget.
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

use geom::{NetState, Surface};
use geom_brep::{
    CertifyError, DihedralClass, MaterialPairing, MaterialWedge, classify_dihedral,
    classify_material_pairing,
};
use geom_core::{Band, BandError, Decide, Indeterminate, Margin, Real, Sign, Tol};
use slotmap::{Key, SecondaryMap};

use crate::body::{Body, Walk};
use crate::boolean::ContainError;
use crate::chart_region::ChartRegionError;
use crate::contact::{ContactRefusal, DeclaredContact};
use crate::face_normal::plane_outward_normal;
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

/// **What a census refusal is ABOUT** — the whole of the subject the
/// refusing arm was examining.
///
/// INVARIANT: an arm whose subject is a candidate CONTACT names both
/// faces. The census decides a face PAIR, never a face, so a refusal
/// that named one half of a pair would let a consumer resolve it
/// against a declaration the census never examined, and which half it
/// named would be the arena's ordering rather than the arm's
/// question. A consumer therefore matches the pair, and the answer is
/// the same in either order.
#[derive(Clone, Copy, Debug, Eq)]
pub enum CensusSubject {
    /// One entity, and the arm's subject is that entity alone.
    Entity(EntityId),
    /// The candidate face pair, in the arm's own order. Unordered as
    /// a subject: `(a, b)` and `(b, a)` name one candidate, and
    /// [`PartialEq`] below is what makes that true rather than
    /// aspirational.
    FacePair(FaceKey, FaceKey),
}

// The unordered-pair invariant, WRITTEN rather than derived. A derived
// `PartialEq` is structural, so `FacePair(a, b) != FacePair(b, a)` —
// which is the arena's ordering deciding an equality question the
// subject does not have, exactly the defect the variant exists to
// close. The order is kept in the value (the arm's own, and what
// `Debug` prints) and dropped from the comparison; every equivalence
// law holds, since unordered-pair equality is the quotient of a
// structural one.
impl PartialEq for CensusSubject {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Entity(x), Self::Entity(y)) => x == y,
            (Self::FacePair(a, b), Self::FacePair(c, d)) => (a, b) == (c, d) || (a, b) == (d, c),
            _ => false,
        }
    }
}

// One register, as the message had before the subject was widened, and
// as `CensusUndecidable`'s pair message already reads: every entity
// through [`EntityId`]'s own `Display`, never a `Debug` key beside a
// rendered one.
impl core::fmt::Display for CensusSubject {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Entity(e) => write!(f, "{e}"),
            Self::FacePair(a, b) => {
                write!(
                    f,
                    "the pair {} / {}",
                    EntityId::Face(*a),
                    EntityId::Face(*b)
                )
            }
        }
    }
}

/// **WHY a census decline declined** — the refusing lane's own typed
/// refusal, carried instead of discarded.
///
/// [`ValidationError::CensusUnsupported`] is raised by four
/// different lanes, and until this was carried they all arrived at a
/// consumer as one sentence about an uncertifiable inventory. That
/// sentence is not always the true cause: a chart-region
/// [`WitnessBudgetExhausted`](ChartRegionError::WitnessBudgetExhausted)
/// decline means the interior-witness SEARCH STOPPED on a pair whose
/// overlap may be fat and perfectly decidable, and its recourse is to
/// simplify the trims — not to declare the geometry or separate it.
/// A refusal whose stated cause is not its real one sends the reader
/// to the wrong repair.
///
/// **This is a cause, not a classification.** It does not partition
/// the refusals into "the geometry cannot be decided" and "the
/// schedule stopped", because that partition is not clean:
/// [`MissingCache`](ChartRegionError::MissingCache) is a fact about
/// the BODY (re-mint its pcurves),
/// [`Corrupt`](ChartRegionError::Corrupt) is a kernel-invariant
/// violation, and [`RayExhausted`](ChartRegionError::RayExhausted) is
/// named for exhaustion and is not one.
///
/// **That last reading is the load-bearing one, so it is argued from
/// the BAND and not from the schedule's length.** It would prove
/// nothing to say the ray schedule is fixed while the witness budget
/// is a cap: a seventeenth direction is available in exactly the
/// sense a larger budget is. What separates them is what the spent
/// work MEASURED. `RayExhausted` fires when every direction tried
/// returned an IN-BAND margin, and an in-band margin is a verdict
/// about where the point sits relative to the boundary — within ε of
/// it — not about the direction that read it. Another direction reads
/// the same configuration and lands in the same band; only moving the
/// point or tightening ε changes the answer.
/// [`WitnessBudgetExhausted`](ChartRegionError::WitnessBudgetExhausted)
/// is the opposite: its cap stops the arrangement being BUILT, so
/// nothing was measured at all, and the work it declined to do would
/// have returned a definite answer on a fat overlap. One says the
/// geometry is undecidable here; the other says nobody looked.
///
/// Carrying the arm itself says all of that and pre-judges none of
/// it.
///
/// **NOT carried to the façade's curated list, and that is a
/// decision rather than an omission** (`scripts/payload-rung-sweep.py`
/// names this rung; the disposition table cites this paragraph as its
/// home). A Rust caller can already name and match this type —
/// `pncad` re-exports `topo` whole — so what the prelude list would
/// add is two things it does not yet have: the CUR3 property row
/// `carried_refusal_payloads_are_matchable_through_the_prelude`
/// extended to cover a new published payload, and a Python word
/// beside `subject_kind` so the binding's callers branch on the cause
/// instead of reading it out of a sentence. Both are the façade
/// crate's to write, and publishing the name without them is what a
/// review refused: it would add two subjects to the rule that makes
/// prelude payloads matchable and no row to check them.
///
/// **The falsifier is a Python caller who must tell a stopped search
/// from a thin overlap.** Today that caller gets the whole of the
/// lane's sentence on the message and no word to match — strictly
/// more than it had, and less than a Rust caller gets. When the
/// façade carries the cause, delete this paragraph and the
/// disposition row with it.
///
/// It carries no bearing on ATTRIBUTION:
/// `editor_core::assembly::attribute` reads which variant refused and
/// what its [`CensusSubject`] was, and a decline is the census
/// neither certifying nor contradicting a declaration whichever lane
/// declined. So this widens what the refusal SAYS and moves no
/// `AssemblyError` verdict.
#[derive(Clone, Debug, PartialEq)]
// The variant roster the sample-coverage row reads (test builds only).
#[cfg_attr(
    test,
    derive(strum::EnumDiscriminants),
    strum_discriminants(
        name(CensusUnsupportedCauseKind),
        vis(pub(crate)),
        derive(strum::EnumIter)
    )
)]
pub enum CensusUnsupportedCause {
    /// The chart-region overlap lane refused typed: its own arm,
    /// whole, with the quantities it metred.
    ///
    /// [`ChartRegionError::Escalated`] does not reach here: both
    /// census matches route an escalation to
    /// [`ValidationError::CensusEscalated`], a different refusal with
    /// a different recourse. The type admits it anyway, because
    /// excluding it costs a second chart-region enum whose only
    /// content is that routing — a type to carry one fact the
    /// matches already carry.
    ChartRegion(ChartRegionError),
    /// The contact-pair certifier refused typed, whole.
    ///
    /// The refusal is carried rather than reduced to its `what`, for
    /// the reason the chart arm is carried rather than reduced: an
    /// enum that exists to stop refusals being flattened must not
    /// itself be where one lane's refusal is flattened. Its `Display`
    /// is what renders here, so `contact.rs`'s composition holds —
    /// including the part that matters most,
    /// [`ContactRefusal::NotCertifiable`] carrying NO recourse on
    /// purpose: a declaration cannot move a configuration inside the
    /// certifiable set, so the declare-or-separate menu is a false
    /// lead there and the `what` is the only honest steering.
    ///
    /// Only `NotCertifiable` reaches this arm today — the census
    /// sends `Contradicted` to
    /// [`ValidationError::ContactContradicted`] and the other two to
    /// [`ValidationError::CensusEscalated`] — and the type is not
    /// narrowed to say so, for the same reason [`Self::ChartRegion`]
    /// admits `Escalated`.
    ContactLane(ContactRefusal),
    /// The census's face-bounding sweep could not bound the face at
    /// all: it has no boundary vertex, because its outer loop is
    /// empty or its boundary does not resolve. Nothing about the
    /// face's CARRIER refused here, so the inventory sentence the
    /// other two arms compose would name the wrong thing entirely.
    FaceUnboundable,
    /// The point-in-face door refused typed: its own arm, whole.
    ///
    /// **Carried because the alternative was a FABRICATED margin.**
    /// The census asks [`contfp`](crate::boolean::contfp) whether a
    /// vertex, an edge midpoint or a crossing point lies inside a
    /// planar face. Three of that door's arms carry no measured
    /// quantity at all — an arc-bearing loop no walk expresses, an
    /// exhausted parity schedule, unwalkable topology — and the census
    /// used to answer all three with
    /// [`ValidationError::CensusEscalated`] over an
    /// [`Indeterminate`] it MINTED: predicate `pm_census_containment`,
    /// margin [`MarginDiag::Invalid`](geom_core::MarginDiag::Invalid).
    /// That reads as "a named predicate was posed and came back
    /// poisoned", which is a claim about a measurement that never
    /// happened, in the one field a reader uses to judge how close the
    /// call was. No predicate of that name decides anything (the
    /// dimension audit lists it among the names that never reach the
    /// funnel); it was a tag standing in for a cause.
    ///
    /// [`ContainError::Escalated`] does not reach here, for the reason
    /// [`Self::ChartRegion`] gives: that arm carries a margin a
    /// predicate really metred, so it routes to
    /// [`ValidationError::CensusEscalated`] with its own diagnostic
    /// and nothing is invented. The type admits it anyway rather than
    /// buying a second containment enum to say so.
    ///
    /// **The three are not one class and are not made one here.** An
    /// unexpressible arc loop is an inventory fact about the MODEL, an
    /// exhausted schedule is a verdict that the point sits within ε of
    /// the boundary, and unwalkable topology is a kernel-invariant
    /// violation; they want three different repairs and each says so
    /// in its own `Display`. What is true of all three — and false of
    /// `Escalated` — is only that nothing metred a margin, which is
    /// exactly why none of them may carry one.
    Containment(ContainError),
}

// Each arm renders the refusing LANE's own sentence, through
// `Display` and never `Debug` — the S6 bug one variant over was a
// `{:?}` that dropped a carrier's recourse entirely.
//
// The carrier owns the recourse, and two of the three say so
// differently. `ChartRegionError` claims every arm names one, and
// `chart_region.rs`'s `every_chart_region_arm_names_a_recourse` is
// what makes that claim hold rather than aspire — eight arms had none
// until this variant stopped the census papering over it.
// `ContactRefusal::NotCertifiable` carries none ON PURPOSE, ratified
// in `contact.rs`: a declaration cannot move a configuration inside
// the certifiable set, so the declare-or-separate menu is a false
// lead and `what` is the only honest steering. The arm that used to
// append that menu to every decline appended it there too.
impl core::fmt::Display for CensusUnsupportedCause {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::ChartRegion(e) => write!(f, "{e}"),
            Self::ContactLane(refusal) => write!(f, "{refusal}"),
            Self::Containment(e) => write!(f, "{e}"),
            Self::FaceUnboundable => write!(
                f,
                "the face has no boundary vertex (an empty outer loop, or a boundary \
                 reference that does not resolve), so its extent could not be read; \
                 repair the face's loop, then check again"
            ),
        }
    }
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
    ///
    /// **Not [`geom_brep::MaterialWedge::Seam`]**, which sits four
    /// lines away in the same check and means something else entirely:
    /// that one is the material VERDICT wedge π (the two faces
    /// continue one another), while this one is a statement about the
    /// edge's DESCRIPTION (a chart seam), and an exemption rather than
    /// a verdict. Both names are load-bearing in their own
    /// vocabularies — the ruling calls wedge π the seam, and chart
    /// seams have been `Seam` since M2 — so neither is renamed; the
    /// hazard is called out instead, here and there.
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
/// arms already handle.
///
/// A site that EXTRACTS carries no such obligation — a
/// `find_map`/`filter_map` picking one variant out and answering
/// `None` to the rest is asking a question, not giving an answer, and
/// its `_` arm IS the question.
///
/// (`Eq` dropped at M2 PR 3: the tier-3 variants carry margin
/// diagnostics with `f64` payloads.)
#[derive(Clone, Debug, PartialEq)]
// The companion fieldless enum is the compiler's own statement of this
// enum's variants and their order: the Display-coverage row indexes by
// it and sizes its array from its `COUNT`, so neither the count nor the
// order is written down twice. Test builds only — nothing in the
// production surface names it.
#[cfg_attr(test, derive(strum::EnumDiscriminants))]
#[cfg_attr(
    test,
    strum_discriminants(
        name(ValidationErrorKind),
        vis(pub(crate)),
        derive(strum::EnumCount, strum::EnumIter)
    )
)]
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
    /// Tier 3: a face's surface is a `Nurbs` net in
    /// [`geom::NetState::Placeholder`] at rest, which must be replaced
    /// via `Body::set_face_surface` before rest.
    UncertifiableSurface {
        /// The face whose surface is the placeholder.
        face: FaceKey,
    },
    /// Tier 3: a face's surface is a `Nurbs` net in
    /// [`geom::NetState::Poisoned`] — a description that claims a locus
    /// and cannot evaluate one.
    ///
    /// A different STATE from [`ValidationError::UncertifiableSurface`],
    /// not a different symptom; `NetState`'s docs are where the three
    /// states are defined and this pair of variants is check 1's
    /// answer to two of them.
    PoisonedSurfaceDescription {
        /// The face whose surface net carries the poison.
        face: FaceKey,
    },
    /// Tier 3: a face's **approximating** surface failed to
    /// re-certify against its own stored description at rest.
    ///
    /// The stored certificate is never read (O5's never-trust
    /// posture): the two-limb bound is re-derived per validation call
    /// from the description and the fit, and classified against the
    /// **run's** ε_precision rather than the tolerance the surface
    /// carries (O3's ratified claim is what tier 3 verifies; the stored
    /// tolerance is the mint's parameter). A fit that has drifted from
    /// what it claims to approximate — coarsened, edited, grafted onto
    /// another base — reports here, naming the limb that caught it, and
    /// so does one minted looser than the ε this run demands.
    ApproxCertification {
        /// The face whose approximating surface failed.
        face: FaceKey,
        /// The fit door's typed refusal.
        error: geom_brep::OffsetFitError,
    },
    /// Tier 3: a face carries an approximating surface and the pass
    /// was handed no re-derivation door.
    ///
    /// Where `Some` comes from, and what its absence means:
    /// [`crate::AtRestPolicy::offset_fit_lane`].
    ///
    /// Reported rather than skipped: a surface certificate is the one
    /// claim this kernel refuses to leave unchecked, and passing a
    /// face whose certificate nothing re-derived would make it exactly
    /// that.
    ApproxLaneUnsupported {
        /// The face whose approximating surface has no lane.
        face: FaceKey,
    },
    /// Tier 3: a face's torus violates D3's ring convention `R > r > 0`
    /// — a horn (`R == r`) or spindle (`R < r`) torus, whose axis
    /// carries a singular point the chart machinery has no
    /// representation for (`geom::Surface::Torus`'s own contract: the
    /// chart normal derivation assumes `R + r·cos v > 0`, and
    /// `crate::chart::Chart::poles` is empty for a torus because a ring
    /// torus has none).
    ///
    /// This is the net every door that can mint a torus passes through,
    /// and there are THREE of them rather than the two this comment used
    /// to name:
    ///
    /// * `sweep::revolve` refuses the configuration at construction;
    /// * `step-import` reads `TOROIDAL_SURFACE`'s two radii verbatim,
    ///   so it can carry one in;
    /// * the BLEND lane (`sweep::blend`, reachable through the public
    ///   `fillet_edges`) mints `Surface::Torus` from a spine radius `s`
    ///   and the blend radius `r`. Its own arms document predicate 3
    ///   (`SpineIrregular`) as the refusal for `0 < s ≤ r` — the spindle
    ///   and horn configurations — but that is the blend lane's claim
    ///   about itself, not something measured here, and the surgery
    ///   arms mint tori too.
    ///
    /// Which is why this check is at REST and not at any one mint: it is
    /// the net under all three, whatever each of them believes about
    /// itself.
    DegenerateTorus {
        /// The face whose torus is a horn or spindle.
        face: FaceKey,
    },
    /// Tier 3: the ring-torus convention margin `R − r` landed in the
    /// ambiguity band (or was poison) — the classification is not
    /// available at this tolerance.
    DegenerateTorusEscalated {
        /// The face whose torus could not be classified.
        face: FaceKey,
        /// The predicate-layer escalation.
        cause: Indeterminate,
    },
    /// Tier 3: a face's analytic surface stores a datum that is not a
    /// finite number, or a plane whose `normal` is the zero vector — a
    /// description that claims a locus and cannot evaluate one.
    ///
    /// **The analytic kinds' answer to `geom`'s totality-and-poison
    /// rule, and not [`ValidationError::PoisonedSurfaceDescription`]**:
    /// that variant is check 1's answer to a NET's state
    /// ([`geom::NetState::Poisoned`], paired with the placeholder's
    /// [`ValidationError::UncertifiableSurface`]) and carries only the
    /// face, because a net's poison sits in no one named field. An
    /// analytic surface's datums are named fields, so this refusal
    /// names the one that failed, and the kind that carries it.
    ///
    /// `+∞` is refused here with NaN: a datum is a number the variant's
    /// formula reads, and an infinite radius or origin is no more a
    /// locus than a NaN one.
    PoisonedSurfaceDatum {
        /// The face whose surface stores the datum.
        face: FaceKey,
        /// The surface kind.
        kind: geom_brep::SurfaceKind,
        /// The datum that describes no locus.
        datum: geom::SurfaceDatum,
    },
    /// Tier 3: a face's analytic surface stores a finite datum outside
    /// its variant's convention — a cylinder or sphere radius, or a
    /// torus tube radius, that is not definitely positive, or a cone
    /// half-angle not definitely inside `(0, π/2)`
    /// ([`geom::Surface::representability_margins`], the one place in
    /// code the at-rest bounds are computed; that door says where else
    /// they are stated).
    ///
    /// **Refused on representability, the torus ring convention's
    /// reason**: such a datum describes no 2-manifold a face can bound
    /// — a radius of zero is a line or a point, not a small cylinder or
    /// sphere; a half-angle of `0` or `π/2` is a line or a plane. That
    /// `geom` evaluates it as "well-defined garbage" is `geom`'s
    /// contract with its callers, not a statement that the face is
    /// valid.
    ///
    /// **Not a metered predicate, deliberately** (the chamfer's
    /// `NonpositiveSize` precedent): whether a stored datum lies inside
    /// its convention is a fact about the DATUM, not a geometric
    /// quantity of the body, so it takes no `k_stats` name and no band.
    /// A zero-straddling enclosure fails it with the rest.
    UnrepresentableSurfaceDatum {
        /// The face whose surface stores the datum.
        face: FaceKey,
        /// The surface kind.
        kind: geom_brep::SurfaceKind,
        /// The datum outside its convention.
        datum: geom::SurfaceDatum,
        /// Which end of the convention it fails: `Lower` for a radius
        /// that is not positive or a cone closed to a line, `Upper` for
        /// a cone opened to a plane.
        end: geom::ConventionEnd,
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
    /// with teeth (D2; ratified with Ev 2026-07-19, M2 PR 4 fix
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
        /// The definitely-transverse edge whose locus the modeler
        /// declared.
        edge: EdgeKey,
    },
    /// Tier 3, **the transience fence** (U2's Q2 as corrected by Ev
    /// 2026-08-27): a body at rest carries an edge still described by
    /// the SCAFFOLDING door — a sketch pushforward standing in for a
    /// description while the edge's surfaces do not exist yet.
    ///
    /// The door is legal and load-bearing: an Euler-op ring's null
    /// edges and a sweep's struts are certified before any surface
    /// they could be charted in exists. What makes it legal is that
    /// they are TRANSIENT. An edge that reaches a valid body has two
    /// faces, so it has a chart, so it can say where its locus lies —
    /// and a scaffold at rest says instead that a construction stopped
    /// half-way and nobody noticed. The fence is transience, not
    /// "pre-body": `MappedCurve` measurably reached rest through the
    /// boolean join's re-description lanes and the fillet's struts,
    /// which is exactly what "pre-body" failed to catch.
    ScaffoldAtRest {
        /// The edge still carrying a scaffolding description.
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
    /// Tier 3 (check 4, material arm): the edge's two faces subtend a
    /// material wedge of **0** (a cusp) or **2π** (a knife slit) and
    /// nothing DECLARED the tangency.
    ///
    /// D1's ratified second-order arm (the #131 ruling): the two ends
    /// of the wedge range are legal only where the C7 `Tangent`
    /// contact vocabulary asserts the contact — **never** inferred
    /// from the values, per the coincidence ladder. Discovery is not
    /// declaration, so an undeclared cusp refuses at every ε: the
    /// verdict does not consult the second-order margin at all, which
    /// is also why no ε-tightening can turn this refusal into a
    /// different one.
    ///
    /// The recourse is [`crate::contact::CONTACT_RECOURSE`]'s two
    /// arms — declare the contact, or move the geometry — and never
    /// the tolerance lever: a missing intent is not a decidability
    /// question.
    UndeclaredCusp {
        /// The edge whose material wedge is 0 or 2π.
        edge: EdgeKey,
        /// Which end — [`MaterialWedge::Cusp`] or
        /// [`MaterialWedge::Slit`]. The two are one another's `revert`
        /// images and are legal together or not at all, so they refuse
        /// through one variant carrying which it saw.
        wedge: MaterialWedge,
    },
    /// Tier 3 (check 4, material arm): the edge's two faces have
    /// opposed material sides along a shared tangent plane — the
    /// wedge-0/2π configuration — and their second-order jets
    /// **osculate**: κ_rel definitely collapsed.
    ///
    /// This is conformal contact along the locus, which fails the
    /// declared arm's curve-locus condition: the surfaces do not
    /// determine which end of the wedge range this is — there is no
    /// crescent, only a zero-thickness sheet — and **no declaration
    /// cures it**, because there is no certifiable curve-locus
    /// tangency to declare.
    ///
    /// **Two different defects wear this local signature**, and the
    /// message says so rather than picking one:
    ///
    /// - a genuine **lamina** — a zero-volume sheet (the flipped
    ///   rimless ball measures exactly 0.0). Recourse: move the
    ///   geometry.
    /// - an **orientation defect** in a body of real volume: two
    ///   faces lying on ONE surface with one face's `sense` inverted
    ///   read as opposed material sides with osculating jets, because
    ///   osculation is what two faces of the same surface DO. The
    ///   flipped conic-trim `cut_cylinder` is this shape, and it
    ///   measures 3.9269908167918763, not zero. Recourse: fix the
    ///   sense.
    ///
    /// The check is edge-LOCAL, so it cannot tell the two apart — both
    /// are genuinely refused, and the diagnosis belongs to whoever
    /// reads the faces. Claiming "zero-volume" of both would be a
    /// false diagnosis attached to a true catch.
    ///
    /// It shares no edge with [`Self::UndeclaredCusp`]: a collapsed
    /// κ_rel yields no wedge end, so the two refusals are mutually
    /// exclusive by construction (`MaterialArmOutcome`), not by a
    /// precedence between them.
    ///
    /// In-band κ_rel is NOT this: an osculation the run cannot decide
    /// escalates as [`Self::SliverDihedral`] with the
    /// `tangent_second_order` cause, so the three outcomes of the
    /// second-order band are three different answers.
    LaminaWedge {
        /// The edge whose opposed faces osculate.
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
    /// is the chart normal with `Face::sense` folded in, so the comparison is
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
    /// tighter ε). Examined on loops of `Line` and `Circle` carriers,
    /// whose winding is exact (chord polygon plus each arc's circular
    /// segment); a loop riding an `Ellipse`, spiric or NURBS carrier is
    /// not examined.
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
    /// Tier 3: `solid`'s exact-B-rep signed volume is **definitely
    /// negative** — global orientation corruption (the +V invariant,
    /// M2 PR 7). The margin is `V / A` over that solid's own faces (a
    /// length: the mean boundary displacement of the volume defect),
    /// classified against the run's linear band; `Zero` and escalated
    /// margins are exempt (escalation never flips valid → invalid).
    ///
    /// **The subject is the SOLID, and a body's total is not a
    /// substitute for it.** A total is a sum, and a sum hides a sign: a
    /// reverted part beside a larger ordinary solid totals positive
    /// while the part is inside-out, and the total says nothing about
    /// either. One refusal per solid, each naming its own.
    NegativeVolume {
        /// The solid whose oriented boundary encloses negative volume.
        solid: SolidKey,
    },
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
    /// property of this site but the gate in front of check 7, and that
    /// gate now has two spellings. At [`validate_geometric`] it is the
    /// `?` between the pass's two halves, so the volume is read only
    /// after the WHOLE structural battery came back clean — checks 8
    /// and 9 included, which the older spelling did not cover. In
    /// [`validate_pseudomanifold`] and [`contact_marks`], which run the
    /// battery in one call, it is still the `if errors.is_empty()`
    /// standing between checks 6 and 7. Neither is an invariant this
    /// variant enforces: a corrupt body has normally already been
    /// refused by the check that names its corruption, "normally" is
    /// the honest word, and [`crate::mass_properties`] called directly
    /// has no guard at all.
    VolumeUncomputable {
        /// The solid whose volume the +V invariant was reading. Check
        /// 7's subject is the solid, so its refusal names one too.
        solid: SolidKey,
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
    /// **Tier 3, check 9.** A face's RING meets its own OUTER loop: it
    /// shares a vertex position with it, one of its edges runs along
    /// one of the outer loop's, or the two cross or touch at a point
    /// ([`RingContact`] names which). A ring states "the region this
    /// face trims has a hole strictly inside it"; a ring that touches
    /// the outer boundary states no such region, and every consumer
    /// that reads the trim — the CDT above all — is entitled to refuse
    /// it. The two loops are compared by POSITION, not by key: the
    /// shapes this catches are minted by surgeries that copy a
    /// boundary, so the copy's vertices and edges are fresh keys
    /// standing on the original's geometry.
    RingMeetsOuter {
        /// The face whose two loops meet.
        face: FaceKey,
        /// The ring.
        ring: LoopKey,
        /// What the contact is, and which entities carry it.
        contact: RingContact,
    },
    /// **Tier 3, check 9 — escalated.** A ring-vs-outer contact margin
    /// landed in the ambiguity band, so whether the two loops meet
    /// cannot be certified either way. Reported rather than rounded to
    /// "disjoint": escalate-never-guess (D4 paragraph 3), and rounding
    /// it toward blessing was exactly the direction that let the
    /// class this check exists for ship once already.
    RingContactEscalated {
        /// The face whose loops could not be separated.
        face: FaceKey,
        /// The ring.
        ring: LoopKey,
        /// The predicate-layer escalation.
        source: Indeterminate,
    },
    /// **Tier 3, check 9 — the NESTING statement.** A face's ring
    /// does not lie inside that face's own outer loop: a named vertex
    /// of the ring is definitely OUTSIDE the region the outer loop
    /// bounds. The contact arms above decide whether the two loops
    /// MEET and say nothing about which one is inside the other, so a
    /// ring drawn around its own face's boundary is disjoint from them
    /// and passes; no other tier-3 check sees it either, which
    /// `check_9_refuses_a_ring_that_lies_outside_its_outer_loop`
    /// asserts of checks 6 and 7 on the body itself.
    RingOutsideOuter {
        /// The face whose ring is not inside its outer loop.
        face: FaceKey,
        /// The ring.
        ring: LoopKey,
        /// The ring vertex the arm's instrument placed outside — the
        /// ray-parity walk for a polygonal outer loop, the radial
        /// decide for a one-circle one — the witness, so the report
        /// names a position and not just a verdict. It is the FIRST
        /// vertex in the ring's cycle order that read outside, not a
        /// distinguished one: the arm stops at the first definite
        /// verdict, so a surgery that re-anchors
        /// or reverses the same cycle — `Body::revert` among them —
        /// can move the witness without moving the finding.
        ring_vertex: VertexKey,
    },
    /// **Tier 3, check 9 — nesting undecided.** Whether a ring lies
    /// inside its face's outer loop could not be certified: no vertex
    /// of the ring was placed either way, and at least one query
    /// escalated (either instrument's margin), exhausted the
    /// ray-parity walk's schedule, or met topology that walk could not
    /// read. That last clause is what separates this
    /// from silence — a ring every one of whose queries came back
    /// `OnBoundary` escalated nothing and is the contact arms'
    /// business, not this one's. Reported rather than rounded to
    /// "nested", the same direction [`Self::RingContactEscalated`] is
    /// reported in and for the same reason (D4 paragraph 3).
    RingNestingUndecided {
        /// The face whose ring could not be placed.
        face: FaceKey,
        /// The ring.
        ring: LoopKey,
        /// What the containment walk stopped on.
        source: ContainError,
    },
    /// Tier 3′ (M3 PR 6a): the global coincidence census found a
    /// position coincidence between distinct entities that no declared
    /// contact record backs (directly, or via the D3 segment
    /// reconstruction) — an **undeclared contact** is a hard error,
    /// never blessed (F1/F2(iii): discovery ≠ declaration, ever).
    UndeclaredContact {
        /// The coinciding entity pair, typed by census kind.
        contact: CensusContact,
        /// Where the contact was witnessed, read after "at" in the
        /// message: a coordinate triple, or — where the arm's evidence
        /// is a region verdict with no point (the conformal-patch
        /// arm) — a phrase naming that region. Never the subject
        /// again: `contact` carries it. A site with more to say (the
        /// edge-edge crossing's side verdict) appends it after " — ";
        /// the message renders the part before, and `Debug` the whole.
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
        /// Where the contradiction holds, as a place read after
        /// "contradicted" in the message, preposition included: along
        /// the witnessing edge (the curve-record confirm pass), or
        /// across both faces (the patch-record confirm pass's Door 1,
        /// whose verdict compares whole surfaces). Never the pair
        /// again, and never a key: `declaration` carries both.
        witness: String,
        /// The margin that decided, and its predicate — carried for a
        /// caller and rendered by `Debug`, never by the message. The
        /// predicate names which check refused, but not in a way a
        /// sentence can state truthfully for every site (the same
        /// carrier predicate reports carriers apart at one site and a
        /// coincidence at another), so the message states what is true
        /// at all of them ([`crate::contact::CONTRADICTION_REASON`]).
        margin: Indeterminate,
        /// Extra recourse steering when the counter-evidence has a
        /// named remedy (AQ6's designed-clearance arm).
        steer: Option<&'static str>,
    },
    /// Tier 3′: a census predicate escalated (sliver band or poison) —
    /// indeterminate coincidence geometry at rest is a defect (the
    /// standard trilean discipline: typed escalation, never a silent
    /// skip). The census examines only pairs whose padded boxes
    /// overlap (`census::Trees`), so a predicate about the CARRIERS
    /// of two entities the boxes prove apart — a marginal angle
    /// between far edges, a vertex in band of a line past the edge's
    /// end — is not asked and does not escalate: that pair is decided
    /// apart by the box answer, definitely, not skipped.
    CensusEscalated {
        /// The classifier's diagnostic.
        cause: Indeterminate,
    },
    /// Tier 3′: an entity or record the census did not certify,
    /// because a certifying lane refused TYPED (M9-2 — the blanket
    /// exact-on-planar refusal retired with the census arms that
    /// replaced it). Refused loudly rather than sampled.
    ///
    /// **The inventory is the commonest cause, not the only one.**
    /// Most of these are a carrier or a trim outside what a certifier
    /// admits — no exact-constant-arm chart, seam-branch divergence,
    /// non-planar trims, a carrier kind outside the Rest ladder — and
    /// for those the recourse is the geometry or the declaration. But
    /// the same arm carries a stopped interior-witness search and an
    /// absent pcurve cache, whose recourses are simpler trims and a
    /// re-mint. [`CensusUnsupportedCause`] is which one it was, and
    /// the `Display` reads it rather than asserting the inventory of
    /// every finding.
    CensusUnsupported {
        /// The unsupported subject, whole: an entity for the arms
        /// whose subject is one entity, the face PAIR for the arms
        /// that examine a candidate contact.
        subject: CensusSubject,
        /// WHY the lane declined — its own typed refusal, carried.
        /// Four lanes raise this error and they decline for
        /// unrelated reasons with unrelated recourses; without this
        /// they all reached a consumer as one sentence about an
        /// uncertifiable inventory, which is the true cause of some
        /// of them and not of the rest.
        cause: CensusUnsupportedCause,
    },
    /// Tier 3′: the DOOR the census ran through holds no certified
    /// chart-overlap lane, so a census arm that needs one could not
    /// examine this face pair — a fact about the door, not about the
    /// geometry. Two arms raise it: the conformal face-pair sweep on an
    /// undeclared candidate, and the declared-record confirm pass
    /// (`confirm_curve_and_patch_records`' Door 2) on a declared patch.
    ///
    /// The `_structural` doors ([`validate_pseudomanifold_structural`],
    /// [`validate_pseudomanifold_certificate_structural`]) hand the
    /// census no region lane at ANY scalar, `f64` included — H5 ruling
    /// 3's letter — so this is what they answer wherever a pair needs
    /// one. The recourse is the certified door family
    /// ([`validate_pseudomanifold`] and its siblings) at a certifying
    /// scalar — `f64`, the telemetry probe or the interval scalar —
    /// where the same pair is examined.
    ///
    /// Distinct from [`ValidationError::CensusUnsupported`] on
    /// purpose, and the distinction is the recourse: that one says a
    /// certifying lane LOOKED at this record or candidate and refused
    /// typed, and carries which lane and why; this one says no lane
    /// looked, and the certified door would. No lane refused here,
    /// which is why this arm carries no [`CensusUnsupportedCause`] —
    /// there is none to carry — and why the finding is about the door
    /// rather than about the pair's geometry.
    CensusLaneUnsupported {
        /// The candidate the arm could not examine — the face PAIR,
        /// carried whole for the same reason
        /// [`ValidationError::CensusUnsupported`] carries it.
        subject: CensusSubject,
    },
    /// Tier 3′ (M9-2 union fix, the conservative loudness backstop):
    /// a cross-solid candidate pair the census can neither examine
    /// nor definitely clear — refused loudly as UNDECIDABLE instead
    /// of silently not looked at (A5's letter: decide or refuse,
    /// never silently not-examine). Two arms fire it: arm 1, on a
    /// cross-solid curved face pair within reach of each other (the
    /// C9-ring conformal-rest/proximity class — the exclusion ring
    /// is the certified excluder this backstop stands in for); and
    /// arm 2, the instance-containment arm, on what its MATERIAL test
    /// could not answer. Arm 2's box test is the gate, not the
    /// verdict: a pair no extent margin definitely separates goes to
    /// the material test (the contained instance's vertices against
    /// the container's material through the per-solid point-in-solid
    /// door), and this variant carries only its residue — a standing
    /// crossing or unexamined finding on the pair, a witness the door
    /// refused, a container whose extent no sound box claims, or an
    /// instance whose every vertex lies on the container's boundary.
    /// A decided interference is
    /// [`ValidationError::InstanceInterference`], never this.
    CensusUndecidable {
        /// One side of the pair the census cannot clear.
        a: EntityId,
        /// The other side.
        b: EntityId,
        /// The not-yet-supported class, named.
        what: &'static str,
    },
    /// Tier 3′ (the census's instance-containment arm): one instance's
    /// material contains a vertex of another's — an interference fit,
    /// DECIDED by the material test and not undecidable. A vertex
    /// strictly inside another instance's material is an overlap of
    /// the two materials by itself (`census.rs` arm 2 states what the
    /// arm probes and why). Recorded gate-skips — the declaration that
    /// would admit a deliberate interference — do not exist, so no
    /// record can answer for this finding.
    InstanceInterference {
        /// The instance whose material holds the witness. Nothing about
        /// size or nesting is implied: the arm probes both instances'
        /// vertices against the other's material, and a container's
        /// own vertex inside the part it surrounds is reported with the
        /// part as `outer`.
        outer: SolidKey,
        /// The instance that owns the witness.
        inner: SolidKey,
        /// `inner`'s first vertex in arena order found strictly inside
        /// `outer`'s material.
        witness: VertexKey,
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
// The variant roster the sample-coverage row reads (test builds only).
#[cfg_attr(
    test,
    derive(strum::EnumDiscriminants),
    strum_discriminants(name(CensusContactKind), vis(pub(crate)), derive(strum::EnumIter))
)]
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
    /// A vertex on an edge's interior. No VERTEX-granularity record
    /// names it — the boolean lane refines every such contact into v-v
    /// records before records are emitted — so at rest it is
    /// certifiable through the face rung alone: a declared face pair
    /// holding the vertex on one boundary and the edge on the other
    /// (census module docs, D4). Unbacked, it is a defect.
    VertexOnEdge {
        /// The resting vertex.
        vertex: VertexKey,
        /// The edge.
        edge: EdgeKey,
    },
    /// An edge piercing a face transversally at both interiors — a
    /// transverse dive, interpenetration at rest: categorically
    /// undeclarable until the C6 interference-fit era's recorded
    /// gate-skips exist (the MATE-4b staging defers this class to
    /// that era by name).
    EdgeFacePierce {
        /// The piercing edge.
        edge: EdgeKey,
        /// The pierced face.
        face: FaceKey,
    },
    /// Two edges crossing at both interiors (coplanar or skew-with-
    /// contact). Backable at the census's unified strength when the
    /// crossing lies in a declared pair's verified overlap region
    /// with material on opposite sides of the shared carrier (an
    /// overhanging seat — `census.rs`'s crossing rung); otherwise a
    /// hard finding, the refusal naming the side verdict where a
    /// region-holding pair answered one.
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

/// Prose, not `Debug` guts: this payload is quoted verbatim into
/// [`ValidationError::UndeclaredContact`]'s message, which every façade
/// pastes as the refusal a user reads. The arena keys stay `{:?}` —
/// tuple-shaped, and the key IS the name a caller resolves — while the
/// enum's own struct-variant braces do not reach the message.
impl fmt::Display for CensusContact {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // In words, without the keys: the typed fields carry them.
        match self {
            Self::VertexVertex { .. } => f.write_str("two vertices at one position"),
            Self::VertexOnFace { .. } => f.write_str("a vertex lying on a face"),
            Self::VertexOnEdge { .. } => f.write_str("a vertex lying on an edge"),
            Self::EdgeFacePierce { .. } => f.write_str("an edge passing through a face"),
            Self::EdgeEdgeCross { .. } => f.write_str("two edges crossing"),
            Self::EdgeEdgeOverlap { .. } => f.write_str("two edges overlapping along a length"),
            Self::EdgeFaceOverlap { .. } => f.write_str("an edge lying in a face along a length"),
            Self::ConformalPatch { finding } => write!(
                f,
                "two faces lying against each other (a {} contact)",
                finding.pair.class.name()
            ),
        }
    }
}

/// A declared contact the census could not confirm (tier 3′).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
// The variant roster the sample-coverage row reads (test builds only).
#[cfg_attr(
    test,
    derive(strum::EnumDiscriminants),
    strum_discriminants(name(StaleDeclarationKind), vis(pub(crate)), derive(strum::EnumIter))
)]
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

/// Prose, not `Debug` guts, for the same reason
/// [`CensusContact`]'s rendering is: this payload is quoted into
/// [`ValidationError::StaleContactDeclaration`]'s user-facing message.
/// Each arm names the record's GRANULARITY — which KIND of declaration
/// went stale. Which record it is rides in the typed fields, and at the
/// viewer the at-rest refusal's attribution names the mate that made
/// it, where a mate did.
impl fmt::Display for StaleDeclaration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // In words, without the keys: the typed fields carry them.
        f.write_str(match self {
            Self::VertexVertex { .. } => "a declared vertex-to-vertex contact",
            Self::VertexOnFace { .. } => "a declared vertex-on-face contact",
            Self::CurveLocus { .. } => "a declared contact along an edge",
            Self::Patch { .. } => "a declared face-to-face contact",
        })
    }
}

/// The recourse a kernel defect gets: nothing the user can change in
/// the model repairs a body whose structure a kernel operation (or a
/// damaged file) left wrong, and the sentence says so plainly.
const DEFECT: &str =
    "There is no way through: this is a kernel defect or a damaged file; report it";

/// The recourse for a shape the kernel cannot check yet.
const NOT_YET: &str = "There is no way through yet";

/// The recourse for a tolerance that forms no usable band.
const TOLERANCE: &str = "Recourse: set a finite, positive tolerance";

/// The recourse for a margin the band could not decide, where a
/// coincidence between two things has an object to declare: the
/// shared menu ([`geom_core::COINCIDENCE_RECOURSE`], which
/// `too_close_spells_the_shared_menu` holds these two spellings to),
/// prefixed by the input check a poisoned margin wants first.
fn too_close(margin: Option<&geom_core::MarginDiag>) -> &'static str {
    match margin {
        Some(geom_core::MarginDiag::Invalid) => {
            "Recourse: check the inputs that built this body, then declare the coincidence, \
             move the geometry, or lower the tolerance"
        }
        _ => "Recourse: declare the coincidence, move the geometry, or lower the tolerance",
    }
}

/// The recourse for an undecided margin that is about ONE thing — a
/// torus's own radii, a corner against its own face's plane, an edge's
/// own two faces — where "declare the coincidence" has no object: the
/// arm's own lever, or, for a poisoned margin (not a number at all),
/// the kernel defect it is.
fn own_close(margin: &geom_core::MarginDiag, recourse: &'static str) -> &'static str {
    match margin {
        geom_core::MarginDiag::Invalid => DEFECT,
        _ => recourse,
    }
}

/// An edge's own faces too close to call: the edge-local lever.
const EDGE_CLOSE: &str =
    "Recourse: move the geometry so the faces meet at a clearer angle, or lower the tolerance";

/// A census subject in words, without its keys (they ride in `Debug`).
fn subject_noun(subject: &CensusSubject) -> &'static str {
    match subject {
        CensusSubject::FacePair(..) => "a pair of faces",
        CensusSubject::Entity(e) => entity_noun(*e),
    }
}

fn entity_noun(e: EntityId) -> &'static str {
    match e {
        EntityId::Solid(_) => "a solid",
        EntityId::Shell(_) => "a shell",
        EntityId::Face(_) => "a face",
        EntityId::Loop(_) => "a loop",
        EntityId::HalfEdge(_) => "a half-edge",
        EntityId::Edge(_) => "an edge",
        EntityId::Vertex(_) => "a vertex",
    }
}

/// A surface kind in words.
fn surface_kind_words(kind: geom_brep::SurfaceKind) -> &'static str {
    use geom_brep::SurfaceKind as K;
    match kind {
        K::Plane => "flat",
        K::Cylinder => "cylindrical",
        K::Cone => "conical",
        K::Sphere => "spherical",
        K::Torus => "toroidal",
        K::Nurbs => "spline",
        K::Approx => "fitted",
    }
}

/// A surface datum in words (its `name` is the field's identifier).
fn datum_words(datum: geom::SurfaceDatum) -> &'static str {
    use geom::SurfaceDatum as D;
    match datum {
        D::Origin => "origin",
        D::Normal => "normal",
        D::URef => "reference direction",
        D::Axis => "axis",
        D::Radius => "radius",
        D::Apex => "apex",
        D::HalfAngle => "half-angle",
        D::Center => "center",
        D::MajorRadius => "ring radius",
        D::MinorRadius => "tube radius",
    }
}

fn datum_article(datum: geom::SurfaceDatum) -> &'static str {
    match datum {
        geom::SurfaceDatum::Origin | geom::SurfaceDatum::Axis | geom::SurfaceDatum::Apex => "an",
        _ => "a",
    }
}

// The rendered-reason CLASSIFIERS below map a nested refusal onto the
// short reason and the one recourse a person at the viewer can act
// on. They classify, so each match is exhaustive (this enum's docs):
// a nested variant added upstream is placed here by hand. The nested
// refusal's own sentence — written for a library caller, naming the
// lane, the sample and the lever — rides whole in the payload and in
// its own `Display`.

fn classify_band(e: &BandError) -> &'static str {
    match e {
        BandError::InvalidValue { .. } => "a tolerance threshold is not finite and positive",
        BandError::InvalidLeverArm { .. } => "its lever arm is not finite and positive",
        BandError::Empty { .. } => "its zero threshold is not below its escalate threshold",
    }
}

fn classify_certify(e: &CertifyError) -> (&'static str, &'static str) {
    use geom_brep::PlaneNurbsRefusal as P;
    const MISMATCH: &str = "its stored description does not match its geometry";
    const CLOSE: &str = "its faces meet too nearly tangentially to decide at this tolerance";
    const KIND: &str = "the kernel cannot yet check an edge of this kind";
    match e {
        CertifyError::ChartImageUnavailable { .. }
        | CertifyError::UnresolvedSurface { .. }
        | CertifyError::IntersectionSameSurface { .. }
        | CertifyError::SeamOnNonPeriodic
        | CertifyError::IntervalNotForward
        | CertifyError::WindingExceeded
        | CertifyError::ResidualExceeded { .. }
        | CertifyError::PlaneNurbs(P::PcurveFit | P::Limb { .. }) => (MISMATCH, DEFECT),
        // The DEFINITE halves of their two-tolerance pairs: the check
        // decided, and what it decided contradicts the description.
        CertifyError::NotTransverse { .. } | CertifyError::PlaneNurbs(P::NotTransverse { .. }) => (
            "its faces are tangent where its description says they cross",
            DEFECT,
        ),
        CertifyError::NotSecondOrderSeparated { .. } => (
            "its faces agree to second order, so they do not fix where it runs, which its \
             description says they do",
            DEFECT,
        ),
        CertifyError::PlaneNurbs(P::FootPointInconclusive { .. }) => (
            "the check could not locate the curve on its spline face (the projection did not \
             converge)",
            NOT_YET,
        ),
        CertifyError::Unimplemented
        | CertifyError::TangentCertificateUnsupported
        | CertifyError::PlaneNurbs(P::Unsupported { .. }) => (KIND, NOT_YET),
        CertifyError::PlaneNurbs(P::TubeStraddles { .. }) => (CLOSE, EDGE_CLOSE),
        CertifyError::Escalated { cause, .. } | CertifyError::PlaneNurbs(P::Escalated(cause)) => {
            (CLOSE, own_close(&cause.margin, EDGE_CLOSE))
        }
        CertifyError::Band(b) => (classify_band(b), TOLERANCE),
    }
}

fn classify_offset_fit(e: &geom_brep::OffsetFitError) -> (&'static str, &'static str) {
    use geom_brep::OffsetFitError as O;
    use geom_brep::offset_meters::MeterError as M;
    match e {
        O::Meter(M::NormalFloor { .. } | M::CurvatureHeadroom { .. }) => (
            "the offset folds or degenerates on this face",
            "Recourse: use a smaller offset distance",
        ),
        O::Meter(M::Escalated { source }) => (
            "whether the offset folds here is too close to call at this tolerance",
            own_close(
                &source.margin,
                "Recourse: use a smaller offset distance, or lower the tolerance",
            ),
        ),
        O::BudgetExhausted { .. }
        | O::SampleCapReached { .. }
        | O::BoundNotFinite { .. }
        | O::RefinementStalled { .. }
        | O::Limb { .. } => (
            "the fitted surface does not stay within the tolerance of the one it stands for",
            "Recourse: loosen the tolerance, or rebuild the offset",
        ),
        O::PatchBound(_)
        | O::Fit(_)
        | O::Structure(_)
        | O::InvalidRequest { .. }
        | O::NonFiniteSample { .. }
        | O::WindowUnsupported { .. } => ("its stored fit is not well-formed", DEFECT),
    }
}

fn classify_mass_props(e: &crate::props::MassPropsError) -> (&'static str, &'static str) {
    use crate::props::MassPropsError as M;
    use geom_brep::props::PropsError as P;
    match e {
        M::Band { error } => (classify_band(error), TOLERANCE),
        M::Face { source, .. } => match source {
            P::Escalated { cause } => (
                "a face's contribution is too close to call at this tolerance",
                own_close(&cause.margin, "Recourse: lower the tolerance"),
            ),
            P::QuadratureBudget { .. } => (
                "a face's contribution did not converge to the tolerance",
                "Recourse: loosen the tolerance",
            ),
            P::Unimplemented
            | P::NotIsoRectangle { .. }
            | P::NappeSpanning
            | P::NotOneChartBranch { .. }
            | P::QuadratureUnsupported { .. } => {
                ("the kernel cannot yet measure a face of this kind", NOT_YET)
            }
            P::DegenerateFace => ("a face encloses no area", DEFECT),
        },
        M::RingOnCurvedFace { .. } => (
            "the kernel cannot yet measure a curved face with a hole",
            NOT_YET,
        ),
        M::Corrupt { .. } | M::NullScaffoldEdge { .. } => ("its structure is incomplete", DEFECT),
    }
}

fn classify_pcurve(e: &crate::pcurves::PcurveMintError) -> (&'static str, &'static str) {
    use crate::pcurves::PcurveMintError as M;
    use geom_brep::PcurveCertifyError as C;
    const WRONG: &str = "the stored boundary does not match the face";
    const KIND: &str = "the kernel cannot yet map a boundary of this kind";
    const CLOSE: &str = "the boundary is too close to call at this tolerance";
    match e {
        M::Corrupt
        | M::LoopDiscontinuity { .. }
        | M::LoopNotClosed { .. }
        | M::SingularChartJoint { .. }
        | M::MissingCache { .. } => (WRONG, DEFECT),
        M::OuterSpansPeriod | M::LoopWraps { .. } => (
            "the face wraps all the way round its surface, which the kernel cannot yet map",
            NOT_YET,
        ),
        M::Escalated { cause, .. } => (
            CLOSE,
            own_close(&cause.margin, "Recourse: lower the tolerance"),
        ),
        M::Band(b) => (classify_band(b), TOLERANCE),
        M::Certify { error, .. } => match error {
            C::UnsupportedChart { .. }
            | C::UnsupportedCarrier
            | C::IsoUnsupported { .. }
            | C::ChartWindingUnsupported
            | C::AzimuthPeriodExceeded
            | C::FittedMateMissing => (KIND, NOT_YET),
            C::FittedLaneUnsupported { .. } => (
                "this scalar cannot certify a fitted boundary",
                "Recourse: check the body at a certifying scalar",
            ),
            C::ChartRow { .. }
            | C::IntervalNotForward
            | C::ResidualExceeded { .. }
            | C::TrimEscape
            | C::FittedCertificate { .. } => (WRONG, DEFECT),
            C::FittedEscalated { cause } | C::Escalated { cause, .. } => (
                CLOSE,
                own_close(&cause.margin, "Recourse: lower the tolerance"),
            ),
            C::Band(b) => (classify_band(b), TOLERANCE),
        },
    }
}

/// A point too near a boundary to place: the lever is the point's own.
const OFF_BOUNDARY: &str =
    "Recourse: move the geometry clear of the boundary, or lower the tolerance";

fn classify_contain(e: &ContainError) -> (&'static str, &'static str) {
    match e {
        ContainError::Escalated(diag) => (
            "a point of it lies too close to a boundary to place at this tolerance",
            own_close(&diag.margin, OFF_BOUNDARY),
        ),
        ContainError::RayExhausted => (
            "a point of it lies too close to a boundary to place at this tolerance",
            OFF_BOUNDARY,
        ),
        ContainError::Corrupt => ("its boundary could not be walked", DEFECT),
        ContainError::ArcLoopUnsupported { .. } => (
            "its boundary is arcs over fewer than three corners, which the check cannot \
             read as a region",
            "Recourse: split an arc so the boundary has three corners, or draw the region \
             as one circle",
        ),
    }
}

fn classify_chart_region(e: &ChartRegionError) -> (&'static str, &'static str) {
    match e {
        ChartRegionError::ChartDivergence { .. } => (
            "the two faces lie on separately described surfaces, which the check cannot \
             compare",
            NOT_YET,
        ),
        ChartRegionError::NonPlanarTrim { .. } => (
            "a face's boundary is curved in a way the check cannot yet measure",
            NOT_YET,
        ),
        // Where a round surface's seam falls is set by the part's
        // placement, so turning a part about its axis moves the seam
        // off the contact — the viewer's spelling of the nested
        // sentences' "re-seat the trims / move the window off the tie".
        ChartRegionError::SeamBranch | ChartRegionError::PeriodFold => (
            "the contact straddles the seam of a round surface, which the check cannot yet \
             place",
            "Recourse: turn one part a little about its axis, so the seam falls away from \
             the contact",
        ),
        ChartRegionError::ArmUnbounded { .. } => (
            "a face reaches a pole, apex or fold of its surface",
            "Recourse: keep the contact clear of the surface's pole, apex or fold",
        ),
        ChartRegionError::CarrierTilt => (
            "the two faces' surfaces drift apart across the contact",
            "Recourse: move the parts so the two faces lie exactly on each other",
        ),
        ChartRegionError::TouchingBoundary => (
            "the faces' edges touch at this tolerance, so their overlap is undecidable",
            "Recourse: move the edges clearly apart or clearly across each other, or lower \
             the tolerance",
        ),
        ChartRegionError::Escalated(diag) => (
            "their overlap is too close to call at this tolerance",
            too_close(Some(&diag.margin)),
        ),
        ChartRegionError::RayExhausted => (
            "a point lies too close to a boundary to place at this tolerance",
            too_close(None),
        ),
        ChartRegionError::WitnessBudgetExhausted { .. } => (
            "their boundaries cross too many times for the check to finish",
            "Recourse: simplify the faces' boundaries",
        ),
        ChartRegionError::DegenerateLoop { .. } => (
            "a face's boundary encloses no definite area at this tolerance (a sliver)",
            "Recourse: widen the face well past the tolerance, or lower the tolerance",
        ),
        ChartRegionError::MissingCache { .. } | ChartRegionError::Corrupt => {
            ("a face's stored boundary is incomplete", DEFECT)
        }
    }
}

fn classify_contact_lane(e: &ContactRefusal) -> (&'static str, &'static str) {
    match e {
        ContactRefusal::Contradicted { .. } => (
            crate::contact::CONTRADICTION_REASON,
            crate::contact::CONTRADICTION_RECOURSE,
        ),
        // A contact site's recourse has no tolerance arm (SELECT §3d,
        // `CONTACT_RECOURSE`): loosening ε cannot supply intent.
        ContactRefusal::Escalated { .. } => (
            "whether the declared faces touch is too close to call",
            crate::contact::CONTACT_RECOURSE_MARKED,
        ),
        ContactRefusal::Undeclared { .. } => (
            "the faces touch with no declared contact behind them",
            crate::contact::CONTACT_RECOURSE_MARKED,
        ),
        ContactRefusal::NotCertifiable { .. } => (
            "the kernel cannot yet check a contact between faces of these kinds",
            NOT_YET,
        ),
    }
}

fn classify_census_cause(cause: &CensusUnsupportedCause) -> (&'static str, &'static str) {
    match cause {
        CensusUnsupportedCause::ChartRegion(e) => classify_chart_region(e),
        CensusUnsupportedCause::ContactLane(e) => classify_contact_lane(e),
        CensusUnsupportedCause::Containment(e) => classify_contain(e),
        CensusUnsupportedCause::FaceUnboundable => (
            "a face has no corner to bound it by (an empty or broken outer loop)",
            DEFECT,
        ),
    }
}

// Every arm says, in the words of a person at the viewer, what is
// wrong and the ONE thing to do about it — `Recourse: …`, or `There is
// no way through …` where nothing in the model repairs it. Arena keys
// are developer detail and ride in `Debug`, except on the tier-1/2
// arms: those report a damaged structure, and the key is what the bug
// report needs. A nested refusal renders through its classifier above,
// never whole.
impl fmt::Display for ValidationError {
    #[allow(clippy::too_many_lines)] // one sentence per arm
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Band { error } => write!(
                f,
                "the tolerance the body was checked at is not usable: {}. {TOLERANCE}",
                classify_band(error)
            ),
            Self::DanglingDescription { from, to } => write!(
                f,
                "{from}'s description references {to}, which does not resolve. {DEFECT}"
            ),
            Self::UncertifiableSurface { .. } => write!(
                f,
                "a face has no real surface yet, only the placeholder a construction starts \
                 from. {DEFECT}"
            ),
            Self::PoisonedSurfaceDescription { .. } => write!(
                f,
                "a face's surface description holds invalid numbers, so it describes no \
                 shape. {DEFECT}"
            ),
            Self::ApproxCertification { error, .. } => {
                let (why, recourse) = classify_offset_fit(error);
                write!(
                    f,
                    "a face's fitted offset surface no longer certifies against the surface \
                     it approximates: {why}. {recourse}"
                )
            }
            Self::ApproxLaneUnsupported { .. } => write!(
                f,
                "a face carries a fitted offset surface, and this scalar has no \
                 re-derivation lane for its certificate. Recourse: check the body at f64, \
                 the one scalar that re-derives it"
            ),
            Self::DegenerateTorus { .. } => write!(
                f,
                "a torus face's tube radius is not smaller than its ring radius (a horn or \
                 spindle torus). Recourse: make the tube radius smaller than the ring radius"
            ),
            Self::DegenerateTorusEscalated { cause, .. } => write!(
                f,
                "whether a torus face's tube radius is smaller than its ring radius is too \
                 close to call at this tolerance. {}",
                own_close(
                    &cause.margin,
                    "Recourse: make the tube radius clearly smaller than the ring radius, or \
                     lower the tolerance"
                )
            ),
            Self::PoisonedSurfaceDatum { kind, datum, .. } => write!(
                f,
                "a {} face's surface stores {} {} that is {}, so it describes no shape. \
                 {DEFECT}",
                surface_kind_words(*kind),
                datum_article(*datum),
                datum_words(*datum),
                if *datum == geom::SurfaceDatum::Normal {
                    "not finite, or is the zero vector"
                } else {
                    "not a finite number"
                },
            ),
            Self::UnrepresentableSurfaceDatum {
                kind, datum, end, ..
            } => write!(
                f,
                "a {} face's surface stores a {} {} the range that surface allows, so it \
                 describes no surface a face can bound. Recourse: give the {} a value \
                 inside its range",
                surface_kind_words(*kind),
                datum_words(*datum),
                match end {
                    geom::ConventionEnd::Lower => "at or below",
                    geom::ConventionEnd::Upper => "at or above",
                },
                datum_words(*datum),
            ),
            Self::EdgeCertification { error, .. } => {
                let (why, recourse) = classify_certify(error);
                write!(
                    f,
                    "an edge's stored curve does not certify against its faces: {why}. \
                     {recourse}"
                )
            }
            Self::DescriptionNotAdjacent { .. } => write!(
                f,
                "an edge's description names surfaces that are not its two faces' \
                 surfaces. {DEFECT}"
            ),
            Self::PlanarFaceResidual { .. } => {
                write!(
                    f,
                    "a corner of a flat face lies off that face's plane. {DEFECT}"
                )
            }
            Self::PlanarFaceEscalated { cause, .. } => write!(
                f,
                "whether a corner of a flat face lies on its plane is too close to call at \
                 this tolerance. {}",
                own_close(&cause.margin, "Recourse: lower the tolerance")
            ),
            Self::PlanarBoundaryResidual { .. } => write!(
                f,
                "an edge of a flat face leaves that face's plane between its ends. {DEFECT}"
            ),
            Self::PlanarBoundaryEscalated { cause, .. } => write!(
                f,
                "whether an edge of a flat face stays on its plane is too close to call at \
                 this tolerance. {}",
                own_close(&cause.margin, "Recourse: lower the tolerance")
            ),
            Self::SliverDihedral { cause, .. } => write!(
                f,
                "the angle between two faces at an edge is too close to call at this \
                 tolerance (a sliver). {}",
                own_close(&cause.margin, EDGE_CLOSE)
            ),
            Self::TransverseNotIntrinsic { .. } => write!(
                f,
                "an edge where two faces cross is stored as a sketch curve, though their \
                 surfaces determine it. {DEFECT}"
            ),
            // The finding never asserts that a chart image exists: a
            // fillet strut on a curved support and a diagonal chord
            // across a cylinder are SECANTS, lying in neither adjacent
            // surface. What it names is a construction that did not
            // come to rest as built.
            Self::ScaffoldAtRest { .. } => write!(
                f,
                "an edge still carries the stand-in description a construction uses before \
                 its faces exist, so the operation that built it stopped half-way. {DEFECT}"
            ),
            Self::TangentNotIntrinsic { .. } => write!(
                f,
                "an edge where two faces meet tangentially is stored as a sketch curve, \
                 though their surfaces determine it. {DEFECT}"
            ),
            Self::UndeclaredCusp { wedge, .. } => write!(
                f,
                "two faces meet at an edge in a {}, which is valid only where a Tangent \
                 contact between them is declared. Recourse: declare a Tangent contact \
                 between the two faces, or move the geometry",
                wedge.name()
            ),
            Self::LaminaWedge { .. } => write!(
                f,
                "two faces lie flat against each other from opposite sides at an edge, \
                 which no contact declaration can make valid: either the body has zero \
                 thickness here, or one face is inside-out. Recourse: move the geometry so \
                 the body has thickness here; if it already has, report the inside-out face \
                 as a kernel defect"
            ),
            Self::LoopRoleInverted { .. } => write!(
                f,
                "a flat face's boundary runs the wrong way round (its outline and holes are \
                 swapped, or the face is inside-out). {DEFECT}"
            ),
            Self::CurvedSenseInverted { .. } => write!(
                f,
                "a curved face is inside-out: its stored orientation disagrees with its \
                 boundary. {DEFECT}"
            ),
            Self::NegativeVolume { .. } => write!(
                f,
                "a solid encloses negative volume, so it is inside-out. {DEFECT}"
            ),
            Self::VolumeUncomputable { source, .. } => {
                let (why, recourse) = classify_mass_props(source);
                write!(
                    f,
                    "a solid's volume could not be computed to check that it is not \
                     inside-out: {why}. {recourse}"
                )
            }
            Self::Pcurve { finding } => {
                let (why, recourse) = classify_pcurve(finding);
                write!(
                    f,
                    "a face's boundary could not be mapped onto its surface: {why}. {recourse}"
                )
            }
            Self::RingMeetsOuter { contact, .. } => write!(
                f,
                "a hole in a face touches the face's outline {}, so the face encloses no \
                 single region. Recourse: move the hole so it lies strictly inside the \
                 outline",
                match contact {
                    RingContact::Vertex { .. } => "at a corner",
                    RingContact::VertexOnEdge { .. } => "where a corner meets an edge",
                    RingContact::Edge { .. } => "along an edge",
                    RingContact::OuterVertexOnEdge { .. } => {
                        "where a corner of the outline meets an edge of the hole"
                    }
                    RingContact::Circles { .. } => "where the two circles cross or touch",
                    RingContact::EdgesMeet { .. } => "where two of their edges cross or touch",
                }
            ),
            Self::RingContactEscalated { source, .. } => write!(
                f,
                "whether a hole in a face touches the face's outline is too close to call at \
                 this tolerance. {}",
                own_close(
                    &source.margin,
                    "Recourse: move the hole clearly inside the outline, or lower the tolerance"
                )
            ),
            Self::RingOutsideOuter { .. } => write!(
                f,
                "a hole in a face reaches outside the face's outline. Recourse: move the \
                 hole so it lies strictly inside the outline"
            ),
            Self::RingNestingUndecided { source, .. } => {
                let (why, recourse) = classify_contain(source);
                write!(
                    f,
                    "whether a hole in a face lies inside the face's outline could not be \
                     decided: {why}. {recourse}"
                )
            }
            // The position alone: a witness may carry detail after " — "
            // (the field's contract), which rides in `Debug`.
            Self::UndeclaredContact { contact, witness } => write!(
                f,
                "{contact} at {} is an undeclared contact. {}",
                witness.split(" — ").next().unwrap_or(witness),
                crate::contact::CONTACT_RECOURSE_MARKED
            ),
            Self::StaleContactDeclaration { declaration } => write!(
                f,
                "{declaration} names a contact the geometry does not have. \
                 Recourse: remove that declaration (the mate named with it, where one is), \
                 or move the geometry so the contact exists"
            ),
            Self::ContactContradicted {
                declaration,
                witness,
                steer,
                ..
            } => write!(
                f,
                "the declared {} contact is contradicted {witness}: {}. {}{}",
                declaration.class.name(),
                crate::contact::CONTRADICTION_REASON,
                crate::contact::CONTRADICTION_RECOURSE,
                crate::contact::steer_clause(*steer),
            ),
            Self::CensusEscalated { cause } => write!(
                f,
                "whether two parts of the body touch is too close to call at \
                 this tolerance. {}",
                too_close(Some(&cause.margin))
            ),
            Self::CensusUnsupported { subject, cause } => {
                let (why, recourse) = classify_census_cause(cause);
                write!(
                    f,
                    "{} could not be checked: {why}. {recourse}",
                    subject_noun(subject)
                )
            }
            Self::CensusLaneUnsupported { .. } => write!(
                f,
                "a pair of faces was not examined, because this structural \
                 check holds no certified chart-overlap lane at any scalar; nothing was \
                 decided about the geometry. Recourse: run the certified check \
                 (validate_pseudomanifold, or validate_pseudomanifold_certificate for a \
                 certificate) at a certifying scalar"
            ),
            Self::CensusUndecidable { a, b, what } => write!(
                f,
                "{} can be neither examined nor cleared: {what}",
                match (a, b) {
                    (EntityId::Face(_), EntityId::Face(_)) => "two faces of different parts",
                    (EntityId::Solid(_), EntityId::Solid(_)) => "two parts",
                    _ => "two parts of the body",
                }
            ),
            Self::InstanceInterference { .. } => write!(
                f,
                "two instances overlap: a corner of one lies inside the other \
                 (an interference fit), which no declaration can make valid. Recourse: move \
                 the instances apart"
            ),
            Self::DanglingTopology { from, to } => {
                write!(
                    f,
                    "{from} references {to}, which does not resolve. {DEFECT}"
                )
            }
            Self::DanglingGeometry { from, to } => {
                write!(
                    f,
                    "{from} references {to}, which does not resolve. {DEFECT}"
                )
            }
            Self::NextPrevMismatch { half_edge } => write!(
                f,
                "half-edge {half_edge:?}'s next half-edge does not point back to it. {DEFECT}"
            ),
            Self::LoopCycleOverrun { loop_ } => write!(
                f,
                "loop {loop_:?} does not close within the arena bound. {DEFECT}"
            ),
            Self::ParentLoopMismatch { half_edge, owner } => write!(
                f,
                "half-edge {half_edge:?} is in loop {owner:?}'s cycle but names another \
                 loop as its parent. {DEFECT}"
            ),
            Self::UnreachableHalfEdge { half_edge } => write!(
                f,
                "half-edge {half_edge:?} is not reached by its loop's boundary. {DEFECT}"
            ),
            Self::EdgeHalvesIdentical { edge } => write!(
                f,
                "edge {edge:?}'s two half-edge slots hold the same half-edge. {DEFECT}"
            ),
            Self::EdgeSlotBackpointerMismatch { edge, half_edge } => write!(
                f,
                "edge {edge:?} claims half-edge {half_edge:?}, which points at a different \
                 edge. {DEFECT}"
            ),
            Self::HalfEdgeUnclaimed { half_edge } => {
                write!(f, "half-edge {half_edge:?} belongs to no edge. {DEFECT}")
            }
            Self::HalfEdgeMultiplyClaimed { half_edge, claims } => write!(
                f,
                "half-edge {half_edge:?} is claimed by {claims} edges, not one. {DEFECT}"
            ),
            Self::EdgeNotAntiparallel { edge } => write!(
                f,
                "edge {edge:?}'s half-edges do not run in opposite directions. {DEFECT}"
            ),
            Self::EmanatingStartMismatch { vertex, emanating } => write!(
                f,
                "vertex {vertex:?}'s outgoing half-edge {emanating:?} does not start at it. \
                 {DEFECT}"
            ),
            Self::EmptyLoopVertexWithEmanating {
                vertex,
                empty_loops,
            } => write!(
                f,
                "vertex {vertex:?} has an outgoing half-edge but is the lone vertex of \
                 {empty_loops} empty loop(s). {DEFECT}"
            ),
            Self::LoneVertexWithIncidence { vertex, incident } => write!(
                f,
                "vertex {vertex:?} has no outgoing half-edge but {incident} half-edge(s) \
                 start at it. {DEFECT}"
            ),
            Self::VertexOrbitOverrun { vertex } => write!(
                f,
                "the half-edges around vertex {vertex:?} do not close within the arena \
                 bound. {DEFECT}"
            ),
            Self::OrbitForeignMember { vertex, half_edge } => write!(
                f,
                "the half-edges around vertex {vertex:?} include {half_edge:?}, which \
                 starts at a different vertex. {DEFECT}"
            ),
            Self::SplitVertexOrbit {
                vertex,
                orbit,
                incident,
            } => write!(
                f,
                "vertex {vertex:?} is non-manifold: {orbit} half-edge(s) circle it but \
                 {incident} start at it. {DEFECT}"
            ),
            Self::OuterListedAsRing { face } => write!(
                f,
                "face {face:?} lists its outline among its holes. {DEFECT}"
            ),
            Self::BackPointerMismatch {
                child,
                stored,
                owner,
            } => write!(
                f,
                "{child} points back at {stored} but is owned by {owner}. {DEFECT}"
            ),
            Self::OrphanEntity { entity } => {
                write!(f, "{entity} belongs to nothing. {DEFECT}")
            }
            Self::MultiplyOwned { child, owners } => {
                write!(f, "{child} has {owners} owners, not one. {DEFECT}")
            }
            Self::OrphanGeometry { geometry } => {
                write!(f, "{geometry} is used by nothing. {DEFECT}")
            }
            Self::SolidWithoutShells { solid } => {
                write!(f, "solid {solid:?} has no shells. {DEFECT}")
            }
            Self::ShellWithoutFaces { shell } => {
                write!(f, "shell {shell:?} has no faces. {DEFECT}")
            }
            Self::EdgeAcrossShells {
                edge,
                shell_plus,
                shell_minus,
            } => write!(
                f,
                "edge {edge:?}'s two halves lie in different shells ({shell_plus:?} and \
                 {shell_minus:?}). {DEFECT}"
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
                    "shell {shell:?}'s component at face {seed:?} fails Euler–Poincaré \
                     (v − e + f − r = {chi}, not 2 − 2g for any genus g). {DEFECT}"
                )
            }
            Self::MissingProvenance { entity } => {
                write!(f, "{entity} has no provenance record. {DEFECT}")
            }
            Self::LeakedProvenance { entity } => {
                write!(f, "a provenance record outlives {entity}. {DEFECT}")
            }
            Self::ScaffoldingEmptyLoop { loop_ } => write!(
                f,
                "loop {loop_:?} is an empty construction loop left in a closed solid. \
                 {DEFECT}"
            ),
            Self::ScaffoldingStrutVertex { vertex } => write!(
                f,
                "vertex {vertex:?} ends a construction strut left in a closed solid. {DEFECT}"
            ),
            Self::ShellDisconnected { shell, components } => write!(
                f,
                "shell {shell:?} falls into {components} separate pieces, not one. {DEFECT}"
            ),
            Self::NullScaffoldShared { curve, edges } => write!(
                f,
                "construction curve {curve:?} is shared by {edges} edges, not one. {DEFECT}"
            ),
            Self::LeakedNullFaceRecord { face } => write!(
                f,
                "a construction record outlives its face {face:?}. {DEFECT}"
            ),
            Self::StaleNullFaceLoop { face, named_loop } => write!(
                f,
                "a construction record on face {face:?} names loop {named_loop:?}, which \
                 no longer exists. {DEFECT}"
            ),
            Self::NullEdgeAtRest { edge } => write!(
                f,
                "edge {edge:?} is a construction edge left in a finished body. {DEFECT}"
            ),
            Self::NullFaceAtRest { face } => write!(
                f,
                "face {face:?} is a construction face left in a finished body. {DEFECT}"
            ),
        }
    }
}

impl std::error::Error for ValidationError {}

/// How a ring meets its face's outer loop
/// ([`ValidationError::RingMeetsOuter`]) — the six shapes check 9's
/// contact arms can find.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
// The variant roster the sample-coverage row reads (test builds only).
#[cfg_attr(
    test,
    derive(strum::EnumDiscriminants),
    strum_discriminants(name(RingContactKind), vis(pub(crate)), derive(strum::EnumIter))
)]
pub enum RingContact {
    /// A vertex of the ring stands on a vertex of the outer loop.
    Vertex {
        /// The ring's vertex.
        ring_vertex: VertexKey,
        /// The outer loop's vertex it stands on.
        outer_vertex: VertexKey,
    },
    /// A vertex of the ring stands on the INTERIOR of an edge of the
    /// outer loop — the shape neither of the other two arms can see,
    /// because it is a vertex of one loop and an interior point of the
    /// other.
    VertexOnEdge {
        /// The ring's vertex.
        ring_vertex: VertexKey,
        /// The outer loop's edge it stands on.
        outer_edge: EdgeKey,
    },
    /// An edge of the ring runs ALONG an edge of the outer loop —
    /// sampled interior points of the ring's edge all lie on the outer
    /// edge's LOCUS, and (on a line, whose locus runs past the trim)
    /// at least one of them lies strictly between that edge's
    /// endpoints, so the two share a positive-length arc rather than
    /// meeting at a point.
    Edge {
        /// The ring's edge.
        ring_edge: EdgeKey,
        /// The outer loop's edge it runs along.
        outer_edge: EdgeKey,
    },
    /// A vertex of the OUTER loop stands on the interior of an edge of
    /// the ring — [`Self::VertexOnEdge`] with the roles swapped, the
    /// touch a reflex corner of the outer boundary makes against the
    /// hole.
    OuterVertexOnEdge {
        /// The outer loop's vertex.
        outer_vertex: VertexKey,
        /// The ring's edge it stands on.
        ring_edge: EdgeKey,
    },
    /// The ring and the outer loop are two WHOLE circles — both in
    /// [`crate::boolean::LoopShape`]'s disc class — that cross or
    /// touch: their centre distance lies between the difference of
    /// their radii and the sum, both ends included. Named by LOOP,
    /// because a crossing need not put a ring vertex outside, or
    /// anywhere near the points where the two circles meet.
    Circles {
        /// The ring.
        ring_loop: LoopKey,
        /// The outer loop.
        outer_loop: LoopKey,
    },
    /// An edge of the ring and an edge of the outer loop share a POINT
    /// that is a vertex of neither loop: a transversal crossing or a
    /// one-point tangency. Found by intersecting the two edges' `Line`
    /// and `Circle` carriers and testing each meeting point against
    /// both edges' trims.
    EdgesMeet {
        /// The ring's edge.
        ring_edge: EdgeKey,
        /// The outer loop's edge it meets.
        outer_edge: EdgeKey,
    },
}

impl fmt::Display for RingContact {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Vertex {
                ring_vertex,
                outer_vertex,
            } => write!(
                f,
                "{ring_vertex:?} stands on the outer loop's {outer_vertex:?}"
            ),
            Self::VertexOnEdge {
                ring_vertex,
                outer_edge,
            } => write!(
                f,
                "{ring_vertex:?} stands on the interior of the outer loop's {outer_edge:?}"
            ),
            Self::Edge {
                ring_edge,
                outer_edge,
            } => write!(
                f,
                "{ring_edge:?} runs along the outer loop's {outer_edge:?}"
            ),
            Self::OuterVertexOnEdge {
                outer_vertex,
                ring_edge,
            } => write!(
                f,
                "the outer loop's {outer_vertex:?} stands on the interior of {ring_edge:?}"
            ),
            Self::Circles {
                ring_loop,
                outer_loop,
            } => write!(
                f,
                "the ring's circle {ring_loop:?} crosses or touches the outer loop's circle \
                 {outer_loop:?}"
            ),
            Self::EdgesMeet {
                ring_edge,
                outer_edge,
            } => write!(
                f,
                "{ring_edge:?} crosses or touches the outer loop's {outer_edge:?} at a point"
            ),
        }
    }
}

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
/// 1. **Surface implementedness** (faces, arena order): a `Nurbs`
///    payload's [`geom::NetState`] is read and each state answered —
///    `Placeholder` reports [`ValidationError::UncertifiableSurface`],
///    `Poisoned` reports
///    [`ValidationError::PoisonedSurfaceDescription`], `Described`
///    passes; every analytic surface's stored datums are numbers, and
///    a plane's normal is not zero
///    ([`ValidationError::PoisonedSurfaceDatum`]); each datum lies
///    inside its convention
///    ([`ValidationError::UnrepresentableSurfaceDatum`], the bounds
///    being [`geom::Surface::representability_margins`]'s); and every
///    torus honours D3's ring convention `R > r > 0`
///    ([`ValidationError::DegenerateTorus`] /
///    [`ValidationError::DegenerateTorusEscalated`]).
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
///    **The material arm** (D1's ratified wedge table, the #131
///    second-order ruling) signs that classification with the faces'
///    material sides on the same samples: outward normals aligned is
///    the legal π seam; opposed is the wedge-0/2π pair, legal iff a
///    `Tangent` contact on the face pair DECLARES the tangency
///    ([`ValidationError::UndeclaredCusp`]) and the jet is determinate
///    — a collapsed κ_rel there is conformal contact along the locus,
///    which the arm does not admit under any declaration
///    ([`ValidationError::LaminaWedge`]: a true lamina, or an
///    inverted face sense on a shared surface — that error's own doc
///    separates them), and an in-band κ_rel is the ordinary
///    `SliverDihedral` escalation. Samples that DISAGREE along one
///    edge escalate too, rather than falling silent.
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
/// 6. **Loop-role and sense orientation** (faces, arena order): a
///    planar face's loop windings against its outward normal
///    ([`ValidationError::LoopRoleInverted`]) and a curved face's
///    stored `sense` bit against the material side its own boundary
///    traversal encodes ([`ValidationError::CurvedSenseInverted`]).
/// 7. **The +V invariant, PER SOLID** (solids, arena order): each
///    solid's own faces enclose definitely-positive volume, decided
///    from a certified enclosure at the round its sign stops being in
///    doubt ([`ValidationError::NegativeVolume`], naming the solid;
///    [`ValidationError::VolumeUncomputable`] where the quadrature
///    produces no enclosure at all or the sign is still indefinite
///    when the schedule runs out). The margin is `V / A` over that
///    solid's faces, a length; `Zero` and escalated margins are
///    exempt. **The subject is the solid and not the body**: a body's
///    total is a sum, and a sum hides a sign — an inside-out part
///    beside a larger ordinary solid totals positive.
/// 8. **Stored pcurve caches** ([`ValidationError::Pcurve`]).
/// 9. **Ring versus outer loop** — disjointness and nesting
///    ([`ValidationError::RingMeetsOuter`] and its siblings).
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
/// - **The wedge on a NURBS- or `Approx`-adjacent edge**: check 4 and
///   its material arm are both exempt by kind there (implicit-form
///   gradients are poison on a spline chart), so such an edge carries
///   no wedge verdict at all — `Unmarked`, the escalation posture,
///   never a blessing.
/// - **The doubled cusp** (two material wedges on one tangent line —
///   the kissing union, a slit interior to material) is not one
///   4-face edge but F2's coincident-distinct-edges class: each edge
///   classifies separately under check 4's material arm, and pairing
///   the two is the coincidence census's business, not this pass's.
/// - **The wedge-conditioned CONSUMERS** (fillet/chamfer, offset and
///   shell, mesh sizing, boolean sector classification, export) each
///   owe their own typed refusal for a wedge-0/2π edge: this pass
///   decides legality at rest and says nothing about what an operation
///   may then do with a legal cusp.
/// - **Face-boundary containment on curved surfaces** (a face's loops
///   actually bounding a region of its surface) — tracked at #638.
///   The **planar** case is now covered between vertices by check 5
///   (sample containment against adjacent planar faces), and its
///   orientation half by check 6 (loop-role winding against the
///   outward normal, on loops of line and circle carriers — a planar
///   loop riding an ellipse, spiric or NURBS carrier is not examined,
///   so such a face's sense bit is falsified by nothing at rest;
///   `work/atrest/check-6-planar-arm-skips-ellipse-and-nurbs-loops.md`).
///   The **curved analytic**
///   kinds' orientation half is covered by check 6's curved arm
///   (M6-6: boundary material side vs the sense bit), and its NESTING
///   half — a ring lying inside the outer loop of its own face — by
///   check 9's nesting arm, on planar faces whose outer loop bears no
///   arc or is one circle ([`crate::boolean::loop_shape`]'s `Polygon`
///   and `Disc` classes). What remains deferred is containment against
///   curved surfaces and the region-bounding statement for curved faces
///   and for the planar loop classes that arm is silent on — the
///   nesting arm's own residue, enumerated at check 9's banner and
///   waiting on the arc-aware walk
///   (`work/atrest/check-9-nesting-arc-parity-and-no-walk-wait-on-the-arc-aware-walk`),
///   sits inside that same deferral, and so does check 9's CONTACT
///   half off a plane and on an `Ellipse`, `Spiric` or NURBS edge,
///   where a ring crossing its outer loop at a point no vertex carries
///   is not seen —
///   plus the curved arm's documented residuals (the
///   rimless sphere band; NURBS faces; the quadrature-owned
///   conic-trimmed walls, whose boundary parse refuses typed and is
///   therefore exempt — such a body's flips, single-face AND
///   whole-body, certify green today; executed on the tilted-section
///   cylinder and pinned as residual).
/// - **A shell NESTED inside another shell's cavity.** No tier reads
///   where one shell of a solid sits relative to another. What that
///   leaves unchecked is precisely a solid holding an `Outer` shell, a
///   `Void`, and a second `Outer` INSIDE that void
///   (`work/atrest/tier-3-does-not-check-shell-roles-per-solid`): the
///   nesting is the claim, and tier 3 has no at-rest containment walk
///   for it — the same family as check 9's deferred nesting half and
///   `validate-tier3-curved-boundary-containment`.
///
///   **The COUNT is not the gap**, and that is measured rather than
///   assumed. A solid holding several `Outer` shells is what four
///   doors produce ON PURPOSE — `graft onto`, the boolean coplanar
///   split, `subtract`'s two-shell complement and the editor's placed
///   union — and how many material components a product should
///   have is answered one layer up, as `editor_core`'s
///   `CheckId::Connectedness` finding against an authored expectation.
///   Tier 3 refusing that count would make tier 3 wrong, not the doors:
///   `work/atrest/one-solid-holding-two-outer-shells-is-what-five-kernel-doors-produce`
///   carries the 36 rows that settled it.
/// - **Curve conventional-invariant certification** (unit `dir`/`axis`,
///   `u_ref ⊥ axis`): partially implied by the residual checks (a
///   non-unit frame breaks the carrier-vs-description comparisons),
///   not independently certified yet.
///
/// # Errors
///
/// A non-empty vector of every failure found: tiers 1–2 verbatim if
/// any, else the tier-3 failures in the documented order.
/// # The two halves, and why the entry carries both bounds
///
/// Tier 3 is a battery of nine checks, eight of which any deciding
/// scalar can answer and one of which — check 7, the +V invariant — is
/// an act of CERTIFICATION: it reads a certified volume enclosure. So
/// the battery is written as two functions and this one is their
/// composition:
///
/// - [`validate_geometric_structural`] runs checks 1–6, 8 and 9 at
///   every [`crate::AtRestPolicy`] scalar. It is a meaningful validator
///   on its own and it is the door a scalar without certification
///   rights uses.
/// - `validate_geometric_certified` runs check 7, bounded on the
///   quantity it actually needs.
///
/// This entry is `structural(…)?` then certified, so its bound is the
/// UNION and the `?` is the sequencing fact: check 7 used to run behind
/// an `if errors.is_empty()` in the middle of the battery, a rule the
/// file stated about itself rather than enforced. The composition
/// enforces it, and widens it in the same direction — a body that fails
/// any structural check never reaches the volume claim, where before
/// only checks 1–6 gated it.
///
/// **A scalar that may not certify cannot write this call**, which is
/// the point rather than a side effect: it is not refused here, there
/// is no arm and no diagnostic. **What such a scalar loses at THIS door
/// is the +V verdict itself, not merely a refusal it used to receive**:
/// check 7's closed-form derivation computes at any scalar, so before
/// the split a dual asking this door about a planar body got a real
/// orientation sign. It gets neither now, because the sign is decided in
/// one place and that place is the certified half. A dual body still
/// validates — through [`validate_geometric_structural`], which is where
/// every certificate a dual build compares bitwise against its `f64`
/// twin is produced — and the orientation verdict is still available to
/// it through [`validate_pseudomanifold_structural`],
/// [`contact_marks_structural`] and [`crate::mass_properties_structural`],
/// the passes that hold no quadrature lane and make check 7 through the
/// closed form. The structural half is open to it:
///
/// ```
/// use geom_core::{Dual64, Tol};
/// use topo::{Body, validate_geometric_structural};
/// fn structural(b: &Body<Dual64>, tol: Tol) {
///     let _ = validate_geometric_structural(b, tol);
/// }
/// ```
///
/// — and this entry is not, because the certified half's bound is on
/// it. The call cannot be FORMED; nothing runs and nothing refuses:
///
/// ```compile_fail,E0277
/// use geom_core::{Dual64, Tol};
/// use topo::{Body, validate_geometric};
/// fn composed(b: &Body<Dual64>, tol: Tol) {
///     let _ = validate_geometric(b, tol);
/// }
/// ```
///
/// The failing row is the guarantee and not a typo, and the row above
/// it is what says so: the two differ in exactly one identifier, and
/// every path either names resolves. So the only thing the second can
/// be failing on is the bound — `E0277`, *required by a bound in
/// `validate_geometric`*, `CertifiedEnclosure` not implemented for
/// `Dual<f64>`.
pub fn validate_geometric<
    T: geom_core::Decide + geom_core::CertifiedBounds + crate::props::AtRestPolicy,
>(
    body: &Body<T>,
    tol: Tol,
) -> Result<(), Vec<ValidationError>> {
    validate_geometric_certificate(body, tol).map(|_| ())
}

/// **[`validate_geometric`], handing back the enclosure its check 7
/// derived** — the same pass, the same verdicts, one certified
/// quadrature.
///
/// A body that is gated at rest and then measured used to pay two
/// certified quadratures for one number: check 7 computes a full
/// [`crate::MassProperties`] to decide the +V invariant and drops it,
/// so a caller that also wants the enclosure runs the identical
/// computation again. This door returns what the gate computed.
///
/// **A SIGN, and the number on request.** What comes back is a
/// [`crate::SignCertificate`]: the enclosure `plus_v_decide`
/// decided on, refined exactly as far as THIS check's certification
/// needed and no further. There is no volume to read off it, by
/// construction — a quadrature stopped at the round its caller was
/// finished has not computed one — and
/// [`crate::SignCertificate::refine_to_target`] is where a caller who
/// wants the number asks for it, paying only the rounds that were not
/// already run.
///
/// **THE value, not a second one.** That continuation is bit-identical
/// in all four fields to [`crate::mass_properties`] on the same body
/// at the same `tol`, and that is a fact about identity rather than
/// about agreement: this door's certified quadrature and the
/// measurement door's lane quadrature are the same computation for
/// every scalar that can reach here (the measurement door's lane IS
/// `quad_lane::cut_face_rounds`), against the same
/// `Band::linear(tol)`, over the same face-arena order, over the same
/// rounds — a face left open at round `k` resumes at `k + 1`, and the
/// lanes' rounds are independent recomputations, so a window changes
/// no arithmetic.
///
/// **What is evidence for that, and at which scalar.** At `f64` the
/// identity is measured on real rational-walled bodies —
/// `sweep`'s `tcost_k3_certificate` compares all four fields as raw
/// bits, and `sign_walk_plus_v` does it over a roster whose
/// schedules run past round 0, where the gate and the continuation
/// genuinely split the rounds between them. At the other certifying
/// scalars it rests on one fact about one value: the measurement door
/// hands its walk [`crate::QuadLane::certified`], whose one field is
/// `quad_lane::cut_face_rounds` — pinned by pointer identity in `props.rs`'s
/// `wiring_rows` — and a scalar with no certification rights cannot
/// construct that value, so it cannot form this call at all. That pin
/// covers the WIRING; it does not re-prove what the quadrature
/// computes, which is the `f64` row's job.
///
/// **A refusing arm returns no certificate**: a refusal carries no
/// blessed enclosure, so the `Err` is the verdict vector exactly as
/// [`validate_geometric`]'s is — same rejections, same typed verdicts,
/// same order.
///
/// # Errors
///
/// As [`validate_geometric`].
pub fn validate_geometric_certificate<
    T: geom_core::Decide + geom_core::CertifiedBounds + crate::props::AtRestPolicy,
>(
    body: &Body<T>,
    tol: Tol,
) -> Result<crate::props::SignCertificate<'_, T>, Vec<ValidationError>> {
    validate_geometric_certificate_declared(body, &[], tol)
}

/// **Tier 3 without its one certifying check** — checks 1–6, 8 and 9,
/// at every [`crate::AtRestPolicy`] scalar with a bracket
/// ([`validate_geometric`]'s two halves; the bound is the policy trait
/// rather than bare `Decide` because check 1 reads the offset-fit seam
/// off it and check 2's carrier lane rides as its supertrait, and
/// `Bounds` because check 1 reads a stored datum's bracket end).
///
/// What a caller gives up by taking this door instead of the composed
/// one is named, not implied: the **+V global orientation invariant**
/// (check 7) does not run, so **this pass says nothing about whether the
/// body's volume is positive, and an inverted body passes it — by
/// design, at every scalar.** It is LESS INFORMATION about a body, not a
/// weaker body: every check it does run is the same check, in the same
/// order, reporting the same errors.
///
/// **Read that as a statement about the DOOR, not about certification
/// rights.** Check 7 has two derivations — the certified quadrature, and
/// a closed form that computes at any scalar with a zero pad — and the
/// split moves the whole check, both derivations, into the certified
/// half. So this door is silent about orientation on a planar body at
/// `f64` exactly as it is at a dual. The alternative, a `Decide`-only
/// closed-form arm living here, is refused deliberately: it would put a
/// certification arm back inside the half whose whole property is having
/// none, which is the mixed-pass shape this split exists to leave
/// behind.
///
/// **Where the sign still lives**, so nothing is lost by accident: the
/// `_structural` passes hold no quadrature lane and still make check 7,
/// through the closed form. A caller that wants the orientation verdict
/// at a scalar this door's composed sibling excludes asks
/// [`validate_pseudomanifold_structural`], [`contact_marks_structural`]
/// or [`crate::mass_properties_structural`] — all three answer at a
/// dual, and on a closed-form body all three still say `NegativeVolume`.
/// `topo/tests/geometric_cube.rs`'s
/// `the_structural_half_does_not_judge_orientation_at_any_scalar` is the
/// three verdicts side by side.
///
/// **So the `_structural` suffix means two things across the six doors
/// that carry it**, and a caller reads which at the door: here (and at
/// [`validate_geometric_structural_declared`]) check 7 is NOT MADE —
/// `Ok` on a body whose volume the closed form cannot compute — while
/// at [`validate_pseudomanifold_structural`], [`contact_marks_structural`],
/// [`crate::mass_properties_structural`] and
/// [`crate::classify_shells_structural`] it is made through the closed
/// form and refuses typed (`VolumeUncomputable`) on the same body. One
/// suffix, two shapes; the row that asks whether they should be one is
/// `work/atrest/structural-suffix-means-two-things-across-the-six-doors.md`.
///
/// # Errors
///
/// As [`validate_geometric`], less [`ValidationError::NegativeVolume`]
/// and [`ValidationError::VolumeUncomputable`].
pub fn validate_geometric_structural<
    T: geom_core::Decide + geom_core::Bounds + crate::props::AtRestPolicy,
>(
    body: &Body<T>,
    tol: Tol,
) -> Result<(), Vec<ValidationError>> {
    validate_geometric_structural_declared(body, &[], tol)
}

/// [`validate_geometric_structural`] with the body's declared contacts
/// in hand — [`validate_geometric_declared`]'s structural half.
///
/// # Errors
///
/// As [`validate_geometric_structural`].
pub fn validate_geometric_structural_declared<
    T: geom_core::Decide + geom_core::Bounds + crate::props::AtRestPolicy,
>(
    body: &Body<T>,
    declarations: &[DeclaredContact],
    tol: Tol,
) -> Result<(), Vec<ValidationError>> {
    structural_declared_via(body, declarations, tol, None)
}

/// [`validate_geometric_structural_declared`] with check 2's plane ×
/// NURBS lane taken as an argument — the shared body of the structural
/// half and of the composed entry's first phase.
///
/// Private, and for the same reason
/// [`validate_geometric_certified`] is: the two public doors differ in
/// exactly what they are entitled to claim, and letting a caller pick
/// the argument would let it claim more than its bound allows.
fn structural_declared_via<
    T: geom_core::Decide + geom_core::Bounds + crate::props::AtRestPolicy,
>(
    body: &Body<T>,
    declarations: &[DeclaredContact],
    tol: Tol,
    nurbs_lane: Option<geom_brep::NurbsLane<'_, T>>,
) -> Result<(), Vec<ValidationError>> {
    // Coarse gate: structural tiers first, verbatim.
    validate_closed(body)?;

    let band = match Band::linear(tol) {
        Ok(band) => band,
        Err(error) => return Err(vec![ValidationError::Band { error }]),
    };
    let mut marks = slotmap::SecondaryMap::new();
    // No certificate: this door does not make check 7, and `None` says
    // exactly that rather than an empty verdict standing in for one.
    let (errors, _) = tier3_local_checks_marked(
        body,
        declarations,
        band,
        &mut marks,
        tol,
        PlusVCheck::NotMade,
        nurbs_lane,
        <T as crate::props::AtRestPolicy>::offset_fit_lane(),
    );
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// **Tier 3's check 7 alone** — the +V global orientation invariant,
/// at a scalar with certification rights.
///
/// Private, and that is the guarantee: no caller can take the
/// certified half without the structural one, so no body is ever
/// blessed by a volume claim while its geometry went unchecked. The
/// composed entry is the only way in, and its `?` is what puts the two
/// in the right order.
fn validate_geometric_certified<T: geom_core::Decide + geom_core::CertifiedBounds>(
    body: &Body<T>,
    tol: Tol,
) -> Result<crate::props::SignCertificate<'_, T>, Vec<ValidationError>> {
    let band = match Band::linear(tol) {
        Ok(band) => band,
        Err(error) => return Err(vec![ValidationError::Band { error }]),
    };
    plus_v_by_sign(body, band, tol, Some(crate::props::QuadLane::certified()))
}

/// **Check 7, the whole of it, at SIGN level** — one walk per solid
/// through the lane the door holds, each stopped at the round where
/// that solid's enclosure's sign stops being in doubt, and the body
/// certificate those walks assemble to.
///
/// Every door that makes check 7 makes it here — [`validate_geometric`]
/// with [`crate::QuadLane::certified`], the tier-3′ battery with
/// whatever lane its door holds — so no body is admitted by one tier-3
/// door and refused by another on check 7 at the same LANE. Doors that
/// hold DIFFERENT lanes can still disagree, and that is the lanes
/// differing rather than the check: at `f64`, [`validate_geometric`]
/// (the certified quadrature) admits a rational-walled body that
/// [`validate_pseudomanifold_structural`] (no lane — the closed form)
/// refuses `VolumeUncomputable`, because the closed form refuses typed
/// on every face that needed the quadrature, exactly as the reporting
/// read over the same lane does.
///
/// ONE walk per solid, held and then handed on: the check decides on
/// these objects and the caller receives the body certificate they
/// assemble to, continuable from there by a caller who wants the
/// number ([`crate::SignCertificate::refine_to_target`]). So a body of
/// several solids pays one read of each face, never a further
/// arena-wide one.
///
/// ONE decision per solid, too, and that is load-bearing rather than
/// tidy: each walk stops on the verdict it returns, so no second reading
/// of a different round's enclosure can disagree with the round it
/// stopped at, and the check's predicates are metered once per round
/// rather than twice.
///
/// **Check 7 is clean or it is not, and the type says which**: `Ok` is
/// the body certificate — every solid derived and passed, so the parts
/// partition the face arena, which [`crate::SignCertificate::assembled`]
/// asserts — and `Err` is check 7's verdicts, never empty (it is built
/// only when one was pushed). There is no state where the check came
/// back clean without a certificate to hand on.
fn plus_v_by_sign<'b, T: geom_core::Decide>(
    body: &'b Body<T>,
    band: Band,
    tol: Tol,
    quad: Option<crate::props::QuadLane<T>>,
) -> Result<crate::props::SignCertificate<'b, T>, Vec<ValidationError>> {
    let mut errors = Vec::new();
    let mut parts = Vec::new();
    for (solid, faces) in check7_subjects(body) {
        match crate::props::sign_walk(
            body,
            &faces,
            band,
            tol,
            quad,
            |e| match plus_v_decide(e, band) {
                PlusVOutcome::Pass => Some(PlusVVerdict::Pass),
                PlusVOutcome::Refuse => Some(PlusVVerdict::Refuse),
                PlusVOutcome::Undecided => None,
            },
            |refusal| plus_v_at_target(PlusVOutcome::Undecided, refusal),
        ) {
            Ok((verdict, certificate)) => {
                errors.extend(plus_v_errors(solid, &verdict));
                parts.push(certificate);
            }
            Err(source) => errors.push(ValidationError::VolumeUncomputable { solid, source }),
        }
    }
    if errors.is_empty() {
        Ok(crate::props::SignCertificate::assembled(
            body, band, tol, quad, parts,
        ))
    } else {
        Err(errors)
    }
}

/// The certificate a CLEAN tier-3 verdict implies.
///
/// INVARIANT: check 7 either hands back a certificate or puts its
/// verdicts in the vector ([`plus_v_by_sign`]'s `Result`), and every door
/// that reaches here gates this call on its own empty verdict vector,
/// so an empty verdict and an absent certificate cannot co-occur. The
/// other state is a bug in the composition above, not a reachable input
/// — D9's bug-state half, announced rather than papered over with a
/// fabricated value.
fn certificate_of_a_clean_verdict<C>(certificate: Option<C>) -> C {
    match certificate {
        Some(certificate) => certificate,
        None => unreachable!(
            "a clean tier-3 verdict with no certificate: check 7 is gated on a clean \
             battery and reports its own refusal as VolumeUncomputable"
        ),
    }
}

/// **Check 7's subject**: one entry per solid of
/// `body`, carrying that solid's faces in FACE-ARENA order
/// ([`Body::faces_of_solid`]).
///
/// The subject is the solid because the invariant is about a solid. A
/// body's total is a sum over its solids, and a sum hides a sign.
///
/// **A body holding ONE solid hands back the face arena itself**, in
/// arena order and entire — so the per-solid read IS the whole-body
/// read, term for term, round for round and bit for bit, and the
/// overwhelmingly common body pays nothing for the check's subject
/// having moved. That is an EQUIVALENCE and not a special case, and
/// tier 1 is what makes it one, by four checks run before this one:
/// [`ValidationError::DanglingTopology`] refuses a `Shell::solid` or a
/// `Face::shell` that does not resolve,
/// [`ValidationError::OrphanEntity`] refuses a shell no solid lists
/// and a face no shell lists, [`ValidationError::MultiplyOwned`]
/// refuses either listed twice, and
/// [`ValidationError::BackPointerMismatch`] refuses a stored
/// back-pointer that disagrees with the owner. Together: the solids
/// partition the shells and the shells partition the faces, so
/// `faces_of_solid`'s back-pointer selection and a `Solid::shells`
/// walk name the same set, and over one solid that set is the arena.
///
/// **What is strictly NEEDED for the one-solid case is narrower than
/// that.** [`ValidationError::DanglingTopology`] on `Shell::solid` and
/// on `Face::shell` alone already forces every face of the arena to
/// resolve to the one solid there is, which is the whole of what makes
/// `faces_of_solid` hand the arena back entire. The other three are
/// what the caller actually buys, and they are what carries the
/// statement from a covering to a PARTITION, which is what a body of
/// several solids needs and what [`crate::SignCertificate::assembled`]
/// asserts.
///
/// **The gate that supplies the premise is tier 2, not the battery's
/// own `if`.** Every door that reaches check 7 opens with
/// `validate_closed(body)?`, which runs tier 1 first:
/// `structural_declared_via` — the half [`validate_geometric`] and its
/// `_declared` twin compose in front of `validate_geometric_certified`
/// — `contact_marks_declared`, and `pseudomanifold_certificate_via`.
/// The battery's `if errors.is_empty()` gates check 7 on the battery's
/// OWN checks 1–6, which is a different premise and not this one.
fn check7_subjects<T: Real>(body: &Body<T>) -> Vec<(SolidKey, Vec<FaceKey>)> {
    body.solids
        .iter()
        .map(|(solid_key, _)| {
            let Some(faces) = body.faces_of_solid(solid_key) else {
                unreachable!("a key read out of the solid arena does not resolve in it")
            };
            (solid_key, faces)
        })
        .collect()
}

/// [`validate_geometric`] with the body's **declared contacts** in
/// hand — the door a body carrying a declared cusp or slit validates
/// through.
///
/// Everything about tier 3 is unchanged except the one arm that must
/// read a declaration: check 4's material arm, where a wedge of 0 or
/// 2π is legal iff the C7 `Tangent` vocabulary asserts the contact on
/// that face pair (D1's ratified second-order arm). Declarations are
/// an INPUT here rather than body state because that is what they are
/// — a claim a modeler makes about geometry, carried by the recipe
/// layer and never inferred from values — and it is why
/// [`validate_geometric`] passing an empty slice is not a shortcut:
/// a body nobody declared anything about genuinely has an undeclared
/// cusp if it has a cusp at all.
///
/// A declaration this pass does not consult costs nothing and asserts
/// nothing: no arm reads `Rest`, and a declaration with no matching
/// geometry is the census's business (`StaleContactDeclaration`), not
/// this pass's.
///
/// # Errors
///
/// As [`validate_geometric`].
pub fn validate_geometric_declared<
    T: geom_core::Decide + geom_core::CertifiedBounds + crate::props::AtRestPolicy,
>(
    body: &Body<T>,
    declarations: &[DeclaredContact],
    tol: Tol,
) -> Result<(), Vec<ValidationError>> {
    validate_geometric_certificate_declared(body, declarations, tol).map(|_| ())
}

/// [`validate_geometric_certificate`] with the body's declared
/// contacts in hand — [`validate_geometric_declared`]'s certificate
/// form, and the one place the two halves are composed.
///
/// The `?` between them is the composition
/// [`validate_geometric_certified`]'s privacy exists to enforce: no
/// caller takes the certified half without the structural one, so no
/// body is blessed by a volume claim while its geometry went
/// unchecked. Returning the certificate does not move that seam — the
/// structural half still runs first and still short-circuits, and it
/// makes no certificate of its own to return.
///
/// # Errors
///
/// As [`validate_geometric_declared`].
pub fn validate_geometric_certificate_declared<
    'b,
    T: geom_core::Decide + geom_core::CertifiedBounds + crate::props::AtRestPolicy,
>(
    body: &'b Body<T>,
    declarations: &[DeclaredContact],
    tol: Tol,
) -> Result<crate::props::SignCertificate<'b, T>, Vec<ValidationError>> {
    // The structural half runs WITH check 2's plane x NURBS lane
    // injected, which is what keeps this composed door re-deriving the
    // M7-8 certificate class at rest; the public structural door,
    // whose bound cannot name the lane, runs without it.
    structural_declared_via(
        body,
        declarations,
        tol,
        Some(&geom_brep::plane_nurbs_limbs::<T>),
    )?;
    validate_geometric_certified(body, tol)
}

/// Tier 3's local check battery (checks 1–6 + the +V invariant, check
/// 7), shared verbatim between [`validate_pseudomanifold`] and
/// [`contact_marks`] (M3 PR 6a: the tier-3′ validator runs the SAME
/// local passes — extraction, not copy-paste). Assumes the tier-1/2
/// coarse gate already passed.
///
/// Check 7 is made through `quad_lane` at SIGN level
/// ([`PlusVCheck::Through`]) — the certified quadrature from a door
/// whose bound names the right, the closed form from a `_structural`
/// door — which is what keeps this battery callable at every
/// [`crate::AtRestPolicy`] scalar, a dual included, with the certified
/// door and its `_structural` twin differing in exactly the lane they
/// hand in.
///
/// `nurbs_lane` is check 2's plane × NURBS derivation, taken as an
/// argument for the same reason and with the same discipline: the
/// `_structural` doors hand `None` and the certified doors hand the
/// certified body, and what a `None` costs is written at
/// [`validate_pseudomanifold_structural`].
pub(crate) fn tier3_local_checks<
    'b,
    T: geom_core::Decide + geom_core::Bounds + crate::props::AtRestPolicy,
>(
    body: &'b Body<T>,
    declarations: &[DeclaredContact],
    band: Band,
    tol: Tol,
    nurbs_lane: Option<geom_brep::NurbsLane<'_, T>>,
    quad_lane: Option<crate::props::QuadLane<T>>,
) -> (
    Vec<ValidationError>,
    Option<crate::props::SignCertificate<'b, T>>,
) {
    let mut marks = slotmap::SecondaryMap::new();
    tier3_local_checks_marked(
        body,
        declarations,
        band,
        &mut marks,
        tol,
        PlusVCheck::Through(quad_lane),
        nurbs_lane,
        <T as crate::props::AtRestPolicy>::offset_fit_lane(),
    )
}

/// **What check 7 has decided, and whether refining could change it.**
///
/// The +V invariant reads a volume ENCLOSURE and refuses only on a
/// definite disagreement, so its verdict on a bracket `[lo, hi]` is
/// settled as soon as that bracket excludes zero — and refinement only
/// tightens a bracket, never moves the truth out of it:
///
/// - `hi` definitely negative ⇒ the body's volume is `≤ hi < 0`, and
///   no finer round produces an upper end above the volume. REFUSE.
/// - `lo` definitely positive ⇒ the body's volume is `≥ lo > 0`, so
///   every finer round's upper end is above `lo` too and none of them
///   can read definitely negative. PASS.
/// - otherwise the bracket straddles zero (or its margin is in-band),
///   and a finer round may still decide it.
///
/// The two ends are read under two names, because they are two
/// questions: `positive_volume` is the refusal this check has always
/// made, on the same quantity and the same lever it always made it on;
/// `positive_volume_enclosure` is the question the coupling to the
/// reporting target used to leave unasked — *is the sign already
/// certain?* — and it is the one an orientation gate actually consumes.
///
/// The lever is the surface area (`V/A`, a length: the mean boundary
/// displacement the volume defect corresponds to). Closed-form bodies
/// have `pad = 0.0`, so `lo` and `hi` are the volume itself and the
/// refusing margin is bit-identical to the pre-PR-11 one.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum PlusVOutcome {
    /// The volume is definitely negative: orientation corruption.
    Refuse,
    /// The volume is definitely positive: the invariant holds, and no
    /// finer round can change that.
    Pass,
    /// The enclosure does not decide. At the reporting target this is
    /// a PASS — only a definite disagreement refuses — but before it,
    /// it is a reason to refine.
    Undecided,
}

/// **Check 7's whole verdict**, which is what a sign-level walk stops
/// on — never [`PlusVOutcome`], which is a reading of ONE enclosure and
/// has an arm that decides nothing.
///
/// The difference is the bug this shape exists to make unwritable. An
/// undecided enclosure is a reason to refine, and at the reporting
/// target it is a PASS — but only for a body the quadrature could
/// actually have measured. A body whose sign never became definite AND
/// whose schedule ran out has not been validated at all, and passing
/// it is the false ACCEPTANCE that mirrors the false refusal this unit
/// removed. So the walk's return type carries no undecided arm: the
/// enclosure reading that yields one is turned into a verdict at the
/// moment the schedule ends, with the outstanding refusal in hand.
#[derive(Clone, Debug)]
pub(crate) enum PlusVVerdict {
    /// The invariant holds.
    Pass,
    /// The volume is definitely negative: orientation corruption.
    Refuse,
    /// Check 7 could not be made: the sign was still undecided when
    /// the certified quadrature ran out of schedule, so the body's
    /// volume is neither measurable nor sign-certifiable at this ε.
    /// The payload is the refusal a target-level reading earns, which
    /// is the refusal the reporting door makes on the same body.
    Uncomputable(crate::props::MassPropsError),
}

fn plus_v_decide<T: geom_core::Decide>(
    enclosure: crate::props::VolumeEnclosure<T>,
    band: Band,
) -> PlusVOutcome {
    let lever = enclosure.surface_area;
    if let Ok(Sign::Negative) = decide(
        "positive_volume",
        Margin::over_lever(enclosure.volume_hi, lever),
        band,
    ) {
        return PlusVOutcome::Refuse;
    }
    if let Ok(Sign::Positive) = decide(
        "positive_volume_enclosure",
        Margin::over_lever(enclosure.volume_lo, lever),
        band,
    ) {
        return PlusVOutcome::Pass;
    }
    PlusVOutcome::Undecided
}

/// **What an enclosure reading means once there is nothing left to
/// refine** — the one place the undecided arm is resolved: the
/// sign-level walk's `last_word`, handed the outstanding target
/// refusal.
fn plus_v_at_target(
    outcome: PlusVOutcome,
    refusal: Option<crate::props::MassPropsError>,
) -> PlusVVerdict {
    match (outcome, refusal) {
        (PlusVOutcome::Refuse, _) => PlusVVerdict::Refuse,
        // Undecided with the schedule run out is NOT a pass: the
        // quadrature never produced an enclosure tight enough to
        // decide, and the body is exactly as unvalidatable as the
        // reporting door says it is.
        (PlusVOutcome::Undecided, Some(source)) => PlusVVerdict::Uncomputable(source),
        (PlusVOutcome::Pass | PlusVOutcome::Undecided, _) => PlusVVerdict::Pass,
    }
}

/// Check 7's verdict as the tier's error vector.
fn plus_v_errors(solid: SolidKey, verdict: &PlusVVerdict) -> Vec<ValidationError> {
    match verdict {
        PlusVVerdict::Pass => Vec::new(),
        PlusVVerdict::Refuse => vec![ValidationError::NegativeVolume { solid }],
        PlusVVerdict::Uncomputable(source) => vec![ValidationError::VolumeUncomputable {
            solid,
            source: source.clone(),
        }],
    }
}

/// Whether the declarations assert a **`Tangent`** contact on this
/// (unordered) face pair — the one read the material-wedge arm makes.
///
/// `Rest` never answers yes, and that is D1's "the arm admits no
/// laminae" written as code: a conformal declaration over a patch
/// asserts the wrong class for a curve locus, so it cannot legalize a
/// wedge end. Nor does absence ever certify anything: an empty
/// declaration list refuses every cusp, which is what makes
/// [`validate_geometric`]'s signature honest.
fn declares_tangent_contact(declarations: &[DeclaredContact], a: FaceKey, b: FaceKey) -> bool {
    declarations.iter().any(|d| {
        d.class == crate::contact::ContactClass::Tangent
            && ((d.a == a && d.b == b) || (d.a == b && d.b == a))
    })
}

/// What check 4's **material arm** concluded about one edge.
///
/// One value rather than a pair of flags, because the outcomes are
/// mutually exclusive BY CONSTRUCTION and this type is where that is
/// enforced: a collapsed κ_rel yields no end, so an edge can never
/// earn both the lamina refusal and the undeclared-wedge refusal.
/// (The earlier shape carried `lamina: bool` beside
/// `material: Option<MaterialWedge>` and documented a PRECEDENCE
/// between them — a ranking that could never fire, and a reader
/// cannot tell a dead branch from a live one. The exclusivity is
/// structural now, so there is no ranking left to state.)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum MaterialArmOutcome {
    /// One material wedge for the whole edge (every sample agreed).
    Wedge(MaterialWedge),
    /// Opposed material sides whose jets **osculate** — conformal
    /// contact along the locus, refused as [`ValidationError::LaminaWedge`].
    Lamina,
    /// The samples did not deliver ONE verdict: the named predicate
    /// classified differently at different samples along the edge (or,
    /// unreachably, at none of them). Escalated, never silent — see
    /// [`material_arm_outcome`].
    Split {
        /// The predicate whose per-sample verdicts disagreed.
        predicate: &'static str,
    },
}

/// The material arm's fold: the flags one edge's sample loop
/// accumulates, resolved into the ONE outcome that edge earns.
///
/// **Two callers accumulate those flags on two different sample
/// schedules, deliberately.** This pass walks an EDGE — an open arc
/// whose endpoints are vertices other rules already classify — so it
/// samples the interior, `1..CERT_SAMPLES-1`. `boolean::rim_wedge`
/// walks a cross-operand RIM, a closed circle with no endpoint to
/// exclude, so every station is interior to it and it takes all
/// `CERT_SAMPLES` at uniform phase. The fold itself is schedule-blind —
/// it reads flags, not samples — which is what lets one function serve
/// both; the divergence is in what "interior" means for an arc versus a
/// circle, and it is named at both ends so neither can drift into
/// looking like the other's bug.
///
/// Total and pure, which is the point. Two of its input states —
/// a pairing that split (`aligned == opposed`) and a wedge end that
/// split (`side_mixed`) — are not known to be reachable through
/// certified geometry, and an unreachable state's handling can only be
/// pinned by CALLING the fold with it; `material_arm_split_states_escalate`
/// does exactly that.
///
/// Both split states **escalate** rather than fall silent. Silence
/// there would validate an undeclared cusp clean on an edge whose own
/// samples disagreed about which material configuration it is — the
/// opposite of the refusal the arm exists for. This is the same
/// posture the sibling decisions already take one order up
/// (`material_wedge_side`'s unreachable `Zero` is announced as
/// `Invalid` rather than guessed), and it is NOT the first-order
/// pass's mixed transverse/smooth exemption: that one exempts an edge
/// from a DEMAND (carry an intrinsic description), while this one
/// would exempt it from a REFUSAL.
pub(crate) fn material_arm_outcome(
    aligned: bool,
    opposed: bool,
    jet_determinate: bool,
    side: Option<MaterialWedge>,
    side_mixed: bool,
) -> MaterialArmOutcome {
    match (aligned, opposed) {
        // Every sample: one material side. The two faces continue one
        // another and the wedge is the legal π seam — a verdict the
        // second-order margin has no say in.
        (true, false) => MaterialArmOutcome::Wedge(MaterialWedge::Seam),
        // Every sample: opposed material sides. The wedge is one of
        // the two ends, and which one is the second-order question.
        (false, true) => {
            if !jet_determinate {
                MaterialArmOutcome::Lamina
            } else {
                match side {
                    Some(wedge) if !side_mixed => MaterialArmOutcome::Wedge(wedge),
                    // Split, or (unreachably, with CERT_SAMPLES = 9)
                    // no sample at all: both are "the end did not
                    // resolve", and neither may pass as legal.
                    _ => MaterialArmOutcome::Split {
                        predicate: "material_cusp_side",
                    },
                }
            }
        }
        // The pairing itself split across samples, or classified at no
        // sample: no material configuration is established for this
        // edge at all.
        _ => MaterialArmOutcome::Split {
            predicate: "material_wedge_side",
        },
    }
}

/// The material arm's REFUSAL, read off its outcome: the second half
/// of check 4's material decision, and pure for the same reason the
/// fold is — the states that matter most here are the ones no fixture
/// can force, so the only way to pin what each one emits is to call
/// this with it (`material_arm_error_table`).
///
/// `declared` is whether the edge's face pair carries a `Tangent`
/// contact declaration ([`declares_tangent_contact`]) — the ONE thing
/// the wedge ends consult, and the reason `None` here is a legality
/// verdict rather than an absence.
pub(crate) fn material_arm_error(
    outcome: Option<MaterialArmOutcome>,
    edge: EdgeKey,
    declared: bool,
    band: Band,
) -> Option<ValidationError> {
    match outcome? {
        // Conformal contact: refused, and no declaration is consulted
        // because none would cure it.
        MaterialArmOutcome::Lamina => Some(ValidationError::LaminaWedge { edge }),
        // The samples disagreed: escalate, naming the predicate that
        // split. Never silence — see `material_arm_outcome`.
        MaterialArmOutcome::Split { predicate } => Some(ValidationError::SliverDihedral {
            edge,
            cause: Indeterminate {
                margin: geom_core::MarginDiag::Invalid,
                band,
                predicate: Some(predicate),
            },
        }),
        // The two ends of the wedge range are legal exactly where
        // declared; the seam and a transverse wedge need nothing.
        MaterialArmOutcome::Wedge(wedge) if wedge.is_declared_arm() && !declared => {
            Some(ValidationError::UndeclaredCusp { edge, wedge })
        }
        MaterialArmOutcome::Wedge(_) => None,
    }
}

/// The per-edge tier-3 contact MARKS at rest (OQ7 level (i), M5 PR 9):
/// runs the structural coarse gate, then the tier-3 battery, and
/// returns the recorded per-edge [`ContactMark`]s (marks are only
/// meaningful on a valid body).
///
/// **This pass makes every check in ONE call**, which is why it is not
/// [`validate_geometric`] with a second return value: the whole
/// nine-check battery runs in one call, check 7 included through the
/// certified quadrature ([`crate::QuadLane::certified`]) and check 2
/// through the certified plane × NURBS lane, so its bound names the
/// certification right. Its `Err` is the battery's vector and differs
/// from the composed door's in one stated way: its check 7 is gated on
/// checks 1-6 only, where the composed door gates on the whole
/// structural half. [`contact_marks_structural`] is the same pass
/// holding neither lane, at every scalar with a bracket.
///
/// # Errors
///
/// The tier-1/2 report, else the tier-3 battery's vector — the same
/// [`ValidationError`]s in the same documented order.
pub fn contact_marks<
    T: geom_core::Decide + geom_core::CertifiedBounds + crate::props::AtRestPolicy,
>(
    body: &Body<T>,
    tol: Tol,
) -> Result<slotmap::SecondaryMap<EdgeKey, ContactMark>, Vec<ValidationError>> {
    contact_marks_declared(body, &[], tol)
}

/// **[`contact_marks`] holding NO lane** — the same pass at every
/// [`crate::AtRestPolicy`] scalar with a bracket, a
/// [`Dual`](geom_core::Dual) included, and it says less at every
/// scalar, for the reasons [`validate_pseudomanifold_structural`]
/// states: check 7 runs through the closed form alone, and check 2
/// makes no claim about an M7-8 edge at all.
///
/// # Errors
///
/// As [`contact_marks`], less the check-2 verdicts on M7-8 edges and
/// plus the closed form's typed refusal.
pub fn contact_marks_structural<
    T: geom_core::Decide + geom_core::Bounds + crate::props::AtRestPolicy,
>(
    body: &Body<T>,
    tol: Tol,
) -> Result<slotmap::SecondaryMap<EdgeKey, ContactMark>, Vec<ValidationError>> {
    contact_marks_declared_structural(body, &[], tol)
}

/// [`contact_marks`] with the body's declared contacts in hand — check
/// 4's material arm reads them, for [`validate_geometric_declared`]'s
/// reason.
///
/// # Errors
///
/// As [`contact_marks`], with check 4's material arm reading the
/// declarations.
pub fn contact_marks_declared<
    T: geom_core::Decide + geom_core::CertifiedBounds + crate::props::AtRestPolicy,
>(
    body: &Body<T>,
    declarations: &[DeclaredContact],
    tol: Tol,
) -> Result<slotmap::SecondaryMap<EdgeKey, ContactMark>, Vec<ValidationError>> {
    contact_marks_declared_via(
        body,
        declarations,
        tol,
        Some(&geom_brep::plane_nurbs_limbs::<T>),
        Some(crate::props::QuadLane::certified()),
    )
}

/// [`contact_marks_structural`]'s declared form.
///
/// # Errors
///
/// As [`contact_marks_structural`], with check 4's material arm reading
/// the declarations.
pub fn contact_marks_declared_structural<
    T: geom_core::Decide + geom_core::Bounds + crate::props::AtRestPolicy,
>(
    body: &Body<T>,
    declarations: &[DeclaredContact],
    tol: Tol,
) -> Result<slotmap::SecondaryMap<EdgeKey, ContactMark>, Vec<ValidationError>> {
    contact_marks_declared_via(body, declarations, tol, None, None)
}

/// The marks pass with its two lanes as arguments — the shared body of
/// the certified door and its `_structural` twin.
fn contact_marks_declared_via<
    T: geom_core::Decide + geom_core::Bounds + crate::props::AtRestPolicy,
>(
    body: &Body<T>,
    declarations: &[DeclaredContact],
    tol: Tol,
    nurbs_lane: Option<geom_brep::NurbsLane<'_, T>>,
    quad_lane: Option<crate::props::QuadLane<T>>,
) -> Result<slotmap::SecondaryMap<EdgeKey, ContactMark>, Vec<ValidationError>> {
    validate_closed(body)?;
    let band = match Band::linear(tol) {
        Ok(band) => band,
        Err(error) => return Err(vec![ValidationError::Band { error }]),
    };
    let mut marks = slotmap::SecondaryMap::new();
    // This pass's product is the MARKS channel; its check-7 certificate
    // has no consumer here and is dropped at the one site that could
    // hand it on (`crate::validate_geometric_certificate` and
    // `crate::validate_pseudomanifold_certificate` are the returning
    // doors).
    let (errors, _) = tier3_local_checks_marked(
        body,
        declarations,
        band,
        &mut marks,
        tol,
        PlusVCheck::Through(quad_lane),
        nurbs_lane,
        <T as crate::props::AtRestPolicy>::offset_fit_lane(),
    );
    if errors.is_empty() {
        Ok(marks)
    } else {
        Err(errors)
    }
}

/// **Whether the battery makes check 7, and through which lane** — the
/// one check of the battery that CERTIFIES is a parameter rather than a
/// dispatch, so a caller that names it says which derivation it meant.
///
/// [`PlusVCheck::NotMade`] is [`validate_geometric_structural`]'s answer
/// and is not a refusal — the battery run without a check that caller
/// does not make, which returns no certificate. Every other door makes
/// check 7 through [`plus_v_by_sign`] with the lane it holds, so the
/// battery and [`validate_geometric`] decide the same question the same
/// way and differ only in which lane they could name.
#[derive(Clone, Copy)]
pub(crate) enum PlusVCheck<T: geom_core::Decide> {
    /// Check 7 is not made.
    NotMade,
    /// Check 7 is made at SIGN level through this lane — `None` being
    /// the closed form, which refuses typed on a face that needed the
    /// quadrature.
    Through(Option<crate::props::QuadLane<T>>),
}

/// **Check 1's DATUM verdicts on one analytic surface**, in the order
/// they are asked, the second gated on the first having found nothing.
/// (The torus's ring half `R > r` is a decided geometric margin and is
/// asked at the call site, after these.)
///
/// 1. **Poison** ([`poisoned_datums`]): `geom`'s totality-and-poison
///    rule is about no particular surface kind, so a stored datum that
///    is not a number is named here exactly as a poisoned net is named
///    by the `Nurbs` arm. Every such datum is named, and the convention
///    is not read over any of them — a margin of poison is poison.
/// 2. **The convention, on REPRESENTABILITY**: each of
///    [`geom::Surface::representability_margins`] must be definitely
///    positive. A datum outside its variant's convention describes no
///    2-manifold a face can bound. The first failing margin is named,
///    with the end of the convention it fails. Why this is a bracket
///    read that decides nothing, unmetered: the `Bounds` scope rule's
///    entry for this file (`geom_core::real`, `bounds_allowlist`, the
///    2026-09-02 certified at-rest entry) — one home, not restated here.
///
/// A SOLE bracket bound, deliberately: nothing here decides.
fn analytic_datum_verdicts<T: geom_core::Bounds>(
    face: FaceKey,
    surface: &Surface<T>,
) -> Vec<ValidationError> {
    let kind = geom_brep::SurfaceKind::of(surface);
    let poisoned = poisoned_datums(surface);
    if !poisoned.is_empty() {
        return poisoned
            .into_iter()
            .map(|datum| ValidationError::PoisonedSurfaceDatum { face, kind, datum })
            .collect();
    }
    surface
        .representability_margins()
        .into_iter()
        .flatten()
        .find(|m| {
            !matches!(
                geom_core::Bounds::lo(m.margin).partial_cmp(&0.0),
                Some(core::cmp::Ordering::Greater)
            )
        })
        .map(|m| ValidationError::UnrepresentableSurfaceDatum {
            face,
            kind,
            datum: m.datum,
            end: m.end,
        })
        .into_iter()
        .collect()
}

/// **Check 1's poison read of an analytic surface**: every stored datum
/// that is not a finite number at this scalar, and a plane `normal`
/// that is the zero vector, in field order. Empty for a surface whose
/// every datum is a number — and for the spline kinds, whose datum is a
/// net that check 1 reads through [`geom::NetState`] instead.
///
/// Asked through the value channel ([`geom_core::is_finite_length`]),
/// with no bracket read and no threshold: whether a stored number is a
/// number is not a decision about geometry. The fields are destructured
/// without `..`, so a datum a variant gains is a compile error here
/// rather than a field this read silently skips.
///
/// The zero normal is the one direction asked for more than
/// finiteness, because a plane's `normal` IS its chart normal: a zero
/// one collapses `v_ref = normal × u_ref`, and the plane with it. It is
/// asked through [`geom_core::is_zero_length`] under that door's
/// precondition — the length is asked [`geom_core::is_finite_length`]
/// first — so a normal whose components are finite but whose NORM
/// overflows (`(1e200, 0, 0)` at `f64`) is a direction, not the zero
/// vector, and passes. Underflowed normals pass too: not unit, which is
/// conventional and unchecked, but a direction.
///
/// **The two scalars answer "finite" differently on one shape**, and
/// that is `is_finite_length`'s value-channel semantics rather than
/// this read's: an interval ENCLOSURE whose upper end overflowed still
/// contains its truth and answers finite, where the same computation at
/// `f64` is `∞` and is refused. A stored datum reaches the interval
/// scalar through `Interval::from_f64` (NaN and `±∞` become NaI, which
/// is refused), so the difference bites only a datum that was COMPUTED
/// at interval type; filed as
/// `work/germ/the-tube-and-radius-guards-decide-on-the-band-where-check-1-reads-lo`.
fn poisoned_datums<T: Real>(surface: &Surface<T>) -> Vec<geom::SurfaceDatum> {
    use geom::SurfaceDatum as D;
    use geom_core::is_finite_length as finite;
    let point = |p: &geom_core::Point3<T>| finite(p.x) && finite(p.y) && finite(p.z);
    let vector = |v: &geom_core::Vec3<T>| finite(v.x) && finite(v.y) && finite(v.z);
    let is_zero_normal = |n: &geom_core::Vec3<T>| {
        let len = n.norm();
        finite(len) && geom_core::is_zero_length(len, n.norm_witness())
    };
    let fields: Vec<(D, bool)> = match surface {
        Surface::Plane {
            origin,
            normal,
            u_ref,
        } => vec![
            (D::Origin, point(origin)),
            (D::Normal, vector(normal) && !is_zero_normal(normal)),
            (D::URef, vector(u_ref)),
        ],
        Surface::Cylinder {
            origin,
            axis,
            radius,
            u_ref,
        } => vec![
            (D::Origin, point(origin)),
            (D::Axis, vector(axis)),
            (D::Radius, finite(*radius)),
            (D::URef, vector(u_ref)),
        ],
        Surface::Cone {
            apex,
            axis,
            half_angle,
            u_ref,
        } => vec![
            (D::Apex, point(apex)),
            (D::Axis, vector(axis)),
            (D::HalfAngle, finite(*half_angle)),
            (D::URef, vector(u_ref)),
        ],
        Surface::Sphere {
            center,
            radius,
            axis,
            u_ref,
        } => vec![
            (D::Center, point(center)),
            (D::Radius, finite(*radius)),
            (D::Axis, vector(axis)),
            (D::URef, vector(u_ref)),
        ],
        Surface::Torus {
            center,
            axis,
            major_radius,
            minor_radius,
            u_ref,
        } => vec![
            (D::Center, point(center)),
            (D::Axis, vector(axis)),
            (D::MajorRadius, finite(*major_radius)),
            (D::MinorRadius, finite(*minor_radius)),
            (D::URef, vector(u_ref)),
        ],
        Surface::Nurbs(_) | Surface::Approx(_) => Vec::new(),
    };
    fields
        .into_iter()
        .filter_map(|(datum, is_number)| (!is_number).then_some(datum))
        .collect()
}

/// [`tier3_local_checks`] with the check-4 contact marks KEPT (the
/// same pass — never classifying twice; the mark is the verdict the
/// dihedral/jet loop derives anyway).
///
/// `plus_v` is check 7, handed in rather than dispatched ([`PlusVCheck`]
/// says why): the one check of this battery that CERTIFIES is the one
/// whose availability differs by scalar. What comes back beside the
/// verdicts is the [`crate::SignCertificate`] check 7 decided on —
/// `None` when the check was not made or a subject could not be derived
/// at all, which the vector then says.
///
/// `nurbs_lane` is check 2's second derivation, handed in for the same
/// reason and with the same discipline. The M7-8 carrier class
/// (`Intersection` of a plane and a described NURBS wall) re-derives
/// only through the certified plane × NURBS lane, so a caller that
/// cannot name that lane does not re-derive that class and this
/// battery SKIPS those edges rather than reporting them
/// ([`geom_brep::EdgeCurve::needs_nurbs_lane`] asks the question
/// before the claim is made). Every other carrier class is
/// re-certified identically either way.
///
/// `offset_fit` is check 1's re-derivation door for an `Approx` face
/// ([`geom_brep::OffsetFitLane`]), handed in for a THIRD reason: what
/// its absence means is
/// [`crate::AtRestPolicy::offset_fit_lane`]'s subject, not this
/// caller's rights. `None` is not a skip — the face is reported
/// [`ValidationError::ApproxLaneUnsupported`], because a surface
/// certificate is the one claim this kernel refuses to leave
/// unchecked.
// The eighth parameter is the third INJECTED DERIVATION (`plus_v`,
// `nurbs_lane`, `offset_fit`), and each is a door whose availability
// differs by caller: bundling them into one struct would hide behind a
// single name exactly the thing each parameter is here to make the
// caller state.
#[allow(clippy::too_many_arguments)]
pub(crate) fn tier3_local_checks_marked<
    'b,
    T: geom_core::Decide + geom_core::Bounds + crate::props::AtRestPolicy,
>(
    body: &'b Body<T>,
    declarations: &[DeclaredContact],
    band: Band,
    marks: &mut slotmap::SecondaryMap<EdgeKey, ContactMark>,
    tol: Tol,
    plus_v: PlusVCheck<T>,
    nurbs_lane: Option<geom_brep::NurbsLane<'_, T>>,
    offset_fit: Option<geom_brep::OffsetFitLane<T>>,
) -> (
    Vec<ValidationError>,
    Option<crate::props::SignCertificate<'b, T>>,
) {
    let mut errors = Vec::new();
    let mut certificate = None;

    // ------------------------------------------------------------------
    // Tier 3, check 1: surface implementedness (face-arena order).
    //
    // This is the consumer's described arm for a face's SURFACE, which
    // `geom`'s totality-and-poison rule requires to exist: the three
    // states a control net can be in are `NetState`'s, defined there,
    // and each is answered below; an analytic surface's stored datums
    // are answered by name, poison first.
    //
    // An analytic surface is implemented only inside its datum
    // conventions — a radius that is positive, a cone half-angle inside
    // `(0, π/2)`, and D3's ring convention `R > r > 0` for a torus,
    // whose horn or spindle configuration puts a singular point on the
    // axis that no chart in the tree represents. The check lives HERE,
    // at rest, because that is the one place every door that can mint a
    // surface passes through, whatever each door checks for itself —
    // `sweep::revolve` refuses a horn or spindle at construction,
    // `step-import` reads a torus's two radii verbatim.
    //
    // Checks 3–5 still read these surfaces for their own questions, and
    // a datum refused here may draw their escalations as well; the
    // named refusal is the one reported first.
    // ------------------------------------------------------------------
    for (face_key, face) in body.faces.iter() {
        match body.surfaces.get(face.surface) {
            Some(Surface::Nurbs(payload)) => match payload.net_state() {
                NetState::Placeholder => {
                    errors.push(ValidationError::UncertifiableSurface { face: face_key });
                }
                NetState::Poisoned => {
                    errors.push(ValidationError::PoisonedSurfaceDescription { face: face_key });
                }
                // Real geometry, and the checks that examine it are
                // elsewhere — its seams at check 2, its flux at check
                // 7. Nothing about the payload itself is a tier-3 fact.
                NetState::Described => {}
            },
            // The approximating surface's re-derivation (O5): the
            // two-limb certificate is recomputed from the stored
            // description and fit on EVERY call, and the stored
            // certificate is not read. A grid per face is a real cost
            // where edges pay a line schedule; the alternative —
            // trust-on-read for surfaces only — would make this the one
            // unchecked claim in tier 3.
            //
            // **Classified against the RUN's ε, exactly as every edge
            // carrier is** — never against the surface's own stored
            // tolerance. O3's ratified claim is `≤ ε_precision`, and a
            // mint's parameter is not that claim; see
            // `geom_brep::OffsetFitLane::recertify` for the argument,
            // and for why ε-tightening turning a loosely-minted surface
            // red is D4's blessed behaviour rather than a regression.
            // The witness travels; the value is read once, inside the
            // door, so this site cannot hand it a number of its own.
            Some(Surface::Approx(approx)) => match offset_fit {
                Some(lane) => {
                    if let Err(error) = lane.recertify(approx, tol, band) {
                        errors.push(ValidationError::ApproxCertification {
                            face: face_key,
                            error,
                        });
                    }
                }
                None => {
                    errors.push(ValidationError::ApproxLaneUnsupported { face: face_key });
                }
            },
            // Every analytic kind: its datums first, then its
            // convention (`analytic_datum_verdicts`), then — for the
            // torus — the one convention that relates two datums.
            Some(
                surface @ (Surface::Plane { .. }
                | Surface::Cylinder { .. }
                | Surface::Cone { .. }
                | Surface::Sphere { .. }
                | Surface::Torus { .. }),
            ) => {
                let verdicts = analytic_datum_verdicts(face_key, surface);
                if !verdicts.is_empty() {
                    errors.extend(verdicts);
                    continue;
                }
                // The torus's ring half `R > r` is a GEOMETRIC question
                // — two datums of the body compared, not one against its
                // bound — so it goes through `decide`. It is asked only
                // after `r > 0` has held, because `R − r` metered against
                // a nonpositive `r` would quote it: at `r = −R` the
                // difference reads `2R`, definitely positive.
                if let Surface::Torus {
                    major_radius,
                    minor_radius,
                    ..
                } = surface
                {
                    match decide(
                        "ring_torus_convention",
                        Margin::of(*major_radius - *minor_radius),
                        band,
                    ) {
                        Ok(Sign::Positive) => {}
                        Ok(Sign::Zero | Sign::Negative) => {
                            errors.push(ValidationError::DegenerateTorus { face: face_key });
                        }
                        Err(cause) => {
                            errors.push(ValidationError::DegenerateTorusEscalated {
                                face: face_key,
                                cause,
                            });
                        }
                    }
                }
            }
            // Cascade discipline: a face whose surface key does not
            // resolve is tier 1's `DanglingGeometry`, already reported,
            // and the coarse gate means we never reach here in that
            // case.
            None => {}
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
        // Re-certification takes the lane the CALLER handed in, not one
        // read off the scalar. The bound that admits a scalar to this
        // battery says nothing about the C9 ring the plane × NURBS
        // certificate lives in, which is a right of its own. So a
        // caller that can name the certified lane supplies it and this
        // check runs whole; a caller that cannot makes no claim about
        // an M7-8 edge at all.
        //
        // **Read that at its true width, because it is wider than the
        // class it is about.** `recertify_via` is ONE call and check 2
        // is a whole-edge check: without the lane the description
        // resolver refuses `Unimplemented` BEFORE the endpoint,
        // interval and chart-image checks run, so what a lane-free
        // caller does not get is every check-2 verdict on that edge —
        // a drifted endpoint on an M7-8 edge included — and not merely
        // the plane × NURBS limbs. That is why the skip is a skip and
        // not a report: `Unimplemented` after the fact cannot be told
        // from a genuine failure, and reporting it would name a defect
        // in the body for a fact about the caller.
        //
        // **An imported or minted body of that class must re-derive
        // its certificate at rest exactly as it did at attach time.**
        // That invariant did not move; what moved is which door
        // honours it. Every door whose bound names the right does
        // ([`validate_geometric`], [`validate_pseudomanifold`],
        // [`contact_marks`] and their declared and certificate forms),
        // and `AtRestPolicy`'s certifying arms and `step-import`'s
        // aggregate gate take those. The `_structural` doors do not,
        // and each says so at its own signature.
        //
        // Every other carrier class is re-certified the same way at
        // both doors. Re-certification re-derives; it never trusts the
        // stored certificate.
        let claimable =
            nurbs_lane.is_some() || !curve.needs_nurbs_lane(|k| body.surfaces.get(k).cloned());
        if claimable
            && let Err(error) = curve.recertify_via(
                p_start,
                p_end,
                |k| body.surfaces.get(k).cloned(),
                band,
                nurbs_lane,
            )
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
        // **The transience fence** (U2's Q2 as corrected): the
        // scaffolding door is for edges whose surfaces do not exist
        // yet. This edge has two faces — the lookup above answered —
        // so it has a chart, and a scaffold here is a construction
        // that stopped half-way.
        if matches!(curve.description(), geom_brep::EdgeDescription::Scaffold(_)) {
            errors.push(ValidationError::ScaffoldAtRest { edge: edge_key });
        }
        let adjacent = match curve.description() {
            geom_brep::EdgeDescription::Intersection { s1, s2, .. }
            | geom_brep::EdgeDescription::TangentIntersection { s1, s2, .. } => {
                (*s1 == fs_plus && *s2 == fs_minus) || (*s1 == fs_minus && *s2 == fs_plus)
            }
            // Chart adjacency (M6-3, the M5-LOG item 6(iii) rule): the
            // described chart is ONE of the edge's two adjacent faces'
            // surfaces — a wall–wall seam is the u-boundary iso of
            // either wall, and the minted convention names one. An
            // image that claims to BE the chart's parameterization
            // seam owes more: both sides of a seam are one surface.
            geom_brep::EdgeDescription::Chart(c) if c.seam => {
                c.surface == fs_plus && c.surface == fs_minus
            }
            geom_brep::EdgeDescription::Chart(c) => c.surface == fs_plus || c.surface == fs_minus,
            // A scaffold names no surface; the fence above is the
            // complaint it earns, and stacking a second one on the
            // same edge would report one fault twice.
            geom_brep::EdgeDescription::Scaffold(_) => true,
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
    // material is. Folding the sense in here would flip
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
    // S10, one category per ARM — not one per check, because check 4
    // has two arms and they differ:
    //
    // - **Check 4, first-order pass: CATEGORY C.** It takes the two
    //   SURFACES (never the faces) and classifies the UNSIGNED
    //   tangent-plane wedge from implicit-form gradients: transverse
    //   or smooth, never which side the material is on. No
    //   sense, and its verdict is invariant under `revert`.
    // - **Check 4, MATERIAL arm: CATEGORY A.** It hands the door both
    //   faces' `Face::sense` bits, and must — the material side IS the face
    //   orientation, and the whole content of "unsigned" above is that
    //   the first-order pass cannot see it (D1's ratified wedge table,
    //   the #131 second-order ruling; the arm's own note sits at its
    //   code below). Its verdict is NOT revert-invariant: reverting a
    //   body maps `Cusp` ↔ `Slit`, which is the ruling's own symmetry.
    // - **Check 5: CATEGORY C.** It tests `(p − origin)·n` against
    //   `Zero`, sign-invariant for the same reason as check 3.
    //
    // (This header said CATEGORY C of both checks, on the strength of
    // a deferral the material arm retired. Two contradictory category
    // claims about one function is exactly what the D6 hand-multiply
    // discipline reads this header to prevent.)
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
        // An `Approx` face is exempt on the SAME terms and by the same
        // rule: its geometry is a spline fit, whose implicit-form
        // gradient is poison, so `classify_dihedral` cannot run there
        // either. O5 records the inheritance explicitly — narrowing it
        // (deriving a dihedral class from the DESCRIPTION rather than
        // the fit) is its own conversation, not smuggled in here.
        let nurbs_adjacent = s_plus.spline_chart().is_some() || s_minus.spline_chart().is_some();
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
            // The prefer-intrinsic rule reads the AUTHORITY record
            // (U2 Q3), not the description's shape: since the
            // conventional forms collapsed there is no shape left that
            // means "the modeler declared this locus", so the
            // declaration is the datum it always was.
            if !escalated && all_transverse && curve.authority().is_declared() {
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
        //
        // **Check 4's MATERIAL arm** (D1's ratified verdict table, the
        // #131 second-order ruling) rides the same schedule, because
        // it is the same classification with the faces' material sides
        // put back. The first-order pass compares tangent PLANES and
        // is therefore unsigned — wedge 0, π and 2π all read Smooth —
        // so a definitely-smooth edge asks two more questions:
        //
        // - **which arm**: the two faces' outward normals (each `∇F`
        //   selected by that face's `Face::sense` bit, minted inside
        //   the door) aligned ⇒ one material side ⇒ the
        //   legal π seam; opposed ⇒ the wedge is 0 or 2π
        //   (`material_wedge_side`);
        // - **which end**, on the opposed arm: the jet's κ_rel signed
        //   into the plus face's outward frame — positive is the cusp
        //   (material is the vanishing crescent), negative the knife
        //   slit (`material_cusp_side`). `revert` negates every
        //   outward normal at once, so it maps one end to the other
        //   and the pair is legal together or not at all.
        //
        // The ends are legal iff the tangency is DECLARED in the C7
        // vocabulary and jet-determinate. The second-order band has
        // three outcomes and they are three different answers:
        // definite κ_rel is the determinate contact the arm admits;
        // a definitely-collapsed κ_rel is conformal contact along the
        // locus — a lamina, refused as `LaminaWedge` and not curable
        // by any declaration; in-band is `SliverDihedral`, the honest
        // escalation. A whole-edge verdict needs every sample to
        // agree, and an edge whose samples DISAGREE — about the
        // pairing, or about which end — escalates
        // (`MaterialArmOutcome::Split`) rather than falling silent:
        // silence there would validate an undeclared cusp clean on the
        // strength of its own samples contradicting one another. That
        // is the opposite of the first-order pass's mixed
        // transverse/smooth exemption, which exempts an edge from a
        // DEMAND rather than from a REFUSAL.
        //
        // S10 CATEGORY A: this arm reads `Face::sense`, and must —
        // the material side IS the face orientation, and the deferral
        // this closes was precisely that the first-order pass cannot
        // see it.
        let mut arm: Option<MaterialArmOutcome> = None;
        #[allow(clippy::if_same_then_else)]
        let mark = if nurbs_adjacent {
            ContactMark::Unmarked
        } else if escalated {
            ContactMark::Unmarked
        } else if matches!(
            curve.description(),
            geom_brep::EdgeDescription::Chart(c) if c.seam
        ) {
            ContactMark::Seam
        } else if all_transverse {
            arm = Some(MaterialArmOutcome::Wedge(MaterialWedge::Transverse));
            ContactMark::Transverse
        } else if all_smooth {
            let sense_plus = match body.get_face(f_plus) {
                Some(face) => face.sense,
                None => continue, // unreachable on tier-1 input
            };
            let sense_minus = match body.get_face(f_minus) {
                Some(face) => face.sense,
                None => continue, // unreachable on tier-1 input
            };
            let mut jet_determinate = true;
            let mut jet_escalated = false;
            let mut opposed = true;
            let mut aligned = true;
            let mut side: Option<MaterialWedge> = None;
            let mut side_mixed = false;
            for i in 1..(geom_brep::CERT_SAMPLES - 1) {
                let t = curve.sample_param(i);
                let (p, tau) = curve.carrier().ders1(t);
                let jet = geom_brep::tangent_jet(s_plus, s_minus, p, tau);
                let arm = geom_brep::folded_lever_arm(s_plus, s_minus, p, extent);
                match classify_material_pairing(
                    s_plus,
                    sense_plus,
                    s_minus,
                    sense_minus,
                    p,
                    arm,
                    band,
                ) {
                    Ok(MaterialPairing::Aligned) => opposed = false,
                    Ok(MaterialPairing::Opposed) => aligned = false,
                    Err(cause) => {
                        errors.push(ValidationError::SliverDihedral {
                            edge: edge_key,
                            cause,
                        });
                        jet_escalated = true;
                        break;
                    }
                }
                let margin = Margin::sagitta(jet.kappa_rel.abs(), arm);
                match decide("tangent_second_order", margin, band) {
                    Ok(Sign::Positive) => {}
                    // ONE zero-side sample ends the edge's determinacy
                    // — and on the opposed arm that is the lamina
                    // refusal for the WHOLE edge, on the strength of a
                    // single sample. Deliberate, and conservative in
                    // the direction the ε rule cares about: the
                    // declared arm's condition is that the surfaces
                    // determine the locus ALONG the edge, so one place
                    // they do not is enough to deny it (the same rule
                    // the mark already follows — any zero-side sample
                    // marks `SmoothUnderdetermined`). ε-tightening
                    // makes zero-side verdicts RARER, so it can only
                    // remove this refusal, never introduce one.
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
                // Which end, decided only where it is asked: the
                // magnitude just classified definitely positive, so
                // this reads the SIGN of the same quantity in the
                // material frame and cannot honestly land on Zero.
                let signed = geom_brep::material_kappa_rel(jet.kappa_rel, sense_plus);
                let this = match decide("material_cusp_side", Margin::sagitta(signed, arm), band) {
                    Ok(Sign::Positive) => MaterialWedge::Cusp,
                    Ok(Sign::Negative) => MaterialWedge::Slit,
                    // Neither outcome is reachable through a margin
                    // the run can read: this is the SAME quantity
                    // whose magnitude classified definitely positive
                    // one decision above. Announced anyway — a state
                    // that cannot occur is reported, never swallowed —
                    // and as an escalation rather than a panic,
                    // because a validator's answer to "I cannot say"
                    // is an error in its vector.
                    Ok(Sign::Zero) => {
                        errors.push(ValidationError::SliverDihedral {
                            edge: edge_key,
                            cause: Indeterminate {
                                margin: geom_core::MarginDiag::Invalid,
                                band,
                                predicate: Some("material_cusp_side"),
                            },
                        });
                        jet_escalated = true;
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
                };
                match side {
                    Some(seen) if seen != this => side_mixed = true,
                    _ => side = Some(this),
                }
            }
            if !jet_escalated {
                arm = Some(material_arm_outcome(
                    aligned,
                    opposed,
                    jet_determinate,
                    side,
                    side_mixed,
                ));
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
        // The verdict table's refusing rows, read off the ONE outcome
        // the arm produced (`MaterialArmOutcome` — the states are
        // exclusive by construction, so nothing here ranks anything).
        // `None` is an edge the arm never judged: exempt by kind, or
        // already escalated with its own error.
        if let Some(error) = material_arm_error(
            arm,
            edge_key,
            declares_tangent_contact(declarations, f_plus, f_minus),
            band,
        ) {
            errors.push(error);
        }
        if mark == ContactMark::Tangent
            && curve.authority().is_declared()
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
    // this closes that class structurally.
    //
    // **Scope: loops of `Line` and `Circle` carriers.** A bare
    // vertex-chord Newell sum is the enclosed area only for straight
    // boundaries — a 270° sector's chord quad self-crosses — so an arc
    // enters exactly: `2A` is the chord Newell plus, per arc, its
    // circular segment `axis · R² · (Δ − sin Δ)`, odd in the signed span
    // `Δ` and therefore carrying the traversal sign, and the perimeter
    // lever is re-metered to the arcs' own lengths. That arithmetic has
    // one home, `crate::loop_winding`, which the merge's role assigner
    // (`merge_faces`) decides on as well, so the checker falsifies a
    // role by the very functional that assigned it — so an error IN the
    // functional moves both sides together and this arm cannot see it.
    // The independent guard is a row whose roles come from another
    // derivation: `m5_s10_face_sense`'s extruded washer (roles from
    // `profile`'s containment pass, chord terms zero) and its R ≠ 1,
    // Δ ≠ π crescent, which refuse the HONEST body under a mis-signed
    // or mis-scaled arc term. A LINE-only loop is decided by the chord
    // sum alone, the correction block structurally skipped.
    //
    // **The residue this arm does not examine**, by carrier: a loop
    // riding an `Ellipse` (the shared winding decides it, and the merge
    // assigns roles by it, but its lever is an arc-length upper bound;
    // what the widening would refuse over the corpora is measured at
    // `work/atrest/check-6-planar-arm-skips-ellipse-and-nurbs-loops.md`),
    // and a loop riding a NURBS or spiric carrier, whose region has no
    // closed-form area. A planar face bounded so carries a stored sense
    // no at-rest check falsifies.
    //
    // **The S10 sense gate.** A face's outward normal is the chart
    // normal with `sense` folded in, so the winding is compared against
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
        // The face's outward normal through the one door: the bit is
        // read here for what it CLAIMS (which side is material), and
        // the loop's stored winding is falsified against that claim.
        let outward = plane_outward_normal(face, normal).vec();
        for (l, is_outer) in
            core::iter::once((face.outer, true)).chain(face.rings.iter().map(|&r| (r, false)))
        {
            // Only a DEFINITE wrong sign refuses (doc on the variant:
            // the check-7 posture — Zero and escalated windings are
            // exempt, so degenerate pillows stay legal and
            // ε-tightening never flips valid → invalid).
            let wrong = if is_outer {
                Sign::Negative
            } else {
                Sign::Positive
            };
            // Line and Circle carriers (banner); an empty ring, a loop
            // riding an ellipse, spiric or NURBS edge, and a torn
            // lookup (unreachable on tier-1 input) are not asked.
            let Ok(Some(winding)) = body.planar_loop_winding(
                l,
                outward,
                band,
                crate::loop_winding::LoopCarriers::Circular,
            ) else {
                continue;
            };
            if winding == Ok(wrong) {
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
    // washer/cone/donut even keeps its POSITIVE volume (this arm
    // refuses their walls; the planar arm above, their arc-bounded
    // caps).
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
        if matches!(surface, Surface::Plane { .. }) || surface.spline_chart().is_some() {
            // Planar: the Newell arm above. Spline charts: the quadrature lane
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
            // The boundary encodes no side, so there is no second
            // encoding to cross-check the stored bit against: the
            // rimless sphere band, the documented residual above.
            // An ANSWER, not a refusal.
            Ok(geom_brep::props::MaterialSign::Unencoded) => {}
            // A REFUSED derivation is exempt here, never a
            // disagreement (the posture above, and the contract on
            // `boundary_material_sign` itself). It is not a discard:
            // every cause reachable on this arm is a premise the flux
            // lane runs before it integrates, so the same face refuses
            // there and check 7 reports it cause-carrying as
            // `VolumeUncomputable { source }`. Pushing anything here
            // would DESTROY that report rather than add to it, check 7
            // being gated on `errors.is_empty()` — the recorded
            // exception is the conic-trimmed wall named above, whose
            // quadrature flux is winding-derived and answers.
            Err(_) => {}
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
    // tier-2 gate above). `validate_geometric` states that gate as its
    // own composition instead — the `?` between its two halves — and
    // therefore gates on checks 8 and 9 too; here the gate is still the
    // `if`, because these callers run the whole battery in one pass.
    //
    // S10: the sense handling is INHERITED, not repeated here.
    // `crate::props` owns it and applies `Face::sense` at exactly
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
        // The DERIVATION and the DECISION, once each and per solid, and
        // the object they were made on kept for the caller that asked
        // for it — the same function `validate_geometric` makes check 7
        // through, so the two tiers cannot disagree on it at one lane.
        if let PlusVCheck::Through(quad) = plus_v {
            match plus_v_by_sign(body, band, tol, quad) {
                Ok(derived) => certificate = Some(derived),
                Err(verdicts) => errors.extend(verdicts),
            }
        }
    }

    // ------------------------------------------------------------------
    // Tier 3, check 8 (M5 PR 6, C4): the stored pcurve caches — the
    // certificate must be PRESENT on every half-edge of a minting
    // chart's face, the certification must REPLAY at rest (re-derived,
    // never trusted), and domain validity (trim containment + the
    // loop's one-branch continuity) must hold. Bodies carrying no
    // stored pcurves — every all-planar body — contribute nothing.
    // The gating runs ONE way, and both directions are worth saying.
    // This check is not gated on the volume one: a pcurve defect is
    // local evidence about a specific half-edge, not cascade noise, so
    // it is reported whatever check 7 concluded. The volume check IS
    // gated on this one at `validate_geometric`, whose composition
    // reaches check 7 only when the whole structural battery came back
    // clean — the reason being check 7's own: a volume read off a body
    // whose stored pcurves do not re-certify is a number derived from
    // geometry the pass has just refused to vouch for. In the two
    // passes that run the battery in one call the older, narrower gate
    // stands (checks 1-6 only), because they answer with one vector
    // rather than a composition.
    // ------------------------------------------------------------------
    for finding in crate::pcurves::validate_pcurves(body, band) {
        errors.push(ValidationError::Pcurve { finding });
    }

    // ------------------------------------------------------------------
    // Tier 3, check 9: ring-vs-outer disjointness, and then NESTING.
    // A ring is the statement "this face's region has a hole strictly
    // inside it", and that statement has two halves. A ring that
    // stands on the outer loop — sharing a vertex position with it,
    // running along one of its edges, or crossing or touching it at a
    // point — is not a trim of any region; that is the DISJOINTNESS
    // half, the arms of `ring_outer_contact`. A ring that is cleanly
    // disjoint from the outer loop but lies OUTSIDE it is not a hole
    // either; that is the NESTING half, `ring_nesting`, and it runs on
    // the pairs the contact arms cleared. Nothing else in this
    // battery sees either half; `check_9_refuses_a_ring_that_lies_outside_its_outer_loop`
    // asserts that of checks 6 and 7 rather than arguing it here. It
    // is the CDT downstream that discovers the body is not
    // triangulable, which is a consumer reporting a producer's bug.
    //
    // Compared by POSITION, not by key: the shape this exists to catch
    // is minted by a surgery that copies a boundary and re-labels the
    // copy as a ring, so the copy carries fresh vertex and edge keys
    // standing on the original's geometry. Not gated on the volume
    // check — a loop contact is local evidence about one face — while
    // the volume check IS gated on this one at `validate_geometric`,
    // for the reason check 8 above states in full.
    //
    // **What the five arms match, and WHAT THEY DO NOT** (D4 honesty —
    // an unstated blind spot is an unverified claim). Five arms,
    // reporting six `RingContact` shapes (arm 2 names which loop's
    // vertex it found). Matched: vertex-on-vertex (arm 1,
    // kind-agnostic, positions only); a vertex of EITHER loop on the
    // interior of an edge of the other (arm 2, both directions, the
    // locus and the trim decided together on a line and an arc alike);
    // edge-along-edge (arm 3), on `Line` and `Circle` carriers; and,
    // on a PLANAR face, the two loops crossing or touching at a point
    // that is a vertex of neither — a transversal crossing or a
    // one-point tangency (circle-circle internal or external,
    // line-circle). That last shape has two arms: two WHOLE circles
    // (arm 4, both loops in `loop_shape`'s `Disc` class) are decided
    // exactly by the centre distance against the radii's sum and
    // difference, with no trim to test; every other pair of `Line` and
    // `Circle` edges (arm 5) has its carriers' meeting points computed
    // in closed form and each tested against both edges' trims
    // (`window`: a line's span; an arc's distances to its ends and
    // apexes, never an angle, which compresses near an end). Every
    // margin escalates typed rather than reading as "disjoint", and
    // only where no gate has already cleared the pair.
    //
    // NOT matched, enumerated rather than gestured at:
    //
    // - **`Ellipse`, `Spiric` and NURBS carriers** in arms 2, 3 and
    //   5: `locus_gap` has no inversion for them and arm 5 no closed
    //   meeting point, so a contact carried by one is skipped. Arm 1
    //   still covers their endpoints.
    // - **A face on a non-planar surface** in arms 4 and 5: there is
    //   no plane to meet in. Arms 1-3 still run there. Both silences
    //   are one row
    //   (`work/atrest/check-9-meeting-arms-silent-off-a-plane-and-on-ellipse-spiric-nurbs-edges.md`).
    //
    // The residue is a floor, not a ceiling: what it costs is that a
    // body carrying one of those shapes validates. The shapes this
    // check exists for — a surgery re-labelling a copied boundary as a
    // ring — are all in the matched set, and the shell verb's own
    // door refuses ahead of them.
    //
    // **The nesting half**, its instruments and ITS residue, in the
    // same honesty. The question is, for each vertex of the ring: is
    // that point inside the region the outer loop bounds? A
    // definitely-outside vertex is the witness the report names. Two
    // instruments answer it, one per outer-loop class below, and
    // neither is minted here: the crate's one trilean containment
    // walk, [`crate::splitting::point_in_loop`], whose K rows are
    // `point_in_loop_*` and which this arm pools as a fourth consumer
    // the way `boolean::contfp` and the solid-containment sweep
    // already pool; and `boolean::contain`'s `disc_side`, one radial
    // margin through one decide on `bool_face_disc_radius`, the row
    // `contfp` decides the same class on.
    //
    // The queries are the ring's VERTICES, exact whatever curve joins
    // them, so an arc-bearing RING is decided as readily as a
    // polygonal one — on the PREMISE that the two loops do not cross,
    // which the contact arms CHECK wherever every edge of both loops
    // is a `Line` or a `Circle`, and which is assumed on the carriers
    // they are silent on; `ring_nesting`'s doc is the premise's one
    // home, and says why a circular ring gets no second instrument.
    // The queries lie in the face's plane because check 5 above
    // certifies that they do
    // (`planar_boundary_residual`), which is both instruments' stated
    // precondition.
    //
    // **What gates the arm is the OUTER loop's class**, and the
    // classifier is `boolean::contain`'s `loop_shape` — the same
    // question that module's walk dispatches on, asked here rather
    // than answered a second time in a narrower spelling. Four
    // classes, four postures:
    //
    // - **`Polygon`** — no arc anywhere, so the ray-parity polygon IS
    //   this loop's region. The arm runs the walk.
    // - **`Disc`** — every edge an arc of ONE circle, whose region is
    //   that circle's disc. The arm runs `disc_side`, exact on the
    //   class: the annular rim between two circles, which is every
    //   shelled vessel of revolution, is decided here.
    // - **`ArcParity`** — arcs over at least three vertices. The
    //   polygon is a proper region and the walk is measured correct on
    //   it, but it is not the LOOP's region: an arc bowing outward
    //   leaves region between the polygon and the boundary, and a
    //   point there reads `Out` when it is in. `contfp` walks it —
    //   one point's verdict — and this arm must not, because here an
    //   `Out` REFUSES a body. Measured, on a bored D-rod's transverse
    //   cap, whose major arc dips past the chord its vertices span
    //   and whose bore sits in the lune between the two. Silent.
    // - **`NoWalk`** — arc-bearing over fewer than three vertices,
    //   where the arcs are not one circle (a half-disc cap, a lens of
    //   two circles): the polygon has zero area and the walk answers
    //   `Out` for every interior point. Silent for the same reason.
    //
    // Two silences, one rule: this arm answers only where its
    // instrument's region IS the loop's region, because everywhere
    // else an `Out` it cannot trust would refuse a valid body, and
    // that is the one direction it must never fail in. Both wait on
    // the general arc-aware walk
    // (`work/atrest/check-9-nesting-arc-parity-and-no-walk-wait-on-the-arc-aware-walk.md`).
    //
    // A face on a non-planar surface is outside the gate for the
    // neighbouring reason — no plane for either instrument to run in
    // — and so is a face whose outer loop `loop_shape` cannot CLASSIFY
    // (a carrier-agreement escalation, or a loop it cannot read). That
    // last silence is the gate failing to open, not a margin rounded
    // toward blessing: which instrument expresses the region is what
    // went undecided, and answering anyway from one that may not be
    // the region is the false-refusal direction this arm must never
    // fail in.
    //
    // One shape inside the gate the arm still does not catch,
    // enumerated rather than gestured at: **a ring that CROSSES its
    // outer loop along an `Ellipse`, `Spiric` or NURBS edge**, part
    // inside and part out — the premise above, failing where the
    // contact arms cannot see it. A vertex definitely inside settles
    // the ring, so such a crossing whose first decided vertex is the
    // inside one passes — the trade that keeps one escalating vertex
    // from refusing a ring another vertex has already placed inside.
    // Every crossing between `Line` and `Circle` edges is the contact
    // half's `RingMeetsOuter` before this arm runs.
    //
    // Order, and why it is that order: the nesting arm runs only on a
    // pair the contact arms cleared. A ring that MEETS its outer loop
    // is already reported by name and by shape, and asking a
    // containment question about a point on the boundary would add a
    // second, vaguer report of the same defect.
    // ------------------------------------------------------------------
    for (face_key, face) in body.faces.iter() {
        // The gate is a property of the FACE — its surface and its
        // outer loop — so it is read once per face. Lazily, inside the
        // `Disjoint` arm: a face with no ring asks no nesting
        // question, and classifying its outer loop would be a carrier
        // walk over every boundary in the body for an answer nothing
        // reads.
        let mut nesting_gate: Option<Option<NestingRegion<T>>> = None;
        for &ring in &face.rings {
            match ring_outer_contact(body, face.outer, ring, band) {
                RingOuterVerdict::Disjoint => {
                    let gate = *nesting_gate.get_or_insert_with(|| {
                        nesting_region(body, face.surface, face.outer, band)
                    });
                    let Some(region) = gate else {
                        continue; // the nesting residue, enumerated above
                    };
                    match ring_nesting(body, face.outer, ring, region, band) {
                        RingNestingVerdict::Inside => {}
                        RingNestingVerdict::Outside { ring_vertex } => {
                            errors.push(ValidationError::RingOutsideOuter {
                                face: face_key,
                                ring,
                                ring_vertex,
                            });
                        }
                        RingNestingVerdict::Undecided(source) => {
                            errors.push(ValidationError::RingNestingUndecided {
                                face: face_key,
                                ring,
                                source,
                            });
                        }
                    }
                }
                RingOuterVerdict::Contact(contact) => {
                    errors.push(ValidationError::RingMeetsOuter {
                        face: face_key,
                        ring,
                        contact,
                    });
                }
                RingOuterVerdict::Escalated(source) => {
                    errors.push(ValidationError::RingContactEscalated {
                        face: face_key,
                        ring,
                        source,
                    });
                }
            }
        }
    }

    (errors, certificate)
}

/// What check 9 found between `ring` and the outer loop of its face.
pub(crate) enum RingOuterVerdict {
    /// The two loops are disjoint, as far as the arms below can see
    /// (check 9's residue list).
    Disjoint,
    /// They meet, in the named shape.
    Contact(RingContact),
    /// A contact margin could not be certified either way. **Never
    /// read as "disjoint"**: an escalation is the one answer this
    /// predicate is not allowed to round toward blessing a body, and
    /// rounding it was the defect the review of this check caught.
    Escalated(Indeterminate),
}

/// The first contact between `ring` and the outer loop `outer` of the
/// same face, in the six shapes the arms below can decide
/// ([`RingContact`]).
///
/// Shared with the shell verb, which runs it as a PRECONDITION of the
/// rim glue so the refusal names the shape rather than arriving as a
/// generic at-rest report.
///
/// **Escalate-never-guess (D4 ¶3)**: the first margin that lands in
/// the ambiguity band returns [`RingOuterVerdict::Escalated`] and stops
/// the walk. Both callers treat it as a refusal.
pub(crate) fn ring_outer_contact<T: Decide>(
    body: &Body<T>,
    outer: LoopKey,
    ring: LoopKey,
    band: Band,
) -> RingOuterVerdict {
    let (Some(ring_cycle), Some(outer_cycle)) =
        (loop_cycle_of(body, ring), loop_cycle_of(body, outer))
    else {
        // An empty loop bounds nothing and can meet nothing.
        return RingOuterVerdict::Disjoint;
    };

    // A separation, decided against ZERO. `Zero` is the contact:
    // metering `eps − gap` instead would put a coincident pair's margin
    // AT the band's own threshold, where it escalates rather than
    // decides.
    macro_rules! coincides {
        ($name:expr, $margin:expr) => {
            match decide($name, Margin::of($margin), band) {
                Ok(Sign::Zero) => true,
                Ok(_) => false,
                Err(source) => return RingOuterVerdict::Escalated(source),
            }
        };
    }
    // ---- Arm 1: a ring vertex standing on an outer VERTEX. ----
    //
    // Kind-agnostic: it reads points, so no carrier kind is exempt.
    // Key-shared pairs are NOT exempt either — tier 1 has no pass that
    // refuses one face's outer loop and its own ring sharing a vertex
    // key (an umbrella pinch walks a single orbit and passes every
    // pass 1-13), so exempting them here would leave exactly that
    // configuration unnetted.
    for &rhe in &ring_cycle {
        let Some(rv) = body.half_edges.get(rhe).map(|h| h.start) else {
            continue;
        };
        let Some(rp) = vertex_point(body, rv) else {
            continue;
        };
        for &ohe in &outer_cycle {
            let Some(ov) = body.half_edges.get(ohe).map(|h| h.start) else {
                continue;
            };
            let Some(op) = vertex_point(body, ov) else {
                continue;
            };
            if coincides!("ring_outer_vertex_gap", (rp - op).norm()) {
                return RingOuterVerdict::Contact(RingContact::Vertex {
                    ring_vertex: rv,
                    outer_vertex: ov,
                });
            }
        }
    }

    // ---- Arm 2: a vertex of either loop standing on an edge's
    // interior in the other. ----
    //
    // The shape arm 1 cannot see and arm 3 cannot either: the two loops
    // touching at a point that is a vertex of one and an interior
    // point of the other. Asked in BOTH directions — a ring vertex on
    // an outer edge, then an outer vertex on a ring edge — because
    // either loop's vertex can be the one that touches, and nothing
    // after this arm computes a meeting point at a vertex. Two margins
    // per pair ([`vertex_on_edge_interior`]): the point's gap to the
    // edge's LOCUS, and whether it lies inside the edge's TRIM — a
    // line's by lying strictly between its ends, an arc's by the
    // distance test [`window`] spells. The trim is asked on an arc as
    // on a line: a vertex on an arc's circle but past its ends is
    // nowhere on that edge.
    for (vertex_cycle, edge_cycle, ring_vertex) in [
        (&ring_cycle, &outer_cycle, true),
        (&outer_cycle, &ring_cycle, false),
    ] {
        for &vhe in vertex_cycle {
            let Some(v) = body.half_edges.get(vhe).map(|h| h.start) else {
                continue;
            };
            let Some(vp) = vertex_point(body, v) else {
                continue;
            };
            for &ehe in edge_cycle {
                match vertex_on_edge_interior(body, ehe, vp, band) {
                    Ok(None) => {}
                    Ok(Some(edge)) => {
                        return RingOuterVerdict::Contact(if ring_vertex {
                            RingContact::VertexOnEdge {
                                ring_vertex: v,
                                outer_edge: edge,
                            }
                        } else {
                            RingContact::OuterVertexOnEdge {
                                outer_vertex: v,
                                ring_edge: edge,
                            }
                        });
                    }
                    Err(source) => return RingOuterVerdict::Escalated(source),
                }
            }
        }
    }

    // ---- Arm 3: a ring edge running ALONG an outer edge. ----
    //
    // Three INTERIOR samples of the ring edge, at the certification
    // schedule's own quarter/middle/three-quarter parameters. Three
    // shared points is the discriminator: a line and a circle, or two
    // distinct circles, meet in at most two, so agreement at three
    // interior samples is a shared LOCUS rather than a crossing.
    //
    // Sharing the locus is not yet sharing an arc — two collinear
    // edges can be disjoint on a nonconvex face, and two arcs of one
    // circle can lie on opposite sides of it — so ANY of the three
    // samples lying inside the outer edge's trim settles it
    // ([`edge_trim`]: strictly between a line's ends, or inside an
    // arc's [`window`]). Any, not
    // the middle one: a partial overlap can put the middle sample past
    // the outer edge's trim while a quarter of it lies well inside.
    for &rhe in &ring_cycle {
        let Some(redge) = body.half_edges.get(rhe).map(|h| h.edge) else {
            continue;
        };
        let Some(rgeom) = certified_carrier(body, redge) else {
            continue;
        };
        let samples: [geom_core::Point3<T>; 3] = [
            rgeom.carrier().eval(rgeom.sample_param(2)),
            rgeom.carrier().eval(rgeom.sample_param(4)),
            rgeom.carrier().eval(rgeom.sample_param(6)),
        ];
        for &ohe in &outer_cycle {
            let Some(oedge) = body.half_edges.get(ohe).map(|h| h.edge) else {
                continue;
            };
            let Some(ogeom) = certified_carrier(body, oedge) else {
                continue;
            };
            let Some((_, oseg)) = meet_segment(body, ohe) else {
                continue; // the recorded residue: Ellipse, Spiric and Nurbs carriers
            };
            // The locus and the trim are two gates, and a sample
            // definitely failing EITHER clears it: one sample
            // definitely off the locus means no shared locus, and every
            // sample definitely past the trim means no shared arc,
            // whatever the other gate's band says. Only then does an
            // in-band margin on either escalate.
            let mut locus_unsure = None;
            let mut off_locus = false;
            for &p in &samples {
                let Some(gap) = locus_gap(ogeom.carrier(), p) else {
                    off_locus = true;
                    break;
                };
                match decide("ring_outer_locus_gap", Margin::of(gap), band) {
                    Ok(Sign::Zero) => {}
                    Ok(Sign::Positive | Sign::Negative) => {
                        off_locus = true;
                        break;
                    }
                    Err(source) => locus_unsure = locus_unsure.or(Some(source)),
                }
            }
            if off_locus {
                continue;
            }
            let mut inside = false;
            let mut trim_unsure = None;
            for &p in &samples {
                match edge_trim(oseg, p, band) {
                    Window::In => inside = true,
                    Window::Out => {}
                    Window::Unsure(source) => trim_unsure = trim_unsure.or(Some(source)),
                }
            }
            if !inside && trim_unsure.is_none() {
                continue;
            }
            if let Some(source) = locus_unsure.or(if inside { None } else { trim_unsure }) {
                return RingOuterVerdict::Escalated(source);
            }
            return RingOuterVerdict::Contact(RingContact::Edge {
                ring_edge: redge,
                outer_edge: oedge,
            });
        }
    }
    ring_outer_meeting(body, outer, ring, &ring_cycle, &outer_cycle, band)
}

/// Check 9's arm 2 for one pair: the edge under `he` if the vertex
/// point `p` of the OTHER loop stands on that edge's interior, `None`
/// if it does not — or if the edge is an `Ellipse`, `Spiric` or NURBS
/// carrier, the recorded residue. Two gates, decided together:
/// `p`'s gap to the edge's locus, and the trim ([`edge_trim`]:
/// strictly between a line's ends, or inside an arc's [`window`]). An
/// end of the edge counts as inside
/// an arc's trim because arm 1 has already reported a vertex standing
/// on a vertex.
fn vertex_on_edge_interior<T: Decide>(
    body: &Body<T>,
    he: HalfEdgeKey,
    p: geom_core::Point3<T>,
    band: Band,
) -> Result<Option<EdgeKey>, Indeterminate> {
    let Some((edge, segment)) = meet_segment(body, he) else {
        return Ok(None);
    };
    let Some(gap) = certified_carrier(body, edge).and_then(|g| locus_gap(g.carrier(), p)) else {
        return Ok(None);
    };
    // Two gates, and a definite miss on EITHER clears the pair: a
    // vertex definitely off the locus is nowhere on the edge, and one
    // definitely past the trim is nowhere on it either, however close
    // it stands to the circle or the line the edge is cut from. Only a
    // pair neither gate clears can escalate.
    match decide("ring_outer_locus_gap", Margin::of(gap), band) {
        Ok(Sign::Positive | Sign::Negative) => Ok(None),
        on_locus => match (on_locus, edge_trim(segment, p, band)) {
            (_, Window::Out) => Ok(None),
            (Err(source), _) | (_, Window::Unsure(source)) => Err(source),
            (Ok(_), Window::In) => Ok(Some(edge)),
        },
    }
}

/// Whether `p` lies inside `segment`'s trim as check 9's arms 2 and 3
/// ask it: strictly between a line's two ends, in projection — metered
/// as the LENGTH it is (the raw dot product is an area) — or inside an
/// arc's [`window`]. A line's END is outside, because a vertex there is
/// arm 1's; an arc's is inside, [`window`]'s own convention, which arm
/// 1 has also already answered.
fn edge_trim<T: Decide>(segment: MeetSegment<T>, p: geom_core::Point3<T>, band: Band) -> Window {
    match segment {
        MeetSegment::Line { a, b } => match decide(
            "ring_outer_segment_side",
            Margin::of((p - a).dot(p - b) / (b - a).norm()),
            band,
        ) {
            Ok(Sign::Negative) => Window::In,
            Ok(Sign::Zero | Sign::Positive) => Window::Out,
            Err(source) => Window::Unsure(source),
        },
        MeetSegment::Arc { .. } => window(segment, p, band),
    }
}

/// Check 9's arms 4 and 5: do `ring` and `outer` share a POINT that
/// arms 1–3 of [`ring_outer_contact`] do not name — a transversal
/// crossing, or a one-point tangency, at a point that is a vertex of
/// neither loop?
///
/// Run only after arms 1–3 have cleared the pair, and that order is
/// what lets both arms read a CLOSED edge: once no vertex of either
/// loop stands on the other loop (arms 1 and 2, both directions), a
/// point the two loops share is the loops meeting whichever edge's
/// end it sits at, so a candidate within the band of an edge's
/// ACTUAL end, measured as a distance, counts as inside that trim.
///
/// - **Arm 4, both loops whole circles** ([`crate::boolean::LoopShape::Disc`]
///   on each): [`circle_pair`], exact without any trim.
/// - **Arm 5, every other pair of `Line` and `Circle` edges**
///   ([`segments_meet`]): the carriers' meeting points, each tested
///   against both edges' trims ([`window`]) — a line's by its span, an
///   arc's by distances to its ends and to its two apexes.
///
/// Both run in the plane of `outer`'s face and are silent off one: a
/// non-planar face has no plane to intersect in, and an `Ellipse`,
/// `Spiric` or `Nurbs` edge has no closed-form meeting point here.
/// Those are check 9's recorded residue.
///
/// A definite meeting anywhere wins over an escalation elsewhere —
/// the report names the shape it can — and an escalation with no
/// meeting found is [`RingOuterVerdict::Escalated`], never
/// `Disjoint`.
fn ring_outer_meeting<T: Decide>(
    body: &Body<T>,
    outer: LoopKey,
    ring: LoopKey,
    ring_cycle: &[HalfEdgeKey],
    outer_cycle: &[HalfEdgeKey],
    band: Band,
) -> RingOuterVerdict {
    let normal = body
        .get_loop(outer)
        .and_then(|l| body.get_face(l.face))
        .and_then(|f| body.surfaces.get(f.surface));
    let Some(&Surface::Plane { normal, .. }) = normal else {
        return RingOuterVerdict::Disjoint; // the recorded residue: no plane
    };

    // ---- Arm 4: two whole circles. ----
    if let (Ok(crate::boolean::LoopShape::Disc(o)), Ok(crate::boolean::LoopShape::Disc(r))) = (
        crate::boolean::loop_shape(body, outer, band),
        crate::boolean::loop_shape(body, ring, band),
    ) {
        return match circle_pair(o.center, o.radius, r.center, r.radius, band) {
            Ok(CirclePair::Apart | CirclePair::Nested) => RingOuterVerdict::Disjoint,
            Ok(
                CirclePair::ExternallyTangent
                | CirclePair::InternallyTangent
                | CirclePair::Crossing,
            ) => RingOuterVerdict::Contact(RingContact::Circles {
                ring_loop: ring,
                outer_loop: outer,
            }),
            Err(source) => RingOuterVerdict::Escalated(source),
        };
    }

    // ---- Arm 5: an edge pair meeting at a point. ----
    // Resolved once: the pair loop below is O(ring × outer).
    let outer_segments: Vec<(EdgeKey, MeetSegment<T>)> = outer_cycle
        .iter()
        .filter_map(|&ohe| meet_segment(body, ohe))
        .collect();
    let mut escalated: Option<Indeterminate> = None;
    for &rhe in ring_cycle {
        let Some((ring_edge, rseg)) = meet_segment(body, rhe) else {
            continue; // the recorded residue: Ellipse, Spiric and Nurbs carriers
        };
        for &(outer_edge, oseg) in &outer_segments {
            match segments_meet(rseg, oseg, normal, band) {
                EdgePair::Apart => {}
                EdgePair::Meet => {
                    return RingOuterVerdict::Contact(RingContact::EdgesMeet {
                        ring_edge,
                        outer_edge,
                    });
                }
                EdgePair::Unsure(source) => escalated = escalated.or(Some(source)),
            }
        }
    }
    escalated.map_or(RingOuterVerdict::Disjoint, RingOuterVerdict::Escalated)
}

/// How two coplanar circles sit, from their centre distance `d`
/// against the sum and the difference of their radii — two decides,
/// and nothing about any trim.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CirclePair {
    /// `d > R + r`: each lies outside the other.
    Apart,
    /// `d = R + r`: they touch at one point, each outside the other.
    ExternallyTangent,
    /// `|R − r| < d < R + r`: they cross at two points.
    Crossing,
    /// `d = |R − r|`: they touch at one point, one inside the other.
    InternallyTangent,
    /// `d < |R − r|`: one lies strictly inside the other. Which one
    /// is the nesting arm's question, not this one's.
    Nested,
}

/// [`CirclePair`] for the circles `(c1, r1)` and `(c2, r2)`, which lie
/// in one plane. The outer decide runs first and settles two circles
/// far apart on its own; an escalation on either is returned rather
/// than read as either side of it.
fn circle_pair<T: Decide>(
    c1: geom_core::Point3<T>,
    r1: T,
    c2: geom_core::Point3<T>,
    r2: T,
    band: Band,
) -> Result<CirclePair, Indeterminate> {
    let d = (c2 - c1).norm();
    match decide("ring_outer_circles_apart", Margin::of(d - (r1 + r2)), band)? {
        Sign::Positive => return Ok(CirclePair::Apart),
        Sign::Zero => return Ok(CirclePair::ExternallyTangent),
        Sign::Negative => {}
    }
    Ok(
        match decide(
            "ring_outer_circles_nested",
            Margin::of((r1 - r2).abs() - d),
            band,
        )? {
            Sign::Positive => CirclePair::Nested,
            Sign::Zero => CirclePair::InternallyTangent,
            Sign::Negative => CirclePair::Crossing,
        },
    )
}

/// One edge as check 9's arm 5 reads it: a `Line` by its two end
/// points, a `Circle` by its carrier and parameter window.
#[derive(Clone, Copy)]
enum MeetSegment<T: Real> {
    /// A straight edge from `a` to `b`.
    Line {
        /// One end.
        a: geom_core::Point3<T>,
        /// The other end.
        b: geom_core::Point3<T>,
    },
    /// An arc of a circle, over `[t0, t1]` of its carrier.
    Arc {
        /// The circle's centre.
        center: geom_core::Point3<T>,
        /// Its unit axis.
        axis: geom_core::Vec3<T>,
        /// Its radius.
        radius: T,
        /// Its seam direction.
        u_ref: geom_core::Vec3<T>,
        /// The window's start.
        t0: T,
        /// The window's end.
        t1: T,
    },
}

/// The edge under `he` as a [`MeetSegment`], or `None` for a carrier
/// arm 5 has no meeting point for (`Ellipse`, `Spiric`, `Nurbs`) or an
/// edge it cannot resolve.
fn meet_segment<T: Real>(body: &Body<T>, he: HalfEdgeKey) -> Option<(EdgeKey, MeetSegment<T>)> {
    let edge = body.half_edges.get(he)?.edge;
    let geom = certified_carrier(body, edge)?;
    let segment = match *geom.carrier() {
        geom::Curve3::Line { .. } => {
            let (a, b) = edge_endpoints(body, he)?;
            MeetSegment::Line { a, b }
        }
        geom::Curve3::Circle {
            center,
            axis,
            radius,
            u_ref,
        } => {
            let (t0, t1) = geom.params();
            MeetSegment::Arc {
                center,
                axis,
                radius,
                u_ref,
                t0,
                t1,
            }
        }
        _ => return None,
    };
    Some((edge, segment))
}

/// Whether two edges share a point, as arm 5 decides it.
enum EdgePair {
    /// Definitely not.
    Apart,
    /// Definitely: a point lies on both carriers and inside both trims.
    Meet,
    /// A margin that could turn a candidate meeting point into a
    /// meeting landed in the band — kept, never read as `Apart`.
    Unsure(Indeterminate),
}

/// Whether a point on an edge's carrier lies inside its trim.
enum Window {
    /// Inside, an end included.
    In,
    /// Definitely past an end.
    Out,
    /// A trim margin landed in the band.
    Unsure(Indeterminate),
}

/// Arm 5 for one pair of edges lying in the plane whose unit normal is
/// `normal`. Two lines are decided by orientation, with no meeting
/// point computed; a pair with an arc computes its candidate meeting
/// points in closed form and hands them to [`meet_at`].
fn segments_meet<T: Decide>(
    first: MeetSegment<T>,
    second: MeetSegment<T>,
    normal: geom_core::Vec3<T>,
    band: Band,
) -> EdgePair {
    match (first, second) {
        (MeetSegment::Line { a: a0, b: a1 }, MeetSegment::Line { a: b0, b: b1 }) => {
            lines_meet(a0, a1, b0, b1, normal, band)
        }
        (MeetSegment::Line { a, b }, arc @ MeetSegment::Arc { center, radius, .. })
        | (arc @ MeetSegment::Arc { center, radius, .. }, MeetSegment::Line { a, b }) => {
            let e = (b - a).normalize();
            // The foot of the centre on the line, and the half-chord
            // either side of it.
            let foot = a + e * (center - a).dot(e);
            let h = (center - foot).norm();
            let half_chord = ((radius - h) * (radius + h)).max(T::zero()).sqrt();
            let (existence, candidates) =
                match decide("ring_outer_meet_reach", Margin::of(radius - h), band) {
                    Ok(Sign::Negative) => return EdgePair::Apart,
                    Ok(Sign::Positive) => {
                        (None, vec![foot + e * half_chord, foot - e * half_chord])
                    }
                    // Tangent within the band: the foot is the touching
                    // point, and the two chord ends — as far apart as
                    // `√(2·r·band)` — are asked as well, so a crossing
                    // this shallow cannot slip past a trim the foot
                    // itself misses.
                    Ok(Sign::Zero) => (
                        None,
                        vec![foot, foot + e * half_chord, foot - e * half_chord],
                    ),
                    Err(source) => (
                        Some(source),
                        vec![foot, foot + e * half_chord, foot - e * half_chord],
                    ),
                };
            meet_at(
                existence,
                &candidates,
                MeetSegment::Line { a, b },
                arc,
                band,
            )
        }
        (
            first @ MeetSegment::Arc {
                center: c1,
                radius: r1,
                ..
            },
            second @ MeetSegment::Arc {
                center: c2,
                radius: r2,
                ..
            },
        ) => {
            let (existence, tangent) = match circle_pair(c1, r1, c2, r2, band) {
                Ok(CirclePair::Apart | CirclePair::Nested) => return EdgePair::Apart,
                Ok(CirclePair::Crossing) => (None, false),
                Ok(CirclePair::ExternallyTangent | CirclePair::InternallyTangent) => (None, true),
                Err(source) => (Some(source), true),
            };
            let w = c2 - c1;
            let d = w.norm();
            if tangent {
                // Concentric within the band with radii equal within
                // it is ONE circle, and two arcs of one circle share a
                // point only if an end of one lies in the other's
                // trim — a vertex on a vertex (arm 1) or on an edge's
                // interior (arm 2, both directions), all reported
                // before this arm runs. So the pair is apart; the
                // direction to a touching point is undefined there
                // anyway, and this is settled before dividing by it.
                match decide("ring_outer_meet_centres", Margin::of(d), band) {
                    Ok(Sign::Positive) => {}
                    Ok(Sign::Zero | Sign::Negative) => return EdgePair::Apart,
                    Err(source) => return EdgePair::Unsure(existence.unwrap_or(source)),
                }
            }
            // The radical line's foot on the centre line, and the
            // half-chord either side of it; at a tangency the foot is
            // the touching point (`a = ±r1`) and the chord ends are
            // asked too, as on a line.
            let u = w.normalize();
            let v = normal.cross(u);
            let a = (d.powi(2) + r1.powi(2) - r2.powi(2)) / (d * T::from_f64(2.0));
            let half_chord = ((r1 - a) * (r1 + a)).max(T::zero()).sqrt();
            let foot = c1 + u * a;
            let candidates = if tangent {
                vec![foot, foot + v * half_chord, foot - v * half_chord]
            } else {
                vec![foot + v * half_chord, foot - v * half_chord]
            };
            meet_at(existence, &candidates, first, second, band)
        }
    }
}

/// Two straight edges `[a0, a1]` and `[b0, b1]` in one plane, by the
/// four orientation margins of each one's ends against the other's
/// line — signed lengths, `(p − o) × ê · n̂`. Both ends strictly on one
/// side of the other's line separates them; otherwise they meet,
/// unless all of one's ends lie ON the other's line, where the pair is
/// collinear and the question is whether the two spans overlap.
fn lines_meet<T: Decide>(
    a0: geom_core::Point3<T>,
    a1: geom_core::Point3<T>,
    b0: geom_core::Point3<T>,
    b1: geom_core::Point3<T>,
    normal: geom_core::Vec3<T>,
    band: Band,
) -> EdgePair {
    let ea = (a1 - a0).normalize();
    let eb = (b1 - b0).normalize();
    let side = |p: geom_core::Point3<T>, o: geom_core::Point3<T>, e: geom_core::Vec3<T>| {
        decide(
            "ring_outer_meet_side",
            Margin::of((p - o).cross(e).dot(normal)),
            band,
        )
    };
    let on_a = [side(b0, a0, ea), side(b1, a0, ea)];
    let on_b = [side(a0, b0, eb), side(a1, b0, eb)];
    let separated = |pair: &[Result<Sign, Indeterminate>; 2]| {
        matches!(
            pair,
            [Ok(Sign::Positive), Ok(Sign::Positive)] | [Ok(Sign::Negative), Ok(Sign::Negative)]
        )
    };
    if separated(&on_a) || separated(&on_b) {
        return EdgePair::Apart;
    }
    if let Some(&Err(source)) = on_a.iter().chain(&on_b).find(|r| r.is_err()) {
        return EdgePair::Unsure(source);
    }
    let collinear =
        |pair: &[Result<Sign, Indeterminate>; 2]| matches!(pair, [Ok(Sign::Zero), Ok(Sign::Zero)]);
    // Collinear: do the spans overlap, measured along one edge's own
    // axis? A point of overlap is a meeting too.
    let overlap = |o0: geom_core::Point3<T>,
                   o1: geom_core::Point3<T>,
                   e: geom_core::Vec3<T>,
                   p0: geom_core::Point3<T>,
                   p1: geom_core::Point3<T>| {
        let len = (o1 - o0).norm();
        let (t0, t1) = ((p0 - o0).dot(e), (p1 - o0).dot(e));
        let margin = t0.max(t1).min(len) - t0.min(t1).max(T::zero());
        match decide("ring_outer_meet_overlap", Margin::of(margin), band) {
            Ok(Sign::Positive | Sign::Zero) => EdgePair::Meet,
            Ok(Sign::Negative) => EdgePair::Apart,
            Err(source) => EdgePair::Unsure(source),
        }
    };
    if collinear(&on_a) {
        overlap(a0, a1, ea, b0, b1)
    } else if collinear(&on_b) {
        overlap(b0, b1, eb, a0, a1)
    } else {
        EdgePair::Meet
    }
}

/// Arm 5's verdict on candidate meeting points of two edges' carriers:
/// a candidate inside both trims is a meeting. `existence` is the
/// escalation, if any, of the margin that said the carriers meet at
/// all — with one, a candidate inside both trims is `Unsure` rather
/// than a meeting, and a candidate definitely past either trim is
/// still ruled out, which is what keeps a near-tangency far from both
/// edges from escalating.
fn meet_at<T: Decide>(
    existence: Option<Indeterminate>,
    candidates: &[geom_core::Point3<T>],
    first: MeetSegment<T>,
    second: MeetSegment<T>,
    band: Band,
) -> EdgePair {
    let mut open: Option<Indeterminate> = None;
    for &p in candidates {
        let wa = window(first, p, band);
        if matches!(wa, Window::Out) {
            continue;
        }
        let diag = match (wa, window(second, p, band)) {
            (_, Window::Out) => continue,
            (Window::In, Window::In) => match existence {
                None => return EdgePair::Meet,
                Some(source) => source,
            },
            (Window::Unsure(source), _) | (_, Window::Unsure(source)) => {
                existence.unwrap_or(source)
            }
            (Window::Out, _) => continue,
        };
        open = open.or(Some(diag));
    }
    open.map_or(EdgePair::Apart, EdgePair::Unsure)
}

/// Whether `p`, a point on `segment`'s carrier, lies inside its trim —
/// an end included, since arm 5 runs only once no vertex arm has
/// spoken ([`ring_outer_meeting`]).
///
/// A line's trim is its two signed spans from its ends, each a length
/// at unit speed. An arc's is decided as DISTANCES, in two steps, and
/// never through the radial band: `p` is a point on the carrier (or a
/// candidate whose distance from it the caller has already decided),
/// so its distance from the circle says nothing about the trim, and
/// deciding it first let an in-band radius escalate a point half a
/// turn away from the arc.
///
/// 1. **At an end**: `p` within the band of either end point,
///    measured as `|p − end|`, is inside. That is the ONLY way an
///    endpoint neighbourhood counts: an angular window compresses
///    arc length near an end by `sin(w/2)`, so on a short arc (and by
///    `sin` of the complement on a near-full one) its `Zero` reaches
///    `ε / sin(w/2)` along the carrier, a hundred times `ε` at
///    `w = 0.02`.
/// 2. **Otherwise, which side of the ends**: the sum of two chordal
///    defects, `(|a − m| − |p − m|) + (|p − m′| − |a − m′|)`, where `a`
///    is an end, `m` the arc's apex and `m′` its complement's. Chord
///    length is monotone in angular distance up to a half turn, so
///    each term is positive exactly on the arc; near an end they
///    move as `cos(w/4)` and `sin(w/4)` times the arc length, whose
///    sum is at least 1, so the margin is never compressed below the
///    distance it measures — on a short arc, a near-full one, or a
///    whole circle (where `m′` is the end and every point is inside).
///    Its `Zero` is therefore within the band of an end, which step 1
///    has already answered, and reads inside.
fn window<T: Decide>(segment: MeetSegment<T>, p: geom_core::Point3<T>, band: Band) -> Window {
    match segment {
        MeetSegment::Line { a, b } => {
            let len = (b - a).norm();
            let s0 = (p - a).dot((b - a).normalize());
            let spans = [
                decide("ring_outer_meet_span", Margin::of(s0), band),
                decide("ring_outer_meet_span", Margin::of(len - s0), band),
            ];
            if spans.iter().any(|s| matches!(s, Ok(Sign::Negative))) {
                return Window::Out;
            }
            match spans.iter().find_map(|s| s.err()) {
                Some(source) => Window::Unsure(source),
                None => Window::In,
            }
        }
        MeetSegment::Arc {
            center,
            axis,
            radius,
            u_ref,
            t0,
            t1,
        } => {
            let carrier = geom::Curve3::Circle {
                center,
                axis,
                radius,
                u_ref,
            };
            let (a, b) = (carrier.eval(t0), carrier.eval(t1));
            let ends = [
                decide("ring_outer_arc_end", Margin::of((p - a).norm()), band),
                decide("ring_outer_arc_end", Margin::of((p - b).norm()), band),
            ];
            if ends.iter().any(|e| matches!(e, Ok(Sign::Zero))) {
                return Window::In;
            }
            if let Some(source) = ends.iter().find_map(|e| e.err()) {
                return Window::Unsure(source);
            }
            let mid = (t0 + t1) * T::from_f64(0.5);
            let apex = carrier.eval(mid);
            let anti = carrier.eval(mid + T::pi());
            let margin =
                ((a - apex).norm() - (p - apex).norm()) + ((p - anti).norm() - (a - anti).norm());
            match decide("ring_outer_arc_trim", Margin::of(margin), band) {
                Ok(Sign::Positive | Sign::Zero) => Window::In,
                Ok(Sign::Negative) => Window::Out,
                Err(source) => Window::Unsure(source),
            }
        }
    }
}

/// The region check 9's nesting arm reads a face's outer loop as, or
/// `None` where the arm is silent — the whole of its gate, read off
/// one face.
///
/// Two conditions, and the second is not this function's to decide:
/// the surface is a `Plane` (there is otherwise no plane for either
/// instrument to run in), and the outer loop's region is one an exact
/// instrument expresses. That second question is
/// [`crate::boolean::loop_shape`]'s — the classifier
/// `boolean::contfp` dispatches its own walks on — so it is asked
/// there and its answer is only PROJECTED here, never re-derived: two
/// of its classes are answered, `Polygon`, where the ray-parity
/// polygon IS the region, and `Disc`, where the region is one
/// circle's disc and [`crate::boolean::disc_side`] decides it exactly.
/// `ArcParity`, `NoWalk` and a loop the classifier could not read are
/// all silent, and check 9's banner states what each silence costs.
fn nesting_region<T: Decide>(
    body: &Body<T>,
    surface: crate::geometry::SurfaceKey,
    outer: LoopKey,
    band: Band,
) -> Option<NestingRegion<T>> {
    let Some(&Surface::Plane { normal, .. }) = body.surfaces.get(surface) else {
        return None;
    };
    match crate::boolean::loop_shape(body, outer, band) {
        Ok(crate::boolean::LoopShape::Polygon) => Some(NestingRegion::Polygon { normal }),
        Ok(crate::boolean::LoopShape::Disc(disc)) => Some(NestingRegion::Disc(disc)),
        Ok(crate::boolean::LoopShape::ArcParity | crate::boolean::LoopShape::NoWalk) | Err(_) => {
            None
        }
    }
}

/// The instrument check 9's nesting arm places one ring vertex with —
/// one per outer-loop class [`nesting_region`] answers, each exact on
/// its class.
///
/// A projection of [`crate::boolean::LoopShape`] onto the two classes
/// the arm answers, not a second classification: `LoopShape` has no
/// place for the chart normal the walk needs, and matching on it in
/// [`ring_nesting`] would carry two arms (`ArcParity`, `NoWalk`) the
/// gate has already shut, with nothing true to say in them. `contfp`
/// dispatches on `LoopShape` itself because it answers every class.
#[derive(Clone, Copy)]
enum NestingRegion<T: Real> {
    /// The ray-parity walk over the outer loop, in the plane whose
    /// chart normal this is (handed over without `Face::sense`
    /// folded in: the walk's verdict is invariant under its sign).
    Polygon {
        /// The face's chart normal.
        normal: geom_core::Vec3<T>,
    },
    /// The disc of the one circle every outer edge is an arc of.
    Disc(crate::boolean::LoopCircle<T>),
}

/// Where check 9's nesting half placed a ring relative to the outer
/// loop of its own face. Private: the arm is its only caller, and one
/// question already carries two trileans down this call chain (this
/// one and [`crate::splitting::LoopContainment`]) without a third
/// leaving the module.
///
/// The three variants partition on ONE fact the walk records —
/// whether any query ESCALATED — and the docs say so rather than
/// leaving a reader to find it in the walk.
enum RingNestingVerdict {
    /// Nothing to report. Either a vertex of the ring was placed
    /// definitely inside the outer loop's region — it is a hole of
    /// that region, as a ring claims to be — or no vertex was placed
    /// either way AND nothing escalated, which is the ring whose
    /// every readable vertex came back `OnBoundary`.
    Inside,
    /// A named ring vertex is definitely OUTSIDE that region, so the
    /// ring is not a hole in it.
    Outside {
        /// The witness.
        ring_vertex: VertexKey,
    },
    /// No vertex was placed either way and at least one query
    /// ESCALATED — that escalation, kept. **Never read as
    /// "inside"**: the same escalate-never-guess posture the contact
    /// half takes, one question over.
    Undecided(ContainError),
}

/// Does `ring` lie inside the region `outer` bounds, both loops of one
/// planar face whose outer loop [`nesting_region`] read as `region`?
///
/// The ring's VERTICES are the queries, in cycle order, and the walk
/// takes the first definite verdict it reaches — `Out` reports, `In`
/// accepts. That is what keeps an escalation at one vertex from
/// refusing a ring another vertex has already placed inside, and it
/// is why the witness reported is the FIRST vertex in cycle order
/// that read outside rather than a distinguished one. An `OnBoundary`
/// is the disjointness half's question, not this one, so it settles
/// nothing here and the walk moves to the next vertex.
///
/// **One vertex speaks for the whole ring, and the premise that makes
/// it so is that the two loops do not CROSS.** A ring disjoint from
/// the outer boundary is a connected curve in one component of the
/// plane minus that boundary, so any one of its points places all of
/// it — whatever curve joins its vertices, arcs and whole circles
/// included. The premise is not checked BY this arm; it is checked
/// FOR it, by the contact arms that run first: on a planar face whose
/// two loops carry only `Line` and `Circle` edges, any point the two
/// loops share is a `RingMeetsOuter` — at a vertex of either loop
/// (arms 1 and 2, arm 2 in both directions), along a shared arc
/// (arm 3), or at a point that is a vertex of neither (arm 4 for two
/// whole circles, arm 5 for every other edge pair) — so this function
/// never runs on a pair that crosses. The premise is ASSUMED only where a loop
/// carries an `Ellipse`, `Spiric` or NURBS edge, which arm 5 has no
/// meeting point for (check 9's banner lists it in the residue).
///
/// **Why a `Disc`-class ring gets no second instrument** — the one
/// home of this argument; check 9's banner and `docs/KERNEL-VERBS.md`
/// point here. The shape the premise excludes — a ring arc bowing past
/// the outer circle while every ring vertex sits inside — is a
/// CROSSING, and the only case the two-circle closed form (centre
/// distance plus ring radius against the outer radius) adds over the
/// vertices is exactly that one. Reported here it would be a
/// `RingOutsideOuter` with no vertex outside to name; its home is the
/// contact half, as the loops MEETING — [`RingContact::Circles`] from
/// arm 4 when both loops are whole circles, [`RingContact::EdgesMeet`]
/// from arm 5 otherwise.
///
/// **The off-boundary precondition** both instruments give a definite
/// `In`/`Out` under — the query is not within the band of the outer
/// boundary — is supplied by check 9's contact arms, not by check 5:
/// on a cycle ring, arm 2 decides every ring vertex's gap to every
/// outer carrier's LOCUS (on a `Disc` outer, the whole circle, which
/// is the whole boundary), so a vertex in band of it is already a
/// `RingMeetsOuter` or `RingContactEscalated` and this function never
/// runs on that pair. A lone-vertex ring has no cycle for the contact
/// arms to walk, so its one query can reach the instrument in band;
/// there the `OnBoundary` or the escalation is the instrument's own
/// answer, read below like any other.
///
/// An EMPTY ring is a lone vertex — `kemr`'s mint — and that vertex is
/// the one query. Such a ring bounds no region, but it stands
/// somewhere, and a lone-vertex ring planted outside its face's outer
/// loop is the same defect as any other ring outside it.
///
/// Run only on a `(outer, ring)` pair [`ring_outer_contact`] has
/// cleared, and only behind [`nesting_region`]; the banner at check 9
/// states both and enumerates what they leave out.
fn ring_nesting<T: Decide>(
    body: &Body<T>,
    outer: LoopKey,
    ring: LoopKey,
    region: NestingRegion<T>,
    band: Band,
) -> RingNestingVerdict {
    let queries: Vec<VertexKey> = match body.get_loop(ring).map(|l| l.boundary) {
        Some(LoopBoundary::Empty { vertex }) => vec![vertex],
        Some(LoopBoundary::Cycle { .. }) | None => loop_cycle_of(body, ring)
            .unwrap_or_default()
            .iter()
            .filter_map(|&rhe| body.half_edges.get(rhe).map(|h| h.start))
            .collect(),
    };
    let mut undecided: Option<ContainError> = None;
    for rv in queries {
        let Some(rp) = vertex_point(body, rv) else {
            continue;
        };
        // A ring vertex lies in the face's plane (check 5), the
        // in-plane precondition of both instruments; the off-boundary
        // one is the contact arms' (the doc above).
        let placed = match region {
            NestingRegion::Polygon { normal } => {
                crate::splitting::point_in_loop(body, outer, normal, rp, band).map_err(Into::into)
            }
            NestingRegion::Disc(disc) => crate::boolean::disc_side(disc, rp, band),
        };
        match placed {
            Ok(crate::splitting::LoopContainment::In) => return RingNestingVerdict::Inside,
            Ok(crate::splitting::LoopContainment::Out) => {
                return RingNestingVerdict::Outside { ring_vertex: rv };
            }
            // The contact half's question, not this one.
            Ok(crate::splitting::LoopContainment::OnBoundary) => {}
            Err(source) => undecided = undecided.or(Some(source)),
        }
    }
    undecided.map_or(RingNestingVerdict::Inside, RingNestingVerdict::Undecided)
}

/// An edge's certified carrier, or `None` on a null or unresolvable
/// one.
fn certified_carrier<T: Real>(body: &Body<T>, edge: EdgeKey) -> Option<&geom_brep::EdgeCurve<T>> {
    body.get_edge(edge)
        .and_then(|e| body.get_curve_geom(e.curve))
        .and_then(CurveGeom::certified)
}

/// The distance from `p` to a carrier's INFINITE locus, in closed
/// form and without a single comparison. `None` for a kind this
/// inversion does not implement — check 9's recorded residue,
/// `Ellipse` and `Nurbs`.
fn locus_gap<T: Real>(carrier: &geom::Curve3<T>, p: geom_core::Point3<T>) -> Option<T> {
    match carrier {
        // `dir` is unit by convention, so the rejection is the residual
        // of the projection.
        geom::Curve3::Line { origin, dir } => {
            let d = p - *origin;
            Some((d - *dir * d.dot(*dir)).norm())
        }
        // The distance from a point to a full circle: the in-plane
        // radial defect and the out-of-plane offset, in quadrature.
        geom::Curve3::Circle {
            center,
            axis,
            radius,
            ..
        } => {
            let d = p - *center;
            let along = d.dot(*axis);
            let radial = (d - *axis * along).norm() - *radius;
            Some((radial.powi(2) + along.powi(2)).sqrt())
        }
        _ => None,
    }
}

/// A loop's half-edge cycle, or `None` for an empty loop (which bounds
/// nothing and can meet nothing).
fn loop_cycle_of<T: Real>(body: &Body<T>, r#loop: LoopKey) -> Option<Vec<HalfEdgeKey>> {
    let LoopBoundary::Cycle { first } = body.get_loop(r#loop)?.boundary else {
        return None;
    };
    body.loop_cycle(first)
}

/// The point a vertex stands at.
fn vertex_point<T: Real>(body: &Body<T>, vertex: VertexKey) -> Option<geom_core::Point3<T>> {
    body.points.get(body.vertices.get(vertex)?.point).copied()
}

/// **Tier 3′** (M3 PR 6a, F1/F2): the pseudomanifold at-rest validator
/// for declared-contact bodies — tier 3's full local battery **plus**
/// the global coincidence census tier 3 defers, certified against the
/// body's declared-contact records.
///
/// Structure (D1):
/// 1. Coarse-gate on tiers 1–2 (as [`validate_geometric`]).
/// 2. All of tier 3's local checks, shared verbatim
///    ([`tier3_local_checks`]) — the whole nine-check battery in one
///    call, check 7 included through the certified quadrature, and its
///    check-7 gate is the battery-internal one (checks 1-6) rather than
///    [`validate_geometric`]'s composition.
/// 3. The **global coincidence census** (only when the local checks are
///    clean — census geometry on a locally corrupt body is cascade
///    noise, the check-7 discipline): every cross-entity position
///    coincidence among distinct entities — vertex-vertex,
///    vertex-on-face (interior), vertex-on-edge (interior), proper
///    edge-face / edge-edge crossings, and the segment-granularity
///    overlaps (edge-edge collinear, edge-on-face) reconstructed per
///    the D3 rule (see [`crate::census`] module docs). The sweeps
///    examine only the pairs whose padded boxes overlap — one C10
///    tree per entity class, the boolean edge×face sweep's own boxes
///    and pad (`escalate + 2·zero`) — and a pair the filter clears is
///    DECIDED by that definite box-disjointness answer, not skipped;
///    the exact sweep's escalations and refusals about the CARRIERS
///    of such a pair (a marginal angle between far edges, a ray walk
///    on a far coplanar vertex, a vertex in band of a line past the
///    edge's end) are therefore not raised. `census::Trees` carries
///    the derivation.
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
///
/// # What this door checks, and which door does less
///
/// **Check 2 re-derives an M7-8 edge here** — a plane × described-NURBS
/// `Intersection` — through the certified plane × NURBS lane, which is
/// why this door's bound names the certification right: an imported or
/// minted body carrying that class re-derives its certificate at rest
/// exactly as it did at attach time, the invariant this pass has always
/// been the home of. It is the door a certifying caller wants, and the
/// callers in the tree take it: [`crate::AtRestPolicy`]'s `f64`,
/// `Probe`, `Interval` and `Sym` arms, `step-import`'s aggregate gate,
/// the `pncad` prelude's re-export and through it `pncad-py`'s
/// `Body.validate_pseudomanifold` (monomorphic at `f64`) and the tour's
/// scenes. [`validate_pseudomanifold_structural`] is the same pass holding
/// none of the three lanes this door holds (the plane × NURBS lane, the
/// quadrature lane and the chart-region door), and is the door a
/// [`Dual`](geom_core::Dual) body goes through the tier-3′ pass by.
pub fn validate_pseudomanifold<
    T: geom_core::Decide + geom_core::CertifiedBounds + crate::props::AtRestPolicy,
>(
    body: &Body<T>,
    contacts: &crate::boolean::ContactRecords,
    tol: Tol,
) -> Result<(), Vec<ValidationError>> {
    validate_pseudomanifold_certificate(body, contacts, tol).map(|_| ())
}

/// **[`validate_pseudomanifold`] holding none of its three lanes** — no
/// plane × NURBS lane, no quadrature lane
/// ([`crate::QuadLane`]) and no chart-region door
/// ([`crate::RegionLane`]) — the tier-3′ pass at every
/// [`crate::AtRestPolicy`] scalar with a bracket, a
/// [`Dual`](geom_core::Dual) included. This door is what keeps the
/// tier-3′ pass callable at a scalar that may not certify, which is the
/// capability H-R3 protects.
///
/// Three things this door does not do, each a statement about the DOOR
/// and never about the scalar it is called at — the `f64` caller of
/// this door gets exactly what the dual caller gets. **Check 7 runs
/// through the closed form alone**: a face that needs the certified
/// quadrature refuses typed ([`ValidationError::VolumeUncomputable`])
/// rather than passing unbounded, and on a closed-form body the verdict
/// is the certified door's, with pads of `0`. **Check 2 makes no claim
/// about an M7-8 edge**, at any scalar, this one's `f64` included: that
/// class re-derives only through the certified plane × NURBS lane,
/// which this door does not hold, and check 2 is a whole-edge check, so
/// what goes unmade is every check-2 verdict on such an edge rather
/// than only its plane × NURBS limbs. **The census examines no
/// chart-region candidate and backs no crossing by a declared pair**:
/// the pass hands the census no region door, so its two chart-region
/// arms refuse rather than examine — the conformal face-pair arm on
/// every same-key opposed-sense curved pair, and the declared-record
/// confirm arm on every patch record whose pair passes Door 1 — each
/// as [`ValidationError::CensusLaneUnsupported`] naming the pair; and
/// the crossing rung's backing consult answers `false`, so an in-plane
/// `EdgeEdgeCross` at a declared seat that [`validate_pseudomanifold`]
/// reports backed is reported here as
/// [`ValidationError::UndeclaredContact`], with the declaration in
/// hand. The declaration is not found wrong; it is not read.
///
/// Note the suffix's other meaning one door over:
/// [`validate_geometric_structural`] does not make check 7 at all (`Ok`
/// on the body this door refuses with `VolumeUncomputable`); the two
/// shapes are stated side by side at that door.
///
/// # Errors
///
/// As [`validate_pseudomanifold`], less the check-2 verdicts above,
/// plus the closed form's typed refusal at check 7, plus the census's
/// `CensusLaneUnsupported` on every conformal candidate and every
/// Door-1-verified patch record, and an `UndeclaredContact` on every
/// crossing a declared pair would have backed.
pub fn validate_pseudomanifold_structural<
    T: geom_core::Decide + geom_core::Bounds + crate::props::AtRestPolicy,
>(
    body: &Body<T>,
    contacts: &crate::boolean::ContactRecords,
    tol: Tol,
) -> Result<(), Vec<ValidationError>> {
    validate_pseudomanifold_certificate_structural(body, contacts, tol).map(|_| ())
}

/// **[`validate_pseudomanifold`], handing back the certificate its
/// check 7 decided on** — the tier-3′ door's certificate form, and
/// [`validate_geometric_certificate`]'s claim verbatim one tier up: the
/// same pass, the same verdicts, and one certified walk per SOLID,
/// which is check 7's subject, each stopped at the round where that
/// solid's SIGN is certain.
///
/// **A SIGN, and the number on request.** What comes back is a
/// [`crate::SignCertificate`], the per-solid walks assembled into
/// face-arena order, whatever the body's solid count — so a body of
/// several solids pays one read of each face and no further
/// arena-wide one. A caller that wants the number calls
/// [`crate::SignCertificate::refine_to_target`], which pays only the
/// rounds the check did not, and whose `Ok` is bit-identical to
/// [`crate::mass_properties`] on the same body at the same `tol`.
///
/// **This door is the one the import path pays.** `step-import`'s
/// aggregate gate runs it over every assembled body and continues the
/// certificate for the enclosure it ships, so the cost of the number is
/// spelled at that call site rather than folded into the gate.
///
/// # Errors
///
/// As [`validate_pseudomanifold`].
pub fn validate_pseudomanifold_certificate<
    'b,
    T: geom_core::Decide + geom_core::CertifiedBounds + crate::props::AtRestPolicy,
>(
    body: &'b Body<T>,
    contacts: &crate::boolean::ContactRecords,
    tol: Tol,
) -> Result<crate::props::SignCertificate<'b, T>, Vec<ValidationError>> {
    pseudomanifold_certificate_via(
        body,
        contacts,
        tol,
        Some(&geom_brep::plane_nurbs_limbs::<T>),
        Some(crate::props::QuadLane::certified()),
        Some(crate::chart_region::RegionLane::certified()),
    )
}

/// [`validate_pseudomanifold_structural`]'s certificate form — the
/// [`crate::SignCertificate`] its closed-form check 7 decided on.
///
/// **At a [`Dual`](geom_core::Dual) this is where the difference from
/// [`validate_geometric_certificate`] shows.** The certified door is
/// bounded on `CertifiedBounds`, so a dual cannot form it at all (the
/// `compile_fail` guarantee); this one holds none of the three lanes
/// [`validate_pseudomanifold_structural`] names — the quadrature's
/// absence is the one the certificate shows: the closed form answers,
/// and a face that needs the quadrature refuses TYPED rather than
/// passing unbounded. So a certificate handed back at a dual is a
/// closed-form body's: every face finished at round 0, its pads `0`,
/// and its continuation the fold of what it already holds. The region door's absence
/// shows in the error vector, at every scalar, exactly as at the
/// `()`-returning twin.
///
/// # Errors
///
/// As [`validate_pseudomanifold_structural`].
pub fn validate_pseudomanifold_certificate_structural<
    'b,
    T: geom_core::Decide + geom_core::Bounds + crate::props::AtRestPolicy,
>(
    body: &'b Body<T>,
    contacts: &crate::boolean::ContactRecords,
    tol: Tol,
) -> Result<crate::props::SignCertificate<'b, T>, Vec<ValidationError>> {
    pseudomanifold_certificate_via(body, contacts, tol, None, None, None)
}

/// The tier-3′ pass with its three lanes as arguments — the shared
/// body of the certified door and its `_structural` twin. The region
/// lane is the census's: `None` is the census's own typed refusal at
/// its two chart-region arms
/// ([`ValidationError::CensusLaneUnsupported`]) and a `false` at the
/// crossing rung's backing consult — the `_structural` twin's path,
/// and a [`Dual`](geom_core::Dual)'s only one.
fn pseudomanifold_certificate_via<
    'b,
    T: geom_core::Decide + geom_core::Bounds + crate::props::AtRestPolicy,
>(
    body: &'b Body<T>,
    contacts: &crate::boolean::ContactRecords,
    tol: Tol,
    nurbs_lane: Option<geom_brep::NurbsLane<'_, T>>,
    quad_lane: Option<crate::props::QuadLane<T>>,
    region: Option<crate::chart_region::RegionLane<T>>,
) -> Result<crate::props::SignCertificate<'b, T>, Vec<ValidationError>> {
    validate_closed(body)?;
    let band = match Band::linear(tol) {
        Ok(band) => band,
        Err(error) => return Err(vec![ValidationError::Band { error }]),
    };
    // The tier-3 local battery, verbatim, with this body's C3
    // curve-granularity records read as what they are: `Tangent`
    // declarations on their face pairs (the class `CurveContact`
    // certifies through the jet schedule). Patch records are NOT
    // passed — a conformal declaration over a region asserts the
    // wrong class for a curve locus, which is D1's "the arm admits no
    // laminae" at the plumbing level. With empty records the slice is
    // empty and 3′ is tier 3 exactly.
    let declarations: Vec<DeclaredContact> = contacts
        .curves
        .iter()
        .map(|c| DeclaredContact {
            a: c.face_a,
            b: c.face_b,
            class: crate::contact::ContactClass::Tangent,
        })
        .collect();
    let (mut errors, certificate) =
        tier3_local_checks(body, &declarations, band, tol, nurbs_lane, quad_lane);
    if errors.is_empty() {
        errors.extend(crate::census::census_and_certify(
            body, contacts, band, tol, region,
        ));
    }
    if errors.is_empty() {
        Ok(certificate_of_a_clean_verdict(certificate))
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
        let face = body.face_of_half_edge(he)?;
        Some((face, body.get_face(face)?.surface))
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
                        let mate_face = body.face_of_half_edge(mate)?;
                        if body.get_face(mate_face)?.shell == shell
                            && !visited.contains_key(mate_face)
                        {
                            visited.insert(mate_face, ());
                            pending.push(mate_face);
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
    use geom_core::Tol;
    use proptest::prelude::*;

    use super::*;
    use crate::body::Body;
    use crate::entity::{Face, Loop, Shell, Solid, Vertex};
    use crate::euler::{MefSite, MevSite};
    use crate::fixtures::{
        mvfs_state, ngon_pillow, ops_genus2, ops_holed_box, pillow, prov, raw_prism,
    };
    use crate::seqgen;
    use crate::test_support_fixtures::{declined_cube, plane_every_face, plant_ring_face};

    /// **Check 1's analytic verdicts at BOTH scalars**, one surface
    /// lifted from `f64` to `Interval` through `Surface::map_scalar`
    /// and handed to the same door. The rows a scalar-shaped
    /// degradation would red: the zero normal (`[0,0]/[0,0]` is the
    /// empty interval, poison), a NaN datum (NaI at the interval
    /// scalar), the cone one ulp inside and exactly at `π/2` (where
    /// `Interval::pi()` is an enclosure, not a point), and a finite
    /// normal whose norm overflows at `f64` — a direction, refused at
    /// neither scalar.
    #[test]
    fn check_1_analytic_verdicts_agree_at_f64_and_interval() {
        use geom::{ConventionEnd, SurfaceDatum as D};
        use geom_brep::SurfaceKind as K;
        use geom_core::{Interval, Vec3};
        let face = FaceKey::default();
        let plane = |normal: Vec3<f64>| Surface::Plane {
            origin: Point3::new(0.0, 0.0, 0.0),
            normal,
            u_ref: Vec3::unit_x(),
        };
        let cone = |half_angle: f64| Surface::Cone {
            apex: Point3::new(0.0, 0.0, 0.0),
            axis: Vec3::unit_z(),
            half_angle,
            u_ref: Vec3::unit_x(),
        };
        let half_pi = core::f64::consts::FRAC_PI_2;
        let normal_verdict = vec![ValidationError::PoisonedSurfaceDatum {
            face,
            kind: K::Plane,
            datum: D::Normal,
        }];
        let rows: Vec<(&str, Surface<f64>, Vec<ValidationError>)> = vec![
            (
                "zero normal",
                plane(Vec3::new(0.0, 0.0, 0.0)),
                normal_verdict.clone(),
            ),
            (
                "NaN normal",
                plane(Vec3::new(f64::NAN, 0.0, 1.0)),
                normal_verdict,
            ),
            (
                "normal (1e200, 0, 0)",
                plane(Vec3::new(1e200, 0.0, 0.0)),
                vec![],
            ),
            (
                "normal (0, 0, 1e160)",
                plane(Vec3::new(0.0, 0.0, 1e160)),
                vec![],
            ),
            (
                "cone at pi/2",
                cone(half_pi),
                vec![ValidationError::UnrepresentableSurfaceDatum {
                    face,
                    kind: K::Cone,
                    datum: D::HalfAngle,
                    end: ConventionEnd::Upper,
                }],
            ),
            (
                "cone one ulp below pi/2",
                cone(f64::from_bits(half_pi.to_bits() - 1)),
                vec![],
            ),
            (
                "cone at 0",
                cone(0.0),
                vec![ValidationError::UnrepresentableSurfaceDatum {
                    face,
                    kind: K::Cone,
                    datum: D::HalfAngle,
                    end: ConventionEnd::Lower,
                }],
            ),
        ];
        for (name, surface, expected) in rows {
            let at_f64 = analytic_datum_verdicts(face, &surface);
            let lifted: Surface<Interval> = surface.map_scalar(Interval::from_f64);
            let at_interval = analytic_datum_verdicts(face, &lifted);
            assert_eq!(at_f64, expected, "{name}, at f64");
            assert_eq!(at_interval, expected, "{name}, at Interval");
        }
    }

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
        let t = pillow(Tol::witness());
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
        let t = ngon_pillow(1, Tol::witness());
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
        let t = raw_prism(4, Tol::witness());
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
        let t = raw_prism(4, Tol::witness());
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
        let t = raw_prism(3, Tol::witness());
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
        let mut t = pillow(Tol::witness());
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
        let mut t = pillow(Tol::witness());
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
        let mut t = pillow(Tol::witness());
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
        let mut t = pillow(Tol::witness());
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
        let mut t = pillow(Tol::witness());
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
        let mut t = pillow(Tol::witness());
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
        let mut t = pillow(Tol::witness());
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
        let mut t = pillow(Tol::witness());
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
        let mut t = pillow(Tol::witness());
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
        let mut t = pillow(Tol::witness());
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
        let mut t = pillow(Tol::witness());
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
        let mut t = pillow(Tol::witness());
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
        let mut t = pillow(Tol::witness());
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
        let mut t = pillow(Tol::witness());
        let v0 = t.vertices[0];
        let curve = t
            .body
            .add_curve(crate::fixtures::test_curve(anchor(), Tol::witness()));
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
        let mut t = pillow(Tol::witness());
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
        let mut t = pillow(Tol::witness());
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
        let mut t = pillow(Tol::witness());
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
        let mut t = pillow(Tol::witness());
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
        let mut t = pillow(Tol::witness());
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
        let mut t = pillow(Tol::witness());
        let p = t.body.add_point(anchor());
        let c = t
            .body
            .add_curve(crate::fixtures::test_curve(anchor(), Tol::witness()));
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

    /// `too_close`'s two sentences are the shared coincidence menu,
    /// spelled once in `geom_core` — the plain one, and the poisoned
    /// margin's with its input check first.
    #[test]
    fn too_close_spells_the_shared_menu() {
        let menu = geom_core::COINCIDENCE_RECOURSE;
        assert_eq!(super::too_close(None), format!("Recourse: {menu}"));
        assert_eq!(
            super::too_close(Some(&geom_core::MarginDiag::Invalid)),
            format!("Recourse: check the inputs that built this body, then {menu}")
        );
    }

    #[test]
    fn errors_display_without_panicking() {
        // At least one sample per `ValidationError` variant, from the
        // shared list a downstream refusal-budget row renders too
        // (`crate::test_support_samples`), indexed by the enum's own
        // compiler-derived companion: `ValidationErrorKind` supplies
        // both the index and the count, so a variant added without a
        // sample fails this row by name — and, once sampled, is held
        // to the budget there — and nothing here restates the enum.
        use strum::{EnumCount as _, IntoEnumIterator as _};
        let all = crate::test_support_samples::validation_error_samples();
        // The two derives agree on order: `from(err) as usize` is
        // the declaration index and `iter()` walks the same
        // sequence, so zipping them below pairs each flag with the
        // kind it stands for. Asserted rather than assumed.
        for (i, kind) in ValidationErrorKind::iter().enumerate() {
            assert_eq!(kind as usize, i, "EnumIter order is the discriminant order");
        }
        let mut covered = [false; ValidationErrorKind::COUNT];
        for (_, err) in &all {
            // Display and Error are wired up; content is human-oriented.
            assert!(!err.to_string().is_empty());
            let _: &dyn std::error::Error = err;
            covered[ValidationErrorKind::from(err) as usize] = true;
        }
        let missing: Vec<ValidationErrorKind> = ValidationErrorKind::iter()
            .zip(covered)
            .filter_map(|(kind, seen)| (!seen).then_some(kind))
            .collect();
        assert!(
            missing.is_empty(),
            "every ValidationError variant needs a Display sample; missing {missing:?}",
        );
        let nested = crate::test_support_samples::nested_coverage_gaps();
        assert!(
            nested.is_empty(),
            "every variant of every enum an arm renders whole needs a sample; missing {nested:?}",
        );
    }

    // ------------------------------------------------------------------
    // Pass 9: arity floors.
    // ------------------------------------------------------------------

    #[test]
    fn solid_without_shells_is_reported() {
        let mut t = pillow(Tol::witness());
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
        let mut t = pillow(Tol::witness());
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
        let mut t = pillow(Tol::witness());
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
        let t = declined_cube::<f64>(Tol::witness());
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
        let mut t = pillow(Tol::witness());
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
        let mut t = pillow(Tol::witness());
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
        let mut t = pillow(Tol::witness());
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
        assert_eq!(validate_closed(&pillow(Tol::witness()).body), Ok(()));
        assert_eq!(
            validate_closed(&ngon_pillow(1, Tol::witness()).body),
            Ok(())
        );
        assert_eq!(validate_closed(&raw_prism(4, Tol::witness()).body), Ok(()));
        // …and the operator-built acceptance bodies, genus 0 through 2.
        assert_eq!(
            validate_closed(&declined_cube::<f64>(Tol::witness()).body),
            Ok(())
        );
        assert_eq!(validate_closed(&ops_holed_box(Tol::witness()).body), Ok(()));
        assert_eq!(validate_closed(&ops_genus2(Tol::witness())), Ok(()));
    }

    /// **Check 9 decides KEY-SHARED loop pairs, and the guards that
    /// exempted them are gone.**
    ///
    /// Those two guards (`ov == rv`, `oedge == redge`) cited tier 1 as
    /// the net for a key-shared vertex or edge between a face's outer
    /// loop and its own ring. The citation was FALSE — passes 1 to 13
    /// were read end to end and none of them refuses that
    /// configuration; an umbrella pinch, whose two loops meet at a
    /// vertex they share by key rather than by position, walks a
    /// single orbit and validates. An exemption resting on a net that
    /// does not exist leaves exactly the shape this check is for
    /// unguarded, so it was removed.
    ///
    /// What is pinned here is the removal, in the maximal form of the
    /// case: a loop compared against ITSELF shares every vertex key
    /// and every edge key with itself, so both guards would have fired
    /// and both arms must now report a contact instead. It is a
    /// property of the predicate, asserted as one; the shapes it lets
    /// through when the keys are DISTINCT are pinned on real bodies by
    /// `verbs_shell::a_ring_standing_on_its_outer_loop_refuses_at_tier_3`.
    #[test]
    fn check_9_decides_a_key_shared_loop_pair() {
        let tol = Tol::witness();
        let body = ops_holed_box(tol).body;
        let band = Band::linear(tol).expect("the run's band");
        let mut seen = 0;
        for (_, face) in body.faces() {
            for lk in core::iter::once(face.outer).chain(face.rings.iter().copied()) {
                let Some(cycle) = loop_cycle_of(&body, lk) else {
                    continue;
                };
                if cycle.is_empty() {
                    continue;
                }
                seen += 1;
                match ring_outer_contact(&body, lk, lk, band) {
                    RingOuterVerdict::Contact(_) => {}
                    RingOuterVerdict::Disjoint => panic!(
                        "loop {lk:?} does not meet ITSELF — the key-shared exemption is \
                         back, and with it the umbrella pinch tier 1 does not refuse"
                    ),
                    RingOuterVerdict::Escalated(source) => {
                        panic!("loop {lk:?} escalated against itself: {source}")
                    }
                }
            }
        }
        assert!(seen > 0, "the fixture must carry loops to compare");
        // And check 9 says nothing about the fixture as it stands:
        // DISTINCT loops of one face do not meet, so the arms are not
        // simply reporting everything. Scoped to this check rather
        // than asserting full tier-3 cleanliness — the holed box has
        // its own epsilon-sensitive rows at 1e-12, which are not this
        // test's subject.
        let report = validate_geometric(&body, tol);
        let ours: Vec<&ValidationError> = match &report {
            Ok(()) => Vec::new(),
            Err(errors) => errors
                .iter()
                .filter(|e| {
                    matches!(
                        e,
                        ValidationError::RingMeetsOuter { .. }
                            | ValidationError::RingContactEscalated { .. }
                    )
                })
                .collect(),
        };
        assert!(
            ours.is_empty(),
            "check 9 must be silent on the fixture as it stands; got {ours:?}"
        );
    }

    /// **Check 9 states ring-INSIDE-outer, not merely
    /// ring-disjoint-from-outer.**
    ///
    /// The mutant is the shape a `kfmrh` glue mints when its two roles
    /// are inverted: one face keeps a ring that ENCLOSES its own outer
    /// loop. It is built here by re-labelling one face's two loops and
    /// flipping that face's `sense`, because that pair of edits is
    /// exactly what the inverted glue produces and nothing else —
    /// every point, every edge, every cycle and every winding is the
    /// honest body's.
    ///
    /// The flip is not decoration: the two coplanar faces an inverted
    /// glue chooses between face OPPOSITE ways, so the loop that
    /// becomes a ring is wound correctly for its new role either way.
    /// The row asserts that directly — check 6 stays silent on the
    /// mutant — because that silence is the reason this check has to
    /// exist, and this is the one site that states it: the variant's
    /// doc and check 9's banner cite the row rather than re-argue it.
    /// Check 7 is silent for its own reason, which check 6's own
    /// banner states: the volume is integrated from the same windings,
    /// and re-labelling which loop is the ring moves none of them.
    #[test]
    fn check_9_refuses_a_ring_that_lies_outside_its_outer_loop() {
        let tol = Tol::witness();
        let band = Band::linear(tol).expect("the run's band");
        let mut honest = ops_holed_box(tol).body;
        plane_every_face(&mut honest, tol);

        // The fixture's through-hole leaves TWO ring-bearing faces.
        // The mutant is built on `holed[0]` — whichever the arena
        // hands over first, the defect being a property of the role
        // inversion and not of which of the two carries it.
        let holed: Vec<FaceKey> = honest
            .faces
            .iter()
            .filter(|(_, f)| !f.rings.is_empty())
            .map(|(k, _)| k)
            .collect();
        assert_eq!(
            holed.len(),
            2,
            "the fixture's through-hole leaves a ring on the top face \
             (the rim) and one on the bottom (the plug)"
        );
        let face = holed[0];
        let (outer, ring) = {
            let f = honest.get_face(face).unwrap();
            (f.outer, f.rings[0])
        };

        let check_9 = |body: &Body<f64>| -> Vec<ValidationError> {
            let (errors, _) = tier3_local_checks(
                body,
                &[],
                band,
                tol,
                None,
                Some(crate::props::QuadLane::certified()),
            );
            errors
                .into_iter()
                .filter(|e| {
                    matches!(
                        e,
                        ValidationError::RingMeetsOuter { .. }
                            | ValidationError::RingContactEscalated { .. }
                            | ValidationError::RingOutsideOuter { .. }
                            | ValidationError::RingNestingUndecided { .. }
                    )
                })
                .collect()
        };
        let inverted_roles = |body: &Body<f64>| -> Vec<LoopKey> {
            let (errors, _) = tier3_local_checks(
                body,
                &[],
                band,
                tol,
                None,
                Some(crate::props::QuadLane::certified()),
            );
            errors
                .into_iter()
                .filter_map(|e| match e {
                    ValidationError::LoopRoleInverted { face: f, r#loop } if f == face => {
                        Some(r#loop)
                    }
                    _ => None,
                })
                .collect()
        };

        assert!(
            check_9(&honest).is_empty(),
            "the honest fixture's ring IS inside its outer loop; got {:?}",
            check_9(&honest)
        );
        assert!(
            inverted_roles(&honest).is_empty(),
            "check 6 is silent on the honest fixture"
        );

        let mut mutant = honest.clone();
        {
            let f = mutant.faces.get_mut(face).unwrap();
            f.outer = ring;
            f.rings[0] = outer;
            f.sense = !f.sense;
        }
        assert!(
            inverted_roles(&mutant).is_empty(),
            "check 6 must be BLIND to the inversion — both loops keep a \
             role-correct winding about the flipped outward normal; got {:?}",
            inverted_roles(&mutant)
        );
        assert_eq!(
            check_9(&mutant),
            vec![ValidationError::RingOutsideOuter {
                face,
                ring: outer,
                ring_vertex: mutant
                    .get_half_edge(match mutant.get_loop(outer).unwrap().boundary {
                        LoopBoundary::Cycle { first } => first,
                        LoopBoundary::Empty { .. } => panic!("the outer loop is a cycle"),
                    })
                    .unwrap()
                    .start,
            }],
            "the ring now ENCLOSES the face's outer loop and nothing else \
             in the battery says so"
        );
    }

    /// **The nesting arm refuses no honest body — and this row can see
    /// the arm switched off.** The direction that costs a user a valid
    /// model is a false refusal, so every closed fixture the battery
    /// already blesses is walked for one, planes grafted on so the
    /// arm's gate is open rather than vacuous.
    ///
    /// The honest half alone is green whether the arm runs or is dead,
    /// which is no evidence about the arm at all. So every ringed face
    /// the gate admits is ALSO inverted in place — the raw
    /// re-labelling
    /// `check_9_refuses_a_ring_that_lies_outside_its_outer_loop`
    /// justifies — and the refusal is asserted by name, on that face
    /// and that loop. A gate forced shut reds this row there.
    #[test]
    fn the_nesting_arm_is_silent_on_every_closed_fixture() {
        let tol = Tol::witness();
        let band = Band::linear(tol).expect("the run's band");
        let mut ringed = 0;
        // The operator-built family: `plane_every_face` fits a Newell
        // plane per face, which needs three vertices on a loop, and
        // the raw pillow/prism fixtures carry two-vertex loops. They
        // carry no rings either, so the arm would be vacuous on them.
        for mut body in [
            declined_cube::<f64>(tol).body,
            ops_holed_box(tol).body,
            ops_genus2(tol),
        ] {
            plane_every_face(&mut body, tol);
            let gated: Vec<(FaceKey, LoopKey, LoopKey)> = body
                .faces
                .iter()
                .filter(|(_, f)| {
                    !f.rings.is_empty() && nesting_region(&body, f.surface, f.outer, band).is_some()
                })
                .map(|(k, f)| (k, f.outer, f.rings[0]))
                .collect();
            ringed += gated.len();
            let ours = nesting_words(&body, band, tol);
            assert!(
                ours.is_empty(),
                "the nesting arm refused a fixture: {ours:?}"
            );
            for (face, outer, ring) in gated {
                let mut mutant = body.clone();
                {
                    let f = mutant.faces.get_mut(face).expect("the face");
                    f.outer = ring;
                    f.rings[0] = outer;
                    f.sense = !f.sense;
                }
                let got = nesting_words(&mutant, band, tol);
                assert!(
                    got.iter().any(|e| matches!(
                        e,
                        ValidationError::RingOutsideOuter {
                            face: f,
                            ring: r,
                            ..
                        } if *f == face && *r == outer
                    )),
                    "the inverted pick on {face:?} must be refused by name; got {got:?}"
                );
            }
        }
        assert!(
            ringed > 0,
            "at least one fixture must reach the arm's gate, or the row \
             asserts nothing"
        );
    }

    /// Check 9's NESTING words, on `body`.
    fn nesting_words(body: &Body<f64>, band: Band, tol: Tol) -> Vec<ValidationError> {
        let (errors, _) = tier3_local_checks(
            body,
            &[],
            band,
            tol,
            None,
            Some(crate::props::QuadLane::certified()),
        );
        errors
            .into_iter()
            .filter(|e| {
                matches!(
                    e,
                    ValidationError::RingOutsideOuter { .. }
                        | ValidationError::RingNestingUndecided { .. }
                )
            })
            .collect()
    }

    /// All four of check 9's words, on `body`.
    fn check_9_words(body: &Body<f64>, band: Band, tol: Tol) -> Vec<ValidationError> {
        let (errors, _) = tier3_local_checks(
            body,
            &[],
            band,
            tol,
            None,
            Some(crate::props::QuadLane::certified()),
        );
        errors
            .into_iter()
            .filter(|e| {
                matches!(
                    e,
                    ValidationError::RingMeetsOuter { .. }
                        | ValidationError::RingContactEscalated { .. }
                        | ValidationError::RingOutsideOuter { .. }
                        | ValidationError::RingNestingUndecided { .. }
                )
            })
            .collect()
    }

    /// A planar lamina whose one face carries one ring, both polygons
    /// chosen by the caller — the operator-built shape
    /// [`ops_holed_box`] has, at points a row can choose. Every step
    /// is a public Euler door; `plane_every_face` then fits the chart
    /// the nesting arm's gate reads.
    fn lamina_with_ring(
        outer: &[Point3<f64>],
        ring: &[Point3<f64>],
        tol: Tol,
    ) -> (Body<f64>, FaceKey) {
        assert!(outer.len() >= 3 && ring.len() >= 3);
        let mut body = Body::<f64>::new();
        let seed = body.mvfs(outer[0]).unwrap();
        let e0 = body
            .mev_line(
                MevSite::Lone {
                    r#loop: seed.r#loop,
                },
                outer[1],
                tol,
            )
            .unwrap();
        let mut prev = e0;
        let mut vs = vec![e0.vertex];
        for p in &outer[2..] {
            prev = body
                .mev_line(
                    MevSite::Fan {
                        he1: prev.he_minus,
                        he2: prev.he_minus,
                    },
                    *p,
                    tol,
                )
                .unwrap();
            vs.push(prev.vertex);
        }
        let vlast = vs[vs.len() - 1];
        let vprev = if vs.len() >= 2 {
            vs[vs.len() - 2]
        } else {
            seed.vertex
        };
        let he_close = body.find_half_edge(seed.face, vlast, vprev).unwrap();
        let f = body
            .mef_chord(
                MefSite::Chords {
                    he1: he_close,
                    he2: e0.he_plus,
                },
                tol,
            )
            .unwrap();
        // Plant the ring and cover it with its membrane.
        plant_ring_face(&mut body, f.he_plus, ring, tol);
        plane_every_face(&mut body, tol);
        let ringed: Vec<FaceKey> = body
            .faces
            .iter()
            .filter(|(_, f)| !f.rings.is_empty())
            .map(|(k, _)| k)
            .collect();
        assert_eq!(ringed.len(), 1, "one ring-bearing face");
        (body, ringed[0])
    }

    /// The inverted `kfmrh` pick, as a raw re-labelling: the two loops
    /// of `face` swap roles and its `sense` flips, which is what the
    /// glue produces and nothing else.
    fn invert_roles(body: &Body<f64>, face: FaceKey) -> Body<f64> {
        let mut m = body.clone();
        let f = m.faces.get_mut(face).unwrap();
        let outer = f.outer;
        let ring = f.rings[0];
        f.outer = ring;
        f.rings[0] = outer;
        f.sense = !f.sense;
        m
    }

    /// The thin cross's outer loop — a shape whose centroid lies in a
    /// hole of its own region.
    fn cross_outer() -> Vec<Point3<f64>> {
        let p = Point3::new;
        vec![
            p(1.0, 1.0, 0.0),
            p(1.0, 10.0, 0.0),
            p(-1.0, 10.0, 0.0),
            p(-1.0, 1.0, 0.0),
            p(-10.0, 1.0, 0.0),
            p(-10.0, -1.0, 0.0),
            p(-1.0, -1.0, 0.0),
            p(-1.0, -10.0, 0.0),
            p(1.0, -10.0, 0.0),
            p(1.0, -1.0, 0.0),
            p(10.0, -1.0, 0.0),
            p(10.0, 1.0, 0.0),
        ]
    }

    /// The ring threading one of that cross's arms.
    fn cross_ring() -> Vec<Point3<f64>> {
        let p = Point3::new;
        vec![
            p(-9.0, -0.4, 0.0),
            p(9.0, -0.4, 0.0),
            p(9.0, 0.4, 0.0),
            p(-9.0, 0.4, 0.0),
        ]
    }

    /// **The arm refuses none of three hand-built nested rings, and
    /// refuses all three inversions.** The false refusal is the
    /// failure to fear, so the fixtures are chosen to be awkward for a
    /// mean-radius or bounding-shape test rather than for the walk: a
    /// hole at the far end of a 50:1 plate, a ring hugging an L's
    /// concave corner, and a ring threading one arm of a thin cross
    /// (whose mean radius about the joint centroid is LARGER than the
    /// outer loop's — see
    /// [`mean_radius_nesting_is_not_a_containment_statement`]).
    #[test]
    fn the_nesting_arm_refuses_no_hand_built_nested_ring() {
        let tol = Tol::witness();
        let band = Band::linear(tol).expect("the run's band");
        let p = Point3::new;
        for (name, outer, ring) in [
            (
                "10 x 0.2 plate, hole at one end",
                vec![
                    p(0.0, 0.0, 0.0),
                    p(10.0, 0.0, 0.0),
                    p(10.0, 0.2, 0.0),
                    p(0.0, 0.2, 0.0),
                ],
                vec![
                    p(8.5, 0.05, 0.0),
                    p(9.5, 0.05, 0.0),
                    p(9.5, 0.15, 0.0),
                    p(8.5, 0.15, 0.0),
                ],
            ),
            (
                "L-plate, ring hugging the concave corner",
                vec![
                    p(0.0, 0.0, 0.0),
                    p(10.0, 0.0, 0.0),
                    p(10.0, 2.0, 0.0),
                    p(2.0, 2.0, 0.0),
                    p(2.0, 10.0, 0.0),
                    p(0.0, 10.0, 0.0),
                ],
                vec![
                    p(1.2, 1.2, 0.0),
                    p(1.8, 1.2, 0.0),
                    p(1.8, 1.8, 0.0),
                    p(1.2, 1.8, 0.0),
                ],
            ),
            (
                "thin cross, ring threading one arm",
                cross_outer(),
                cross_ring(),
            ),
        ] {
            let (body, face) = lamina_with_ring(&outer, &ring, tol);
            let f = body.get_face(face).unwrap();
            assert!(
                nesting_region(&body, f.surface, f.outer, band).is_some(),
                "{name}: the gate must be OPEN or the row asserts nothing"
            );
            assert!(
                check_9_words(&body, band, tol).is_empty(),
                "{name}: the arm refused a valid body: {:?}",
                check_9_words(&body, band, tol)
            );
            let got = check_9_words(&invert_roles(&body, face), band, tol);
            assert!(
                got.iter()
                    .any(|e| matches!(e, ValidationError::RingOutsideOuter { .. })),
                "{name}: the inversion must be refused; got {got:?}"
            );
        }
    }

    /// **A mean radius is not a containment statement**, which is why
    /// check 9's nesting arm does not use the shape `shell::encloses`
    /// uses — a TOPO row about a SHELL helper's SHAPE, kept beside the
    /// nesting rows because it is the argument for the instrument they
    /// pin.
    ///
    /// `encloses` compares mean radii about the pair's common
    /// centroid, which its own doc scopes to loops "concentric by
    /// construction". The thin cross is neither: measured off the
    /// body's own stored vertex positions, its nested ring's mean
    /// radius is LARGER than its outer loop's, so the mean-radius
    /// shape reads a genuinely nested ring as un-nested. The same body
    /// is an honest fixture the nesting arm blesses one row above.
    #[test]
    fn mean_radius_nesting_is_not_a_containment_statement() {
        let tol = Tol::witness();
        let band = Band::linear(tol).expect("the run's band");
        let (body, face) = lamina_with_ring(&cross_outer(), &cross_ring(), tol);
        let f = body.get_face(face).unwrap();
        let points = |lk: LoopKey| -> Vec<Point3<f64>> {
            loop_cycle_of(&body, lk)
                .unwrap()
                .into_iter()
                .filter_map(|he| {
                    vertex_point(&body, body.half_edges.get(he).map(|h| h.start).unwrap())
                })
                .collect()
        };
        let (outer_pts, ring_pts) = (points(f.outer), points(f.rings[0]));
        // The centroid of the two runs together, and each run's mean
        // distance from it — `shell::encloses`' arithmetic, restated
        // here because that helper is private to its own module.
        let origin = Point3::new(0.0, 0.0, 0.0);
        let mut sum = geom_core::Vec3::new(0.0, 0.0, 0.0);
        for p in outer_pts.iter().chain(ring_pts.iter()) {
            sum = sum + (*p - origin);
        }
        let centre = origin + sum / ((outer_pts.len() + ring_pts.len()) as f64);
        let mean = |run: &[Point3<f64>]| -> f64 {
            run.iter().map(|p| (*p - centre).norm()).sum::<f64>() / (run.len() as f64)
        };
        let (outer_mean, ring_mean) = (mean(&outer_pts), mean(&ring_pts));
        assert!(
            ring_mean > outer_mean,
            "the cross's nested ring must out-radius its outer loop for this \
             row to say anything: outer {outer_mean}, ring {ring_mean}"
        );
        // And the instrument that IS a containment statement blesses
        // the same body.
        assert!(
            check_9_words(&body, band, tol).is_empty(),
            "the containment walk reads the same body as nested"
        );
    }

    /// **The verdict is blind to orientation.** `Body::revert`
    /// reverses every cycle and flips every sense; the chart normal is
    /// handed to the walk without `Face::sense` folded in. Neither
    /// moves the FINDING. The witness may move, and that is stated in
    /// [`ValidationError::RingOutsideOuter`]'s doc rather than
    /// asserted away here, so the comparison is by variant and face.
    #[test]
    fn the_nesting_verdict_is_blind_to_orientation() {
        let tol = Tol::witness();
        let band = Band::linear(tol).expect("the run's band");
        let mut honest = ops_holed_box(tol).body;
        plane_every_face(&mut honest, tol);
        let face = honest
            .faces
            .iter()
            .find(|(_, f)| !f.rings.is_empty())
            .map(|(k, _)| k)
            .unwrap();
        let mutant = invert_roles(&honest, face);
        let shape = |b: &Body<f64>| -> Vec<String> {
            check_9_words(b, band, tol)
                .iter()
                .map(|e| match e {
                    ValidationError::RingOutsideOuter { face, .. } => {
                        format!("RingOutsideOuter({face:?})")
                    }
                    other => format!("{other:?}"),
                })
                .collect()
        };
        for (name, body) in [("honest", &honest), ("mutant", &mutant)] {
            let reverted = body.revert().unwrap();
            assert_eq!(
                shape(body),
                shape(&reverted),
                "{name}: revert moved check 9's verdict"
            );
        }
        // And directly, both signs of the chart normal at the arm.
        for (name, body) in [("honest", &honest), ("mutant", &mutant)] {
            let f = body.get_face(face).unwrap();
            let Some(&Surface::Plane { normal, .. }) = body.surfaces.get(f.surface) else {
                panic!("a plane")
            };
            let render = |v: RingNestingVerdict| match v {
                RingNestingVerdict::Inside => "Inside".to_string(),
                RingNestingVerdict::Outside { ring_vertex } => format!("Outside({ring_vertex:?})"),
                RingNestingVerdict::Undecided(e) => format!("Undecided({e})"),
            };
            assert_eq!(
                render(ring_nesting(
                    body,
                    f.outer,
                    f.rings[0],
                    NestingRegion::Polygon { normal },
                    band
                )),
                render(ring_nesting(
                    body,
                    f.outer,
                    f.rings[0],
                    NestingRegion::Polygon { normal: -normal },
                    band,
                )),
                "{name}: the verdict moved with the chart normal's sign"
            );
        }
    }

    /// **The gate is shut on a non-planar face, in BOTH directions.**
    /// A body the arm refuses while its face carries a plane goes
    /// silent when the same face carries the `Nurbs` placeholder — the
    /// residue, and the control beside it is what makes the silence a
    /// measurement rather than a vacuum.
    #[test]
    fn the_nesting_arm_is_silent_on_a_non_planar_face() {
        let tol = Tol::witness();
        let band = Band::linear(tol).expect("the run's band");
        let p = Point3::new;
        let outer = vec![
            p(0.0, 0.0, 0.0),
            p(10.0, 0.0, 0.0),
            p(10.0, 10.0, 0.0),
            p(0.0, 10.0, 0.0),
        ];
        let ring = vec![
            p(4.0, 4.0, 0.0),
            p(6.0, 4.0, 0.0),
            p(6.0, 6.0, 0.0),
            p(4.0, 6.0, 0.0),
        ];
        let (body, face) = lamina_with_ring(&outer, &ring, tol);
        let inverted = invert_roles(&body, face);
        assert!(
            check_9_words(&inverted, band, tol)
                .iter()
                .any(|e| matches!(e, ValidationError::RingOutsideOuter { .. })),
            "the control: the planar inversion IS refused"
        );
        for (name, b) in [("nested", &body), ("inverted", &inverted)] {
            let mut c = b.clone();
            c.set_face_surface(
                face,
                crate::FaceSurface::New(geom::Surface::nurbs_placeholder()),
            )
            .unwrap();
            let got = nesting_words(&c, band, tol);
            assert!(got.is_empty(), "non-planar {name}: not silent: {got:?}");
        }
    }

    /// **An arc anywhere in the outer loop shuts the gate**, in BOTH
    /// directions, and the control beside it is what makes the silence
    /// a measurement. One edge of a four-vertex outer loop re-carried
    /// as an arc moves the loop from `boolean::loop_shape`'s `Polygon`
    /// class to its `ArcParity` class, where the polygon the walk
    /// reads is a proper region but not the LOOP's region — so an
    /// `Out` from it is not a fact this arm may refuse a body on.
    #[test]
    fn an_arc_bearing_outer_loop_is_the_gates_residue() {
        let tol = Tol::witness();
        let band = Band::linear(tol).expect("the run's band");
        let p = Point3::new;
        let outer = vec![
            p(0.0, 0.0, 0.0),
            p(10.0, 0.0, 0.0),
            p(10.0, 10.0, 0.0),
            p(0.0, 10.0, 0.0),
        ];
        let ring = vec![
            p(4.0, 4.0, 0.0),
            p(6.0, 4.0, 0.0),
            p(6.0, 6.0, 0.0),
            p(4.0, 6.0, 0.0),
        ];
        let (body, face) = lamina_with_ring(&outer, &ring, tol);
        assert!(
            check_9_words(&invert_roles(&body, face), band, tol)
                .iter()
                .any(|e| matches!(e, ValidationError::RingOutsideOuter { .. })),
            "the control: the all-line inversion IS refused"
        );
        for (name, base) in [
            ("nested", body.clone()),
            ("inverted", invert_roles(&body, face)),
        ] {
            let mut c = base;
            let outer_loop = c.get_face(face).unwrap().outer;
            let LoopBoundary::Cycle { first } = c.get_loop(outer_loop).unwrap().boundary else {
                panic!("a cycle")
            };
            let edge = c.get_half_edge(first).unwrap().edge;
            let curve = c.get_edge(edge).unwrap().curve;
            // Anchored far from the fixture: the contact arms compare a
            // ring vertex against the outer loop's carrier LOCUS, and a
            // circle drawn through the fixture would fire arm 2 before
            // the nesting arm is reached at all.
            *c.curves.get_mut(curve).unwrap() =
                CurveGeom::Certified(crate::fixtures::test_curve(p(1.0e3, 1.0e3, 1.0e3), tol));
            let f = c.get_face(face).unwrap();
            assert!(
                nesting_region(&c, f.surface, f.outer, band).is_none(),
                "{name}: one arc shuts the gate"
            );
            let got = nesting_words(&c, band, tol);
            assert!(got.is_empty(), "arc-bearing {name}: not silent: {got:?}");
        }
    }

    /// **An empty ring is decided on its vertex's position.** `kemr`
    /// mints a ring holding one lone vertex and no cycle. It bounds no
    /// region, so "does it enclose the outer loop" has no answer — but
    /// "does it sit inside the face's region" does, and a lone-vertex
    /// ring planted outside is the same defect as any other ring
    /// outside. The row plants one well outside a 10 x 10 face and one
    /// inside it, and asserts the refusal names the lone vertex.
    #[test]
    fn an_empty_ring_outside_the_outer_loop_is_refused() {
        let tol = Tol::witness();
        let band = Band::linear(tol).expect("the run's band");
        let p = Point3::new;
        let outer = vec![
            p(0.0, 0.0, 0.0),
            p(10.0, 0.0, 0.0),
            p(10.0, 10.0, 0.0),
            p(0.0, 10.0, 0.0),
        ];
        let ring = vec![
            p(4.0, 4.0, 0.0),
            p(6.0, 4.0, 0.0),
            p(6.0, 6.0, 0.0),
            p(4.0, 6.0, 0.0),
        ];
        let (body, face) = lamina_with_ring(&outer, &ring, tol);
        for (name, at, outside) in [
            ("outside", p(100.0, 100.0, 0.0), true),
            ("inside", p(2.0, 2.0, 0.0), false),
        ] {
            let mut b = body.clone();
            let point = b.add_point(at);
            let vertex = b.vertices.insert(Vertex {
                point,
                emanating: None,
            });
            let lone = b.loops.insert(Loop {
                boundary: LoopBoundary::Empty { vertex },
                face,
            });
            b.faces.get_mut(face).unwrap().rings.push(lone);
            let f = b.get_face(face).unwrap();
            let Some(&Surface::Plane { normal, .. }) = b.surfaces.get(f.surface) else {
                panic!("a plane")
            };
            let verdict = ring_nesting(&b, f.outer, lone, NestingRegion::Polygon { normal }, band);
            if outside {
                assert!(
                    matches!(
                        verdict,
                        RingNestingVerdict::Outside { ring_vertex } if ring_vertex == vertex
                    ),
                    "{name}: the lone vertex must be the witness"
                );
            } else {
                assert!(
                    matches!(verdict, RingNestingVerdict::Inside),
                    "{name}: a lone vertex inside the region is no finding"
                );
            }
        }
    }

    /// **A lone-vertex ring on a DISC outer loop is decided on its one
    /// point, in all three outcomes the radial decide has.** Crate-side
    /// because no public door can put it there: tier 2 refuses an empty
    /// loop at rest (`ScaffoldingEmptyLoop`), so this is the arm's own
    /// contract on a ring the arm is written to read. The outer loop is
    /// re-carried as arcs of one circle (centre (5, 5), radius 1) so the
    /// gate reads the `Disc` class; inside is `Inside`, outside names
    /// the lone vertex, and a margin strictly between the band's
    /// coincidence and escalation thresholds is `Undecided` — never read
    /// as nested. On a CYCLE ring that third outcome is shadowed: the
    /// contact arms decide the same radial gap first
    /// ([`ring_nesting`]'s doc).
    #[test]
    fn an_empty_ring_on_a_disc_outer_loop_is_decided_three_ways() {
        let tol = Tol::witness();
        let band = Band::linear(tol).expect("the run's band");
        let p = Point3::new;
        let outer = vec![
            p(0.0, 0.0, 0.0),
            p(10.0, 0.0, 0.0),
            p(10.0, 10.0, 0.0),
            p(0.0, 10.0, 0.0),
        ];
        let ring = vec![
            p(4.0, 4.0, 0.0),
            p(6.0, 4.0, 0.0),
            p(6.0, 6.0, 0.0),
            p(4.0, 6.0, 0.0),
        ];
        let (mut body, face) = lamina_with_ring(&outer, &ring, tol);
        let outer_loop = body.get_face(face).unwrap().outer;
        let LoopBoundary::Cycle { first } = body.get_loop(outer_loop).unwrap().boundary else {
            panic!("a cycle")
        };
        for he in body.loop_cycle(first).unwrap() {
            let edge = body.get_half_edge(he).unwrap().edge;
            let curve = body.get_edge(edge).unwrap().curve;
            // `test_curve` is the unit circle about `anchor + x`.
            *body.curves.get_mut(curve).unwrap() =
                CurveGeom::Certified(crate::fixtures::test_curve(p(4.0, 5.0, 0.0), tol));
        }
        let f = body.get_face(face).unwrap();
        let region = nesting_region(&body, f.surface, f.outer, band).expect("the gate opens");
        assert!(
            matches!(region, NestingRegion::Disc(_)),
            "one circle on every outer edge is the disc class"
        );
        // Strictly between ε and K·ε for any K > 1.
        let in_band = tol.eps() * tol.k().sqrt();
        for (name, at) in [
            ("inside", p(5.3, 5.2, 0.0)),
            ("outside", p(100.0, 100.0, 0.0)),
            ("in band", p(5.0, 6.0 - in_band, 0.0)),
        ] {
            let mut b = body.clone();
            let point = b.add_point(at);
            let vertex = b.vertices.insert(Vertex {
                point,
                emanating: None,
            });
            let lone = b.loops.insert(Loop {
                boundary: LoopBoundary::Empty { vertex },
                face,
            });
            b.faces.get_mut(face).unwrap().rings.push(lone);
            let outer = b.get_face(face).unwrap().outer;
            let verdict = ring_nesting(&b, outer, lone, region, band);
            let ok = match name {
                "inside" => matches!(verdict, RingNestingVerdict::Inside),
                "outside" => matches!(
                    verdict,
                    RingNestingVerdict::Outside { ring_vertex } if ring_vertex == vertex
                ),
                _ => matches!(verdict, RingNestingVerdict::Undecided(_)),
            };
            assert!(ok, "{name}: wrong verdict");
        }
    }

    /// **A ring that CROSSES its outer loop is refused by name** — the
    /// line case: the ring walks out past `x = 10` and back, crossing
    /// the outer edge `(10, 0)–(10, 10)` at two points neither loop has
    /// a vertex at. The first cycle vertex is inside, so without the
    /// meeting arm the nesting walk settles the ring as nested and the
    /// body passes.
    #[test]
    fn a_ring_crossing_its_outer_loop_is_refused() {
        let tol = Tol::witness();
        let band = Band::linear(tol).expect("the run's band");
        let p = Point3::new;
        let ring = vec![
            p(5.0, 5.0, 0.0),
            p(15.0, 5.0, 0.0),
            p(15.0, 7.0, 0.0),
            p(5.0, 7.0, 0.0),
        ];
        let (body, face) = lamina_with_ring(&square_outer(), &ring, tol);
        let got = check_9_words(&body, band, tol);
        assert!(
            matches!(
                got.as_slice(),
                [ValidationError::RingMeetsOuter {
                    face: f,
                    contact: RingContact::EdgesMeet { .. },
                    ..
                }] if *f == face
            ),
            "one crossing, named by its edges: got {got:?}"
        );
    }

    /// The outer loop of the crossing rows: the square `[0, 10]²`.
    fn square_outer() -> Vec<Point3<f64>> {
        let p = Point3::new;
        vec![
            p(0.0, 0.0, 0.0),
            p(10.0, 0.0, 0.0),
            p(10.0, 10.0, 0.0),
            p(0.0, 10.0, 0.0),
        ]
    }

    /// Four points on the circle `(center, radius)` in `z = 0`, at the
    /// given angles in degrees, counter-clockwise.
    fn on_circle(center: Point3<f64>, radius: f64, degrees: [f64; 4]) -> Vec<Point3<f64>> {
        degrees
            .iter()
            .map(|d| {
                let t = d.to_radians();
                Point3::new(
                    center.x + radius * t.cos(),
                    center.y + radius * t.sin(),
                    0.0,
                )
            })
            .collect()
    }

    /// The angles the circular loops of these rows put their vertices
    /// at: every arc under a half turn, and none at 0° or 180°, where
    /// the rows put their crossings and tangencies — so no vertex arm
    /// can see the contact and only the meeting arms are measured.
    const RING_DEGREES: [f64; 4] = [60.0, 120.0, 240.0, 300.0];
    /// The same, for a circular outer loop.
    const OUTER_DEGREES: [f64; 4] = [45.0, 135.0, 225.0, 315.0];

    /// Re-carries `edge` of a `z = 0` lamina as the arc about `center`
    /// between its two stored end points that is under a half turn —
    /// the certified arc a sketch would have minted there.
    fn recarry_as_arc(body: &mut Body<f64>, edge: EdgeKey, center: Point3<f64>, tol: Tol) {
        recarry_as_sweep(body, edge, center, false, tol);
    }

    /// [`recarry_as_arc`], or with `long` the arc OVER a half turn —
    /// the other way round the same circle.
    fn recarry_as_sweep(
        body: &mut Body<f64>,
        edge: EdgeKey,
        center: Point3<f64>,
        long: bool,
        tol: Tol,
    ) {
        let stored = body.get_edge(edge).unwrap().clone();
        let (start, end) = edge_endpoints(body, stored.he_plus).unwrap();
        let radius = (start - center).norm();
        let u_ref = geom_core::Vec3::new(1.0, 0.0, 0.0);
        let (axis, t0, t1) = [1.0, -1.0]
            .into_iter()
            .find_map(|z| {
                let axis = geom_core::Vec3::new(0.0, 0.0, z);
                let v_ref = axis.cross(u_ref);
                let angle = |q: Point3<f64>| {
                    let w = q - center;
                    w.dot(v_ref).atan2(w.dot(u_ref))
                };
                let t0 = angle(start);
                let t1 = t0 + (angle(end) - t0).rem_euclid(std::f64::consts::TAU);
                ((t1 - t0 < std::f64::consts::PI) != long).then_some((axis, t0, t1))
            })
            .expect("one sense of the circle is the arc asked for");
        let carrier = geom::Curve3::Circle {
            center,
            axis,
            radius,
            u_ref,
        };
        let spec = geom_brep::EdgeCurveSpec::arc_of_circle(carrier, t0, t1).unwrap();
        let curve =
            geom_brep::EdgeCurve::certify(spec, start, end, |_| None, Band::linear(tol).unwrap())
                .expect("the arc certifies against its own end points");
        *body.curves.get_mut(stored.curve).unwrap() = CurveGeom::Certified(curve);
    }

    /// Re-carries every edge of `r#loop` as an arc about `center`.
    fn recarry_loop(body: &mut Body<f64>, r#loop: LoopKey, center: Point3<f64>, tol: Tol) {
        let edges: Vec<EdgeKey> = loop_cycle_of(body, r#loop)
            .unwrap()
            .into_iter()
            .map(|he| body.get_half_edge(he).unwrap().edge)
            .collect();
        for edge in edges {
            recarry_as_arc(body, edge, center, tol);
        }
    }

    /// A lamina whose outer loop is `outer` and whose ring is the circle
    /// `(ring_center, ring_radius)`, carried by four arcs; with
    /// `outer_circle` set, the outer loop's edges are re-carried as arcs
    /// of that circle too. Hand-built: no public door mints a ring that
    /// crosses or touches its outer loop — `Profile` validation refuses
    /// both — and that is the shape these rows exist to put at rest.
    fn lamina_with_circular_ring(
        outer: &[Point3<f64>],
        outer_circle: Option<Point3<f64>>,
        ring_center: Point3<f64>,
        ring_radius: f64,
        tol: Tol,
    ) -> (Body<f64>, FaceKey) {
        let ring = on_circle(ring_center, ring_radius, RING_DEGREES);
        let (mut body, face) = lamina_with_ring(outer, &ring, tol);
        let f = body.get_face(face).unwrap().clone();
        recarry_loop(&mut body, f.rings[0], ring_center, tol);
        if let Some(center) = outer_circle {
            recarry_loop(&mut body, f.outer, center, tol);
        }
        (body, face)
    }

    /// Check 9's one word on `body`, asserted to be a `RingMeetsOuter`
    /// on `face` whose contact `shape` accepts.
    fn assert_meets(
        name: &str,
        body: &Body<f64>,
        face: FaceKey,
        shape: impl Fn(&RingContact) -> bool,
    ) {
        let tol = Tol::witness();
        let got = check_9_words(body, Band::linear(tol).unwrap(), tol);
        assert!(
            matches!(
                got.as_slice(),
                [ValidationError::RingMeetsOuter { face: f, contact, .. }]
                    if *f == face && shape(contact)
            ),
            "[{name}] got {got:?}"
        );
    }

    /// **Two whole circles that cross or touch are refused, as circles**
    /// — arm 4, exact on the class. Outer: radius 5 about the origin.
    /// The crossing ring (radius 1 about `(4.3, 0)`) has every vertex
    /// inside the outer circle and bows past it between two of them,
    /// the shape `ring_nesting`'s doc says only the contact half can
    /// see. The internally tangent ring (about `(4, 0)`) and the
    /// externally tangent one (about `(6, 0)`) touch at `(5, 0)`, where
    /// neither loop has a vertex.
    #[test]
    fn two_whole_circles_that_cross_or_touch_are_refused() {
        let tol = Tol::witness();
        let o = Point3::new(0.0, 0.0, 0.0);
        let outer = on_circle(o, 5.0, OUTER_DEGREES);
        for (name, at) in [
            ("crossing", 4.3),
            ("internally tangent", 4.0),
            ("externally tangent", 6.0),
        ] {
            let (body, face) =
                lamina_with_circular_ring(&outer, Some(o), Point3::new(at, 0.0, 0.0), 1.0, tol);
            let f = body.get_face(face).unwrap();
            let (outer_loop, ring_loop) = (f.outer, f.rings[0]);
            let got = check_9_words(&body, Band::linear(tol).unwrap(), tol);
            assert!(
                matches!(
                    got.as_slice(),
                    [ValidationError::RingMeetsOuter {
                        contact: RingContact::Circles { ring_loop: r, outer_loop: q },
                        ..
                    }] if *r == ring_loop && *q == outer_loop
                ),
                "[{name}] got {got:?}"
            );
        }
    }

    /// The five placements [`circle_pair`] tells apart, each decided
    /// exactly — the same outer circle, radius 5 about the origin, and
    /// a radius-1 circle whose centre sits at `x`.
    #[test]
    fn circle_pair_names_all_five_placements() {
        let band = Band::linear(Tol::witness()).unwrap();
        let o = Point3::new(0.0, 0.0, 0.0);
        for (x, want) in [
            (7.0, CirclePair::Apart),
            (6.0, CirclePair::ExternallyTangent),
            (4.3, CirclePair::Crossing),
            (4.0, CirclePair::InternallyTangent),
            (2.0, CirclePair::Nested),
        ] {
            let got = circle_pair(o, 5.0, Point3::new(x, 0.0, 0.0), 1.0, band);
            assert_eq!(got, Ok(want), "centre at x = {x}");
        }
        // Symmetric in the pair: the larger circle as the second one.
        assert_eq!(
            circle_pair(Point3::new(4.0, 0.0, 0.0), 1.0, o, 5.0, band),
            Ok(CirclePair::InternallyTangent)
        );
    }

    /// **An edge pair meeting at a point is refused, by its edges** —
    /// arm 5 — on each carrier pairing it has a closed form for that a
    /// row can reach without arm 4: a circular ring crossing and touching
    /// a square's straight edge `x = 10` (line × arc), and crossing an
    /// outer loop one of whose edges is an arc bulging out to `x = 12`
    /// (arc × arc — the outer loop bears one arc among lines, so it is
    /// not a whole circle and arm 4 does not open). Every ring vertex
    /// in the crossing rows is inside the outer loop.
    #[test]
    fn an_edge_pair_meeting_at_a_point_is_refused() {
        let tol = Tol::witness();
        let p = Point3::new;
        let square = square_outer();
        for (name, at) in [("line × arc crossing", 9.3), ("line × arc tangency", 9.0)] {
            let (body, face) = lamina_with_circular_ring(&square, None, p(at, 5.0, 0.0), 1.0, tol);
            assert_meets(name, &body, face, |c| {
                matches!(c, RingContact::EdgesMeet { .. })
            });
        }
        // The square's right edge re-carried as the arc through
        // (10, 0) and (10, 10) about (4.75, 5): radius 7.25, bulging to
        // x = 12. The ring, radius 1 about (11.2, 5), bows past it.
        let (mut body, face) =
            lamina_with_circular_ring(&square, None, p(11.2, 5.0, 0.0), 1.0, tol);
        let outer_loop = body.get_face(face).unwrap().outer;
        let right = loop_cycle_of(&body, outer_loop)
            .unwrap()
            .into_iter()
            .map(|he| body.get_half_edge(he).unwrap().edge)
            .find(|&e| {
                let (a, b) = edge_endpoints(&body, body.get_edge(e).unwrap().he_plus).unwrap();
                a.x == 10.0 && b.x == 10.0
            })
            .expect("the square's right edge");
        recarry_as_arc(&mut body, right, p(4.75, 5.0, 0.0), tol);
        assert_meets(
            "arc × arc crossing",
            &body,
            face,
            |c| matches!(c, RingContact::EdgesMeet { outer_edge, .. } if *outer_edge == right),
        );
    }

    /// **The controls: a ring near its outer loop but clear of it
    /// certifies, and one in the band escalates rather than passing.**
    /// Each shape of the meeting rows — a whole circle inside a whole
    /// circle, and a circle beside a straight edge — placed three ways:
    /// the concentric annulus and a hole well clear of the rim
    /// (silent), a hole `4·K·ε` from the rim — the closest a clear ring
    /// can be at this run's ε, down to `4e-11` m at `ε = 1e-12` —
    /// (silent), and a hole `ε·√K` from the rim, strictly inside the
    /// band (`RingContactEscalated`, never read as clear).
    #[test]
    fn a_ring_near_its_outer_loop_is_decided_three_ways() {
        let tol = Tol::witness();
        let band = Band::linear(tol).unwrap();
        let p = Point3::new;
        let o = p(0.0, 0.0, 0.0);
        let disc = on_circle(o, 5.0, OUTER_DEGREES);
        let clear = 4.0 * tol.k() * tol.eps();
        let in_band = tol.eps() * tol.k().sqrt();
        let square = square_outer();
        // (name, outer loop, outer circle, ring centre, ring radius)
        let silent = [
            ("concentric annulus", &disc, Some(o), o, 2.0),
            (
                "off-centre hole near the rim",
                &disc,
                Some(o),
                p(3.9, 0.0, 0.0),
                1.0,
            ),
            (
                "disc hole 4Kε from the rim",
                &disc,
                Some(o),
                p(4.0 - clear, 0.0, 0.0),
                1.0,
            ),
            (
                "hole 4Kε from a straight edge",
                &square,
                None,
                p(9.0 - clear, 5.0, 0.0),
                1.0,
            ),
            (
                "hole well clear of a straight edge",
                &square,
                None,
                p(8.5, 5.0, 0.0),
                1.0,
            ),
        ];
        for (name, outer, outer_circle, center, radius) in silent {
            let (body, _) = lamina_with_circular_ring(outer, outer_circle, center, radius, tol);
            let got = check_9_words(&body, band, tol);
            assert!(got.is_empty(), "[{name}] a clear ring drew {got:?}");
        }
        for (name, outer, outer_circle, center) in [
            (
                "disc hole in band of the rim",
                disc.clone(),
                Some(o),
                p(4.0 - in_band, 0.0, 0.0),
            ),
            (
                "hole in band of a straight edge",
                square_outer(),
                None,
                p(9.0 - in_band, 5.0, 0.0),
            ),
        ] {
            let (body, face) = lamina_with_circular_ring(&outer, outer_circle, center, 1.0, tol);
            let got = check_9_words(&body, band, tol);
            assert!(
                matches!(
                    got.as_slice(),
                    [ValidationError::RingContactEscalated { face: f, .. }] if *f == face
                ),
                "[{name}] got {got:?}"
            );
        }
    }

    /// Check 9's words on a `z = 0` lamina whose outer loop is `outer`
    /// and whose ring is `ring`, after `arcs` re-carries the named
    /// edges — each `(from, to, centre)` an edge between two of the
    /// given points, re-carried as the short arc about `centre`.
    fn words_with_arcs(
        outer: &[Point3<f64>],
        ring: &[Point3<f64>],
        arcs: &[(Point3<f64>, Point3<f64>, Point3<f64>)],
    ) -> (Vec<ValidationError>, FaceKey) {
        let arcs: Vec<_> = arcs.iter().map(|&(a, b, c)| (a, b, c, false)).collect();
        words_with_sweeps(outer, ring, &arcs)
    }

    /// An edge to re-carry: its two ends, the centre, and whether the
    /// arc asked for is the one over a half turn.
    type Sweep = (Point3<f64>, Point3<f64>, Point3<f64>, bool);

    /// [`words_with_arcs`], each arc's fourth field asking for the arc
    /// over a half turn ([`recarry_as_sweep`]).
    fn words_with_sweeps(
        outer: &[Point3<f64>],
        ring: &[Point3<f64>],
        arcs: &[Sweep],
    ) -> (Vec<ValidationError>, FaceKey) {
        let tol = Tol::witness();
        let (mut body, face) = lamina_with_ring(outer, ring, tol);
        for &(from, to, centre, long) in arcs {
            let edge = body
                .edges
                .iter()
                .find(|(_, e)| {
                    edge_endpoints(&body, e.he_plus).is_some_and(|(a, b)| {
                        let same = |x: Point3<f64>, y: Point3<f64>| (x - y).norm() == 0.0;
                        (same(a, from) && same(b, to)) || (same(a, to) && same(b, from))
                    })
                })
                .map(|(k, _)| k)
                .expect("the named edge");
            recarry_as_sweep(&mut body, edge, centre, long, tol);
        }
        (check_9_words(&body, Band::linear(tol).unwrap(), tol), face)
    }

    /// **A short arc's end facing a wall, clear of it, is not a
    /// meeting** (review MAJOR-1). The ring's arc — radius 10, a
    /// 0.02-rad sweep, centre `(10 − δ, −5)` — ends at `Q = (10 − δ, 5)`
    /// facing the square's wall `x = 10`. The arc's circle meets the
    /// wall about `δ` of arc length PAST `Q`, outside the trim. An
    /// angular trim test compresses that distance by `sin(w/2) ≈ 0.01`,
    /// so it read `δ = 50ε` as the endpoint and `δ = 500ε` as the band;
    /// a trim decided as a distance reads both as clear.
    #[test]
    fn a_short_arcs_end_near_a_wall_is_clear() {
        let tol = Tol::witness();
        let p = Point3::new;
        for k in [50.0, 500.0] {
            let d = k * tol.eps();
            let c = p(10.0 - d, -5.0, 0.0);
            let q = p(10.0 - d, 5.0, 0.0);
            let s = 0.02_f64;
            let a = p(c.x - 10.0 * s.sin(), c.y + 10.0 * s.cos(), 0.0);
            let r = p(9.9 - d, 4.0, 0.0);
            let (got, _) = words_with_arcs(&square_outer(), &[q, a, r], &[(q, a, c)]);
            assert!(got.is_empty(), "[δ = {k}ε] a clear ring drew {got:?}");
        }
    }

    /// **A near-tangency far from the arc's trim is not an
    /// escalation** (review MAJOR-2). A quarter-disc hole — centre
    /// `(5, 1 + 3ε)`, radius 1, its arc from 0° to 90° — above the
    /// square's bottom edge: the circle comes within `3ε` of the edge,
    /// in the band, but at 270°, where the arc is not. The trim must
    /// be decided before the band is.
    #[test]
    fn a_near_tangency_off_the_arcs_trim_is_clear() {
        let tol = Tol::witness();
        let p = Point3::new;
        let y = 1.0 + 3.0 * tol.eps();
        let c = p(5.0, y, 0.0);
        let e = p(6.0, y, 0.0);
        let n = p(5.0, y + 1.0, 0.0);
        let (got, _) = words_with_arcs(&square_outer(), &[c, e, n], &[(e, n, c)]);
        assert!(got.is_empty(), "a clear quarter-disc drew {got:?}");
    }

    /// **An OUTER vertex on a ring edge's interior is a meeting**
    /// (review MINOR-1), the mirror of arm 2. The ring is the circle
    /// about `(5, 5)`, radius 2; the outer loop's vertex
    /// `V = (7 + δ, 5)` points at it, its two edges falling away. At
    /// `δ = ε/2` the loops touch at `V`; at `δ = 3ε` whether they do is
    /// in the band, and must escalate rather than certify.
    #[test]
    fn an_outer_vertex_on_a_ring_edge_is_a_meeting() {
        let tol = Tol::witness();
        let band = Band::linear(tol).unwrap();
        let p = Point3::new;
        let centre = p(5.0, 5.0, 0.0);
        for (k, touches) in [(0.5, true), (3.0, false)] {
            let d = k * tol.eps();
            let outer = vec![
                p(7.04 + d, 1.0, 0.0),
                p(7.0 + d, 5.0, 0.0),
                p(7.04 + d, 9.0, 0.0),
                p(0.0, 9.0, 0.0),
                p(0.0, 1.0, 0.0),
            ];
            let (body, face) = lamina_with_circular_ring(&outer, None, centre, 2.0, tol);
            let got = check_9_words(&body, band, tol);
            let ok = if touches {
                matches!(
                    got.as_slice(),
                    [ValidationError::RingMeetsOuter {
                        face: f,
                        contact: RingContact::OuterVertexOnEdge { .. },
                        ..
                    }] if *f == face
                )
            } else {
                matches!(got.as_slice(), [ValidationError::RingContactEscalated { face: f, .. }] if *f == face)
            };
            assert!(ok, "[δ = {k}ε] got {got:?}");
        }
    }

    /// **A ring vertex on an outer arc's CIRCLE but far from its trim
    /// is not a meeting** (review NOTE-2). The outer loop is a 30 × 10
    /// rectangle whose right edge bulges out as an arc of the circle
    /// about `(22, 5)` through its two corners; the D-shaped hole's arc
    /// lies on that same circle, around 180° — twenty metres from the
    /// outer arc's window. The two arcs share a carrier and no point.
    #[test]
    fn a_ring_arc_on_an_outer_arcs_circle_off_its_trim_is_clear() {
        let p = Point3::new;
        let centre = p(22.0, 5.0, 0.0);
        let radius = (64.0_f64 + 25.0).sqrt();
        let at = |deg: f64| {
            let t = deg.to_radians();
            p(
                centre.x + radius * t.cos(),
                centre.y + radius * t.sin(),
                0.0,
            )
        };
        let outer = vec![
            p(0.0, 0.0, 0.0),
            p(30.0, 0.0, 0.0),
            p(30.0, 10.0, 0.0),
            p(0.0, 10.0, 0.0),
        ];
        let (a, b) = (at(170.0), at(190.0));
        let ring = vec![a, b, p(14.0, 5.0, 0.0)];
        let (got, _) = words_with_arcs(
            &outer,
            &ring,
            &[
                (p(30.0, 0.0, 0.0), p(30.0, 10.0, 0.0), centre),
                (a, b, centre),
            ],
        );
        assert!(
            got.is_empty(),
            "a hole on the outer arc's circle drew {got:?}"
        );
    }

    /// **A SHORT arc and a NEAR-FULL arc near a wall are decided three
    /// ways**, the controls the trim's conditioning owes: an angular
    /// window compresses arc length near an arc's end by `sin(w/2)`,
    /// which is small on both. Each is placed `4·K·ε` from the square's
    /// wall `x = 10` (silent), `ε·√K` from it (escalated, never read as
    /// clear), and `1e-4` through it (refused by its edges).
    ///
    /// - The short arc: radius 10, a 0.02-rad sweep centred on 0°, its
    ///   middle nearest the wall, closed by a vertex behind it.
    /// - The near-full arc: radius 1, a 350° sweep whose 10° mouth faces
    ///   away from the wall, closed by two lines into its centre.
    #[test]
    fn short_and_near_full_arcs_near_a_wall_are_decided_three_ways() {
        let tol = Tol::witness();
        let p = Point3::new;
        let clear = 4.0 * tol.k() * tol.eps();
        let in_band = tol.eps() * tol.k().sqrt();
        let short = |gap: f64| {
            let c = p(-gap, 5.0, 0.0);
            let h = 0.01_f64;
            let a = p(c.x + 10.0 * h.cos(), 5.0 + 10.0 * h.sin(), 0.0);
            let b = p(c.x + 10.0 * h.cos(), 5.0 - 10.0 * h.sin(), 0.0);
            words_with_sweeps(
                &square_outer(),
                &[a, b, p(9.0, 5.0, 0.0)],
                &[(a, b, c, false)],
            )
        };
        let near_full = |gap: f64| {
            let c = p(9.0 - gap, 5.0, 0.0);
            let h = 5.0_f64.to_radians();
            let a = p(c.x - h.cos(), 5.0 + h.sin(), 0.0);
            let b = p(c.x - h.cos(), 5.0 - h.sin(), 0.0);
            words_with_sweeps(&square_outer(), &[a, b, c], &[(a, b, c, true)])
        };
        for (name, build) in [
            (
                "short arc",
                &short as &dyn Fn(f64) -> (Vec<ValidationError>, FaceKey),
            ),
            ("near-full arc", &near_full),
        ] {
            let (got, _) = build(clear);
            assert!(got.is_empty(), "[{name}, clear] got {got:?}");
            let (got, face) = build(in_band);
            assert!(
                matches!(got.as_slice(), [ValidationError::RingContactEscalated { face: f, .. }] if *f == face),
                "[{name}, in band] got {got:?}"
            );
            let (got, face) = build(-1e-4);
            assert!(
                matches!(
                    got.as_slice(),
                    [ValidationError::RingMeetsOuter {
                        face: f,
                        contact: RingContact::EdgesMeet { .. },
                        ..
                    }] if *f == face
                ),
                "[{name}, through] got {got:?}"
            );
        }
        // The near-full arc's MOUTH facing the wall, its carrier poking
        // through inside the mouth — 0.14 rad either side of 0°, where
        // the 0.6-rad mouth has no arc — and its ends clear inside.
        let c = p(9.01, 5.0, 0.0);
        let h = 0.3_f64;
        let a = p(c.x + h.cos(), 5.0 + h.sin(), 0.0);
        let b = p(c.x + h.cos(), 5.0 - h.sin(), 0.0);
        let (got, _) = words_with_sweeps(&square_outer(), &[a, c, b], &[(a, b, c, true)]);
        assert!(got.is_empty(), "[mouth facing the wall] got {got:?}");
    }

    /// **A ring LINE crossing an outer ARC is a meeting** — arm 5 with
    /// the line as the ring's edge: a square hole whose right side pokes
    /// out of a disc face of radius 5.
    #[test]
    fn a_ring_line_crossing_an_outer_arc_is_refused() {
        let tol = Tol::witness();
        let p = Point3::new;
        let o = p(0.0, 0.0, 0.0);
        let outer = on_circle(o, 5.0, OUTER_DEGREES);
        let ring = vec![
            p(3.0, -1.0, 0.0),
            p(6.0, -1.0, 0.0),
            p(6.0, 1.0, 0.0),
            p(3.0, 1.0, 0.0),
        ];
        let (mut body, face) = lamina_with_ring(&outer, &ring, tol);
        let outer_loop = body.get_face(face).unwrap().outer;
        recarry_loop(&mut body, outer_loop, o, tol);
        assert_meets("ring line × outer arc", &body, face, |c| {
            matches!(c, RingContact::EdgesMeet { .. })
        });
    }

    /// An arc of the `z = 0` circle `(center, radius)` from `from` to
    /// `to` degrees, counter-clockwise, as arm 5 reads it.
    fn arc_segment(center: Point3<f64>, radius: f64, from: f64, to: f64) -> MeetSegment<f64> {
        MeetSegment::Arc {
            center,
            axis: geom_core::Vec3::new(0.0, 0.0, 1.0),
            radius,
            u_ref: geom_core::Vec3::new(1.0, 0.0, 0.0),
            t0: from.to_radians(),
            t1: to.to_radians(),
        }
    }

    /// **Arm 5's branches no body in the rows above reaches first**,
    /// asked of the pair function directly (the arms before it answer
    /// these shapes on a real loop pair, which is the argument for the
    /// branches, not a test of them).
    ///
    /// - Two collinear lines: overlapping, touching end to end, and
    ///   apart along their common line.
    /// - Two arcs of ONE circle with disjoint trims: apart
    ///   (`ring_outer_meet_centres`).
    /// - Two crossing circles whose arcs hold one of the two meeting
    ///   points each (a meeting), or hold neither (apart).
    #[test]
    fn arm_5_decides_the_branches_the_loop_rows_cannot_reach() {
        let band = Band::linear(Tol::witness()).unwrap();
        let p = Point3::new;
        let n = geom_core::Vec3::new(0.0, 0.0, 1.0);
        let line = |a: (f64, f64), b: (f64, f64)| MeetSegment::Line {
            a: p(a.0, a.1, 0.0),
            b: p(b.0, b.1, 0.0),
        };
        for (name, second, meets) in [
            ("collinear overlap", line((2.0, 0.0), (5.0, 0.0)), true),
            ("collinear end to end", line((3.0, 0.0), (5.0, 0.0)), true),
            ("collinear apart", line((4.0, 0.0), (5.0, 0.0)), false),
        ] {
            let got = segments_meet(line((0.0, 0.0), (3.0, 0.0)), second, n, band);
            assert_eq!(matches!(got, EdgePair::Meet), meets, "[{name}]");
            assert!(!matches!(got, EdgePair::Unsure(_)), "[{name}] decided");
        }
        let o = p(0.0, 0.0, 0.0);
        assert!(
            matches!(
                segments_meet(
                    arc_segment(o, 2.0, 0.0, 90.0),
                    arc_segment(o, 2.0, 180.0, 270.0),
                    n,
                    band
                ),
                EdgePair::Apart
            ),
            "two arcs of one circle, disjoint trims"
        );
        // Circles about the origin and (2, 0), both radius 2: they meet
        // at (1, ±√3), i.e. at 60° and 300° on the first and at 120°
        // and 240° on the second.
        let c2 = p(2.0, 0.0, 0.0);
        assert!(
            matches!(
                segments_meet(
                    arc_segment(o, 2.0, 30.0, 90.0),
                    arc_segment(c2, 2.0, 100.0, 170.0),
                    n,
                    band
                ),
                EdgePair::Meet
            ),
            "the upper meeting point is in both trims"
        );
        assert!(
            matches!(
                segments_meet(
                    arc_segment(o, 2.0, 30.0, 90.0),
                    arc_segment(c2, 2.0, 200.0, 260.0),
                    n,
                    band
                ),
                EdgePair::Apart
            ),
            "each arc holds a different meeting point"
        );
    }

    /// **A vertex in the band of an edge's LOCUS but definitely off
    /// its TRIM is clear** (review NEW-MAJOR): arm 2 decides the trim
    /// alongside the locus, and a vertex whose trim is definitely out
    /// draws nothing whatever the locus band says. Both examples put an
    /// outer notch vertex `3ε` off a ring edge's locus, in the band,
    /// and far from the edge itself:
    ///
    /// - a quarter-disc hole (centre `(5, 5)`, radius 1, its arc from
    ///   0° to 90°) above a notch at `(5, 4 − 3ε)` — on the arc's
    ///   CIRCLE at 270°, a metre from any point of the arc;
    /// - a square hole `[4, 6]²` beside a notch at `(8, 4 − 3ε)` — on
    ///   the infinite EXTENSION of the hole's bottom edge, two metres
    ///   past its end.
    #[test]
    fn a_vertex_in_a_locus_band_off_the_trim_is_clear() {
        let tol = Tol::witness();
        let p = Point3::new;
        let e3 = 3.0 * tol.eps();
        let notch_below = vec![
            p(0.0, 0.0, 0.0),
            p(4.0, 0.0, 0.0),
            p(5.0, 4.0 - e3, 0.0),
            p(6.0, 0.0, 0.0),
            p(10.0, 0.0, 0.0),
            p(10.0, 10.0, 0.0),
            p(0.0, 10.0, 0.0),
        ];
        let c = p(5.0, 5.0, 0.0);
        let (e, n) = (p(6.0, 5.0, 0.0), p(5.0, 6.0, 0.0));
        let (got, _) = words_with_arcs(&notch_below, &[c, e, n], &[(e, n, c)]);
        assert!(got.is_empty(), "[on the arc's circle] got {got:?}");
    }

    /// The second example of [`a_vertex_in_a_locus_band_off_the_trim_is_clear`]:
    /// a notch vertex `3ε` off the infinite extension of a square hole's
    /// bottom edge, two metres past the edge's end.
    #[test]
    fn a_vertex_in_a_line_locus_band_off_the_trim_is_clear() {
        let tol = Tol::witness();
        let p = Point3::new;
        let e3 = 3.0 * tol.eps();
        let notch_right = vec![
            p(0.0, 0.0, 0.0),
            p(10.0, 0.0, 0.0),
            p(10.0, 3.5, 0.0),
            p(8.0, 4.0 - e3, 0.0),
            p(10.0, 4.5, 0.0),
            p(10.0, 10.0, 0.0),
            p(0.0, 10.0, 0.0),
        ];
        let hole = vec![
            p(4.0, 4.0, 0.0),
            p(6.0, 4.0, 0.0),
            p(6.0, 6.0, 0.0),
            p(4.0, 6.0, 0.0),
        ];
        let (got, _) = words_with_arcs(&notch_right, &hole, &[]);
        assert!(got.is_empty(), "[on the edge's extension] got {got:?}");
    }

    /// **A near-full arc whose END faces a wall, clear of it, is not a
    /// meeting** — the MAJOR-1 shape on a 350° arc, where an angular
    /// window compresses arc length near an end by `sin(175°) ≈ 0.087`.
    /// The hole is a pac-man about `(5, 5)`, radius 1, its 10° mouth
    /// facing right between `A` (5°) and `B` (355°). An outer tongue
    /// reaches in from the wall `x = 10` below `A`: its bottom edge at
    /// `A.y − 50ε`, its tip `200ε` short of `A.x`, its top edge rising
    /// gently past `A`. The arc's circle crosses both tongue edges about
    /// `50ε` of arc length past `A`, inside the mouth; no ring or outer
    /// point comes within `30ε` of the other loop.
    #[test]
    fn a_near_full_arcs_end_facing_a_wall_is_clear() {
        let tol = Tol::witness();
        let p = Point3::new;
        let eps = tol.eps();
        let c = p(5.0, 5.0, 0.0);
        let h = 5.0_f64.to_radians();
        let a = p(5.0 + h.cos(), 5.0 + h.sin(), 0.0);
        let b = p(5.0 + h.cos(), 5.0 - h.sin(), 0.0);
        let outer = vec![
            p(0.0, 0.0, 0.0),
            p(10.0, 0.0, 0.0),
            p(10.0, a.y - 50.0 * eps, 0.0),
            p(a.x - 200.0 * eps, a.y - 50.0 * eps, 0.0),
            p(10.0, a.y + 0.01, 0.0),
            p(10.0, 10.0, 0.0),
            p(0.0, 10.0, 0.0),
        ];
        let (got, _) = words_with_sweeps(&outer, &[a, b, c], &[(a, b, c, true)]);
        assert!(got.is_empty(), "a clear pac-man drew {got:?}");
    }

    /// **M5 S10 acceptance row 1: tier 3 is the sense gate (check 6).**
    ///
    /// A face's outward normal is its surface's chart normal with
    /// `Face::sense` folded in, and by the interior-left rule its outer
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
    /// before the flip". The fixture is [`crate::test_support_fixtures::declined_cube`] with real planes
    /// grafted on; its twelve chords stay conventional, so the honest
    /// report is about those chords and nothing else. (The all-green
    /// variant of this row, on the fully certified cube, lives in
    /// `tests/geometric_cube.rs`.)
    ///
    /// **Re-expressed at PCURVE P-1b.** The honest report used to be
    /// twelve `TransverseNotIntrinsic` complaints; U2's transience
    /// fence added a second, independent at-rest rule that the same
    /// twelve chords break — they were minted through the Euler-op
    /// door, which describes an edge before any face surface exists,
    /// and `plane_every_face` grafts the planes without restating
    /// them. The row asserts the PAIR — one report per rule per chord
    /// and nothing else — rather than widening the `matches!` to admit
    /// a second variant, which would have let a body with eleven of
    /// one and thirteen of the other through.
    #[test]
    fn tier_three_refuses_a_hand_flipped_face_sense() {
        let mut cube = declined_cube::<f64>(Tol::witness()).body;
        plane_every_face(&mut cube, Tol::witness());
        let honest = validate_geometric(&cube, Tol::witness()).unwrap_err();
        let edges: Vec<EdgeKey> = cube.edges().map(|(k, _)| k).collect();
        let named = |pick: fn(&ValidationError) -> Option<EdgeKey>| {
            honest.iter().filter_map(pick).collect::<Vec<_>>()
        };
        assert_eq!(
            named(|e| match e {
                ValidationError::ScaffoldAtRest { edge } => Some(*edge),
                _ => None,
            }),
            edges,
            "the fence names every chord still at the scaffolding \
             door, once; got {honest:?}"
        );
        assert_eq!(
            named(|e| match e {
                ValidationError::TransverseNotIntrinsic { edge } => Some(*edge),
                _ => None,
            }),
            edges,
            "prefer-intrinsic names every declared transverse chord, \
             once; got {honest:?}"
        );
        assert_eq!(
            honest.len(),
            2 * edges.len(),
            "the grafted cube's only complaints are its conventional \
             chords; got {honest:?}"
        );

        let (face, f) = cube.faces.iter().next().unwrap();
        let outer = f.outer;
        let flipped = cube.flipped_face_sense_for_tests(face).unwrap();
        let errors = validate_geometric(&flipped, Tol::witness()).unwrap_err();
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
        let volume = |b: &Body<f64>| {
            crate::props::mass_properties(b, Tol::witness())
                .unwrap()
                .volume
        };
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
                Tol::witness(),
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
                Tol::witness(),
            )
            .unwrap();
        body.mef_chord(
            MefSite::Chords {
                he1: seg.he_plus,
                he2: seg.he_minus,
            },
            Tol::witness(),
        )
        .unwrap();
        let strut = body
            .mev_line(
                MevSite::Fan {
                    he1: seg.he_plus,
                    he2: seg.he_plus,
                },
                p(2.0),
                Tol::witness(),
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
                Tol::witness(),
            )
            .unwrap();
        body.mef_chord(
            MefSite::Chords {
                he1: seg.he_plus,
                he2: seg.he_minus,
            },
            Tol::witness(),
        )
        .unwrap();
        let strut = body
            .mev_line(
                MevSite::Fan {
                    he1: seg.he_plus,
                    he2: seg.he_plus,
                },
                p(2.0),
                Tol::witness(),
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
                Tol::witness(),
            )
            .unwrap();
        body.mef_chord(
            MefSite::Chords {
                he1: seg.he_plus,
                he2: seg.he_minus,
            },
            Tol::witness(),
        )
        .unwrap();
        let strut = body
            .mev_line(
                MevSite::Fan {
                    he1: seg.he_plus,
                    he2: seg.he_plus,
                },
                p(2.0),
                Tol::witness(),
            )
            .unwrap();
        let kill = body.kemr(strut.he_plus, strut.he_minus).unwrap();
        let grow = body
            .mev_line(MevSite::Lone { r#loop: kill.ring }, p(3.0), Tol::witness())
            .unwrap();
        body.mef_chord(
            MefSite::Chords {
                he1: grow.he_plus,
                he2: grow.he_minus,
            },
            Tol::witness(),
        )
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
                Tol::witness(),
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
                Tol::witness(),
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
                let choice = seqgen::choose_op(&body, d1, d2, Tol::witness())
                    .expect("an op always applies");
                seqgen::apply(&mut body, choice, &mut counter, Tol::witness());
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
                let choice = seqgen::choose_op(&body, d1, d2, Tol::witness())
                    .expect("an op always applies");
                seqgen::apply(&mut body, choice, &mut counter, Tol::witness());
            }
            seqgen::teardown(&mut body, Tol::witness());
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
            let t = ngon_pillow(n, Tol::witness());
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
            let t = raw_prism(n, Tol::witness());
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

    /// A census decline says WHICH lane declined and why, so two
    /// declines with opposite recourses do not read as one refusal.
    ///
    /// The pair here is the sharpest one the chart-region doors have. A
    /// `TouchingBoundary` decline is a statement about the GEOMETRY —
    /// the trims touch, the area is not decidable at this ε — and a
    /// `WitnessBudgetExhausted` decline is a statement about the
    /// WORK: the interior-witness search stopped, on a pair whose
    /// overlap may be fat and perfectly decidable. The repairs are
    /// unrelated, and while the census flattened both onto its
    /// subject the two sentences were byte-identical.
    ///
    /// The falsifier is the inequality below: drop the cause from
    /// either push site in `census.rs` and the two messages coincide
    /// again.
    ///
    /// **Every quantity here is derived, none restated.** The segment
    /// figure comes from [`crate::chart_region::WITNESS_BUDGET`], so
    /// raising the cap moves this row with it instead of leaving it
    /// green over a state the guard can no longer reach; and each
    /// arm's reason is asserted as its classifier's own output rather
    /// than as a fragment this row believes the classifier emits.
    #[test]
    fn a_census_decline_names_the_lane_and_the_arm_that_declined() {
        let subject = CensusSubject::FacePair(FaceKey::default(), FaceKey::default());
        let says = |cause: CensusUnsupportedCause| {
            let msg = ValidationError::CensusUnsupported {
                subject,
                cause: cause.clone(),
            }
            .to_string();
            // The lane's reason reaches the reader as the classifier
            // words it, with the one recourse that fits it — never the
            // cause's `Debug`, which is the S6 bug one variant over,
            // and never its library-caller sentence.
            let (why, recourse) = super::classify_census_cause(&cause);
            assert!(msg.ends_with(&format!("{why}. {recourse}")), "{msg}");
            assert!(!msg.contains(&format!("{cause:?}")), "{msg}");
            msg
        };

        let thin = says(CensusUnsupportedCause::ChartRegion(
            ChartRegionError::TouchingBoundary,
        ));
        // One past the cap: the state the guard actually answers, and
        // it moves when the cap moves.
        let over_cap = crate::chart_region::WITNESS_BUDGET.segments + 1;
        let stopped = says(CensusUnsupportedCause::ChartRegion(
            ChartRegionError::WitnessBudgetExhausted {
                segments: over_cap,
                cells: 0,
            },
        ));
        assert_ne!(thin, stopped);
        assert!(
            stopped.contains("their boundaries cross too many times for the check to finish"),
            "{stopped}"
        );
        assert!(
            thin.contains("the faces' edges touch at this tolerance"),
            "{thin}"
        );
        // And the blanket recourse the arm used to append to every
        // decline is gone: it is the inventory lanes' repair, and it
        // is the wrong instruction for a stopped search.
        assert!(!stopped.contains("separate the geometry"), "{stopped}");

        // The other two lanes compose from their own vocabularies.
        // The `what` is one of production's own, copied from
        // `boolean/contact_verify.rs`'s Rest-ladder arm rather than
        // invented, so a reader grepping the string finds the site
        // that emits it.
        let contact = says(CensusUnsupportedCause::ContactLane(
            ContactRefusal::NotCertifiable {
                what: "a declared face's surface kind is outside the Rest ladder's \
                       inventory (plane, sphere, cylinder)",
            },
        ));
        // `NotCertifiable` carries NO recourse on purpose
        // (`contact.rs`): a declaration cannot move a configuration
        // inside the certifiable set, so the menu the census used to
        // append there was a false lead, and its absence is the fix
        // rather than a loss.
        assert!(!contact.contains("separate the geometry"), "{contact}");
        let unboundable = ValidationError::CensusUnsupported {
            subject: CensusSubject::Entity(EntityId::Face(FaceKey::default())),
            cause: CensusUnsupportedCause::FaceUnboundable,
        }
        .to_string();
        assert!(
            unboundable.contains("a face has no corner to bound it by"),
            "{unboundable}"
        );
        assert!(
            contact.ends_with("There is no way through yet"),
            "a declaration cannot move a configuration into the certifiable set, so the \
             finding says plainly that nothing does yet: {contact}"
        );
        assert_ne!(contact, unboundable);
    }

    /// **Every decline the census can raise ends in the repair that
    /// fits its cause.**
    ///
    /// This arm once appended one blanket tail to all of them, naming
    /// the wrong repair for every cause it did not describe. The
    /// repair is now chosen per cause by the classifier, and this row
    /// holds three of them to theirs.
    ///
    /// [`ContactRefusal::NotCertifiable`] gets no repair to make —
    /// `contact.rs` ratified that the declare-or-move menu is a false
    /// lead there, since a declaration cannot move a configuration into
    /// the certifiable set — so the finding says plainly that there is
    /// no way through yet, as the refusal standard asks of a dead end.
    #[test]
    fn no_census_decline_renders_without_a_repair_to_make() {
        let what = "a declared face's surface kind is outside the Rest ladder's \
                    inventory (plane, sphere, cylinder)";
        // The chart lane's own row proves all thirteen of its arms
        // (`chart_region::tests::every_chart_region_arm_names_a_recourse`);
        // what this one adds is that the census's wrapper does not
        // swallow it, and that the other two causes are covered too.
        let chart = ValidationError::CensusUnsupported {
            subject: CensusSubject::FacePair(FaceKey::default(), FaceKey::default()),
            cause: CensusUnsupportedCause::ChartRegion(ChartRegionError::TouchingBoundary),
        }
        .to_string();
        assert!(
            chart.ends_with(
                "Recourse: move the edges clearly apart or clearly across each other, or \
                 lower the tolerance"
            ),
            "{chart}"
        );
        let unboundable = ValidationError::CensusUnsupported {
            subject: CensusSubject::Entity(EntityId::Face(FaceKey::default())),
            cause: CensusUnsupportedCause::FaceUnboundable,
        }
        .to_string();
        assert!(
            unboundable.ends_with("this is a kernel defect or a damaged file; report it"),
            "{unboundable}"
        );
        let contact = ValidationError::CensusUnsupported {
            subject: CensusSubject::Entity(EntityId::Edge(EdgeKey::default())),
            cause: CensusUnsupportedCause::ContactLane(ContactRefusal::NotCertifiable { what }),
        }
        .to_string();
        assert!(
            contact.ends_with("There is no way through yet"),
            "{contact}"
        );
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
            // An INVENTED witness, not one this test sampled: no
            // census ran here, and `census::witness` renders an f64
            // triple as `(0.0, 0.0, 0.0)`. The arm treats the field as
            // opaque, so any triple-shaped string exercises it.
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

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod review_census_display_keys {
    /// **A census payload's keys ride in its `Debug`, never in its
    /// `Display`.** The `Display` is the sentence a person at the
    /// viewer reads, where an arena key is developer detail (the
    /// refusal standard); the typed payload — its `Debug`, and the
    /// fields a caller resolves against — carries every key. Both
    /// halves are held here, over every payload variant the samples
    /// render, so a rewording that put a key back on screen reds, and
    /// so does one that lost a key from the payload.
    ///
    /// Multiplicity, not containment, for the `Debug` half: the
    /// same-type pairs (`a`/`b`) are built from ONE key, so a payload
    /// that dropped either half would still contain it once.
    #[test]
    fn census_payload_keys_ride_in_debug_and_not_on_screen() {
        let samples = crate::test_support_samples::validation_error_samples();
        let mut checked = 0;
        for (label, e) in &samples {
            let keys_in_debug = format!("{e:?}").matches("Key(").count();
            match e {
                super::ValidationError::UndeclaredContact { contact, .. } => {
                    assert!(!contact.to_string().contains("Key("), "{label}: {contact}");
                    // Every contact names two entities.
                    assert!(keys_in_debug >= 2, "{label}: {e:?}");
                    checked += 1;
                }
                super::ValidationError::StaleContactDeclaration { declaration } => {
                    assert!(
                        !declaration.to_string().contains("Key("),
                        "{label}: {declaration}"
                    );
                    let want = match declaration {
                        super::StaleDeclaration::CurveLocus { .. } => 3,
                        _ => 2,
                    };
                    assert_eq!(keys_in_debug, want, "{label}: {e:?}");
                    checked += 1;
                }
                _ => {}
            }
        }
        assert_eq!(
            checked, 12,
            "every CensusContact and StaleDeclaration sample"
        );
    }
}

/// **Check 1's offset-fit door, as the pass takes it** — the rows that
/// say what each of its two answers costs.
///
/// The battery is called directly because these rows are about the
/// PARAMETER: the public doors read the scalar's own seam
/// (`crate::AtRestPolicy::offset_fit_lane`), and a row that could only reach the
/// door the seam hands it could not tell an absent door from a scalar
/// that has none.
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod offset_fit_door_rows {
    use geom_brep::OffsetFitLane;
    use geom_core::{Band, Tol};

    use super::{DeclaredContact, PlusVCheck, ValidationError, tier3_local_checks_marked};
    use crate::entity::FaceKey;
    use crate::fixtures::approx_faced_body;

    /// The battery over the one-`Approx`-face seed body, with whatever
    /// door the caller names.
    fn check1<T: geom_core::Decide + geom_core::Bounds + crate::props::AtRestPolicy>(
        door: Option<OffsetFitLane<T>>,
    ) -> (Vec<ValidationError>, FaceKey) {
        let tol = Tol::witness();
        let band = Band::linear(tol).unwrap();
        let (body, face) = approx_faced_body::<T>();
        let declarations: [DeclaredContact; 0] = [];
        let mut marks = slotmap::SecondaryMap::new();
        let (errors, _) = tier3_local_checks_marked(
            &body,
            &declarations,
            band,
            &mut marks,
            tol,
            PlusVCheck::NotMade,
            None,
            door,
        );
        (errors, face)
    }

    /// **No door: the face is REPORTED, not skipped**, with the variant
    /// and the payload the absence has always had.
    #[test]
    fn no_door_refuses_the_approx_face_by_name() {
        let (errors, face) = check1::<f64>(None);
        assert!(
            errors.iter().any(
                |e| matches!(e, ValidationError::ApproxLaneUnsupported { face: f } if *f == face)
            ),
            "check 1 must report the face it could not re-derive: {errors:?}"
        );
    }

    /// **The `f64` door: the same face passes check 1**, and the
    /// certificate arm never fires — the door re-derives the claim the
    /// mint made, on the surface the mint made it about.
    #[test]
    fn the_f64_door_re_derives_the_approx_face() {
        let (errors, _) = check1::<f64>(Some(OffsetFitLane::fit()));
        assert!(
            !errors
                .iter()
                .any(|e| matches!(e, ValidationError::ApproxLaneUnsupported { .. })),
            "the `f64` door is present, so the absence arm may not fire: {errors:?}"
        );
        assert!(
            !errors
                .iter()
                .any(|e| matches!(e, ValidationError::ApproxCertification { .. })),
            "the surface was minted at this tolerance, so its re-derivation must hold: {errors:?}"
        );
    }

    /// **A refusing scalar reaches the same arm through its own seam.**
    /// The subject is the `f64` mint's surface lifted verbatim, which is
    /// the point: an `ApproxSurface<T>` is representable here, so the
    /// face arrives and the absence is about the DERIVATION.
    #[cfg(feature = "probe")]
    #[test]
    fn the_probe_seam_reaches_the_same_refusal() {
        let (errors, face) = check1::<geom_core::Probe>(
            <geom_core::Probe as crate::props::AtRestPolicy>::offset_fit_lane(),
        );
        assert!(
            errors.iter().any(
                |e| matches!(e, ValidationError::ApproxLaneUnsupported { face: f } if *f == face)
            ),
            "the probe scalar has no fit, so its face must report the absence: {errors:?}"
        );
    }

    /// **The seam is what the passes actually read.** The rows above
    /// name the door by hand; this one takes [`tier3_local_checks`],
    /// which reads `T::offset_fit_lane()` — so an `f64` arm that stopped
    /// answering would refuse every `Approx` face in production, and
    /// this is the row that says so.
    #[test]
    fn the_f64_seam_is_what_check_1_reads() {
        let tol = Tol::witness();
        let band = Band::linear(tol).unwrap();
        let (body, face) = approx_faced_body::<f64>();
        let declarations: [DeclaredContact; 0] = [];
        let (errors, _) = super::tier3_local_checks(
            &body,
            &declarations,
            band,
            tol,
            None,
            Some(crate::props::QuadLane::certified()),
        );
        assert!(
            !errors.iter().any(
                |e| matches!(e, ValidationError::ApproxLaneUnsupported { face: f } if *f == face)
            ),
            "the `f64` seam answers the door, so check 1 must re-derive rather than refuse: \
             {errors:?}"
        );
    }

    /// A scalar with no offset fit: its seam answers `None`. One row
    /// per scalar.
    fn no_door<T: crate::props::AtRestPolicy>(named: &str) {
        assert!(
            T::offset_fit_lane().is_none(),
            "{named} has no offset fit, so its seam must answer `None`"
        );
    }

    /// **The seam's roster**: `f64` answers the door, and the tiers
    /// over it do not. A scalar that gained one silently would pass
    /// every green corpus and only these rows.
    #[test]
    fn only_f64_answers_the_seam() {
        assert!(
            <f64 as crate::props::AtRestPolicy>::offset_fit_lane().is_some(),
            "the `f64` fit is the door's one constructor"
        );
        no_door::<geom_core::Sym<f64>>("the symbolic tier over `f64`");
        no_door::<geom_core::Dual<f64>>("the dual tier over `f64`");
    }

    /// The recording scalar is `f64` with a sink attached everywhere
    /// else, and it still has no fit: the fit is written at `f64` the
    /// TYPE, not at "whatever behaves like `f64`".
    #[cfg(feature = "probe")]
    #[test]
    fn the_probe_has_no_door() {
        no_door::<geom_core::Probe>("the telemetry probe");
    }

    /// The certifying interval scalar certifies plenty and still has no
    /// fit — the arm that makes the two absences distinguishable.
    #[test]
    fn the_interval_scalar_has_no_door() {
        no_door::<geom_core::interval::Interval>("the interval scalar");
    }

    /// **The door IS the free function**, limb for limb, bit for bit —
    /// the assertion that the bodies moved rather than being rewritten.
    ///
    /// **What this row does NOT see**: a door re-pointed at the
    /// neighbouring `_at` instrument. The certificate's limbs are
    /// MEASUREMENTS of the pair in front of them and do not move with
    /// the target; the target only classifies, which is the invariant
    /// `ApproxCertification`'s own doc asserts. What sees that is the
    /// wiring row beside the door
    /// (`geom_brep::offset_fit_lane`'s `wiring_rows`, which compares
    /// the stored function pointer) and the `_at` census
    /// (`tests/shell_tolerance_chain.rs`, which reds on a door in that
    /// file reaching any numeric-target routine but the remap's).
    /// What this row holds is that the body behind the door is the
    /// same derivation the free function runs.
    #[test]
    fn the_door_is_the_offset_fit_module_bit_for_bit() {
        let tol = Tol::witness();
        let band = Band::linear(tol).unwrap();
        let approx = crate::fixtures::bowed_offset_approx::<f64>();
        let door = OffsetFitLane::fit()
            .recertify(&approx, tol, band)
            .expect("the surface re-certifies at the tolerance it was minted at");
        let free = geom_brep::recertify_approx(&approx, tol, band)
            .expect("the free function agrees that it re-certifies");
        crate::fixtures::assert_certificates_agree("check 1's recertify door", &door, &free);
    }
}
