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
//! - [`Assembly`](BooleanResultKind::Assembly): a multi-shell body
//!   combining components of both operands without a seam — the
//!   disjoint union (∪ of separated bodies), including
//!   touching-at-declared-contacts assemblies (the carried
//!   [`ContactRecords`] say where; genuinely 3′, certified by PR 6's
//!   validator).
//! - [`Voided`](BooleanResultKind::Voided): **legitimate voids** —
//!   A∖B with B strictly inside A yields the outer shell plus the
//!   reverted inner shell, a tier-2-legal multi-shell body. The
//!   insertion itself is [`super::voids::insert_void`] — the shared
//!   void-insertion door every cavity is born through (this fallback,
//!   the holed full revolve, and `shell`'s sealed hollow), with this
//!   fallback's probe verdicts as the door's containment evidence.
//!
//! When operand boundaries do not intersect, classification falls back
//! to per-shell vertex-in-solid containment
//! ([`point_in_solid`], F8's ray
//! design promoted to 3-D), probing non-contact vertices against the
//! pristine other operand.
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
//! `point_in_loop`); interior-rest flush contacts (pillar standing on
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
//! - **Reflex-corner-vertex tilted crossings** (PR 5.5 review): a
//!   seam through the VERTEX of a reflex boundary corner under a
//!   tilted section plane (a 315°-corner pierced by a z-sheared
//!   brick's cap) can refuse `SeamOrientation`. Root cause: the
//!   angular strut spike order (`bool_strut_order`) is FORCED only on
//!   sectors of width W ≤ π (which covers the whole crossing-minted
//!   corpus class — edge-interior sites are exact half-planes);
//!   reflex corners W > 3π/2 with germ angle θ ∈ (π/2, W−π) sit in
//!   the unforced window. Face-interior and convex-corner crossings
//!   of the same shape succeed exactly.

use geom_core::{Band, Bounds, Decide, Margin, Point3, Real, Sign, Tol, Vec3};

use std::collections::{BTreeMap, BTreeSet};

use super::boxes;
use super::combine::{GraftMap, graft_solid};
use super::contain::{ContainError, FaceContainment, contfp};
use super::finish::{contact_skip_set, kept_side, setopfinish};
use super::join::bool_connect;
use super::solid_contain::{
    PointInSolidError, SolidContainment, closed_sphere_group, point_in_solid,
};
use super::voids;
use super::zip::zip_seam;
use super::{
    BooleanDeclarations, BooleanError, BooleanOp, BooleanReduction, CarriedContacts,
    ContactRecords, CurveContact, FacePairDeclaration, Operand, PatchContact, SideCode,
    SweepStrategy, VfContact, VvContact,
};
use crate::body::Body;
use crate::entity::{EdgeKey, FaceKey, LoopBoundary, ShellKey, VertexKey};
use crate::geometry::SurfaceKey;
use crate::splitting::finish::{carve, single_solid};
use crate::validate::{decide, validate, validate_closed};

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
    /// without a seam (disjoint or touching-only).
    Assembly,
    /// A∖B with B inside A: outer shell + reverted inner void shell.
    Voided,
}

