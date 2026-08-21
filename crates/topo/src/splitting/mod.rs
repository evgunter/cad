//! Plane-splitting (M3 PRs 2 + 3): ch. 14 end to end, re-derived under
//! our conventions. PR 2 built **reduction + vertex-neighborhood
//! classification** (`splitgenerate` + `splitclassify`, this module +
//! `classify`/`neighborhood`/`rules`/`insert`); PR 3 adds **joining**
//! (`join` — `splitconnect`, with the total in-plane lexicographic
//! `order` and the `containment` trilean for ring re-homing),
//! **finish** (`finish` — `splitfinish`: section-face promotion,
//! component distribution, the carve into two result bodies), the
//! public [`split`] op, and **slicing** ([`plane_section`], `section`).
//!
//! Pipeline of [`split_reduce`] (functional: operates on a clone, the
//! operand is untouched):
//!
//! 1. **Operand gate (F5 → THE C5 table, M5 PR 5)**: every face's
//!    `(kind × plane)` arm must be one the pipeline executes —
//!    `Plane` (the M3 seam, bit-identical) or `Cylinder` (the rung-2
//!    conic lane); other kinds refuse typed CITING their rung routing
//!    ([`SplitReduceError::CurvedBooleanUnsupported`] — per-arm
//!    retirement, C12.1). Edge carriers `Line`/`Circle`/`Ellipse`
//!    pass; `Nurbs` refuses
//!    ([`SplitReduceError::CurvedEdgeUnsupported`]) — a rung-3 carrier
//!    in the input operand, refused on this gate's own footing: the
//!    general rung is implemented, and gates retire per arm.
//! 2. **Vertex sweep (F6)**: every vertex classified against the plane
//!    through the Q1 trilean `split_vertex_side` — definitely-off ⇒
//!    clean side, coincident ⇒ [`PlaneSide::On`], in-band ⇒ the typed
//!    [`SplitReduceError::SliverVertex`] escalation (**no snapping**:
//!    the book conscripts near vertices into ON; we refuse — the
//!    operand is ill-conditioned at this ε). Verdicts are cached per
//!    vertex (one predicate site, one evaluation per vertex).
//! 3. **Crossing insertion**: edges whose cached endpoint verdicts are
//!    strictly opposite are split by [`crate::Body::split_edge`] at the
//!    interpolated carrier parameter — the crossing goes through the
//!    certified lane (the `split_edge_param_interior` trilean plus full
//!    child re-certification), never a raw interpolation trusted blind.
//!    New vertices are ON **by construction** (declared coincidence —
//!    round-8 ladder), not re-measured.
//! 4. **Neighborhood classification** per ON vertex: the vertex orbit
//!    becomes a typed sector array ([`SectorEntry`] — no parallel raw
//!    arrays), wide/reflex sectors stored twice with a bisector entry
//!    (`neighborhood` — the convex-subdivision derivation), rule (a)
//!    via the F3 primitive [`geom_brep::enters_material`] and rule (b)
//!    per the F4 adjudication ([`rules`] — the derivation lives there).
//! 5. **Null-edge insertion** (`insert`): an explicit transition-pair
//!    run scan in worklist form (sidesteps the Program 14.7
//!    head-rebinding erratum structurally), one
//!    [`crate::Body::mev_null`] per ABOVE-run, orientation recorded as
//!    F9 **data** (`NullEdge { below_end, above_end }`), ≥2 disjoint
//!    ABOVE-runs handled and tested.
//!
//! The result ([`SplitReduction`]) is the annotated body the joining
//! step consumes: cached per-vertex sides, the ON set, and the minted
//! null edges with their side attributes. [`split`] composes
//! reduce → join → finish; [`plane_section`] stops after join and
//! reads the polygons off the scratch clone.

