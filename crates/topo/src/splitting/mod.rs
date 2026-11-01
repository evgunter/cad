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
//! The doors that take an operand ([`split`], [`split_reduce`],
//! [`plane_section`], [`vertex_sides`]) take a finished one
//! ([`crate::AtRestBody`]): a body tier 3 passed, or one whose scalar runs
//! no at-rest gate (a dual). Over the latter the door reads what the type
//! promises itself before anything else, with the Boolean's read
//! ([`crate::AtRestBody::gate_unverdicted`]):
//!
//! - tier 2, over the whole body: a strut, an empty loop, a null edge or
//!   a shell in pieces refuses [`SplitReduceError::ScaffoldingOperand`];
//! - orientation, as tier 3 reads it: check 7 per solid, then check 10
//!   per shell. A solid whose total is negative, or a shell bounding
//!   negative material inside a solid whose total is positive, refuses
//!   [`SplitReduceError::InsideOutOperand`].
//!
//! Pipeline of [`split_reduce`] (functional: operates on a clone, the
//! operand is untouched):
//!
//! 1. **Carrier gate (the C5 table, scoped to the plane's reach)**:
//!    a face of a kind the pipeline has no `(kind × plane)` arm for
//!    (`Sphere`, `Torus`, `Nurbs`, `Approx`) refuses typed
//!    ([`SplitReduceError::CurvedBooleanUnsupported`]) only when the
//!    plane may meet its padded reach box; one behind a box that
//!    clears passes through the split whole. Edge carriers
//!    `Line`/`Circle`/`Ellipse` pass; `Spiric` and `Nurbs` refuse
//!    ([`SplitReduceError::CurvedEdgeUnsupported`]) on the same
//!    reach-scoped terms.
//! 2. **Vertex sweep (F6)**: every vertex classified against the plane
//!    through the Q1 trilean `split_vertex_side` — definitely-off ⇒
//!    clean side, coincident ⇒ [`PlaneSide::On`], in-band ⇒ the typed
//!    [`SplitReduceError::SliverVertex`] escalation (**no snapping**:
//!    the book conscripts near vertices into ON; we refuse — the
//!    operand is ill-conditioned at this ε). Verdicts are cached per
//!    vertex (one predicate site, one evaluation per vertex).
//! 3. **Crossing insertion**: edges whose cached endpoint verdicts are
//!    strictly opposite are split by [`crate::Body::split_edge`](crate::construction::Construct::split_edge) at the
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
//! 5. **Null-edge insertion** (`insert`): one
//!    [`crate::Body::mev_null`](crate::construction::Construct::mev_null) per corner of the section at the
//!    vertex, hung as the tree the corners make (a star of one per
//!    ABOVE-run where the runs lie apart; deeper where runs nest),
//!    orientation recorded as F9 **data**
//!    (`NullEdge { below_end, above_end }`).
//!
//! The result ([`SplitReduction`]) is the annotated body the joining
//! step consumes: cached per-vertex sides, the ON set, and the minted
//! null edges with their side attributes. [`split`] composes
//! reduce → join → finish's carve into two sides, then on each side:
//! tier 2, the fusion of a pinch's cones where the section's corners
//! pair their germs (`section_cones`), the coplanar merges and edge
//! joins (`finish::end_maximal`), tier 2 again, and the pcurves.
//! [`plane_section`] stops after join and reads the polygons off the
//! scratch clone.

mod classify;
pub mod containment;
pub(crate) use classify::{ConicPlaneMeet, PlaneCrossingLane, plane_crossing_lane};
pub use classify::{ConicRootFault, CrossingDecision};
pub(crate) mod finish;
mod insert;
pub(crate) mod join;
mod neighborhood;
pub(crate) use neighborhood::{EntryFault, entry_side, wide_entries};
// `pub(crate)` for `order::{FRAME_ARM, FRAME_AXIS}`, which
// `chord_join` reads rather than minting a second copy.
pub(crate) mod order;
pub(crate) mod pieced_arc;
#[cfg(test)]
pub(crate) mod reassembly;
pub mod rules;
mod section;
mod section_cones;
mod section_loops;
pub(crate) mod spiric_arc;
pub(crate) mod spline_arc;

use geom_brep::recourse::{LeverOnly, Reading, RefusedArm};
use geom_core::{Band, BandError, Indeterminate, MarginDiag, Point3, Real, UnitVec3, Vec3};

use crate::body::Body;
use crate::entity::{EdgeKey, FaceKey, HalfEdgeKey, VertexKey};
use crate::euler::EulerOpError;
use crate::null::NullEdge;
use crate::validate::{AtRestBody, Unfinished, ValidationError};
use geom_core::Tol;
use slotmap::SecondaryMap;

pub use crate::chord_join::{
    ConicCrossingsCase, SectionEndReading, SectionPinMiss, SplitJoinError,
};
pub use containment::{
    Escalation, LoopContainment, LoopDecision, OffPlane, OffPlaneCause, PointInLoopError,
    Uncrossable, UncrossableCarrier, point_in_loop,
};
pub use finish::{
    PlanePieces, SplitContacts, SplitFinishError, SplitNaming, SplitPart, SplitResult,
};
pub use neighborhood::classify_neighborhood;
pub use section::{
    Section, SectionEdge, SectionError, SectionPolygon, SectionRegion, plane_section,
};

/// The splitting plane: a point on the plane and its unit normal. The
/// positive side (`(p − origin)·normal > 0`) is **Above**.
///
/// The normal is a [`UnitVec3`], so its length is decided where the
/// caller mints it (`UnitVec3::new`) rather than assumed here: every
/// conic section of a curved face reads the normal's components as
/// direction cosines, and a longer vector reads as a shallower tilt.
#[derive(Clone, Copy, Debug)]
pub struct SplitPlane<T: Real> {
    /// A point on the plane.
    pub origin: Point3<T>,
    /// The unit normal; Above is the side it points to.
    pub normal: UnitVec3<T>,
}