/// A real (non-empty) boolean result.
///
/// # Validity-class carriage (M3 PR 6a, D2 — the F1 contract)
///
/// The validity class rides THIS wrapper, never a mutable field on
/// [`Body`] (validity stays checked-on-demand; raw-insertion
/// disclaimers unchanged): a `BooleanBody` with non-empty `contacts`
/// is **tier-3′-grade currency**, and
/// `validate_pseudomanifold(&b.body, &b.contacts)` is its at-rest
/// gate — the declarations are the machine-checkable record of every
/// intentional touching the pipeline propagated (F2's
/// explicit-intent condition). An empty-contact result remains
/// ordinary tier-3 currency (`validate_geometric`), and on such a
/// body the two gates agree (3′ ≡ tier 3 plus the census actually
/// run — pinned by the PR 6a acceptance suite).
#[derive(Debug)]
pub struct BooleanBody<T: Real> {
    /// The result body: one solid, possibly multi-shell.
    pub body: Body<T>,
    /// How it was produced.
    pub kind: BooleanResultKind,
    /// Declared contacts surviving into the result, in result keys
    /// (module docs) — the tier-3′ declarations (see the type-level
    /// docs: non-empty ⇒ 3′ currency).
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
    /// Zip vertex fusions `(dead, kept)` in zip order, result keys.
    pub vertex_merges: Vec<(VertexKey, VertexKey)>,
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
pub fn union<T: Decide + Bounds + geom_brep::PcurveFittedLane + crate::props::AtRestPolicy>(
    a: &Body<T>,
    b: &Body<T>,
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
pub fn intersect<T: Decide + Bounds + geom_brep::PcurveFittedLane + crate::props::AtRestPolicy>(
    a: &Body<T>,
    b: &Body<T>,
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
pub fn subtract<T: Decide + Bounds + geom_brep::PcurveFittedLane + crate::props::AtRestPolicy>(
    a: &Body<T>,
    b: &Body<T>,
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
pub fn union_with<T: Decide + Bounds + geom_brep::PcurveFittedLane + crate::props::AtRestPolicy>(
    a: &Body<T>,
    b: &Body<T>,
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
pub fn intersect_with<
    T: Decide + Bounds + geom_brep::PcurveFittedLane + crate::props::AtRestPolicy,
>(
    a: &Body<T>,
    b: &Body<T>,
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
pub fn subtract_with<
    T: Decide + Bounds + geom_brep::PcurveFittedLane + crate::props::AtRestPolicy,
>(
    a: &Body<T>,
    b: &Body<T>,
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
pub fn boolean_op_with<
    T: Decide + Bounds + geom_brep::PcurveFittedLane + crate::props::AtRestPolicy,
>(
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
    boolean_op_recut(op, a, b, decls, strategy, true, tol)
}

/// The pipeline behind the front door, parameterized on whether the
/// no-crossings sphere RE-CUT (M5 S13) may still run: the re-entry
/// pass sets `recut = false`, so a re-cut that surfaces no crossings
/// is a loud invariant failure rather than a loop.
fn boolean_op_recut<
    T: Decide + Bounds + geom_brep::PcurveFittedLane + crate::props::AtRestPolicy,
>(
    op: BooleanOp,
    a: &Body<T>,
    b: &Body<T>,
    decls: &BooleanDeclarations,
    strategy: SweepStrategy,
    recut: bool,
    tol: Tol,
) -> Result<BooleanResult<T>, BooleanError> {
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
        //   cylinder-near-sphere, sphere×sphere overlap, tangency,
        //   boundary-grazing circles, one group escaping through
        //   NON-PARALLEL faces): typed refusal — the S12 silence
        //   never re-opens.
        let recuts = sphere_extent_scan(a, b, band)?;
        if !recuts.is_empty() {
            if !recut {
                return Err(BooleanError::ClassificationInvariant {
                    what: "re-cut sphere operands still produced no crossings",
                });
            }
            let (a2, b2) = apply_recuts(a, b, &recuts, tol)?;
            return boolean_op_recut(op, &a2, &b2, decls, strategy, false, tol);
        }
        // The curved kinds the extent scan leaves: every torus,
        // cylinder and cone face's pairs, certified per pair by the
        // section certificate or refused typed.
        section_extent_pass(a, b, band)?;
        return fallback(op, &red, a, b, decls, band, tol);
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
                        Ok(result)
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
    let contacts = red.contacts.clone();
    let reduction_contacts = red.contacts.clone();
    let fin = setopfinish(op, red, &connected.completed, a, b, band, tol)?;
    // The zip, the merge, the re-description and the closing mint are
    // one door's surgery (`crate::surgery`): the operators inside them
    // do not each re-derive the whole body, and `gate` below — tier 1
    // AND tier 2 over the result, on every build — is what this door
    // pays instead. The guard owns the borrow, so a refusal on the way
    // closes the scope by dropping it.
    let mut finished = fin.body;
    let mut body = finished.begin_surgery();
    let mut seam_edges = Vec::new();
    let mut vertex_merges = Vec::new();
    let mut desc = Descendants::default();
    for &(a_face, b_face) in &fin.seams {
        let rep = zip_seam(&mut body, a_face, b_face, &fin.vertex_map, tol)?;
        desc.absorb_zip(&rep);
        vertex_merges.extend(rep.vertex_merges.iter().copied());
        seam_edges.extend(rep.seam_edges);
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
    );
    remap_carried(
        &mut contacts,
        &body,
        decls,
        &KeyView::Direct,
        &KeyView::Graft(&fin.graft),
        &desc,
    );
    // Curved results carry certified per-half-edge pcurves at rest
    // (M5 PR 9, the PR 6 contract): re-derive the whole cache set on
    // the finished body — the same pass the split lane runs. A planar
    // body mints nothing (no curved faces), so the M3 lane is
    // untouched bit-identically.
    crate::pcurves::mint_pcurves(&mut body, tol)
        .map_err(|source| BooleanError::Pcurves { source })?;
    body.sweep_and_close();
    let body = finished;
    gate(&body)?;
    volume_backstop(op, a, b, &body, band, tol)?;
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
        merge_groups: merge_rows(&merged),
        merge_skipped: merged.skipped.clone(),
        face_fragments_a: connected.a_fragments,
        face_fragments_b: connected.b_fragments,
        reduction_contacts,
    };
    Ok(BooleanResult::Body(BooleanBody {
        body,
        kind: BooleanResultKind::Seamed,
        contacts,
        naming,
    }))
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
fn interior_loop_verdict<
    T: Decide + Bounds + geom_brep::PcurveFittedLane + crate::props::AtRestPolicy,
>(
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
    pub(crate) fn describes<T: geom_brep::PcurveFittedLane>(
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
    /// plane, except a sphere against a plane, a sphere, a cylinder or a
    /// spline — those are the extent scan's ([`sphere_extent_scan`]),
    /// which runs first and keeps its re-cut. A sphere against a torus
    /// or a cone is certified here: neither is ever an escape face, so
    /// the scan's only question of the pair is disjointness, and with no
    /// event anywhere "every component cleared" is disjointness.
    Fallback,
}

impl SectionPath {
    fn scope<T: Real>(self, x: &geom::Surface<T>, y: &geom::Surface<T>) -> bool {
        use geom::Surface as S;
        let curved = |s: &geom::Surface<T>| !matches!(s, S::Plane { .. });
        let sphere = |s: &geom::Surface<T>| matches!(s, S::Sphere { .. });
        let passed = |s: &geom::Surface<T>| matches!(s, S::Torus { .. } | S::Cone { .. });
        let scanned = (sphere(x) && !passed(y)) || (sphere(y) && !passed(x));
        (curved(x) || curved(y)) && !(self == Self::Fallback && scanned)
    }

    /// Is `s` a face this path's refusal names?
    fn names<T: Real>(self, s: &geom::Surface<T>) -> bool {
        self.scope(s, s)
    }
}

/// One pair's certificate outcome.
#[derive(Clone, Debug)]
pub(crate) struct PairVerdict {
    /// The pair's face of A.
    pub a_face: FaceKey,
    /// Its kind.
    pub a_kind: geom_brep::SurfaceKind,
    /// The pair's face of B.
    pub b_face: FaceKey,
    /// Its kind.
    pub b_kind: geom_brep::SurfaceKind,
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
        geom_brep::SurfaceKind,
        FaceKey,
        geom_brep::SurfaceKind,
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

/// **The section certificate over every in-scope pair** of `a` × `b`
/// whose certified boxes overlap and which `skip` does not exempt, in
/// arena order. `evented` says whether the reduction recorded an event
/// on the pair `(A face, B face)`. With `stop` the scan returns at the
/// first refusing pair.
///
/// `chart_boundary` is asked once per face and cached; the pair's
/// reach — the ball about the two boxes' overlap, which pivots and
/// levers the angular margins — is built here
/// ([`super::section_cert`]'s module docs).
///
/// # Errors
///
/// [`BooleanError::ClassificationInvariant`] for a face whose surface
/// or loops do not resolve, and the box builder's own errors.
pub(crate) fn section_pairs<T: Decide + Bounds + geom_brep::PcurveFittedLane>(
    a: &Body<T>,
    b: &Body<T>,
    band: Band,
    path: SectionPath,
    skip: impl Fn(FaceKey, FaceKey) -> bool,
    evented: impl Fn(FaceKey, FaceKey) -> bool,
    stop: bool,
) -> Result<Vec<PairVerdict>, BooleanError> {
    use super::section_cert::{Refusal, Side, certify, classify};
    let lost = || BooleanError::ClassificationInvariant {
        what: "section certificate: a face surface is lost",
    };
    let pad = boxes::sweep_pad(band);
    let faces =
        |body: &Body<T>| -> Result<Vec<(FaceKey, geom::Surface<T>, bvh::Aabb)>, BooleanError> {
            body.faces()
                .map(|(k, fd)| {
                    let s = body.get_surface(fd.surface).ok_or_else(lost)?.clone();
                    Ok((k, s, boxes::face_box(body, k, pad)?))
                })
                .collect()
        };
    let (a_faces, b_faces) = (faces(a)?, faces(b)?);
    let mut charts = ChartCache::default();
    let mut out = Vec::new();
    for (fa, sa, box_a) in &a_faces {
        for (fb, sb, box_b) in &b_faces {
            if !path.scope(sa, sb) || !box_a.overlaps(box_b) || skip(*fa, *fb) {
                continue;
            }
            let verdict = if has_lone_vertex(a, *fa)? || has_lone_vertex(b, *fb)? {
                Err(Refusal::LoneVertex)
            } else {
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
                let section = classify(sa, sb, reach, band);
                certify(
                    &section,
                    evented(*fa, *fb),
                    |side| {
                        let (operand, body, face, surface) = match side {
                            Side::F => (Operand::A, a, *fa, sa),
                            Side::G => (Operand::B, b, *fb, sb),
                        };
                        charts.describes(operand, body, face, surface, band)
                    },
                    |p| {
                        [
                            place_witness(a, *fa, sa, p, band),
                            place_witness(b, *fb, sb, p, band),
                        ]
                    },
                )
            };
            let refused = verdict.is_err();
            out.push(PairVerdict {
                a_face: *fa,
                a_kind: geom_brep::SurfaceKind::of(sa),
                b_face: *fb,
                b_kind: geom_brep::SurfaceKind::of(sb),
                a_named: path.names(sa),
                verdict,
            });
            if stop && refused {
                return Ok(out);
            }
        }
    }
    Ok(out)
}

/// **The no-crossings path's section pass.** With no event anywhere,
/// W4 never fires and the no-event decision decides every lone
/// component, so each pair the vertex probe could not see into is
/// either certified or refused here, typed as the fallback's extent
/// refusal ([`BooleanError::FallbackExtentUnsupported`]) naming the
/// pair's curved face. Declarations exempt nothing on this path: with
/// no crossings, a declared coincident pair is exactly what the vertex
/// probe cannot decide.
fn section_extent_pass<T: Decide + Bounds + geom_brep::PcurveFittedLane>(
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
pub(crate) fn section_report<
    T: Decide + Bounds + geom_brep::PcurveFittedLane + crate::props::AtRestPolicy,
>(
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
) -> Result<(Sign, T), geom_core::Indeterminate> {
    let s = (center - origin).dot(normal);
    let sign = decide("bool_sphere_extent_gap", Margin::of(radius - s.abs()), band)?;
    Ok((sign, s))
}

/// The (A face, B face) pairs the reduction recorded an event on: a
/// vertex-on-face contact pairs every face the vertex bounds with the
/// face it lies on, and a vertex-on-vertex contact pairs every face
/// each vertex bounds.
fn event_pairs<T: Real>(
    red: &BooleanReduction<T>,
) -> Result<BTreeSet<(FaceKey, FaceKey)>, BooleanError> {
    let a_faces = faces_by_vertex(&red.a)?;
    let b_faces = faces_by_vertex(&red.b)?;
    let around = |m: &BTreeMap<VertexKey, Vec<FaceKey>>, v: VertexKey| {
        m.get(&v).cloned().unwrap_or_default()
    };
    let mut out = BTreeSet::new();
    for c in &red.contacts.a_on_b {
        for fa in around(&a_faces, c.vertex) {
            out.insert((fa, c.face));
        }
    }
    for c in &red.contacts.b_on_a {
        for fb in around(&b_faces, c.vertex) {
            out.insert((c.face, fb));
        }
    }
    for c in &red.contacts.vv {
        for fa in around(&a_faces, c.a) {
            for fb in around(&b_faces, c.b) {
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
/// vol(∩) ≤ min(vol A, vol B), vol(∪) ≥ max(vol A, vol B),
/// vol(∖) ≤ vol A — computed with the exact planar
/// [`crate::mass_properties`]. The min/max are decomposed into per-operand
/// inequalities, so no operand-vs-operand comparison is needed.
///
/// Comparison posture: each bound margin is classified through the
/// k_stats funnel (`decide` — the certified trilean against the op's
/// linear band, under this gate's own predicate name), the codebase's
/// only legal comparison (Q1). Only a CERTIFIED violating sign
/// refuses ([`BooleanError::ResultVolumeImplausible`]);
/// `Zero` and in-band indeterminate margins PASS: on the planar
/// corpus the flux sums are exact for dyadic fixtures (margin exactly
/// 0 or macroscopic), and the bug class this backstop guards —
/// wrong-component results — violates its bound by whole regions, so
/// refusing on an ulp-scale tie would make the gate noisier than the
/// property it guards. A POISONED margin (NaN) still refuses loudly
/// ([`BooleanError::Escalated`]) — poison never passes a gate.
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
///    boundary the defect is smeared over. This arm decides against the
///    **exact (bit-hairline) band**, where "certain" means "proven
///    beyond the enclosure's own width": at `f64` any nonzero negative,
///    at the interval scalar an enclosure entirely below the hairline
///    (a straddling enclosure escalates and falls through to arm 2).
///    No ε enters, so no dimensional claim is made — this is a sign,
///    not a comparison against a length. Predicate:
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
/// The gate is therefore strictly stronger than BOTH of its
/// predecessors: it refuses every violation the raw-m³ comparand
/// refused (arm 1 subsumes it) AND runs on the mm-scale operands the
/// raw comparand silently skipped (see `bounded`'s note).
///
/// Both gates decide through the k_stats funnel under those names.
/// They used to call [`Decide::sign_within`] RAW — the kernel's only
/// funnel bypass — which never set the recorder's current predicate
/// name, so on the recording lane these volumes were attributed to
/// whichever predicate decided last (measured in
/// `topo/tests/rim_dim_boolean_twins.rs` at ε = 1e-12: the
/// operand/result volume set {1, 1, 3, 8, 8, 16} m³ logged under
/// certify's `witness_at_mid_parameter`, scaling ×1e-9 — cubic).
/// Routing through `decide` retires that misattribution.
pub(super) fn volume_backstop<T: Decide>(
    op: BooleanOp,
    a: &Body<T>,
    b: &Body<T>,
    result: &Body<T>,
    band: Band,
    tol: Tol,
) -> Result<(), BooleanError> {
    let corrupt = || BooleanError::ClassificationInvariant {
        what: "volume backstop: mass properties refused on a tier-valid planar body",
    };
    // Closed-form lane on purpose (M5 PR 11 lane split): this is the
    // boolean engine's INTERNAL invariant backstop on its own planar/
    // iso results — the certified quadrature lane is the at-rest
    // measurement door, and a trimmed face here keeps the historical
    // fail-loud refusal.
    // Volume AND surface area: the area is this gate's metering lever
    // (fn docs, audit F3), read from the same closed-form pass.
    let props = |body: &Body<T>| -> Result<(T, T), BooleanError> {
        let p =
            crate::props::mass_properties_closed_form(body, band, tol).map_err(|_| corrupt())?;
        Ok((p.volume, p.surface_area))
    };
    // The exact (bit-hairline) band for the sign arm below — the same
    // device the splitter's total order uses (`splitting::order`, audit
    // note N6): its open interior holds no representable `f64`, so at
    // `f64` a sign is certain iff the margin is not exactly zero, and at
    // the interval scalar an enclosure straddling the hairline escalates
    // honestly. That IS "proven beyond the enclosure's own width".
    let exact = crate::splitting::order::exact_band()?;
    let ((va, aa), (vb, ab), (vr, ar)) = (props(a)?, props(b)?, props(result)?);
    // A bound applies only against a certified-bounded operand
    // (positive flux volume); complement operands (certified negative)
    // make it vacuous. Poison refuses. Metered `V/A` — the operand's
    // MEAN THICKNESS (fn docs). Surface area is unsigned (props.rs), so
    // the quotient keeps V's sign and the complement arm is unchanged.
    // An in-band quotient is an operand thinner than the model's own
    // resolution — no bound is certifiable from it, skip. (Pre-F3 this
    // read "an in-band operand VOLUME", and that is exactly the defect
    // the audit's F3 row records: a raw m³ against the linear band put
    // ordinary mm-scale operands in the band and switched their bound
    // checks off. The skip zone survives, now meaning sub-resolution
    // thickness — which is what it always claimed to mean.)
    // The backstops live on the INVARIANT LANE (Ev's #213 layering
    // ruling): consistency inequalities between integral results are
    // outside the length seam by design — no door, bare T — and a
    // certified violation is a kernel invariant failure, not a
    // validity refusal. Values and predicate names are unchanged.
    let bounded = |v: T, area: T| -> Result<bool, BooleanError> {
        match geom_core::k_stats::decide_invariant("volume_backstop_operand", v / area, band) {
            Ok(Sign::Positive) => Ok(true),
            Ok(Sign::Zero | Sign::Negative) => Ok(false),
            Err(diag) if diag.margin.is_invalid() => Err(BooleanError::Escalated { diag }),
            Err(_) => Ok(false),
        }
    };
    // `margin` ≥ 0 (within band) or the bound named by `which` is
    // violated: margin = bound − got for upper bounds, got − bound for
    // lower bounds. `lever` is the two compared bodies' summed surface
    // area, which turns the volume defect into a boundary displacement
    // (fn docs). TWO ARMS, in order — see the fn docs' "Two questions,
    // two bands": the SIGN question (is the inequality violated at all?)
    // against the exact band, then the MAGNITUDE question (is the
    // displacement above the model's resolution?) against ε.
    let check =
        |which: &'static str, margin: T, got: T, bound: T, lever: T| -> Result<(), BooleanError> {
            let implausible = || BooleanError::ResultVolumeImplausible {
                which,
                got: format!("{got:?}"),
                bound: format!("{bound:?}"),
            };
            let metered = margin / lever;
            // Arm 1 — the inequality itself. A sign-certain violation is
            // a violated bound whatever its size, so nothing about ε
            // enters here.
            if geom_core::k_stats::decide_invariant("volume_backstop_violation", metered, exact)
                == Ok(Sign::Negative)
            {
                return Err(implausible());
            }
            // Arm 2 — the magnitude, for the near-zero region arm 1
            // leaves open. Unchanged posture: only a certified negative
            // refuses (unreachable now, since arm 1 subsumes it — kept
            // as the honest statement of the gate rather than a dead
            // arm removed), Zero and in-band PASS, poison refuses.
            match geom_core::k_stats::decide_invariant("volume_backstop", metered, band) {
                Ok(Sign::Negative) => Err(implausible()),
                Ok(Sign::Zero | Sign::Positive) => Ok(()),
                Err(diag) if diag.margin.is_invalid() => Err(BooleanError::Escalated { diag }),
                Err(_) => Ok(()),
            }
        };
    let (ba, bb) = (bounded(va, aa)?, bounded(vb, ab)?);
    match op {
        BooleanOp::Intersect => {
            if ba {
                check("vol(A ∩ B) ≤ vol(A)", va - vr, vr, va, aa + ar)?;
            }
            if bb {
                check("vol(A ∩ B) ≤ vol(B)", vb - vr, vr, vb, ab + ar)?;
            }
        }
        BooleanOp::Union => {
            if ba {
                check("vol(A ∪ B) ≥ vol(A)", vr - va, vr, va, aa + ar)?;
            }
            if bb {
                check("vol(A ∪ B) ≥ vol(B)", vr - vb, vr, vb, ab + ar)?;
            }
        }
        BooleanOp::Subtract => {
            if ba {
                check("vol(A ∖ B) ≤ vol(A)", va - vr, vr, va, aa + ar)?;
            }
        }
    }
    Ok(())
}

/// D6 (M3 PR 6a): honest descriptions on boolean-minted edges AT MINT
/// TIME — the worklist is tracked lineage (the zips' surviving seam
/// edges plus every boundary edge of a merge-kept face, whose
/// adjacency the merge just rewrote), never a post-hoc scan of the
/// body. Each worklist edge that still resolves is described from its
/// two faces' surfaces (structural adjacency): definitely transverse ⇒
/// `Intersection` with the chord-midpoint witness; definitely smooth ⇒
/// the existing conventional description stays (D2's split — the
/// surfaces under-determine the locus); escalation refuses typed.
pub(super) fn describe_minted_edges<T: Decide>(
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
    for edge in worklist {
        let edge_data = body.get_edge(edge).ok_or_else(corrupt)?.clone();
        let face_of = |body: &Body<T>, he| -> Option<crate::geometry::SurfaceKey> {
            Some(body.get_face(body.face_of_half_edge(he)?)?.surface)
        };
        let (Some(s1), Some(s2)) = (
            face_of(body, edge_data.he_plus),
            face_of(body, edge_data.he_minus),
        ) else {
            return Err(corrupt());
        };
        let start = body
            .get_half_edge(edge_data.he_plus)
            .ok_or_else(corrupt)?
            .start;
        let end = body.half_edge_end(edge_data.he_plus).ok_or_else(corrupt)?;
        let p0 = *body
            .get_point(body.get_vertex(start).ok_or_else(corrupt)?.point)
            .ok_or_else(corrupt)?;
        let p1 = *body
            .get_point(body.get_vertex(end).ok_or_else(corrupt)?.point)
            .ok_or_else(corrupt)?;
        let (Some(surf1), Some(surf2)) = (body.get_surface(s1), body.get_surface(s2)) else {
            return Err(corrupt());
        };
        // Curved seam edges (M5 PR 9) keep their minted conic carrier
        // and pin the witness at the carrier's mid parameter (the S2
        // contract); planar chords keep the M3 line lane bit-
        // identically (fresh chord carrier, lerp witness).
        let existing = body
            .get_curve_geom(edge_data.curve)
            .and_then(crate::null::CurveGeom::certified)
            .cloned();
        let curved = existing
            .as_ref()
            .is_some_and(|c| !matches!(c.carrier(), geom::Curve3::Line { .. }));
        let (witness, extent) = if curved {
            let c = existing.as_ref().ok_or_else(corrupt)?;
            let (t0, t1) = c.params();
            let mid = c.carrier().eval(t0 + (t1 - t0) * T::from_f64(0.5));
            (
                mid,
                geom_brep::edge_extent(c.carrier(), t0, t1, p0.distance(p1)),
            )
        } else {
            (p0.lerp(p1, T::from_f64(0.5)), p0.distance(p1))
        };
        match geom_brep::classify_dihedral(surf1, surf2, witness, extent, band) {
            Ok(geom_brep::DihedralClass::Transverse) => {
                let spec = if curved {
                    let c = existing.as_ref().ok_or_else(corrupt)?;
                    let (t0, t1) = c.params();
                    geom_brep::EdgeCurveSpec {
                        description: geom_brep::EdgeDescriptionSpec::Intersection {
                            s1,
                            s2,
                            witness,
                        },
                        carrier: c.carrier().clone(),
                        param_start: t0,
                        param_end: t1,
                    }
                } else {
                    let mut spec = geom_brep::EdgeCurveSpec::line_between(p0, p1);
                    spec.description =
                        geom_brep::EdgeDescriptionSpec::Intersection { s1, s2, witness };
                    spec
                };
                body.set_edge_curve(edge, spec, tol)
                    .map_err(|_| BooleanError::JoinDesync {
                        what: "minted-edge description failed certification",
                    })?;
            }
            Ok(geom_brep::DihedralClass::Smooth) => {
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
                        !((*d1 == s1 && *d2 == s2) || (*d1 == s2 && *d2 == s1))
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
                    geom_brep::EdgeDescription::Scaffold(_) => false,
                };
                // The D6 smooth ladder (M9-3): a definitely-smooth
                // seam descends one order, exactly as the tier-3
                // contact mark does — the jet's second-order margin at
                // the same interior schedule (rows
                // `tangent_second_order`, reused). Determinate at
                // every sample ⇒ the surfaces DETERMINE the locus and
                // the intrinsic `TangentIntersection` is minted (the
                // must-carry's own regime) — a G1 rim's line ruling
                // included; a zero-side or in-band sample keeps the
                // CONVENTIONAL posture (tier 3's ratified
                // `SmoothUnderdetermined` stance — coplanar planes'
                // exact-zero jet lands here, so every planar split
                // keeps its chord description bit-identically; the
                // weaker description is never a lie, and ε-tightening
                // never flips a valid body through this choice).
                let jet_determinate = {
                    let c = existing.as_ref().ok_or_else(corrupt)?;
                    let (t0, t1) = c.params();
                    let mut det = true;
                    for i in 1..(geom_brep::CERT_SAMPLES - 1) {
                        let t = geom_brep::sample_param(t0, t1, i);
                        let (p, tau) = c.carrier().ders1(t);
                        let jet = geom_brep::tangent_jet(surf1, surf2, p, tau);
                        let arm = geom_brep::curvature_lever_arm(surf1, p)
                            .min(geom_brep::curvature_lever_arm(surf2, p))
                            .min(extent);
                        match decide(
                            "tangent_second_order",
                            Margin::sagitta(jet.kappa_rel.abs(), arm),
                            band,
                        ) {
                            Ok(Sign::Positive) => {}
                            _ => {
                                det = false;
                                break;
                            }
                        }
                    }
                    det
                };
                if jet_determinate {
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
            Err(diag) => return Err(BooleanError::Escalated { diag }),
        }
    }
    Ok(())
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
/// entity replacement — seam-zip vertex fusions and
/// `merge_coplanar_faces` face absorption — as old key → surviving
/// key rows, extending the graft's key lineage so a contact record
/// drops ONLY when its coincidence is consumed (entity gone, not
/// renamed). Re-derivation at the 3′ gate is rejected as
/// scan-to-bless (F1); the descendants ARE the mint-time knowledge.
#[derive(Default)]
pub(super) struct Descendants {
    vertices: std::collections::BTreeMap<VertexKey, VertexKey>,
    faces: std::collections::BTreeMap<FaceKey, FaceKey>,
    /// Every vertex that participated in a zip fusion (dead OR kept):
    /// its point rests were consumed into seam structure.
    fused: std::collections::BTreeSet<VertexKey>,
}

impl Descendants {
    pub(super) fn absorb_zip(&mut self, rep: &super::zip::ZipReport) {
        for &(dead, kept) in &rep.vertex_merges {
            self.vertices.insert(dead, kept);
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

    /// Chases a vertex key through the fusion rows until it resolves
    /// live (bounded by the map size — rows never cycle: a dead key
    /// maps to its survivor).
    fn live_vertex<T: Real>(&self, body: &Body<T>, v: VertexKey) -> Option<VertexKey> {
        let mut k = v;
        for _ in 0..=self.vertices.len() {
            if body.get_vertex(k).is_some() {
                return Some(k);
            }
            k = *self.vertices.get(&k)?;
        }
        None
    }

    /// Chases a face key through the absorption rows until live.
    fn live_face<T: Real>(&self, body: &Body<T>, f: FaceKey) -> Option<FaceKey> {
        let mut k = f;
        for _ in 0..=self.faces.len() {
            if body.get_face(k).is_some() {
                return Some(k);
            }
            k = *self.faces.get(&k)?;
        }
        None
    }
}

/// Remaps the declared contacts into result keys — operand views
/// first (graft lineage), then the D5 descendant chase — dropping
/// records only when the entity is genuinely consumed (module docs).
pub(super) fn remap_contacts<T: Real>(
    body: &Body<T>,
    contacts: &ContactRecords,
    a_view: KeyView<'_>,
    b_view: KeyView<'_>,
    desc: &Descendants,
) -> ContactRecords {
    // v-v pairs chase through zip fusions (a fused vertex's partner
    // may still coincide with the survivor); a pair fused into ONE
    // vertex is consumed (structural now) and drops.
    let vert = |view: &KeyView<'_>, v: VertexKey| desc.live_vertex(body, view.vertex(v)?);
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
    let face = |view: &KeyView<'_>, f: FaceKey| desc.live_face(body, view.face(f)?);
    let mut out = ContactRecords::default();
    for c in &contacts.vv {
        if let (Some(a), Some(b)) = (vert(&a_view, c.a), vert(&b_view, c.b))
            && a != b
        {
            out.vv.push(VvContact { a, b });
        }
    }
    for c in &contacts.a_on_b {
        if let (Some(vertex), Some(face)) = (vert_strict(&a_view, c.vertex), face(&b_view, c.face))
        {
            out.a_on_b.push(VfContact { vertex, face });
        }
    }
    for c in &contacts.b_on_a {
        if let (Some(vertex), Some(face)) = (vert_strict(&b_view, c.vertex), face(&a_view, c.face))
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
            face(&a_view, c.face_a),
            face(&b_view, c.face_b),
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
        if let (Some(face_a), Some(face_b)) = (face(&a_view, c.face_a), face(&b_view, c.face_b)) {
            out.patches.push(PatchContact { face_a, face_b });
        }
    }
    out
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
                // Only the CONFORMAL class declares a merge-stage
                // coincidence; a `Tangent` pair's carriers are DISTINCT
                // by its own verification and never merge.
                if class != crate::contact::ContactClass::Rest {
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
pub(super) fn remap_carried<T: Real>(
    out: &mut ContactRecords,
    body: &Body<T>,
    decls: &BooleanDeclarations,
    a_view: &KeyView<'_>,
    b_view: &KeyView<'_>,
    desc: &Descendants,
) {
    let vert = |view: &KeyView<'_>, v: VertexKey| desc.live_vertex(body, view.vertex(v)?);
    let vert_strict = |view: &KeyView<'_>, v: VertexKey| {
        let k = view.vertex(v)?;
        if desc.fused.contains(&k) {
            return None;
        }
        body.get_vertex(k).map(|_| k)
    };
    let face = |view: &KeyView<'_>, f: FaceKey| desc.live_face(body, view.face(f)?);
    let push_vv = |out: &mut ContactRecords, carried: &CarriedContacts, view: &KeyView<'_>| {
        for c in &carried.vv {
            if let (Some(a), Some(b)) = (vert(view, c.pair.a), vert(view, c.pair.b))
                && a != b
                && !out
                    .vv
                    .iter()
                    .any(|r| (r.a, r.b) == (a, b) || (r.a, r.b) == (b, a))
            {
                out.vv.push(VvContact { a, b });
            }
        }
    };
    push_vv(out, &decls.carried_a, a_view);
    push_vv(out, &decls.carried_b, b_view);
    let dup_vf = |out: &ContactRecords, v: VertexKey, f: FaceKey| {
        out.a_on_b
            .iter()
            .chain(&out.b_on_a)
            .any(|r| (r.vertex, r.face) == (v, f))
    };
    for c in &decls.carried_a.vf {
        if let (Some(vertex), Some(fk)) = (
            vert_strict(a_view, c.rest.vertex),
            face(a_view, c.rest.face),
        ) && !dup_vf(out, vertex, fk)
        {
            out.a_on_b.push(VfContact { vertex, face: fk });
        }
    }
    for c in &decls.carried_b.vf {
        if let (Some(vertex), Some(fk)) = (
            vert_strict(b_view, c.rest.vertex),
            face(b_view, c.rest.face),
        ) && !dup_vf(out, vertex, fk)
        {
            out.b_on_a.push(VfContact { vertex, face: fk });
        }
    }
}

/// The tier gates: tier 1 + tier 2 on the finished result (tier 3 is
/// an at-rest posture with the PR 3 description gap — see the
/// acceptance suite's documented posture).
pub(super) fn gate<T: Real>(body: &Body<T>) -> Result<(), BooleanError> {
    validate(body).map_err(|errors| BooleanError::ResultInvalid { errors })?;
    validate_closed(body).map_err(|errors| BooleanError::ResultInvalid { errors })?;
    Ok(())
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
///   plane face is repairable by a re-chart.
/// - **Torus, cylinder and cone**: no closed-group extent exists, so
///   their pairs are certified per pair by the section certificate
///   ([`section_extent_pass`]), which runs after this scan. A sphere's
///   pairs with a torus or cone face are the pass's too: neither is an
///   escape face, so disjointness is all the scan would ask of them.
///
/// Determinism (D9): face-arena order throughout; the first escape's
/// normal is the alignment target.
fn sphere_extent_scan<T: Decide + Bounds>(
    a: &Body<T>,
    b: &Body<T>,
    band: Band,
) -> Result<Vec<SphereRecut<T>>, BooleanError> {
    let esc = |diag| BooleanError::Escalated { diag };
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
    let mut out: Vec<SphereRecut<T>> = Vec::new();
    for (x_is, x, y) in [(Operand::A, a, b), (Operand::B, b, a)] {
        // The scope is the whole operand: what the escape arm re-charts
        // is the sphere itself, which every wearer shares.
        let charts = crate::chart_groups::ChartGroups::within(x, x.faces().map(|(k, _)| k))
            .map_err(|_| BooleanError::JoinDesync {
                what: "sphere recut: an operand face does not resolve",
            })?;
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
            for (yf, yfd) in y.faces() {
                match y.get_surface(yfd.surface) {
                    Some(&geom::Surface::Plane {
                        origin,
                        normal,
                        u_ref,
                    }) => {
                        let (side, s) = ball_against_plane(center, radius, origin, normal, band)
                            .map_err(esc)?;
                        match side {
                            // Clear of the whole carrier plane.
                            Sign::Negative => {}
                            // Tangency: a touching configuration the
                            // crossing layer cannot represent — typed
                            // (its in-band twin escalates above).
                            Sign::Zero => {
                                return Err(BooleanError::FallbackExtentUnsupported {
                                    operand: x_is,
                                    face,
                                    what: "the sphere is exactly tangent to a plane face's \
                                           carrier — a touching configuration, the typed \
                                           frontier of the supported envelope",
                                });
                            }
                            Sign::Positive => {
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
                                    ContainError::Escalated(diag) => {
                                        BooleanError::Escalated { diag }
                                    }
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
                                    ContainError::ArcLoopUnsupported { r#loop } => {
                                        BooleanError::ArcLoopContainmentUnsupported {
                                            operand: x_is,
                                            r#loop,
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
                    Some(geom::Surface::Cylinder { .. }) => {
                        // No exact sphere-vs-cylinder-face certificate
                        // is wired: the cyl×sphere lane is PR 9c
                        // deviation 1, and since M6-2 its blocker is
                        // the unwired JOIN lane alone — the generic
                        // lift and Pcurve::Fitted both landed there.
                        // The exact DECLARED-coaxial classification
                        // does not retire this and the message is
                        // re-verified rather than moved: this scan asks
                        // about NEARNESS between two arbitrary trimmed
                        // faces, which no coaxial section answers, and
                        // it has no declaration channel to reach one
                        // through in any case. Certified boxes prove
                        // separation, anything closer refuses typed.
                        if boxes::face_box(y, yf, pad)?.overlaps(&ball_box) {
                            return Err(BooleanError::FallbackExtentUnsupported {
                                operand: x_is,
                                face,
                                what: "the sphere's certified extent meets a cylinder \
                                       face's box — the cyl×sphere seam lane is not \
                                       wired (its fitted-chord window has no azimuth \
                                       analog), so nearness cannot be classified",
                            });
                        }
                    }
                    Some(&geom::Surface::Sphere {
                        center: c2,
                        radius: r2,
                        ..
                    }) => {
                        let d = (c2 - center).norm();
                        match decide(
                            "bool_sphere_sphere_gap",
                            Margin::of(d - (radius + r2)),
                            band,
                        )
                        .map_err(esc)?
                        {
                            // Definitely separated.
                            Sign::Positive => {}
                            Sign::Zero | Sign::Negative => {
                                // Nested (one strictly inside the
                                // other) is boundary-disjoint too;
                                // anything else is the sphere×sphere
                                // seam frontier.
                                let big = radius.max(r2);
                                let small = radius.min(r2);
                                match decide(
                                    "bool_sphere_sphere_nested",
                                    Margin::of(big - (d + small)),
                                    band,
                                )
                                .map_err(esc)?
                                {
                                    Sign::Positive => {}
                                    Sign::Zero | Sign::Negative => {
                                        return Err(BooleanError::FallbackExtentUnsupported {
                                            operand: x_is,
                                            face,
                                            what: "two sphere boundaries meet (neither \
                                                   separated nor strictly nested) — the \
                                                   sphere×sphere section is the exact \
                                                   closed-form Circle and the germ frame \
                                                   names it, but the JOIN has no arm for a \
                                                   curved×curved germ pair: its arc-side \
                                                   rule needs a chart the pair does not \
                                                   have, and a crossing found here would \
                                                   pierce a curved face first",
                                        });
                                    }
                                }
                            }
                        }
                    }
                    Some(geom::Surface::Nurbs(_)) => {
                        // Unreachable: the re-gate above runs first.
                        return Err(BooleanError::NurbsExtentUnsupported {
                            operand: x_is.other(),
                            face: yf,
                        });
                    }
                    // A cone or torus face is never an escape plane, so
                    // the pair's one question is disjointness, which the
                    // section pass certifies (`SectionPath::Fallback`).
                    Some(geom::Surface::Cone { .. } | geom::Surface::Torus { .. }) => {}
                    // `Approx` joins the no-wired-arm refusal, not the
                    // NURBS lane: the pair-scoped operand gate refuses
                    // it by kind before this scan runs.
                    Some(geom::Surface::Approx(_)) => {
                        // REACH FIRST, kind second. This arm asks
                        // whether the ball can escape past THIS face;
                        // a face whose box cannot meet the ball's
                        // certified extent bounds no escape route
                        // through it, and its kind is then no more
                        // relevant here than it is at the operand
                        // gate. Only a face the ball may actually
                        // reach costs the operation its answer.
                        if !boxes::face_box(y, yf, pad)?.overlaps(&ball_box) {
                            continue;
                        }
                        return Err(BooleanError::CurvedBooleanUnsupported {
                            operand: x_is.other(),
                            face: yf,
                            kind: geom_brep::SurfaceKind::of(y.get_surface(yfd.surface).ok_or(
                                BooleanError::ClassificationInvariant {
                                    what: "extent scan: face surface lost",
                                },
                            )?),
                        });
                    }
                    None => {
                        return Err(BooleanError::ClassificationInvariant {
                            what: "extent scan: face surface lost",
                        });
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
                    .map_err(esc)?
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

/// Applies the scan's re-cuts: each escaping group's shell is carved
/// out, rigidly rotated about the sphere's own center so the stored
/// polar axis lands on the escape normal (the same point set — a
/// sphere is rotation-invariant about its center — with the seam
/// meridians now transverse to the escape planes), and grafted back.
fn apply_recuts<T: Decide + Bounds + geom_brep::PcurveFittedLane + crate::props::AtRestPolicy>(
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
            // Rotation source → target: definite by construction — an
            // ALIGNED yet crossing-free escape is a graze the crossing
            // layer must have seen, so it refuses loudly instead.
            let cross = r.axis.cross(r.align);
            let sin = cross.norm();
            match decide(
                "bool_sphere_recut_align",
                Margin::levered(sin, r.radius),
                band,
            )
            .map_err(|diag| BooleanError::Escalated { diag })?
            {
                Sign::Positive | Sign::Negative => {}
                Sign::Zero => {
                    return Err(BooleanError::FallbackExtentUnsupported {
                        operand,
                        face: r.representative,
                        what: "the sphere chart's polar axis is already aligned with the \
                               escape normal yet the crossing layer saw no event — a \
                               grazing/contact configuration",
                    });
                }
            }
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
                    graft_solid(base, base_solid, &turned, tol)?;
                }
            }
        }
        out.adopt(rebuilt.ok_or(corrupt("re-cut produced no body"))?);
    }
    Ok((out_a, out_b))
}

/// Per-shell classification of one operand's clone against the other
/// pristine operand (containment fallback; contact vertices skipped,
/// `OnBoundary` probes advanced past).
///
/// **The vertex probe below is the WITNESS, not the certificate**
/// (M5 S13). A curved boundary can leave the other solid strictly
/// between its vertices (the S12 finding), so for the sphere class
/// the answer is only sound because [`sphere_extent_scan`] ran first
/// and certified every sphere-involved boundary pair disjoint (or
/// re-cut / refused): a connected shell whose surface avoids the
/// other boundary lies in one component, and the witness names it.
fn classify_shells<T: Decide>(
    body: &Body<T>,
    other: &Body<T>,
    contacts: &ContactRecords,
    operand: Operand,
    band: Band,
    tol: Tol,
) -> Result<Vec<(ShellKey, SideCode)>, BooleanError> {
    let corrupt = || BooleanError::JoinDesync {
        what: "fallback operand clone is not walkable",
    };
    let skip = contact_skip_set(contacts, operand);
    let mut out = Vec::new();
    for (shell, shell_data) in body.shells() {
        let mut verdict = None;
        'probe: for &face in &shell_data.faces {
            let face_data = body.get_face(face).ok_or_else(corrupt)?;
            for l in core::iter::once(face_data.outer).chain(face_data.rings.iter().copied()) {
                let LoopBoundary::Cycle { first } = body.get_loop(l).ok_or_else(corrupt)?.boundary
                else {
                    continue;
                };
                for he in body.loop_cycle(first).ok_or_else(corrupt)? {
                    let v = body.get_half_edge(he).ok_or_else(corrupt)?.start;
                    if skip.contains_key(v) {
                        continue;
                    }
                    let q = *body
                        .get_vertex(v)
                        .and_then(|vd| body.get_point(vd.point))
                        .ok_or_else(corrupt)?;
                    match point_in_solid(other, q, band, tol).map_err(BooleanError::Containment)? {
                        SolidContainment::In => {
                            verdict = Some(SideCode::In);
                            break 'probe;
                        }
                        SolidContainment::Out => {
                            verdict = Some(SideCode::Out);
                            break 'probe;
                        }
                        SolidContainment::OnBoundary => continue,
                    }
                }
            }
        }
        let side = verdict.ok_or(BooleanError::Containment(PointInSolidError::RayExhausted))?;
        out.push((shell, side));
    }
    Ok(out)
}

/// The containment fallback (F8): no crossings — classify whole
/// shells, keep per Eq. 15.1's sides, and assemble the typed result.
fn fallback<T: Decide + geom_brep::PcurveFittedLane>(
    op: BooleanOp,
    red: &BooleanReduction<T>,
    a_pristine: &Body<T>,
    b_pristine: &Body<T>,
    decls: &BooleanDeclarations,
    band: Band,
    tol: Tol,
) -> Result<BooleanResult<T>, BooleanError> {
    let desync = |what| BooleanError::JoinDesync { what };
    let a_sides = classify_shells(&red.a, b_pristine, &red.contacts, Operand::A, band, tol)?;
    let b_sides = classify_shells(&red.b, a_pristine, &red.contacts, Operand::B, band, tol)?;
    let keep_a = kept_side(op, Operand::A);
    let keep_b = kept_side(op, Operand::B);
    let a_keep: Vec<ShellKey> = a_sides
        .iter()
        .filter(|(_, s)| *s == keep_a)
        .map(|(k, _)| *k)
        .collect();
    let b_keep: Vec<ShellKey> = b_sides
        .iter()
        .filter(|(_, s)| *s == keep_b)
        .map(|(k, _)| *k)
        .collect();

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
            finish_fallback(
                op,
                body,
                &red.contacts,
                decls,
                BooleanResultKind::OperandA,
                band,
                tol,
            )
        }
        (true, false) => {
            let body = carve_kept(&red.b, &b_keep)?;
            finish_fallback(
                op,
                body,
                &red.contacts,
                decls,
                BooleanResultKind::OperandB,
                band,
                tol,
            )
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
                voids::insert_void(&mut body, solid, b_body, &evidence, tol)
                    .map_err(|e| match e {
                        voids::VoidInsertError::Revert(r) => BooleanError::Revert(r),
                        voids::VoidInsertError::Corrupt { what } => {
                            BooleanError::JoinDesync { what }
                        }
                        voids::VoidInsertError::Recertify(c) => BooleanError::GraftRecertify(c),
                        voids::VoidInsertError::MissingEvidence { .. }
                        | voids::VoidInsertError::NotStrictlyContained { .. }
                        | voids::VoidInsertError::ForeignShell { .. }
                        | voids::VoidInsertError::DuplicateEvidence { .. } => {
                            desync("void evidence desynced from the kept B shells")
                        }
                    })?
                    .graft
            } else {
                graft_solid(&mut body, solid, &b_body, tol)?
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
            );
            remap_carried(
                &mut contacts,
                &body,
                decls,
                &KeyView::Direct,
                &KeyView::Graft(&graft),
                &desc,
            );
            gate(&body)?;
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
fn finish_fallback<T: Decide + geom_brep::PcurveFittedLane>(
    op: BooleanOp,
    body: Body<T>,
    contacts: &ContactRecords,
    decls: &BooleanDeclarations,
    kind: BooleanResultKind,
    band: Band,
    tol: Tol,
) -> Result<BooleanResult<T>, BooleanError> {
    let reduction_contacts = contacts.clone();
    let mut body = body;
    if kind == BooleanResultKind::OperandB && op == BooleanOp::Subtract {
        body = body.revert().map_err(BooleanError::Revert)?;
    }
    // Cross-operand declared pairs are inapplicable here (one operand
    // is absent from the result); the surviving operand's CARRIED
    // records still apply.
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
    let mut contacts = remap_contacts(&body, contacts, a_view, b_view, &desc);
    remap_carried(&mut contacts, &body, decls, &a_view, &b_view, &desc);
    gate(&body)?;
    let naming = match kind {
        BooleanResultKind::OperandA => BooleanNaming {
            a_keys: OperandKeys::Direct,
            b_keys: OperandKeys::Absent,
            merge_groups: merge_rows(&merged),
            merge_skipped: merged.skipped.clone(),
            reduction_contacts: reduction_contacts.clone(),
            ..BooleanNaming::default()
        },
        // The result arena IS the B clone: B keys direct, A absent.
        _ => BooleanNaming {
            a_keys: OperandKeys::Absent,
            b_keys: OperandKeys::Direct,
            merge_groups: merge_rows(&merged),
            merge_skipped: merged.skipped.clone(),
            reduction_contacts: reduction_contacts.clone(),
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

    use geom_core::{Band, Tol};

    use super::volume_backstop;
    use crate::boolean::{BooleanError, BooleanOp};
    use crate::splitting::reassembly::quad_prism;

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
            volume_backstop(BooleanOp::Union, &cube, &cube, &small, band, Tol::witness())
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
            )
            .unwrap_err(),
        );
        // Equal volumes: every bound is non-strict — all pass.
        for op in [BooleanOp::Union, BooleanOp::Intersect, BooleanOp::Subtract] {
            volume_backstop(op, &cube, &cube, &cube, band, Tol::witness()).unwrap();
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
            )
            .unwrap_err(),
        );
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
    /// test goes red while every other row stays green.
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
    /// surface can no longer reach the fallback at all: a placeholder's
    /// control net is poison, so its face box is poison, so it is never
    /// pruned and its pairs meet the crossing layer's typed refusal
    /// first. Both doors are pinned below; what neither may become is a
    /// silent assembly.
    #[test]
    fn nurbs_faces_refuse_typed_at_both_doors() {
        use crate::boolean::{BooleanDeclarations, SweepStrategy, boolean_op_with};
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
        let err = boolean_op_with(
            BooleanOp::Union,
            &a,
            &b,
            &BooleanDeclarations::none(),
            SweepStrategy::Realized,
            Tol::witness(),
        )
        .expect_err("a NURBS operand must refuse typed, never be vertex-probed");
        // Door 1 — the placeholder is unbounded, so the pair is a
        // candidate and the crossing layer refuses it by kind.
        let BooleanError::CurvedBooleanUnsupported {
            kind: geom_brep::SurfaceKind::Nurbs,
            ..
        } = err
        else {
            panic!("expected the crossing-layer refusal, got {err:?}");
        };

        // Door 2 — the fallback's own re-gate, at the mechanism: any
        // fallback entry carrying a NURBS face refuses BEFORE a vertex
        // is probed. NO end-to-end path reaches it today (a lofted
        // operand is refused at its NURBS EDGES first, a placeholder's
        // poison box is never pruned) — `sweep`'s `s16_box_soundness`
        // pins both blockers, so the day one lifts is loud.
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
        );
        assert!(out.a_on_b.is_empty());
        // With the row: the record survives, renamed to the survivor.
        let mut desc = Descendants::default();
        desc.faces.insert(dead_face, live_face);
        let out = remap_contacts(&body, &contacts, KeyView::Direct, KeyView::Direct, &desc);
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
        desc.vertices.insert(dead_vertex, live_vertex);
        let out = remap_contacts(&body, &contacts, KeyView::Direct, KeyView::Direct, &desc);
        assert!(out.vv.is_empty(), "fused-into-one pair is consumed");
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
        use super::{Descendants, KeyView, remap_contacts};
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
        desc.vertices.insert(fused_in, bend);
        desc.fused.insert(bend);
        assert_eq!(
            desc.live_vertex(&body, bend),
            None,
            "no survivor for {bend:?}"
        );

        let records = |v: VertexKey| ContactRecords {
            vv: vec![VvContact { a: v, b: survivor }],
            a_on_b: vec![VfContact { vertex: v, face }],
            b_on_a: vec![VfContact { vertex: v, face }],
            ..ContactRecords::default()
        };
        let remap =
            |c: &ContactRecords| remap_contacts(&body, c, KeyView::Direct, KeyView::Direct, &desc);
        assert_eq!(
            desc.live_vertex(&body, fused_in),
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
}