mod classify;
pub mod containment;
pub(crate) use classify::conic_plane_crossing_roots;
pub(crate) mod finish;
mod insert;
pub(crate) mod join;
mod neighborhood;
// `pub(crate)` for `order::exact_band` alone: the bit-hairline band is
// a shared DEVICE (audit note N6), and the boolean backstop's
// sign-certainty arm decides against the same one rather than minting
// a second copy of the constants.
pub(crate) mod order;
#[cfg(test)]
pub(crate) mod reassembly;
pub mod rules;
mod section;

use geom_core::{BandError, Indeterminate, Point3, Real, Vec3};

use crate::body::Body;
use crate::entity::{EdgeKey, FaceKey, HalfEdgeKey, VertexKey};
use crate::euler::EulerOpError;
use crate::null::NullEdge;
use geom_core::Tol;
use slotmap::SecondaryMap;

pub use crate::chord_join::{ArcWindowCase, SplitJoinError};
pub use containment::{LoopContainment, PointInLoopError, point_in_loop};
pub use finish::{SplitFinishError, SplitNaming, SplitPart, SplitResult};
pub use neighborhood::classify_neighborhood;
pub use section::{Section, SectionPolygon, plane_section};

/// The splitting plane: a point on the plane and its **unit** normal
/// (conventional, unchecked — same posture as `Surface::Plane`). The
/// positive side (`(p − origin)·normal > 0`) is **Above**.
#[derive(Clone, Copy, Debug)]
pub struct SplitPlane<T: Real> {
    /// A point on the plane.
    pub origin: Point3<T>,
    /// The unit normal; Above is the side it points to.
    pub normal: Vec3<T>,
}

/// A trilean side verdict against the split plane (the classification
/// currency of the whole reduction; `comp`'s −1/0/+1 as a typed enum).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlaneSide {
    /// Definitely on the negative side of the plane.
    Below,
    /// Coincident with the plane (|margin| ≤ ε — structural/declared
    /// coincidence lands here; near-misses escalate instead, F6).
    On,
    /// Definitely on the positive side.
    Above,
}

/// What a neighborhood entry stands for (typed, F9-style — never
/// inferred from array position).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SectorEntryKind {
    /// A real neighborhood edge, classified by its final vertex.
    Edge,
    /// The duplicate entry of a wide (≥180°) sector, classified by the
    /// sector's interior bisector direction (convex subdivision — see
    /// the `neighborhood` module docs). Shares its half-edge with the
    /// preceding [`SectorEntryKind::Edge`] entry.
    WideBisector,
}

/// One entry of an ON vertex's typed sector array: a half-edge of the
/// vertex orbit plus its (re)classification. Entry `k` implicitly names
/// the sector between entry `k` and entry `k+1` (cyclic); that sector's
/// face is `face(loop(mate(entries[k].he)))` under our orbit
/// conventions (derived in the `neighborhood` module docs).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SectorEntry {
    /// The orbit half-edge (starts at the base vertex).
    pub he: HalfEdgeKey,
    /// Edge entry or wide-sector bisector duplicate.
    pub kind: SectorEntryKind,
    /// The current classification (initial → after rule (a) → after
    /// rule (b)).
    pub class: PlaneSide,
}

/// One minted null edge, with its F9 orientation attribute and birth
/// site — the record PR 3's joining consumes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NullEdgeRecord {
    /// The ON vertex whose neighborhood classification minted this
    /// null edge (its below-side copy survives as `attr.below_end`).
    pub at_vertex: VertexKey,
    /// The null edge itself.
    pub edge: EdgeKey,
    /// The F9 side attribute (`below_end` = old vertex, `above_end` =
    /// the minted copy holding the ABOVE run) — orientation as data.
    pub attr: NullEdge,
    /// True for a dangling null edge (a wide same-side sector whose
    /// bisector crossed — the strut case).
    pub dangling: bool,
}

