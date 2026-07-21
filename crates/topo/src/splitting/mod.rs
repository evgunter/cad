//! Plane-splitting, part 1 (M3 PR 2): **reduction + vertex-neighborhood
//! classification** — ch. 14's `splitgenerate` + `splitclassify`
//! re-derived under our conventions, ending at null-edge insertion. The
//! joining/finish half and the public `split` op are PR 3.
//!
//! Pipeline of [`split_reduce`] (functional: operates on a clone, the
//! operand is untouched):
//!
//! 1. **Planar gate (F5)**: every face must be a `Plane` and every edge
//!    carrier a `Line`; anything else is the typed
//!    [`SplitReduceError::CurvedBooleanUnsupported`] /
//!    [`SplitReduceError::CurvedEdgeUnsupported`] refusal (curved
//!    splitting is M5, and this module deliberately builds NO
//!    curved-readiness abstraction).
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
//! The result ([`SplitReduction`]) is the annotated body ready for
//! PR 3's joining: cached per-vertex sides, the ON set, and the minted
//! null edges with their side attributes.

mod classify;
pub mod containment;
mod finish;
mod insert;
mod join;
mod neighborhood;
mod order;
#[cfg(test)]
mod reassembly;
pub mod rules;
mod section;

use geom_core::{BandError, Indeterminate, Point3, Real, Vec3};

use crate::body::Body;
use crate::entity::{EdgeKey, FaceKey, HalfEdgeKey, VertexKey};
use crate::euler::EulerOpError;
use crate::null::NullEdge;
use slotmap::SecondaryMap;

pub use containment::{LoopContainment, PointInLoopError, point_in_loop};
pub use finish::{SplitFinishError, SplitPart, SplitResult};
pub use join::SplitJoinError;
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
    /// A face of the operand is not a `Plane` — M3 splitting is
    /// planar-only (F5); curved booleans/splitting are M5.
    CurvedBooleanUnsupported {
        /// The offending face.
        face: FaceKey,
    },
    /// An edge carrier is not a `Line` (unreachable for a legal
    /// all-planar operand, but typed rather than assumed — F5).
    CurvedEdgeUnsupported {
        /// The offending edge.
        edge: EdgeKey,
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
    /// the "no consecutive ONs" invariant failed, which for a planar
    /// operand means a coplanar sector escaped the gate (documented
    /// invariant, checked loudly rather than assumed).
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
            Self::CurvedBooleanUnsupported { face } => write!(
                f,
                "split_reduce: face {face:?} is not planar — M3 splitting/booleans are \
                 planar-only (curved intersections arrive with M5 SSI)"
            ),
            Self::CurvedEdgeUnsupported { edge } => write!(
                f,
                "split_reduce: edge {edge:?} has a non-line carrier — planar-only (M5)"
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
) -> Result<(SecondaryMap<VertexKey, PlaneSide>, Vec<VertexKey>), SplitReduceError> {
    let band = geom_core::Band::linear()?;
    classify::gate_planar(body)?;
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
) -> Result<SplitReduction<T>, SplitReduceError> {
    let band = geom_core::Band::linear()?;
    let mut body = operand.clone();

    classify::gate_planar(&body)?;
    let (mut sides, mut on_vertices) = classify::classify_vertices(&body, plane, band)?;
    classify::insert_crossings(&mut body, plane, &mut sides, &mut on_vertices)?;

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

impl core::fmt::Display for SplitError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Reduce(e) => write!(f, "split: {e}"),
            Self::Join(e) => write!(f, "split: {e}"),
            Self::Finish(e) => write!(f, "split: {e}"),
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
) -> Result<(SplitReduction<T>, Vec<join::CompletedSection>), SplitError> {
    let band = geom_core::Band::linear().map_err(SplitReduceError::from)?;
    let mut red = split_reduce(operand, plane)?;
    let completed = join::split_connect(&mut red, band)?;
    Ok((red, completed))
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
/// # Errors
///
/// [`SplitError`], each stage's typed refusals passed through whole —
/// including the one-sided-tangency degenerate section/side refusals
/// (no degenerate body is ever emitted).
pub fn split<T: geom_core::Decide>(
    operand: &Body<T>,
    plane: &SplitPlane<T>,
) -> Result<SplitResult<T>, SplitError> {
    let (red, completed) = split_scratch(operand, plane)?;
    Ok(finish::split_finish(red, &completed)?)
}
