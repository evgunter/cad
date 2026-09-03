//! `merge_coplanar_faces` — explicit opt-in maximal-faces normalization
//! (M3 PR 1, fork F7): merge maximal runs of adjacent faces whose
//! planes are **structurally or declaredly** the same, killing the
//! shared edges and re-homing rings.
//!
//! Ch. 15's booleans require maximal-faced operands (no two adjacent
//! coplanar faces), and the seam zip *manufactures* coplanar pairs by
//! construction — so F7 ratified a fail-loud precondition on the
//! boolean side plus this **public, explicit** normalization op (the
//! M2 no-automatic-face-merging ratification: merging is never
//! silent; boolean outputs run this op as a documented final stage of
//! their own contract).
//!
//! **Coincidence discipline (the F6/round-8 ladder; N6 retirement,
//! M4 PR 5)**: two adjacent faces merge iff their surfaces are the
//! *same key* (structural), the *same [`crate::GeomSource`]*
//! (declared — shared recipe source, syntactic identity), or a
//! *declared face pair* of the call
//! ([`Body::merge_coplanar_faces_declared`] — recipe intent, verified
//! not trusted). The M3-era bit-identical-description rung is RETIRED:
//! a pair that is merely **numerically or bitwise** value-equal — same
//! plane, independent sources — stays unmerged **by design** (the
//! ladder's ratified rung (b): coincidence is never inferred from
//! values; the boolean's `NonMaximalFaces` gate agrees — the ladder is
//! consistent end to end). Since M5 PR 9 (C12.5) the hard rungs are
//! **kind-agnostic**: same-key and same-source CURVED neighbors merge
//! through the same never-numeric ladder (the cosurface
//! generalization — the boolean zip's cylinder-wall re-merge is the
//! named consumer); only the per-call declared-PAIR rung stays planar
//! (its verification predicate is `oriented_plane_eq`; the curved
//! counterpart's verification is the contact census's — CONTACT-DESIGN
//! C2's decision procedure and C4's per-class tables — not a
//! merge-local predicate).
//!
//! Serves the ch. 15 boolean pipeline's operand precondition and
//! output stage (M3 PRs 4–5).

use std::collections::BTreeMap;

use geom::Surface;
use geom_core::{Band, BandError, Decide, Indeterminate, Margin, Tol};
use slotmap::SecondaryMap;

use crate::body::Body;
use crate::boolean::{PlaneDesc, PlaneEqError, PlaneIdentity, PlaneRelation, oriented_plane_eq};
use crate::entity::{EdgeKey, EntityId, FaceKey, GeomRef, LoopKey, VertexKey};
use crate::euler::EulerOpError;
use crate::geometry::SurfaceKey;
use crate::readback::DanglingRef;
use crate::validate::{ValidationError, validate_closed};

/// One merged run: the surviving face and what was consumed into it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MergedGroup {
    /// The surviving face (the group's first face in face-arena order).
    pub kept: FaceKey,
    /// The absorbed faces (dead keys), in kill order.
    pub absorbed: Vec<FaceKey>,
    /// The killed shared edges (dead keys), in kill order.
    pub killed_edges: Vec<EdgeKey>,
    /// Rings minted by intra-face shared-edge kills (`kemr` — a merged
    /// run that surrounds a hole grows a genuine ring), in mint order.
    pub rings_made: Vec<LoopKey>,
    /// Vertices killed by the straight-seam repair (`kev`), in kill
    /// order — a junction that was interior to one straight carrier
    /// and went with its seam.
    ///
    /// Recorded rather than left implicit because this is the one
    /// thing the op destroys that no other field names: `absorbed`
    /// carries the dead faces and `killed_edges` the dead edges, and
    /// without this a caller reconciling the Euler delta would find a
    /// `v −1` with nothing accounting for it.
    pub killed_vertices: Vec<VertexKey>,
}

/// A merge group that was NOT glued: its shape is outside the merge's
/// never-elide Euler inventory. Loud in the record, never a silent
/// drop — and never a partial commit.
///
/// **The scope statements below are the KERNEL's, not the type's.**
/// Both fields are public and there is no private constructor, so
/// anything outside this crate can build a `SkippedMerge` holding any
/// [`MergeCoplanarError`] at all. What the kernel guarantees is about
/// the values IT produces — the ones reached through
/// [`MergeCoplanarOutcome::skipped`]; a record an external caller
/// mints carries no such promise and none is claimed for it.
#[derive(Clone, Debug, PartialEq)]
pub struct SkippedMerge {
    /// The group's faces (group order).
    pub faces: Vec<FaceKey>,
    /// The inventory refusal that stopped the glue, carried whole:
    /// the same [`MergeCoplanarError`] vocabulary the door refuses
    /// with, so a caller matches this exactly as it would match an
    /// `Err` ([`Body::merge_coplanar_faces_declared`] states which
    /// group gets which regime). Two variants are scope-specific and
    /// tell the reader which gate spoke:
    /// [`MergeCoplanarError::GroupNotClosed`] reaches a caller only
    /// here, and [`MergeCoplanarError::ResultNotClosed`] — the whole
    /// run's after-gate — only as an `Err`.
    pub reason: MergeCoplanarError,
}

/// The outcome of one [`Body::merge_coplanar_faces`] call.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct MergeCoplanarOutcome {
    /// The merged runs, in group order (first face's arena order).
    pub groups: Vec<MergedGroup>,
    /// Groups left unmerged as outside the inventory, with the
    /// refusal that stopped each. Non-empty needs no declaration: a
    /// curved run that would close its chart's full period is
    /// recorded here through either entry point.
    pub skipped: Vec<SkippedMerge>,
}

/// Which ladder rung licensed one mergeable adjacency (crate-
/// internal: declared-pair-licensed groups get per-group staging).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum MergeRung {
    /// Same surface key or same GeomSource — the ratified hard rungs.
    Hard,
    /// A per-call declared surface pair.
    DeclaredPair,
}

/// One half-edge resolved to the facts the merge's scans read through
/// it, taken in a single walk (see [`Body::edge_halves`]).
#[derive(Clone, Copy, Debug)]
struct HalfEdgeFacts {
    /// The parent loop.
    r#loop: LoopKey,
    /// The parent loop's face.
    face: FaceKey,
    /// The half-edge's start vertex.
    start: VertexKey,
}

/// What becomes of one group's INVENTORY refusal — the door's two
/// failure regimes, named so the boundary between them is a value
/// rather than a condition spelled inline.
///
/// Both regimes raise the SAME [`MergeCoplanarError`], which is why
/// nothing here duplicates that enum. Which regime a group runs under
/// is a property of the group — how its adjacency was licensed, and
/// whether its surface is curved.
///
/// **The regime governs inventory failures and nothing else.** An
/// inventory failure says *this group cannot be merged*; a stale key
/// or a stale geometry reference says nothing about the group at all,
/// it reports a torn arena. That is a fact about the whole body, so
/// it refuses the call under BOTH regimes
/// ([`MergeCoplanarError::is_arena_fault`]) and never becomes a skip
/// record. The split is over inventory refusals; the escape is a
/// class of refusal, not a third regime.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum GroupRegime {
    /// An inventory refusal is the CALL's refusal: nothing commits
    /// and the body is untouched. Structural planar runs — the
    /// ratified whole-refusal semantics.
    RefusesTheCall,
    /// An inventory refusal is recorded in
    /// [`MergeCoplanarOutcome::skipped`] and the remaining groups
    /// commit. Declared-licensed runs (the declaration served the
    /// consuming op's classification even where the glue is outside
    /// the inventory) and curved runs (a full-period closure keeps
    /// the operands' cut-carrying canonical form). The group is
    /// staged on its own clone and adopted only after its own tier-2
    /// gate, so a recorded skip is never a partial commit.
    RecordsASkip,
}

impl GroupRegime {
    /// Whether one group's refusal is RECORDED (`true`) or refuses
    /// the whole call (`false`).
    ///
    /// This is the whole of the regime split, in one place with one
    /// production call site, so the rule and the code that applies it
    /// cannot drift: a refusal is recorded only under
    /// [`GroupRegime::RecordsASkip`] **and** only when it is an
    /// inventory refusal rather than an arena fault
    /// ([`MergeCoplanarError::is_arena_fault`]).
    fn records(self, reason: &MergeCoplanarError) -> bool {
        self == Self::RecordsASkip && !reason.is_arena_fault()
    }
}