/// The reduction result: the annotated clone plus the classification
/// data PR 3's joining step consumes. The operand body is untouched.
#[derive(Debug)]
pub struct SplitReduction<T: Real> {
    /// The clone with crossings split and null edges inserted.
    pub body: Body<T>,
    /// The plane the reduction classified against.
    pub plane: SplitPlane<T>,
    /// Cached per-vertex side verdicts — every operand vertex, plus
    /// crossing vertices and null-edge copies (both ON by
    /// construction).
    pub sides: SecondaryMap<VertexKey, PlaneSide>,
    /// The ON-vertex set (`soov`), in deterministic discovery order:
    /// operand vertices in arena order, then crossing vertices in edge
    /// arena order. Null-edge copies are deliberately NOT members (they
    /// are the products of classification, not its inputs).
    pub on_vertices: Vec<VertexKey>,
    /// Every null edge minted, in insertion order.
    pub null_edges: Vec<NullEdgeRecord>,
}

/// Typed failure of [`split_reduce`]; the returned operand clone is
/// dropped, the operand itself was never touched.
#[derive(Debug)]
pub enum SplitReduceError {
    /// The run's tolerance cannot form a valid band (D4 residue).
    Band(BandError),
    /// A face's `(kind × plane)` arm of THE C5 dispatch table is not
    /// executed by the split pipeline (M5 PR 5: `Plane` and `Cylinder`
    /// are; the refusal retires PER ARM, never wholesale — C12.1). The
    /// Display cites the arm's rung routing from
    /// [`geom_brep::intersect::route`].
    CurvedBooleanUnsupported {
        /// The offending face.
        face: FaceKey,
        /// Its surface kind (the table row).
        kind: geom_brep::SurfaceKind,
    },
    /// An edge carrier is the `Nurbs` fallback — a rung-3 carrier in
    /// the INPUT operand. The general rung itself is implemented (SSI);
    /// this gate is what has not retired, and gates retire per arm,
    /// never wholesale (C12.1) — so the refusal rests on its own
    /// footing, not on SSI's absence. Line/circle/ellipse carriers all
    /// pass the gate.
    CurvedEdgeUnsupported {
        /// The offending edge.
        edge: EdgeKey,
    },
    /// A conic edge's plane-crossing root landed in the ambiguity band
    /// of the edge's far end (the crossing grazes a vertex): the
    /// operand/plane pair is ill-conditioned at this ε (F6).
    CrossingEscalated {
        /// The crossing edge.
        edge: EdgeKey,
        /// The escalation diagnostics.
        diag: Indeterminate,
    },
    /// The split plane is tangent to a curved face at an ON vertex
    /// (the local normal is plane-parallel) AND the second-order
    /// descent ties: the surface osculates the plane (its largest
    /// tangent-plane normal curvature is exactly zero at the site —
    /// `tangent_sector_osculation`), so even the C12.2 lane cannot
    /// classify the contact — the surfaces under-determine it. Since
    /// M5 PR 9 a DEFINITELY-bending tangent contact classifies one
    /// order down instead of refusing here; in-band bending escalates
    /// (F6).
    TangencyUnsupported {
        /// The tangent face.
        face: FaceKey,
        /// The ON vertex where the contact was classified.
        vertex: VertexKey,
    },
    /// The operand already contains null-edge scaffolding — it is a
    /// mid-surgery body, not a splittable operand.
    ScaffoldingOperand {
        /// The scaffolding edge.
        edge: EdgeKey,
    },
    /// A vertex landed in the sliver band of the plane (F6): the
    /// operand/plane pair is ill-conditioned at this ε. No snapping —
    /// resolution is an explicit repair/adoption op (D7 machinery).
    SliverVertex {
        /// The offending vertex.
        vertex: VertexKey,
        /// The escalation diagnostics.
        diag: Indeterminate,
    },
    /// A sector-level predicate (coplanarity gate, material sense,
    /// wideness, or bisector side) escalated at an ON vertex.
    SliverSector {
        /// The ON vertex being classified.
        vertex: VertexKey,
        /// The sector's face.
        face: FaceKey,
        /// The escalation diagnostics (named predicate inside).
        diag: Indeterminate,
    },
    /// Two cyclically-consecutive entries remained ON after rule (a) —
    /// the "no consecutive ONs" invariant failed. For a planar operand
    /// this means a coplanar sector escaped the gate (documented
    /// invariant, checked loudly rather than assumed). Since M5 PR 9
    /// the conic in-plane-departure chain resolves ONE ORDER DOWN
    /// first (the C12.2 second-order trilean,
    /// `tangent_sector_order2`: an arc grazing the plane classifies
    /// by which side it CURVES to), so reaching this refusal on a
    /// conic boundary means the chain tied at second order too — a
    /// genuinely osculating configuration this machinery will not
    /// guess at (in-band second order escalates F6 upstream instead).
    ConsecutiveOnSectors {
        /// The ON vertex whose neighborhood violated the invariant.
        vertex: VertexKey,
    },
    /// A traversal failed (broken orbit/loop or a lone vertex): the
    /// operand is not a well-formed closed solid at this vertex.
    CorruptOperand {
        /// The vertex whose neighborhood could not be walked.
        vertex: VertexKey,
    },
    /// `split_edge` refused while inserting the crossing vertex on an
    /// edge whose endpoints straddle the plane — typically the
    /// certification lane's `ResidualExceeded` re-certifying the
    /// children of a large-coordinate crossing at a strict ε row. The
    /// inner typed error is nested whole; this variant only adds the
    /// crossing site.
    CrossingInsertion {
        /// The straddling edge being split.
        edge: EdgeKey,
        /// Its endpoint vertices (the strictly Above/Below pair).
        endpoints: (VertexKey, VertexKey),
        /// The underlying Euler refusal, untouched.
        source: EulerOpError,
    },
    /// An underlying Euler operation refused (`split_edge` includes the
    /// certified-interiority refusals and escalations).
    Euler(EulerOpError),
}

