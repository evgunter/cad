//! The public boolean set operations (M3 PR 5): [`union`],
//! [`intersect`], [`subtract`] — functional (operands untouched),
//! composing reduce → classify (PR 4) → join → `setopfinish` → the
//! combine door → seam zip → the `merge_coplanar_faces` output stage
//! (F7) → tier gates. Every stage's refusal passes through typed as a
//! [`BooleanError`] variant.
//!
//! # Results (F8)
//!
//! ∅ is a typed SUCCESS ([`BooleanResult::Empty`]) — GQ2's per-node
//! result DAG wants a value, not an error. Real results carry a
//! [`BooleanResultKind`]:
//!
//! - [`Seamed`](BooleanResultKind::Seamed): boundaries intersected;
//!   the seam was zipped.
//! - [`OperandA`](BooleanResultKind::OperandA) /
//!   [`OperandB`](BooleanResultKind::OperandB): one operand's material
//!   is the whole answer (disjoint ∖, nested ∩, …).
//! - [`Assembly`](BooleanResultKind::Assembly): components of both
//!   operands without a seam, each piece a solid — the disjoint union
//!   (∪ of separated bodies), including assemblies touching at declared
//!   vertex and edge contacts (the carried
//!   [`ContactRecords`] say where; genuinely 3′, certified by PR 6's
//!   validator). A declared line contact through a face's interior is
//!   not one: its records would name points of the line, so the union
//!   refuses it ([`BooleanError::TangentSlitArmUnbuilt`]).
//! - [`Voided`](BooleanResultKind::Voided): **legitimate voids** —
//!   A∖B with B strictly inside A yields the outer shell plus the
//!   reverted inner shell, a tier-2-legal multi-shell body. The
//!   insertion itself is [`super::voids::insert_void`] — the shared
//!   void-insertion door every cavity is born through (this fallback,
//!   the holed full revolve, and `shell`'s sealed hollow), with this
//!   fallback's probe verdicts as the door's containment evidence. A
//!   hollow B's cavities land as pieces inside the void B leaves.
//!
//! **Bodies in, bodies out** (`docs/DESIGN.md`, "A solid is one piece
//! of material"). An operand may hold any number of solids: the
//! pipeline reads each operand as one multi-shell solid
//! ([`Body::merge_all_solids`] on a clone), since it classifies, keeps and
//! grafts shells, and every result leaves [`boolean_op_with`] sorted
//! into pieces ([`crate::pieces`]) — one `Outer` per solid, each `Void`
//! under the piece whose material surrounds it.
//!
//! When operand boundaries do not intersect, classification falls back
//! to per-shell containment against the pristine other operand: the
//! uncut-shell witness ([`super::shell_witness`]) probes the shell's
//! points with [`super::solid_contain::point_in_solid`] (F8's ray design
//! promoted to 3-D) until one lies off the other boundary.
//!
//! # The merge output stage (F7)
//!
//! The seam zip manufactures coplanar same-surface-key face pairs by
//! construction (a cut face's fragments), so each op runs
//! `merge_coplanar_faces` as a documented final stage — part of the
//! op's contract, not hidden healing (the recipe records ONE boolean
//! node). The mergeable pairs are structural/declared by construction;
//! cross-operand *numeric* coplanarity is honestly left unmerged (the
//! coincidence ladder has no numeric rung).
//!
//! # Carried contacts
//!
//! Result bodies carry the declared-contact records whose entities
//! survive into the result, remapped to result keys (B-side keys
//! through the combine door's graft map). Records referencing
//! discarded entities are dropped — a contact between A and B is only
//! meaningful in a result containing both sides.
//!
//! # Known limitations (PR 5.5 — the honest envelope)
//!
//! The seam lane's WORKING envelope (all exact-oracle-verified):
//! transversal boundary crossings; single-ring pockets/bosses on ALL
//! six face orientations (PR 5's R1 closed); double-ring single-face
//! seams (through-pillar tunnel, inset-leg union); multi-collinear-
//! site seams (R2) and crossing-polygon disconnections (R3),
//! including mixed collinear+transversal channel cuts (the PR 5.5
//! review's E-2, closed by the degenerate-segment fix in
//! `point_in_vertex_polygon`); interior-rest flush contacts (pillar standing on
//! a face); and the Fig 15.1 coplanar-overlap ∩ (seam partly on
//! shared cap planes) — the `join` module's derived sense/role
//! discipline is the consistency theorem behind all of them.
//!
//! Still refusing — typed, deterministic, operands untouched; never a
//! silent wrong body:
//!
//! - **Boundary-on-boundary seams** — NARROWED by M5 S1: declared
//!   UNIONS of pure REST contacts (the full-overlap stacked union,
//!   corner-flush rests, the mated cross-lap) now build through the
//!   declared-REST zip (`rest` module): when the chord join refuses
//!   typed on a declared ∪, the lane re-examines the reduction,
//!   realizes the seam structurally (existing edges reused, single
//!   chords minted), removes the coincident contact patches, and
//!   fuses the boundary — exact dyadic volume additivity. What still
//!   refuses, typed: undeclared mates (the coincidence door, ladder
//!   rung (b)); REST sub-frontiers the lane names
//!   (`RestZipUnsupported` — e.g. ring-carrying contact patches,
//!   non-star patch adjacency); and boundary-on-boundary
//!   configurations that are not pure REST contacts (the original
//!   `Join(UnpairedLooseEnds)` surfaces verbatim).
//! - **Four-germ vertex–vertex sites**: where a vertex of the other
//!   operand coincides with a 315° reflex corner and its wall lies
//!   flush on the corner's notch wall under a tilted cap, the vertex
//!   pair keeps four crossing germs, and `insert` runs each pair's null
//!   edge in B in A's germ order rather than B's: the B runs overlap and
//!   the op refuses (`Euler(FanStartMismatch)`, `JoinDesync`;
//!   `work/join/four-germ-vertex-pairs-run-b-in-a-order`). Some such
//!   unions are not refused: the declared-REST zip answers them after
//!   the join's refusal, with a wrong volume
//!   (`work/zip/a-flush-declared-reflex-union-ships-the-wrong-volume`).
//!   The vertex-on-face form of the same corner (the corner piercing a
//!   cap's interior) is a whole-orbit pierce run and answers exactly.

use geom_core::interval::Interval;
use geom_core::{Band, Bounds, Decide, Margin, Point3, Real, Sign, Tol, Vec3};

use std::collections::{BTreeMap, BTreeSet};

use super::BooleanDecision;
use super::SphereQuestion;
use super::boxes;
use super::combine::{Bridge, GraftMap, graft_solids_with};
use super::contain::{ContainError, FaceContainment, contfp};
use super::finish::setopfinish;
use super::join::bool_connect;
use super::section_cert::Refusal as SectionRefusal;
use super::shell_witness::{
    ShellVerdict, check_mutual, debug_assert_contacts_undecisive, kept_shells, shell_verdict,
};
use super::solid_contain::{SolidContainment, closed_sphere_group};
use super::voids;
use super::zip::{SeamCorrespondence, survivor, survivor_checked, zip_seam};
use super::{
    BooleanDeclarations, BooleanError, BooleanOp, BooleanReduction, CarriedContacts,
    ContactRecords, CurveContact, FacePairDeclaration, Operand, PatchContact, SweepStrategy,
    VfContact, VvContact,
};
use crate::body::Body;
use crate::entity::{EdgeKey, FaceKey, LoopBoundary, ShellKey, VertexKey};
use crate::geometry::SurfaceKey;
use crate::props::AtRestPolicy;
use crate::props::QuadLane;
use crate::splitting::finish::{carve, single_solid};
use crate::validate::{AtRestBody, decide};
use geom_brep::recourse::Refused;
use geom_core::k_stats::NonzeroSign;

/// How a boolean result body came to be (module docs).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BooleanResultKind {
    /// Boundaries intersected; the seam was joined and zipped.
    Seamed,
    /// The result is operand A's material.
    OperandA,
    /// The result is operand B's material.
    OperandB,
    /// A multi-shell combination of components from both operands
    /// without a seam (disjoint, or touching at vertices and edges).
    Assembly,
    /// A∖B with B inside A: outer shell + reverted inner void shell.
    Voided,
}

/// A real (non-empty) boolean result.
///
/// # Validity-class carriage (D2 — the F1 contract)
///
/// The validity class rides THIS wrapper, never a mutable field on
/// [`Body`]: `body` is a finished body ([`AtRestBody`]), whose tier-3
/// verdict the door took on these bits ([`AtRestPolicy::gate_at_rest_kept`]:
/// [`AtRestOutcome::Validated`](crate::AtRestOutcome::Validated) at a
/// certifying scalar, [`AtRestOutcome::NotRunAtThisScalar`](crate::AtRestOutcome::NotRunAtThisScalar)
/// at a dual). `contacts` are the machine-checkable record of every
/// intentional touching the pipeline propagated (F2's explicit-intent
/// condition), and `body.validate_pseudomanifold(&contacts, tol)` is
/// the tier-3′ pass over them, empty or not. The door does not run
/// that census: a curved solid within reach of another solid is
/// beyond its cross-solid lane, which would refuse valid disjoint
/// unions (`work/contact/census-cross-solid-curved-pairs-undecidable-on-shell-results.md`).
#[derive(Debug)]
pub struct BooleanBody<T: Real> {
    /// The result body, finished: one solid per piece of material
    /// ([`crate::pieces`]), gated at tier 3 by the door that built it.
    pub body: AtRestBody<T>,
    /// How it was produced.
    pub kind: BooleanResultKind,
    /// Declared contacts surviving into the result, in result keys
    /// (module docs) — the tier-3′ declarations (type-level docs).
    pub contacts: ContactRecords,
    /// Naming emission (M4 PR 3, NAMING-DESIGN N4): the mint-time
    /// wiring facts the naming layer consumes — recorded as the
    /// pipeline runs, never reconstructed by post-hoc inspection.
    pub naming: BooleanNaming,
}

/// How one operand's keys relate to the result body's keys.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum OperandKeys {
    /// The result arena IS this operand's clone: operand keys resolve
    /// directly (surviving entities keep their keys).
    #[default]
    Direct,
    /// This operand was grafted in: its surviving keys appear as the
    /// source column of the corresponding `graft_*` rows.
    Grafted,
    /// No material of this operand is in the result.
    Absent,
}

/// Mint-time naming facts of one boolean result (M4 PR 3). Rows are
/// historical: a listed key may have been consumed by a later stage
/// (zip scaffolding, merge absorption, discarded material) — consumers
/// filter against the entities alive in `body` / chase the rows.
#[derive(Debug, Default)]
pub struct BooleanNaming {
    /// How operand A's keys map into the result.
    pub a_keys: OperandKeys,
    /// How operand B's keys map into the result.
    pub b_keys: OperandKeys,
    /// B-side graft lineage, `(B key, result key)` in arena slot
    /// order (empty unless `b_keys` is `Grafted`). Source keys are
    /// B-CLONE keys: operand keys for surviving operand entities plus
    /// reduction-minted keys.
    pub graft_vertices: Vec<(VertexKey, VertexKey)>,
    /// B-side edge graft lineage (see `graft_vertices`).
    pub graft_edges: Vec<(EdgeKey, EdgeKey)>,
    /// B-side edges the grafted body's records name but that died in
    /// B before the graft, `(B key, result key)` in B-key order: the
    /// result key is dead on arrival and stands for the B key in every
    /// forwarded record (a split's parent), so a lineage chased in the
    /// result reads back to the B ancestor through this row.
    pub graft_dead_edges: Vec<(EdgeKey, EdgeKey)>,
    /// B-side face graft lineage (see `graft_vertices`).
    pub graft_faces: Vec<(FaceKey, FaceKey)>,
    /// Seam edges surviving the zips, in zip/cycle order, result keys.
    pub seam_edges: Vec<EdgeKey>,
    /// Vertex fusions `(dead, kept)` in mint order, result keys: the
    /// A-side pinch welds' first, then the zips'.
    pub vertex_merges: Vec<(VertexKey, VertexKey)>,
    /// The B-side pinch welds' vertex fusions `(dead, kept)` in mint
    /// order, in B-CLONE keys: they ran before the graft, so a dead key
    /// has no result key (translate the kept column through
    /// `graft_vertices`).
    pub weld_merges_b: Vec<(VertexKey, VertexKey)>,
    /// `merge_coplanar_faces` absorption groups `(kept, absorbed…)`,
    /// result keys.
    pub merge_groups: Vec<(FaceKey, Vec<FaceKey>)>,
    /// Curved merge groups the output stage did NOT glue, as outside
    /// the merge's Euler inventory (M4 PR 5), and declared surface pairs
    /// the door has no rung for (a non-planar carrier) — the record's
    /// faces plus the typed
    /// [`MergeCoplanarError`](crate::merge_faces::MergeCoplanarError)
    /// that stopped each, carried whole. WHICH groups are recorded
    /// here rather than refusing the whole call is the regime's own
    /// statement, at
    /// [`MergeCoplanarOutcome::skipped`](crate::merge_faces::MergeCoplanarOutcome::skipped),
    /// and is not restated here. The skip is visible HERE — a
    /// consumer can see what was not glued and why; the skipped
    /// faces' in-plane descriptions are re-checked against the
    /// actual adjacency before the result ships (review F1/F2).
    pub merge_skipped: Vec<crate::merge_faces::SkippedMerge>,
    /// A-side chord-mef fragment rows `(new face, divided-from face)`
    /// in mint order — A-clone keys, which ARE result keys when
    /// `a_keys` is `Direct`.
    pub face_fragments_a: Vec<(FaceKey, FaceKey)>,
    /// B-side chord-mef fragment rows, in B-CLONE keys (translate the
    /// new-face column through `graft_faces` for result keys).
    pub face_fragments_b: Vec<(FaceKey, FaceKey)>,
    /// The reduction's declared-contact records BEFORE result
    /// remapping: each row's A column in A-CLONE keys and its B column
    /// in B-CLONE keys — the result's keys on whichever side is
    /// `Direct`, the graft's source keys on a `Grafted` side, and keys
    /// of no body in the result on an `Absent` side. The mint-time
    /// crossing correspondences the
    /// naming layer reads even when one side's key was consumed
    /// (`BooleanBody::contacts` drops such rows by design).
    pub reduction_contacts: ContactRecords,
    /// Every face an operand lost to the result, with the kept faces it
    /// bordered (`boolean::discard`, which says which paths record
    /// rows and why the others have none to record).
    pub discards: Vec<super::DiscardRow>,
    /// Each pair `(A face, B face)` of coincident faces where the result
    /// holds either face's region through the other: their materials lie
    /// on one side, so the classification keeps one operand's copy of the
    /// region they share and drops the other's (Eq. 15.3). The pair is
    /// one fact whichever copy is kept. Clone keys, as
    /// [`super::DiscardRow::face`]: chase `face_fragments_a`/`face_fragments_b`
    /// for the operand faces. Read off the classification, so every path
    /// that classifies records it — the section path, the containment
    /// fallback and the declared-REST union — sorted and deduplicated;
    /// a path that never classifies (disjoint boxes) has none.
    pub covered: Vec<(FaceKey, FaceKey)>,
}

impl BooleanNaming {
    /// Each result vertex an A-side weld or a zip fused away → the
    /// vertex it finally fused into, following `vertex_merges` through
    /// every hop (a discard's `bordered` ends are read through it). B-side
    /// welds are not here: they killed B-clone keys before the graft, so
    /// no result key names them (`weld_merges_b`).
    #[must_use]
    pub fn fused_into(&self) -> BTreeMap<VertexKey, VertexKey> {
        self.vertex_merges
            .iter()
            .filter(|(dead, kept)| dead != kept)
            .map(|&(dead, _)| (dead, survivor(&self.vertex_merges, dead)))
            .collect()
    }
}

/// The typed result of a boolean op: a body, or the typed empty
/// success (F8: ∅ is a value, not an error).
// Size skew vs `Empty` is inherent (same posture as `SplitPart`).
#[allow(clippy::large_enum_variant)]
#[derive(Debug)]
pub enum BooleanResult<T: Real> {
    /// The regularized result is empty.
    Empty,
    /// A real result body.
    Body(BooleanBody<T>),
}

impl<T: Real> BooleanResult<T> {
    /// The result body, if non-empty.
    pub fn body(&self) -> Option<&BooleanBody<T>> {
        match self {
            Self::Body(b) => Some(b),
            Self::Empty => None,
        }
    }
}

/// A ∪* B (module docs; functional). Surface kinds are gated per arm,
/// not wholesale — see `reduce::gate_operand_pairs`.
///
/// # Errors
///
/// [`BooleanError`] — every stage's typed refusals pass through.
pub fn union<T: Decide + Bounds + crate::props::AtRestPolicy>(
    a: &AtRestBody<T>,
    b: &AtRestBody<T>,
    tol: Tol,
) -> Result<BooleanResult<T>, BooleanError> {
    boolean_op_with(
        BooleanOp::Union,
        a,
        b,
        &BooleanDeclarations::none(),
        SweepStrategy::Realized,
        tol,
    )
}

/// A ∩* B (module docs).
///
/// # Errors
///
/// [`BooleanError`].
pub fn intersect<T: Decide + Bounds + crate::props::AtRestPolicy>(
    a: &AtRestBody<T>,
    b: &AtRestBody<T>,
    tol: Tol,
) -> Result<BooleanResult<T>, BooleanError> {
    boolean_op_with(
        BooleanOp::Intersect,
        a,
        b,
        &BooleanDeclarations::none(),
        SweepStrategy::Realized,
        tol,
    )
}

/// A ∖* B (module docs).
///
/// # Errors
///
/// [`BooleanError`].
pub fn subtract<T: Decide + Bounds + crate::props::AtRestPolicy>(
    a: &AtRestBody<T>,
    b: &AtRestBody<T>,
    tol: Tol,
) -> Result<BooleanResult<T>, BooleanError> {
    boolean_op_with(
        BooleanOp::Subtract,
        a,
        b,
        &BooleanDeclarations::none(),
        SweepStrategy::Realized,
        tol,
    )
}

/// A ∪* B with declared coincidence intents (F5, M4 PR 5) — see
/// [`BooleanDeclarations`].
///
/// # Errors
///
/// [`BooleanError`].
pub fn union_with<T: Decide + Bounds + crate::props::AtRestPolicy>(
    a: &AtRestBody<T>,
    b: &AtRestBody<T>,
    decls: &BooleanDeclarations,
    tol: Tol,
) -> Result<BooleanResult<T>, BooleanError> {
    boolean_op_with(BooleanOp::Union, a, b, decls, SweepStrategy::Realized, tol)
}

/// A ∩* B with declared coincidence intents ([`union_with`]).
///
/// # Errors
///
/// [`BooleanError`].
pub fn intersect_with<T: Decide + Bounds + crate::props::AtRestPolicy>(
    a: &AtRestBody<T>,
    b: &AtRestBody<T>,
    decls: &BooleanDeclarations,
    tol: Tol,
) -> Result<BooleanResult<T>, BooleanError> {
    boolean_op_with(
        BooleanOp::Intersect,
        a,
        b,
        decls,
        SweepStrategy::Realized,
        tol,
    )
}

/// A ∖* B with declared coincidence intents ([`union_with`]).
///
/// # Errors
///
/// [`BooleanError`].
pub fn subtract_with<T: Decide + Bounds + crate::props::AtRestPolicy>(
    a: &AtRestBody<T>,
    b: &AtRestBody<T>,
    decls: &BooleanDeclarations,
    tol: Tol,
) -> Result<BooleanResult<T>, BooleanError> {
    boolean_op_with(
        BooleanOp::Subtract,
        a,
        b,
        decls,
        SweepStrategy::Realized,
        tol,
    )
}

/// The shared pipeline (module docs), with an explicit
/// [`SweepStrategy`] — the idealized/realized door (PERF-PLAN §4.4):
/// the tree only changes candidate GENERATION, so both strategies
/// produce bit-identical results; the differential suite runs full
/// ops through both and pins exactly that. Production wrappers pass
/// [`SweepStrategy::Realized`].
///
/// # Errors
///
/// [`BooleanError`] — identical to [`union`] and friends.
pub fn boolean_op_with<T: Decide + Bounds + crate::props::AtRestPolicy>(
    op: BooleanOp,
    a: &AtRestBody<T>,
    b: &AtRestBody<T>,
    decls: &BooleanDeclarations,
    strategy: SweepStrategy,
    tol: Tol,
) -> Result<BooleanResult<T>, BooleanError> {
    #[cfg(feature = "door-tier3-meter")]
    let door_from = std::time::Instant::now();
    let result = boolean_door(op, a, b, decls, strategy, tol);
    #[cfg(feature = "door-tier3-meter")]
    super::door_meter::record(op, &result, door_from.elapsed(), [a.outcome(), b.outcome()]);
    result
}

/// [`boolean_op_with`]'s body.
fn boolean_door<T: Decide + Bounds + crate::props::AtRestPolicy>(
    op: BooleanOp,
    a: &Body<T>,
    b: &Body<T>,
    decls: &BooleanDeclarations,
    strategy: SweepStrategy,
    tol: Tol,
) -> Result<BooleanResult<T>, BooleanError> {
    // The curved ∖/∩ front door, per class (C12.1 — retire per class,
    // never wholesale). Both ops route regions through `revert`
    // (A∖B ≡ A∩revert(B), the §15.9 posture), which is kind-generic:
    // `Face::sense` carries the flip on every carrier. The roster
    // (`reduce::revert_arm_exists`) is ∪'s (`boolean_arm_exists`) minus
    // `Nurbs`. A kind on it has a crossing layer whose every door is
    // certified or typed, and a no-crossings path that certifies its
    // reach or refuses — the sphere's extent by `sphere_extent_scan`
    // and its re-cut, then every pair with a curved face by
    // `section_extent_pass` — with `interior_loop_verdict` guarding the crossings path, so no
    // face pair of the kind is answered by a vertex probe that could
    // not see into it. `Nurbs` is on ∪'s roster for its plane×NURBS
    // germ arm, but has no edge×NURBS-face crossing layer (deviation 5),
    // so ∖ and ∩ have no seam lane for it; `Cone` has no arm under any
    // op.
    //
    // Up front and PAIR-SCOPED: the kinds are read exactly, and the
    // question of whether a kind can matter to this operation is
    // decided by boxes (`reduce::first_unsupported_pair` — non-overlap
    // is a certificate, overlap is a may). Operands untouched, no
    // reduction work before it.
    //
    // **No covered-pair rung here, and the asymmetry is the point.**
    // The operand gate admits a declared pair because a declaration
    // supplies the VERDICT a germ arm would have supplied. This roster
    // is not about verdicts: it names the kinds that have a seam lane
    // to revert through, and no declaration can supply one. A declared
    // pair of a kind off this roster is exactly as refused as an
    // undeclared one, and says so at the same site.
    if !matches!(op, BooleanOp::Union) {
        let band = Band::linear(tol)?;
        if let Some(p) = super::reduce::first_unsupported_pair(
            a,
            b,
            band,
            super::reduce::revert_arm_exists,
            |_, _, _| false,
        )? {
            return Err(BooleanError::CurvedPairUnsupported {
                op: Some(op),
                site: super::PairRefusalSite::RevertRoster,
                operand: p.operand,
                face: p.face,
                kind: p.kind,
                other_face: p.other_face,
                other_kind: p.other_kind,
            });
        }
    }
    let (a, b) = (one_solid(a)?, one_solid(b)?);
    boolean_op_recut(op, &a, &b, decls, strategy, true, tol)
}