/// Which reading of where the split plane leaves the faces round a
/// vertex escalated ([`SplitReduceError::SectionGermsEscalated`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SectionGermReading {
    /// Whether a face meets the plane along a line there
    /// (`split_germ_line`).
    Line,
    /// The order of two germs about the plane's normal
    /// (`split_germ_side`, `split_germ_sense`, `split_germ_order`), or of
    /// two along one tangent by how their sections curve off it
    /// (`tangent_sector_order2`, `tangent_sector_order2_arm`).
    Order,
}

/// How the section table's reading of a plane against a cone disagrees
/// with the walk, which read the plane crossing one of the cone's
/// sub-sectors at its apex ([`SplitReduceError::ApexSection`]).
#[derive(Debug)]
pub enum ApexSectionReading {
    /// The table reads the plane's section of the cone as a closed
    /// conic, standing off the apex.
    ClosedOffApex,
    /// The table refused the pair; its refusal, which for a plane the
    /// walk read through the apex is most often an open conic standing
    /// off it ([`SplitJoinError::OpenConicChord`]).
    Refused(Box<SplitJoinError>),
    /// The table's section rulings through the apex: none of them lies
    /// inside the sub-sector the walk read the plane crossing.
    NoRulingInside,
    /// Both of the table's section rulings lie inside one sub-sector,
    /// across which the walk read the plane's side monotone.
    BothRulingsInside,
}

/// Why the germs round a vertex fix no corners of the section there
/// ([`SplitReduceError::SectionGermsUnordered`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SectionGermFault {
    /// A face whose bounds lie on two sides of the plane meets it along
    /// no line: its bounds both lie in the plane, or it lies in it to
    /// the tolerance.
    NoLine,
    /// Two germs of different sectors lie along one direction, to the
    /// tolerance, and their sections curve alike off it.
    Coincident,
    /// Two germs lie along one direction, and a face's section has no
    /// curvature to read there: its surface is singular at the vertex
    /// (a cone's apex) or meets the plane tangentially.
    Singular,
    /// The germs as read do not alternate between the material's starts
    /// and ends. Argued to need faces within the tolerance of one
    /// another, which the readings decided apart rule out; not reached.
    Alternation,
    /// The corners as read cross as chords of the vertex's orbit. Argued
    /// as `Alternation` is; not reached.
    Crossing,
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

impl PlaneSide {
    /// The side as a refusal names it.
    #[must_use]
    pub fn word(self) -> &'static str {
        match self {
            Self::Below => "below",
            Self::On => "on",
            Self::Above => "above",
        }
    }

    /// The side across the plane; `On` stays `On`.
    #[must_use]
    pub(crate) fn opposite(self) -> Self {
        match self {
            Self::Below => Self::Above,
            Self::On => Self::On,
            Self::Above => Self::Below,
        }
    }
}

/// What a neighborhood entry stands for (typed, F9-style — never
/// inferred from array position).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SectorEntryKind {
    /// A real neighborhood edge, classified by its final vertex.
    Edge,
    /// A subdivision entry of one sector, classified by its direction: a
    /// wide (≥180°) sector's interior bisector (convex subdivision), a slit
    /// tip's full turn's three quarter directions, or, at a cone's apex,
    /// each extremal ruling inside the sector, one or two at any width (the
    /// `neighborhood` module docs). Shares its half-edge with
    /// the sector's [`SectorEntryKind::Edge`] entry before it.
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
    /// The vertex this null edge was minted at: the ON vertex whose
    /// neighborhood classification minted it, or a copy minted there
    /// before it; the end of `attr` that is not the minted copy.
    pub at_vertex: VertexKey,
    /// The null edge itself.
    pub edge: EdgeKey,
    /// The F9 side attribute (`above_end` = the end standing for the
    /// piece above the plane) — orientation as data.
    pub attr: NullEdge,
    /// True for a dangling null edge (subdivision entries of one sector
    /// that crossed where its bounds did not — the strut case, either way
    /// round).
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
    /// Each null-edge half whose face is a cone at its apex there, with
    /// the unit ruling of the section the face leaves the apex along
    /// beside it: the germ its cut was read at (`insert`), which the join
    /// puts the half on its section ruling by.
    pub(crate) apex_germs: SecondaryMap<HalfEdgeKey, Vec3<T>>,
    /// The pinches the ON verdicts decided, in ON-vertex order, keyed in
    /// the operand ([`crate::coincidence`]).
    pub coincidences: Vec<crate::Coincidence>,
    /// Each piece a crossing split minted, `(new edge, divided-from
    /// edge)` in mint order, checked as each was minted.
    pub edge_pieces: crate::lineage::SplitEdgeDivisions,
}

/// **A knife edge the split would mint and cannot declare.** The plane
/// is tangent to a curved wall that bends away from its material (a
/// hole's wall touched from inside): material lies on both sides of the
/// plane at the contact, and the piece on the wall's side meets the cut
/// face tangentially along it, tapering to an edge of wedge 0. A split
/// has no declaration channel, so it refuses every such edge (D1). One
/// refusal, raised by whichever stage reads the contact first
/// ([`SplitReduceError::KnifeEdge`], [`SplitFinishError::KnifeEdge`];
/// [`SplitError::knife_edge`] reads either).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct KnifeEdge {
    /// The curved wall the plane is tangent to.
    pub wall: FaceKey,
    /// Where the contact was read.
    pub at: KnifeEdgeSite,
}