impl From<BandError> for SplitReduceError {
    fn from(e: BandError) -> Self {
        Self::Band(e)
    }
}

impl From<EulerOpError> for SplitReduceError {
    fn from(e: EulerOpError) -> Self {
        Self::Euler(e)
    }
}

impl core::fmt::Display for SplitReduceError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Band(e) => write!(f, "split_reduce: invalid band: {e}"),
            Self::CurvedBooleanUnsupported { face, kind } => write!(
                f,
                "split_reduce: face {face:?}: {}",
                geom_brep::intersect::route(*kind, geom_brep::SurfaceKind::Plane)
                    .refusal(*kind, geom_brep::SurfaceKind::Plane)
            ),
            Self::CurvedEdgeUnsupported { edge } => write!(
                f,
                "split_reduce: edge {edge:?} has a NURBS carrier — a rung-3 carrier in \
                 an INPUT operand. The general rung itself is implemented (SSI); this \
                 gate has not retired, and gates retire one arm at a time"
            ),
            Self::CrossingEscalated { edge, diag } => write!(
                f,
                "split_reduce: the plane-crossing root on conic edge {edge:?} grazes the \
                 edge end — an ill-conditioned operand/plane pair at this tolerance \
                 (F6): {diag}"
            ),
            Self::TangencyUnsupported { face, vertex } => write!(
                f,
                "split_reduce: the split plane is tangent to curved face {face:?} at \
                 vertex {vertex:?} — the transversality margin dies along the contact; \
                 tangent loci are TangentIntersection (C7) territory, constructed at \
                 M5 PR 9, never marched into"
            ),
            Self::ScaffoldingOperand { edge } => write!(
                f,
                "split_reduce: operand carries null-edge scaffolding at {edge:?} (mid-surgery \
                 body, not a splittable operand)"
            ),
            Self::SliverVertex { vertex, diag } => write!(
                f,
                "split_reduce: vertex {vertex:?} lies in the sliver band of the split plane \
                 ({diag}); the operand/plane pair is ill-conditioned at this tolerance — \
                 resolve by explicit repair/adoption, never by snapping"
            ),
            Self::SliverSector { vertex, face, diag } => write!(
                f,
                "split_reduce: sector classification escalated at vertex {vertex:?} \
                 (face {face:?}): {diag}"
            ),
            Self::ConsecutiveOnSectors { vertex } => write!(
                f,
                "split_reduce: consecutive ON entries survived rule (a) at vertex {vertex:?} \
                 (coplanar sector escaped the gate — invariant violation)"
            ),
            Self::CorruptOperand { vertex } => write!(
                f,
                "split_reduce: neighborhood of vertex {vertex:?} could not be walked \
                 (broken orbit or lone vertex)"
            ),
            Self::CrossingInsertion {
                edge,
                endpoints: (u, v),
                source,
            } => write!(
                f,
                "split_reduce: crossing insertion refused on edge {edge:?} (endpoints \
                 {u:?}/{v:?} straddle the plane): {source}"
            ),
            Self::Euler(e) => write!(f, "split_reduce: euler operation refused: {e}"),
        }
    }
}