/// A refused [`Body::merge_coplanar_faces`] call (closed enum, D3
/// style). **Returned as an `Err`, the body is untouched on every
/// variant** — the op stages its work on a clone and commits only a
/// tier-2-valid result. The same values also ride
/// [`SkippedMerge::reason`], where they are not refusals of the call
/// and carry no such promise: the call succeeded and the groups that
/// merged are in the body.
#[derive(Clone, Debug, PartialEq)]
pub enum MergeCoplanarError {
    /// The input is not a tier-2 closed solid — normalization is
    /// defined on at-rest bodies ("tier-valid before").
    InputNotClosed {
        /// The tier-1/2 failures.
        errors: Vec<ValidationError>,
    },
    /// The WHOLE run's merged result failed tier 2 ("tier-valid
    /// after") — the configuration is outside what this op can safely
    /// merge (e.g. a kill sequence that would strand scaffolding);
    /// refused whole, nothing commits. Its scope is the run: every
    /// group that merged is in the abandoned body. One group's own
    /// trial failing is the separate [`MergeCoplanarError::GroupNotClosed`],
    /// so a consumer holding either always knows which gate spoke.
    ResultNotClosed {
        /// The tier-1/2 failures of the abandoned attempt.
        errors: Vec<ValidationError>,
    },
    /// ONE group's staged trial failed tier 2, so that group is not
    /// adopted. Scope is the single group, named by the record that
    /// carries this ([`SkippedMerge::faces`]); the run continues and
    /// `work` is exactly as it was before the trial. The kernel
    /// raises it only under [`GroupRegime::RecordsASkip`] — the
    /// refusing regime runs no sub-stage gate — so it never appears
    /// as an `Err` of the door. That is a fact about where the kernel
    /// constructs it, not something the type enforces.
    GroupNotClosed {
        /// The tier-1/2 failures of the abandoned trial.
        errors: Vec<ValidationError>,
    },
    /// A merge group's members are not all of one surface KIND, so
    /// the run is neither a planar run nor a curved one and there is
    /// no regime to give it.
    ///
    /// The hard rungs glue on *source* identity, not on kind, and
    /// [`Body::set_surface_source`] is a public door that stamps a
    /// source without comparing the descriptions it joins — so a
    /// caller can declare a plane and a cylinder to be one recipe
    /// surface. Deciding the group's kind off one member would let
    /// arena order pick its contract; this refuses instead.
    GroupKindSplit {
        /// A planar member.
        planar: FaceKey,
        /// A member that is not planar.
        curved: FaceKey,
    },
    /// A shared edge's two halves lie in **different loops of one
    /// face** after absorption (a ring-adjacent merge shape) — outside
    /// the M2+PR-7 inventory this op handles; refused rather than
    /// guessed at (the kev/ring bookkeeping for it arrives with the
    /// pipeline that produces it, if any does).
    UnsupportedConfiguration {
        /// The edge the op cannot safely kill.
        edge: EdgeKey,
    },
    /// A CURVED cosurface run would close its chart's full period
    /// (M5 PR 9, C12.5): killing the last shared edge leaves either a
    /// ring on a curved face or a full-wrap seam-form loop — shapes
    /// the exact-B-rep props cannot integrate yet, so the run refuses
    /// here and the driver records a loud skip (the operands'
    /// cut-carrying canonical form stays; sub-period re-merges — the
    /// through-cut case — commit normally). FLIP NOTE: the du_of_rims
    /// per-level-sum repair may already make the kept-cut seam form
    /// integrable; re-evaluating this skip belongs to M5 PR 11 (the
    /// tessellation/props unit), which owns the curved-face
    /// quadrature story.
    PeriodClosure {
        /// The shared edge whose kill would close the period.
        edge: EdgeKey,
    },
    /// An internal Euler step refused — surfaced typed (unreachable on
    /// tier-2 input in the supported inventory; never a panic, D9).
    /// Also the spelling for a dangling reference the surgery's own
    /// plan steps observe: [`crate::DanglingRef`] maps into
    /// [`EulerOpError::StaleKey`] / [`EulerOpError::StaleGeometry`],
    /// the state is the one the operators name and the caller's
    /// recourse is identical, so it is not given a second name here.
    /// Those two are the arena faults that escape [`GroupRegime`]'s
    /// split ([`MergeCoplanarError::is_arena_fault`]).
    Op {
        /// The refusing operator's error.
        error: EulerOpError,
    },
    /// A declared surface pair references a key that does not
    /// resolve, or a non-plane surface (M4 PR 5) — a caller bug,
    /// refused up front.
    InvalidDeclaration {
        /// The offending surface key.
        surface: SurfaceKey,
        /// What was wrong.
        what: &'static str,
    },
    /// A declared face pair's planes are DEFINITELY distinct — the
    /// declaration contradicts the geometry; refused loudly, never
    /// glued (M4 PR 5; `plane_eq` rung 2's verification direction).
    DeclarationContradicted {
        /// The contradicting predicate's diagnostics.
        diag: Indeterminate,
    },
    /// A declared face pair meets with OPPOSITE orientations at a
    /// shared edge — no valid closed solid merges such a pair; the
    /// declaration cannot be honored here.
    DeclaredOppositeOrientation {
        /// The pair's first face (arena order at the meeting edge).
        f1: FaceKey,
        /// The second face.
        f2: FaceKey,
    },
    /// After absorbing a group, the survivor's loops admit no unique
    /// positively-wound outline (zero or several positive windings) —
    /// the outer/ring roles cannot be assigned; refused rather than
    /// guessed (M5 S1 fix pass: the intra-face `kemr`'s provisional
    /// ring designation is now RESOLVED by winding, and shapes the
    /// resolution cannot decide are outside the inventory).
    MergedFaceRoleAmbiguous {
        /// The merged survivor face.
        face: FaceKey,
    },
    /// A plane-identity margin escalated while verifying a declared
    /// pair (in-band sliver) — typed, never guessed.
    Escalated {
        /// The predicate's diagnostics.
        diag: Indeterminate,
    },
    /// The run's tolerance cannot form a valid band (needed only when
    /// declared pairs are present).
    Band {
        /// The band construction failure.
        error: BandError,
    },
    /// The staged result's pcurve RE-MINT refused (M6-3: an input that
    /// carried stored caches re-mints them on the staged clone before
    /// commit — the `topo::pcurves` module docs' rule for ops that
    /// mutate minted bodies; the merged loops' one-branch walks are
    /// derived fresh, never stitched from the absorbed fragments'
    /// rows). The body is untouched, exactly as on every other
    /// variant.
    Pcurve {
        /// The mint pass's typed refusal.
        source: crate::pcurves::PcurveMintError,
    },
}

impl core::fmt::Display for MergeCoplanarError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InputNotClosed { errors } => {
                write!(
                    f,
                    "merge_coplanar_faces: input is not tier-2 ({} errors)",
                    errors.len()
                )
            }
            Self::ResultNotClosed { errors } => write!(
                f,
                "merge_coplanar_faces: merged result failed tier 2 ({} errors); refused",
                errors.len()
            ),
            Self::GroupNotClosed { errors } => write!(
                f,
                "merge_coplanar_faces: this group's staged merge failed tier 2 ({} errors); \
                 the group is left unmerged and the run continues",
                errors.len()
            ),
            Self::GroupKindSplit { planar, curved } => write!(
                f,
                "merge_coplanar_faces: group members {planar:?} (planar) and {curved:?} \
                 (not planar) are one group but not one surface kind — the run is neither \
                 planar nor curved; re-check the shared surface source that joined them"
            ),
            Self::UnsupportedConfiguration { edge } => write!(
                f,
                "merge_coplanar_faces: shared edge {edge:?} spans two loops of \
                 one face — unsupported configuration, refused"
            ),
            Self::PeriodClosure { edge } => write!(
                f,
                "merge_coplanar_faces: killing shared edge {edge:?} would close the \
                 curved cosurface run's full chart period — outside the merge's \
                 inventory: sub-period re-merges commit, full closures stay in their \
                 cut-carrying canonical form and are recorded as a loud skip"
            ),
            Self::Op { error } => write!(f, "merge_coplanar_faces: {error}"),
            Self::InvalidDeclaration { surface, what } => write!(
                f,
                "merge_coplanar_faces: invalid declared pair at surface {surface:?}: {what}"
            ),
            Self::DeclarationContradicted { diag } => write!(
                f,
                "merge_coplanar_faces: declared coincidence contradicts the geometry ({diag}) \
                 — fix the declaration or the geometry, the op never glues a lie"
            ),
            Self::DeclaredOppositeOrientation { f1, f2 } => write!(
                f,
                "merge_coplanar_faces: declared pair ({f1:?}, {f2:?}) meets with opposite \
                 orientations — unmergeable in a closed solid"
            ),
            Self::MergedFaceRoleAmbiguous { face } => write!(
                f,
                "merge_coplanar_faces: merged face {face:?} has no unique positively-wound \
                 outline among its loops — outer/ring roles cannot be assigned; refused"
            ),
            Self::Escalated { diag } => write!(
                f,
                "merge_coplanar_faces: plane-identity margin escalated verifying a declared \
                 pair ({diag})"
            ),
            Self::Band { error } => write!(f, "merge_coplanar_faces: {error}"),
            Self::Pcurve { source } => write!(
                f,
                "merge_coplanar_faces: the staged result's pcurve re-mint refused \
                 ({source}) — the body is untouched"
            ),
        }
    }
}

impl std::error::Error for MergeCoplanarError {}

impl From<EulerOpError> for MergeCoplanarError {
    fn from(error: EulerOpError) -> Self {
        Self::Op { error }
    }
}

/// The crate's dangling-reference vocabulary reaches this door
/// through the operator layer's own mapping, so a failed lookup here
/// is spelled once and named the same way it is everywhere else.
impl From<DanglingRef> for MergeCoplanarError {
    fn from(what: DanglingRef) -> Self {
        Self::Op {
            error: EulerOpError::from(what),
        }
    }
}

impl MergeCoplanarError {
    /// Whether this refusal reports a torn ARENA rather than a fact
    /// about the group's mergeability.
    ///
    /// An arena fault is a statement about the whole body, so it
    /// escapes [`GroupRegime`]'s split and refuses the call under
    /// both regimes: recording one as a skip would return `Ok` from a
    /// door that has just observed a kernel bug. Every other variant
    /// — the inventory refusals — is the regime's to place.
    ///
    /// The class is **not enumerated here**. Every arena fault this
    /// door can raise arrives from the operator layer, so membership
    /// is asked of the operator layer
    /// ([`EulerOpError::reports_tier1_corruption`], whose exhaustive
    /// match is what stops the two lists drifting apart). A second
    /// copy of the list in this file is exactly how the door came to
    /// promise a rule it kept for two variants out of nine.
    fn is_arena_fault(&self) -> bool {
        match self {
            Self::Op { error } => error.reports_tier1_corruption(),
            _ => false,
        }
    }
}