/// `body` as the pipeline reads an operand: as is when it holds at most
/// one solid, else a clone with every shell under one solid (module
/// docs, "Bodies in, bodies out").
fn one_solid<T: Decide>(body: &Body<T>) -> Result<std::borrow::Cow<'_, Body<T>>, BooleanError> {
    if body.solids().nth(1).is_none() {
        return Ok(std::borrow::Cow::Borrowed(body));
    }
    let mut flat = body.clone();
    flat.merge_all_solids().map_err(BooleanError::Euler)?;
    Ok(std::borrow::Cow::Owned(flat))
}

/// The pipeline behind the front door, parameterized on whether the
/// no-crossings sphere RE-CUT (M5 S13) may still run: the re-entry
/// pass sets `recut = false`, so a re-cut that surfaces no crossings
/// is a loud invariant failure rather than a loop.
fn boolean_op_recut<T: Decide + Bounds + crate::props::AtRestPolicy>(
    op: BooleanOp,
    a: &Body<T>,
    b: &Body<T>,
    decls: &BooleanDeclarations,
    strategy: SweepStrategy,
    recut: bool,
    tol: Tol,
) -> Result<BooleanResult<T>, BooleanError> {
    let band = Band::linear(tol)?;
    let (red, connected, interior_loops) =
        match through_the_join(op, a, b, decls, strategy, recut, tol)? {
            Joined::Answered(result) => return Ok(*result),
            Joined::Connected {
                red,
                connected,
                interior_loops,
            } => (*red, connected, interior_loops),
        };
    let contacts = red.contacts.clone();
    let reduction_contacts = red.contacts.clone();
    let covered = red.covered.clone();
    let copies = Descendants::null_copies(&red.null_edges);
    let fin = setopfinish(op, red, &connected, a, b, band, tol)?;
    // The zip, the merge, the re-description and the closing mint are
    // one door's surgery (`crate::surgery`): the operators inside them
    // do not each re-derive the whole body, and `gate` below — tier 3
    // over the result, on every build — is what this door pays instead.
    // The guard owns the borrow, so a refusal on the way closes the
    // scope by dropping it.
    let mut finished = fin.body;
    let mut body = finished.begin_surgery();
    let mut seam_edges = Vec::new();
    let mut vertex_merges = fin.weld_merges_a.clone();
    let mut desc = Descendants::welded(&fin.weld_merges_a, &fin.weld_merges_b).with_copies(copies);
    // A pinch is one vertex on two seams: the first zip fuses it, so
    // each later zip reads the correspondence through the fusions made.
    let mut vertex_map = fin.vertex_map.clone();
    for &(a_face, b_face) in &fin.seams {
        let rep = zip_seam(&mut body, a_face, b_face, &vertex_map, tol)?;
        desc.absorb_zip(&rep);
        vertex_merges.extend(rep.vertex_merges.iter().copied());
        seam_edges.extend(rep.seam_edges);
        vertex_map = fused_through(&vertex_map, &rep.vertex_merges);
    }
    let declared_pairs = declared_surface_pairs(&body, a, b, decls, &fin.graft);
    let merged = body
        .merge_coplanar_faces_declared(&declared_pairs, tol)
        .map_err(BooleanError::Merge)?;
    desc.absorb_merge(&merged);
    describe_minted_edges(&mut body, &seam_edges, &merged, band, tol)?;
    let mut contacts = remap_contacts(
        &body,
        &contacts,
        KeyView::Direct,
        KeyView::Graft(&fin.graft),
        &desc,
    )?;
    remap_carried(
        &mut contacts,
        &body,
        decls,
        &KeyView::Direct,
        &KeyView::Graft(&fin.graft),
        &desc,
    )?;
    // Curved results carry certified per-half-edge pcurves at rest
    // (M5 PR 9, the PR 6 contract): re-derive the whole cache set on
    // the finished body — the same pass the split lane runs. A planar
    // body mints nothing (no curved faces), so the M3 lane is
    // untouched bit-identically.
    crate::pcurves::mint_pcurves(&mut body, tol)
        .map_err(|source| BooleanError::Pcurves { source })?;
    body.sweep_and_close();
    let body = gate(finished, band, tol)?;
    T::gate_volume_backstop(op, a, b, &body, band, tol)?;
    interior_loops?;
    let (graft_vertices, graft_edges, graft_dead_edges, graft_faces) = graft_rows(&fin.graft);
    let naming = BooleanNaming {
        a_keys: OperandKeys::Direct,
        b_keys: OperandKeys::Grafted,
        graft_vertices,
        graft_edges,
        graft_dead_edges,
        graft_faces,
        seam_edges,
        vertex_merges,
        weld_merges_b: fin.weld_merges_b,
        merge_groups: merge_rows(&merged),
        merge_skipped: merged.skipped.clone(),
        face_fragments_a: [connected.a_fragments, fin.weld_fragments_a].concat(),
        face_fragments_b: [connected.b_fragments, fin.weld_fragments_b].concat(),
        reduction_contacts,
        discards: fin.discards,
        covered,
    };
    Ok(BooleanResult::Body(BooleanBody {
        body,
        kind: BooleanResultKind::Seamed,
        contacts,
        naming,
    }))
}

/// `map` with each vertex on either side read as the vertex a zip's
/// fusions `(dead, kept)` left in its place.
fn fused_through(
    map: &SeamCorrespondence,
    merges: &[(VertexKey, VertexKey)],
) -> SeamCorrespondence {
    let mut out = SeamCorrespondence::new();
    for (&a, bs) in map {
        out.entry(survivor(merges, a))
            .or_default()
            .extend(bs.iter().map(|&b| survivor(merges, b)));
    }
    out
}

/// What the pipeline reaches through its join ([`through_the_join`]).
pub(super) enum Joined<T: Real> {
    /// The pipeline's answer, reached without a join to finish: the
    /// no-crossings path (the re-cut or the containment fallback), or
    /// the declared-REST door taking a refused join.
    Answered(Box<BooleanResult<T>>),
    /// The join, done: the reduction with both operands as it leaves
    /// them, every null edge killed, what it completed (never empty),
    /// and the interior-loop verdict, which the pipeline raises only
    /// where a body is about to be returned.
    Connected {
        /// The reduction, its operands joined.
        red: Box<BooleanReduction<T>>,
        /// The completed polygons and the fragments the join made.
        connected: super::join::Connected,
        /// [`interior_loop_verdict`]'s answer, not yet raised.
        interior_loops: Result<(), BooleanError>,
    },
}

/// **The pipeline through its join**: the reduction, then the
/// no-crossings path where there is no null pair, and otherwise the
/// join, with the declared-REST door behind a join that refuses.
/// [`boolean_op_recut`] finishes what it returns, and the test hook
/// that stops at the join (`boolean::through_the_join`) reads it, so
/// the two run one sequence.
///
/// # Errors
///
/// The reduction's, the no-crossings path's and the join's refusals.
pub(super) fn through_the_join<T: Decide + Bounds + crate::props::AtRestPolicy>(
    op: BooleanOp,
    a: &Body<T>,
    b: &Body<T>,
    decls: &BooleanDeclarations,
    strategy: SweepStrategy,
    recut: bool,
    tol: Tol,
) -> Result<Joined<T>, BooleanError> {
    let band = Band::linear(tol)?;
    let mut red = super::boolean_reduce_declared_strategy(op, a, b, decls, strategy, tol)?;

    if red.null_pairs.is_empty() {
        if !red.null_edges.is_empty() {
            return Err(BooleanError::ClassificationInvariant {
                what: "null edges without pairs reached the op",
            });
        }
        // M5 S13, the containment-fallback re-cut. Before any vertex
        // is probed, the curved-EXTENT scan certifies the sphere class
        // structurally: every closed sphere group's true extent
        // (center ± r) is consulted against every face of the other
        // operand — exact structure and certified enclosures, never a
        // sampled normal. Three outcomes:
        //
        // - **no escape**: the boundaries are certified disjoint
        //   (sphere-involved pairs), so the vertex-probe fallback's
        //   whole-shell answer is sound — proceed.
        // - **escape** (a sphere definitely leaves the other solid
        //   through a plane face — the S12 finding's
        //   poking-but-not-crossing shape): the operand is RE-CUT —
        //   the closed group is rigidly re-charted about the escape
        //   normal (a rotation about its own center: the same point
        //   set, seams now transverse to the escape planes) and the
        //   pipeline re-enters once; the ordinary crossing layer then
        //   finds the section circles and the (Plane, Sphere) germ arm
        //   joins them exactly.
        // - **uncertifiable** (NURBS re-gate, trimmed sphere groups,
        //   sphere faces meeting, tangency, boundary-grazing circles,
        //   one group escaping through NON-PARALLEL faces): typed
        //   refusal — the S12 silence never re-opens.
        let recuts = sphere_extent_scan(a, b, band)?;
        if !recuts.is_empty() {
            if !recut {
                return Err(BooleanError::ClassificationInvariant {
                    what: "re-cut sphere operands still produced no crossings",
                });
            }
            let (a2, b2) = apply_recuts(a, b, &recuts, tol)?;
            return boolean_op_recut(op, &a2, &b2, decls, strategy, false, tol)
                .map(|result| Joined::Answered(Box::new(result)));
        }
        // The curved kinds the extent scan leaves: every torus,
        // cylinder and cone face's pairs, certified per pair by the
        // section certificate or refused typed.
        section_extent_pass(a, b, band)?;
        return fallback(op, &red, a, b, decls, band, tol)
            .map(|result| Joined::Answered(Box::new(result)));
    }

    // The declared-REST union door (M5 S1): a declared union whose
    // join refuses typed may be the boundary-on-boundary REST
    // frontier — the lane re-examines the UNMUTATED reduction and
    // either zips the mate or reproduces the original refusal
    // verbatim. The clones are taken only when the door can open
    // (declared union), so undeclared and non-union ops pay nothing.
    // Decided on the reduction, while its contacts still name the
    // operands' own faces; RAISED only where a body is about to be
    // returned, so every refusal the pipeline meets first stands
    // verbatim ([`interior_loop_verdict`]).
    let interior_loops = interior_loop_verdict(op, a, b, &red, decls, band);
    let rest_door = op == BooleanOp::Union && !decls.coincident_faces.is_empty();
    let saved = rest_door.then(|| (red.a.clone(), red.b.clone()));
    // The join carves both reduction operands through the Euler
    // operators; one scope per operand body, and what certifies the
    // result is `gate` below, over the body they are finished into.
    // The pair is guardless because the join takes the whole
    // reduction — `BooleanReduction::enter_join_surgery` carries the
    // argument — and `red` is a local of this pipeline, so a refusal
    // on the way drops it.
    red.enter_join_surgery();
    let connected = bool_connect(&mut red, a, b, band, tol);
    red.leave_join_surgery(connected.is_ok());
    let connected = match connected {
        Ok(c) => c,
        Err(
            err @ (BooleanError::Join(_)
            | BooleanError::JoinDesync { .. }
            | BooleanError::CurvedBooleanUnsupported { .. }),
        ) => match saved {
            Some((sa, sb)) => {
                red.a = sa;
                red.b = sb;
                return match super::rest::try_rest_union(red, a, b, decls, band, tol)? {
                    Some(result) => {
                        interior_loops?;
                        Ok(Joined::Answered(Box::new(result)))
                    }
                    // Not the REST frontier: the original join
                    // refusal stands, verbatim.
                    None => Err(err),
                };
            }
            None => return Err(err),
        },
        Err(e) => return Err(e),
    };
    if connected.completed.is_empty() {
        return Err(BooleanError::JoinDesync {
            what: "null pairs joined into no completed polygon",
        });
    }
    Ok(Joined::Connected {
        red: Box::new(red),
        connected,
        interior_loops,
    })
}

/// **The crossings path's guard for the interior-loop class: the
/// section certificate, per pair** ([`super::section_cert`], whose
/// module docs carry the argument).
///
/// Every undeclared cross-operand pair whose certified boxes overlap,
/// where either face is not a plane, is classified and certified. A DECLARED pair is exempt: its contact is
/// the verified carrier the declared rungs walk along its edges. The
/// first refusing pair, in arena order (A's faces, then B's), refuses
/// the operation as the operand gate refuses a pair with no arm —
/// [`BooleanError::CurvedPairUnsupported`] with the op named — and its
/// `face` is the pair's A face when that face is curved, else its B
/// face.
///
/// Decided on the unmutated reduction and raised only where a body
/// would be returned (the call site), so every refusal the pipeline
/// meets first stands verbatim.
fn interior_loop_verdict<T: Decide + Bounds + crate::props::AtRestPolicy>(
    op: BooleanOp,
    a: &Body<T>,
    b: &Body<T>,
    red: &BooleanReduction<T>,
    decls: &BooleanDeclarations,
    band: Band,
) -> Result<(), BooleanError> {
    let events = event_pairs(red)?;
    let pairs = section_pairs(
        a,
        b,
        band,
        SectionPath::Crossings,
        |fa, fb| declares_pair(decls, fa, fb),
        |fa, fb| events.contains(&(fa, fb)),
        true,
    )?;
    match pairs.iter().find(|p| p.verdict.is_err()) {
        None => Ok(()),
        Some(p) => {
            let (operand, face, kind, other_face, other_kind) = p.named();
            Err(BooleanError::CurvedPairUnsupported {
                op: Some(op),
                site: super::PairRefusalSite::InteriorLoopGuard,
                operand,
                face,
                kind,
                other_face,
                other_kind,
            })
        }
    }
}

/// Does a declaration speak for EXACTLY the pair `(A face, B face)`?
/// A declaration covers its own pair and no other: a face declared
/// against one partner meets every other partner undeclared.
pub(crate) fn declares_pair(decls: &BooleanDeclarations, fa: FaceKey, fb: FaceKey) -> bool {
    decls
        .coincident_faces
        .iter()
        .any(|d| d.a == fa && d.b == fb)
}

/// **`chart_boundary`'s verdict per face, asked once.** Keyed by the
/// operand as well as the face: the two operands' arenas mint their
/// keys independently, so A's face and B's face can share a key. (The
/// map keys the operand as "is B", `Operand` carrying no order.)
#[derive(Default)]
pub(crate) struct ChartCache(BTreeMap<(bool, FaceKey), bool>);

impl ChartCache {
    /// Does `operand`'s `face` of `body` describe — does every closed
    /// curve in its interior lift to a closed curve with zero winding
    /// (W2's premise)? Yes when [`crate::pcurves::chart_boundary`]
    /// answers `Ok`, and on a cone face also when the apex closure
    /// closes ([`crate::chord_join::cone_apex_closure`]: one apex
    /// visit, no ring) on a window at most a period wide.
    ///
    /// The cone clause is sound although `chart_boundary` refuses the
    /// apex as a singular joint. The apex lies on `∂F`, so `int F`
    /// excludes it, and `int F` maps homeomorphically onto the interior
    /// of the lifted region: a bounded region of one sheet of the
    /// punctured nappe's universal cover, at most a period wide. Every
    /// closed curve in `int F` therefore lifts to a closed curve with
    /// zero winding, and no essential component lies in `int F`.
    pub(crate) fn describes<T: crate::props::AtRestPolicy>(
        &mut self,
        operand: Operand,
        body: &Body<T>,
        face: FaceKey,
        surface: &geom::Surface<T>,
        band: Band,
    ) -> bool {
        *self
            .0
            .entry((operand == Operand::B, face))
            .or_insert_with(|| {
                crate::pcurves::chart_boundary(body, face, surface, band).is_ok()
                    || apex_closure_describes(body, face, surface, band)
            })
    }
}

/// The cone clause of [`ChartCache::describes`]: the apex closure
/// closes, on a window not definitely wider than a period.
fn apex_closure_describes<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    surface: &geom::Surface<T>,
    band: Band,
) -> bool {
    use crate::chord_join::ApexClosure;
    if !matches!(surface, geom::Surface::Cone { .. }) {
        return false;
    }
    let Ok(ApexClosure::Closed { window, reach, .. }) =
        crate::chord_join::cone_apex_closure(body, surface, face, band)
    else {
        return false;
    };
    matches!(
        crate::validate::decide(
            "bool_cone_closure_period",
            Margin::levered(T::tau() - (window.1 - window.0), reach),
            band,
        ),
        Ok(Sign::Positive | Sign::Zero)
    )
}

/// Which path a section scan serves; the two differ in which pairs
/// they examine.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SectionPath {
    /// The crossings path: every pair with a face that is not a plane.
    Crossings,
    /// The no-crossings fallback: every pair with a face that is not a
    /// plane, except a sphere against a plane, a sphere or a spline —
    /// those are the extent scan's ([`sphere_extent_scan`]), which runs
    /// first and keeps its re-cut. A sphere against a cylinder, a torus
    /// or a cone is certified here: none is ever an escape face, so the
    /// scan's only question of the pair is disjointness, and with no
    /// event anywhere "every component cleared" is disjointness.
    Fallback,
}

impl SectionPath {
    fn scope<T: Real>(self, x: &geom::Surface<T>, y: &geom::Surface<T>) -> bool {
        use geom::Surface as S;
        let curved = |s: &geom::Surface<T>| !matches!(s, S::Plane { .. });
        let sphere = |s: &geom::Surface<T>| matches!(s, S::Sphere { .. });
        let scanned =
            (sphere(x) && !section_pass_takes(y)) || (sphere(y) && !section_pass_takes(x));
        (curved(x) || curved(y)) && !(self == Self::Fallback && scanned)
    }

    /// Is `s` a face this path's refusal names?
    fn names<T: Real>(self, s: &geom::Surface<T>) -> bool {
        self.scope(s, s)
    }
}

/// **The kinds whose pairs with a sphere the section pass certifies on
/// the no-crossings path**, rather than the extent scan
/// ([`sphere_extent_scan`]): none is ever an escape face, so the scan's
/// only question of the pair is disjointness, which the pass answers per
/// face pair.
fn section_pass_takes<T: Real>(s: &geom::Surface<T>) -> bool {
    use geom::Surface as S;
    match s {
        S::Cylinder { .. } | S::Torus { .. } | S::Cone { .. } => true,
        S::Plane { .. } | S::Sphere { .. } | S::Nurbs(_) | S::Approx(_) => false,
    }
}

/// One pair's certificate outcome.
#[derive(Clone, Debug)]
pub(crate) struct PairVerdict {
    /// The pair's face of A.
    pub a_face: FaceKey,
    /// Its kind.
    pub a_kind: geom::SurfaceKind,
    /// The pair's face of B.
    pub b_face: FaceKey,
    /// Its kind.
    pub b_kind: geom::SurfaceKind,
    /// A's face is one the path names in its refusal.
    pub a_named: bool,
    /// Each component's witness, or the pair's refusal.
    pub verdict: Result<Vec<super::section_cert::Cleared>, super::section_cert::Refusal>,
}

impl PairVerdict {
    /// `(operand, face, kind, other_face, other_kind)`, the named face
    /// first.
    fn named(
        &self,
    ) -> (
        Operand,
        FaceKey,
        geom::SurfaceKind,
        FaceKey,
        geom::SurfaceKind,
    ) {
        if self.a_named {
            (
                Operand::A,
                self.a_face,
                self.a_kind,
                self.b_face,
                self.b_kind,
            )
        } else {
            (
                Operand::B,
                self.b_face,
                self.b_kind,
                self.a_face,
                self.a_kind,
            )
        }
    }
}

/// Does the face carry a lone-vertex loop?
fn has_lone_vertex<T: Real>(body: &Body<T>, face: FaceKey) -> Result<bool, BooleanError> {
    let corrupt = || BooleanError::ClassificationInvariant {
        what: "section certificate: a face loop is lost",
    };
    let fd = body.get_face(face).ok_or_else(corrupt)?;
    for l in core::iter::once(fd.outer).chain(fd.rings.iter().copied()) {
        if matches!(
            body.get_loop(l).ok_or_else(corrupt)?.boundary,
            LoopBoundary::Empty { .. }
        ) {
            return Ok(true);
        }
    }
    Ok(false)
}

/// Places a witness point in one face: `contfp` on a plane, the chart
/// trim on a curved face. A point the trim puts definitely OFF the
/// carrier is no verdict — the witness was built on the carrier, so
/// that answer contradicts its construction rather than placing it.
pub(crate) fn place_witness<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    surface: &geom::Surface<T>,
    p: Point3<T>,
    band: Band,
) -> Option<FaceContainment> {
    match *surface {
        geom::Surface::Plane { normal, .. } => contfp(body, face, normal, p, band).ok(),
        _ => match super::contain::curved_face_placement(body, face, p, band) {
            Ok(super::contain::CurvedPlacement::Trim(v)) => v,
            _ => None,
        },
    }
}

/// One face as the section certificate reads it: its key, its surface's
/// key and description, and its certified box.
pub(crate) struct FaceRow<T: Real> {
    face: FaceKey,
    key: SurfaceKey,
    surface: geom::Surface<T>,
    bbox: bvh::Aabb,
}

/// Every face of `body` as a [`FaceRow`], in face-arena order.
///
/// # Errors
///
/// [`BooleanError::ClassificationInvariant`] for a face whose surface
/// does not resolve, and the box builder's own errors.
pub(crate) fn face_rows<T: Decide + Bounds>(
    body: &Body<T>,
    band: Band,
) -> Result<Vec<FaceRow<T>>, BooleanError> {
    let pad = boxes::sweep_pad(band);
    body.faces()
        .map(|(face, fd)| {
            let surface = body
                .get_surface(fd.surface)
                .ok_or(BooleanError::ClassificationInvariant {
                    what: "section certificate: a face surface is lost",
                })?
                .clone();
            Ok(FaceRow {
                face,
                key: fd.surface,
                surface,
                bbox: boxes::face_box(body, face, pad, band)?,
            })
        })
        .collect()
}

/// **The section certificate's per-pair rule** for A's face `fa` (the
/// section's `F`) against B's face `fb` (its `G`), whose boxes overlap:
/// the pair's classified section and every component's witness, or the
/// pair's refusal. The reach — the ball about the two boxes' overlap,
/// which pivots and levers the angular margins — is built here
/// ([`super::section_cert`]'s module docs).
///
/// # Errors
///
/// [`BooleanError::ClassificationInvariant`] for a face whose loops do
/// not resolve.
fn pair_verdict<T: Decide + Bounds + crate::props::AtRestPolicy>(
    (a, fa): (&Body<T>, &FaceRow<T>),
    (b, fb): (&Body<T>, &FaceRow<T>),
    band: Band,
    evented: bool,
    charts: &mut ChartCache,
) -> Result<Result<Vec<super::section_cert::Cleared>, super::section_cert::Refusal>, BooleanError> {
    use super::section_cert::{Refusal, Side, certify, classify};
    if has_lone_vertex(a, fa.face)? || has_lone_vertex(b, fb.face)? {
        return Ok(Err(Refusal::LoneVertex));
    }
    let (box_a, box_b) = (&fa.bbox, &fb.bbox);
    let (lo, hi) = (
        Vec3::new(
            box_a.min_x.max(box_b.min_x),
            box_a.min_y.max(box_b.min_y),
            box_a.min_z.max(box_b.min_z),
        ),
        Vec3::new(
            box_a.max_x.min(box_b.max_x),
            box_a.max_y.min(box_b.max_y),
            box_a.max_z.min(box_b.max_z),
        ),
    );
    let reach = super::section_cert::Reach {
        centre: Point3::origin() + ((lo + hi) * 0.5).map(T::from_f64),
        radius: T::from_f64((hi - lo).norm() * 0.5),
    };
    let section = classify(&fa.surface, &fb.surface, reach, band);
    Ok(certify(
        &section,
        evented,
        |side| {
            let (operand, body, row) = match side {
                Side::F => (Operand::A, a, fa),
                Side::G => (Operand::B, b, fb),
            };
            charts.describes(operand, body, row.face, &row.surface, band)
        },
        |p| {
            [
                place_witness(a, fa.face, &fa.surface, p, band),
                place_witness(b, fb.face, &fb.surface, p, band),
            ]
        },
    ))
}