impl std::error::Error for SplitReduceError {}

/// The non-mutating prefix of the reduction — the F5 planar gate plus
/// the cached vertex sweep — exposed so classification
/// ([`classify_neighborhood`]) can be inspected/reviewed independently
/// of surgery. Returns the per-vertex side cache and the ON set of the
/// body **as given** (crossing vertices only exist after
/// [`split_reduce`] inserts them).
///
/// # Errors
///
/// As the corresponding [`split_reduce`] stages.
pub fn vertex_sides<T: geom_core::Decide>(
    body: &Body<T>,
    plane: &SplitPlane<T>,
    tol: Tol,
) -> Result<(SecondaryMap<VertexKey, PlaneSide>, Vec<VertexKey>), SplitReduceError> {
    let band = geom_core::Band::linear(tol)?;
    classify::gate_operand(body)?;
    classify::classify_vertices(body, plane, band)
}

/// **`split_reduce`** — the reduction + neighborhood-classification
/// half of plane splitting (module docs for the pipeline). Functional:
/// `operand` is cloned and never touched; the annotated clone comes
/// back inside [`SplitReduction`].
///
/// Determinism (D9): vertex and edge sweeps run in arena slot order;
/// ON vertices are processed in discovery order; null-edge worklists
/// execute in entry order — no hash iteration anywhere.
///
/// # Errors
///
/// [`SplitReduceError`] — see each variant; the first failure wins and
/// the operand is never mutated (the clone is dropped).
pub fn split_reduce<T: geom_core::Decide>(
    operand: &Body<T>,
    plane: &SplitPlane<T>,
    tol: Tol,
) -> Result<SplitReduction<T>, SplitReduceError> {
    let band = geom_core::Band::linear(tol)?;
    let mut body = operand.clone();

    classify::gate_operand(&body)?;
    let (mut sides, mut on_vertices) = classify::classify_vertices(&body, plane, band)?;
    classify::insert_crossings(&mut body, plane, &mut sides, &mut on_vertices, tol)?;

    let mut null_edges = Vec::new();
    for &v in &on_vertices {
        let entries = neighborhood::classify_neighborhood(&body, plane, &sides, v, band)?;
        let runs = insert::above_runs(&entries);
        insert::insert_null_edges(&mut body, v, &entries, &runs, &mut sides, &mut null_edges)?;
    }

    Ok(SplitReduction {
        body,
        plane: *plane,
        sides,
        on_vertices,
        null_edges,
    })
}

/// Typed failure of the public [`split`] op (and [`plane_section`]):
/// each stage's errors pass through whole.
#[derive(Debug)]
pub enum SplitError {
    /// The reduction/classification stage refused (M3 PR 2's typed
    /// surface, unchanged).
    Reduce(SplitReduceError),
    /// The joining stage refused (incl. the degenerate one-sided
    /// tangency section).
    Join(SplitJoinError),
    /// The finish stage refused (incl. the degenerate-component net).
    Finish(SplitFinishError),
    /// The pcurve minting pass refused (M5 PR 6): a curved face's
    /// per-half-edge chart-image cache failed certification, or a
    /// loop's one-branch chart walk was discontinuous. The split
    /// itself succeeded topologically; the refusal is loud rather
    /// than shipping a body whose caches are uncertified (D4 ¶2).
    Pcurves(crate::pcurves::PcurveMintError),
}