/// A non-empty declared-pair context: the surface equivalence plus
/// the band its verification decisions run in.
struct DeclaredCtx {
    eq: DeclaredSurfaceEq,
    band: Band,
}

/// The declared surface-key equivalence (M4 PR 5): union-find classes
/// over the declared face pairs' surface keys. Fragments of a face
/// inherit its surface key (`FaceSurface::Inherit`), so surface-level
/// equivalence covers every fragment of a declared pair without
/// key-chasing.
#[derive(Debug, Default)]
struct DeclaredSurfaceEq {
    parent: BTreeMap<SurfaceKey, SurfaceKey>,
}

impl DeclaredSurfaceEq {
    fn find(&self, mut k: SurfaceKey) -> SurfaceKey {
        while let Some(&p) = self.parent.get(&k) {
            if p == k {
                break;
            }
            k = p;
        }
        k
    }

    fn union(&mut self, a: SurfaceKey, b: SurfaceKey) {
        let (ra, rb) = (self.find(a), self.find(b));
        self.parent.entry(ra).or_insert(ra);
        self.parent.entry(rb).or_insert(rb);
        if ra != rb {
            self.parent.insert(rb, ra);
        }
    }

    fn same(&self, a: SurfaceKey, b: SurfaceKey) -> bool {
        if self.parent.is_empty() {
            return false;
        }
        self.find(a) == self.find(b)
    }

    fn is_empty(&self) -> bool {
        self.parent.is_empty()
    }
}

impl<T: Decide> Body<T> {
    /// Merges every maximal run of adjacent same-plane faces (module
    /// docs: structural or declared coincidence only), killing shared
    /// edges (`kef`; intra-face duplicates via `kemr`, whose new ring
    /// takes the plus half's side — a **provisional designation, not
    /// truth**: which loop is "the" ring is a containment question this
    /// op does not ask, so a region-sensitive consumer must re-home the
    /// ring via containment — PR 2+ machinery, `ring_move` the
    /// mechanism. Until then the convention is simply not detected
    /// wrong) and re-homing absorbed faces' rings onto the survivor.
    ///
    /// **The straight-seam repair (`kev`).** An intra-face duplicate is
    /// not always a ring. When the group's shared boundary is exactly
    /// two edges meeting at a valence-2 vertex whose two departures are
    /// collinear and OPPOSED — the vertex is interior to one straight
    /// carrier — the surviving duplicate is a dangling STRUT, and the
    /// op kills it with `kev` instead of minting a ring with `kemr`.
    /// The surgery DELETES BOTH seam edges and the junction vertex: the
    /// `kef` takes one, the `kev` takes the other along with the vertex
    /// it dangles from. Nothing is re-described, because the removed
    /// vertex was interior to a straight locus and the union of the two
    /// collinear pieces is that same locus. The motivating instance is
    /// a full revolve's axis-touching cap (the two seam edges are the
    /// halves of the disc's diameter, the vertex is the pole), but the
    /// licence is collinearity and not provenance
    /// ([`Body::redundant_subdivision_vertex`]'s docs carry the
    /// argument and its residue).
    ///
    /// **Atomic and deterministic (D9)**: the op stages on a clone —
    /// on any refusal `self` is untouched; on success the staged body
    /// replaces `self` wholesale. All scans are arena-order; the
    /// surviving face of each group is its first face in face-arena
    /// order; edges die in edge-arena order. Composite Euler delta per
    /// group: `f −(n−1)`, `e −k`, plus `r +m` for intra-face `kemr`
    /// kills, and `v −1` for each straight-seam `kev` (which is what
    /// keeps χ conserved when a ring is NOT minted: `kemr` trades an
    /// edge for a ring, `kev` trades an edge for a vertex). Each step
    /// is an Euler operator, so tier 1 holds throughout and χ is
    /// conserved at every step.
    ///
    /// A body with nothing to merge returns `Ok` with an empty outcome
    /// and is untouched (deterministic no-op).
    ///
    /// # Errors
    ///
    /// [`MergeCoplanarError`], the body untouched in every case.
    pub fn merge_coplanar_faces(
        &mut self,
        tol: Tol,
    ) -> Result<MergeCoplanarOutcome, MergeCoplanarError>
    where
        T: geom_brep::PcurveFittedLane,
    {
        self.merge_coplanar_faces_declared(&[], tol)
    }

    /// [`Body::merge_coplanar_faces`] with declared coincident
    /// SURFACE pairs (M4 PR 5, F5): each pair's surfaces are declared
    /// to describe one plane by recipe intent — they become
    /// equivalent for the adjacency test (fragments inherit surface
    /// keys, so every fragment of a declared face is covered),
    /// verified at each meeting edge through `plane_eq`'s declared
    /// rung (contradiction refuses typed). Same-source surfaces (N6)
    /// glue with zero declarations — the retired bit rung's
    /// replacement.
    ///
    /// A declared pair whose surfaces never meet at an edge licenses
    /// nothing and is a no-op (the equivalence is consulted only
    /// across shared edges); a pair whose keys do not resolve is a
    /// typed refusal.
    ///
    /// # Two failure regimes, one refusal vocabulary
    ///
    /// A group's **inventory** refusal — *this group cannot be
    /// merged* — either refuses the call or is recorded as a
    /// [`SkippedMerge`] while the remaining groups commit, and which
    /// of the two is a property of the GROUP: declared-licensed and
    /// curved runs record, structural planar runs refuse. Both raise
    /// the same [`MergeCoplanarError`] and a recorded one is carried
    /// whole in [`SkippedMerge::reason`], so the diagnosis a caller
    /// can make does not depend on which side of the boundary a group
    /// fell.
    ///
    /// **A refusal that reports a torn ARENA is not an inventory
    /// refusal and never becomes a record**: it says nothing about
    /// the group, so it refuses the call under both regimes rather
    /// than returning `Ok` from a door that has just observed a
    /// kernel bug. The class is the operator layer's
    /// ([`EulerOpError::reports_tier1_corruption`]) — a dangling
    /// reference and the walks and bijections that cannot fail on a
    /// tier-1-valid body — not a list kept here.
    ///
    /// [`MergeCoplanarError::GroupKindSplit`] also refuses under both
    /// regimes, but for a different reason and with a cost worth
    /// stating: it is raised while the regime is being COMPUTED, so
    /// there is no regime yet to record it under. **A
    /// declared-licensed group that straddles two surface kinds
    /// therefore loses the recording semantics its declaration bought
    /// it** — the call refuses where a kind-uniform licensed group
    /// would have carried on. That is the honest outcome of having no
    /// contract to give such a group, not a decision to refuse
    /// licensed work.
    ///
    /// The recording side is bounded the same way the refusing side
    /// is: each such group is staged on its own clone behind its own
    /// tier-2 gate, so a recorded skip leaves the run exactly as it
    /// was — never a partial commit, and the unglued coplanar
    /// adjacency persists as the operands already carried it (and
    /// refuses loudly downstream if reused undeclared).
    ///
    /// # Errors
    ///
    /// [`MergeCoplanarError`], the body untouched in every case.
    pub fn merge_coplanar_faces_declared(
        &mut self,
        declared: &[(SurfaceKey, SurfaceKey)],
        tol: Tol,
    ) -> Result<MergeCoplanarOutcome, MergeCoplanarError>
    where
        T: geom_brep::PcurveFittedLane,
    {
        // ---- Gate: tier-valid before. ----
        if let Err(errors) = validate_closed(self) {
            return Err(MergeCoplanarError::InputNotClosed { errors });
        }
        // ---- Declared pairs: validate, then class the surfaces. ----
        let planar = |body: &Self, k: SurfaceKey| -> Result<(), MergeCoplanarError> {
            match body.get_surface(k) {
                Some(Surface::Plane { .. }) => Ok(()),
                Some(_) => Err(MergeCoplanarError::InvalidDeclaration {
                    surface: k,
                    what: "declared surface is not a plane",
                }),
                None => Err(MergeCoplanarError::InvalidDeclaration {
                    surface: k,
                    what: "declared surface key does not resolve",
                }),
            }
        };
        let mut eq = DeclaredSurfaceEq::default();
        for &(k1, k2) in declared {
            planar(self, k1)?;
            planar(self, k2)?;
            eq.union(k1, k2);
        }
        let declared_ctx = if eq.is_empty() {
            None
        } else {
            Some(DeclaredCtx {
                eq,
                band: Band::linear(tol).map_err(|error| MergeCoplanarError::Band { error })?,
            })
        };
        // ---- Mergeable adjacency (read-only, edge-arena order). ----
        let mut neighbors: SecondaryMap<FaceKey, Vec<FaceKey>> = SecondaryMap::new();
        let mut declared_faces: std::collections::BTreeSet<FaceKey> =
            std::collections::BTreeSet::new();
        let mut any = false;
        for (edge_key, edge) in self.edges() {
            let (hp, hm) = self.edge_halves(edge.he_plus, edge.he_minus)?;
            let (fp, fm) = (hp.face, hm.face);
            if fp != fm
                && let Some(rung) =
                    self.planes_declared_equal(fp, fm, edge_key, declared_ctx.as_ref())?
            {
                if let Some(entry) = neighbors.entry(fp) {
                    entry.or_default().push(fm);
                }
                if let Some(entry) = neighbors.entry(fm) {
                    entry.or_default().push(fp);
                }
                if rung == MergeRung::DeclaredPair {
                    declared_faces.insert(fp);
                    declared_faces.insert(fm);
                }
                any = true;
            }
        }
        if !any {
            return Ok(MergeCoplanarOutcome::default());
        }
        // ---- Group labeling (face-arena order seeds, DFS worklist). ----
        //
        // A group is (seed, rest) rather than one list: the seed is
        // the survivor and every later step needs it, so carrying it
        // separately is what stops an empty group from being
        // spellable at all.
        let mut label: SecondaryMap<FaceKey, usize> = SecondaryMap::new();
        let mut groups: Vec<(FaceKey, Vec<FaceKey>)> = Vec::new();
        for (face_key, _) in self.faces() {
            if !neighbors.contains_key(face_key) || label.contains_key(face_key) {
                continue;
            }
            let id = groups.len();
            let mut rest = Vec::new();
            label.insert(face_key, id);
            let mut pending = vec![face_key];
            while let Some(next) = pending.pop() {
                for &n in neighbors.get(next).map(Vec::as_slice).unwrap_or(&[]) {
                    if !label.contains_key(n) {
                        label.insert(n, id);
                        rest.push(n);
                        pending.push(n);
                    }
                }
            }
            groups.push((face_key, rest));
        }
        // ---- Staged surgery on a clone. ----
        //
        // One refusal vocabulary; [`GroupRegime`] places the INVENTORY
        // refusals and an arena fault escapes it under both regimes.
        // The recording arm additionally clones a trial and runs its
        // own tier-2 gate, which is the price of letting the rest of
        // the run commit.
        let mut work = self.clone();
        let mut outcome = MergeCoplanarOutcome::default();
        for (rep, rest) in groups {
            match work.group_regime(rep, &rest, &declared_faces)? {
                GroupRegime::RefusesTheCall => {
                    outcome.groups.push(work.merge_group(rep, &rest, tol)?);
                }
                GroupRegime::RecordsASkip => {
                    let mut trial = work.clone();
                    // The sub-stage's own tier-2 gate: the group is
                    // adopted only if its trial validates, so a
                    // recorded skip leaves `work` exactly as it was.
                    let staged =
                        trial
                            .merge_group(rep, &rest, tol)
                            .and_then(|group| match validate_closed(&trial) {
                                Ok(()) => Ok(group),
                                Err(errors) => Err(MergeCoplanarError::GroupNotClosed { errors }),
                            });
                    match staged {
                        Ok(group) => {
                            work = trial;
                            outcome.groups.push(group);
                        }
                        Err(reason) => {
                            // An arena fault is not this group's
                            // failure to be recorded; it reports the
                            // body, so it refuses here too.
                            if !GroupRegime::RecordsASkip.records(&reason) {
                                return Err(reason);
                            }
                            outcome.skipped.push(SkippedMerge {
                                faces: core::iter::once(rep).chain(rest).collect(),
                                reason,
                            });
                        }
                    }
                }
            }
        }
        // ---- Gate: tier-valid after; commit. ----
        if let Err(errors) = validate_closed(&work) {
            return Err(MergeCoplanarError::ResultNotClosed { errors });
        }
        // A body that carried stored pcurve caches RE-MINTS them on
        // the staged result before commit (the `topo::pcurves` module
        // docs' rule for ops that mutate minted bodies): the merge
        // rebuilds face loops, and two absorbed fragments' walks were
        // branch-anchored independently — the merged loop's one-branch
        // walk must be derived fresh, never stitched from the
        // fragments' rows. Still on the staged clone, so a mint
        // refusal keeps the untouched-on-error contract.
        //
        // LATENT (named, not reachable by any current path): the mint
        // pass carries the `PcurveFittedLane` bound since PCURVE P-2
        // (#498) and mints U2's `General` arm through it, but the
        // FITTED variant itself still has no mint site, so a `Fitted`
        // cache (at rest since M6-2) on a merged body would still come
        // back as the mint pass's honest-skip — the face legally
        // UNCACHED, its fitted certificate silently dropped. What is
        // left of that item is `certify_fitted`'s own wiring, not the
        // bound; this site inherits the fix when that lands.
        if !self.pcurves.is_empty() {
            crate::pcurves::mint_pcurves(&mut work, tol)
                .map_err(|source| MergeCoplanarError::Pcurve { source })?;
        }
        *self = work;
        Ok(outcome)
    }