/// Where a [`KnifeEdge`]'s contact was read.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KnifeEdgeSite {
    /// An edge that runs along the contact: a seam of the wall, or the
    /// edge between the wall and a face in the plane.
    Edge(EdgeKey),
    /// An ON vertex the contact passes through where no edge runs along
    /// it (the contact crosses a rim mid-wall).
    Vertex(VertexKey),
}

impl core::fmt::Display for KnifeEdge {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "the split plane is tangent to a curved face where it cuts, so a piece would \
             taper to a knife edge nobody asked for. Recourse: move the split plane off the \
             tangency"
        )
    }
}

/// Typed failure of [`split_reduce`]; the returned operand clone is
/// dropped, the operand itself was never touched.
#[derive(Debug)]
pub enum SplitReduceError {
    /// The run's tolerance cannot form a valid band (D4 residue).
    Band(BandError),
    /// The plane may meet a face whose `(kind × plane)` arm of the C5
    /// dispatch table the split pipeline does not execute (`Plane`,
    /// `Cylinder` and `Cone` are; C12.1). "May meet": the face's padded
    /// reach box, read in the plane's own frame so that it turns with
    /// the body, is not certainly on one side of the plane; a face
    /// behind a box that clears does not refuse.
    CurvedBooleanUnsupported {
        /// The offending face.
        face: FaceKey,
        /// Its surface kind (the table row).
        kind: geom::SurfaceKind,
    },
    /// The plane may meet an edge on a `Spiric` or `Nurbs` carrier, which
    /// no crossing lane reads (`Line`, `Circle` and `Ellipse` carriers
    /// all pass). An edge clears behind its own reach box or the box of
    /// either face it bounds.
    CurvedEdgeUnsupported {
        /// The offending edge.
        edge: EdgeKey,
    },
    /// A rung of a conic edge's plane-crossing root lane escalated: the
    /// operand/plane pair is ill-conditioned at this ε (F6).
    CrossingEscalated {
        /// The crossing edge.
        edge: EdgeKey,
        /// Which rung escalated, with its diagnostics.
        fault: ConicRootFault,
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
    /// The plane is tangent to a curved wall that bends away from its
    /// material, read where the reduction finds the wall tangent at an
    /// ON vertex (`rules::apply_rule_a`): the knife edge [`KnifeEdge`]
    /// names.
    KnifeEdge(KnifeEdge),
    /// The operand is well-formed but not a closed solid at rest: tier 2
    /// ([`crate::validate_closed`]) refuses it for construction
    /// scaffolding an edit left behind — a strut's valence-1 vertex, an
    /// empty loop, a null edge, a shell in pieces. The split serves
    /// finished solids only.
    ScaffoldingOperand {
        /// The validator's tier-2 findings, each naming its entity.
        errors: Vec<ValidationError>,
    },
    /// The operand holds material wound negative: a solid whose signed
    /// volume tier 3's check 7 decides definitely negative
    /// ([`ValidationError::NegativeVolume`]), or a shell check 10 finds
    /// bounding a region of winding −1 inside a solid whose total is
    /// positive ([`ValidationError::ShellWinding`]). Its faces bound the
    /// complement of the region they enclose, and each side would carry
    /// that complement's half.
    InsideOutOperand {
        /// The validator's findings, each naming its solid (and, for a
        /// shell, the shell).
        errors: Vec<ValidationError>,
    },
    /// The operand holds a vertex the join would take, or one whose
    /// reading lands in the sliver band (tier 3's check 11): a body every
    /// finisher would have joined, read where its scalar runs no at-rest
    /// gate ([`AtRestBody::gate_unverdicted`]).
    UnjoinedOperand {
        /// The check-11 findings, each naming its vertex.
        errors: Vec<ValidationError>,
    },
    /// Whether a vertex lies on the plane is undecided (F6): its distance
    /// from the plane, or from a section the plane ends at it
    /// ([`crate::vertex_section`]), lies in the ambiguity band, beyond ε,
    /// or cannot be read. No snapping — resolution is an explicit
    /// repair/adoption op (D7 machinery).
    SliverVertex {
        /// The offending vertex.
        vertex: VertexKey,
        /// The escalation diagnostics.
        diag: Indeterminate,
    },
    /// A sector-level reading at an ON vertex (the face's extent, the
    /// coplanarity gate, the material sense, wideness, the bisector side,
    /// or a grazed wall's material) cannot place the vertex's
    /// neighbourhood at this ε.
    SliverSector {
        /// The ON vertex being classified.
        vertex: VertexKey,
        /// The sector's face.
        face: FaceKey,
        /// How the refused reading stands.
        reading: SectorReading,
    },
    /// Where the plane leaves the faces round an ON vertex could not be
    /// read: a reading landed in the band, so which germs bound each
    /// corner of the section there is undecided.
    SectionGermsEscalated {
        /// The ON vertex.
        vertex: VertexKey,
        /// Which reading escalated.
        reading: SectionGermReading,
        /// The escalation diagnostics (named predicate inside).
        diag: Indeterminate,
    },
    /// The directions in which the plane leaves the faces round an ON
    /// vertex fix no corners of the section there; `fault` names why.
    SectionGermsUnordered {
        /// The ON vertex.
        vertex: VertexKey,
        /// What fixed no corners.
        fault: SectionGermFault,
    },
    /// The walk read the plane crossing a cone face at its apex, and the
    /// section table, asked for the ruling the plane leaves the apex
    /// along inside one of the face's sub-sectors, does not agree:
    /// `reading` says how.
    ApexSection {
        /// The apex vertex.
        vertex: VertexKey,
        /// The cone face.
        face: FaceKey,
        /// How the table's section disagrees with the walk.
        reading: ApexSectionReading,
    },
    /// A vertex of a cone face lies within the band of the cone's axis
    /// ([`crate::face_normal::cone_point`] escalated): whether the
    /// face has a tangent plane there or is at its apex is undecided.
    ApexUndecided {
        /// The vertex.
        vertex: VertexKey,
        /// The cone face.
        face: FaceKey,
        /// The escalation.
        diag: Indeterminate,
    },
    /// A sector's bounding chord has **no finite length**: its
    /// components overflow the norm (past ~1e154), or one of them is
    /// not a number. Distinct from
    /// [`SplitReduceError::SliverSector`] on purpose — nothing about
    /// this is a band question, and no tolerance lever reaches it.
    NonFiniteSectorChord {
        /// The vertex being classified.
        vertex: VertexKey,
        /// The sector's face.
        face: FaceKey,
    },
    /// A sector's bounding chord has a length that **underflowed out
    /// of the format**: its components are too small for the norm to
    /// hold (below ~1e-162 at `f64`), so it measures exactly zero while
    /// still naming a direction. Distinct from
    /// [`SplitReduceError::SliverSector`] on purpose — the chord is
    /// not a sliver and no tolerance lever reaches it, because the
    /// squared norm is zero at every eps.
    UnderflowedSectorChord {
        /// The vertex being classified.
        vertex: VertexKey,
        /// The sector's face.
        face: FaceKey,
    },
    /// Two cyclically-consecutive entries remained ON after rule (a) —
    /// the "no consecutive ONs" invariant failed. For a planar operand
    /// this means a coplanar sector escaped the gate (documented
    /// invariant, checked loudly rather than assumed). A conic's
    /// in-plane departure reads one order down first
    /// ([`geom_brep::conic_bend_side`]: an arc grazing the plane
    /// classifies by which side it bends to), so reaching this refusal
    /// on a conic boundary means a whole conic edge lay within the band
    /// of the plane (an undecided bend escalates upstream instead).
    ConsecutiveOnSectors {
        /// The ON vertex whose neighborhood violated the invariant.
        vertex: VertexKey,
    },
    /// The vertex [`classify_neighborhood`] was asked about does not
    /// resolve in the body.
    StaleVertex {
        /// The caller's vertex.
        vertex: VertexKey,
    },
    /// The vertex is a lone vertex: no edge leaves it, so it has no
    /// neighborhood to classify.
    LoneVertex {
        /// The lone vertex.
        vertex: VertexKey,
    },
    /// An edge leaving the vertex [`classify_neighborhood`] was asked
    /// about is a null edge: a sector is read off its edge's carrier, and
    /// a null edge has none. The pipeline never hands it one (the
    /// finished-body gate runs tier 2 first); a caller of the public read
    /// over a mid-surgery body can.
    NullEdgeAtVertex {
        /// The caller's vertex.
        vertex: VertexKey,
        /// The null edge leaving it.
        edge: EdgeKey,
    },
    /// The side map [`classify_neighborhood`] was handed holds no
    /// verdict for a vertex its neighborhood reaches.
    UnrecordedSide {
        /// The vertex with no verdict.
        vertex: VertexKey,
    },
    /// A face whose outer loop is a lone vertex: it has no outer
    /// boundary, so no finite lever arm meters a predicate across it.
    UnboundedFace {
        /// The face.
        face: FaceKey,
        /// Its outer loop's lone vertex.
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
        /// The underlying Euler refusal, untouched (boxed: it is the
        /// largest payload any split refusal carries).
        source: Box<EulerOpError>,
    },
    /// An underlying Euler operation refused (`split_edge` includes the
    /// certified-interiority refusals and escalations).
    Euler(EulerOpError),
    /// The crossing insertion's edge division list refused a row where
    /// a split added it ([`crate::lineage::SplitEdgeDivisions`]): a
    /// kernel defect, since each split records the piece it just
    /// minted.
    LineageRow(crate::lineage::RowRefusal),
}

/// **How the reading a [`SplitReduceError::SliverSector`] refuses
/// stands**, which its ending follows from (D4 ¶1 (i)): every arm ends
/// in the split's lever, read at the operation, and a margin that could
/// not be read adds its note.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SectorReading {
    /// A sector predicate's escalation: its margin in the band or
    /// unreadable, or a gate's rejection of the sign it decided
    /// ([`geom_core::MarginDiag::rejected_sign`]).
    Escalated(Indeterminate),
    /// A wall the plane grazes holds its material across from the edges
    /// beside it in the vertex's cycle: the wall's outward normal over its
    /// face extent and those edges' own sides, decided apart.
    GrazeAgainstNeighbours {
        /// The sense margin the wall's material side was decided on.
        margin: MarginDiag,
    },
    /// At a smooth edge in the plane, one face reads the plane tangent
    /// over its own face extent and the other reads it clear over its own:
    /// two decided readings of the plane's tilt, over two arms, that
    /// disagree.
    SmoothEdgeDisagrees(TangencyReadings),
}