/// **The section certificate over pairs of rows**: every `(A row, B
/// row)` pair whose certified boxes overlap and which `admit` takes, in
/// the rows' order, each [`pair_verdict`]'s. `evented` says whether
/// the reduction recorded an event on the pair `(A face, B face)`;
/// `named` says whether A's face is the one a refusal names. With
/// `stop` the walk returns at the first refusing pair.
///
/// # Errors
///
/// [`pair_verdict`]'s.
#[allow(clippy::too_many_arguments)]
fn walk_pairs<'r, T: Decide + Bounds + crate::props::AtRestPolicy + 'r>(
    (a, a_rows): (&Body<T>, impl IntoIterator<Item = &'r FaceRow<T>>),
    (b, b_rows): (&Body<T>, impl IntoIterator<Item = &'r FaceRow<T>>),
    band: Band,
    charts: &mut ChartCache,
    admit: impl Fn(&FaceRow<T>, &FaceRow<T>) -> bool,
    evented: impl Fn(FaceKey, FaceKey) -> bool,
    named: impl Fn(&geom::Surface<T>) -> bool,
    stop: bool,
) -> Result<Vec<PairVerdict>, BooleanError> {
    let b_rows: Vec<&FaceRow<T>> = b_rows.into_iter().collect();
    let mut out = Vec::new();
    for fa in a_rows {
        for &fb in &b_rows {
            if !fa.bbox.overlaps(&fb.bbox) || !admit(fa, fb) {
                continue;
            }
            let verdict = pair_verdict((a, fa), (b, fb), band, evented(fa.face, fb.face), charts)?;
            let refused = verdict.is_err();
            out.push(PairVerdict {
                a_face: fa.face,
                a_kind: fa.surface.kind(),
                b_face: fb.face,
                b_kind: fb.surface.kind(),
                a_named: named(&fa.surface),
                verdict,
            });
            if stop && refused {
                return Ok(out);
            }
        }
    }
    Ok(out)
}

/// **The section certificate over every in-scope pair** of `a` × `b`
/// whose certified boxes overlap and which `skip` does not exempt, in
/// arena order ([`walk_pairs`]). `evented` says whether the reduction
/// recorded an event on the pair `(A face, B face)`. With `stop` the
/// scan returns at the first refusing pair. `chart_boundary` is asked
/// once per face and cached.
///
/// # Errors
///
/// [`BooleanError::ClassificationInvariant`] for a face whose surface
/// or loops do not resolve, and the box builder's own errors.
pub(crate) fn section_pairs<T: Decide + Bounds + crate::props::AtRestPolicy>(
    a: &Body<T>,
    b: &Body<T>,
    band: Band,
    path: SectionPath,
    skip: impl Fn(FaceKey, FaceKey) -> bool,
    evented: impl Fn(FaceKey, FaceKey) -> bool,
    stop: bool,
) -> Result<Vec<PairVerdict>, BooleanError> {
    let (a_rows, b_rows) = (face_rows(a, band)?, face_rows(b, band)?);
    walk_pairs(
        (a, &a_rows),
        (b, &b_rows),
        band,
        &mut ChartCache::default(),
        |fa, fb| path.scope(&fa.surface, &fb.surface) && !skip(fa.face, fb.face),
        evented,
        |s| path.names(s),
        stop,
    )
}

/// **The no-crossings path's section pass.** With no event anywhere,
/// W4 never fires and the no-event decision decides every lone
/// component, so each pair the vertex probe could not see into is
/// either certified or refused here, typed as the fallback's extent
/// refusal ([`BooleanError::FallbackExtentUnsupported`]) naming the
/// pair's curved face. Declarations exempt nothing on this path: with
/// no crossings, a declared coincident pair is exactly what the vertex
/// probe cannot decide.
fn section_extent_pass<T: Decide + Bounds + crate::props::AtRestPolicy>(
    a: &Body<T>,
    b: &Body<T>,
    band: Band,
) -> Result<(), BooleanError> {
    let pairs = section_pairs(
        a,
        b,
        band,
        SectionPath::Fallback,
        |_, _| false,
        |_, _| false,
        true,
    )?;
    match pairs
        .iter()
        .find_map(|p| p.verdict.as_ref().err().map(|r| (p, *r)))
    {
        None => Ok(()),
        Some((p, refusal)) => {
            let (operand, face, ..) = p.named();
            Err(BooleanError::FallbackExtentUnsupported {
                operand,
                face,
                what: refusal.what(),
            })
        }
    }
}

/// **The section certificate's per-pair report, for the verdict rows:**
/// the reduction `op` would run, and every in-scope pair's outcome on
/// the crossings path with that reduction's events.
///
/// # Errors
///
/// The reduction's own refusals, and [`section_pairs`]'.
#[cfg(any(test, feature = "test-support"))]
pub(crate) fn section_report<T: Decide + Bounds + crate::props::AtRestPolicy>(
    op: BooleanOp,
    a: &Body<T>,
    b: &Body<T>,
    tol: Tol,
) -> Result<Vec<PairVerdict>, BooleanError> {
    let band = Band::linear(tol)?;
    let decls = BooleanDeclarations::default();
    let red =
        super::boolean_reduce_declared_strategy(op, a, b, &decls, SweepStrategy::Realized, tol)?;
    let events = event_pairs(&red)?;
    section_pairs(
        a,
        b,
        band,
        SectionPath::Crossings,
        |_, _| false,
        |fa, fb| events.contains(&(fa, fb)),
        false,
    )
}

/// **The no-crossings path's two certificates**, run as the path runs
/// them before the vertex probe: the sphere extent scan, then — when
/// it asks for no re-cut — the section pass. `Ok` with the number of
/// re-cuts the scan asked for.
///
/// # Errors
///
/// Either certificate's refusal.
#[cfg(any(test, feature = "test-support"))]
pub(crate) fn no_crossings_certificates(
    a: &Body<f64>,
    b: &Body<f64>,
    tol: Tol,
) -> Result<usize, BooleanError> {
    let band = Band::linear(tol)?;
    let recuts = sphere_extent_scan(a, b, band)?;
    if recuts.is_empty() {
        section_extent_pass(a, b, band)?;
    }
    Ok(recuts.len())
}

/// **A ball against a plane's CARRIER: the one home of that gap.**
/// Decides `r − |s|` under `bool_sphere_extent_gap`, where `s` is the
/// centre's signed distance to the plane along its stored normal, which
/// the plane convention makes unit (and which is read as stored rather
/// than re-normalised, because at the interval scalar a division by the
/// normal's own norm widens `s` and moves the scan's escalations):
/// `Negative` is a ball definitely clear of the carrier, `Zero` a
/// tangency, `Positive` a ball the carrier cuts in a circle of radius
/// `√((r − |s|)(r + |s|))` about `centre − n̂·s`. Returns the sign and
/// `s`. Read by the no-crossings sphere scan ([`sphere_extent_scan`]).
fn ball_against_plane<T: Decide>(
    center: Point3<T>,
    radius: T,
    origin: Point3<T>,
    normal: Vec3<T>,
    band: Band,
) -> Result<(NonzeroSign, T), geom_core::Indeterminate> {
    let s = (center - origin).dot(normal);
    let sign = crate::validate::decide_nonzero_reported(
        "bool_sphere_extent_gap",
        Margin::of(radius - s.abs()),
        band,
    )?;
    Ok((sign, s))
}

/// The (A face, B face) pairs the reduction recorded an event on: a
/// vertex-on-face contact pairs every face the vertex bounds with the
/// face it lies on, and a vertex-on-vertex contact pairs every face
/// each vertex bounds.
///
/// "The faces a vertex bounds" are read at the contact point, not at
/// one key: the classification's null edges split a contact vertex into
/// copies at that point, and which copy keeps which face's corner is
/// the scaffolding's choice (the side each null edge's new vertex
/// took), so every copy the vertex's null edges reach, transitively (a
/// strut nested in another's segment hangs at its tip), is read with it.
fn event_pairs<T: Real>(
    red: &BooleanReduction<T>,
) -> Result<BTreeSet<(FaceKey, FaceKey)>, BooleanError> {
    let a_faces = faces_by_vertex(&red.a)?;
    let b_faces = faces_by_vertex(&red.b)?;
    let desc = Descendants::default().with_copies(Descendants::null_copies(&red.null_edges));
    let around = |operand: Operand, v: VertexKey| {
        let m = match operand {
            Operand::A => &a_faces,
            Operand::B => &b_faces,
        };
        let mut faces: Vec<FaceKey> = desc
            .copies_of(operand, v)
            .into_iter()
            .flat_map(|u| m.get(&u).cloned().unwrap_or_default())
            .collect();
        faces.sort();
        faces.dedup();
        faces
    };
    let mut out = BTreeSet::new();
    for c in &red.contacts.a_on_b {
        for fa in around(Operand::A, c.vertex) {
            out.insert((fa, c.face));
        }
    }
    for c in &red.contacts.b_on_a {
        for fb in around(Operand::B, c.vertex) {
            out.insert((c.face, fb));
        }
    }
    for c in &red.contacts.vv {
        for fa in around(Operand::A, c.a) {
            for fb in around(Operand::B, c.b) {
                out.insert((fa, fb));
            }
        }
    }
    Ok(out)
}

/// Every face each vertex bounds, from the faces' own loops.
fn faces_by_vertex<T: Real>(
    body: &Body<T>,
) -> Result<BTreeMap<VertexKey, Vec<FaceKey>>, BooleanError> {
    let corrupt = || BooleanError::JoinDesync {
        what: "interior-loop guard: a reduction operand is not walkable",
    };
    let mut out: BTreeMap<VertexKey, Vec<FaceKey>> = BTreeMap::new();
    for (face, fd) in body.faces() {
        for l in core::iter::once(fd.outer).chain(fd.rings.iter().copied()) {
            // A lone-vertex loop's vertex bounds the face as much as a
            // cycle's do.
            let vertices = match body.get_loop(l).ok_or_else(corrupt)?.boundary {
                LoopBoundary::Empty { vertex } => vec![vertex],
                LoopBoundary::Cycle { first } => body
                    .loop_cycle(first)
                    .ok_or_else(corrupt)?
                    .into_iter()
                    .map(|he| body.get_half_edge(he).map(|h| h.start).ok_or_else(corrupt))
                    .collect::<Result<Vec<_>, _>>()?,
            };
            for v in vertices {
                let faces = out.entry(v).or_default();
                if !faces.contains(&face) {
                    faces.push(face);
                }
            }
        }
    }
    Ok(out)
}

/// The graft map as sorted-order row vectors (naming emission).
type GraftRows = (
    Vec<(VertexKey, VertexKey)>,
    Vec<(EdgeKey, EdgeKey)>,
    Vec<(EdgeKey, EdgeKey)>,
    Vec<(FaceKey, FaceKey)>,
);

pub(super) fn graft_rows(g: &GraftMap) -> GraftRows {
    (
        g.vertices.iter().map(|(k, &v)| (k, v)).collect(),
        g.edges.iter().map(|(k, &v)| (k, v)).collect(),
        g.dead_edges.iter().map(|(&k, &v)| (k, v)).collect(),
        g.faces.iter().map(|(k, &v)| (k, v)).collect(),
    )
}

/// The merge outcome as naming rows.
pub(super) fn merge_rows(
    m: &crate::merge_faces::MergeCoplanarOutcome,
) -> Vec<(FaceKey, Vec<FaceKey>)> {
    m.groups
        .iter()
        .map(|g| (g.kept, g.absorbed.clone()))
        .collect()
}

/// The volume-inequality backstop at the op gate (PR 5 review): every
/// `Seamed` result must satisfy the set-theoretic bounds
/// vol(∩) ≤ min(vol A, vol B), max(vol A, vol B) ≤ vol(∪) ≤ vol A + vol B,
/// vol A − vol B ≤ vol(∖) ≤ vol A; and a bounded result must not
/// enclose negative volume, tier 3's +V invariant read by its own rule
/// (`crate::validate::plus_v_read`, in-band exempt). The
/// min/max are decomposed into per-operand inequalities. No inequality over these three volumes bounds vol(∩)
/// from below — that needs vol(∪), which the op does not compute — so a
/// short positive intersect is invisible here.
///
/// # What it measures with
///
/// The three bodies are measured through `lane`, the certified
/// quadrature, by the sign-level walk ([`crate::props::sign_walk`]) run
/// to its reporting target: a face trimmed by an ellipse or a spline is
/// enclosed by the quadrature, every other face takes its closed form,
/// and the enclosures are the ones [`crate::mass_properties`] reports.
/// The scalar's policy decides whether the gate runs at all
/// ([`crate::AtRestPolicy::gate_volume_backstop`]): a dual runs nothing
/// (DL3). A body the property layer cannot measure refuses
/// [`BooleanError::VolumeUnmeasured`], and one whose structure does not
/// resolve [`BooleanError::VolumeCorrupt`] — tier 3's reading of the
/// same refusals (`validate::classify_mass_props`). A bound that cannot
/// be evaluated is not a pass.
///
/// # Enclosures, and the resolution they leave
///
/// A quadrature volume is an enclosure `[volume_lo, volume_hi]`
/// ([`crate::MassProperties::enclosure`]), so each decision reads the
/// end that makes it conservative: an operand is bounded only when its
/// lower end is certified positive, and a bound is violated only when
/// the margin's UPPER end — the two enclosures at their least
/// favourable — is certified negative. Where the upper end is not
/// negative but the lower one is, the sign is open because of the
/// enclosures and not the bodies, and the gate REFINES before it
/// decides: both compared bodies' quadrature faces are taken one round
/// further at a time from the round each met its target at
/// ([`crate::props::PastTarget::refine`]), until the
/// sign is decided or every face reaches the last round every lane runs
/// (`geom_brep::props::quad::LAST_ROUND_EVERY_LANE_RUNS`).
///
/// There the rounds run out at a resolution: the last round's
/// half-width, measured on the oblique-capped rod (`r = 0.5`, cut 20° at
/// `z = 3.5`, `sweep/tests/reach_volume_backstop.rs`) as ≈ 2.2e-10 m³
/// of rule remainder whatever ε plus ≈ 0.55·ε m³ (7.7e-10 m³ at the
/// default ε, against a reporting half-width of 9.2e-7 m³), the
/// remainder growing with the body's size. What is still open is then
/// metered as a boundary displacement — the margin's lower end over
/// the two bodies' summed area — and decided against the model's own
/// band: inside it, the open range is
/// below the model's resolution and the bound is accepted as an
/// in-band margin is (below); certified beyond it, the measurement
/// cannot decide a question the model can tell apart, and the gate
/// refuses [`BooleanError::VolumeUndecided`]. On a body whose faces are
/// all closed-form both half-widths are exactly zero and every
/// decision is the closed form's, re-derived in interval arithmetic
/// before it refuses (below).
///
/// Comparison posture: each bound margin is classified through the
/// k_stats funnel (`decide` — the certified trilean against the op's
/// linear band, under this gate's own predicate name), the codebase's
/// only legal comparison (Q1). Only a CERTIFIED violating sign
/// refuses ([`BooleanError::ResultVolumeImplausible`]);
/// `Zero` and in-band indeterminate margins PASS. A POISONED margin
/// (NaN) still refuses loudly ([`BooleanError::Escalated`]) — poison
/// never passes a gate.
///
/// # Rounding
///
/// A closed-form face's flux and the walk's sum round at `f64` with no
/// pad, so a result congruent to an operand through another face order
/// can land ulps past it. A violation the walk's sums call is therefore
/// re-derived in interval arithmetic before it refuses
/// ([`crate::props::PastTarget::interval_volume`]); one the interval
/// margin does not certify stays open.
///
/// A declared coincidence the door glues inside the band also moves a
/// correct result's volume, by up to the band over the glued face, and
/// at a tight bound that refuses: a correct body refused, the safe
/// direction
/// (`work/reach/a-settled-declared-coincidence-crosses-a-tight-volume-bound.md`).
///
/// Complement operands: a reverted body's flux volume is NEGATIVE
/// (its true set volume is infinite — the A∖B ≡ A∩revert(B) oracle
/// route feeds such operands legitimately), so each bound applies
/// only when its reference operand's volume is certified POSITIVE
/// (bounded solid); against a complement the set bound is vacuous
/// and is skipped, never misread as a violation.
///
/// # Dimension (audit F3, `docs/predicate-dimension-audit.md`)
///
/// The bounds are statements about VOLUMES (m³) but ε is a point
/// deviation (D4), so each margin is metered to a LENGTH before it is
/// decided. Displacing a boundary by δ changes the volume it encloses
/// by ≈ δ·A, so a volume defect `ΔV` between two compared bodies is
/// explained by a boundary deviation of `ΔV / (A_got + A_bound)` — the
/// sum of the two bodies' surface areas is the whole boundary that
/// could have moved, and the quotient is the MEAN BOUNDARY DISPLACEMENT
/// the bound violation corresponds to. (Summed, not one body's area:
/// both boundaries are free to move, so the sum is the honest total
/// lever. It is also the larger denominator, hence the smaller margin —
/// the anti-refusal direction, which is why arm 1 below exists.) The
/// operand-boundedness question is about one body, so
/// `volume_backstop_operand` uses that body's own area (`V/A`) —
/// verbatim the `positive_volume` precedent in `crate::validate`'s
/// tier-3 check 7. Both quotients are exact zero when the volumes
/// agree exactly, so the gate's non-strict pass direction is unmoved.
///
/// ## Two questions, two bands (the #200 review's MAJ-1)
///
/// Metering ALONE would have weakened this gate, and the direction is
/// worth naming: `ΔV/(A_got + A_bound)` shrinks with the bodies' area,
/// so a localized wrong-component defect on a large body can meter
/// below ε even though the defect is macroscopic. Executed
/// (`tests/probe_f34_review.rs`): a wrongly-kept 3 mm cube on a
/// 2 m × 2 m × 0.1 m plate is ΔV = 2.7e-8 m³ against ~17.6 m² of
/// boundary — 1.53e-9 m metered, inside the default band, where the
/// raw-m³ comparand had refused decisively.
///
/// The resolution is that the backstop asks TWO different questions and
/// only one of them is about a magnitude:
///
/// 1. **Is the inequality violated?** `vol(A ∖ B) ≤ vol(A)` is an
///    inequality, so a SIGN-CERTAIN negative margin is a violated bound
///    — a dimension-free fact, true regardless of how many metres of
///    boundary the defect is smeared over, down to the enclosures'
///    resolution on a quadrature-measured pair (above). This arm
///    decides against the **exact (bit-hairline) band**, where
///    "certain" means "proven beyond the enclosure's own width": a
///    negative the walk's sums call, which the interval re-derivation
///    then certifies (a straddling enclosure escalates and falls
///    through to the open arm). No ε enters — this is a sign, not a
///    comparison against a length. Predicate:
///    `volume_backstop_violation`.
/// 2. **Is the violation above the model's own resolution?** Only for
///    the near-zero region arm 1 leaves open, and there ε *is* the
///    right scale — which is exactly where the metered mean
///    displacement belongs. Predicate: `volume_backstop`.
///
/// Both arms consume the SAME metered comparand. Dividing by a
/// certainly-positive lever cannot change a sign, so arm 1 certifies
/// precisely the fact it would have certified on the raw volume, while
/// the recorded margin stays a length and the K telemetry stays
/// dimensionally honest (the whole point of F3). The one gap is
/// arithmetic rather than semantic: a defect small enough that the
/// quotient underflows to exactly zero (`|ΔV| ≲ 1e-322 m³` at `f64`)
/// would lose its sign — far below any representable model.
///
/// Both gates decide through the k_stats funnel under those names.
pub(crate) fn volume_backstop<T: Decide>(
    op: BooleanOp,
    a: &Body<T>,
    b: &Body<T>,
    result: &Body<T>,
    band: Band,
    tol: Tol,
    lane: QuadLane<T>,
) -> Result<(), BooleanError> {
    // Each body to its reporting target, keeping the walk so a bound
    // the reporting enclosures leave open can be refined past it.
    let measure = |body, operand| {
        let faces = crate::query::all_faces(body);
        let (refused, certificate) = crate::props::sign_walk(
            body,
            &faces,
            band,
            tol,
            Some(lane),
            |_| None,
            |refused| refused,
        )
        .map_err(|source| props_refusal(operand, source))?;
        match refused {
            Some(source) => Err(props_refusal(operand, source)),
            None => Ok(certificate.past_target()),
        }
    };
    // The exact (bit-hairline) band for the sign arm below — the same
    // device the splitter's total order uses (`splitting::order`, audit
    // note N6): its open interior holds no representable `f64`, so at
    // `f64` a sign is certain iff the margin is not exactly zero, and at
    // the interval scalar an enclosure straddling the hairline escalates
    // honestly. That IS "proven beyond the enclosure's own width".
    let exact = crate::splitting::order::exact_band()?;
    let mut ca = measure(a, Some(Operand::A))?;
    let mut cb = measure(b, Some(Operand::B))?;
    let mut cr = measure(result, None)?;
    // A bound applies only against a certified-bounded operand
    // (positive flux volume, at its enclosure's lower end); complement
    // operands (certified negative) make it vacuous. Poison refuses.
    // Metered `V/A` — the operand's MEAN THICKNESS (fn docs). Surface
    // area is unsigned (props.rs), so the quotient keeps V's sign and
    // the complement arm is unchanged. An in-band quotient is an
    // operand thinner than the model's own resolution — no bound is
    // certifiable from it, skip.
    // The backstops live on the INVARIANT LANE (Ev's #213 layering
    // ruling): consistency inequalities between integral results are
    // outside the length seam by design — no door, bare T — and a
    // certified violation is a kernel invariant failure, not a
    // validity refusal.
    let bounded = |p: crate::MassProperties<T>| -> Result<bool, BooleanError> {
        let e = p.enclosure();
        match geom_core::k_stats::decide_invariant(
            "volume_backstop_operand",
            e.volume_lo / e.surface_area,
            band,
        ) {
            Ok(Sign::Positive) => Ok(true),
            Ok(Sign::Zero | Sign::Negative) => Ok(false),
            Err(diag) if diag.margin.is_invalid() => Err(BooleanError::Escalated {
                decision: BooleanDecision::VolumeBackstop,
                diag,
            }),
            Err(_) => Ok(false),
        }
    };
    let (ba, bb) = (bounded(ca.props())?, bounded(cb.props())?);
    let (a_, b_, r_) = (Some(Operand::A), Some(Operand::B), None);
    let bound = Posture::Bound { band, exact };
    let plus_v = Posture::PlusV { band };
    match op {
        BooleanOp::Intersect => {
            if ba {
                let (small, large) = (&mut [(r_, &mut cr)], &mut [(a_, &mut ca)]);
                bound_holds("vol(A ∩ B) ≤ vol(A)", small, large, bound)?;
            }
            if bb {
                let (small, large) = (&mut [(r_, &mut cr)], &mut [(b_, &mut cb)]);
                bound_holds("vol(A ∩ B) ≤ vol(B)", small, large, bound)?;
            }
            if ba || bb {
                bound_holds("vol(A ∩ B) ≥ 0", &mut [], &mut [(r_, &mut cr)], plus_v)?;
            }
        }
        BooleanOp::Union => {
            if ba {
                let (small, large) = (&mut [(a_, &mut ca)], &mut [(r_, &mut cr)]);
                bound_holds("vol(A ∪ B) ≥ vol(A)", small, large, bound)?;
            }
            if bb {
                let (small, large) = (&mut [(b_, &mut cb)], &mut [(r_, &mut cr)]);
                bound_holds("vol(A ∪ B) ≥ vol(B)", small, large, bound)?;
            }
            if ba && bb {
                let (small, large) = (&mut [(r_, &mut cr)], &mut [(a_, &mut ca), (b_, &mut cb)]);
                bound_holds("vol(A ∪ B) ≤ vol(A) + vol(B)", small, large, bound)?;
            }
        }
        BooleanOp::Subtract => {
            if ba {
                let (small, large) = (&mut [(r_, &mut cr)], &mut [(a_, &mut ca)]);
                bound_holds("vol(A ∖ B) ≤ vol(A)", small, large, bound)?;
                bound_holds("vol(A ∖ B) ≥ 0", &mut [], &mut [(r_, &mut cr)], plus_v)?;
            }
            if ba && bb {
                let (small, large) = (&mut [(a_, &mut ca)], &mut [(r_, &mut cr), (b_, &mut cb)]);
                bound_holds("vol(A ∖ B) ≥ vol(A) − vol(B)", small, large, bound)?;
            }
        }
    }
    Ok(())
}

/// The property layer's refusal, as the backstop reports it — in tier
/// 3's reading (`validate::classify_mass_props`): structure that does
/// not resolve is [`BooleanError::VolumeCorrupt`], anything else a body
/// the kernel has no measurement for, [`BooleanError::VolumeUnmeasured`].
fn props_refusal(operand: Option<Operand>, source: crate::MassPropsError) -> BooleanError {
    if crate::validate::classify_mass_props(&source).defect {
        BooleanError::VolumeCorrupt { operand, source }
    } else {
        BooleanError::VolumeUnmeasured { operand, source }
    }
}

/// One body of a backstop inequality: the operand a refusal of its
/// measurement names (`None` is the result), and the measurement.
type Term<'t, 'b, T> = (Option<Operand>, &'t mut crate::props::PastTarget<'b, T>);

/// Which side of `Σ small ≤ Σ large` a body's volume is summed on.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Side {
    Small,
    Large,
}

/// How a backstop inequality's margin is decided.
#[derive(Clone, Copy)]
enum Posture {
    /// A set bound ([`volume_backstop`]'s fn docs, "Two questions, two
    /// bands"): a violation certified at the `exact` band refuses
    /// whatever its size, an open sign is refined and then decided
    /// against `band`, and the magnitude arm reads `band`.
    Bound { band: Band, exact: Band },
    /// Tier 3's positive-volume invariant (`docs/DESIGN.md`, tier 3),
    /// read at the door by tier 3's own reading
    /// ([`crate::validate::plus_v_read`]): only a negative certified
    /// past the band refuses, and zero, in-band and open are exempt.
    PlusV { band: Band },
}

/// What a margin's two ends say, under a [`Posture`].
#[derive(Clone, Copy, PartialEq, Eq)]
enum Reading {
    /// Certified violated.
    Violated,
    /// Certified held.
    Held,
    /// Neither, at these ends.
    Open,
}

impl Posture {
    /// The margin `[lo, hi]` over `lever`, read at whatever scalar it is
    /// carried in. Unpadded, the two ends are one value and the upper
    /// end's sign is the margin's.
    fn read<U: Decide>(self, lo: U, hi: U, lever: U, padded: bool) -> Reading {
        match self {
            Posture::Bound { exact, .. } => {
                let sign = |end: U| {
                    geom_core::k_stats::decide_invariant(
                        "volume_backstop_violation",
                        end / lever,
                        exact,
                    )
                };
                let upper = sign(hi);
                if upper == Ok(Sign::Negative) {
                    Reading::Violated
                } else if !padded && upper.is_ok()
                    || padded && matches!(sign(lo), Ok(Sign::Zero | Sign::Positive))
                {
                    Reading::Held
                } else {
                    Reading::Open
                }
            }
            Posture::PlusV { band } => {
                let enclosure = crate::props::VolumeEnclosure {
                    volume_lo: lo,
                    volume_hi: hi,
                    surface_area: lever,
                };
                match crate::validate::plus_v_read(enclosure, band) {
                    Some(crate::props::ShellRole::Void) => Reading::Violated,
                    Some(crate::props::ShellRole::Outer) => Reading::Held,
                    None => Reading::Open,
                }
            }
        }
    }
}

/// A body's volume and area in interval arithmetic
/// ([`crate::props::PastTarget::interval_volume`]), a refusal naming
/// the body by `operand`.
fn interval_measure<T: Decide>(
    which: &'static str,
    operand: Option<Operand>,
    target: &crate::props::PastTarget<'_, T>,
) -> Result<(Interval, Interval), BooleanError> {
    match target.interval_volume() {
        Some(Ok(measured)) => Ok(measured),
        Some(Err(source)) => Err(props_refusal(operand, source)),
        None => Err(BooleanError::VolumeUndecided { which }),
    }
}

/// One inequality of [`volume_backstop`], `Σ small ≤ Σ large` over the
/// bodies' volumes, named by `which` and decided as `posture` says:
/// refuse a certified violation once the interval re-derivation
/// confirms it, refine while the enclosures alone keep the sign open,
/// and (a set bound) decide what the last round leaves against the
/// band, then the magnitude arm.
fn bound_holds<'t, 'b, T: Decide>(
    which: &'static str,
    small: &mut [Term<'t, 'b, T>],
    large: &mut [Term<'t, 'b, T>],
    posture: Posture,
) -> Result<(), BooleanError> {
    let escalated = |diag| BooleanError::Escalated {
        decision: BooleanDecision::VolumeBackstop,
        diag,
    };
    loop {
        // The margin `Σ large − Σ small` at its two ends, and the
        // summed surface area it is metered over (fn docs, audit F3).
        let (mut lo, mut hi, mut lever) = (T::zero(), T::zero(), T::zero());
        let mut padded = false;
        // The result's volume, and what the other bodies bound it by,
        // for the refusal's text.
        let (mut got, mut others) = (T::zero(), T::zero());
        let mut result_side = Side::Small;
        for (side, terms) in [(Side::Small, &*small), (Side::Large, &*large)] {
            for (operand, target) in terms {
                let p = target.props();
                let e = p.enclosure();
                padded |= p.volume_pad > 0.0;
                lever = lever + e.surface_area;
                (lo, hi) = match side {
                    Side::Small => (lo - e.volume_hi, hi - e.volume_lo),
                    Side::Large => (lo + e.volume_lo, hi + e.volume_hi),
                };
                match (operand, side) {
                    (None, _) => (got, result_side) = (p.volume, side),
                    (Some(_), Side::Small) => others = others - p.volume,
                    (Some(_), Side::Large) => others = others + p.volume,
                }
            }
        }
        let bound = match result_side {
            Side::Small => others,
            Side::Large => -others,
        };
        let implausible = || BooleanError::ResultVolumeImplausible {
            which,
            got: format!("{got:?}"),
            bound: format!("{bound:?}"),
        };
        let metered = hi / lever;
        let reading = posture.read(lo, hi, lever, padded);
        // Open: the enclosures keep the sign open.
        let mut open = padded && reading == Reading::Open;
        // Arm 1 — the inequality itself, at the margin's upper end. The
        // walk's own sums round (`PastTarget::interval_volume`), so a
        // violation they call is re-derived in interval arithmetic
        // before it refuses: one the interval margin does not certify
        // is the rounding's, and stays open.
        if reading == Reading::Violated {
            let (mut margin, mut area) = (Interval::zero(), Interval::zero());
            for (side, terms) in [(Side::Small, &*small), (Side::Large, &*large)] {
                for (operand, target) in terms {
                    let (v, a) = interval_measure(which, *operand, target)?;
                    margin = match side {
                        Side::Small => margin - v,
                        Side::Large => margin + v,
                    };
                    area = area + a;
                }
            }
            if posture.read(margin, margin, area, false) == Reading::Violated {
                return Err(implausible());
            }
            open = true;
            lo = T::from_f64(margin.lo());
        }
        if open {
            // Every body, not the first that moves.
            let mut moved = false;
            for (_, target) in small.iter_mut().chain(large.iter_mut()) {
                moved |= target.refine();
            }
            if moved {
                continue;
            }
        }
        let Posture::Bound { band, .. } = posture else {
            // The +V invariant exempts what is left open, as at rest.
            return Ok(());
        };
        // The rounds ran out with the sign open: what is left open,
        // metered, is either below the model's resolution or not.
        if open {
            match geom_core::k_stats::decide_invariant("volume_backstop", lo / lever, band) {
                Ok(Sign::Negative) => return Err(BooleanError::VolumeUndecided { which }),
                Err(diag) if diag.margin.is_invalid() => return Err(escalated(diag)),
                Ok(Sign::Zero | Sign::Positive) | Err(_) => {}
            }
        }
        // Arm 2 — the magnitude, for the near-zero region arm 1 leaves
        // open: only a certified negative refuses, Zero and in-band
        // PASS, poison refuses.
        return match geom_core::k_stats::decide_invariant("volume_backstop", metered, band) {
            Ok(Sign::Negative) => Err(implausible()),
            Ok(Sign::Zero | Sign::Positive) => Ok(()),
            Err(diag) if diag.margin.is_invalid() => Err(escalated(diag)),
            Err(_) => Ok(()),
        };
    }
}

/// D6 (M3 PR 6a): honest descriptions on boolean-minted edges AT MINT
/// TIME — the worklist is tracked lineage (the zips' surviving seam
/// edges plus every boundary edge of a merge-kept face, whose
/// adjacency the merge just rewrote), never a post-hoc scan of the
/// body. Each worklist edge that still resolves is described from its
/// two faces' surfaces (structural adjacency): definitely transverse ⇒
/// `Intersection` with the chord-midpoint witness; definitely smooth ⇒
/// the must-carry rule over the edge ([`seam_must_carry`]) — the
/// intrinsic `TangentIntersection` where the surfaces determine the
/// locus, else the conventional description (D2's split); escalation
/// refuses typed.
pub(super) fn describe_minted_edges<T: Decide + crate::props::AtRestPolicy>(
    body: &mut Body<T>,
    seam_edges: &[crate::entity::EdgeKey],
    merged: &crate::merge_faces::MergeCoplanarOutcome,
    band: Band,
    tol: Tol,
) -> Result<(), BooleanError> {
    let corrupt = || BooleanError::JoinDesync {
        what: "description worklist edge not walkable",
    };
    let mut worklist: Vec<crate::entity::EdgeKey> = Vec::new();
    for &e in seam_edges {
        if body.get_edge(e).is_some() {
            worklist.push(e); // merge may have consumed flush seam edges
        }
    }
    // Merge-KEPT faces' boundaries (adjacency rewritten by the glue)
    // AND SKIPPED groups' faces' boundaries (M4 PR 5 F1: the glue
    // those groups' classification anticipated did NOT happen, so
    // their in-plane cut edges may carry descriptions citing
    // no-longer-adjacent surfaces — they must be re-checked against
    // the ACTUAL adjacency below). A declared pair the door declined
    // (a non-planar carrier) enters this worklist through the same
    // field: its faces were left as the zip shipped them, and their
    // boundaries are re-checked here for the same reason.
    let group_faces = merged
        .groups
        .iter()
        .map(|g| g.kept)
        .chain(merged.skipped.iter().flat_map(|s| s.faces.iter().copied()));
    for f in group_faces {
        let Some(face) = body.get_face(f) else {
            continue;
        };
        for &lk in core::iter::once(&face.outer).chain(&face.rings) {
            let LoopBoundary::Cycle { first } = body.get_loop(lk).ok_or_else(corrupt)?.boundary
            else {
                continue;
            };
            for he in body.loop_cycle(first).ok_or_else(corrupt)? {
                worklist.push(body.get_half_edge(he).ok_or_else(corrupt)?.edge);
            }
        }
    }
    // Whether two faces both belong to one recorded merge skip: the
    // licensed cosurface pairs the merge stage ships unglued.
    let recorded_skip = |f1: Option<crate::entity::FaceKey>, f2: Option<crate::entity::FaceKey>| {
        let (Some(f1), Some(f2)) = (f1, f2) else {
            return false;
        };
        merged
            .skipped
            .iter()
            .any(|s| s.faces.contains(&f1) && s.faces.contains(&f2))
    };
    for edge in worklist {
        let edge_data = body.get_edge(edge).ok_or_else(corrupt)?.clone();
        let sides = crate::readback::edge_sides(body, edge).map_err(|_| corrupt())?;
        let (s1, s2) = sides.surfaces();
        let he_plus = sides.plus.half_edge;
        let start = body.get_half_edge(he_plus).ok_or_else(corrupt)?.start;
        let end = body.half_edge_end(he_plus).ok_or_else(corrupt)?;
        let p0 = *body
            .get_point(body.get_vertex(start).ok_or_else(corrupt)?.point)
            .ok_or_else(corrupt)?;
        let p1 = *body
            .get_point(body.get_vertex(end).ok_or_else(corrupt)?.point)
            .ok_or_else(corrupt)?;
        let (Some(surf1), Some(surf2)) = (body.get_surface(s1), body.get_surface(s2)) else {
            return Err(corrupt());
        };
        let existing = body
            .get_curve_geom(edge_data.curve)
            .and_then(crate::null::CurveGeom::certified)
            .cloned();
        let curved = existing.as_ref().is_some_and(|c| c.carrier().is_curved());
        let draft = geom_brep::IntersectionDraft::of(existing.as_ref(), p0, p1);
        let (witness, extent) = (draft.witness, draft.extent);
        match seam_class(surf1, surf2, witness, extent, band)? {
            geom_brep::DihedralClass::Transverse => {
                body.set_edge_curve(edge, draft.into_spec(s1, s2), tol)
                    .map_err(|_| BooleanError::JoinDesync {
                        what: "minted-edge description failed certification",
                    })?;
            }
            geom_brep::DihedralClass::Smooth => {
                // F1 (the declared-merge SKIP lane): a SURVIVING
                // smooth-adjacency edge whose existing
                // `Intersection`/`Seam` description no longer cites
                // its two adjacent faces' surfaces would violate D2
                // adjacency coherence at tier 3 — the glue its
                // description anticipated was skipped (or the merge
                // re-homed its neighbors, or the zip fused it between
                // new faces). Re-describe conventionally where the
                // surfaces under-determine the locus (D2's split).
                let stale = match body
                    .get_curve_geom(edge_data.curve)
                    .and_then(crate::null::CurveGeom::certified)
                    .ok_or_else(corrupt)?
                    .description()
                {
                    geom_brep::EdgeDescription::Intersection { s1: d1, s2: d2, .. }
                    | geom_brep::EdgeDescription::TangentIntersection { s1: d1, s2: d2, .. } => {
                        !Body::<T>::cites_pair((*d1, *d2), s1, s2)
                    }
                    // A chart image cites ONE adjacent surface (its
                    // residual chart); stale iff neither side is it
                    // (the attach-door adjacency rule, M6-3) — except
                    // a SEAM image, whose two sides are one surface by
                    // what a seam is.
                    geom_brep::EdgeDescription::Chart(c) if c.seam => {
                        !(c.surface == s1 && c.surface == s2)
                    }
                    geom_brep::EdgeDescription::Chart(c) => !(c.surface == s1 || c.surface == s2),
                    // A scaffold comes to rest here only between the two
                    // faces of a declared pair the merge stage could not
                    // glue and RECORDED (a curved group: DESIGN's
                    // "only a curved group's skip is recorded and
                    // shipped"); it is described where it rests. Any
                    // other scaffold stays one, and tier 3 refuses it.
                    geom_brep::EdgeDescription::Scaffold(_) => recorded_skip(
                        body.face_of_half_edge(edge_data.he_plus),
                        body.face_of_half_edge(edge_data.he_minus),
                    ),
                };
                // The D6 smooth ladder (M9-3): a definitely-smooth
                // seam descends one order through the must-carry rule
                // over the edge, at the stations tier 3's must-carry arm
                // re-reads (`seam_must_carry`).
                let mint_intrinsic = {
                    let c = existing.as_ref().ok_or_else(corrupt)?;
                    let (t0, t1) = c.params();
                    seam_must_carry(surf1, surf2, c.carrier(), t0, t1, extent, band)?
                };
                if mint_intrinsic {
                    // Mint the intrinsic tangency on the existing
                    // carrier (U2: today's taxonomy, 1:1 onto
                    // (surface, exact-lane pcurve)) — this also
                    // refreshes a stale citation, since the minted
                    // surfaces are the CURRENT adjacency.
                    let c = existing.as_ref().ok_or_else(corrupt)?;
                    let (t0, t1) = c.params();
                    let spec = geom_brep::EdgeCurveSpec {
                        description: geom_brep::EdgeDescriptionSpec::TangentIntersection {
                            s1,
                            s2,
                            witness,
                        },
                        carrier: c.carrier().clone(),
                        param_start: t0,
                        param_end: t1,
                    };
                    body.set_edge_curve(edge, spec, tol)
                        .map_err(|_| BooleanError::JoinDesync {
                            what: "tangent-seam description failed certification",
                        })?;
                } else if stale {
                    if curved {
                        // The conventional re-description for an arc
                        // the adjacent surfaces under-determine, on the
                        // UNCHANGED carrier (no silent geometric
                        // rewrite — only the description moves). The
                        // edge comes to REST here, so it is described
                        // where it rests — as an image in `s1`'s own
                        // chart (D3's transience fence) — and the arc
                        // pushforward that used to BE the description
                        // is recorded as the authority beside it.
                        // Carrier kinds with no conventional
                        // pushforward keep the typed refusal.
                        let c = existing.as_ref().ok_or_else(corrupt)?;
                        let (t0, t1) = c.params();
                        let Some(spec) =
                            geom_brep::EdgeCurveSpec::arc_of_circle(c.carrier().clone(), t0, t1)
                        else {
                            return Err(BooleanError::JoinDesync {
                                what: "stale CURVED smooth-seam description (no conventional \
                                       re-description lane exists for this carrier kind)",
                            });
                        };
                        body.set_edge_curve(edge, spec.at_rest_in_chart(s1, false), tol)
                            .map_err(|_| BooleanError::JoinDesync {
                                what: "stale arc description failed re-certification",
                            })?;
                    } else {
                        body.set_edge_curve(
                            edge,
                            geom_brep::EdgeCurveSpec::line_between(p0, p1)
                                .at_rest_in_chart(s1, false),
                            tol,
                        )
                        .map_err(|_| BooleanError::JoinDesync {
                            what: "stale in-plane description failed re-certification",
                        })?;
                    }
                }
            }
        }
    }
    Ok(())
}

/// **A seam edge of the result, as its re-description reads it**: the
/// dihedral of its two surfaces at `witness` over `extent`, whose arm
/// rung is the seam's own lever and whose reading is the seam's wedge.
pub(super) fn seam_class<T: Decide>(
    surf1: &geom::Surface<T>,
    surf2: &geom::Surface<T>,
    witness: Point3<T>,
    extent: T,
    band: Band,
) -> Result<geom_brep::DihedralClass, BooleanError> {
    geom_brep::classify_dihedral(surf1, surf2, witness, extent, band).map_err(|escalation| {
        BooleanError::of_lever(
            super::LeverArm::Seam,
            super::DeclarationRead::Moot,
            escalation,
        )
    })
}

/// **A smooth seam edge of the result, one order down**: whether its
/// two surfaces determine the locus along it, by the must-carry rule
/// over the edge ([`geom_brep::must_carry_over_edge`]), at the
/// stations tier 3's must-carry arm reads.
///
/// - jet-determinate ⇒ `true`: the intrinsic `TangentIntersection` is
///   demanded;
/// - under-determined ⇒ `false`: the conventional description is the
///   honest one (coplanar planes' exact-zero jet lands here, so a planar
///   split keeps its chord description);
/// - a station that reads the seam a corner ⇒ `false`: the seam is
///   smooth at its witness and transverse elsewhere, an edge tier 3
///   holds to neither description, so it keeps the conventional one;
/// - in band at a station ⇒ the typed escalation of the reading that
///   raised it: the seam's first-order arm or wedge, by rung, as
///   [`seam_class`] ends it, or its second-order bend
///   ([`BooleanDecision::SeamJet`]). Certifiable as neither, so never
///   folded into either description (D4 ¶3).
pub(super) fn seam_must_carry<T: Decide>(
    surf1: &geom::Surface<T>,
    surf2: &geom::Surface<T>,
    carrier: &geom::Curve3<T>,
    t0: T,
    t1: T,
    extent: T,
    band: Band,
) -> Result<bool, BooleanError> {
    use geom_brep::{MustCarryEscalation, MustCarryVerdict};
    match geom_brep::must_carry_over_edge(surf1, surf2, carrier, t0, t1, extent, band) {
        MustCarryVerdict::JetDeterminate => Ok(true),
        MustCarryVerdict::UnderDetermined | MustCarryVerdict::Transverse => Ok(false),
        MustCarryVerdict::InBand(MustCarryEscalation::FirstOrder(escalation)) => {
            Err(BooleanError::of_lever(
                super::LeverArm::Seam,
                super::DeclarationRead::Moot,
                escalation,
            ))
        }
        MustCarryVerdict::InBand(MustCarryEscalation::SecondOrder(diag)) => {
            Err(BooleanError::Escalated {
                decision: BooleanDecision::SeamJet,
                diag,
            })
        }
    }
}

/// How one operand's keys map into the result body.
#[derive(Clone, Copy)]
pub(super) enum KeyView<'a> {
    /// Keys carried through unchanged (carve preserves keys).
    Direct,
    /// Keys bridged by the combine door's graft.
    Graft(&'a GraftMap),
    /// The operand is not part of the result.
    Absent,
}

impl KeyView<'_> {
    fn vertex(&self, v: VertexKey) -> Option<VertexKey> {
        match self {
            Self::Direct => Some(v),
            Self::Graft(g) => g.vertices.get(v).copied(),
            Self::Absent => None,
        }
    }

    fn face(&self, f: FaceKey) -> Option<FaceKey> {
        match self {
            Self::Direct => Some(f),
            Self::Graft(g) => g.faces.get(f).copied(),
            Self::Absent => None,
        }
    }

    fn edge(&self, e: EdgeKey) -> Option<EdgeKey> {
        match self {
            Self::Direct => Some(e),
            Self::Graft(g) => g.edges.get(e).copied(),
            Self::Absent => None,
        }
    }
}