    /// Whether the group's face is planar, announcing both lookups.
    ///
    /// # Errors
    ///
    /// [`MergeCoplanarError::Op`], through the crate's dangling-
    /// reference vocabulary.
    fn face_is_planar(&self, face: FaceKey) -> Result<bool, MergeCoplanarError> {
        let surface = self
            .get_face(face)
            .ok_or(DanglingRef::Entity(EntityId::Face(face)))?
            .surface;
        let described = self
            .get_surface(surface)
            .ok_or(DanglingRef::Geometry(GeomRef::Surface(surface)))?;
        Ok(matches!(described, Surface::Plane { .. }))
    }

    /// **Does `toward` dangle alone at its start vertex** — the
    /// condition that licenses `kev` over `kemr` on a straight seam's
    /// surviving duplicate?
    ///
    /// A BROKEN orbit is ANNOUNCED, not read as "no tip". `kev` and
    /// `kemr` are different operators with different Euler deltas, so
    /// a torn arena answering this question silently chooses which
    /// surgery runs and which delta the group reports.
    ///
    /// # Errors
    ///
    /// [`EulerOpError::OrbitBroken`], naming the half-edge whose
    /// start vertex's orbit failed to close.
    fn strut_tip(&self, toward: crate::entity::HalfEdgeKey) -> Result<bool, EulerOpError> {
        let orbit = self
            .vertex_orbit(toward)
            .ok_or(EulerOpError::OrbitBroken { he: toward })?;
        Ok(orbit.len() == 1)
    }

    /// Which failure regime one group's INVENTORY refusals run under.
    ///
    /// A group records a skip when its adjacency was licensed by a
    /// declared pair (any member), or when it is curved — the two
    /// cases whose refusals are statements about the merge's
    /// inventory rather than about the body, and whose unglued
    /// adjacency is a legal output the operands already carried.
    /// Everything else — a structural planar run — refuses the call.
    ///
    /// **The kind question is asked of EVERY member.** The hard rungs
    /// glue on surface-key or surface-SOURCE identity, and neither
    /// tests the surface's kind, so a group can straddle planar and
    /// curved faces; answering off one member would let arena order
    /// decide which contract the group is handed. A straddling group
    /// refuses ([`MergeCoplanarError::GroupKindSplit`]).
    ///
    /// # Errors
    ///
    /// [`MergeCoplanarError::GroupKindSplit`] where the members
    /// disagree, or [`MergeCoplanarError::Op`] carrying an unresolved
    /// reference. The kind lookups are ANNOUNCED rather than read as
    /// "not curved": they decide which contract the group is handed,
    /// and a failed lookup silently spelled "planar" would move a
    /// group between regimes on a torn arena.
    fn group_regime(
        &self,
        rep: FaceKey,
        rest: &[FaceKey],
        declared_faces: &std::collections::BTreeSet<FaceKey>,
    ) -> Result<GroupRegime, MergeCoplanarError> {
        let rep_planar = self.face_is_planar(rep)?;
        for &f in rest {
            if self.face_is_planar(f)? != rep_planar {
                let (planar, curved) = if rep_planar { (rep, f) } else { (f, rep) };
                return Err(MergeCoplanarError::GroupKindSplit { planar, curved });
            }
        }
        let licensed =
            declared_faces.contains(&rep) || rest.iter().any(|f| declared_faces.contains(f));
        Ok(if licensed || !rep_planar {
            GroupRegime::RecordsASkip
        } else {
            GroupRegime::RefusesTheCall
        })
    }

    /// One resolved half of an edge — every fact the merge's scans
    /// read through a half-edge, taken in the one walk that proves
    /// the keys, so no later step looks a proven key up again.
    fn half_edge_facts(
        &self,
        he: crate::entity::HalfEdgeKey,
    ) -> Result<HalfEdgeFacts, EulerOpError> {
        let hd = self.get_half_edge(he).ok_or(EulerOpError::StaleKey {
            key: EntityId::HalfEdge(he),
        })?;
        let parent = hd.parent_loop;
        let face = self
            .get_loop(parent)
            .ok_or(EulerOpError::StaleKey {
                key: EntityId::Loop(parent),
            })?
            .face;
        Ok(HalfEdgeFacts {
            r#loop: parent,
            face,
            start: hd.start,
        })
    }

    /// An edge's two halves, resolved.
    ///
    /// Every link this walks is one tier 1 requires to resolve, so a
    /// refusal names a torn arena and never an ordinary shape. It is
    /// returned rather than folded into "this edge is not interesting"
    /// because the two are indistinguishable to the scans that call
    /// this, and treating a torn link as an uninteresting edge drops a
    /// mergeable adjacency or a shared seam without a word.
    ///
    /// It carries the parent loop and start vertex beside the face
    /// because the scans need those too: returning them from the walk
    /// that proved the keys is what leaves the later steps with
    /// nothing to look up and therefore nothing to discard.
    ///
    /// # Errors
    ///
    /// [`EulerOpError::StaleKey`], naming the link that did not
    /// resolve.
    fn edge_halves(
        &self,
        he_plus: crate::entity::HalfEdgeKey,
        he_minus: crate::entity::HalfEdgeKey,
    ) -> Result<(HalfEdgeFacts, HalfEdgeFacts), EulerOpError> {
        Ok((
            self.half_edge_facts(he_plus)?,
            self.half_edge_facts(he_minus)?,
        ))
    }