/// The two decided readings a [`SectorReading::SmoothEdgeDisagrees`]
/// holds: `split_sector_coplanar`'s sine margins, each over its face's
/// extent.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TangencyReadings {
    /// The grazed face's, decided zero.
    pub tangent: MarginDiag,
    /// The other face's, decided past the band.
    pub clear: MarginDiag,
    /// The band both were decided against.
    pub band: Band,
}

impl SectorReading {
    /// The refused arm the shared table reads this reading as.
    #[must_use]
    pub fn arm(&self) -> RefusedArm<'_> {
        match self {
            Self::Escalated(diag) => RefusedArm::Undecided(diag),
            Self::GrazeAgainstNeighbours { margin } => RefusedArm::SignCertain(Some(*margin)),
            Self::SmoothEdgeDisagrees(TangencyReadings { clear, .. }) => {
                RefusedArm::SignCertain(Some(*clear))
            }
        }
    }

    /// The one ending a refusal of this reading carries.
    #[must_use]
    pub fn ending(&self) -> String {
        LeverOnly {
            lever: SPLIT_COINCIDENCE_RECOURSE,
        }
        .recourse(self.arm(), Reading::Build)
    }
}

impl core::fmt::Display for SectorReading {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Escalated(diag) => diag.payload().fmt(f),
            Self::GrazeAgainstNeighbours { margin } => write!(
                f,
                "a wall the plane grazes holds its material across from the edges beside it \
                 (margin {margin:e})"
            ),
            Self::SmoothEdgeDisagrees(TangencyReadings { tangent, clear, .. }) => write!(
                f,
                "the faces at a smooth edge in the plane read it tangent (margin {tangent:e}) and \
                 clear (margin {clear:e}), each over its own extent"
            ),
        }
    }
}