/// The D5 descendant map (M3 PR 6a, PR 5 review R5): result-stage
/// entity replacement — pinch-weld and seam-zip vertex fusions and
/// `merge_coplanar_faces` face absorption — as old key → surviving
/// key rows, extending the graft's key lineage so a contact record
/// drops ONLY when its coincidence is consumed (entity gone, not
/// renamed). Re-derivation at the 3′ gate is rejected as
/// scan-to-bless (F1); the descendants ARE the mint-time knowledge.
#[derive(Default)]
pub(super) struct Descendants {
    /// Each operand's pinch-weld fusions, in its clone keys: read
    /// before its key view. A weld fuses ring vertices minted after the
    /// contacts were recorded, so no record cites a weld's keys: no rest
    /// is consumed by a weld (`fused` holds only the zips'), and these
    /// rows chase only a record a producer mints in clone keys.
    a_welds: Vec<(VertexKey, VertexKey)>,
    b_welds: Vec<(VertexKey, VertexKey)>,
    /// The zips' fusions in mint order, result keys.
    vertices: Vec<(VertexKey, VertexKey)>,
    /// Merge absorption, absorbed face → the group's kept face: an
    /// acyclic relation, since a kept face is never absorbed. A cycle
    /// is a corrupt record, and [`Self::live_face`] refuses it typed.
    faces: std::collections::BTreeMap<FaceKey, FaceKey>,
    /// Every vertex that participated in a zip fusion (dead OR kept):
    /// its point rests were consumed into seam structure.
    fused: std::collections::BTreeSet<VertexKey>,
    /// Each operand's null-edge copies, in its clone keys: the vertices
    /// one null edge joins, on one point by construction.
    copies: [Vec<(VertexKey, VertexKey)>; 2],
}