impl From<SplitReduceError> for SplitError {
    fn from(e: SplitReduceError) -> Self {
        Self::Reduce(e)
    }
}

impl From<SplitJoinError> for SplitError {
    fn from(e: SplitJoinError) -> Self {
        Self::Join(e)
    }
}

impl From<SplitFinishError> for SplitError {
    fn from(e: SplitFinishError) -> Self {
        Self::Finish(e)
    }
}

impl From<crate::pcurves::PcurveMintError> for SplitError {
    fn from(e: crate::pcurves::PcurveMintError) -> Self {
        Self::Pcurves(e)
    }
}

// A stage that names itself is not re-named here: `split_reduce`,
// `split join` and `split finish` each lead with their own stage, so a
// prefix would only stutter once this error is forwarded — the node
// layer prefixes again, and so does the binding. `Pcurves` is the one
// stage whose error is shared with callers that are not splits, so it
// is the one arm that says where it ran.
impl core::fmt::Display for SplitError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Reduce(e) => write!(f, "{e}"),
            Self::Join(e) => write!(f, "{e}"),
            Self::Finish(e) => write!(f, "{e}"),
            Self::Pcurves(e) => write!(f, "split: {e}"),
        }
    }
}

impl std::error::Error for SplitError {}

/// Runs reduce + join on a scratch clone, returning the joined
/// reduction and the completed sections (shared prefix of [`split`]
/// and [`plane_section`]).
pub(crate) fn split_scratch<T: geom_core::Decide>(
    operand: &Body<T>,
    plane: &SplitPlane<T>,
    tol: Tol,
) -> Result<
    (
        SplitReduction<T>,
        Vec<join::CompletedSection>,
        crate::chord_join::FragmentRows,
    ),
    SplitError,
> {
    let band = geom_core::Band::linear(tol).map_err(SplitReduceError::from)?;
    let mut red = split_reduce(operand, plane, tol)?;
    let (completed, fragments) = join::split_connect(&mut red, band, tol)?;
    Ok((red, completed, fragments))
}

