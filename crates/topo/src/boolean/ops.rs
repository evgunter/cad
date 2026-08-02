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
//! - [`Voided`](BooleanResultKind::Voided): **the first legitimate
//!   voids** — A∖B with B strictly inside A yields the outer shell
//!   plus the reverted inner shell, a tier-2-legal multi-shell body
//!   (exactly the voids-born-only-from-booleans ratification; sweeps
//!   never produce these — `FullRevolveHoles` points here).
//!
//! When operand boundaries do not intersect, classification falls back
//! to per-shell vertex-in-solid containment
//! ([`point_in_solid`](super::solid_contain::point_in_solid), F8's ray
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

use geom_core::{Band, Bounds, Decide, MarginDiag, Point3, Real, Sign, Vec3};

use super::boxes;
use super::combine::{GraftMap, graft_solid};
use super::contain::{ContainError, FaceContainment, contfp};
use super::finish::{contact_skip_set, kept_side, setopfinish};
use super::join::bool_connect;
use super::solid_contain::{
    PointInSolidError, SolidContainment, closed_sphere_group, point_in_solid,
};
use super::zip::zip_seam;
use super::{
    BooleanDeclarations, BooleanError, BooleanOp, BooleanReduction, CarriedContacts,
    ContactRecords, Operand, SideCode, SweepStrategy, VfContact, VvContact,
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
    /// reduction-minted keys (whose provenance rows, transplanted
    /// verbatim, also speak B-clone keys).
    pub graft_vertices: Vec<(VertexKey, VertexKey)>,
    /// B-side edge graft lineage (see `graft_vertices`).
    pub graft_edges: Vec<(EdgeKey, EdgeKey)>,
    /// B-side face graft lineage (see `graft_vertices`).
    pub graft_faces: Vec<(FaceKey, FaceKey)>,
    /// Seam edges surviving the zips, in zip/cycle order, result keys.
    pub seam_edges: Vec<EdgeKey>,
    /// Zip vertex fusions `(dead, kept)` in zip order, result keys.
    pub vertex_merges: Vec<(VertexKey, VertexKey)>,
    /// `merge_coplanar_faces` absorption groups `(kept, absorbed…)`,
    /// result keys.
    pub merge_groups: Vec<(FaceKey, Vec<FaceKey>)>,
    /// Declared-licensed merge groups the output stage SKIPPED as
    /// outside the never-elide inventory (M4 PR 5): faces + the
    /// actual refusing diagnostics. The skip is visible HERE — a
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
    /// remapping (A rows in A-clone = result keys, B rows in B-CLONE
    /// = operand keys): the mint-time crossing correspondences the
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

/// A ∪* B (module docs; functional, planar-only per F5).
///
/// # Errors
///
/// [`BooleanError`] — every stage's typed refusals pass through.
pub fn union<T: Decide + Bounds>(
    a: &Body<T>,
    b: &Body<T>,
) -> Result<BooleanResult<T>, BooleanError> {
    boolean_op_with(
        BooleanOp::Union,
        a,
        b,
        &BooleanDeclarations::none(),
        SweepStrategy::Realized,
    )
}

/// A ∩* B (module docs).
///
/// # Errors
///
/// [`BooleanError`].
pub fn intersect<T: Decide + Bounds>(
    a: &Body<T>,
    b: &Body<T>,
) -> Result<BooleanResult<T>, BooleanError> {
    boolean_op_with(
        BooleanOp::Intersect,
        a,
        b,
        &BooleanDeclarations::none(),
        SweepStrategy::Realized,
    )
}

/// A ∖* B (module docs).
///
/// # Errors
///
/// [`BooleanError`].
pub fn subtract<T: Decide + Bounds>(
    a: &Body<T>,
    b: &Body<T>,
) -> Result<BooleanResult<T>, BooleanError> {
    boolean_op_with(
        BooleanOp::Subtract,
        a,
        b,
        &BooleanDeclarations::none(),
        SweepStrategy::Realized,
    )
}

/// A ∪* B with declared coincidence intents (F5, M4 PR 5) — see
/// [`BooleanDeclarations`].
///
/// # Errors
///
/// [`BooleanError`].
pub fn union_with<T: Decide + Bounds>(
    a: &Body<T>,
    b: &Body<T>,
    decls: &BooleanDeclarations,
) -> Result<BooleanResult<T>, BooleanError> {
    boolean_op_with(BooleanOp::Union, a, b, decls, SweepStrategy::Realized)
}

/// A ∩* B with declared coincidence intents ([`union_with`]).
///
/// # Errors
///
/// [`BooleanError`].
pub fn intersect_with<T: Decide + Bounds>(
    a: &Body<T>,
    b: &Body<T>,
    decls: &BooleanDeclarations,
) -> Result<BooleanResult<T>, BooleanError> {
    boolean_op_with(BooleanOp::Intersect, a, b, decls, SweepStrategy::Realized)
}

/// A ∖* B with declared coincidence intents ([`union_with`]).
///
/// # Errors
///
/// [`BooleanError`].
pub fn subtract_with<T: Decide + Bounds>(
    a: &Body<T>,
    b: &Body<T>,
    decls: &BooleanDeclarations,
) -> Result<BooleanResult<T>, BooleanError> {
    boolean_op_with(BooleanOp::Subtract, a, b, decls, SweepStrategy::Realized)
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
pub fn boolean_op_with<T: Decide + Bounds>(
    op: BooleanOp,
    a: &Body<T>,
    b: &Body<T>,
    decls: &BooleanDeclarations,
    strategy: SweepStrategy,
) -> Result<BooleanResult<T>, BooleanError> {
    // The curved ∖/∩ front door, NARROWED FROM WHOLESALE TO PER-CLASS
    // (M5 S12; C12.1 — retire per class, never wholesale; M5 S13
    // retires the SPHERE row).
    //
    // It used to refuse on ANY non-plane face, because both ops route
    // regions through `revert` (A∖B ≡ A∩revert(B), the §15.9 posture)
    // and `revert` was planar-only. That premise is gone: S10 ratified
    // `Face::sense`, S11 made the incoming bits honest, S12 wired the
    // flip — so `Cylinder` operands go all the way through, and since
    // M5 S13 `Sphere` operands do too: the (Plane, Sphere) germ arm is
    // wired (the exact C5 Circle) and the no-crossings fallback is
    // extent-certified with a re-cut, so the sphere class's failure
    // mode is typed, not silent.
    //
    // What is NOT retired is the classes with no seam lane behind them.
    // `Cone`/`Torus` germ pairs have no join arm at all (PR 9c
    // deviation 1 lineage) and NURBS faces have no crossing layer
    // (deviation 5) — and their downstream failure mode is not a typed
    // refusal but a SILENT one: with no crossings found the pipeline
    // falls through to the containment fallback, whose certified
    // extent scan covers the sphere class only (everything else it
    // meets refuses typed there — the second door).
    //
    // Structural, exact and up front: an arena scan of surface kinds, no
    // reduction work before it, operands untouched.
    if !matches!(op, BooleanOp::Union) {
        for (operand, body) in [(Operand::A, a), (Operand::B, b)] {
            for (face, fd) in body.faces() {
                if !matches!(
                    body.get_surface(fd.surface),
                    Some(
                        geom_surfaces::Surface::Plane { .. }
                            | geom_surfaces::Surface::Cylinder { .. }
                            | geom_surfaces::Surface::Sphere { .. }
                    )
                ) {
                    return Err(BooleanError::CurvedOpUnsupported { op, operand, face });
                }
            }
        }
    }
    boolean_op_recut(op, a, b, decls, strategy, true)
}

/// The pipeline behind the front door, parameterized on whether the
/// no-crossings sphere RE-CUT (M5 S13) may still run: the re-entry
/// pass sets `recut = false`, so a re-cut that surfaces no crossings
/// is a loud invariant failure rather than a loop.
fn boolean_op_recut<T: Decide + Bounds>(
    op: BooleanOp,
    a: &Body<T>,
    b: &Body<T>,
    decls: &BooleanDeclarations,
    strategy: SweepStrategy,
    recut: bool,
) -> Result<BooleanResult<T>, BooleanError> {
    let band = Band::linear()?;
    let mut red = super::boolean_reduce_declared_strategy(op, a, b, decls, strategy)?;

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
        //   boundary-grazing circles): typed refusal — the S12 silence
        //   never re-opens.
        let recuts = sphere_extent_scan(a, b, band)?;
        if !recuts.is_empty() {
            if !recut {
                return Err(BooleanError::ClassificationInvariant {
                    what: "re-cut sphere operands still produced no crossings",
                });
            }
            let (a2, b2) = apply_recuts(a, b, &recuts)?;
            return boolean_op_recut(op, &a2, &b2, decls, strategy, false);
        }
        return fallback(op, &red, a, b, decls, band);
    }

    // The declared-REST union door (M5 S1): a declared union whose
    // join refuses typed may be the boundary-on-boundary REST
    // frontier — the lane re-examines the UNMUTATED reduction and
    // either zips the mate or reproduces the original refusal
    // verbatim. The clones are taken only when the door can open
    // (declared union), so undeclared and non-union ops pay nothing.
    let rest_door = op == BooleanOp::Union && !decls.coincident_faces.is_empty();
    let saved = rest_door.then(|| (red.a.clone(), red.b.clone()));
    let connected = match bool_connect(&mut red, a, b, band) {
        Ok(c) => c,
        Err(err @ (BooleanError::Join(_) | BooleanError::JoinDesync { .. })) => match saved {
            Some((sa, sb)) => {
                red.a = sa;
                red.b = sb;
                return match super::rest::try_rest_union(red, a, b, decls, band)? {
                    Some(result) => Ok(result),
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
    let fin = setopfinish(op, red, &connected.completed, a, b, band)?;
    let mut body = fin.body;
    let mut seam_edges = Vec::new();
    let mut vertex_merges = Vec::new();
    let mut desc = Descendants::default();
    for &(a_face, b_face) in &fin.seams {
        let rep = zip_seam(&mut body, a_face, b_face, &fin.vertex_map)?;
        desc.absorb_zip(&rep);
        vertex_merges.extend(rep.vertex_merges.iter().copied());
        seam_edges.extend(rep.seam_edges);
    }
    let declared_pairs = declared_surface_pairs(&body, a, b, decls, &fin.graft);
    let merged = body
        .merge_coplanar_faces_declared(&declared_pairs)
        .map_err(BooleanError::Merge)?;
    desc.absorb_merge(&merged);
    describe_minted_edges(&mut body, &seam_edges, &merged, band)?;
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
    crate::pcurves::mint_pcurves(&mut body).map_err(|source| BooleanError::Pcurves { source })?;
    gate(&body)?;
    volume_backstop(op, a, b, &body, band)?;
    let (graft_vertices, graft_edges, graft_faces) = graft_rows(&fin.graft);
    let naming = BooleanNaming {
        a_keys: OperandKeys::Direct,
        b_keys: OperandKeys::Grafted,
        graft_vertices,
        graft_edges,
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

/// The graft map as sorted-order row vectors (naming emission).
type GraftRows = (
    Vec<(VertexKey, VertexKey)>,
    Vec<(EdgeKey, EdgeKey)>,
    Vec<(FaceKey, FaceKey)>,
);

pub(super) fn graft_rows(g: &GraftMap) -> GraftRows {
    (
        g.vertices.iter().map(|(k, &v)| (k, v)).collect(),
        g.edges.iter().map(|(k, &v)| (k, v)).collect(),
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
/// [`mass_properties_with`]. The min/max are decomposed into per-operand
/// inequalities, so no operand-vs-operand comparison is needed.
///
/// Comparison posture: each bound margin is classified through the
/// certified trilean (`sign_within` against the op's linear band) —
/// the codebase's only legal comparison (Q1). Only a CERTIFIED
/// violating sign refuses ([`BooleanError::ResultVolumeImplausible`]);
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
pub(super) fn volume_backstop<T: Decide>(
    op: BooleanOp,
    a: &Body<T>,
    b: &Body<T>,
    result: &Body<T>,
    band: Band,
) -> Result<(), BooleanError> {
    let corrupt = || BooleanError::ClassificationInvariant {
        what: "volume backstop: mass properties refused on a tier-valid planar body",
    };
    // Closed-form lane on purpose (M5 PR 11 lane split): this is the
    // boolean engine's INTERNAL invariant backstop on its own planar/
    // iso results — the certified quadrature lane is the at-rest
    // measurement door, and a trimmed face here keeps the historical
    // fail-loud refusal.
    let vol = |body: &Body<T>| -> Result<T, BooleanError> {
        Ok(crate::props::mass_properties_closed_form(body, band)
            .map_err(|_| corrupt())?
            .volume)
    };
    let (va, vb, vr) = (vol(a)?, vol(b)?, vol(result)?);
    // A bound applies only against a certified-bounded operand
    // (positive flux volume); complement operands (certified negative)
    // make it vacuous. Poison refuses; an in-band operand volume is a
    // degenerate operand — no bound is certifiable from it, skip.
    let bounded = |v: T| -> Result<bool, BooleanError> {
        match v.sign_within(band) {
            Ok(Sign::Positive) => Ok(true),
            Ok(Sign::Zero | Sign::Negative) => Ok(false),
            Err(diag) if matches!(diag.margin, MarginDiag::Invalid) => {
                Err(BooleanError::Escalated {
                    diag: diag.with_predicate("volume_backstop"),
                })
            }
            Err(_) => Ok(false),
        }
    };
    // `margin` ≥ 0 (within band) or the bound named by `which` is
    // violated: margin = bound − got for upper bounds, got − bound for
    // lower bounds.
    let check = |which: &'static str, margin: T, got: T, bound: T| -> Result<(), BooleanError> {
        match margin.sign_within(band) {
            Ok(Sign::Negative) => Err(BooleanError::ResultVolumeImplausible {
                which,
                got: format!("{got:?}"),
                bound: format!("{bound:?}"),
            }),
            Ok(Sign::Zero | Sign::Positive) => Ok(()),
            Err(diag) if matches!(diag.margin, MarginDiag::Invalid) => {
                Err(BooleanError::Escalated {
                    diag: diag.with_predicate("volume_backstop"),
                })
            }
            Err(_) => Ok(()),
        }
    };
    let (ba, bb) = (bounded(va)?, bounded(vb)?);
    match op {
        BooleanOp::Intersect => {
            if ba {
                check("vol(A ∩ B) ≤ vol(A)", va - vr, vr, va)?;
            }
            if bb {
                check("vol(A ∩ B) ≤ vol(B)", vb - vr, vr, vb)?;
            }
        }
        BooleanOp::Union => {
            if ba {
                check("vol(A ∪ B) ≥ vol(A)", vr - va, vr, va)?;
            }
            if bb {
                check("vol(A ∪ B) ≥ vol(B)", vr - vb, vr, vb)?;
            }
        }
        BooleanOp::Subtract => {
            if ba {
                check("vol(A ∖ B) ≤ vol(A)", va - vr, vr, va)?;
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
    // the ACTUAL adjacency below).
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
            let l = body.get_half_edge(he)?.parent_loop;
            Some(body.get_face(body.get_loop(l)?.face)?.surface)
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
            .is_some_and(|c| !matches!(c.carrier(), geom_curves::Curve3::Line { .. }));
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
                        description: geom_brep::EdgeGeometry::Intersection { s1, s2, witness },
                        carrier: c.carrier().clone(),
                        param_start: t0,
                        param_end: t1,
                    }
                } else {
                    let mut spec = geom_brep::EdgeCurveSpec::line_between(p0, p1);
                    spec.description = geom_brep::EdgeGeometry::Intersection { s1, s2, witness };
                    spec
                };
                body.set_edge_curve(edge, spec)
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
                // re-homed its neighbors). Re-describe as the
                // conventional chord line: coplanar surfaces
                // under-determine the locus (D2's split), so the
                // line's own data is the honest description.
                let stale = match *body
                    .get_curve_geom(edge_data.curve)
                    .and_then(crate::null::CurveGeom::certified)
                    .ok_or_else(corrupt)?
                    .description()
                {
                    geom_brep::EdgeGeometry::Intersection { s1: d1, s2: d2, .. }
                    | geom_brep::EdgeGeometry::TangentIntersection { s1: d1, s2: d2, .. } => {
                        !((d1 == s1 && d2 == s2) || (d1 == s2 && d2 == s1))
                    }
                    geom_brep::EdgeGeometry::Seam { surface } => !(surface == s1 && surface == s2),
                    geom_brep::EdgeGeometry::MappedCurve(_) => false,
                };
                if stale {
                    if curved {
                        // A curved smooth seam whose description went
                        // stale has no honest conventional line form —
                        // refuse rather than replace an arc with its
                        // chord (M5 PR 9: no silent geometric rewrite).
                        return Err(BooleanError::JoinDesync {
                            what: "stale CURVED smooth-seam description (no conventional \
                                   re-description lane exists for arcs)",
                        });
                    }
                    body.set_edge_curve(edge, geom_brep::EdgeCurveSpec::line_between(p0, p1))
                        .map_err(|_| BooleanError::JoinDesync {
                            what: "stale in-plane description failed re-certification",
                        })?;
                }
            }
            Err(diag) => return Err(BooleanError::Escalated { diag }),
        }
    }
    Ok(())
}

/// How one operand's keys map into the result body.
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
        .filter_map(|&(fa, fb)| {
            // A-clone surface keys ARE result keys (carve/clone
            // preserve them); B bridges through the graft.
            let ka = a.get_face(fa)?.surface;
            let kb = graft.surfaces.get(b.get_face(fb)?.surface).copied()?;
            (result.get_surface(ka).is_some() && result.get_surface(kb).is_some() && ka != kb)
                .then_some((ka, kb))
        })
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
            if let (Some(a), Some(b)) = (vert(view, c.a), vert(view, c.b))
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
        if let (Some(vertex), Some(fk)) = (vert_strict(a_view, c.vertex), face(a_view, c.face))
            && !dup_vf(out, vertex, fk)
        {
            out.a_on_b.push(VfContact { vertex, face: fk });
        }
    }
    for c in &decls.carried_b.vf {
        if let (Some(vertex), Some(fk)) = (vert_strict(b_view, c.vertex), face(b_view, c.face))
            && !dup_vf(out, vertex, fk)
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

/// The M5 S13 curved-EXTENT scan (fn-level story on the call site in
/// [`boolean_op_recut`]). Sound because every sphere-involved
/// boundary pair is either **certified disjoint** (so a connected
/// shell shares its witness vertex's side — the vertex probe's missing
/// certificate), **an escape** (re-cut and re-entered), or **refused
/// typed**. Cylinder/plane-only configurations run zero new
/// predicates: the scan's outer loop keys on sphere surfaces.
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
            if matches!(
                body.get_surface(fd.surface),
                Some(geom_surfaces::Surface::Nurbs(_))
            ) {
                return Err(BooleanError::NurbsExtentUnsupported { operand, face });
            }
        }
    }
    let pad = boxes::sweep_pad(band);
    let mut out: Vec<SphereRecut<T>> = Vec::new();
    for (x_is, x, y) in [(Operand::A, a, b), (Operand::B, b, a)] {
        let mut seen: Vec<SurfaceKey> = Vec::new();
        for (face, fd) in x.faces() {
            let Some(&geom_surfaces::Surface::Sphere {
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
            // The group-arm discipline (PR 9c): the extent `center ± r`
            // is the whole group's, so it is only honest for a CLOSED
            // group — a trimmed sphere face refuses typed.
            let Some(representative) = closed_sphere_group(x, face) else {
                return Err(BooleanError::FallbackExtentUnsupported {
                    operand: x_is,
                    face,
                    what: "a trimmed sphere face group — the extent certificate needs the \
                           closed-group discipline (PR 9c), and no per-face chart-trim \
                           extent exists",
                });
            };
            let ball_box = bvh::Aabb {
                min_x: center.x.lo() - radius.hi(),
                min_y: center.y.lo() - radius.hi(),
                min_z: center.z.lo() - radius.hi(),
                max_x: center.x.hi() + radius.hi(),
                max_y: center.y.hi() + radius.hi(),
                max_z: center.z.hi() + radius.hi(),
            }
            .padded(pad);
            let mut align: Option<Vec3<T>> = None;
            for (yf, yfd) in y.faces() {
                match y.get_surface(yfd.surface) {
                    Some(&geom_surfaces::Surface::Plane {
                        origin,
                        normal,
                        u_ref,
                    }) => {
                        let s = (center - origin).dot(normal);
                        match decide("bool_sphere_extent_gap", radius - s.abs(), band)
                            .map_err(esc)?
                        {
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
                                           carrier — a touching configuration (the M5 \
                                           envelope's typed frontier)",
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
                                })? {
                                    // The circle misses this face
                                    // (it crosses the carrier plane
                                    // elsewhere).
                                    FaceContainment::Out => {}
                                    FaceContainment::In => {
                                        if align.is_none() {
                                            align = Some(normal);
                                        }
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
                    Some(geom_surfaces::Surface::Cylinder { .. }) => {
                        // No exact sphere-vs-cylinder-face certificate
                        // is wired (the cyl×sphere lane is PR 9c
                        // deviation 1, behind the SSI generic lift);
                        // certified boxes prove separation, anything
                        // closer refuses typed.
                        if boxes::face_box(y, yf, pad)?.overlaps(&ball_box) {
                            return Err(BooleanError::FallbackExtentUnsupported {
                                operand: x_is,
                                face,
                                what: "the sphere's certified extent meets a cylinder \
                                       face's box — the cyl×sphere seam lane (fitted \
                                       chords behind Pcurve::Fitted / the SSI generic \
                                       lift, PR 9c deviation 1) is not wired, so \
                                       nearness cannot be classified",
                            });
                        }
                    }
                    Some(&geom_surfaces::Surface::Sphere {
                        center: c2,
                        radius: r2,
                        ..
                    }) => {
                        let d = (c2 - center).norm();
                        match decide("bool_sphere_sphere_gap", d - (radius + r2), band)
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
                                    big - (d + small),
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
                                                   sphere×sphere germ arm (a closed-form \
                                                   Circle, C5) has no join lane in this \
                                                   build",
                                        });
                                    }
                                }
                            }
                        }
                    }
                    Some(geom_surfaces::Surface::Nurbs(_)) => {
                        // Unreachable: the re-gate above runs first.
                        return Err(BooleanError::NurbsExtentUnsupported {
                            operand: x_is.other(),
                            face: yf,
                        });
                    }
                    Some(
                        geom_surfaces::Surface::Cone { .. } | geom_surfaces::Surface::Torus { .. },
                    ) => {
                        return Err(BooleanError::CurvedBooleanUnsupported {
                            operand: x_is.other(),
                            face: yf,
                            kind: geom_brep::SurfaceKind::of(
                                y.get_surface(yfd.surface).ok_or(
                                    BooleanError::ClassificationInvariant {
                                        what: "extent scan: face surface lost",
                                    },
                                )?,
                            ),
                        });
                    }
                    None => {
                        return Err(BooleanError::ClassificationInvariant {
                            what: "extent scan: face surface lost",
                        });
                    }
                }
            }
            if let Some(align) = align {
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
fn apply_recuts<T: Decide + Bounds>(
    a: &Body<T>,
    b: &Body<T>,
    recuts: &[SphereRecut<T>],
) -> Result<(Body<T>, Body<T>), BooleanError> {
    let band = Band::linear()?;
    let mut out_a = a.clone();
    let mut out_b = b.clone();
    for (operand, out) in [(Operand::A, &mut out_a), (Operand::B, &mut out_b)] {
        let mine: Vec<&SphereRecut<T>> =
            recuts.iter().filter(|r| r.operand == operand).collect();
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
            match decide("bool_sphere_recut_align", sin * r.radius, band)
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
            let angle = sin.atan2(r.axis.dot(r.align));
            let map = geom_core::Affine3::rotation_about_axis(r.center, cross, angle);
            let turned = crate::transform::transform_rigid(&ball, &map)
                .map_err(|_| corrupt("re-cut rotation failed to re-certify"))?;
            rotated.push(turned);
        }
        let keep: Vec<ShellKey> = src
            .get_solid(solid)
            .ok_or(corrupt("re-cut solid lost"))?
            .shells
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
                    let base_solid = single_solid(base)
                        .map_err(|_| corrupt("re-cut base is not one solid"))?;
                    graft_solid(base, base_solid, &turned)?;
                }
            }
        }
        *out = rebuilt.ok_or(corrupt("re-cut produced no body"))?;
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
                    match point_in_solid(other, q, band).map_err(BooleanError::Containment)? {
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
fn fallback<T: Decide>(
    op: BooleanOp,
    red: &BooleanReduction<T>,
    a_pristine: &Body<T>,
    b_pristine: &Body<T>,
    decls: &BooleanDeclarations,
    band: Band,
) -> Result<BooleanResult<T>, BooleanError> {
    let desync = |what| BooleanError::JoinDesync { what };
    let a_sides = classify_shells(&red.a, b_pristine, &red.contacts, Operand::A, band)?;
    let b_sides = classify_shells(&red.b, a_pristine, &red.contacts, Operand::B, band)?;
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
            )
        }
        (false, false) => {
            let mut body = carve_kept(&red.a, &a_keep)?;
            let mut b_body = carve_kept(&red.b, &b_keep)?;
            if op == BooleanOp::Subtract {
                b_body = b_body.revert().map_err(BooleanError::Revert)?;
            }
            let solid =
                single_solid(&body).map_err(|_| desync("fallback A carve not one solid"))?;
            let graft = graft_solid(&mut body, solid, &b_body)?;
            let kind = match op {
                BooleanOp::Subtract => BooleanResultKind::Voided,
                _ => BooleanResultKind::Assembly,
            };
            let declared_pairs =
                declared_surface_pairs(&body, a_pristine, b_pristine, decls, &graft);
            let merged = body
                .merge_coplanar_faces_declared(&declared_pairs)
                .map_err(BooleanError::Merge)?;
            let mut desc = Descendants::default();
            desc.absorb_merge(&merged);
            describe_minted_edges(&mut body, &[], &merged, band)?;
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
            let (graft_vertices, graft_edges, graft_faces) = graft_rows(&graft);
            let naming = BooleanNaming {
                a_keys: OperandKeys::Direct,
                b_keys: OperandKeys::Grafted,
                graft_vertices,
                graft_edges,
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
fn finish_fallback<T: Decide>(
    op: BooleanOp,
    body: Body<T>,
    contacts: &ContactRecords,
    decls: &BooleanDeclarations,
    kind: BooleanResultKind,
    band: Band,
) -> Result<BooleanResult<T>, BooleanError> {
    let reduction_contacts = contacts.clone();
    let mut body = body;
    if kind == BooleanResultKind::OperandB && op == BooleanOp::Subtract {
        body = body.revert().map_err(BooleanError::Revert)?;
    }
    // Cross-operand declared pairs are inapplicable here (one operand
    // is absent from the result); the surviving operand's CARRIED
    // records still apply.
    let merged = body.merge_coplanar_faces().map_err(BooleanError::Merge)?;
    let mut desc = Descendants::default();
    desc.absorb_merge(&merged);
    describe_minted_edges(&mut body, &[], &merged, band)?;
    let (a_view, b_view) = match kind {
        BooleanResultKind::OperandA => (KeyView::Direct, KeyView::Absent),
        _ => (KeyView::Absent, KeyView::Direct),
    };
    let mut contacts = remap_contacts(&body, contacts, a_view, b_view, &desc);
    let (a_view, b_view) = match kind {
        BooleanResultKind::OperandA => (KeyView::Direct, KeyView::Absent),
        _ => (KeyView::Absent, KeyView::Direct),
    };
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

    use geom_core::Band;

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
        let band = Band::linear().unwrap();
        let square = [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];
        let cube = quad_prism(&square, 1.0);
        let small = quad_prism(&square, 0.5);
        let implausible = |e: BooleanError| {
            assert!(
                matches!(e, BooleanError::ResultVolumeImplausible { .. }),
                "expected ResultVolumeImplausible, got {e:?}"
            );
        };
        // vol(∪) ≥ max: a union "result" smaller than an operand.
        implausible(volume_backstop(BooleanOp::Union, &cube, &cube, &small, band).unwrap_err());
        // vol(∖) ≤ vol(A): a subtract "result" larger than A.
        implausible(volume_backstop(BooleanOp::Subtract, &small, &cube, &cube, band).unwrap_err());
        // vol(∩) ≤ min: an intersect "result" larger than an operand.
        implausible(volume_backstop(BooleanOp::Intersect, &small, &cube, &cube, band).unwrap_err());
        // Equal volumes: every bound is non-strict — all pass.
        for op in [BooleanOp::Union, BooleanOp::Intersect, BooleanOp::Subtract] {
            volume_backstop(op, &cube, &cube, &cube, band).unwrap();
        }
        // Complement operand (negative flux volume): its bound is
        // vacuous and must be SKIPPED — A ∩ revert(B) legitimately
        // exceeds vol(revert B); the A-side bound still applies.
        let rev = quad_prism(&square, 0.5).revert().unwrap();
        volume_backstop(BooleanOp::Intersect, &cube, &rev, &cube, band).unwrap();
        implausible(volume_backstop(BooleanOp::Intersect, &small, &rev, &cube, band).unwrap_err());
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
        let mut body = quad_prism(&square, 1.0);
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
        };
        let mut desc = Descendants::default();
        desc.vertices.insert(dead_vertex, live_vertex);
        let out = remap_contacts(&body, &contacts, KeyView::Direct, KeyView::Direct, &desc);
        assert!(out.vv.is_empty(), "fused-into-one pair is consumed");
    }
}