impl Descendants {
    /// The map that starts from each operand's pinch welds.
    pub(super) fn welded(a: &[(VertexKey, VertexKey)], b: &[(VertexKey, VertexKey)]) -> Self {
        Self {
            a_welds: a.to_vec(),
            b_welds: b.to_vec(),
            ..Self::default()
        }
    }

    /// The two ends of each of the reduction's null edges, per operand.
    pub(super) fn null_copies<T: Real>(
        null_edges: &[super::BoolNullEdgeRecord<T>],
    ) -> [Vec<(VertexKey, VertexKey)>; 2] {
        let mut copies = [Vec::new(), Vec::new()];
        for r in null_edges {
            copies[usize::from(r.operand == Operand::B)].push((r.attr.below_end, r.attr.above_end));
        }
        copies
    }

    /// The map that also reaches each v-v group's null-edge copies
    /// ([`remap_contacts`]).
    pub(super) fn with_copies(self, copies: [Vec<(VertexKey, VertexKey)>; 2]) -> Self {
        Self { copies, ..self }
    }

    /// `v` and every vertex null edges join it to, transitively, in
    /// `side`'s clone keys.
    fn copies_of(&self, side: Operand, v: VertexKey) -> Vec<VertexKey> {
        let rows = &self.copies[usize::from(side == Operand::B)];
        let mut out = vec![v];
        let mut i = 0;
        while let Some(&at) = out.get(i) {
            for &(x, y) in rows {
                let other = if x == at {
                    y
                } else if y == at {
                    x
                } else {
                    continue;
                };
                if !out.contains(&other) {
                    out.push(other);
                }
            }
            i += 1;
        }
        out
    }

    pub(super) fn absorb_zip(&mut self, rep: &super::zip::ZipReport) {
        for &(dead, kept) in &rep.vertex_merges {
            self.vertices.push((dead, kept));
            self.fused.insert(dead);
            self.fused.insert(kept);
        }
    }

    pub(super) fn absorb_merge(&mut self, merged: &crate::merge_faces::MergeCoplanarOutcome) {
        for group in &merged.groups {
            for &absorbed in &group.absorbed {
                self.faces.insert(absorbed, group.kept);
            }
        }
    }

    /// Operand `side`'s vertex `v`, through its pinch welds, the key
    /// `view`, and the zips' fusions, if it is live.
    ///
    /// # Errors
    ///
    /// [`BooleanError::JoinDesync`] on a corrupt fusion list
    /// ([`survivor_checked`]): it would chase `v` onto a dead key and
    /// drop the record as consumed.
    fn live_vertex<T: Real>(
        &self,
        body: &Body<T>,
        (side, view): (Operand, &KeyView<'_>),
        v: VertexKey,
    ) -> Result<Option<VertexKey>, BooleanError> {
        let welds = match side {
            Operand::A => &self.a_welds,
            Operand::B => &self.b_welds,
        };
        let Some(k) = view.vertex(survivor_checked(welds, v)?) else {
            return Ok(None);
        };
        let k = survivor_checked(&self.vertices, k)?;
        Ok(body.get_vertex(k).map(|_| k))
    }

    /// Chases a face key through the absorption rows until live:
    /// `None` when the chain ends at a dead key with no row (the face
    /// was consumed).
    ///
    /// # Errors
    ///
    /// [`BooleanError::JoinDesync`] when the rows cycle: a walk that
    /// outlasts the row count has revisited a key, a corrupt record,
    /// and reading it as consumed would drop a declared contact.
    fn live_face<T: Real>(
        &self,
        body: &Body<T>,
        f: FaceKey,
    ) -> Result<Option<FaceKey>, BooleanError> {
        let live = |k| body.get_face(k).is_some();
        let end = super::discard::lineage_root(f, self.faces.len(), |k| {
            if live(k) {
                None
            } else {
                self.faces.get(&k).copied()
            }
        })
        .ok_or(BooleanError::JoinDesync {
            what: "a face's absorption rows are cyclic",
        })?;
        Ok(live(end).then_some(end))
    }
}

/// Remaps the declared contacts into result keys — operand views
/// first (graft lineage), then the D5 descendant chase — dropping
/// records only when the entity is genuinely consumed (module docs).
///
/// **v-v rows are remapped as groups.** Rows that name a common key
/// on the same side (an A vertex or a B vertex in two rows) are one
/// group, closed transitively, and every two distinct live vertices
/// the group's ends and their null-edge copies map to are recorded,
/// though no single row named that pair, two A vertices included. A
/// copy is minted on its vertex's point, and where two crossing pairs
/// cut one vertex the pieces the result keeps there are copies no row
/// names. Whatever the pair, both its
/// vertices sit at the point the reduction coincided the shared key
/// with each of them. The inference reads keys and never positions:
/// it records what the reduction's own coincidences imply, and no
/// pair the census sees at one point is blessed for being there. A
/// group whose ends map to one live vertex records nothing, since a
/// pair fused into one vertex is structure now. A lone row maps as
/// its two ends.
///
/// # Errors
///
/// [`BooleanError::JoinDesync`] on cycling absorption rows
/// ([`Descendants::live_face`]).
pub(super) fn remap_contacts<T: Real>(
    body: &Body<T>,
    contacts: &ContactRecords,
    a_view: KeyView<'_>,
    b_view: KeyView<'_>,
    desc: &Descendants,
) -> Result<ContactRecords, BooleanError> {
    // v-v ends chase through zip fusions (a fused vertex's partner
    // may still coincide with the survivor); the group rule is in the
    // doc above.
    let vert = |side: (Operand, &KeyView<'_>), v: VertexKey| desc.live_vertex(body, side, v);
    // v-on-f VERTICES deliberately do NOT chase, and any vertex that
    // took part in a zip fusion (either side of a kev) drops its
    // rests: a fused vertex IS a seam vertex — the point rest was
    // consumed into structure (it now sits on the pierced face's cut
    // boundary), and carrying the record forward would declare a
    // contact the census sees as boundary incidence (stale). FACES
    // chase: merge absorption renames the face while the rest
    // persists (the R5 bug class this map exists for).
    let vert_strict = |view: &KeyView<'_>, v: VertexKey| {
        let k = view.vertex(v)?;
        if desc.fused.contains(&k) {
            return None;
        }
        body.get_vertex(k).map(|_| k)
    };
    let face =
        |view: &KeyView<'_>, f: FaceKey| view.face(f).map_or(Ok(None), |k| desc.live_face(body, k));
    let mut out = ContactRecords::default();
    // The groups (doc above): a vertex coincident with two of the
    // other operand's fuses into one and keeps touching the other,
    // whose row names the end that fused away.
    let mut group: Vec<usize> = (0..contacts.vv.len()).collect();
    for i in 0..group.len() {
        for j in 0..i {
            let (ci, cj) = (contacts.vv[i], contacts.vv[j]);
            let (gi, gj) = (group[i], group[j]);
            if (ci.a == cj.a || ci.b == cj.b) && gi != gj {
                group.iter_mut().filter(|g| **g == gi).for_each(|g| *g = gj);
            }
        }
    }
    let mut live: Vec<(usize, VertexKey)> = Vec::new();
    for (c, &g) in contacts.vv.iter().zip(&group) {
        for (side, view, end) in [(Operand::A, &a_view, c.a), (Operand::B, &b_view, c.b)] {
            for k in desc.copies_of(side, end) {
                if let Some(v) = vert((side, view), k)?
                    && !live.contains(&(g, v))
                {
                    live.push((g, v));
                }
            }
        }
    }
    for (i, &(g, a)) in live.iter().enumerate() {
        for &(_, b) in live[i + 1..].iter().filter(|(h, _)| *h == g) {
            // Two rows whose ends fused into one pair are one record.
            if !out
                .vv
                .iter()
                .any(|r| (r.a, r.b) == (a, b) || (r.a, r.b) == (b, a))
            {
                out.vv.push(VvContact { a, b });
            }
        }
    }
    for c in &contacts.a_on_b {
        if let (Some(vertex), Some(face)) = (vert_strict(&a_view, c.vertex), face(&b_view, c.face)?)
        {
            out.a_on_b.push(VfContact { vertex, face });
        }
    }
    for c in &contacts.b_on_a {
        if let (Some(vertex), Some(face)) = (vert_strict(&b_view, c.vertex), face(&a_view, c.face)?)
        {
            out.b_on_a.push(VfContact { vertex, face });
        }
    }
    // The curved granularities carry by FACE lineage — the descendant
    // map, never re-derivation (C4's replay rule): merge absorption
    // renames a face while the contact persists, which is exactly the
    // rename the chase exists to follow. The WITNESS edge does not
    // chase, because no edge descendant map exists: an edge dissolved
    // by the zip is genuinely consumed, so its curve record drops
    // under the same strict rule as a fused vertex's rests. Inventing
    // an edge chase here would be a second lineage source of truth.
    //
    // The witness is looked up through the A-SIDE view, which is the
    // convention and not an oversight: a `CurveContact`'s locus is a
    // seam edge of the RESULT, and the result arena is A's clone
    // (carve/clone preserve A's keys), so the A view is the identity
    // map for exactly the edges that can carry one. A B-side witness
    // would have to be grafted first and does not arise while nothing
    // mints these records; when a producer lands it must mint the
    // witness in result keys, and this convention is what it has to
    // meet.
    let live_edge = |view: &KeyView<'_>, e: EdgeKey| {
        let k = view.edge(e)?;
        body.get_edge(k).map(|_| k)
    };
    for c in &contacts.curves {
        if let (Some(face_a), Some(face_b), Some(witness)) = (
            face(&a_view, c.face_a)?,
            face(&b_view, c.face_b)?,
            live_edge(&a_view, c.witness),
        ) {
            out.curves.push(CurveContact {
                face_a,
                face_b,
                witness,
            });
        }
    }
    for c in &contacts.patches {
        if let (Some(face_a), Some(face_b)) = (face(&a_view, c.face_a)?, face(&b_view, c.face_b)?) {
            out.patches.push(PatchContact { face_a, face_b });
        }
    }
    Ok(out)
}

/// The declared face pairs lowered to SURVIVING result SURFACE pairs
/// (M4 PR 5): the equivalence rides surfaces because fragments
/// inherit their parent's surface key — face-key churn (a declared
/// face whose original key died with a discarded fragment) cannot
/// strand the intent as long as any fragment keeps the surface
/// alive. A pair with a consumed side (surface gone from the result)
/// licenses nothing and drops — its contact material did not survive
/// the op (the same consumed-record rule as contact rows);
/// resolution-level dangling was already refused at the door
/// (`validate_declarations`).
pub(super) fn declared_surface_pairs<T: Real>(
    result: &Body<T>,
    a: &Body<T>,
    b: &Body<T>,
    decls: &BooleanDeclarations,
    graft: &GraftMap,
) -> Vec<(SurfaceKey, SurfaceKey)> {
    decls
        .coincident_faces
        .iter()
        .filter_map(
            |&FacePairDeclaration {
                 a: fa,
                 b: fb,
                 class,
             }| {
                // A one-carrier declaration (`Rest` or a continuation)
                // licenses a merge-stage coincidence; a `Tangent` or
                // seam pair's carriers are DISTINCT by its own
                // verification and never merge.
                if !class.is_one_carrier() {
                    return None;
                }
                // A-clone surface keys ARE result keys (carve/clone
                // preserve them); B bridges through the graft.
                let ka = a.get_face(fa)?.surface;
                let kb = graft.surfaces.get(b.get_face(fb)?.surface).copied()?;
                (result.get_surface(ka).is_some() && result.get_surface(kb).is_some() && ka != kb)
                    .then_some((ka, kb))
            },
        )
        .collect()
}

/// Appends the operand-internal CARRIED contacts (F5) to the result
/// records, remapped through the operand views and the descendant
/// chase under the same strict drop rules as discovered records
/// ([`remap_contacts`]); duplicates of already-present rows are not
/// re-added. Carried A rows land in `vv`/`a_on_b`, carried B rows in
/// `vv`/`b_on_a` (the census flattens the split; the fields record
/// which lineage carried the row).
///
/// # Errors
///
/// As [`remap_contacts`].
pub(super) fn remap_carried<T: Real>(
    out: &mut ContactRecords,
    body: &Body<T>,
    decls: &BooleanDeclarations,
    a_view: &KeyView<'_>,
    b_view: &KeyView<'_>,
    desc: &Descendants,
) -> Result<(), BooleanError> {
    let vert = |side: (Operand, &KeyView<'_>), v: VertexKey| desc.live_vertex(body, side, v);
    let vert_strict = |view: &KeyView<'_>, v: VertexKey| {
        let k = view.vertex(v)?;
        if desc.fused.contains(&k) {
            return None;
        }
        body.get_vertex(k).map(|_| k)
    };
    let face =
        |view: &KeyView<'_>, f: FaceKey| view.face(f).map_or(Ok(None), |k| desc.live_face(body, k));
    let push_vv = |out: &mut ContactRecords, carried: &CarriedContacts, side| {
        for c in &carried.vv {
            if let (Some(a), Some(b)) = (vert(side, c.pair.a)?, vert(side, c.pair.b)?)
                && a != b
                && !out
                    .vv
                    .iter()
                    .any(|r| (r.a, r.b) == (a, b) || (r.a, r.b) == (b, a))
            {
                out.vv.push(VvContact { a, b });
            }
        }
        Ok::<_, BooleanError>(())
    };
    push_vv(out, &decls.carried_a, (Operand::A, a_view))?;
    push_vv(out, &decls.carried_b, (Operand::B, b_view))?;
    let dup_vf = |out: &ContactRecords, v: VertexKey, f: FaceKey| {
        out.a_on_b
            .iter()
            .chain(&out.b_on_a)
            .any(|r| (r.vertex, r.face) == (v, f))
    };
    for c in &decls.carried_a.vf {
        if let (Some(vertex), Some(fk)) = (
            vert_strict(a_view, c.rest.vertex),
            face(a_view, c.rest.face)?,
        ) && !dup_vf(out, vertex, fk)
        {
            out.a_on_b.push(VfContact { vertex, face: fk });
        }
    }
    for c in &decls.carried_b.vf {
        if let (Some(vertex), Some(fk)) = (
            vert_strict(b_view, c.rest.vertex),
            face(b_view, c.rest.face)?,
        ) && !dup_vf(out, vertex, fk)
        {
            out.b_on_a.push(VfContact { vertex, face: fk });
        }
    }
    Ok(())
}

/// **The result gate every [`BooleanBody`] passes**, at the site that
/// built it and before the volume backstop reads it: the body is
/// sorted one solid per piece of material ([`crate::pieces`]), then
/// finished ([`AtRestPolicy::gate_at_rest_kept`]: tier 3, whose first
/// act is tiers 1 and 2), so the verdict rides the result and is taken
/// on the bits the caller receives.
///
/// # Errors
///
/// [`BooleanError::Pieces`] where the body's pieces cannot be read;
/// [`BooleanError::ResultInvalid`] carrying the validator's findings.
pub(super) fn gate<T: Decide + Bounds + AtRestPolicy>(
    body: Body<T>,
    band: Band,
    tol: Tol,
) -> Result<AtRestBody<T>, BooleanError> {
    let mut body = body;
    let pad = super::boxes::sweep_pad(band);
    let face_box = |body: &Body<T>, f| super::boxes::face_box(body, f, pad, band).ok();
    crate::pieces::sort_into_pieces(&mut body, band, tol, T::quad_lane(), Some(&face_box))
        .map_err(BooleanError::Pieces)?;
    #[cfg(feature = "door-tier3-meter")]
    let from = std::time::Instant::now();
    let kept = T::gate_at_rest_kept(body, tol);
    #[cfg(feature = "door-tier3-meter")]
    super::door_meter::note_gate(super::door_meter::GateReading::of(
        from.elapsed(),
        kept.as_ref().map(|_| ()).map_err(Vec::as_slice),
    ));
    kept.map_err(|errors| BooleanError::ResultInvalid { errors })
}

/// One sphere group the extent scan wants re-cut: rigidly re-charted
/// about `align` (the first escape plane's normal) so its seam
/// meridians run pole-to-pole TRANSVERSE to the escape planes and the
/// ordinary crossing layer sees the section circles.
struct SphereRecut<T: Real> {
    /// The operand whose group escapes.
    operand: Operand,
    /// The group's representative face (its shell is what rotates).
    representative: FaceKey,
    /// The sphere's center — the rotation's fixed point.
    center: Point3<T>,
    /// Its radius (the alignment trilean's lever arm).
    radius: T,
    /// The stored polar axis (rotation source direction).
    axis: Vec3<T>,
    /// The escape plane's normal (rotation target direction).
    align: Vec3<T>,
}