impl From<BandError> for SplitReduceError {
    fn from(e: BandError) -> Self {
        Self::Band(e)
    }
}

impl SplitReduceError {
    /// The operand gate's refusal ([`AtRestBody::gate_unverdicted`]),
    /// typed for the split.
    pub(crate) fn unfinished(unfinished: Unfinished) -> Self {
        match unfinished {
            Unfinished::Scaffolding(errors) => Self::ScaffoldingOperand { errors },
            Unfinished::InsideOut(errors) => Self::InsideOutOperand { errors },
            Unfinished::Unjoined(errors) => Self::UnjoinedOperand { errors },
        }
    }
}

impl From<EulerOpError> for SplitReduceError {
    fn from(e: EulerOpError) -> Self {
        Self::Euler(e.from_driver())
    }
}

/// The recourse a split's escalations carry: a split takes no
/// declarations, so its plane is the first lever.
pub(crate) use geom_core::SPLIT_PLANE_RECOURSE as SPLIT_COINCIDENCE_RECOURSE;

impl core::fmt::Display for SplitReduceError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        use geom_core::RANGE_RECOURSE;
        match self {
            Self::Band(e) => write!(f, "{e}"),
            Self::CurvedBooleanUnsupported { kind, .. } => write!(
                f,
                "the split plane may meet {}, and the split cannot cut such a face yet. \
                 Recourse: move the split plane clear of that face's bounding box",
                match kind {
                    geom::SurfaceKind::Approx => "an approximated spline face",
                    geom::SurfaceKind::Cone => "a cone face",
                    geom::SurfaceKind::Sphere => "a sphere face",
                    geom::SurfaceKind::Torus => "a torus face",
                    geom::SurfaceKind::Plane => "a plane face",
                    geom::SurfaceKind::Cylinder => "a cylinder face",
                    geom::SurfaceKind::Nurbs => "a spline (NURBS) face",
                }
            ),
            Self::CurvedEdgeUnsupported { .. } => write!(
                f,
                "the split plane may meet an edge on a spline (NURBS) or spiric curve, which \
                 the split cannot take yet. Recourse: move the split plane clear of the \
                 bounding boxes of that edge and the faces beside it"
            ),
            // The fault's routing is the Boolean's too: a crossing
            // decision ends as that decision does, and a coincidence
            // between the plane and the conic takes the split's levers,
            // since a split takes no declaration.
            Self::CrossingEscalated { fault, .. } => {
                let diag = fault.diag();
                let ending = fault.decision().map_or_else(
                    || geom_core::lever_recourse(SPLIT_COINCIDENCE_RECOURSE, None),
                    |decision| decision.ending_of(&diag),
                );
                diag.undecided(fault.subject(), ending).fmt(f)
            }
            Self::TangencyUnsupported { .. } => write!(
                f,
                "the split plane is tangent to a curved face at a vertex, and a tangent \
                 cut is not supported yet. Recourse: move the split plane off the tangency"
            ),
            Self::KnifeEdge(k) => write!(f, "{k}"),
            Self::ScaffoldingOperand { .. } => {
                write!(f, "the body {}", Unfinished::SCAFFOLDING_REFUSAL)
            }
            Self::InsideOutOperand { .. } => {
                write!(f, "the body {}", Unfinished::INSIDE_OUT_REFUSAL)
            }
            Self::UnjoinedOperand { .. } => {
                write!(f, "the body {}", Unfinished::UNJOINED_REFUSAL)
            }
            Self::SliverVertex { diag, .. } => diag
                .undecided(
                    "whether a vertex lies on the split plane",
                    diag.lever_ending(SPLIT_COINCIDENCE_RECOURSE),
                )
                .fmt(f),
            Self::SliverSector { reading, .. } => write!(
                f,
                concat!(
                    geom_core::undecided!(
                        "which side of the split plane a face leaves a vertex on",
                        payload
                    ),
                    ". {}"
                ),
                reading,
                reading.ending()
            ),
            Self::SectionGermsEscalated { reading, diag, .. } => match reading {
                SectionGermReading::Line => write!(
                    f,
                    "a face round a vertex lies too nearly in the split plane to say where \
                     the plane leaves it ({}). Recourse: {SPLIT_COINCIDENCE_RECOURSE}",
                    diag.payload()
                ),
                SectionGermReading::Order => write!(
                    f,
                    "where the split plane leaves two faces round a vertex is too close to \
                     order ({}). Recourse: {SPLIT_COINCIDENCE_RECOURSE}",
                    diag.payload()
                ),
            },
            Self::SectionGermsUnordered { fault, .. } => {
                let what = match fault {
                    SectionGermFault::NoLine => {
                        "a face round a vertex leaves the split plane along no line"
                    }
                    SectionGermFault::Coincident => {
                        "the split plane leaves two faces round a vertex along one direction, \
                         curving alike off it"
                    }
                    SectionGermFault::Singular => {
                        "the split plane leaves two faces round a vertex along one direction, \
                         and one face has no curvature to read there"
                    }
                    SectionGermFault::Alternation => {
                        "material and gap do not alternate round a vertex in the split plane"
                    }
                    SectionGermFault::Crossing => {
                        "the corners of the section round a vertex cross one another"
                    }
                };
                write!(f, "{what}. Recourse: {SPLIT_COINCIDENCE_RECOURSE}")
            }
            Self::ApexSection { reading, .. } => {
                let what = match reading {
                    ApexSectionReading::ClosedOffApex => {
                        "the section it cuts in the cone reads as a closed curve clear of the tip"
                    }
                    ApexSectionReading::Refused(section) => match **section {
                        SplitJoinError::OpenConicChord { .. } => {
                            "the section it cuts in the cone reads as an open curve clear of \
                             the tip"
                        }
                        _ => "the section it cuts in the cone does not read there",
                    },
                    ApexSectionReading::NoRulingInside => {
                        "no line of the section it cuts in the cone leaves the tip where the \
                         plane crosses the cone there"
                    }
                    ApexSectionReading::BothRulingsInside => {
                        "both lines of the section it cuts in the cone leave the tip where the \
                         plane crosses the cone once there"
                    }
                };
                write!(
                    f,
                    "the split plane passes through the tip of a cone, and {what}. Recourse: \
                     {SPLIT_COINCIDENCE_RECOURSE}"
                )
            }
            Self::ApexUndecided { diag, .. } => write!(
                f,
                "a corner of a cone face lies too near the cone's axis to say whether it is \
                 the cone's tip ({}). Recourse: {SPLIT_COINCIDENCE_RECOURSE}",
                diag.payload()
            ),
            Self::NonFiniteSectorChord { .. } => write!(
                f,
                "a chord at a vertex has no finite length (a component overflows the norm \
                 or is not a number). Recourse: {RANGE_RECOURSE}"
            ),
            Self::UnderflowedSectorChord { .. } => write!(
                f,
                "a chord at a vertex has a length that underflows to zero. Recourse: \
                 {RANGE_RECOURSE}"
            ),
            Self::ConsecutiveOnSectors { vertex } => write!(
                f,
                "consecutive on-plane sectors survived at vertex {vertex:?}, which the \
                 coplanar gate rules out (kernel bug)"
            ),
            Self::StaleVertex { vertex } => {
                write!(f, "vertex {vertex:?} does not resolve in this body")
            }
            Self::LoneVertex { vertex } => write!(
                f,
                "vertex {vertex:?} is a lone vertex, with no neighborhood to classify"
            ),
            Self::NullEdgeAtVertex { vertex, edge } => write!(
                f,
                "null edge {edge:?} leaves vertex {vertex:?}, and a neighborhood is read off \
                 carriers a null edge does not have"
            ),
            Self::UnrecordedSide { vertex } => write!(
                f,
                "the side map holds no verdict for vertex {vertex:?}, which the neighborhood \
                 reaches"
            ),
            Self::UnboundedFace { face, vertex } => write!(
                f,
                "face {face:?}'s outer loop is the lone vertex {vertex:?}, so the face has no \
                 extent to meter a predicate across"
            ),
            Self::CrossingInsertion {
                edge,
                endpoints: (u, v),
                source,
            } => write!(
                f,
                "inserting the plane crossing on edge {edge:?} (endpoints {u:?}/{v:?}) \
                 refused: {source}"
            ),
            Self::Euler(e) => write!(f, "an Euler operation refused: {e}"),
            Self::LineageRow(refusal) => write!(
                f,
                "{}. {}",
                refusal.refused_by("split"),
                geom_core::KERNEL_DEFECT_ENDING
            ),
        }
    }
}