    /// The F6 ladder's merge test (M4 PR 5, the N6 retirement): same
    /// surface key (structural), same [`crate::GeomSource`] including
    /// orient (declared — shared recipe source, syntactic identity,
    /// zero numerics), or the pair's surfaces are declared-equivalent
    /// by this call's face pairs (verified through `plane_eq`'s
    /// declared rung at the meeting edge; contradiction refuses).
    ///
    /// The M3-era rung — bit-identical nine-scalar descriptions — is
    /// RETIRED from production: equal bits without shared source stay
    /// unglued (the ladder's ratified rung (b)). The bit comparison
    /// survives as the debug assertion that same-source records agree
    /// with the bits. *No banded comparison certifies coincidence
    /// here by design* — the declared-pair verification only checks
    /// the declaration is not a lie; the INTENT does the gluing.
    ///
    /// Non-plane surfaces never merge, same-key included (curved
    /// maximality is M5's).
    ///
    /// **Shared sense is a precondition of every rung** (S10). Two
    /// faces on one surface whose `sense` bits differ have OPPOSITE
    /// outward normals: they are the two sides of a slit, not one
    /// region cut in two, and gluing them would mint a face that is
    /// its own reverse. The hard rungs therefore stop firing on such a
    /// pair — they answer "same SURFACE", which is no longer the same
    /// question as "same FACE geometry". They fall through to the
    /// declared rung, where the verified `oriented_plane_eq` verdict
    /// on the two OUTWARD normals is `SameOpposite` and the existing
    /// [`MergeCoplanarError::DeclaredOppositeOrientation`] refusal
    /// fires — a declaration that such a pair is mergeable is exactly
    /// the lie that variant was minted to refuse. An UNDECLARED
    /// opposite-sense pair is not refused, it is simply not a merge
    /// candidate: a slit is legal geometry, and this op has no
    /// standing to reject a body for containing one.
    fn planes_declared_equal(
        &self,
        f1: FaceKey,
        f2: FaceKey,
        edge: EdgeKey,
        declared: Option<&DeclaredCtx>,
    ) -> Result<Option<MergeRung>, MergeCoplanarError> {
        let (Some((k1, sense1, sign1)), Some((k2, sense2, sign2))) = (
            self.get_face(f1)
                .map(|f| (f.surface, f.sense, f.sense_sign::<T>())),
            self.get_face(f2)
                .map(|f| (f.surface, f.sense, f.sense_sign::<T>())),
        ) else {
            return Ok(None);
        };
        let (Some(s1), Some(s2)) = (self.get_surface(k1), self.get_surface(k2)) else {
            return Ok(None);
        };
        // The shared-sense precondition (fn docs): a differing bit
        // makes the two outward normals opposite, so neither hard rung
        // — both of which certify the SURFACE, not the face — may
        // conclude the faces are one region. Falling through leaves
        // the declared rung to refuse loudly if the pair was declared.
        let same_sense = sense1 == sense2;
        // The hard rungs are KIND-AGNOSTIC since M5 PR 9 (C12.5, the
        // cosurface generalization): the same-key and same-source
        // tests never touch a numeric coordinate, so nothing about
        // them was planar — the M3-era "curved same-key neighbors
        // stay unmerged" note flips here, with the same ladder, the
        // same never-numeric rule, and N3 naming semantics unchanged.
        // The named consumer: the boolean zip's re-merge of a
        // cylinder wall split by a through cut.
        if k1 == k2 && same_sense {
            return Ok(Some(MergeRung::Hard)); // structural
        }
        // Declared rung, N6 form: same recipe source INCLUDING orient
        // — a provenance lookup, no numerics (M4's GeomSource
        // retirement consumed, NOT bit_identity). The debug assertion
        // is DESIGN.md's "records agree with bits", stated for the
        // planar kind where the bit predicate exists.
        if same_sense
            && let (Some(g1), Some(g2)) = (self.surface_source(k1), self.surface_source(k2))
            && g1 == g2
        {
            #[cfg(debug_assertions)]
            if let (
                Surface::Plane {
                    origin: o1,
                    normal: n1,
                    u_ref: u1,
                },
                Surface::Plane {
                    origin: o2,
                    normal: n2,
                    u_ref: u2,
                },
            ) = (s1.clone(), s2.clone())
            {
                debug_assert!(
                    crate::source::plane_bits_agree(o1, n1, o2, n2, false)
                        && crate::source::vec3_bits_agree(u1, u2),
                    "same-source theorem violated: same-source surface descriptions disagree \
                     bitwise (kernel bug: a source survived a geometric rewrite)"
                );
            }
            return Ok(Some(MergeRung::Hard));
        }
        // The declared-PAIR rung stays planar (its verification is
        // `oriented_plane_eq`; the curved-pair verification predicate
        // is the contact census's — CONTACT-DESIGN C2/C4 — not minted
        // here).
        let (
            Surface::Plane {
                origin: o1,
                normal: n1,
                ..
            },
            Surface::Plane {
                origin: o2,
                normal: n2,
                ..
            },
        ) = (s1.clone(), s2.clone())
        else {
            return Ok(None);
        };
        // Declared face pairs (this call's recipe intent), verified.
        if let Some(ctx) = declared
            && ctx.eq.same(k1, k2)
        {
            let band = ctx.band;
            let arm = self.edge_chord_len(edge).unwrap_or_else(T::one);
            let id = PlaneIdentity {
                s1: None,
                s2: None,
                declared: true,
            };
            // Outward normals, not chart normals (S10): `PlaneDesc`'s
            // contract, and the reason the SameOpposite arm below can
            // stand as the shared-sense refusal — an opposite-sense
            // pair on one plane lands there by construction.
            let p1 = PlaneDesc {
                origin: o1,
                normal: n1 * sign1,
            };
            let p2 = PlaneDesc {
                origin: o2,
                normal: n2 * sign2,
            };
            return match oriented_plane_eq(&p1, &p2, id, arm, band) {
                Ok(PlaneRelation::SameOriented) => Ok(Some(MergeRung::DeclaredPair)),
                Ok(PlaneRelation::SameOpposite) => {
                    Err(MergeCoplanarError::DeclaredOppositeOrientation { f1, f2 })
                }
                // Unreachable through the declared rung; kept typed.
                Ok(PlaneRelation::Distinct) => Ok(None),
                Err(PlaneEqError::Contradicted(diag)) => {
                    Err(MergeCoplanarError::DeclarationContradicted { diag })
                }
                Err(PlaneEqError::Escalated(diag) | PlaneEqError::Undeclared { diag, .. }) => {
                    Err(MergeCoplanarError::Escalated { diag })
                }
            };
        }
        Ok(None)
    }

    /// The chord length between an edge's endpoints — the lever arm
    /// metering the declared-pair verification at that edge.
    fn edge_chord_len(&self, edge: EdgeKey) -> Option<T> {
        let e = self.get_edge(edge)?;
        let pa = *self.get_point(self.get_vertex(self.get_half_edge(e.he_plus)?.start)?.point)?;
        let pb = *self.get_point(
            self.get_vertex(self.get_half_edge(e.he_minus)?.start)?
                .point,
        )?;
        Some((pb - pa).norm())
    }