/// The curved-EXTENT scan (fn-level story on the call site in
/// [`boolean_op_recut`]). Sound because every CURVED-involved boundary
/// pair is either **certified disjoint** (so a connected shell shares
/// its witness vertex's side — the vertex probe's missing
/// certificate), **an escape** (re-cut and re-entered), or **refused
/// typed**. Plane-only configurations run zero new predicates.
///
/// Two classes, and they get different answers because different
/// evidence exists for them:
///
/// - **Sphere**: a closed group's true extent is `center ± r`, so the
///   pairs it can meet are enumerable exactly, and an escape through a
///   plane face is repairable by a re-chart. Two spheres whose carriers
///   cross are asked whether their FACES meet ([`sphere_faces_apart`]).
/// - **Torus, cylinder and cone**: no closed-group extent exists, so
///   their pairs are certified per pair by the section certificate
///   ([`section_extent_pass`]), which runs after this scan. A sphere's
///   pairs with a cylinder, torus or cone face are the pass's too: none
///   is an escape face, so disjointness is all the scan would ask of
///   them.
///
/// Determinism (D9): face-arena order throughout; the first escape's
/// normal is the alignment target.
fn sphere_extent_scan<T: Decide + Bounds + crate::props::AtRestPolicy>(
    a: &Body<T>,
    b: &Body<T>,
    band: Band,
) -> Result<Vec<SphereRecut<T>>, BooleanError> {
    let esc = |question| {
        move |diag| BooleanError::Escalated {
            decision: BooleanDecision::Sphere(question),
            diag,
        }
    };
    // The NURBS re-gate (M5 S13, pinned): ANY fallback entry with a
    // NURBS face refuses before a vertex is probed — the extent test
    // is unwritable for the kind (variant docs).
    for (operand, body) in [(Operand::A, a), (Operand::B, b)] {
        for (face, fd) in body.faces() {
            if matches!(body.get_surface(fd.surface), Some(geom::Surface::Nurbs(_))) {
                return Err(BooleanError::NurbsExtentUnsupported { operand, face });
            }
        }
    }
    let pad = boxes::sweep_pad(band);
    let mut section_charts = ChartCache::default();
    let rows = [face_rows(a, band)?, face_rows(b, band)?];
    let mut out: Vec<SphereRecut<T>> = Vec::new();
    for (x_is, x, x_rows, y, y_rows) in [
        (Operand::A, a, &rows[0], b, &rows[1]),
        (Operand::B, b, &rows[1], a, &rows[0]),
    ] {
        // The scope is the whole operand: what the escape arm re-charts
        // is the sphere itself, which every wearer shares.
        let charts = crate::chart_groups::ChartGroups::of_body(x);
        let mut seen: Vec<SurfaceKey> = Vec::new();
        for (face, fd) in x.faces() {
            let Some(&geom::Surface::Sphere {
                center,
                radius,
                axis,
                ..
            }) = x.get_surface(fd.surface)
            else {
                continue;
            };
            if seen.contains(&fd.surface) {
                continue;
            }
            seen.push(fd.surface);
            // **Closedness is asked where it is USED, not on arrival.**
            // The certificate this scan spends is `center ± r`, and for
            // a TRIMMED group that box over-claims — which is the sound
            // direction for every SEPARATION test below, because a
            // trimmed face is a subset of the sphere and a box that
            // proves the whole sphere clear proves the subset clear.
            // What genuinely needs the closed group is the plane arm's
            // ESCAPE conclusion: it reasons about the whole section
            // circle of the sphere CARRIER, and it hands the result to
            // a re-chart that rotates the group about its own centre —
            // neither statement survives trimming. So the refusal lives
            // at that conclusion, and a trimmed group whose extent
            // clears everything gets its answer like any other.
            let group = closed_sphere_group(x, face, &charts);
            let ball_box = bvh::Aabb {
                min_x: center.x.lo() - radius.hi(),
                min_y: center.y.lo() - radius.hi(),
                min_z: center.z.lo() - radius.hi(),
                max_x: center.x.hi() + radius.hi(),
                max_y: center.y.hi() + radius.hi(),
                max_z: center.z.hi() + radius.hi(),
            }
            .padded(pad);
            let mut escape_normals: Vec<Vec3<T>> = Vec::new();
            for (y_row, (yf, yfd)) in y_rows.iter().zip(y.faces()) {
                if y_row.face != yf {
                    return Err(BooleanError::ClassificationInvariant {
                        what: "extent scan: face rows out of arena order",
                    });
                }
                match &y_row.surface {
                    &geom::Surface::Plane {
                        origin,
                        normal,
                        u_ref,
                    } => {
                        // A tangency (a decided zero) is a touching
                        // configuration the crossing layer cannot
                        // represent: it refuses with its decided margin,
                        // as its in-band twin does.
                        let (side, s) = ball_against_plane(center, radius, origin, normal, band)
                            .map_err(esc(SphereQuestion::AgainstPlane))?;
                        match side {
                            // Clear of the whole carrier plane.
                            NonzeroSign::Negative => {}
                            // A TRIMMED group's faces may never reach
                            // the circle the carrier cuts: certified
                            // apart from this face, they pose no escape
                            // through it.
                            NonzeroSign::Positive
                                if group.is_none()
                                    && sphere_faces_apart(
                                        (x_is, x, x_rows),
                                        fd.surface,
                                        (y, y_row),
                                        band,
                                        &mut section_charts,
                                    )?
                                    .is_none() => {}
                            NonzeroSign::Positive => {
                                // The sphere definitely crosses the
                                // CARRIER in a circle; classify the
                                // circle against the FACE. Certified
                                // enclosure first: a circle box clear
                                // of every boundary-edge box cannot
                                // cross the boundary, so one exact
                                // witness extends to the whole circle.
                                let foot = center - normal * s;
                                let rho = ((radius - s.abs()) * (radius + s.abs())).sqrt();
                                let circle_box = bvh::Aabb {
                                    min_x: foot.x.lo() - rho.hi(),
                                    min_y: foot.y.lo() - rho.hi(),
                                    min_z: foot.z.lo() - rho.hi(),
                                    max_x: foot.x.hi() + rho.hi(),
                                    max_y: foot.y.hi() + rho.hi(),
                                    max_z: foot.z.hi() + rho.hi(),
                                }
                                .padded(pad);
                                let mut near_boundary = false;
                                let mut walk = |lk| -> Result<(), BooleanError> {
                                    let l = y.get_loop(lk).ok_or(
                                        BooleanError::ClassificationInvariant {
                                            what: "extent scan: face loop lost",
                                        },
                                    )?;
                                    let LoopBoundary::Cycle { first } = l.boundary else {
                                        return Ok(());
                                    };
                                    for he in y.loop_cycle(first).ok_or(
                                        BooleanError::ClassificationInvariant {
                                            what: "extent scan: unwalkable loop",
                                        },
                                    )? {
                                        let ek = y
                                            .get_half_edge(he)
                                            .ok_or(BooleanError::ClassificationInvariant {
                                                what: "extent scan: half-edge lost",
                                            })?
                                            .edge;
                                        if boxes::edge_box(y, ek, pad)?.overlaps(&circle_box) {
                                            near_boundary = true;
                                        }
                                    }
                                    Ok(())
                                };
                                walk(yfd.outer)?;
                                for &ring in &yfd.rings {
                                    walk(ring)?;
                                }
                                if near_boundary {
                                    return Err(BooleanError::FallbackExtentUnsupported {
                                        operand: x_is,
                                        face,
                                        what: "the sphere's section circle runs near the \
                                               plane face's boundary — whole-circle \
                                               membership cannot be certified from the \
                                               enclosures, and no crossing layer saw an \
                                               event",
                                    });
                                }
                                let witness = foot + u_ref * rho;
                                match contfp(y, yf, normal, witness, band).map_err(|e| match e {
                                    ContainError::Escalated(diag) => BooleanError::Escalated {
                                        decision: BooleanDecision::Containment,
                                        diag,
                                    },
                                    ContainError::RayExhausted => {
                                        BooleanError::ClassificationInvariant {
                                            what: "extent scan: contfp ray schedule exhausted",
                                        }
                                    }
                                    ContainError::Corrupt => {
                                        BooleanError::ClassificationInvariant {
                                            what: "extent scan: contfp met corrupt topology",
                                        }
                                    }
                                    ContainError::Uncrossable(cause) => {
                                        BooleanError::ArcLoopContainmentUnsupported {
                                            operand: x_is,
                                            cause,
                                        }
                                    }
                                })? {
                                    // The circle misses this face
                                    // (it crosses the carrier plane
                                    // elsewhere).
                                    FaceContainment::Out => {}
                                    FaceContainment::In => {
                                        // The escape is the one
                                        // conclusion the closed group
                                        // is load-bearing for: the
                                        // whole-circle membership just
                                        // decided is the CARRIER's, and
                                        // the re-chart it feeds rotates
                                        // the group about its centre.
                                        if group.is_none() {
                                            return Err(BooleanError::FallbackExtentUnsupported {
                                                operand: x_is,
                                                face,
                                                what: "a TRIMMED sphere face group escapes \
                                                           through a plane face — the whole \
                                                           section circle is the carrier's, not \
                                                           the trimmed face's, and the re-chart \
                                                           that would follow rotates a closed \
                                                           group about its own centre",
                                            });
                                        }
                                        escape_normals.push(normal);
                                    }
                                    // Boxes cleared yet the witness is
                                    // ON the boundary: contradictory
                                    // enclosures, loudly.
                                    FaceContainment::OnEdge(_) | FaceContainment::OnVertex(_) => {
                                        return Err(BooleanError::ClassificationInvariant {
                                            what: "extent scan: witness on a boundary the \
                                                   certified boxes cleared",
                                        });
                                    }
                                }
                            }
                        }
                    }
                    &geom::Surface::Sphere {
                        center: c2,
                        radius: r2,
                        ..
                    } => {
                        let d = (c2 - center).norm();
                        // A decided zero is band-decided: the spheres
                        // touch within the tolerance, and a positive gap
                        // there is one a smaller tolerance decides apart.
                        // It refuses as the question's in-band arm does,
                        // with its decided margin.
                        match crate::validate::decide_nonzero_reported(
                            "bool_sphere_sphere_gap",
                            Margin::of(d - (radius + r2)),
                            band,
                        )
                        .map_err(esc(SphereQuestion::Apart))?
                        {
                            // Definitely separated.
                            NonzeroSign::Positive => {}
                            NonzeroSign::Negative => {
                                // Nested (one strictly inside the
                                // other) is boundary-disjoint too;
                                // anything else is the sphere×sphere
                                // seam frontier.
                                let big = radius.max(r2);
                                let small = radius.min(r2);
                                // Neither separated nor strictly nested:
                                // the CARRIERS cross in a circle while the
                                // crossing layer found no edge crossing a
                                // face. Whether the FACES meet is the
                                // section certificate's question, asked
                                // of every face on this sphere against
                                // `yf`: with no event anywhere, a circle
                                // certified out of one face of each pair
                                // leaves the two boundaries disjoint. A
                                // circle certified inside both faces is
                                // spheres that meet off every edge; any
                                // other refusal of the certificate is
                                // its own reason, raised as the pass
                                // raises it. A touch (a decided zero)
                                // refuses on the carriers.
                                let nested = crate::validate::decide_reported(
                                    "bool_sphere_sphere_nested",
                                    Margin::of(big - (d + small)),
                                    band,
                                )
                                .map_err(esc(SphereQuestion::Nested))?;
                                match Refused::of(nested, band) {
                                    None => {}
                                    Some(verdict @ Refused::Zero(_)) => {
                                        return Err(BooleanError::SpheresMeet {
                                            operand: x_is,
                                            face,
                                            verdict,
                                        });
                                    }
                                    Some(verdict @ Refused::Negative { .. }) => {
                                        match sphere_faces_apart(
                                            (x_is, x, x_rows),
                                            fd.surface,
                                            (y, y_row),
                                            band,
                                            &mut section_charts,
                                        )? {
                                            None => {}
                                            Some(SectionRefusal::Loop) => {
                                                return Err(BooleanError::SpheresMeet {
                                                    operand: x_is,
                                                    face,
                                                    verdict,
                                                });
                                            }
                                            Some(refusal) => {
                                                return Err(
                                                    BooleanError::FallbackExtentUnsupported {
                                                        operand: x_is,
                                                        face,
                                                        what: refusal.what(),
                                                    },
                                                );
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    geom::Surface::Nurbs(_) => {
                        // Unreachable: the re-gate above runs first.
                        return Err(BooleanError::NurbsExtentUnsupported {
                            operand: x_is.other(),
                            face: yf,
                        });
                    }
                    // `Approx` joins the no-wired-arm refusal, not the
                    // NURBS lane: the pair-scoped operand gate refuses
                    // it by kind before this scan runs.
                    geom::Surface::Approx(_) => {
                        // REACH FIRST, kind second. This arm asks
                        // whether the ball can escape past THIS face;
                        // a face whose box cannot meet the ball's
                        // certified extent bounds no escape route
                        // through it, and its kind is then no more
                        // relevant here than it is at the operand
                        // gate. Only a face the ball may actually
                        // reach costs the operation its answer.
                        if !y_row.bbox.overlaps(&ball_box) {
                            continue;
                        }
                        return Err(BooleanError::CurvedBooleanUnsupported {
                            operand: x_is.other(),
                            face: yf,
                            kind: y_row.surface.kind(),
                        });
                    }
                    // Every other kind's pairs with a sphere are the
                    // section pass's ([`section_pass_takes`]): never an
                    // escape face, so the one question is disjointness.
                    other => {
                        if !section_pass_takes(other) {
                            return Err(BooleanError::ClassificationInvariant {
                                what: "extent scan: a surface kind neither the scan nor the \
                                       section pass takes",
                            });
                        }
                    }
                }
            }
            // An escape was recorded, so the group is closed (the arm
            // above refuses otherwise) and has a representative.
            if let (Some((&align, rest)), Some(representative)) =
                (escape_normals.split_first(), group)
            {
                // ONE alignment per group (M5 S13 fix pass, review
                // MAJOR): the re-chart makes every section polar only
                // when ALL of this group's escape planes share a
                // normal direction. A group poking two NON-PARALLEL
                // faces would re-chart for the first and leave the
                // second cap's join to a tilted-section refusal that
                // the re-entered crossing layer may never reach — the
                // reviewer's witness answered 16 + cap_top, tier-3
                // valid, silently short one cap. Refused typed here
                // instead (metered at the group's own radius);
                // antiparallel normals are the SAME direction (the
                // finding row's top+bottom pair) and pass. Multi-chart
                // re-cutting stays banked as an extension.
                for &n in rest {
                    match decide(
                        "bool_sphere_escape_parallel",
                        Margin::levered(align.cross(n).norm(), radius),
                        band,
                    )
                    .map_err(esc(SphereQuestion::EscapeParallel))?
                    {
                        Sign::Zero => {}
                        Sign::Positive | Sign::Negative => {
                            return Err(BooleanError::FallbackExtentUnsupported {
                                operand: x_is,
                                face,
                                what: "one sphere group escapes through NON-PARALLEL \
                                       plane faces — a single re-chart cannot make \
                                       every section polar, and multi-chart \
                                       re-cutting is not built; refused whole rather \
                                       than metering one cap and dropping the other",
                            });
                        }
                    }
                }
                out.push(SphereRecut {
                    operand: x_is,
                    representative,
                    center,
                    radius,
                    axis,
                    align,
                });
            }
        }
    }
    Ok(out)
}

/// **Whether every face on `x`'s sphere `surface` is certified apart
/// from `y`'s face `y_row`**, whose carrier the sphere crosses: the
/// section certificate's walk ([`walk_pairs`]) over those pairs on the
/// no-event path. `None` when every pair clears, else the first
/// pair's refusal, which is the cause.
///
/// # Errors
///
/// [`walk_pairs`]'.
fn sphere_faces_apart<T: Decide + Bounds + crate::props::AtRestPolicy>(
    (x_is, x, x_rows): (Operand, &Body<T>, &[FaceRow<T>]),
    surface: SurfaceKey,
    (y, y_row): (&Body<T>, &FaceRow<T>),
    band: Band,
    charts: &mut ChartCache,
) -> Result<Option<SectionRefusal>, BooleanError> {
    let on_sphere = x_rows.iter().filter(|r| r.key == surface);
    let pairs = match x_is {
        Operand::A => walk_pairs(
            (x, on_sphere),
            (y, [y_row]),
            band,
            charts,
            |_, _| true,
            |_, _| false,
            |_| true,
            true,
        )?,
        Operand::B => walk_pairs(
            (y, [y_row]),
            (x, on_sphere),
            band,
            charts,
            |_, _| true,
            |_, _| false,
            |_| true,
            true,
        )?,
    };
    Ok(pairs.into_iter().find_map(|p| p.verdict.err()))
}

/// **Whether a re-cut sphere's polar axis leans off the escape normal**
/// ([`SphereQuestion::RecutAlign`]): the axes' cross levered at the
/// radius. Definite by construction on a crossing-free escape (an
/// aligned axis's seam crosses the escape plane, which the crossing
/// layer sees first), so an aligned or in-band axis refuses, with its
/// decided margin where it decided zero.
pub(super) fn recut_lean<T: Decide>(
    axis: Vec3<T>,
    align: Vec3<T>,
    radius: T,
    band: Band,
) -> Result<(), BooleanError> {
    crate::validate::decide_nonzero_reported(
        "bool_sphere_recut_align",
        Margin::levered(axis.cross(align).norm(), radius),
        band,
    )
    .map(|_| ())
    .map_err(|diag| BooleanError::Escalated {
        decision: BooleanDecision::Sphere(SphereQuestion::RecutAlign),
        diag,
    })
}

/// Applies the scan's re-cuts: each escaping group's shell is carved
/// out, rigidly rotated about the sphere's own center so the stored
/// polar axis lands on the escape normal (the same point set — a
/// sphere is rotation-invariant about its center — with the seam
/// meridians now transverse to the escape planes), and grafted back.
fn apply_recuts<T: Decide + Bounds + crate::props::AtRestPolicy>(
    a: &Body<T>,
    b: &Body<T>,
    recuts: &[SphereRecut<T>],
    tol: Tol,
) -> Result<(Body<T>, Body<T>), BooleanError> {
    let band = Band::linear(tol)?;
    let mut out_a = a.clone();
    let mut out_b = b.clone();
    for (operand, out) in [(Operand::A, &mut out_a), (Operand::B, &mut out_b)] {
        let mine: Vec<&SphereRecut<T>> = recuts.iter().filter(|r| r.operand == operand).collect();
        if mine.is_empty() {
            continue;
        }
        let src: &Body<T> = if operand == Operand::A { a } else { b };
        let corrupt = |what| BooleanError::ClassificationInvariant { what };
        let solid = single_solid(src).map_err(|_| corrupt("re-cut operand is not one solid"))?;
        // Shell of each group's representative face, arena order.
        let shell_of = |face: FaceKey| -> Result<ShellKey, BooleanError> {
            src.shells()
                .find(|(_, sd)| sd.faces.contains(&face))
                .map(|(k, _)| k)
                .ok_or(corrupt("re-cut representative face has no shell"))
        };
        let mut rotated: Vec<Body<T>> = Vec::new();
        let mut cut_shells: Vec<ShellKey> = Vec::new();
        for r in &mine {
            let shell = shell_of(r.representative)?;
            cut_shells.push(shell);
            let ball = carve(src, solid, &[shell])
                .map_err(|_| corrupt("re-cut carve of the sphere shell failed"))?;
            recut_lean(r.axis, r.align, r.radius, band)?;
            let cross = r.axis.cross(r.align);
            // The alignment rotation, built ALGEBRAICALLY (Rodrigues
            // with the angle eliminated: R = I + K + K²/(1+c) for
            // K = [â×n̂]ₓ, c = â·n̂ — division guarded by the
            // alignment trilean above, which already refused the
            // antiparallel c ≈ −1 arm together with the parallel one).
            // No atan2/sin/cos anywhere: under the Interval scalar a
            // trig-built rotation carries straddling-zero matrix
            // entries whose downstream crossing-root phases explode at
            // the atan2 branch cut; the algebraic form keeps exact
            // zeros exact, and the f64 lane is bit-for-bit the same
            // standard construction.
            let k = cross;
            let c = r.axis.dot(r.align);
            let kx = |v: Vec3<T>| k.cross(v);
            let one = T::one();
            let col = |e: Vec3<T>| {
                let kv = kx(e);
                e + kv + kx(kv) / (one + c)
            };
            let linear = geom_core::Mat3::from_cols(
                col(Vec3::new(one, T::zero(), T::zero())),
                col(Vec3::new(T::zero(), one, T::zero())),
                col(Vec3::new(T::zero(), T::zero(), one)),
            );
            let q = r.center - Point3::origin();
            let map = geom_core::Affine3::from_parts(linear, q - linear * q);
            let turned = crate::transform::transform_rigid(&ball, &map, tol)
                .map_err(|_| corrupt("re-cut rotation failed to re-certify"))?;
            rotated.push(turned);
        }
        let keep: Vec<ShellKey> = src
            .shells_of_solid(solid)
            .ok_or(corrupt("re-cut solid lost"))?
            .iter()
            .copied()
            .filter(|s| !cut_shells.contains(s))
            .collect();
        let mut rebuilt: Option<Body<T>> = if keep.is_empty() {
            None
        } else {
            Some(
                carve(src, solid, &keep)
                    .map_err(|_| corrupt("re-cut carve of the kept shells failed"))?,
            )
        };
        for turned in rotated {
            match rebuilt.as_mut() {
                None => rebuilt = Some(turned),
                Some(base) => {
                    let base_solid =
                        single_solid(base).map_err(|_| corrupt("re-cut base is not one solid"))?;
                    graft_solids_with(base, &[base_solid], &turned, Bridge::RemapKeys)?;
                }
            }
        }
        out.adopt(rebuilt.ok_or(corrupt("re-cut produced no body"))?);
    }
    Ok((out_a, out_b))
}

/// Per-shell classification of one operand's clone against the other
/// pristine operand (containment fallback), by the uncut-shell witness
/// ([`super::shell_witness`]).
///
/// **The witness is not the certificate** (M5 S13). A curved boundary
/// can leave the other solid strictly between a shell's witnesses (the
/// S12 finding), so for the sphere class the answer is only sound
/// because [`sphere_extent_scan`] ran first and certified every
/// sphere-involved boundary pair disjoint (or re-cut / refused): a
/// connected shell whose surface avoids the other boundary lies in one
/// component, and the witness names it.
fn classify_shells<T: Decide>(
    body: &Body<T>,
    other: &Body<T>,
    operand: Operand,
    coincident: &[super::SettledPair],
    band: Band,
    tol: Tol,
) -> Result<Vec<(ShellKey, ShellVerdict)>, BooleanError> {
    body.shells()
        .map(|(shell, _)| {
            let verdict = shell_verdict((body, shell, operand), other, coincident, band, tol)?;
            Ok((shell, verdict))
        })
        .collect()
}

/// The containment fallback (F8): no crossings — classify whole
/// shells, keep per Eq. 15.1's sides, and assemble the typed result.
fn fallback<T: Decide + Bounds + crate::props::AtRestPolicy>(
    op: BooleanOp,
    red: &BooleanReduction<T>,
    a_pristine: &Body<T>,
    b_pristine: &Body<T>,
    decls: &BooleanDeclarations,
    band: Band,
    tol: Tol,
) -> Result<BooleanResult<T>, BooleanError> {
    let desync = |what| BooleanError::JoinDesync { what };
    debug_assert_contacts_undecisive(
        &red.contacts,
        (&red.a, b_pristine),
        (&red.b, a_pristine),
        band,
        tol,
    );
    let a_sides = classify_shells(&red.a, b_pristine, Operand::A, &red.coincident, band, tol)?;
    let b_sides = classify_shells(&red.b, a_pristine, Operand::B, &red.coincident, band, tol)?;
    check_mutual(
        [(&red.a, &a_sides), (&red.b, &b_sides)],
        [a_pristine, b_pristine],
    )?;
    let a_keep = kept_shells(op, Operand::A, &a_sides);
    let b_keep = kept_shells(op, Operand::B, &b_sides);

    let carve_kept = |body: &Body<T>, keep: &[ShellKey]| -> Result<Body<T>, BooleanError> {
        let solid = single_solid(body).map_err(|_| desync("fallback operand not one solid"))?;
        carve(body, solid, keep).map_err(|_| desync("fallback carve failed"))
    };

    match (a_keep.is_empty(), b_keep.is_empty()) {
        (true, true) => {
            if op == BooleanOp::Union {
                // ∪ can never be empty; only complement operands (both
                // boundaries inside the other's material) reach here.
                Err(BooleanError::UnrepresentableResult)
            } else {
                Ok(BooleanResult::Empty)
            }
        }
        (false, true) => {
            let body = carve_kept(&red.a, &a_keep)?;
            finish_fallback(op, body, red, decls, BooleanResultKind::OperandA, band, tol)
        }
        (true, false) => {
            let body = carve_kept(&red.b, &b_keep)?;
            finish_fallback(op, body, red, decls, BooleanResultKind::OperandB, band, tol)
        }
        (false, false) => {
            let mut body = carve_kept(&red.a, &a_keep)?;
            let b_body = carve_kept(&red.b, &b_keep)?;
            let solid =
                single_solid(&body).map_err(|_| desync("fallback A carve not one solid"))?;
            let graft = if op == BooleanOp::Subtract {
                // ∖ with disjoint boundaries and kept B shells: every
                // kept shell classified strictly `In` A, so this is
                // the cavity case, routed through THE void-insertion
                // door (`voids` module — the one birthplace of
                // cavities), with this fallback's own probe verdicts
                // supplied as the door's containment evidence. The
                // evidence-shape refusals are unreachable from here
                // (the kept set IS the evidence set), mapped to the
                // desync they would represent.
                let evidence = voids::VoidEvidence {
                    shells: b_keep
                        .iter()
                        .map(|&s| (s, voids::VoidContainment::Probed(SolidContainment::In)))
                        .collect(),
                };
                voids::insert_hollow_voids(&mut body, &[solid], b_body, &evidence)
                    .map_err(|e| match e {
                        voids::VoidInsertError::Revert(r) => BooleanError::Revert(r),
                        voids::VoidInsertError::Corrupt { what } => {
                            BooleanError::JoinDesync { what }
                        }
                        voids::VoidInsertError::MissingEvidence { .. }
                        | voids::VoidInsertError::NotStrictlyContained { .. }
                        | voids::VoidInsertError::ForeignShell { .. }
                        | voids::VoidInsertError::DuplicateEvidence { .. }
                        | voids::VoidInsertError::HollowCavity { .. } => {
                            desync("void evidence desynced from the kept B shells")
                        }
                    })?
                    .graft
            } else {
                // The kept B shells cross whole, as the reduction left
                // them: any edge it split at a contact was certified
                // there through the policy's lane, and nothing touches
                // them since, so they keep their certificates, as at the
                // void door above.
                graft_solids_with(&mut body, &[solid], &b_body, Bridge::RemapKeys)?
            };
            let kind = match op {
                BooleanOp::Subtract => BooleanResultKind::Voided,
                _ => BooleanResultKind::Assembly,
            };
            let declared_pairs =
                declared_surface_pairs(&body, a_pristine, b_pristine, decls, &graft);
            let merged = body
                .merge_coplanar_faces_declared(&declared_pairs, tol)
                .map_err(BooleanError::Merge)?;
            let mut desc = Descendants::default();
            desc.absorb_merge(&merged);
            describe_minted_edges(&mut body, &[], &merged, band, tol)?;
            let mut contacts = remap_contacts(
                &body,
                &red.contacts,
                KeyView::Direct,
                KeyView::Graft(&graft),
                &desc,
            )?;
            remap_carried(
                &mut contacts,
                &body,
                decls,
                &KeyView::Direct,
                &KeyView::Graft(&graft),
                &desc,
            )?;
            let body = gate(body, band, tol)?;
            let (graft_vertices, graft_edges, graft_dead_edges, graft_faces) = graft_rows(&graft);
            let naming = BooleanNaming {
                a_keys: OperandKeys::Direct,
                b_keys: OperandKeys::Grafted,
                graft_vertices,
                graft_edges,
                graft_dead_edges,
                graft_faces,
                merge_groups: merge_rows(&merged),
                merge_skipped: merged.skipped.clone(),
                reduction_contacts: red.contacts.clone(),
                covered: red.covered.clone(),
                ..BooleanNaming::default()
            };
            Ok(BooleanResult::Body(BooleanBody {
                body,
                kind,
                contacts,
                naming,
            }))
        }
    }
}

/// Finishes a single-operand fallback result (the merge output stage
/// is a documented no-op on a maximal-faced operand but runs anyway —
/// the contract is uniform), applying ∖'s B-side revert when needed.
fn finish_fallback<T: Decide + Bounds + AtRestPolicy>(
    op: BooleanOp,
    body: Body<T>,
    red: &BooleanReduction<T>,
    decls: &BooleanDeclarations,
    kind: BooleanResultKind,
    band: Band,
    tol: Tol,
) -> Result<BooleanResult<T>, BooleanError> {
    let (contacts, covered) = (&red.contacts, &red.covered);
    let reduction_contacts = contacts.clone();
    let mut body = body;
    if kind == BooleanResultKind::OperandB && op == BooleanOp::Subtract {
        body = body.revert().map_err(BooleanError::Revert)?;
    }
    // No cross-operand pair merges here: one operand is absent from the
    // result. A declared pair that held the absent operand's region
    // through the kept one is `covered`; the surviving operand's
    // CARRIED records still apply.
    let merged = body
        .merge_coplanar_faces(tol)
        .map_err(BooleanError::Merge)?;
    let mut desc = Descendants::default();
    desc.absorb_merge(&merged);
    describe_minted_edges(&mut body, &[], &merged, band, tol)?;
    let (a_view, b_view) = match kind {
        BooleanResultKind::OperandA => (KeyView::Direct, KeyView::Absent),
        _ => (KeyView::Absent, KeyView::Direct),
    };
    let mut contacts = remap_contacts(&body, contacts, a_view, b_view, &desc)?;
    remap_carried(&mut contacts, &body, decls, &a_view, &b_view, &desc)?;
    let body = gate(body, band, tol)?;
    let naming = match kind {
        BooleanResultKind::OperandA => BooleanNaming {
            a_keys: OperandKeys::Direct,
            b_keys: OperandKeys::Absent,
            merge_groups: merge_rows(&merged),
            merge_skipped: merged.skipped.clone(),
            reduction_contacts: reduction_contacts.clone(),
            covered: covered.to_vec(),
            ..BooleanNaming::default()
        },
        // The result arena IS the B clone: B keys direct, A absent.
        _ => BooleanNaming {
            a_keys: OperandKeys::Absent,
            b_keys: OperandKeys::Direct,
            merge_groups: merge_rows(&merged),
            merge_skipped: merged.skipped.clone(),
            reduction_contacts: reduction_contacts.clone(),
            covered: covered.to_vec(),
            ..BooleanNaming::default()
        },
    };
    Ok(BooleanResult::Body(BooleanBody {
        body,
        kind,
        contacts,
        naming,
    }))
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use geom_core::{Band, Point3, Tol, Vec3};

    use super::{gate, seam_class, seam_must_carry, volume_backstop};
    use crate::boolean::{BooleanDecision, BooleanError, BooleanOp, LeverArm};
    use crate::props::QuadLane;
    use crate::splitting::reassembly::quad_prism;
    use crate::test_support::finished;

    /// The result gate refuses a scaffold at rest: a box whose chords
    /// were never described is a closed solid (tiers 1 and 2 pass), and
    /// tier 3 refuses it with one `ScaffoldAtRest` per edge among its
    /// findings; the same box described passes.
    #[test]
    fn the_result_gate_refuses_a_scaffold_at_rest() {
        let tol = Tol::witness();
        let square = [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];
        let raw = quad_prism(&square, 1.0, tol);
        assert_eq!(
            crate::validate_closed(&raw),
            Ok(()),
            "the raw box is closed"
        );
        let band = Band::linear(tol).unwrap();
        let Err(BooleanError::ResultInvalid { errors }) = gate(raw.clone(), band, tol) else {
            panic!("the raw box's chords are scaffolds at rest");
        };
        let mut named: Vec<_> = errors
            .iter()
            .filter_map(|e| match e {
                crate::ValidationError::ScaffoldAtRest { edge } => Some(*edge),
                _ => None,
            })
            .collect();
        named.sort();
        let mut edges: Vec<_> = raw.edges().map(|(k, _)| k).collect();
        edges.sort();
        assert_eq!(named, edges, "one finding per edge, each edge once");
        let described =
            crate::test_support_fixtures::brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol);
        assert_eq!(
            gate(described, band, tol)
                .map(|_| ())
                .map_err(|e| e.to_string()),
            Ok(()),
            "the described box passes"
        );
    }

    /// An operand's own scaffold never reaches the door: the undescribed
    /// box is refused at the at-rest gate that finishes it, naming its
    /// twelve chords, and the described box answers through the door.
    #[test]
    fn an_operand_carried_scaffold_is_refused_at_the_at_rest_gate() {
        let tol = Tol::witness();
        let square = [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];
        let raw = quad_prism(&square, 1.0, tol);
        let errors = crate::AtRestBody::validate(raw.clone(), tol)
            .expect_err("the box's chords are scaffolds at rest");
        let mut named: Vec<_> = errors
            .iter()
            .filter_map(|e| match e {
                crate::ValidationError::ScaffoldAtRest { edge } => Some(*edge),
                _ => None,
            })
            .collect();
        named.sort();
        let mut edges: Vec<_> = raw.edges().map(|(k, _)| k).collect();
        edges.sort();
        assert_eq!(named, edges, "one finding per chord: {errors:?}");
        let far = finished(
            "far brick",
            crate::test_support_fixtures::brick::<f64>((5.0, 6.0), (0.0, 1.0), (0.0, 1.0), tol),
            tol,
        );
        let described = finished(
            "described box",
            crate::test_support_fixtures::brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol),
            tol,
        );
        assert!(
            crate::boolean::subtract(&described, &far, tol).is_ok(),
            "the described box answers"
        );
    }

    /// The backstop's refusal wiring, by construction: feed it a
    /// "result" whose exact volume violates the op's bound (a half-height
    /// prism vs the unit cube, both surface-certified geometric bodies)
    /// and require the typed `ResultVolumeImplausible`; the non-strict
    /// pass direction (result ≡ operand, margin exactly zero) must pass
    /// all three ops.
    #[test]
    fn volume_backstop_wiring() {
        let band = Band::linear(Tol::witness()).unwrap();
        let square = [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];
        let cube = quad_prism(&square, 1.0, Tol::witness());
        let small = quad_prism(&square, 0.5, Tol::witness());
        let implausible = |e: BooleanError| {
            assert!(
                matches!(e, BooleanError::ResultVolumeImplausible { .. }),
                "expected ResultVolumeImplausible, got {e:?}"
            );
        };
        // vol(∪) ≥ max: a union "result" smaller than an operand.
        implausible(
            volume_backstop(
                BooleanOp::Union,
                &cube,
                &cube,
                &small,
                band,
                Tol::witness(),
                QuadLane::certified(),
            )
            .unwrap_err(),
        );
        // vol(∖) ≤ vol(A): a subtract "result" larger than A.
        implausible(
            volume_backstop(
                BooleanOp::Subtract,
                &small,
                &cube,
                &cube,
                band,
                Tol::witness(),
                QuadLane::certified(),
            )
            .unwrap_err(),
        );
        // vol(∩) ≤ min: an intersect "result" larger than an operand.
        implausible(
            volume_backstop(
                BooleanOp::Intersect,
                &small,
                &cube,
                &cube,
                band,
                Tol::witness(),
                QuadLane::certified(),
            )
            .unwrap_err(),
        );
        // Equal volumes: every bound is non-strict — all pass.
        for op in [BooleanOp::Union, BooleanOp::Intersect, BooleanOp::Subtract] {
            volume_backstop(
                op,
                &cube,
                &cube,
                &cube,
                band,
                Tol::witness(),
                QuadLane::certified(),
            )
            .unwrap();
        }
        // Complement operand (negative flux volume): its bound is
        // vacuous and must be SKIPPED — A ∩ revert(B) legitimately
        // exceeds vol(revert B); the A-side bound still applies.
        let rev = quad_prism(&square, 0.5, Tol::witness()).revert().unwrap();
        volume_backstop(
            BooleanOp::Intersect,
            &cube,
            &rev,
            &cube,
            band,
            Tol::witness(),
            QuadLane::certified(),
        )
        .unwrap();
        implausible(
            volume_backstop(
                BooleanOp::Intersect,
                &small,
                &rev,
                &cube,
                band,
                Tol::witness(),
                QuadLane::certified(),
            )
            .unwrap_err(),
        );
    }

    /// The arms the operands bound together, and the result's own sign:
    /// `vol(A ∪ B) ≤ vol(A) + vol(B)`, `vol(A ∖ B) ≥ vol(A) − vol(B)`,
    /// and a bounded result enclosing material. Each refuses a planted
    /// result that breaks it and only it, named by `which`.
    #[test]
    fn volume_backstop_joint_and_sign_arms() {
        let band = Band::linear(Tol::witness()).unwrap();
        let square = [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];
        let prism = |h: f64| quad_prism(&square, h, Tol::witness());
        let (cube, small, quarter, tall) = (prism(1.0), prism(0.5), prism(0.25), prism(1.5));
        let rev = cube.revert().expect("the cube reverts");
        let refuses = |op, a, b, result, which: &str| {
            let err = volume_backstop(
                op,
                a,
                b,
                result,
                band,
                Tol::witness(),
                QuadLane::certified(),
            )
            .unwrap_err();
            assert!(
                matches!(err, BooleanError::ResultVolumeImplausible { which: w, .. } if w == which),
                "expected {which}, got {err:?}"
            );
        };
        refuses(
            BooleanOp::Union,
            &small,
            &small,
            &tall,
            "vol(A ∪ B) ≤ vol(A) + vol(B)",
        );
        refuses(
            BooleanOp::Subtract,
            &cube,
            &small,
            &quarter,
            "vol(A ∖ B) ≥ vol(A) − vol(B)",
        );
        refuses(BooleanOp::Intersect, &cube, &cube, &rev, "vol(A ∩ B) ≥ 0");
        refuses(BooleanOp::Subtract, &cube, &small, &rev, "vol(A ∖ B) ≥ 0");
    }

    /// **A tie between congruent closed-form bodies passes.** One prism
    /// over a non-dyadic quadrilateral, its profile started at two
    /// different corners: the same solid, whose `f64` flux sums round
    /// apart (0.88374999999999970 against 0.88375000000000004 m³). Each
    /// bound reads the tie through the interval re-derivation and
    /// passes, in both orders; a result the walk alone would call past
    /// its bound.
    #[test]
    fn volume_backstop_passes_a_closed_form_rounding_tie() {
        let band = Band::linear(Tol::witness()).unwrap();
        let profile = [(0.1, 0.2), (1.3, 0.25), (1.1, 1.7), (0.05, 0.9)];
        let mut turned = profile;
        turned.rotate_left(3);
        let (low, high) = (
            quad_prism(&profile, 0.7, Tol::witness()),
            quad_prism(&turned, 0.7, Tol::witness()),
        );
        let volume = |body| crate::mass_properties(body, Tol::witness()).unwrap().volume;
        assert!(
            volume(&low) < volume(&high),
            "the two corners' sums round apart: {} vs {}",
            volume(&low),
            volume(&high)
        );
        for (op, operand, result) in [
            (BooleanOp::Intersect, &low, &high),
            (BooleanOp::Subtract, &low, &high),
            (BooleanOp::Union, &high, &low),
        ] {
            let got = volume_backstop(
                op,
                operand,
                operand,
                result,
                band,
                Tol::witness(),
                QuadLane::certified(),
            );
            assert!(got.is_ok(), "{op:?}: {got:?}");
        }
    }

    /// **The +V arm reads tier 3's rule**: `V/A` against the band, a
    /// negative certified past it refuses and an in-band one is exempt.
    /// An intersect of two unit cubes whose "result" is one body of two
    /// shells, a reverted unit cube and a cube of side `s` beside it, so
    /// `V = s³ − 1` over `A ≈ 12 m²`: at `V/A = −ε/2` it is in band and
    /// passes, and one step past the band's edge (`−1.01·escalate`) and
    /// well past it (`−4·escalate`) it refuses.
    #[test]
    fn volume_backstop_reads_the_sign_against_the_band() {
        use crate::test_support_fixtures::{cube_into, mapped_cube};
        let tol = Tol::witness();
        let band = Band::linear(tol).unwrap();
        let unit = |x0: f64, side: f64| {
            move |u: f64, v: f64, w: f64| geom_core::Point3::new(x0 + side * u, side * v, side * w)
        };
        let cube = mapped_cube::<f64>(unit(0.0, 1.0), tol);
        let run = |v_over_a: f64| {
            // `V = s³ − 1` over `A = 6 + 6s²`.
            let s = (1.0 + 12.0 * v_over_a).cbrt();
            let mut result = mapped_cube::<f64>(unit(0.0, 1.0), tol)
                .revert()
                .expect("the cube reverts");
            cube_into(&mut result, unit(5.0, s), tol);
            volume_backstop(
                BooleanOp::Intersect,
                &cube,
                &cube,
                &result,
                band,
                tol,
                QuadLane::certified(),
            )
        };
        let in_band = run(-0.5 * band.zero());
        assert!(in_band.is_ok(), "in band: {in_band:?}");
        for past in [1.01, 4.0] {
            assert!(
                matches!(
                    run(-past * band.escalate()),
                    Err(BooleanError::ResultVolumeImplausible {
                        which: "vol(A ∩ B) ≥ 0",
                        ..
                    })
                ),
                "{past}·escalate past the band"
            );
        }
    }

    /// **The #200 review's MAJ-1, end to end through the real gate.**
    /// A wrongly-kept 3 mm cube on a 2 m × 2 m × 0.1 m plate: the
    /// violation is ΔV = 2.7e-8 m³ spread over ~17.6 m² of boundary, so
    /// the METERED mean displacement is 1.53e-9 m — inside the default
    /// `Band{1e-9, 1e-8}`, i.e. exactly the region where a
    /// magnitude-only gate passes a macroscopic wrong component. The
    /// sign arm (`volume_backstop_violation`, exact band) certifies the
    /// inequality is violated regardless, so the gate REFUSES.
    ///
    /// This is the pin on the dual-arm structure: delete arm 1 and this
    /// test goes red, the one row whose violation hides inside the band
    /// by its area (the other planted-violation rows red with it).
    #[test]
    fn volume_backstop_refuses_a_wrong_component_hidden_by_a_large_area() {
        let band = Band::linear(Tol::witness()).unwrap();
        let plate_profile = [(0.0, 0.0), (2.0, 0.0), (2.0, 2.0), (0.0, 2.0)];
        let plate = quad_prism(&plate_profile, 0.1, Tol::witness());
        // The "result" of `plate ∖ tool` that wrongly KEPT a 3 mm cube:
        // same footprint, 2.7e-8 m³ of extra material (a 6.75e-9 m lift
        // over the 4 m² footprint — the volume of a 3 mm cube).
        let kept = 0.003_f64.powi(3);
        let wrong = quad_prism(&plate_profile, 0.1 + kept / 4.0, Tol::witness());
        let err = volume_backstop(
            BooleanOp::Subtract,
            &plate,
            &plate,
            &wrong,
            band,
            Tol::witness(),
            QuadLane::certified(),
        )
        .unwrap_err();
        assert!(
            matches!(err, BooleanError::ResultVolumeImplausible { .. }),
            "a macroscopic wrong component must refuse however much \
             boundary area it is smeared over, got {err:?}"
        );
        // The exact-zero pass direction is untouched by the sign arm.
        volume_backstop(
            BooleanOp::Subtract,
            &plate,
            &plate,
            &plate,
            band,
            Tol::witness(),
            QuadLane::certified(),
        )
        .unwrap();
    }

    /// **The NURBS re-gate, pinned (M5 S13 §1), and the placeholder's
    /// own door.** The fallback's curved-extent test is UNWRITABLE for
    /// NURBS today (`implicit_residual` is poison there, and no
    /// projection-based extent argument has been written — the
    /// `NurbsSurface::project` half of the old blocker retired at
    /// M6-2's lift), so the class is re-gated AT THE FALLBACK with a
    /// typed refusal naming its TRUE blocker — a future NURBS body
    /// constructor inherits this door, never the vertex-probe silence
    /// the S12 finding executed. That re-gate is pinned here at the
    /// mechanism, because a body on the `mvfs` `Nurbs` PLACEHOLDER
    /// surface cannot reach the boolean at all: it is not a finished
    /// body, and the at-rest gate refuses each of its faces as
    /// uncertifiable. Both are pinned below; what neither may become is
    /// a silent assembly.
    #[test]
    fn nurbs_faces_refuse_typed_at_both_doors() {
        use crate::test_support_fixtures::{FaceGeometry, UNIT_SQUARE, declined_cube, prism_ops};
        use geom_core::Point3;

        // `declined_cube`, x-shifted: the same builder under one
        // translating map. The fixture is anchored at the origin, and
        // this operand needs a disjoint certified box so the realized
        // sweep examines no pair at all.
        let far_cube = |dx: f64| {
            let mut body = crate::Body::<f64>::new();
            prism_ops(
                &mut body,
                &UNIT_SQUARE,
                (0.0, 1.0),
                |x, y, z| Point3::new(x + dx, y, z),
                FaceGeometry::Declined,
                Tol::witness(),
            );
            body
        };

        let a = declined_cube::<f64>(Tol::witness()).body;
        let b = far_cube(10.0);
        // Door 1 — the placeholder is not a finished body: the at-rest
        // gate that would hand it to the boolean refuses each of its
        // faces as uncertifiable, so no NURBS placeholder reaches a pair.
        for (what, body) in [("the declined cube", &a), ("the far declined cube", &b)] {
            let errors = crate::AtRestBody::validate(body.clone(), Tol::witness())
                .expect_err("a NURBS placeholder is not a finished body");
            let mut uncertifiable: Vec<_> = errors
                .iter()
                .filter_map(|e| match e {
                    crate::ValidationError::UncertifiableSurface { face } => Some(*face),
                    _ => None,
                })
                .collect();
            uncertifiable.sort();
            let mut faces: Vec<_> = body.faces().map(|(k, _)| k).collect();
            faces.sort();
            assert_eq!(uncertifiable, faces, "{what}: every face named: {errors:?}");
        }

        // Door 2 — the fallback's own re-gate, at the mechanism: any
        // fallback entry carrying a NURBS face refuses BEFORE a vertex
        // is probed. NO end-to-end path reaches it today (a lofted
        // operand is refused at its NURBS EDGES first, a placeholder at
        // the at-rest gate) — `sweep`'s `s16_box_soundness` pins the
        // first and Door 1 above the second, so the day one lifts is
        // loud.
        let band = Band::linear(Tol::witness()).unwrap();
        let Err(err) = super::sphere_extent_scan(&a, &b, band) else {
            panic!("the NURBS fallback must be re-gated, never vertex-probed");
        };
        let BooleanError::NurbsExtentUnsupported { .. } = err else {
            panic!("expected the NURBS re-gate, got {err:?}");
        };
        // The refusal names the face kind that stopped it and ends on
        // what the person can do; the lift blocker is the variant's
        // rustdoc, not the sentence.
        let msg = err.to_string();
        assert!(msg.contains("spline (NURBS) face"), "{msg}");
        assert!(msg.contains("Recourse: "), "{msg}");
    }

    /// The D5 descendant chase, pinned at the mechanism level (M3
    /// PR 6a): a v-on-f record whose FACE key is dead (an absorbed
    /// merge fragment — realized here with a foreign-arena key, the
    /// same dead-key shape) survives `remap_contacts` when the
    /// descendant map names its surviving fragment, and drops without
    /// the row — record loss over a live coincidence is exactly what
    /// the map exists to prevent (PR 5 review R5). The v-v lane's
    /// consumed-pair rule is pinned too: a pair fused into ONE vertex
    /// drops.
    #[test]
    fn descendant_chase_wiring() {
        use super::{Descendants, KeyView, remap_contacts};
        use crate::boolean::{ContactRecords, VfContact, VvContact};

        let square = [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];
        let mut body = quad_prism(&square, 1.0, Tol::witness());
        let live_vertex = body.vertices().next().map(|(k, _)| k).unwrap();
        let live_face = body.faces().next().map(|(k, _)| k).unwrap();
        // Dead keys: arena entries removed in place (the test only
        // exercises key lookups; tier validity is irrelevant here).
        let dead_face = body.faces().map(|(k, _)| k).nth(5).unwrap();
        body.faces.remove(dead_face);
        assert!(body.get_face(dead_face).is_none(), "key is dead");

        let contacts = ContactRecords {
            vv: vec![],
            a_on_b: vec![VfContact {
                vertex: live_vertex,
                face: dead_face,
            }],
            b_on_a: vec![],
            ..ContactRecords::default()
        };
        // Without the descendant row: the record drops (pre-D5 loss).
        let out = remap_contacts(
            &body,
            &contacts,
            KeyView::Direct,
            KeyView::Direct,
            &Descendants::default(),
        )
        .unwrap();
        assert!(out.a_on_b.is_empty());
        // With the row: the record survives, renamed to the survivor.
        let mut desc = Descendants::default();
        desc.faces.insert(dead_face, live_face);
        let out =
            remap_contacts(&body, &contacts, KeyView::Direct, KeyView::Direct, &desc).unwrap();
        assert_eq!(out.a_on_b.len(), 1);
        assert_eq!(out.a_on_b[0].face, live_face);
        assert_eq!(out.a_on_b[0].vertex, live_vertex);

        // v-v consumed-pair rule: both sides chased into one vertex ⇒
        // the coincidence is structural now, the record drops.
        let dead_vertex = body.vertices().map(|(k, _)| k).nth(3).unwrap();
        assert_ne!(dead_vertex, live_vertex);
        body.vertices.remove(dead_vertex);
        let contacts = ContactRecords {
            vv: vec![VvContact {
                a: dead_vertex,
                b: live_vertex,
            }],
            a_on_b: vec![],
            b_on_a: vec![],
            ..ContactRecords::default()
        };
        let mut desc = Descendants::default();
        desc.vertices.push((dead_vertex, live_vertex));
        let out =
            remap_contacts(&body, &contacts, KeyView::Direct, KeyView::Direct, &desc).unwrap();
        assert!(out.vv.is_empty(), "fused-into-one pair is consumed");
    }

    /// **A fusion list whose row keeps a key an earlier row killed
    /// refuses; one that chases to a dead key drops the record.** Rows
    /// `(a, b), (c, a)` fold `c` onto the dead `a`, so the chase would
    /// read a corrupt list as "consumed". The refusal must hold in every
    /// build: in one where `survivor`'s `debug_assert!` were the only
    /// guard, this panics there instead of answering `Err`, and drops
    /// the record silently where it compiles out.
    #[test]
    fn a_corrupt_fusion_list_refuses_where_a_dead_end_drops() {
        use super::{Descendants, KeyView, remap_carried, remap_contacts};
        use crate::boolean::{
            BooleanDeclarations, CarriedContacts, CarriedVv, ContactClass, ContactRecords,
            VvContact,
        };
        use crate::entity::VertexKey;

        let square = [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];
        let mut body = quad_prism(&square, 1.0, Tol::witness());
        let keys: Vec<VertexKey> = body.vertices().map(|(k, _)| k).collect();
        let (a, b, c, partner) = (keys[0], keys[1], keys[2], keys[3]);
        body.vertices.remove(a);
        body.vertices.remove(c);
        let pair = VvContact { a: c, b: partner };
        let contacts = ContactRecords {
            vv: vec![pair],
            ..ContactRecords::default()
        };
        let decls = BooleanDeclarations {
            carried_a: CarriedContacts {
                vv: vec![CarriedVv {
                    pair,
                    class: ContactClass::Rest,
                }],
                ..CarriedContacts::default()
            },
            ..BooleanDeclarations::default()
        };
        let remap = |desc: &Descendants| {
            remap_contacts(&body, &contacts, KeyView::Direct, KeyView::Direct, desc)
        };
        let carry = |desc: &Descendants| {
            let mut out = ContactRecords::default();
            remap_carried(
                &mut out,
                &body,
                &decls,
                &KeyView::Direct,
                &KeyView::Direct,
                desc,
            )
            .map(|()| out)
        };
        let joined = |r: Result<ContactRecords, BooleanError>| match r {
            Err(BooleanError::JoinDesync { what }) => what,
            other => panic!("a corrupt fusion list must refuse JoinDesync, got {other:?}"),
        };

        // A well-ordered list that ends on a dead key: consumed, dropped.
        let mut dead_end = Descendants::default();
        dead_end.vertices.push((c, a));
        let dropped = remap(&dead_end).expect("a well-ordered list is not corrupt");
        assert!(dropped.vv.is_empty(), "dead end: {:?}", dropped.vv);
        let carried = carry(&dead_end).expect("a well-ordered list is not corrupt");
        assert!(carried.vv.is_empty(), "carried dead end: {:?}", carried.vv);

        let mut corrupt = Descendants::default();
        corrupt.vertices.push((a, b));
        corrupt.vertices.push((c, a));
        let what = "a fusion row names a key an earlier row killed";
        assert_eq!(
            joined(remap(&corrupt)),
            what,
            "remap_contacts on a corrupt list"
        );
        assert_eq!(
            joined(carry(&corrupt)),
            what,
            "remap_carried on a corrupt list"
        );
    }

    /// **A cycle in the face absorption rows refuses; a chain that ends
    /// at a dead key drops the record.** The two dead faces name each
    /// other, so the chase never reaches a live face nor a key without
    /// a row: it revisits, and a revisit is a corrupt record. Read as
    /// "consumed", it would drop a declared contact without a word —
    /// the dead end's answer, which this pins as different.
    #[test]
    fn a_cycling_absorption_row_refuses_where_a_dead_end_drops() {
        use super::{Descendants, KeyView, remap_carried, remap_contacts};
        use crate::boolean::{
            BooleanDeclarations, CarriedContacts, CarriedVf, ContactClass, ContactRecords,
            VfContact,
        };
        use crate::entity::FaceKey;

        let square = [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];
        let mut body = quad_prism(&square, 1.0, Tol::witness());
        let vertex = body.vertices().next().map(|(k, _)| k).unwrap();
        let keys: Vec<FaceKey> = body.faces().map(|(k, _)| k).collect();
        let (d0, d1) = (keys[4], keys[5]);
        body.faces.remove(d0);
        body.faces.remove(d1);
        let contacts = ContactRecords {
            a_on_b: vec![VfContact { vertex, face: d0 }],
            ..ContactRecords::default()
        };
        let remap = |desc: &Descendants| {
            remap_contacts(&body, &contacts, KeyView::Direct, KeyView::Direct, desc)
        };

        let mut dead_end = Descendants::default();
        dead_end.faces.insert(d0, d1);
        let dropped = remap(&dead_end).expect("a chain that ends is not corrupt");
        assert!(dropped.a_on_b.is_empty(), "dead end: {:?}", dropped.a_on_b);

        let mut cycle = Descendants::default();
        cycle.faces.insert(d0, d1);
        cycle.faces.insert(d1, d0);
        let joined = |r: Result<_, BooleanError>| match r {
            Err(BooleanError::JoinDesync { what }) => what,
            other => panic!("a cycle must refuse JoinDesync, got {other:?}"),
        };
        assert_eq!(
            joined(remap(&cycle).map(|_| ())),
            "a face's absorption rows are cyclic",
            "remap_contacts on a cycle"
        );

        // The carried lane reads the same chase.
        let decls = BooleanDeclarations {
            carried_a: CarriedContacts {
                vf: vec![CarriedVf {
                    rest: VfContact { vertex, face: d0 },
                    class: ContactClass::Rest,
                }],
                ..CarriedContacts::default()
            },
            ..BooleanDeclarations::default()
        };
        let carry = |desc: &Descendants| {
            let mut out = ContactRecords::default();
            remap_carried(
                &mut out,
                &body,
                &decls,
                &KeyView::Direct,
                &KeyView::Direct,
                desc,
            )
            .map(|()| out)
        };
        let carried = carry(&dead_end).expect("a chain that ends is not corrupt");
        assert!(
            carried.a_on_b.is_empty(),
            "carried dead end: {:?}",
            carried.a_on_b
        );
        assert_eq!(
            joined(carry(&cycle).map(|_| ())),
            "a face's absorption rows are cyclic",
            "remap_carried on a cycle"
        );
    }

    /// **A v-v record follows a vertex either operand's pinch weld
    /// fused away.** B's weld runs in B-clone keys before the graft, so
    /// its dead key has no graft row: the chase reads the weld first,
    /// then the graft. A's weld rows are result keys. Without the weld
    /// rows both records drop.
    ///
    /// Only this row reaches the weld rows, by building `Descendants`
    /// by hand: no boolean can hand them such a record. A weld fuses
    /// pierce ring vertices `vtxfac` mints after the reduction's
    /// contacts are recorded, and carried records cite operand keys, so
    /// no record cites a weld's dead key.
    #[test]
    fn a_vv_record_follows_a_pinch_weld_on_either_side() {
        use super::{Descendants, KeyView, remap_contacts};
        use crate::boolean::combine::GraftMap;
        use crate::boolean::{ContactRecords, VvContact};

        let square = [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];
        let mut body = quad_prism(&square, 1.0, Tol::witness());
        let keys: Vec<crate::entity::VertexKey> = body.vertices().map(|(k, _)| k).collect();
        let (a_live, a_kept, a_dead, b_dead, b_kept) =
            (keys[0], keys[1], keys[2], keys[3], keys[4]);
        body.vertices.remove(a_dead);
        // B-clone keys: `b_dead` stands for the welded-away pierce (no
        // graft row), `b_kept` for its partner, grafted to `keys[5]`.
        let mut graft = GraftMap::default();
        graft.vertices.insert(b_kept, keys[5]);
        let contacts = ContactRecords {
            vv: vec![
                VvContact {
                    a: a_live,
                    b: b_dead,
                },
                VvContact {
                    a: a_dead,
                    b: b_kept,
                },
            ],
            ..ContactRecords::default()
        };
        let remap = |desc: &Descendants| {
            remap_contacts(
                &body,
                &contacts,
                KeyView::Direct,
                KeyView::Graft(&graft),
                desc,
            )
            .unwrap()
            .vv
        };
        assert!(
            remap(&Descendants::default()).is_empty(),
            "without the weld rows neither record resolves"
        );
        let welded = Descendants::welded(&[(a_dead, a_kept)], &[(b_dead, b_kept)]);
        assert_eq!(
            remap(&welded),
            vec![
                VvContact {
                    a: a_live,
                    b: keys[5]
                },
                VvContact {
                    a: a_kept,
                    b: keys[5]
                },
            ],
            "each record follows its side's weld"
        );
    }

    /// **A record citing a vertex the merge pruned drops, through the
    /// merge's REAL outcome.** The records cannot reach this point in a
    /// boolean by construction, so the row drives the rule itself with
    /// a real pruning merge instead:
    ///
    /// - A vertex the pruning deletes is a junction of the seam the
    ///   zip laid in the merged plane — a vertex interior to the other
    ///   operand's coplanar face, where its cut turns. A vertex-on-face
    ///   rest there is one the zip fused into seam structure, so it
    ///   has already dropped as fused before the merge runs.
    /// - A vertex-vertex record there names the other operand's vertex
    ///   at the same point, which the zip fuses into it too, so the
    ///   pair is one vertex and consumed.
    ///
    /// What is left for the merge to get wrong is the descendant
    /// chase: a fusion row whose survivor the pruning then deletes. So
    /// the row takes a prism whose top is split by a seam bent at an
    /// interior vertex, runs the real `merge_coplanar_faces`, absorbs
    /// its outcome into [`Descendants`] exactly as the boolean does,
    /// adds the fusion row a zip would have written into the deleted
    /// vertex, and requires every lane to drop a record citing either
    /// key. A `Descendants` that mapped the deleted vertex to a
    /// survivor would carry the records and turn this red.
    #[test]
    fn a_record_citing_a_pruned_free_end_drops() {
        use super::{Descendants, KeyView, Operand, remap_contacts};
        use crate::boolean::{ContactRecords, VfContact, VvContact};
        use crate::entity::VertexKey;
        use crate::{MefSite, MevSite};
        use geom_core::Point3;

        let tol = Tol::witness();
        let p = crate::test_support_fixtures::prism_z::<f64>(
            &[(0.0, 0.0), (2.0, 0.0), (2.0, 2.0), (0.0, 2.0)],
            0.0,
            1.0,
            tol,
        );
        let mut body = p.body;
        let corner = |body: &crate::Body<f64>, x: f64, y: f64| {
            let outer = body.get_face(p.top_face).unwrap().outer;
            let crate::LoopBoundary::Cycle { first } = body.get_loop(outer).unwrap().boundary
            else {
                panic!("the top is a cycle")
            };
            body.loop_cycle(first)
                .unwrap()
                .into_iter()
                .find(|&he| {
                    let v = body.get_half_edge(he).unwrap().start;
                    let q = body.get_point(body.get_vertex(v).unwrap().point).unwrap();
                    (q.x, q.y) == (x, y)
                })
                .unwrap()
        };
        let he1 = corner(&body, 0.0, 0.0);
        let strut = body
            .mev_line(
                MevSite::Fan { he1, he2: he1 },
                Point3::new(0.9, 0.6, 1.0),
                tol,
            )
            .unwrap();
        let he2 = corner(&body, 2.0, 2.0);
        body.mef_chord(
            MefSite::Chords {
                he1: strut.he_minus,
                he2,
            },
            tol,
        )
        .unwrap();
        let bend = strut.vertex;

        let merged = body
            .merge_coplanar_faces(tol)
            .expect("the bent seam repairs");
        let killed: Vec<VertexKey> = merged
            .groups
            .iter()
            .flat_map(|g| g.killed_vertices.iter().copied())
            .collect();
        assert_eq!(killed, vec![bend], "the bend is the pruned free end");
        assert!(body.get_vertex(bend).is_none());

        let mut desc = Descendants::default();
        desc.absorb_merge(&merged);
        // The row the zip writes when it fuses a coincident vertex
        // into the one the merge later deletes; the fused-in key is
        // dead (the null key, which no arena holds).
        let fused_in = VertexKey::default();
        let survivor = body.vertices().map(|(k, _)| k).next().unwrap();
        let face = body.faces().map(|(k, _)| k).next().unwrap();
        desc.vertices.push((fused_in, bend));
        desc.fused.insert(bend);
        assert_eq!(
            desc.live_vertex(&body, (Operand::A, &KeyView::Direct), bend)
                .unwrap(),
            None,
            "no survivor for {bend:?}"
        );

        let records = |v: VertexKey| ContactRecords {
            vv: vec![VvContact { a: v, b: survivor }],
            a_on_b: vec![VfContact { vertex: v, face }],
            b_on_a: vec![VfContact { vertex: v, face }],
            ..ContactRecords::default()
        };
        let remap = |c: &ContactRecords| {
            remap_contacts(&body, c, KeyView::Direct, KeyView::Direct, &desc).unwrap()
        };
        assert_eq!(
            desc.live_vertex(&body, (Operand::A, &KeyView::Direct), fused_in)
                .unwrap(),
            None,
            "the chase ends dead"
        );
        for cited in [bend, fused_in] {
            let out = remap(&records(cited));
            assert!(out.vv.is_empty(), "{cited:?}: {:?}", out.vv);
            assert!(out.a_on_b.is_empty(), "{cited:?}: {:?}", out.a_on_b);
            assert!(out.b_on_a.is_empty(), "{cited:?}: {:?}", out.b_on_a);
        }
        // The control: the same records on a live vertex carry.
        let other = body.vertices().map(|(k, _)| k).nth(1).unwrap();
        let out = remap(&records(other));
        assert_eq!(
            out.vv,
            vec![VvContact {
                a: other,
                b: survivor
            }]
        );
        assert_eq!(
            out.a_on_b,
            vec![VfContact {
                vertex: other,
                face
            }]
        );
    }

    fn band() -> Band {
        Band::linear(Tol::witness()).unwrap()
    }

    /// A margin inside the band at every ε row: its two edges'
    /// geometric mean.
    fn in_band() -> f64 {
        (band().zero() * band().escalate()).sqrt()
    }

    /// The floor `z = 0` and a unit cylinder resting on it along the
    /// `y` axis, which is their tangency ruling: `κ_rel = 1` across it.
    fn resting() -> (geom::Surface<f64>, geom::Surface<f64>, geom::Curve3<f64>) {
        let floor = geom::Surface::Plane {
            origin: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 1.0),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let cylinder = geom::Surface::Cylinder {
            origin: Point3::new(0.0, 0.0, 1.0),
            axis: Vec3::new(0.0, 1.0, 0.0),
            radius: 1.0,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let ruling = geom::Curve3::Line {
            origin: Point3::new(0.0, 0.0, 0.0),
            dir: Vec3::new(0.0, 1.0, 0.0),
        };
        (floor, cylinder, ruling)
    }

    /// The seam's answer over `[0, extent]` of `carrier`, in both
    /// argument orders, with its witness read smooth first (the arm the
    /// rule is asked from).
    fn both_orders(
        s1: &geom::Surface<f64>,
        s2: &geom::Surface<f64>,
        carrier: &geom::Curve3<f64>,
        (t0, t1): (f64, f64),
        extent: f64,
    ) -> [Result<bool, BooleanError>; 2] {
        let witness = carrier.eval((t0 + t1) / 2.0);
        [(s1, s2), (s2, s1)].map(|(a, b)| {
            assert!(
                matches!(
                    seam_class(a, b, witness, extent, band()),
                    Ok(geom_brep::DihedralClass::Smooth)
                ),
                "the row's seam reads smooth at its witness"
            );
            seam_must_carry(a, b, carrier, t0, t1, extent, band())
        })
    }

    /// **A smooth seam whose bend is in band refuses as `SeamJet`, in
    /// both orders**: its stations' second-order sagitta is certifiable
    /// as neither the intrinsic tangency nor the conventional posture,
    /// so it is never stored as either (tier 3 refuses such an edge
    /// `SliverDihedral`).
    #[test]
    fn a_seam_bending_apart_in_band_refuses_as_its_second_order_decision() {
        let (floor, cylinder, ruling) = resting();
        // `κ_rel · extent² / 2` with `κ_rel = 1` and the arm the extent.
        let extent = (2.0 * in_band()).sqrt();
        for (order, got) in both_orders(&floor, &cylinder, &ruling, (0.0, extent), extent)
            .into_iter()
            .enumerate()
        {
            match got {
                Err(BooleanError::Escalated {
                    decision: BooleanDecision::SeamJet,
                    diag,
                }) => assert_eq!(
                    diag.predicate,
                    Some("tangent_second_order"),
                    "order {order}: the escalation is the station's sagitta"
                ),
                other => panic!("order {order}: an in-band bend must refuse typed, got {other:?}"),
            }
        }
    }

    /// **A station whose first-order dihedral is in band refuses as the
    /// seam's own lever**, the rung `seam_class` gives the same reading
    /// at the witness: over an extent in band the arm cannot measure the
    /// angle.
    #[test]
    fn a_seam_whose_station_arm_is_in_band_refuses_as_the_seam_lever() {
        let (floor, cylinder, ruling) = resting();
        let extent = in_band();
        for (a, b) in [(&floor, &cylinder), (&cylinder, &floor)] {
            let got = seam_must_carry(a, b, &ruling, 0.0, 1.0, extent, band());
            assert!(
                matches!(
                    got,
                    Err(BooleanError::Escalated {
                        decision: BooleanDecision::LeverArm(LeverArm::Seam),
                        ..
                    })
                ),
                "an in-band arm is the seam lever's refusal, got {got:?}"
            );
        }
    }

    /// **A determinate seam is intrinsic, a flush one conventional**:
    /// the resting cylinder over a unit extent bends apart definitely,
    /// and two coplanar planes split along a line bend apart not at
    /// all — exactly zero, so the conventional chord stays.
    #[test]
    fn a_determinate_seam_is_intrinsic_and_a_flush_one_conventional() {
        let (floor, cylinder, ruling) = resting();
        for (order, got) in both_orders(&floor, &cylinder, &ruling, (0.0, 1.0), 1.0)
            .into_iter()
            .enumerate()
        {
            assert!(
                matches!(got, Ok(true)),
                "order {order}: a definite bend demands the intrinsic tangency, got {got:?}"
            );
        }
        let split = geom::Surface::Plane {
            origin: Point3::new(0.0, 5.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 1.0),
            u_ref: Vec3::new(0.0, 1.0, 0.0),
        };
        for (order, got) in both_orders(&floor, &split, &ruling, (0.0, 1.0), 1.0)
            .into_iter()
            .enumerate()
        {
            assert!(
                matches!(got, Ok(false)),
                "order {order}: coplanar planes keep the conventional chord, got {got:?}"
            );
        }
    }

    /// A torus (`R = 2`, `r = 1`) about `z`, its bitangent plane, and the
    /// Villarceau circle they share through the plane's point of
    /// tangency, which is the circle's `θ = 0`: the plane touches the
    /// torus there and crosses it everywhere else on the circle.
    fn villarceau() -> (geom::Surface<f64>, geom::Surface<f64>, geom::Curve3<f64>) {
        let (big, r) = (2.0_f64, 1.0_f64);
        let (sin, cos) = (r / big, (1.0 - (r / big).powi(2)).sqrt());
        let torus = geom::Surface::Torus {
            center: Point3::new(0.0, 0.0, 0.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
            major_radius: big,
            minor_radius: r,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let normal = Vec3::new(sin, 0.0, -cos);
        let bitangent = geom::Surface::Plane {
            origin: Point3::new(0.0, 0.0, 0.0),
            normal,
            u_ref: Vec3::new(0.0, 1.0, 0.0),
        };
        let center = Point3::new(0.0, r, 0.0);
        let touch = Point3::new(big - r * sin, 0.0, r * cos);
        let circle = geom::Curve3::Circle {
            center,
            axis: normal,
            radius: big,
            u_ref: (touch - center) / big,
        };
        for i in 0..geom_brep::CERT_SAMPLES {
            let p = circle.eval(geom_brep::sample_param(-1.0, 1.0, i));
            let on = |s: &geom::Surface<f64>| geom_brep::implicit_residual(s, p).abs() < 1e-12;
            assert!(
                on(&torus) && on(&bitangent),
                "the circle lies on both surfaces at {p:?}"
            );
        }
        (torus, bitangent, circle)
    }

    /// **A seam smooth at its witness and a corner at a station keeps
    /// the conventional posture, in both orders**, on [`villarceau`]'s
    /// arc a radian either side of the tangency. Each station's jet read
    /// off a smooth join measures a transverse direction tangent to the
    /// first surface alone, so the per-station sagitta reads definitely
    /// positive in both orders here, at values that differ with the
    /// order; the rule reads the corner first-order instead, and tier 3
    /// holds an edge that is not smooth throughout to neither
    /// description, so no intrinsic tangency is demanded of it.
    #[test]
    fn a_seam_smooth_at_its_witness_and_a_corner_at_a_station_stays_conventional() {
        let (torus, bitangent, arc) = villarceau();
        let span = (-1.0, 1.0);
        assert!(
            matches!(
                seam_class(&torus, &bitangent, arc.eval(span.0), 1.0, band()),
                Ok(geom_brep::DihedralClass::Transverse)
            ),
            "the arc's end is a corner"
        );
        for (a, b) in [(&torus, &bitangent), (&bitangent, &torus)] {
            assert_eq!(
                geom_brep::must_carry_over_edge(a, b, &arc, span.0, span.1, 1.0, band()),
                geom_brep::MustCarryVerdict::Transverse,
                "a station reads the seam a corner"
            );
        }
        for (order, got) in both_orders(&torus, &bitangent, &arc, span, 1.0)
            .into_iter()
            .enumerate()
        {
            assert!(
                matches!(got, Ok(false)),
                "order {order}: a seam that is a corner somewhere keeps the conventional \
                 description, got {got:?}"
            );
        }
    }

    /// **A station whose wedge is in band refuses as `SeamWedge`, even
    /// behind a station that reads a corner.** [`villarceau`]'s arc over
    /// a span short enough that the stations beside the tangency open a
    /// wedge in band while those further out read definitely
    /// transverse: the stations read corner, corner, in band, smooth,
    /// in band, corner, corner. Tier 3 classifies every station and
    /// refuses an in-band one `SliverDihedral` wherever it sits, so the
    /// seam is never stored as either description; a walk that answered
    /// from the first station would keep it conventional.
    #[test]
    fn an_in_band_wedge_behind_a_corner_refuses_as_the_seam_wedge() {
        let (torus, bitangent, arc) = villarceau();
        let extent = 1.0;
        // The wedge's margin, `sin θ` levered over the folded arm, at
        // the arc's `t`: linear in `t` beside the tangency.
        let wedge = |t: f64| {
            let p = arc.eval(t);
            let n = |s: &geom::Surface<f64>| geom_brep::implicit_gradient(s, p).normalize();
            n(&torus).cross(n(&bitangent)).norm()
                * geom_brep::folded_lever_arm(&torus, &bitangent, p, extent)
        };
        let slope = wedge(1e-4) / 1e-4;
        // The stations sit a quarter of the half-span apart: the inner
        // two read 0.7 of the band's escalation edge, the next 1.4.
        let half = 4.0 * 0.7 * band().escalate() / slope;
        let span = (-half, half);
        let classes: Vec<_> = (1..geom_brep::CERT_SAMPLES - 1)
            .map(|i| {
                let p = arc.eval(geom_brep::sample_param(span.0, span.1, i));
                match geom_brep::classify_dihedral(&torus, &bitangent, p, extent, band()) {
                    Ok(geom_brep::DihedralClass::Transverse) => 'T',
                    Ok(geom_brep::DihedralClass::Smooth) => 'S',
                    Err(_) => 'E',
                }
            })
            .collect();
        assert_eq!(
            classes.iter().collect::<String>(),
            "TTESETT",
            "the fixture's stations (half-span {half:e})"
        );
        for (a, b) in [(&torus, &bitangent), (&bitangent, &torus)] {
            let verdict =
                geom_brep::must_carry_over_edge(a, b, &arc, span.0, span.1, extent, band());
            assert!(
                matches!(
                    verdict,
                    geom_brep::MustCarryVerdict::InBand(
                        geom_brep::MustCarryEscalation::FirstOrder(geom_brep::LeverEscalation {
                            rung: geom_brep::LeverRung::Reading,
                            ..
                        })
                    )
                ),
                "an in-band wedge anywhere escalates, got {verdict:?}"
            );
        }
        for (order, got) in both_orders(&torus, &bitangent, &arc, span, extent)
            .into_iter()
            .enumerate()
        {
            assert!(
                matches!(
                    got,
                    Err(BooleanError::Escalated {
                        decision: BooleanDecision::SeamWedge,
                        ..
                    })
                ),
                "order {order}: the in-band wedge refuses as the seam's wedge, got {got:?}"
            );
        }
    }

    /// **The rebuild's smooth arm decides through [`seam_must_carry`]
    /// and spells no second-order reading of its own**, so the rows
    /// above, which call the helper, speak for the boolean's seams: a
    /// loop inlined back into `describe_minted_edges` reds here.
    #[test]
    fn the_smooth_seam_arm_routes_through_the_must_carry_rule() {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/boolean/ops.rs");
        let source = test_utils::source::code_only(&std::fs::read_to_string(path).unwrap());
        let start = source
            .find("fn describe_minted_edges")
            .expect("the rebuild's description pass");
        let body = &source[start..start + source[start..].find("\n}\n").expect("its end")];
        assert_eq!(
            body.matches("seam_must_carry(").count(),
            1,
            "the smooth arm asks the rule once"
        );
        for spelling in [
            "tangent_jet",
            "tangent_second_order",
            "must_carry_over_edge",
            "Margin::sagitta",
            "lever_arm(",
            "decide(",
        ] {
            assert!(
                !body.contains(spelling),
                "describe_minted_edges spells `{spelling}` beside the rule"
            );
        }
    }
}