/// **`split`** — plane-splitting of a solid (ch. 14 end to end):
/// reduce ([`split_reduce`]) → join (`splitconnect`) → finish
/// (`splitfinish`), composed functionally. The operand is never
/// touched; both sides come back as independent bodies
/// ([`SplitResult`]), with a side the plane misses entirely as the
/// typed [`SplitPart::Empty`] (never an empty body value).
///
/// Coplanar-artifact faces (operand faces IN the plane; cut faces as
/// same-key coplanar pairs) are **left in place** —
/// [`Body::merge_coplanar_faces`] is the caller's explicit opt-in
/// (F7: merging is never silent).
///
/// Determinism (D9): every stage sweeps in arena/list order; the
/// joining order is the total lexicographic sort (`order` module) —
/// byte-identical replay is pinned by test.
///
/// # Below-side pinches (M3 PR 6a, D7)
///
/// At a **BOB pinch** vertex (pieces meeting at a tip line, pinched
/// pieces on the plane normal's NEGATIVE side) the ch. 14 insertion —
/// which mints vertex copies for ABOVE runs only — leaves every below
/// fan sharing the one original vertex, and the join is forced into a
/// zero-area 2-gon in each tip-adjacent face
/// ([`SplitJoinError::DegenerateSection`], the PR 3 posture). The
/// below fans need their own copies. This op realizes that
/// below-copy minting through the **mirror identity**
/// `split(S, n) ≡ swap(split(S, −n))` (exact: piece assignment is
/// plane-orientation-equivariant — PR 2's principle, executed by the
/// PR 3 review's orientation table, tol): on a `DegenerateSection` refusal
/// the pipeline reruns under the flipped plane — where the pinched
/// fans ARE the above runs and receive their distinct copies — and
/// swaps the sides back. Success is therefore
/// orientation-INDEPENDENT for the single-sided pinch class. A run
/// whose mirror also refuses surfaces the original typed refusal —
/// and this is a KNOWN COMPLETENESS FRONTIER, not a proof of
/// impossibility: a body pinched on BOTH sides of the plane
/// (`review_m3_pr6::r1_both_sided_pinch` is the pinned witness) has
/// a valid decomposition, but each run direction refuses at its own
/// below-side pinch — the mirror lane only relocates which side
/// lacks copies. Resolving the both-sided class needs the native
/// below-copy insertion lane (mixed copy loops in join
/// role-resolution); until then it refuses typed, never emits a
/// degenerate body. Note the error plumbing on asymmetric failures:
/// if the mirror run fails DIFFERENTLY (e.g. `JoinDesync`), the
/// pipeline reruns the direct orientation and surfaces the direct
/// refusal — loud and typed, but attributed to the direct run (the
/// mirror's distinct failure is not reported), at the cost of up to
/// three pipeline runs.
/// The result's section-face normals still follow THIS
/// call's plane convention (above face m = −n, below face m = +n)
/// because the mirrored run's roles are the swap of ours.
///
/// # Errors
///
/// [`SplitError`], each stage's typed refusals passed through whole —
/// including the one-sided-tangency degenerate section/side refusals
/// (no degenerate body is ever emitted).
pub fn split<T: geom_core::Decide>(
    operand: &Body<T>,
    plane: &SplitPlane<T>,
    tol: Tol,
) -> Result<SplitResult<T>, SplitError> {
    match split_direct(operand, plane, tol) {
        Err(SplitError::Join(SplitJoinError::DegenerateSection { .. })) => {}
        other => return other,
    }
    // D7: the pinch lane — rerun mirrored (the below fans become
    // above runs and mint their copies), swap the sides back. A
    // double degenerate refusal surfaces the DIRECT run's error.
    let mirrored = SplitPlane {
        origin: plane.origin,
        normal: -plane.normal,
    };
    match split_direct(operand, &mirrored, tol) {
        Ok(SplitResult {
            above,
            below,
            naming,
        }) => Ok(SplitResult {
            above: below,
            below: above,
            // The naming sides were recorded against the MIRRORED
            // plane; swap them back with the bodies so `sections`
            // states sides in the caller's orientation.
            naming: finish::SplitNaming {
                sections: naming
                    .sections
                    .into_iter()
                    .map(|(f, s)| {
                        let flipped = match s {
                            PlaneSide::Above => PlaneSide::Below,
                            PlaneSide::Below => PlaneSide::Above,
                            other => other,
                        };
                        (f, flipped)
                    })
                    .collect(),
                face_fragments: naming.face_fragments,
                // Pairs stay (copy, original): the mirrored run's
                // copies land on the caller's BELOW side, but
                // consumers resolve pair roles by which body holds
                // each key, so no swap is needed here.
                vertex_pairs: naming.vertex_pairs,
            },
        }),
        Err(_) => split_direct(operand, plane, tol),
    }
}

/// One direct (non-mirrored) run of the split pipeline.
///
/// The pcurve minting pass (M5 PR 6, C4) runs last, on each side that
/// carries material: this lane is where curved faces are minted, so it
/// is where their per-half-edge chart-image caches are minted and
/// certified (spec §1). Planar sides pick up nothing — planar faces
/// keep M2's derive-on-demand status — so an all-planar split is
/// bit-identical to before this pass existed.
fn split_direct<T: geom_core::Decide>(
    operand: &Body<T>,
    plane: &SplitPlane<T>,
    tol: Tol,
) -> Result<SplitResult<T>, SplitError> {
    let (red, completed, fragments) = split_scratch(operand, plane, tol)?;
    let mut result = finish::split_finish(red, &completed, fragments, tol)?;
    for part in [&mut result.above, &mut result.below] {
        if let finish::SplitPart::Body(body) = part {
            crate::pcurves::mint_pcurves(body, tol)?;
        }
    }
    Ok(result)
}