impl std::error::Error for SplitReduceError {}

/// The non-mutating prefix of the reduction — the finished-body gate,
/// the carrier gate and the cached vertex sweep — exposed so classification
/// ([`classify_neighborhood`]) can be inspected/reviewed independently
/// of surgery. Returns the per-vertex side cache and the ON set of the
/// body **as given** (crossing vertices only exist after
/// [`split_reduce`] inserts them).
///
/// # Errors
///
/// As the corresponding [`split_reduce`] stages.
pub fn vertex_sides<T: geom_core::Decide + crate::props::AtRestPolicy>(
    body: &AtRestBody<T>,
    plane: &SplitPlane<T>,
    tol: Tol,
) -> Result<(SecondaryMap<VertexKey, PlaneSide>, Vec<VertexKey>), SplitReduceError> {
    gate_finished(body, tol)?;
    carrier_gate_and_sides(body, plane, tol)
}

/// [`vertex_sides`] behind the finished-body gate.
pub(crate) fn carrier_gate_and_sides<T: geom_core::Decide>(
    body: &Body<T>,
    plane: &SplitPlane<T>,
    tol: Tol,
) -> Result<(SecondaryMap<VertexKey, PlaneSide>, Vec<VertexKey>), SplitReduceError> {
    let band = geom_core::Band::linear(tol)?;
    classify::carrier_gate(body, plane, band)?;
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
pub fn split_reduce<T: geom_core::Decide + crate::props::AtRestPolicy>(
    operand: &AtRestBody<T>,
    plane: &SplitPlane<T>,
    tol: Tol,
) -> Result<SplitReduction<T>, SplitReduceError> {
    gate_finished(operand, tol)?;
    reduce(operand, plane, tol)
}

/// **The split's finished-body gate**, every door's first read: what a
/// finished body promises, read where no verdict rides the operand
/// ([`AtRestBody::gate_unverdicted`], the Boolean's read too), refused
/// typed. The carrier gate (`classify::carrier_gate`) runs after it,
/// inside the pipeline.
pub(super) fn gate_finished<T: geom_core::Decide + crate::props::AtRestPolicy>(
    operand: &AtRestBody<T>,
    tol: Tol,
) -> Result<(), SplitReduceError> {
    operand
        .gate_unverdicted("the split's operand", geom_core::Band::linear(tol)?, tol)
        .map_err(SplitReduceError::unfinished)
}

/// [`split_reduce`] behind the finished-body gate.
pub(crate) fn reduce<T: geom_core::Decide + crate::props::AtRestPolicy>(
    operand: &Body<T>,
    plane: &SplitPlane<T>,
    tol: Tol,
) -> Result<SplitReduction<T>, SplitReduceError> {
    let band = geom_core::Band::linear(tol)?;
    // The crossing insertion and the null-edge insertion are this
    // door's operator sequence; tier 1 is paid once, over the reduced
    // body, rather than once per operator (`crate::surgery`). The
    // guard owns the borrow, so a refusal on the way closes the scope
    // by dropping it.
    let mut reduced = operand.clone();
    let mut body = reduced.begin_surgery();

    classify::carrier_gate(&body, plane, band)?;
    let tips = crate::sector_shape::SlitTips::of(&body);
    let (mut sides, mut on_vertices, on_margins) =
        classify::classify_vertices_margined(&body, plane, &tips, band)?;
    let edge_pieces =
        classify::insert_crossings(&mut body, plane, &mut sides, &mut on_vertices, tol)?;

    let mut null_edges = Vec::new();
    let mut apex_germs = SecondaryMap::new();
    let mut coincidences = Vec::new();
    for &v in &on_vertices {
        let (entries, rays) =
            neighborhood::classify_neighborhood_rays(&body, plane, &sides, v, &tips, band)?;
        let runs = insert::above_runs(&entries);
        // An operand vertex decided ON whose neighbourhood leaves two or
        // more runs on a side: pieces of the solid touch there. A
        // crossing's vertex was minted on the plane, so no margin
        // decided it, and one run is a cut, not a touch.
        if let (true, Some(&margin)) = (runs.len() >= 2, on_margins.get(v)) {
            coincidences.push(crate::Coincidence {
                cells: [
                    crate::RowCell::Input {
                        input: crate::Operand::A,
                        cell: crate::Cell::Vertex(v),
                    },
                    crate::RowCell::Tool,
                ],
                more: [None; 3],
                relation: crate::Relation::OnCarrier,
                site: crate::DecisionSite::SplitOn,
                margin,
                discharge: crate::Discharge::of(margin),
            });
        }
        let hood = insert::Neighborhood {
            vertex: v,
            entries: &entries,
            rays: &rays,
        };
        insert::insert_null_edges(
            &mut body,
            plane,
            &hood,
            &mut sides,
            &mut null_edges,
            &mut apex_germs,
            band,
        )?;
    }

    body.sweep_and_close();
    Ok(SplitReduction {
        body: reduced,
        plane: *plane,
        sides,
        on_vertices,
        null_edges,
        apex_germs,
        coincidences,
        edge_pieces,
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
    /// A side could not be sorted into solids ([`crate::pieces`]): the
    /// piece a shell of it belongs to could not be read.
    Pieces(crate::pieces::PieceSortError),
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

impl SplitError {
    /// The knife edge this refusal names, whichever stage raised it.
    #[must_use]
    pub fn knife_edge(&self) -> Option<&KnifeEdge> {
        match self {
            Self::Reduce(SplitReduceError::KnifeEdge(k))
            | Self::Finish(SplitFinishError::KnifeEdge(k)) => Some(k),
            _ => None,
        }
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
            Self::Pcurves(e) => write!(f, "{e}"),
            Self::Pieces(e) => write!(
                f,
                "a side of the split could not be sorted into solids: {e}"
            ),
        }
    }
}

impl std::error::Error for SplitError {}

/// Runs reduce + join on a scratch clone, returning the joined
/// reduction and the completed sections (shared prefix of [`split`]
/// and [`plane_section`]).
pub(crate) fn split_scratch<T: geom_core::Decide + crate::props::AtRestPolicy>(
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
    let mut red = reduce(operand, plane, tol)?;
    // The section join carves the reduced body through the Euler
    // operators; one scope, one sweep at the end of the phase.
    //
    // **Guardless, and this is one of the two sites where it has to
    // be.** A `Surgery` would borrow `red.body` for the scope's whole
    // span, and the next line hands `split_connect` the WHOLE
    // reduction — the sides table, the null-edge records and the body
    // together — so the guard and the call cannot both exist.
    // `red` is a local of this function and the failure path drops it,
    // which is what makes an early `?` between the two lines harmless.
    red.body.enter_surgery();
    let (completed, fragments) = join::split_connect(&mut red, band, tol)?;
    red.body.leave_surgery_and_sweep();
    Ok((red, completed, fragments))
}

/// **The direct split run through its join**: the scratch body as the
/// join leaves it, every null edge killed, before the finish and the
/// closing mint. Test vocabulary (`topo::test_support`), for the rows
/// that read the rows a face carries at that point.
///
/// # Errors
///
/// The reduction's or the join's refusal.
#[cfg(any(test, feature = "test-support"))]
pub(crate) fn through_the_join<T: geom_core::Decide + crate::props::AtRestPolicy>(
    operand: &AtRestBody<T>,
    plane: &SplitPlane<T>,
    tol: Tol,
) -> Result<Body<T>, SplitError> {
    gate_finished(operand, tol)?;
    Ok(split_scratch(operand, plane, tol)?.0.body)
}

/// **`split`** — plane-splitting of a solid (ch. 14 end to end):
/// reduce ([`split_reduce`]) → join (`splitconnect`) → finish
/// (`splitfinish`), composed functionally. The operand is never
/// touched; both sides come back as independent bodies
/// ([`SplitResult`]), with a side the plane misses entirely as the
/// typed [`SplitPart::Empty`] (never an empty body value).
///
/// Each side ends with maximal faces, except where a recorded skip
/// ([`SplitNaming::merge_skipped`]) leaves a curved run unglued, and
/// maximal edges: an operand face lying IN the plane and the section
/// faces beside it are one face, its merge recorded in
/// [`SplitNaming::merge_groups`].
///
/// Determinism (D9): every stage sweeps in arena/list order; the
/// joining order is the total lexicographic sort (`order` module) —
/// byte-identical replay is pinned by test.
///
/// # Pinches
///
/// At a vertex where pieces of one side meet (a **pinch**: pieces
/// meeting at a tip line, on either side of the plane or both), the
/// reduction keeps one vertex per piece round it on each side, the
/// tree of the section's corners there (`insert`), so each side's
/// pieces get their own copies and no section closes across another's
/// corner. Piece assignment does not depend on the plane's orientation:
/// `split(S, n)` is `split(S, −n)` with its sides swapped.
///
/// A plane grazing a curved wall makes no pinch: rule (b) lands a
/// convex graze with its material, and rule (a) refuses a concave one
/// in the reduction ([`SplitReduceError::KnifeEdge`]). The result's
/// section-face normals follow this call's plane (above face m = −n,
/// below face m = +n).
///
/// **The operand is a finished body** ([`AtRestBody`]); one that carries
/// no verdict (a dual's) is read for what the type promises, per solid,
/// before a several-solid operand is read as one solid
/// ([`SplitReduceError::ScaffoldingOperand`],
/// [`SplitReduceError::InsideOutOperand`]).
///
/// # Panics
///
/// On a kernel bug: an edge the body stores, re-minted or restated on its
/// own carrier and interval, whose span or winding reads certainly
/// refused at the run's band.
///
/// # Errors
///
/// [`SplitError`], each stage's typed refusals passed through whole (no
/// degenerate body is ever emitted). Each side is gated at tier 2
/// ([`SplitFinishError::ResultInvalid`]) and, as sorted into its
/// solids, at the scalar's at-rest gate
/// ([`crate::AtRestPolicy::gate_at_rest`], the gate the Boolean's result
/// passes, its findings typed as the Boolean types them:
/// [`SplitFinishError::ResultEscalated`] for shells in band,
/// [`SplitFinishError::ResultNotAtRest`] otherwise). So at a certifying
/// scalar no side is returned that [`crate::validate_geometric`]
/// refuses; at a dual the gate runs nothing beyond tier 2. A pinch side's
/// touching pieces need no contact record, since their copies of the
/// pinched vertex share its point key, and the ON verdict that pinched
/// them is in [`SplitResult::coincidences`].
pub fn split<T: geom_core::Decide + crate::props::AtRestPolicy>(
    operand: &AtRestBody<T>,
    plane: &SplitPlane<T>,
    tol: Tol,
) -> Result<SplitResult<T>, SplitError> {
    gate_finished(operand, tol)?;
    let band = geom_core::Band::linear(tol).map_err(SplitReduceError::from)?;
    let flat;
    let operand: &Body<T> = if operand.solids().nth(1).is_some() {
        let mut body = Body::clone(operand);
        body.merge_all_solids()
            .map_err(|e| SplitError::Finish(finish::SplitFinishError::from(e)))?;
        flat = body;
        &flat
    } else {
        operand
    };
    let finish::RawSplit {
        above,
        below,
        naming,
        coincidences,
    } = split_direct(operand, plane, tol)?;
    // Each side sorted into its pieces and gated at rest, the verdict kept
    // ([`finish::SplitPart::Body`]).
    let at_rest = |side, part| match part {
        finish::RawPart::Body(mut body) => {
            crate::pieces::sort_into_pieces(&mut body, band, tol, T::quad_lane(), None)
                .map_err(SplitError::Pieces)?;
            T::gate_at_rest_kept(body, tol)
                .map(finish::SplitPart::Body)
                .map_err(|errors| SplitError::from(SplitFinishError::not_at_rest(side, errors)))
        }
        finish::RawPart::Empty => Ok(finish::SplitPart::Empty),
    };
    Ok(SplitResult {
        above: at_rest(PlaneSide::Above, above)?,
        below: at_rest(PlaneSide::Below, below)?,
        naming,
        coincidences,
    })
}

/// One run of the split pipeline over an operand read as one solid.
///
/// The pcurve minting pass (M5 PR 6, C4) runs last, on each side that
/// carries material: this lane is where curved faces are minted, so it
/// is where their per-half-edge chart-image caches are minted and
/// certified (spec §1). Planar sides pick up nothing — planar faces
/// keep M2's derive-on-demand status — so an all-planar split is
/// bit-identical to before this pass existed.
fn split_direct<T: geom_core::Decide + crate::props::AtRestPolicy>(
    operand: &Body<T>,
    plane: &SplitPlane<T>,
    tol: Tol,
) -> Result<finish::RawSplit<T>, SplitError> {
    let (red, completed, fragments) = split_scratch(operand, plane, tol)?;
    let mut result = finish::split_finish(red, &completed, fragments, tol)?;
    for (side, part) in [
        (PlaneSide::Above, &mut result.above),
        (PlaneSide::Below, &mut result.below),
    ] {
        if let finish::RawPart::Body(body) = part {
            crate::validate::validate_closed(body)
                .map_err(|errors| SplitFinishError::ResultInvalid { side, errors })?;
            section_cones::fuse_section_cones(body, side, &mut result.naming, tol)?;
            // On this run's own output: a refusal discards it.
            finish::end_maximal(body, side, &mut result.naming, plane.normal.get(), tol)?;
            // The side's tier-2 gate again, on the body as joined.
            crate::validate::validate_closed(body)
                .map_err(|errors| SplitFinishError::ResultInvalid { side, errors })?;
            crate::pcurves::mint_pcurves(body, tol)?;
        }
    }
    Ok(result)
}