    /// **Is `v` a redundant subdivision vertex of a straight seam?**
    ///
    /// This is the geometric licence for removing a seam vertex, and it
    /// is deliberately NOT a claim about provenance. A vertex of
    /// valence 2 whose two edges lie on ONE straight carrier, leaving
    /// it in OPPOSITE directions, is interior to a single line
    /// segment: deleting it and merging its two edges replaces two
    /// collinear pieces with their union and **no locus changes**. The
    /// repair is then geometry-preserving by construction, whatever
    /// produced the vertex.
    ///
    /// A full revolve's axis-touching cap is the motivating instance —
    /// its two meridians are the two halves of the disc's DIAMETER,
    /// with the pole interior to it — but nothing here mentions poles
    /// or axes, and it should not: the same fact licenses the same
    /// removal on any straight seam.
    ///
    /// What it refuses is the case the F7 rule exists for. Two
    /// coplanar faces meeting along a bent seam — `merge_skip`'s
    /// L-corner, where two overlapping rectangles meet at a
    /// re-entrant corner — have a valence-2 junction too, and an
    /// earlier form of this trigger that tested only valence was
    /// falsified by exactly that fixture. Perpendicular departures
    /// fail the collinearity decision, so the corner survives and the
    /// merge still refuses, which is the pinned behaviour.
    ///
    /// Both decisions are metered on the shorter incident segment (the
    /// honest lever for an angular quantity read as a length).
    fn redundant_subdivision_vertex(
        &self,
        v: VertexKey,
        band: Band,
    ) -> Result<bool, MergeCoplanarError> {
        let Some(em) = self.get_vertex(v).and_then(|vd| vd.emanating) else {
            return Ok(false);
        };
        // PAIRED, deliberately unfixed here: this reads a broken
        // orbit as "not a redundant vertex", where `strut_tip` — the
        // other consumer of the identical condition, forty lines down
        // the same surgery — announces it. The two answers are the
        // open row's subject and moving one without the other would
        // hide the pair rather than settle it.
        let Some(orbit) = self.vertex_orbit(em) else {
            return Ok(false);
        };
        if orbit.len() != 2 {
            return Ok(false);
        }
        let point = |vk: VertexKey| {
            self.get_vertex(vk)
                .and_then(|vd| self.get_point(vd.point).copied())
        };
        let Some(pv) = point(v) else {
            return Ok(false);
        };
        // Each orbit member starts at `v`; its mate starts at the far
        // end. Both carriers must be straight — an arc through `v` is
        // not a subdivision of anything.
        let mut departures = Vec::with_capacity(2);
        for &he in &orbit {
            let Some(hd) = self.get_half_edge(he) else {
                return Ok(false);
            };
            let Some(e) = self.get_edge(hd.edge) else {
                return Ok(false);
            };
            let straight = self
                .get_curve_geom(e.curve)
                .and_then(crate::null::CurveGeom::certified)
                .is_some_and(|c| matches!(c.carrier(), geom::Curve3::Line { .. }));
            if !straight {
                return Ok(false);
            }
            let far = if e.he_plus == he {
                e.he_minus
            } else {
                e.he_plus
            };
            let Some(pf) = self.get_half_edge(far).and_then(|h| point(h.start)) else {
                return Ok(false);
            };
            departures.push(pf - pv);
        }
        let (d1, d2) = (departures[0], departures[1]);
        // The lever is the SHORTER incident segment — an angular
        // quantity is read here as a length, and the shorter arm is the
        // conservative one. `Real::min`, not `<`: the scalar backends
        // order intervals, not values, so a bare comparison on `T` is
        // not available and would not mean this if it were (the S10
        // exact-bit discipline). A degenerate zero-length edge makes
        // the normalization poison, which `decide` escalates typed
        // rather than silently answering.
        let (n1, n2) = (d1.norm(), d2.norm());
        let arm = n1.min(n2);
        let (u1, u2) = (d1 / n1, d2 / n2);
        let escalate = |diag| MergeCoplanarError::Escalated { diag };
        // Collinear: the two departures span no angle.
        if crate::validate::decide(
            "merge_seam_collinear",
            Margin::levered(u1.cross(u2).norm(), arm),
            band,
        )
        .map_err(escalate)?
            != geom_core::Sign::Zero
        {
            return Ok(false);
        }
        // ...and OPPOSED, so `v` is interior to the union rather than a
        // point the seam doubles back from.
        Ok(
            crate::validate::decide("merge_seam_opposed", Margin::levered(u1.dot(u2), arm), band)
                .map_err(escalate)?
                == geom_core::Sign::Negative,
        )
    }

    /// Merges one group into `rep`, its arena-first member (see the
    /// public op's docs for order and refusals). Runs on the staged
    /// clone.
    ///
    /// The survivor is a parameter rather than `members[0]` so that a
    /// group with no survivor cannot be spelled: every caller has the
    /// seed in hand, and a bare index here would be a panic path for
    /// a state the labeling never produces.
    fn merge_group(
        &mut self,
        rep: FaceKey,
        rest: &[FaceKey],
        tol: Tol,
    ) -> Result<MergedGroup, MergeCoplanarError> {
        let mut group = MergedGroup {
            kept: rep,
            absorbed: Vec::new(),
            killed_edges: Vec::new(),
            rings_made: Vec::new(),
            killed_vertices: Vec::new(),
        };
        let in_group = |f: FaceKey| f == rep || rest.contains(&f);
        // **The straight-seam junction, decided ONCE** on the group as
        // it arrives — before any mutation, because the answer licenses
        // a different repair below and must not be re-derived from a
        // body that repair is halfway through changing.
        //
        // Verified against this crate's whole merge/boolean fixture
        // corpus before it was wired to anything (the method the
        // reviewers' falsifications earned): it fires on a collinear
        // subdivided seam and on nothing else in the corpus — not on
        // `merge_skip`'s L-corner, not on either review arm's bent
        // chords, not on an inset ring.
        let straight_seam = {
            let band = Band::linear(tol).map_err(|error| MergeCoplanarError::Band { error })?;
            // The scan keeps the start vertices it already walked, so
            // the junction test below looks nothing up.
            let mut shared: Vec<[VertexKey; 2]> = Vec::new();
            for (_, e) in self.edges() {
                let (hp, hm) = self.edge_halves(e.he_plus, e.he_minus)?;
                if hp.face != hm.face && in_group(hp.face) && in_group(hm.face) {
                    shared.push([hp.start, hm.start]);
                }
            }
            let mut verdict = false;
            if let [a, b] = shared[..] {
                for v in a.iter().filter(|v| b.contains(v)) {
                    if self.redundant_subdivision_vertex(*v, band)? {
                        verdict = true;
                    }
                }
            }
            verdict
        };
        // Absorption: repeatedly kill the first (edge-arena order)
        // edge shared between rep and another group member.
        loop {
            let mut found = None;
            for (edge_key, edge) in self.edges() {
                let (hp, hm) = self.edge_halves(edge.he_plus, edge.he_minus)?;
                if hp.face == rep && hm.face != rep && in_group(hm.face) {
                    found = Some((edge_key, edge.he_minus, hm.face));
                    break;
                }
                if hm.face == rep && hp.face != rep && in_group(hp.face) {
                    found = Some((edge_key, edge.he_plus, hp.face));
                    break;
                }
            }
            let Some((edge_key, dying_he, other)) = found else {
                break;
            };
            // Re-home the dying face's rings onto the survivor, then
            // kill the shared edge and the face together (kef).
            //
            // The lookup is ANNOUNCED rather than defaulted to an
            // empty ring list, because "this face has no rings" and
            // "this face is gone" are different answers and the
            // default gave them one spelling. It is a typed refusal
            // and not an `unreachable!`: `other` arrives from a
            // loop's back-pointer and no check in this call proves it
            // live. `kef` below re-derives the same key and refuses
            // on it too, so the ANSWER here was never reachable — the
            // announcement is what makes the two agree at the site
            // that reads the rings.
            let dying = self
                .get_face(other)
                .ok_or(DanglingRef::Entity(EntityId::Face(other)))?;
            for ring in dying.rings.clone() {
                self.ring_move(ring, rep)?;
            }
            self.kef(dying_he)?;
            group.absorbed.push(other);
            group.killed_edges.push(edge_key);
        }
        // Intra-face duplicates: edges now occurring twice within the
        // survivor's loops. On a PLANAR survivor a same-loop duplicate
        // bounds a genuine hole and `kemr` mints the ring. On a CURVED
        // survivor (C12.5, M5 PR 9) a same-face duplicate means the
        // cosurface run CLOSED THE FULL PERIOD — a shape outside the
        // merge's inventory at M5 (neither the ring form nor the
        // kept-cut seam form is integrable by the exact-B-rep props
        // yet), refused typed here; the driver records it as a LOUD
        // skip for curved structural runs, so sub-period re-merges
        // (the C12.5 through-cut case) proceed and full closures stay
        // unmerged exactly as the operands arrived.
        let survivor_curved = !self.face_is_planar(rep)?;
        loop {
            let mut found = None;
            for (edge_key, edge) in self.edges() {
                let (hp, hm) = self.edge_halves(edge.he_plus, edge.he_minus)?;
                if hp.face != rep || hm.face != rep {
                    continue;
                }
                found = Some((edge_key, (edge.he_plus, hp), (edge.he_minus, hm)));
                break;
            }
            let Some((edge_key, (he_plus, hp), (he_minus, hm))) = found else {
                break;
            };
            let same_loop = hp.r#loop == hm.r#loop;
            if survivor_curved {
                return Err(MergeCoplanarError::PeriodClosure { edge: edge_key });
            }
            if !same_loop {
                return Err(MergeCoplanarError::UnsupportedConfiguration { edge: edge_key });
            }
            // **A straight seam's surviving duplicate is a STRUT, not a
            // ring.** The absorption above killed one of the two seam
            // edges with `kef`; the other is now a duplicate inside the
            // survivor whose junction end is left with valence 1 — a
            // dangling remnant the merge itself created, enclosing
            // nothing. `kemr` would mint a ring from it and the winding
            // pass would then find no unique positive cycle and refuse
            // `MergedFaceRoleAmbiguous`, which is the dead end this op
            // hits on every revolve cap.
            //
            // `kev` is the op for it — it kills the strut AND the far
            // vertex, leaving the face bounded by its outline alone.
            // The licence is that the removed vertex was interior to
            // one straight carrier, so the union of the two collinear
            // pieces is the same locus: geometry-preserving by
            // construction, which is why this is gated on
            // `straight_seam` and not on the strut's shape alone.
            //
            // A BROKEN orbit is announced, not read as "no tip":
            // `kev` and `kemr` are different operators with different
            // Euler deltas, so letting a torn arena answer this
            // question silently chooses which surgery runs.
            let mut tip = None;
            if straight_seam {
                // `vertex_orbit` walks the halves STARTING at its
                // argument's start vertex, so each candidate asks
                // about the far end of the other half; the start
                // vertex is the one `kev` takes with the strut.
                for (from_rim, toward, killed) in
                    [(he_plus, he_minus, hm.start), (he_minus, he_plus, hp.start)]
                {
                    if self.strut_tip(toward)? {
                        tip = Some((from_rim, killed));
                        break;
                    }
                }
            }
            if let Some((from_rim, killed)) = tip {
                self.kev(from_rim)?;
                group.killed_edges.push(edge_key);
                group.killed_vertices.push(killed);
                continue;
            }
            let result = self.kemr(he_plus, he_minus)?;
            group.killed_edges.push(edge_key);
            group.rings_made.push(result.ring);
        }
        // Role normalization (M5 S1 fix pass, review MAJOR-1): the
        // intra-face `kemr` above designates its ring PROVISIONALLY
        // (the plus half's side — the module docs' documented
        // containment question). A group absorbed across TWO disjoint
        // shared runs closes a genuine hole, and the provisional side
        // can put the OUTLINE in the ring slot and the hole in the
        // outer slot — every downstream volume gate is role-invariant,
        // but tessellation is not (the silent-corrupt-export class).
        // Resolve by winding: the outline is the unique cycle winding
        // POSITIVELY around the face's outward normal (the same
        // Newell-functional predicate the boolean join's ring lane
        // decides on); swap it into the outer slot if the kemr put it
        // elsewhere; no unique positive cycle refuses typed.
        if !group.rings_made.is_empty() {
            // The survivor is resolved HERE, where its liveness is
            // proven in this call: the curved-survivor gate above
            // resolved `rep`, and the absorption between only runs
            // `kemr`, which kills no face. The role pass then takes
            // resolved data and performs no lookup of its own.
            let Some(survivor) = self.get_face(rep) else {
                unreachable!(
                    "merge_group: `rep` resolved by the curved-survivor gate above and \
                     the absorption's `kemr` kills no face"
                )
            };
            let survivor = survivor.clone();
            if let Some(i) = self.merged_outline_ring(rep, &survivor, tol)? {
                let Some(fm) = self.faces.get_mut(rep) else {
                    unreachable!(
                        "merge_group: `rep` resolved a few lines above and the winding \
                         pass between is read-only"
                    )
                };
                fm.outer = survivor.rings[i];
                fm.rings[i] = survivor.outer;
            }
        }
        Ok(group)
    }

