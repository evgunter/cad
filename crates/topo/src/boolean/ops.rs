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

use geom_core::{Band, Bounds, Decide, Margin, MarginDiag, Point3, Real, Sign, Tol, Vec3};

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

/// A ∪* B (module docs; functional). Surface kinds are gated per arm,
/// not wholesale — see `reduce::gate_operand_pairs`.
///
/// # Errors
///
/// [`BooleanError`] — every stage's typed refusals pass through.
pub fn union<T: Decide + Bounds>(
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
pub fn intersect<T: Decide + Bounds>(
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
pub fn subtract<T: Decide + Bounds>(
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
pub fn union_with<T: Decide + Bounds>(
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
pub fn intersect_with<T: Decide + Bounds>(
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
pub fn subtract_with<T: Decide + Bounds>(
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
pub fn boolean_op_with<T: Decide + Bounds>(
    op: BooleanOp,
    a: &Body<T>,
    b: &Body<T>,
    decls: &BooleanDeclarations,
    strategy: SweepStrategy,
    tol: Tol,
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
    // Up front and PAIR-SCOPED: the kinds are read exactly, and the
    // question of whether a kind can matter to this operation is
    // decided by boxes (`reduce::first_unsupported_pair` — non-overlap
    // is a certificate, overlap is a may). Operands untouched, no
    // reduction work before it.
    if !matches!(op, BooleanOp::Union) {
        let band = Band::linear(tol)?;
        if let Some(p) =
            super::reduce::first_unsupported_pair(a, b, band, super::reduce::revert_arm_exists)?
        {
            return Err(BooleanError::CurvedPairUnsupported {
                op: Some(op),
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
fn boolean_op_recut<T: Decide + Bounds>(
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
        return fallback(op, &red, a, b, decls, band, tol);
    }

    // The declared-REST union door (M5 S1): a declared union whose
    // join refuses typed may be the boundary-on-boundary REST
    // frontier — the lane re-examines the UNMUTATED reduction and
    // either zips the mate or reproduces the original refusal
    // verbatim. The clones are taken only when the door can open
    // (declared union), so undeclared and non-union ops pay nothing.
    let rest_door = op == BooleanOp::Union && !decls.coincident_faces.is_empty();
    let saved = rest_door.then(|| (red.a.clone(), red.b.clone()));
    let connected = match bool_connect(&mut red, a, b, band, tol) {
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
    let fin = setopfinish(op, red, &connected.completed, a, b, band, tol)?;
    let mut body = fin.body;
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
    gate(&body)?;
    volume_backstop(op, a, b, &body, band, tol)?;
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
    // The backstops live on the INVARIANT LANE (Evan's #213 layering
    // ruling): consistency inequalities between integral results are
    // outside the length seam by design — no door, bare T — and a
    // certified violation is a kernel invariant failure, not a
    // validity refusal. Values and predicate names are unchanged.
    let bounded = |v: T, area: T| -> Result<bool, BooleanError> {
        match geom_core::k_stats::decide_invariant("volume_backstop_operand", v / area, band) {
            Ok(Sign::Positive) => Ok(true),
            Ok(Sign::Zero | Sign::Negative) => Ok(false),
            Err(diag) if matches!(diag.margin, MarginDiag::Invalid) => {
                Err(BooleanError::Escalated { diag })
            }
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
                Err(diag) if matches!(diag.margin, MarginDiag::Invalid) => {
                    Err(BooleanError::Escalated { diag })
                }
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
                    // The iso description cites ONE adjacent surface
                    // (its residual chart); stale iff neither side is
                    // it (the attach-door adjacency rule, M6-3).
                    geom_brep::EdgeGeometry::IsoCurve { surface, .. } => {
                        !(surface == s1 || surface == s2)
                    }
                    geom_brep::EdgeGeometry::MappedCurve(_) => false,
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
                        let p = c.carrier().eval(t);
                        let jet = geom_brep::tangent_jet(surf1, surf2, p, c.carrier().deriv(t));
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
                        description: geom_brep::EdgeGeometry::TangentIntersection {
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
                        // the adjacent surfaces under-determine: the
                        // same pushforward posture as the planar chord
                        // lane, on the UNCHANGED carrier (no silent
                        // geometric rewrite — only the description
                        // moves). Carrier kinds with no conventional
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
                        body.set_edge_curve(edge, spec, tol).map_err(|_| {
                            BooleanError::JoinDesync {
                                what: "stale arc description failed re-certification",
                            }
                        })?;
                    } else {
                        body.set_edge_curve(
                            edge,
                            geom_brep::EdgeCurveSpec::line_between(p0, p1),
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
            if matches!(body.get_surface(fd.surface), Some(geom::Surface::Nurbs(_))) {
                return Err(BooleanError::NurbsExtentUnsupported { operand, face });
            }
        }
    }
    let pad = boxes::sweep_pad(band);
    let mut out: Vec<SphereRecut<T>> = Vec::new();
    for (x_is, x, y) in [(Operand::A, a, b), (Operand::B, b, a)] {
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
            // The group-arm discipline (PR 9c): the extent `center ± r`
            // is the whole group's, so it is only honest for a CLOSED
            // group — a trimmed sphere face refuses typed.
            let Some(representative) = closed_sphere_group(x, face) else {
                return Err(BooleanError::FallbackExtentUnsupported {
                    operand: x_is,
                    face,
                    what: "a trimmed sphere face group — the extent certificate needs the \
                           closed-group discipline, and no per-face chart-trim extent \
                           exists",
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
            let mut escape_normals: Vec<Vec3<T>> = Vec::new();
            for (yf, yfd) in y.faces() {
                match y.get_surface(yfd.surface) {
                    Some(&geom::Surface::Plane {
                        origin,
                        normal,
                        u_ref,
                    }) => {
                        let s = (center - origin).dot(normal);
                        match decide("bool_sphere_extent_gap", Margin::of(radius - s.abs()), band)
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
                                })? {
                                    // The circle misses this face
                                    // (it crosses the carrier plane
                                    // elsewhere).
                                    FaceContainment::Out => {}
                                    FaceContainment::In => {
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
                        // Certified boxes prove separation, anything
                        // closer refuses typed.
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
                                                   sphere×sphere germ arm (a closed-form \
                                                   Circle) has no join lane in this \
                                                   build",
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
                    // `Approx` joins the no-wired-arm refusal, not the
                    // NURBS lane: the pair-scoped operand gate refuses
                    // it by kind before this scan runs.
                    Some(
                        geom::Surface::Cone { .. }
                        | geom::Surface::Torus { .. }
                        | geom::Surface::Approx(_),
                    ) => {
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
            if let Some((&align, rest)) = escape_normals.split_first() {
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
fn apply_recuts<T: Decide + Bounds>(
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
                    let base_solid =
                        single_solid(base).map_err(|_| corrupt("re-cut base is not one solid"))?;
                    graft_solid(base, base_solid, &turned, tol)?;
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
fn fallback<T: Decide>(
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
        use crate::euler::{MefSite, MevSite};
        use crate::fixtures::ops_cube;
        use geom_core::Point3;

        // The ops_cube recipe, x-shifted (the fixture is anchored at
        // the origin; the far copy needs disjoint certified boxes so
        // the realized sweep examines no pair at all).
        let far_cube = |dx: f64| {
            let pt = |x: f64, y: f64, z: f64| Point3::new(x + dx, y, z);
            let mut body = crate::Body::<f64>::new();
            let seed = body.mvfs(pt(0.0, 0.0, 0.0)).unwrap();
            let e_ab = body
                .mev_line(
                    MevSite::Lone {
                        r#loop: seed.r#loop,
                    },
                    pt(1.0, 0.0, 0.0),
                    Tol::witness(),
                )
                .unwrap();
            let strut = |body: &mut crate::Body<f64>, at, x, y, z| {
                body.mev_line(
                    MevSite::Fan { he1: at, he2: at },
                    pt(x, y, z),
                    Tol::witness(),
                )
                .unwrap()
            };
            let mef = |body: &mut crate::Body<f64>, he1, he2| {
                body.mef_chord(MefSite::Chords { he1, he2 }, Tol::witness())
                    .unwrap()
            };
            let e_bc = strut(&mut body, e_ab.he_minus, 1.0, 1.0, 0.0);
            let e_cd = strut(&mut body, e_bc.he_minus, 0.0, 1.0, 0.0);
            let he_dc = body
                .find_half_edge(seed.face, e_cd.vertex, e_bc.vertex)
                .unwrap();
            let f_bottom = mef(&mut body, he_dc, e_ab.he_plus);
            let e_aa = strut(&mut body, e_ab.he_plus, 0.0, 0.0, 1.0);
            let e_bb = strut(&mut body, e_bc.he_plus, 1.0, 0.0, 1.0);
            let e_cc = strut(&mut body, e_cd.he_plus, 1.0, 1.0, 1.0);
            let e_dd = strut(&mut body, f_bottom.he_plus, 0.0, 1.0, 1.0);
            let f_front = mef(&mut body, e_aa.he_minus, e_bb.he_minus);
            let _ = mef(&mut body, e_bb.he_minus, e_cc.he_minus);
            let _ = mef(&mut body, e_cc.he_minus, e_dd.he_minus);
            let _ = mef(&mut body, e_dd.he_minus, f_front.he_plus);
            body
        };

        let a = ops_cube(Tol::witness()).body;
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
        // The refusal names the lift blocker, so the recourse is
        // discoverable from the error alone.
        let msg = err.to_string();
        assert!(msg.contains("NurbsSurface::project"), "{msg}");
        assert!(msg.contains("implicit_residual"), "{msg}");
        assert!(msg.contains("re-gated"), "{msg}");
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
}