    /// The signed winding of a cycle loop around `normal`, through the
    /// reified `bool_ring_run_winding` predicate (the plane's Newell
    /// functional — twice the enclosed signed area; the same margin
    /// the boolean join's ring lane decides on). `None` for empty
    /// loops (a lone-vertex ring bounds no area and stays a ring).
    ///
    /// Dimension (audit F4): the Newell area is metered to a LENGTH by
    /// the loop's own perimeter — `2A/P`, the region's mean width. The
    /// derivation, and why this predicate must state it identically at
    /// all three of its sites, is in `boolean::join::ring_run_ccw`.
    ///
    /// Behaviour change riding with that metering (the unit's
    /// deviation 1, SECOND site — the join lane's zero-perimeter note
    /// has the same shape): a cycle whose perimeter is exactly zero —
    /// every vertex coincident — now divides `0/0`, poisons, and
    /// escalates typed, where it previously answered `Some(Zero)` and
    /// let `normalize_merged_roles` read it as "not the positively-wound
    /// cycle". Empty loops still return `None` earlier, so reaching this
    /// needs a real cycle of coincident points. The fail-loud direction
    /// is deliberate: a loop with no extent has no winding to report,
    /// and refusing typed beats handing back a role decision derived
    /// from an area and a perimeter that are both nothing.
    ///
    /// `normal` must be the face's OUTWARD normal (S10): the caller
    /// multiplies the chart normal by `sense_sign` exactly once, and
    /// the Newell sum here is left alone. That sum is built from the
    /// loop's STORED cycle order, which `revert` reverses in the same
    /// breath as it flips the sense bit, so it already changes sign on
    /// its own — threading the sense onto both factors would cancel
    /// and leave the outer/ring roles as wrong as threading neither.
    fn loop_winding(
        &self,
        l: LoopKey,
        normal: geom_core::Vec3<T>,
        band: Band,
    ) -> Result<Option<geom_core::Sign>, MergeCoplanarError> {
        let corrupt = || MergeCoplanarError::Op {
            error: EulerOpError::StaleKey {
                key: EntityId::Loop(l),
            },
        };
        let crate::entity::LoopBoundary::Cycle { first } =
            self.get_loop(l).ok_or_else(corrupt)?.boundary
        else {
            return Ok(None);
        };
        let cycle = self.loop_cycle(first).ok_or_else(corrupt)?;
        // Line-bounded cycles only (the tier-3 check-6 scope): an
        // arc's vertex chord is not the boundary, so a curved-bounded
        // loop's chord winding says nothing about the region — treat
        // it as undecidable (`None`; the caller then refuses rather
        // than guesses if roles hinge on it).
        let all_lines = cycle.iter().all(|&he| {
            self.get_half_edge(he)
                .and_then(|hd| self.get_edge(hd.edge))
                .and_then(|e| self.get_curve_geom(e.curve))
                .and_then(crate::null::CurveGeom::certified)
                .is_some_and(|c| matches!(c.carrier(), geom::Curve3::Line { .. }))
        });
        if !all_lines {
            return Ok(None);
        }
        let point_of = |he| -> Result<geom_core::Point3<T>, MergeCoplanarError> {
            let v = self.get_half_edge(he).ok_or_else(corrupt)?.start;
            self.get_vertex(v)
                .and_then(|vd| self.get_point(vd.point).copied())
                .ok_or_else(corrupt)
        };
        let p0 = point_of(cycle[0])?;
        let mut newell = geom_core::Vec3::new(T::zero(), T::zero(), T::zero());
        // The F4 metering lever: this cycle's perimeter, accumulated
        // with the area (line-bounded only, so chords ARE the boundary).
        let mut perimeter = T::zero();
        let mut prev = p0;
        for &he in &cycle[1..] {
            let p = point_of(he)?;
            newell = newell + (prev - p0).cross(p - p0);
            perimeter = perimeter + (p - prev).norm();
            prev = p;
        }
        perimeter = perimeter + (p0 - prev).norm();
        match crate::validate::decide(
            "bool_ring_run_winding",
            Margin::over_lever(normal.dot(newell), perimeter),
            band,
        ) {
            Ok(sign) => Ok(Some(sign)),
            Err(diag) => Err(MergeCoplanarError::Escalated { diag }),
        }
    }

    /// Which of the merged survivor's rings is its outline, decided by
    /// winding (doc at the call site): the unique positively-wound
    /// cycle is the outer loop. `Some(i)` means ring `i` must swap into
    /// the outer slot; `None` means the roles are already correct, or
    /// the survivor is not a planar rung. Zero or multiple positive
    /// cycles refuse [`MergeCoplanarError::MergedFaceRoleAmbiguous`].
    ///
    /// Takes the survivor's resolved data rather than looking it up:
    /// the caller holds the liveness proof, and a helper that cannot
    /// look anything up cannot discard a failed lookup. `face` is the
    /// refusal's payload only — nothing here dereferences it.
    fn merged_outline_ring(
        &self,
        face: FaceKey,
        survivor: &crate::entity::Face,
        tol: Tol,
    ) -> Result<Option<usize>, MergeCoplanarError> {
        let band = Band::linear(tol).map_err(|error| MergeCoplanarError::Band { error })?;
        // The face's OUTWARD normal (S10): "positively wound" means
        // CCW seen from OUTSIDE the material, so on a reversed face
        // the chart normal names the opposite convention and every
        // role assignment below would come out inverted.
        let normal = match self.get_surface(survivor.surface) {
            Some(Surface::Plane { normal, .. }) => *normal * survivor.sense_sign::<T>(),
            _ => return Ok(None), // merges only fire on planar rungs
        };
        let mut positives: Vec<Option<usize>> = Vec::new(); // None = outer
        if self.loop_winding(survivor.outer, normal, band)? == Some(geom_core::Sign::Positive) {
            positives.push(None);
        }
        for (i, &r) in survivor.rings.iter().enumerate() {
            if self.loop_winding(r, normal, band)? == Some(geom_core::Sign::Positive) {
                positives.push(Some(i));
            }
        }
        match positives[..] {
            [None] => Ok(None), // roles already correct
            [Some(i)] => Ok(Some(i)),
            _ => Err(MergeCoplanarError::MergedFaceRoleAmbiguous { face }),
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use crate::fixtures::ops_cube;

    /// A shared edge of the ops cube, as the pair of faces meeting
    /// there — addressed exactly as the absorption scan addresses it,
    /// through the loops' `face` back-pointers.
    fn adjacent_pair(body: &Body<f64>) -> (FaceKey, FaceKey) {
        body.edges()
            .find_map(|(_, e)| {
                let (hp, hm) = body.edge_halves(e.he_plus, e.he_minus).ok()?;
                (hp.face != hm.face).then_some((hp.face, hm.face))
            })
            .expect("a cube has adjacent faces")
    }

    /// **A dangling absorbed-face key is refused typed, naming the
    /// face** — the contract the absorption owes, pinned end to end.
    ///
    /// It does NOT isolate the ring lookup, and cannot: `kef`'s own
    /// plan phase re-derives the same key
    /// (`loops[half_edges[dying_he].parent_loop].face`) and refuses on
    /// it with the same value, before any mutation. The two sites are
    /// indistinguishable from outside by construction — which is why
    /// the earlier `unwrap_or_default()` there could never actually
    /// drop a ring. This row pins the answer; it is not evidence about
    /// which of the two produced it.
    #[test]
    fn a_dangling_absorbed_face_key_is_refused_typed_and_names_the_face() {
        let tol = Tol::witness();
        let mut body = ops_cube(tol).body;
        let (rep, other) = adjacent_pair(&body);
        body.faces
            .remove(other)
            .expect("the pair's second face is live before the tear");

        assert_eq!(
            body.merge_group(rep, &[other], tol),
            Err(MergeCoplanarError::Op {
                error: EulerOpError::StaleKey {
                    key: EntityId::Face(other),
                },
            }),
        );
    }

    /// The control: with the arena intact the same call absorbs, so
    /// the row above pins the tear and not the fixture.
    #[test]
    fn absorption_of_the_same_pair_runs_on_an_intact_arena() {
        let tol = Tol::witness();
        let mut body = ops_cube(tol).body;
        let (rep, other) = adjacent_pair(&body);

        let group = body
            .merge_group(rep, &[other], tol)
            .expect("an intact adjacent pair absorbs");
        assert_eq!(group.kept, rep);
        assert_eq!(group.absorbed, vec![other]);
    }

    /// **The regime split, wired.** `records` is the whole of it and
    /// has one production call site, so this reds if the arena-fault
    /// conjunct is deleted from it — which pinning the classifier
    /// alone did not.
    ///
    /// A behavioural pin through the public door is impossible and
    /// that is the point of the escape: the door's tier-2 entry gate
    /// refuses a torn body before any group is staged, so no valid
    /// input reaches a corruption refusal under either regime. The
    /// escape states what the door promises, and this states that the
    /// promise is applied.
    #[test]
    fn only_inventory_refusals_are_ever_recorded() {
        let arena = MergeCoplanarError::Op {
            error: EulerOpError::OrbitBroken {
                he: crate::entity::HalfEdgeKey::default(),
            },
        };
        let inventory = MergeCoplanarError::GroupNotClosed { errors: Vec::new() };
        assert!(
            GroupRegime::RecordsASkip.records(&inventory),
            "the recording regime records an inventory refusal"
        );
        assert!(
            !GroupRegime::RecordsASkip.records(&arena),
            "a torn arena refuses the call even where the group would record"
        );
        assert!(
            !GroupRegime::RefusesTheCall.records(&inventory),
            "the refusing regime records nothing"
        );
        assert!(!GroupRegime::RefusesTheCall.records(&arena));
    }

    /// **The corruption class is the operator layer's, and it is
    /// nine variants wide, not two.** The door's docs promise that a
    /// refusal reporting a torn arena never becomes a record; this
    /// pins the membership that promise needs, at the sample the
    /// merge can actually raise, and pins that inventory refusals
    /// stay out of it.
    #[test]
    fn the_arena_fault_class_is_the_operator_layers_tier_one_row() {
        let he = crate::entity::HalfEdgeKey::default();
        let torn = [
            EulerOpError::StaleKey {
                key: EntityId::Face(FaceKey::default()),
            },
            EulerOpError::StaleGeometry {
                key: GeomRef::Surface(SurfaceKey::default()),
            },
            EulerOpError::OrbitBroken { he },
            EulerOpError::LoopCycleBroken {
                r#loop: LoopKey::default(),
            },
            EulerOpError::UnclaimedHalfEdge {
                he,
                edge: EdgeKey::default(),
            },
        ];
        for error in torn {
            assert!(
                error.reports_tier1_corruption(),
                "{error} is tier-1-invalid input by its own docs"
            );
            assert!(MergeCoplanarError::Op { error }.is_arena_fault());
        }
        // Facts about the operation, legal to meet on a valid body.
        let inventory = [
            EulerOpError::FaceHasRings {
                face: FaceKey::default(),
            },
            EulerOpError::SameFace {
                face: FaceKey::default(),
            },
        ];
        for error in inventory {
            assert!(!error.reports_tier1_corruption());
            assert!(!MergeCoplanarError::Op { error }.is_arena_fault());
        }
        // A merge-local refusal is never an arena fault.
        assert!(!MergeCoplanarError::GroupNotClosed { errors: Vec::new() }.is_arena_fault());
        assert!(
            !MergeCoplanarError::PeriodClosure {
                edge: EdgeKey::default()
            }
            .is_arena_fault()
        );
    }

    /// **`edge_halves` announces a torn link; it does not skip the
    /// edge.** Reds against the `else { continue }` it replaced: with
    /// the discard, the poisoned edge is passed over, the absorption
    /// finds nothing, and the group merges nothing while reporting
    /// `Ok`.
    #[test]
    fn a_torn_parent_loop_link_refuses_rather_than_skipping_the_edge() {
        let tol = Tol::witness();
        let mut body = ops_cube(tol).body;
        let (rep, other, he_plus) = body
            .edges()
            .find_map(|(_, e)| {
                let (hp, hm) = body.edge_halves(e.he_plus, e.he_minus).ok()?;
                (hp.face != hm.face).then_some((hp.face, hm.face, e.he_plus))
            })
            .expect("a cube has adjacent faces");
        body.half_edges
            .get_mut(he_plus)
            .expect("the shared edge's plus half is live")
            .parent_loop = LoopKey::default();

        assert_eq!(
            body.merge_group(rep, &[other], tol),
            Err(MergeCoplanarError::Op {
                error: EulerOpError::StaleKey {
                    key: EntityId::Loop(LoopKey::default()),
                },
            }),
            "a torn parent-loop link is announced, never passed over"
        );
    }

    /// **A broken orbit is announced, not read as "no tip".** Reds
    /// against the `is_some_and` it replaced, which answered `false`
    /// — routing the seam repair from `kev` to `kemr` and changing
    /// the group's Euler delta on a torn arena.
    ///
    /// It pins `strut_tip` rather than the surgery: reaching the tip
    /// search with a broken orbit needs the arena torn BETWEEN the
    /// straight-seam decision and the strut test, inside one call,
    /// which no test can do. `strut_tip` is that decision and has one
    /// production call site.
    #[test]
    fn a_broken_vertex_orbit_refuses_rather_than_answering_no_tip() {
        let tol = Tol::witness();
        let mut body = ops_cube(tol).body;
        let he = body
            .edges()
            .map(|(_, e)| e.he_plus)
            .next()
            .expect("a cube has edges");
        assert_eq!(
            body.strut_tip(he),
            Ok(false),
            "an intact cube's half-edge has a two-member orbit"
        );

        // Tear the edge back-pointer so the orbit's `mate` step
        // cannot resolve; the half-edge itself stays live.
        body.half_edges
            .get_mut(he)
            .expect("the half-edge is live")
            .edge = EdgeKey::default();
        assert_eq!(
            body.strut_tip(he),
            Err(EulerOpError::OrbitBroken { he }),
            "a broken orbit is a refusal, not a `false`"
        );
    }

    /// **The kind question is asked of every member.** A group whose
    /// members are not all of one surface kind has no regime, and the
    /// refusal names both sides rather than letting arena order pick
    /// the contract.
    ///
    /// The mixed group is built at the arena, because the door that
    /// mints one — the same-source rung gluing a plane to a cylinder
    /// on a stamped `GeomSource` — needs a curved body this crate has
    /// no fixture for; `sweep`'s cosurface suite carries that half.
    #[test]
    fn a_group_that_straddles_two_surface_kinds_has_no_regime() {
        let tol = Tol::witness();
        let mut body = ops_cube(tol).body;
        let (rep, other) = adjacent_pair(&body);
        // Both surfaces are set here: the fixture's faces carry the
        // mvfs placeholder, and the row is about KINDS, not about
        // which kind a fixture happens to leave behind.
        let plane = body.surfaces.insert(Surface::Plane {
            origin: geom_core::Point3::new(0.0, 0.0, 0.0),
            normal: geom_core::Vec3::new(0.0, 0.0, 1.0),
            u_ref: geom_core::Vec3::new(1.0, 0.0, 0.0),
        });
        let cylinder = body.surfaces.insert(Surface::Cylinder {
            origin: geom_core::Point3::new(0.0, 0.0, 0.0),
            axis: geom_core::Vec3::new(0.0, 0.0, 1.0),
            u_ref: geom_core::Vec3::new(1.0, 0.0, 0.0),
            radius: 1.0,
        });
        body.faces
            .get_mut(rep)
            .expect("the pair's first face is live")
            .surface = plane;
        body.faces
            .get_mut(other)
            .expect("the pair's second face is live")
            .surface = cylinder;

        assert_eq!(
            body.group_regime(rep, &[other], &std::collections::BTreeSet::new()),
            Err(MergeCoplanarError::GroupKindSplit {
                planar: rep,
                curved: other,
            }),
        );
        // Both orders answer, and both name the same two faces.
        assert_eq!(
            body.group_regime(other, &[rep], &std::collections::BTreeSet::new()),
            Err(MergeCoplanarError::GroupKindSplit {
                planar: rep,
                curved: other,
            }),
        );
    }

    /// The crate's one executable pin on this door's rendered
    /// refusals: `DESIGN.md`'s D4 ¶2(ii) names them as the in-repo
    /// precedent for message-level recourse, so the recourse sentence
    /// is asserted rather than left to a reader.
    #[test]
    fn the_refusals_render_their_recourse() {
        let rendered = |e: &MergeCoplanarError| e.to_string();
        assert!(
            rendered(&MergeCoplanarError::ResultNotClosed { errors: Vec::new() })
                .contains("refused"),
            "the whole run's after-gate says the call is refused"
        );
        let group = rendered(&MergeCoplanarError::GroupNotClosed { errors: Vec::new() });
        assert!(
            group.contains("unmerged") && group.contains("run continues"),
            "a group's own gate says the run survives it: {group}"
        );
        assert!(
            rendered(&MergeCoplanarError::DeclarationContradicted {
                diag: Indeterminate {
                    margin: geom_core::MarginDiag::Value(0.0),
                    band: Band::linear(Tol::witness()).expect("the witness band"),
                    predicate: Some("merge_declared_plane_eq"),
                },
            })
            .contains("fix the declaration or the geometry"),
            "the declared-pair contradiction carries its recourse"
        );
    }
}
