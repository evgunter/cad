//! The **chord-join core**: ch. 14 Program 14.10's `join`/`cut`
//! mechanics, and the section-chord geometry they mint with — one
//! implementation, shared by the two lanes that join null edges into
//! section polygons.
//!
//! # What the core is
//!
//! [`ChordJoiner`] connects two null-edge halves with up to two real
//! chord edges (head↔head and tail↔tail — each connecting edge's
//! endpoints lie on ONE side, pinned by test), and retires a
//! fully-joined null edge:
//!
//! `join(h1 = old end, h2 = new half)`:
//!
//! - same loop ⇒ `mef(Chords { he1: h1, he2: next(h2) })` (the book's
//!   `lmef(h1, h2->nxt)`; our [`MefSite::Chords`] documents the same
//!   run association, so the argument pair ports literally — and the
//!   mirror test pins the outcome, not the citation, tol), guarded by
//!   `prev(prev(h1)) != h2` (adjacent ⇒ the chord already exists);
//! - different loops ⇒ `mekr` with the **ring chosen structurally**
//!   (the loop that is not the face's outer; the book's fixed
//!   `lmekr(h1, h2->nxt)` argument order assumes GWB's list layout —
//!   ours is explicit outer/ring data);
//! - then the second chord `mef(Chords { he1: h2, he2: next(h1) })`
//!   guarded by `next(next(h1)) != h2`; if the first `mef` split a
//!   face that still owns rings, the rings are re-homed by trilean
//!   containment ([`crate::splitting::containment`] on a plane,
//!   [`chart_ring_side`] on a cylinder wall's chart) +
//!   [`Body::ring_move`] — the `laringmv` step (lkemr/ring-placement
//!   mirror site).
//!
//! `cut(edge)` retires a fully-joined null edge: halves in different
//! loops ⇒ `kef` (merge the two sliver faces — the killed side must be
//! a join-minted sliver, asserted); same loop ⇒ the section polygon is
//! COMPLETE and `kemr` leaves the 2-loop null face, which the core
//! hands back as [`CutOutcome::Completed`] with the roles UNRESOLVED.
//!
//! **Role resolution and side-specific certification are the callers'**
//! — that is what makes this core side-agnostic. The split lane
//! resolves roles by membership of its minted above-copy vertex set and
//! certifies the section area (`split_section_area`, refusing a
//! zero-area polygon as [`SplitJoinError::DegenerateSection`]); the
//! boolean lane resolves them from its own germ records. Neither
//! posture is visible here.
//!
//! # Why the code is here and not in either lane
//!
//! It was born inside one half **by instruction**, not by drift: the
//! M3 plan (RATIFIED #42) item 5 said *"Ch. 14 join reused
//! with A↔B correspondence disambiguation"*, and ch. 14's join is the
//! split lane's. So the boolean joining imported [`ChordJoiner`],
//! [`CutOutcome`], [`SectionCtx`] and [`SplitJoinError`] out of
//! `splitting/join.rs`, while `splitting/`
//! reciprocated by hosting the [`JoinLane::BoolPlanar`] arm and
//! [`bool_planar_chord_spec`], which only the boolean reaches. The
//! three-way [`JoinLane`] threaded through [`chord_spec`] was the
//! visible cost of a shared core with no home of its own.
//!
//! This module is that home — a **top-level sibling** of `boolean/` and
//! `splitting/`, like [`crate::sector_shape`] and
//! [`crate::sector_face`], so neither half hosts the other's core.
//! `JoinLane::BoolPlanar` is NOT deleted by the move and was never the
//! defect: the planar side of a curved germ pair sections the OTHER
//! operand's wall, which must arrive by value. What changes is that
//! both arms of a shared enum now live in shared scope, instead of one
//! lane hosting the other's.
//!
//! # The section-chord geometry
//!
//! [`chord_spec`] answers what curve a chord between two vertices of a
//! divided face rides: `None` for planar faces (the straight-chord
//! lane), and for a curved face the section conic of the face's
//! carrier against the section plane — or, on the
//! [`JoinLane::BoolPlanar`] arm, of the plane against the partner wall.
//! Which of the conic's two arcs between the vertices is not asked of
//! the face: the lane that paired the chord's two ends already decided
//! it, and hands it over as the section's direction of departure at
//! each end ([`Leave`]) — the boolean's germ direction, the split's
//! conic walk. The chord only orients the arc along it
//! (`chord_arc_leave`), so a chord reads no chart and every conic the
//! table mints, on any carrier and at any tilt, takes its arc the same
//! way.

use geom_brep::{EdgeCurveSpec, Pcurve, chart_pcurve};
use geom_core::{
    Band, BandError, Decide, Indeterminate, InfSpeed, Margin, Point3, Real, Sign, UnitVec3, Vec3,
};
use slotmap::SecondaryMap;

use crate::body::Body;
use crate::entity::{EdgeKey, EntityId, FaceKey, HalfEdgeKey, LoopBoundary, LoopKey, VertexKey};
use crate::euler::{EulerOpError, FaceSurface, MefSite};
use crate::euler_ring::MekrSite;
use crate::geometry::SurfaceKey;
use crate::null::CurveGeom;
use crate::splitting::containment::{LoopContainment, PointInLoopError, point_in_loop};
use crate::splitting::rules::face_extent;
use crate::validate::decide;
use geom_core::Tol;

/// Why a curved face's crossings could not be paired along the face's
/// section conic (`splitting::join`'s conic pairing): the conic's
/// heading at a crossing — which way along it runs into the face — is
/// what pairs them, and here it did not.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConicCrossingsCase {
    /// At a crossing the plane runs within the band of tangent to the
    /// face's wall (**`split_join_conic_heading`** Zero), so the
    /// crossing neither enters nor leaves the face definitely. No
    /// shipped fixture reaches it.
    Grazing,
    /// Taken along the conic, the crossings do not alternate between
    /// entering the face and leaving it. Crossings closer along the
    /// conic than the band read as one point and keep their insertion
    /// order (`split_join_line_gap` Zero), so two crossings of the face
    /// at one point — the plane through a vertex of it — can land
    /// here, as can every crossing of the face at one point, or up/down
    /// senses that disagree with the geometry. No shipped fixture
    /// reaches it.
    NotAlternating,
}

impl core::fmt::Display for ConicCrossingsCase {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Grazing => write!(
                f,
                "the plane grazes the face at one of its crossings, so the crossing neither \
                 enters nor leaves it definitely"
            ),
            Self::NotAlternating => write!(
                f,
                "along the section the face's crossings do not alternate between entering \
                 and leaving it"
            ),
        }
    }
}

/// Typed failure of the joining step.
#[derive(Debug)]
pub enum SplitJoinError {
    /// The exact-order comparator escalated (interval lane only).
    OrderEscalated {
        /// Diagnostics (named predicate inside).
        diag: Indeterminate,
    },
    /// The section-area or containment machinery escalated.
    Escalated {
        /// The site face (the null face or the ring's face).
        face: FaceKey,
        /// Diagnostics.
        diag: Indeterminate,
    },
    /// A completed section polygon bounds zero area: no degenerate body
    /// is ever emitted.
    ///
    /// A run reaches it two ways: a below-side PINCH (pieces meeting
    /// at a tip line on the NEGATIVE side of the run's plane normal,
    /// where the ch. 14 insertion mints no vertex copies), and a
    /// concave GRAZE of a curved face (the plane tangent to a hole's
    /// wall from inside, whose contact closes a polygon of its own). A
    /// plane tangent along a convex edge or to a convex wall does not
    /// reach it: rule (b) classifies that entry with its material, and
    /// the contact mints nothing. Since M3 PR 6a (D7) the public
    /// [`crate::splitting::split`] consumes this refusal as the pinch
    /// trigger and reruns under the mirrored plane — where pinched
    /// fans are ABOVE runs and mint their copies — so a pinch's
    /// success is orientation-independent. The rerun cannot tell a
    /// graze from a pinch, so it reruns a graze too; a graze alone
    /// refuses again there, and one whose contact meets a real section
    /// refuses [`Self::SectionSpur`]. The error surfaces from
    /// [`crate::splitting::split`] when the mirror run also refuses,
    /// and from the join lane directly (e.g.
    /// [`crate::splitting::plane_section`], which has no sides to
    /// swap).
    DegenerateSection {
        /// The completed null face.
        face: FaceKey,
    },
    /// A completed section polygon of positive area carries a SPUR: its
    /// loop runs out along a straight edge the plane only touches and
    /// straight back. The spur is a concave graze's contact joined
    /// into a real section's polygon instead of closing one of its own;
    /// it would leave a zero-width slit in both halves, with two copies
    /// of every vertex along it on one side. Refused, as the graze
    /// standing alone is ([`Self::DegenerateSection`]); no degenerate
    /// body is ever emitted.
    SectionSpur {
        /// The completed null face.
        face: FaceKey,
    },
    /// Ring re-homing could not decide: the walk escalated, exhausted
    /// its schedule, or met an edge of the divided face's outline it
    /// cannot cross ([`PointInLoopError::Uncrossable`]).
    RingHoming(PointInLoopError),
    /// Every vertex of a ring landed ON the run dividing its face off,
    /// so no vertex says which side the ring is on: a ring an
    /// ill-conditioned operand put on the run, or a pierce's strut at a
    /// pinch, every vertex of which is the pinch point.
    RingHomingAmbiguous {
        /// The undecidable ring.
        ring: LoopKey,
    },
    /// Loose ends survived the sweep — the null-edge set does not
    /// close into section polygons. Both lanes read a line edge's side
    /// at its far vertex, so a vertex's germs agree with the sections
    /// that reach it; what can still disagree is an edge leaving a
    /// vertex tangent to the surface it is read against (read to first
    /// order in the boolean, to second in the split), a coincidence of
    /// the two solids the join has no rule for, a corrupt reduction, or
    /// a kernel defect.
    UnpairedLooseEnds {
        /// How many halves remained.
        count: usize,
    },
    /// A closed section loop has ONE site: a closed curve of one solid
    /// (a circle edge, or a face's closed section) lies in a face of the
    /// other and meets nothing else of it, so the one vertex on it holds
    /// both ends of the loop. The join pairs ends at two distinct sites
    /// and has no arm that closes a loop on itself; refused typed, before
    /// the loose ends are counted.
    SingleSiteSectionLoop {
        /// How many such loops.
        count: usize,
    },
    /// A section loop mixed above copies with below-side vertices —
    /// the joining invariant (heads join heads, tails join tails)
    /// failed, loudly.
    ///
    /// The join's role probe reads each copy's side through
    /// `point_in_solid`, so a misread there arrives here too: a planar
    /// arm that took an arc-bounded cap for the polygon through its
    /// vertices would see the cap as transparent and return a mixed
    /// loop. A box driven through a cylinder cap and a pocket engraved
    /// in one (`editor-core/tests/pierce_ring_engraving.rs`) are the
    /// poses that pin that arm's arc crossing.
    SectionLoopMixed {
        /// The offending null face.
        face: FaceKey,
    },
    /// Neither section loop of a null face reads which side of the other
    /// solid it lies on: every witness the role probe holds for either
    /// loop's regions lies on the other solid's boundary or within its
    /// band of it. A crossing's two flanks cannot both read that way
    /// unless their faces are curved and the witness can only sit on
    /// their boundaries — the frontier of
    /// `work/cleave/the-uncut-shell-witness-reads-no-curved-face-interior`
    /// — or the two solids' faces lie within the band of each other (a
    /// settled in-band coincidence,
    /// `topo/tests/door_backstop_settled_residue.rs`). No kernel defect.
    SectionLoopUndecided {
        /// The null face whose loops' roles went unread.
        face: FaceKey,
    },
    /// `cut` found neither side of an interior null edge to be a
    /// join-minted sliver face (kernel bug, loudly).
    CutInvariant {
        /// The null edge being retired.
        edge: EdgeKey,
    },
    /// A traversal failed mid-join: the arena did not resolve a key the
    /// join was handed, or a cycle did not close on the half-edge it
    /// was walking to. `entity` is what the join was reading — the
    /// TOPOLOGICAL entity even when the lookup that failed was the
    /// geometry hanging off it (a vertex's point, a face's surface, an
    /// edge's curve), because that is the entity a caller can find in
    /// the body it holds.
    ///
    /// This arm is corruption ONLY. A state the join's own construction
    /// rules out is [`Self::SectionInvariant`], which says which.
    Corrupt {
        /// The entity the join was reading when it could not continue.
        entity: EntityId,
    },
    /// The exact-order band's constants did not construct —
    /// structurally impossible for the two literal bit patterns, typed
    /// rather than panicked (no panic paths in operator code).
    Band(BandError),
    /// An underlying Euler operation refused.
    Euler(EulerOpError),
    /// The C5 section classification refused while minting a curved
    /// face's section chord (M5 PR 5) — the typed table verdict nested
    /// whole (escalations carry the shared recourse through it).
    Section {
        /// The face being divided.
        face: FaceKey,
        /// The table's refusal.
        source: geom_brep::SectionError,
    },
    /// A cone face's chart window would be read across its APEX, where
    /// every azimuth maps to one point and no branch pin carries: the
    /// walk reached the apex, or the face meets it in a way no single
    /// chart lift closes (twice, from both nappes, or around a ring —
    /// [`cone_apex_closure`]'s `Open`). A window guessed there reads
    /// the face's complement without any later check seeing it.
    ApexUnlifted {
        /// The cone face whose window was read.
        face: FaceKey,
    },
    /// A curved-section invariant failed. Two DISTINCT populations
    /// share this arm (M5 PR 9 fix pass — read `what` to tell them
    /// apart, it says which):
    ///
    /// - **kernel bugs, loudly**: states the lane's own construction
    ///   makes unreachable (an empty classification under a minted
    ///   chord, a section frame with no chart orientation, a run
    ///   edge with no chart image on the shipped carriers) — reaching
    ///   one means a lane invariant is broken, never user geometry;
    /// - **deliberate typed frontiers**: configurations the M5 lane
    ///   refuses BY DESIGN with the front door named in `what` (a
    ///   tangent germ pair inside the boolean zip — a touching
    ///   configuration, the M5 envelope's frontier; a non-cylinder
    ///   planar-side germ partner — the PR 9c arms).
    SectionInvariant {
        /// The face being divided.
        face: FaceKey,
        /// What failed.
        what: &'static str,
    },
    /// A ring on a curved face that is not a cylinder wall: the ring
    /// lane winds an island, and re-homes a ring, on a cylinder wall's
    /// chart only. Valid input whose lane is not yet built (D2 addendum
    /// row 2), refused typed; no reachable pose built a sphere island
    /// when the cylinder reading was written.
    RingOffCylinderChart {
        /// The face carrying the ring.
        face: FaceKey,
        /// Its surface kind.
        kind: geom::SurfaceKind,
    },
    /// A curved face crossed more than twice could not have its
    /// crossings paired along its section conic, which is the only
    /// pairing that keeps each chord on an arc inside the face. The
    /// case says which way the heading reading failed.
    SectionCrossings {
        /// The curved face whose crossings were being paired.
        face: FaceKey,
        /// Why they could not be.
        case: ConicCrossingsCase,
        /// The band the headings were decided against.
        band: Band,
    },
}

impl From<EulerOpError> for SplitJoinError {
    fn from(e: EulerOpError) -> Self {
        Self::Euler(e)
    }
}

impl From<PointInLoopError> for SplitJoinError {
    fn from(e: PointInLoopError) -> Self {
        Self::RingHoming(e)
    }
}

/// The recourse the section join's escalations carry. The join runs
/// under a split, which takes no declarations, and under a Boolean,
/// which does; the levers true at both are the geometry and the
/// tolerance, and `BooleanError::Join` adds the declaration.
use geom_core::NO_DECLARATION_RECOURSE as JOIN_RECOURSE;

impl core::fmt::Display for SplitJoinError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.render(f, JOIN_RECOURSE)
    }
}

/// A [`SplitJoinError`] as a Boolean shows it: the Boolean takes
/// declarations, so its escalations offer the shared
/// [`geom_core::COINCIDENCE_RECOURSE`] where the join's own `Display`
/// (which a split shares) offers only the geometry and the tolerance.
pub(crate) struct UnderBoolean<'a>(pub(crate) &'a SplitJoinError);

impl core::fmt::Display for UnderBoolean<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.0.render(f, geom_core::COINCIDENCE_RECOURSE)
    }
}

impl SplitJoinError {
    /// The sentence, with the recourse every ill-conditioned arm
    /// states supplied by the caller that knows which levers its reader
    /// has: [`JOIN_RECOURSE`] under a split, the shared coincidence
    /// recourse under a Boolean ([`UnderBoolean`]).
    fn render(&self, f: &mut core::fmt::Formatter<'_>, recourse: &str) -> core::fmt::Result {
        match self {
            Self::OrderEscalated { diag } => write!(
                f,
                "the order of two section points is too close to call ({}). Recourse: \
                 {recourse}",
                diag.payload()
            ),
            Self::Escalated { diag, .. } => write!(
                f,
                "where a section runs across a face is too close to call ({}). Recourse: \
                 {recourse}",
                diag.payload()
            ),
            Self::DegenerateSection { .. } => write!(
                f,
                "a section is degenerate: it bounds zero area, where the plane only \
                 grazes a hole's wall from inside or pinches the solid. Recourse: \
                 {recourse}"
            ),
            Self::SectionSpur { .. } => write!(
                f,
                "the plane only touches the solid along a line while cutting it elsewhere, \
                 and the section would run out along that line and back. Recourse: \
                 {recourse}"
            ),
            Self::RingHoming(e) => match e {
                crate::splitting::PointInLoopError::Escalated { diag, .. } => write!(
                    f,
                    "which piece a hole loop falls in is too close to call ({}). Recourse: \
                     {recourse}",
                    diag.payload()
                ),
                crate::splitting::PointInLoopError::RayExhausted { .. } => write!(
                    f,
                    "every test ray grazed the divided face's boundary, so which piece \
                     holds a hole loop is ill-conditioned at this tolerance. Recourse: \
                     {recourse}"
                ),
                crate::splitting::PointInLoopError::CorruptLoop { .. } => {
                    write!(f, "re-homing a hole loop refused: {e}")
                }
                crate::splitting::PointInLoopError::Uncrossable(u) => write!(
                    f,
                    "which piece holds a hole loop cannot be read: {u}. Recourse: {recourse}"
                ),
                crate::splitting::PointInLoopError::OffPlane(o) => write!(
                    f,
                    "which piece holds a hole loop cannot be read: {o}. Recourse: {recourse}"
                ),
            },
            Self::RingHomingAmbiguous { .. } => write!(
                f,
                "every vertex of a hole loop lies on the boundary of the piece being \
                 divided off, so which piece holds it cannot be decided. Recourse: {recourse}"
            ),
            Self::UnpairedLooseEnds { count } => write!(
                f,
                "{count} section ends found no partner: the sides read at the vertices do \
                 not close into section polygons. An edge leaving a vertex tangent to the \
                 surface it is read against, whose side is then read to finite order, can \
                 cause this, as can a coincidence of the two solids the join has no rule \
                 for yet"
            ),
            Self::SingleSiteSectionLoop { count } => write!(
                f,
                "{count} section loop(s) close through a single vertex: a closed curve of \
                 one solid lies in a face of the other and meets nothing else of it, and a \
                 loop joined at one site is not built. {}",
                geom_core::NOT_YET_ENDING
            ),
            Self::SectionLoopUndecided { .. } => write!(
                f,
                "which of a section's two loops bounds the result cannot be read: every \
                 point it is read at lies on a curved face's boundary or too near the \
                 other part. {}",
                geom_core::NOT_YET_ENDING
            ),
            Self::SectionLoopMixed { face } => write!(
                f,
                "null face {face:?} has a side-mixed section loop: its two copies do not \
                 read as one above and one below, so which one bounds the result is \
                 undecided"
            ),
            Self::CutInvariant { edge } => write!(
                f,
                "neither face flanking null edge {edge:?} is a sliver (kernel bug)"
            ),
            Self::Corrupt { entity } => {
                write!(f, "the join's traversal failed at {entity} (corrupt body)")
            }
            Self::Band(e) => write!(f, "{e}"),
            Self::Euler(e) => write!(f, "an Euler operation refused: {e}"),
            // The carrier's constructor states its escalation for a
            // caller that could build the circle instead; no reader of
            // a join can, so the join keeps the constructor's subject
            // and offers its own door's levers.
            Self::Section {
                source: geom_brep::SectionError::Carrier(geom::EllipseInvalid::Escalated(diag)),
                ..
            } => write!(
                f,
                "{} is undecided for the section through a curved face: {}. Recourse: \
                 {recourse}",
                geom::EllipseInvalid::escalated_subject(diag),
                diag.payload()
            ),
            Self::Section { source, .. } => {
                write!(f, "the section through a curved face refused: {source}")
            }
            Self::ApexUnlifted { .. } => write!(
                f,
                "a cone face's azimuth window would be read across its apex, where the chart \
                 has no azimuth: the walk reached the apex, or the face meets it in a way no \
                 single chart lift closes (twice, from both nappes, or around a ring)"
            ),
            Self::SectionInvariant { face, what } => {
                write!(f, "curved-section invariant at face {face:?}: {what}")
            }
            Self::RingOffCylinderChart { kind, .. } => write!(
                f,
                "a cut passes through a {kind:?} face without reaching its boundary, leaving \
                 a ring the join reads only on a cylinder wall. Recourse: move the cut so it \
                 crosses the face's edge, or divide the face there first"
            ),
            Self::SectionCrossings { case, band, .. } => write!(
                f,
                "a curved face's crossings cannot be paired along the section: {case} \
                 ('split_join_conic_heading', band ({:e}, {:e})). Recourse: {recourse}",
                band.zero(),
                band.escalate(),
            ),
        }
    }
}

impl std::error::Error for SplitJoinError {}

/// The corruption refusal naming the half-edge the join was reading.
pub(crate) fn corrupt_he(he: HalfEdgeKey) -> SplitJoinError {
    SplitJoinError::Corrupt {
        entity: EntityId::HalfEdge(he),
    }
}

/// The corruption refusal naming the loop the join was reading.
pub(crate) fn corrupt_loop(r#loop: LoopKey) -> SplitJoinError {
    SplitJoinError::Corrupt {
        entity: EntityId::Loop(r#loop),
    }
}

/// The corruption refusal naming the face the join was reading.
pub(crate) fn corrupt_face(face: FaceKey) -> SplitJoinError {
    SplitJoinError::Corrupt {
        entity: EntityId::Face(face),
    }
}

/// The corruption refusal naming the edge the join was reading.
pub(crate) fn corrupt_edge(edge: EdgeKey) -> SplitJoinError {
    SplitJoinError::Corrupt {
        entity: EntityId::Edge(edge),
    }
}

/// The corruption refusal naming the vertex the join was reading.
pub(crate) fn corrupt_vertex(vertex: VertexKey) -> SplitJoinError {
    SplitJoinError::Corrupt {
        entity: EntityId::Vertex(vertex),
    }
}

/// Chord-mef fragment rows: `(new face, divided-from face)` in mint
/// order (naming emission, M4 PR 3).
pub(crate) type FragmentRows = Vec<(FaceKey, FaceKey)>;

/// The point of a vertex. Either empty lookup means the same thing
/// here — a body that reached this lane corrupt — so the read-back
/// door's discriminated reference collapses to one verdict.
pub(crate) fn vertex_point<T: Decide>(
    body: &Body<T>,
    v: VertexKey,
) -> Result<Point3<T>, SplitJoinError> {
    crate::readback::vertex_point_ref(body, v).map_err(|_| corrupt_vertex(v))
}

/// The outcome of retiring a fully-joined null edge (`cut`):
/// either the section polygon completed (a 2-loop null face remains,
/// roles unresolved — the caller resolves them from its own role data)
/// or two in-progress slivers were merged.
#[derive(Clone, Copy, Debug)]
pub(crate) enum CutOutcome {
    /// The kemr completion: `face` is the 2-loop null face, `ring` the
    /// loop kemr demoted (the caller must not assume which side it is).
    Completed {
        /// The completed 2-loop null face.
        face: FaceKey,
        /// The loop kemr left as the ring.
        ring: LoopKey,
    },
    /// An interior null edge: kef merged two slivers.
    Merged,
}

/// The reusable chord-join core (ch. 14 Program 14.10's `join`/`cut`
/// mechanics), shared between the split sweep and the boolean joining
/// (M3 PR 5 — "the ch. 14 join reused"): chord `mef`/`mekr` insertion,
/// ring re-homing (`laringmv`), and null-edge retirement. Role
/// resolution and any side-specific certification stay with the
/// callers — the core is side-agnostic.
pub(crate) struct ChordJoiner {
    /// Faces minted by `join`'s mefs — the sliver (section-polygon-in-
    /// progress) faces `cut` may kill.
    slivers: SecondaryMap<FaceKey, ()>,
    /// Naming emission (M4 PR 3): every face the chord mefs minted,
    /// paired with the face it was divided from, in mint order —
    /// `(new face, divided-from face)` at CALL-TIME keys. Rows are
    /// historical (a recorded face may later die — slivers killed by
    /// `cut`, discarded material at finish); consumers filter to the
    /// entities alive in the body they hold. This is mint-time wiring
    /// knowledge, recorded so the naming layer never reconstructs
    /// parentage by inspection (NAMING-DESIGN N4: no post-hoc scans).
    fragments: Vec<(FaceKey, FaceKey)>,
    /// The run band (ring re-homing containment).
    band: Band,
}

impl ChordJoiner {
    /// A fresh core.
    pub(crate) fn new(band: Band) -> Self {
        Self {
            slivers: SecondaryMap::new(),
            fragments: Vec::new(),
            band,
        }
    }

    /// Consumes the recorded `(new face, divided-from face)` rows
    /// (naming emission; see the field docs).
    pub(crate) fn take_fragments(&mut self) -> FragmentRows {
        core::mem::take(&mut self.fragments)
    }
}

/// The split lane's section-geometry context (M5 PR 5): the split
/// plane plus the lazily-minted auxiliary plane SURFACE the conic
/// section chords' `Intersection` descriptions resolve against (minted
/// once per split, at the first conic chord; the finish step's
/// promoted section faces carry their own oriented copies — this one
/// stays alive through description references, the carve orphan
/// sweep's rule). The boolean builds one per wall-side join of a
/// plane×curved germ pair, from the germ plane.
pub(crate) struct SectionCtx<T: Real> {
    /// A point on the section plane.
    pub(crate) origin: Point3<T>,
    /// The section plane's normal. Its sense is the caller's: the
    /// split's Above side, or a boolean germ plane's chart normal,
    /// which names no material side.
    pub(crate) normal: UnitVec3<T>,
    /// The minted auxiliary plane surface, once needed.
    pub(crate) plane_key: Option<SurfaceKey>,
}

/// Which curved-section lane a [`ChordJoiner::join`] call runs
/// (M5 PR 9 generalized the M3 `Option<&mut SectionCtx>`):
///
/// - [`JoinLane::Planar`] — the boolean's lane for a plane×plane germ
///   pair: straight chords only (`chord_spec` returns `None` for the
///   planar divided face). Its adjacency skip reads the segment's
///   locus, so no section rides along.
/// - [`JoinLane::Split`] — the split lane's conic machinery, AND the
///   boolean's WALL-side chord (the germ pair's plane arrives as a
///   transient context).
/// - [`JoinLane::BoolPlanar`] — the boolean's PLANAR-side chord of a
///   curved germ pair: the divided face is the plane, and the partner
///   wall arrives by value from the OTHER operand.
///
/// Every lane takes its arc from the join's [`Leave`].
pub(crate) enum JoinLane<'a, T: Real> {
    /// Straight chords only.
    Planar,
    /// The split lane / boolean wall-side conic lane.
    Split(&'a mut SectionCtx<T>),
    /// The boolean planar-side chord of a curved germ pair.
    BoolPlanar {
        /// The partner wall surface (value; from the other operand).
        wall: geom::Surface<T>,
        /// The aux wall key in THIS body (minted once, caller-cached).
        partner_key: &'a mut Option<SurfaceKey>,
    },
    /// A section segment that is an edge of BOTH solids: its chords are
    /// copies of that edge ([`along_edge_spec`]) and no section is
    /// read, since the germ's face pair there may be two faces on one
    /// carrier, with no section between them.
    AlongEdge,
}

impl<T: Real> JoinLane<'_, T> {
    /// A reborrowing view (the join mints up to two chords per call).
    fn reborrow(&mut self) -> JoinLane<'_, T> {
        match self {
            JoinLane::Planar => JoinLane::Planar,
            JoinLane::Split(ctx) => JoinLane::Split(ctx),
            JoinLane::BoolPlanar { wall, partner_key } => JoinLane::BoolPlanar {
                wall: wall.clone(),
                partner_key,
            },
            JoinLane::AlongEdge => JoinLane::AlongEdge,
        }
    }
}

/// The section conic's frame — the curve both chord lanes take an arc
/// of.
pub(crate) struct SectionConic<T: Real> {
    /// The conic's centre.
    center: Point3<T>,
    /// The section plane's normal — the conic's own axis, about which
    /// its parameter runs ccw.
    normal: Vec3<T>,
    /// The major direction.
    major: Vec3<T>,
    /// The semi-axis along `major` (the radius, for a circle).
    sa: T,
    /// The semi-axis along `normal × major`.
    sb: T,
    /// The carrier itself.
    carrier: geom::Curve3<T>,
}

impl<T: Real> SectionConic<T> {
    /// The eccentric anomaly of a point on the conic: `θ` with
    /// `p = center + sa·cos θ·major + sb·sin θ·(normal × major)`.
    pub(crate) fn param(&self, p: Point3<T>) -> T {
        let d = p - self.center;
        (d.dot(self.normal.cross(self.major)) / self.sb).atan2(d.dot(self.major) / self.sa)
    }

    /// The conic's derivative in `θ` (metres per radian).
    pub(crate) fn tangent(&self, theta: T) -> Vec3<T> {
        self.normal.cross(self.major) * (self.sb * theta.cos())
            - self.major * (self.sa * theta.sin())
    }
}

/// What the C5 table made of `plane × wall` for a chord that has to
/// ride it.
///
/// `Straight` and `Tangent` are handed BACK rather than decided here:
/// the two chord lanes mean different things by them — the split lane
/// mints a tangent chord along the ruling, the boolean zip refuses a
/// tangent germ pair as a touching frontier — and that difference is
/// the whole of what the two lanes do not share.
pub(crate) enum SectionCase<T: Real> {
    /// A conic to select an arc of.
    Conic(SectionConic<T>),
    /// Ruling seams: the straight chord is the honest carrier, so the
    /// caller mints no spec.
    Straight,
    /// The tangent locus, as the table constructed it.
    Tangent(geom::Curve3<T>),
}

/// The section of the surface PAIR `(s1, s2)` under THE C5 table, as
/// the conic a chord takes an arc of — **one implementation for both
/// chord lanes** — this classification was written twice in this file,
/// once per lane, differing only in the wording of its refusals and in
/// what it did with the tangent arm.
///
/// **Pair-general, not plane-first.** The arms wired today are
/// plane×cylinder, plane×cone and plane×sphere, and either order is accepted: the
/// caller hands over the pair it has, and which member is the plane is
/// this function's question rather than the caller's. A pair with no
/// arm — every curved×curved pair, today — refuses typed here, which
/// is the same discipline the germ-pair frame dispatch keeps
/// (`boolean::join::pair_section_frame`): a missing arm is never a
/// straight chord.
///
/// The sphere lane (M5 S13) classifies through `plane_sphere_section`
/// — an exact Circle, never a fitted chord — whatever the section's
/// tilt against the sphere's chart. The cylinder lane is PR 5/PR 9's
/// `plane_cylinder_section`; the cone lane is `plane_cone_section`.
fn section_case<T: Decide>(
    face: FaceKey,
    band: Band,
    s1: &geom::Surface<T>,
    s2: &geom::Surface<T>,
    extent: T,
) -> Result<SectionCase<T>, SplitJoinError> {
    let invariant = |what: &'static str| SplitJoinError::SectionInvariant { face, what };
    // The pair normalization: exactly one member must be the plane the
    // section rides. Two planes have no conic to select an arc of, and
    // a curved pair has no arm — both are named rather than folded
    // into the wall match below.
    let (plane_s, wall) = match (s1, s2) {
        (geom::Surface::Plane { .. }, geom::Surface::Plane { .. }) => {
            return Err(invariant(
                "a plane×plane pair reached the chord's section table — a planar pair's \
                 chord is straight and is minted by the planar lane, never here",
            ));
        }
        (geom::Surface::Plane { .. }, other) => (s1, other),
        (other, geom::Surface::Plane { .. }) => (s2, other),
        _ => {
            return Err(invariant(
                "a chord's section pair has no plane — the C5 arms this lane reads are \
                 plane×cylinder, plane×cone and plane×sphere, and a curved×curved pair has no conic \
                 to take an arc of; refused typed rather than defaulted to a straight chord. A \
                 sphere pair never arrives here: its section lies in the pair's radical \
                 plane, which the boolean's join hands each side as a plane×sphere pair",
            ));
        }
    };
    let table = |e: geom_brep::SectionError| match e {
        geom_brep::SectionError::Escalated(diag) => SplitJoinError::Escalated { face, diag },
        other => SplitJoinError::Section {
            face,
            source: other,
        },
    };
    if let geom::Surface::Sphere { .. } = wall {
        let sec = geom_brep::plane_sphere_section(plane_s, wall, band).map_err(table)?;
        let circle = match sec {
            geom_brep::PlaneSphereSection::Circle(c) => c,
            // C7: the tangent locus is a POINT — a touching
            // configuration, refused typed, never minted. (Both lanes
            // refuse it; only the cylinder's tangent LINE divides
            // them, which is why this arm is decided here.)
            geom_brep::PlaneSphereSection::TangentPoint(_) => {
                return Err(invariant(
                    "tangent plane×sphere germ pair under a minted chord — a touching \
                     configuration, the typed frontier of the supported envelope",
                ));
            }
            geom_brep::PlaneSphereSection::Empty => {
                return Err(invariant(
                    "empty plane×sphere classification under a minted chord",
                ));
            }
        };
        let &geom::Curve3::Circle {
            center,
            axis,
            radius,
            u_ref,
        } = &circle
        else {
            return Err(invariant(
                "plane×sphere classification carried a non-circle",
            ));
        };
        return Ok(SectionCase::Conic(SectionConic {
            center,
            normal: axis,
            major: u_ref,
            sa: radius,
            sb: radius,
            carrier: circle,
        }));
    }
    let conic =
        |carrier: geom::Curve3<T>| {
            section_conic(carrier).map(SectionCase::Conic).ok_or_else(|| {
            invariant("a conic classification carried a carrier that is neither ellipse nor circle")
        })
        };
    // The cone (C5's plane×cone arm): the tilted ellipse and the
    // axis-normal circle are conics to select an arc of; the apex lane's
    // generator pair is the ruling case, its tangent generator the
    // tangent one — the cylinder's parallel-axis lane, one kind over.
    if let geom::Surface::Cone { .. } = wall {
        return match geom_brep::plane_cone_section(plane_s, wall, extent, band).map_err(table)? {
            geom_brep::PlaneConeSection::TiltedEllipse(c)
            | geom_brep::PlaneConeSection::AxisNormalCircle(c) => conic(c),
            geom_brep::PlaneConeSection::ApexLinePair { .. } => Ok(SectionCase::Straight),
            geom_brep::PlaneConeSection::ApexTangentLine(line) => Ok(SectionCase::Tangent(line)),
            geom_brep::PlaneConeSection::ApexPoint(_) => Err(invariant(
                "apex-point plane×cone classification under a minted chord — the plane \
                 touches the cone at its apex alone",
            )),
        };
    }
    let sec = geom_brep::plane_cylinder_section(plane_s, wall, extent, band).map_err(table)?;
    match sec {
        geom_brep::PlaneCylinderSection::TiltedEllipse(c)
        | geom_brep::PlaneCylinderSection::Rim(c) => conic(c),
        geom_brep::PlaneCylinderSection::ParallelLines { .. } => Ok(SectionCase::Straight),
        // C7 (M5 PR 9): the tangent locus is CONSTRUCTED by
        // classification, never marched. What it MEANS is the caller's
        // (see the enum).
        geom_brep::PlaneCylinderSection::TangentLine(line) => Ok(SectionCase::Tangent(line)),
        geom_brep::PlaneCylinderSection::Empty => Err(invariant(
            "empty plane×cylinder classification under a minted chord",
        )),
    }
}

/// The frame of a section carrier: an ellipse's own axes, a circle's
/// radius twice. `None` for any other kind.
fn section_conic<T: Real>(carrier: geom::Curve3<T>) -> Option<SectionConic<T>> {
    let (center, normal, major, sa, sb) = match carrier {
        geom::Curve3::Ellipse {
            center,
            axis,
            major,
            minor,
            u_ref,
        } => (center, axis, u_ref, major, minor),
        geom::Curve3::Circle {
            center,
            axis,
            radius,
            u_ref,
        } => (center, axis, u_ref, radius, radius),
        _ => return None,
    };
    Some(SectionConic {
        center,
        normal,
        major,
        sa,
        sb,
        carrier,
    })
}

/// The candidate arc of `conic` from `th1` to `th2` (exact conic
/// parameters of the chord's ends) running forward `p1 → p2`: the ccw
/// arc as it stands, the cw arc on the axis-flipped carrier.
fn oriented_arc<T: Real>(
    conic: &SectionConic<T>,
    th1: T,
    th2: T,
    ccw: bool,
) -> (geom::Curve3<T>, T, T) {
    let tau = T::tau();
    // The arc's own span, forward from `th1`, in `[0, τ)`: a chord may
    // span more than half the conic. The window's jump is at a span of
    // zero, two ends sharing a conic parameter — a zero-length or a
    // whole-conic arc. Nothing here gates it: the boolean joins only
    // distinct sites (`bool_join_chord`); that the split's pairing does
    // is not established here.
    if ccw {
        let span = (th2 - th1).reduce_periodic(tau);
        (conic.carrier.clone(), th1, th1 + span)
    } else {
        // The cw arc: flip the carrier's axis so it runs forward.
        let span = tau - (th2 - th1).reduce_periodic(tau);
        let flipped = match conic.carrier.clone() {
            geom::Curve3::Ellipse {
                center,
                axis,
                major,
                minor,
                u_ref,
            } => geom::Curve3::Ellipse {
                center,
                axis: -axis,
                major,
                minor,
                u_ref,
            },
            geom::Curve3::Circle {
                center,
                axis,
                radius,
                u_ref,
            } => geom::Curve3::Circle {
                center,
                axis: -axis,
                radius,
                u_ref,
            },
            other => other,
        };
        let th1f = T::zero() - th1;
        (flipped, th1f, th1f + span)
    }
}

/// The direction the section leaves each of a join's two sites in,
/// toward the other: the datum the lane that paired the sites decided
/// the pairing on, handed to the chord so that it takes the arc the
/// pairing saw rather than deriving it again.
///
/// The boolean hands each half's germ direction ([`crate::boolean::HalfGerm::dir`]);
/// the split hands `±(n_plane × n_out)`, the way its conic walk enters
/// the face at a down crossing and leaves it at an up one
/// (`splitting::join`'s `split_leave`). Either is tangent to the section
/// at the site, of any positive length.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Leave<T: Real> {
    /// At the site of the join's first half (`h1`).
    pub(crate) h1: Vec3<T>,
    /// At the site of its second half (`h2`).
    pub(crate) h2: Vec3<T>,
}

/// Whether the chord `p1 → p2` takes the conic's ccw candidate: the one
/// whose tangent at `p1` agrees with `leave`, the section's direction of
/// departure there ([`Leave`]). Decided as `leave · Ĉ′(θ₁)` levered by
/// the semi-major axis (`chord_arc_leave`); a datum with no component
/// along the section is malformed and refuses.
///
/// # Errors
///
/// [`SplitJoinError::SectionInvariant`] for a datum normal to the
/// section; [`SplitJoinError::Escalated`] in the band.
fn arc_leaving<T: Decide>(
    face: FaceKey,
    band: Band,
    conic: &SectionConic<T>,
    p1: Point3<T>,
    leave: Vec3<T>,
) -> Result<bool, SplitJoinError> {
    let tangent = conic.tangent(conic.param(p1));
    match decide(
        "chord_arc_leave",
        Margin::levered(leave.dot(tangent) / tangent.norm(), conic.sa),
        band,
    )
    .map_err(|diag| SplitJoinError::Escalated { face, diag })?
    {
        Sign::Positive => Ok(true),
        Sign::Negative => Ok(false),
        Sign::Zero => Err(SplitJoinError::SectionInvariant {
            face,
            what: "the section's direction of departure handed to a chord end has no \
                   component along the section there",
        }),
    }
}

/// The chord spec for dividing `face` between vertices `u1 → u2`
/// (the mef/mekr `he_plus` direction): `None` for planar faces and
/// for ruling sections (the straight chord IS the honest carrier —
/// the M3 lane, bit-identical), `Some(spec)` with the C5 conic arc
/// for curved faces.
///
/// The conic lane (M5 PR 5):
///
/// 1. Classify (plane × wall surface) through THE table
///    ([`section_case`]) — trileans before any rung; a conic proceeds,
///    a ruling falls back to the straight chord, a tangent ruling is
///    described as such (C7), escalations pass through whole.
/// 2. Orient: of the conic's two arcs from `u1` to `u2`, take the one
///    leaving `u1` along `leave`, the section's direction of departure
///    there as the lane that paired the chord's ends decided it
///    ([`Leave`], [`arc_leaving`]). Which arc lies in the face is the
///    pairing's answer; the chord does not ask the face again.
/// 3. Describe as `Intersection { wall, aux plane, witness }` with the
///    witness minted at the carrier's mid-parameter (the witness
///    contract) — certification then pins endpoints, residuals, and
///    transversality through the ordinary gate.
#[allow(clippy::too_many_arguments)] // one internal lane, each argument a named duty
fn chord_spec<T: Decide>(
    body: &mut Body<T>,
    band: Band,
    lane: JoinLane<'_, T>,
    face: FaceKey,
    u1: VertexKey,
    u2: VertexKey,
    leave: Vec3<T>,
) -> Result<Option<EdgeCurveSpec<T>>, SplitJoinError> {
    // Self-loop chords keep the scaffolding-circle convention.
    if u1 == u2 {
        return Ok(None);
    }
    let face_data = body.get_face(face).ok_or_else(|| corrupt_face(face))?;
    let wall_key = face_data.surface;
    if let Some(geom::Surface::Plane { .. }) = body.get_surface(wall_key) {
        // The boolean's planar-side chord of a curved germ pair
        // takes its own lane (M5 PR 9); every other lane keeps
        // the straight chord BIT-IDENTICALLY.
        return match lane {
            JoinLane::BoolPlanar { wall, partner_key } => bool_planar_chord_spec(
                body,
                band,
                face,
                wall_key,
                &wall,
                partner_key,
                u1,
                u2,
                leave,
            ),
            JoinLane::Planar | JoinLane::Split(_) => Ok(None),
            JoinLane::AlongEdge => Err(SplitJoinError::SectionInvariant {
                face,
                what: "a chord along an edge of both solids asked for a section: it copies the \
                       edge",
            }),
        };
    }
    let JoinLane::Split(ctx) = lane else {
        return Err(SplitJoinError::SectionInvariant {
            face,
            what: "curved section chord outside the split/wall lane (the plane×plane lane divides \
                   only planar faces)",
        });
    };
    let Some(WallSection { case, .. }) =
        wall_section(body, band, ctx.origin, ctx.normal, face, u1)?
    else {
        return Err(SplitJoinError::SectionInvariant {
            face,
            what: "a face read planar by its key reads curved by its section",
        });
    };
    let conic = match case {
        // Ruling sections: the straight chord is the honest carrier.
        SectionCase::Straight => return Ok(None),
        // C7 (M5 PR 9): the tangent ruling is described
        // `TangentIntersection { wall, aux plane }` and pushed through
        // the ordinary certification gate by the mef/mekr caller. No
        // arc-side rule applies: a line has no complementary candidate.
        SectionCase::Tangent(line) => {
            let geom::Curve3::Line { origin, dir } = line else {
                return Err(SplitJoinError::SectionInvariant {
                    face,
                    what: "tangent classification carried a non-line",
                });
            };
            let p1 = vertex_point(body, u1)?;
            let p2 = vertex_point(body, u2)?;
            let len = dir.norm();
            let t1 = (p1 - origin).dot(dir) / len.powi(2);
            let t2 = (p2 - origin).dot(dir) / len.powi(2);
            // The spec must run u1 → u2 (the mef `he_plus` direction):
            // whether that is the classified direction or its reverse
            // is a named trilean (metered in metres), never a raw
            // comparison; a zero span is a degenerate chord site.
            let (carrier, s1, s2) = match decide(
                "split_tangent_chord_forward",
                Margin::metered(t2 - t1, InfSpeed::new(len)),
                band,
            )
            .map_err(|diag| SplitJoinError::Escalated { face, diag })?
            {
                Sign::Positive => (geom::Curve3::Line { origin, dir }, t1, t2),
                Sign::Negative => (
                    geom::Curve3::Line { origin, dir: -dir },
                    T::zero() - t1,
                    T::zero() - t2,
                ),
                Sign::Zero => {
                    return Err(SplitJoinError::SectionInvariant {
                        face,
                        what: "tangent section chord endpoints coincide along the ruling",
                    });
                }
            };
            let plane_key = match ctx.plane_key {
                Some(k) => k,
                None => {
                    let k = body.add_surface(geom::Surface::Plane {
                        origin: ctx.origin,
                        normal: ctx.normal.get(),
                        // Honest u_ref: the ruling direction lies in
                        // the plane by the tangency classification.
                        u_ref: dir / len,
                    });
                    ctx.plane_key = Some(k);
                    k
                }
            };
            let witness = carrier.mid_point(s1, s2);
            return Ok(Some(EdgeCurveSpec {
                description: geom_brep::EdgeDescriptionSpec::TangentIntersection {
                    s1: wall_key,
                    s2: plane_key,
                    witness,
                },
                carrier,
                param_start: s1,
                param_end: s2,
            }));
        }
        SectionCase::Conic(c) => c,
    };
    let p1 = vertex_point(body, u1)?;
    let p2 = vertex_point(body, u2)?;
    let ccw = arc_leaving(face, band, &conic, p1, leave)?;
    let (carrier, t_start, t_end) = oriented_arc(&conic, conic.param(p1), conic.param(p2), ccw);
    // The aux plane surface (honest u_ref: the section's major
    // direction, ⊥ normal by construction), minted once per split.
    let plane_key = match ctx.plane_key {
        Some(k) => k,
        None => {
            let k = body.add_surface(geom::Surface::Plane {
                origin: ctx.origin,
                normal: ctx.normal.get(),
                u_ref: conic.major,
            });
            ctx.plane_key = Some(k);
            k
        }
    };
    let witness = carrier.mid_point(t_start, t_end);
    Ok(Some(EdgeCurveSpec {
        description: geom_brep::EdgeDescriptionSpec::Intersection {
            s1: wall_key,
            s2: plane_key,
            witness,
        },
        carrier,
        param_start: t_start,
        param_end: t_end,
    }))
}

/// What the split plane cuts a curved face in: the face's wall surface
/// and THE C5 table's verdict for the pair. `None` for a planar face;
/// a kind the split's gate refuses is refused typed. `at` is a vertex
/// of the face, the base of the extent the table levers its verdicts
/// by.
///
/// One reading for both consumers of that conic — a chord minted in
/// the face ([`chord_spec`]) and the split's pairing of the face's
/// crossings along it (`splitting::join`) — so the two cannot read
/// different conics. It is lane-neutral: the pair is the face and a
/// plane, and nothing here knows which lane asks.
///
/// # Errors
///
/// [`SplitJoinError::SectionInvariant`] for a kind the gate refuses;
/// the table's refusals ([`section_case`]).
pub(crate) fn wall_section<T: Decide>(
    body: &Body<T>,
    band: Band,
    origin: Point3<T>,
    normal: UnitVec3<T>,
    face: FaceKey,
    at: VertexKey,
) -> Result<Option<WallSection<T>>, SplitJoinError> {
    let wall = body
        .get_face(face)
        .and_then(|f| body.get_surface(f.surface))
        .cloned()
        .ok_or_else(|| corrupt_face(face))?;
    match wall {
        geom::Surface::Plane { .. } => return Ok(None),
        geom::Surface::Cylinder { .. }
        | geom::Surface::Sphere { .. }
        | geom::Surface::Cone { .. } => {}
        _ => {
            return Err(SplitJoinError::SectionInvariant {
                face,
                what: "a section through a face kind the gate refuses",
            });
        }
    }
    // A transient classification value: only origin/normal are read by
    // the table (u_ref is a placement convention the classification
    // never consumes; a STORED aux plane gets an honest one).
    let plane_s = geom::Surface::Plane {
        origin,
        normal: normal.get(),
        u_ref: normal.get(),
    };
    let extent = face_extent(body, at, face).map_err(|_| corrupt_face(face))?;
    let case = section_case(face, band, &plane_s, &wall, extent)?;
    Ok(Some(WallSection { wall, case }))
}

/// [`wall_section`]'s answer.
pub(crate) struct WallSection<T: Real> {
    /// The face's wall surface.
    pub(crate) wall: geom::Surface<T>,
    /// What the table made of the plane against it.
    pub(crate) case: SectionCase<T>,
}

pub(crate) fn face_azimuth_window<T: Decide>(
    body: &Body<T>,
    surface: &geom::Surface<T>,
    face: FaceKey,
    band: Band,
) -> Result<Option<(T, T)>, SplitJoinError> {
    Ok(face_azimuth_images(body, surface, face, band)?.and_then(|images| azimuth_hull(&images)))
}

/// [`face_azimuth_window`] on `face`'s own surface: the window every
/// reader folds, read from outside the crate so a suite can hold the
/// interval lane's window against the `f64` replay's. `sweep-testing`
/// only, never production surface.
///
/// # Errors
///
/// As [`face_azimuth_window`], and [`SplitJoinError::Corrupt`] for a
/// face or surface key that does not resolve.
#[cfg(feature = "sweep-testing")]
pub fn face_azimuth_window_traces<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    band: Band,
) -> Result<Option<(T, T)>, SplitJoinError> {
    let key = body
        .get_face(face)
        .ok_or_else(|| corrupt_face(face))?
        .surface;
    let surface = body.get_surface(key).ok_or_else(|| corrupt_face(face))?;
    face_azimuth_window(body, surface, face, band)
}

/// The azimuth hull of a walk's images: the window every caller folds.
pub(crate) fn azimuth_hull<T: Real>(images: &[AzimuthImage<T>]) -> Option<(T, T)> {
    images
        .iter()
        .map(|image| image.range)
        .reduce(|(a, b), (lo, hi)| (a.min(lo), b.max(hi)))
}

/// The boolean PLANAR-side chord of a curved germ pair (M5 PR 9): the
/// divided face IS the germ plane, and the section conic comes from the
/// C5 table against the partner wall (by value, from the other
/// operand). The arc is the one the germ's direction names
/// ([`arc_leaving`]), the same datum the wall side's chord reads, so
/// both operands' chords of one polygon side are one geometric arc —
/// which is what keeps the zip's seams antiparallel-congruent.
#[allow(clippy::too_many_arguments)]
fn bool_planar_chord_spec<T: Decide>(
    body: &mut Body<T>,
    band: Band,
    face: FaceKey,
    plane_key: SurfaceKey,
    wall: &geom::Surface<T>,
    partner_key: &mut Option<SurfaceKey>,
    u1: VertexKey,
    u2: VertexKey,
    leave: Vec3<T>,
) -> Result<Option<EdgeCurveSpec<T>>, SplitJoinError> {
    if !matches!(
        wall,
        geom::Surface::Cylinder { .. } | geom::Surface::Sphere { .. }
    ) {
        return Err(SplitJoinError::SectionInvariant {
            face,
            what: "boolean planar-side germ partner is neither a cylinder nor a sphere (arm not \
                   wired)",
        });
    }
    let (p_o, p_n) = match body.get_surface(plane_key) {
        Some(&geom::Surface::Plane { origin, normal, .. }) => (origin, normal),
        // The two failures are different things and are typed apart: a
        // key that does not resolve is corruption, a key that resolves
        // to the wrong kind is this lane's invariant. A single `_` arm
        // would have called the first one an invariant.
        None => return Err(corrupt_face(face)),
        Some(_) => {
            return Err(SplitJoinError::SectionInvariant {
                face,
                what: "the section context's auxiliary plane key does not name a plane surface",
            });
        }
    };
    let plane_s = geom::Surface::Plane {
        origin: p_o,
        normal: p_n,
        u_ref: p_n,
    };
    let extent = face_extent(body, u1, face).map_err(|_| corrupt_vertex(u1))?;
    let conic = match section_case(face, band, &plane_s, wall, extent)? {
        // Ruling seams are straight chords on the plane too.
        SectionCase::Straight => return Ok(None),
        // A tangent germ pair inside the boolean zip means TOUCHING
        // operands — the M5 envelope refuses those upstream; reaching
        // here is a frontier configuration, refused typed. (The split
        // lane mints a chord on the same ruling; that difference is
        // why `section_case` hands the arm back instead of deciding.)
        SectionCase::Tangent(_) => {
            return Err(SplitJoinError::SectionInvariant {
                face,
                what: "tangent plane×cylinder germ pair in the boolean zip — a touching \
                       configuration, the typed frontier of the supported envelope",
            });
        }
        SectionCase::Conic(c) => c,
    };
    let p1 = vertex_point(body, u1)?;
    let p2 = vertex_point(body, u2)?;
    let ccw = arc_leaving(face, band, &conic, p1, leave)?;
    let (carrier, t_start, t_end) = oriented_arc(&conic, conic.param(p1), conic.param(p2), ccw);
    // The aux WALL surface in this body (honest full copy of the
    // mate's wall; minted once per germ wall face, caller-cached).
    let wall_aux = match *partner_key {
        Some(k) => k,
        None => {
            let k = body.add_surface(wall.clone());
            *partner_key = Some(k);
            k
        }
    };
    let witness = carrier.mid_point(t_start, t_end);
    Ok(Some(EdgeCurveSpec {
        description: geom_brep::EdgeDescriptionSpec::Intersection {
            s1: plane_key,
            s2: wall_aux,
            witness,
        },
        carrier,
        param_start: t_start,
        param_end: t_end,
    }))
}

/// How a [`ChordJoiner::join`] knows the section segment it chords is
/// an edge the face already has.
#[derive(Clone, Copy, Debug)]
pub(crate) enum SegmentEdge {
    /// The split lane: an edge between the two halves that lies in the
    /// section plane is the segment ([`between_edge_is_section`]).
    InPlane,
    /// The boolean lanes: the segment is this edge, named by the matched
    /// germs' locus on this solid, or lies inside a face (`None`).
    Is(Option<EdgeKey>),
}

/// The adjacency skip, for both chords of a `join`: an already-adjacent
/// pair whose between edge IS the section segment needs no chord. On
/// the split lane that is an edge lying in the plane
/// ([`between_edge_is_section`]); any other between edge needs its chord
/// minted, and an escalated verdict refuses typed rather than guessing
/// either way. On the boolean lanes it is structural: the between edge
/// is the edge the segment's locus names.
fn skip_adjacent_chord<T: Decide>(
    body: &Body<T>,
    lane: &JoinLane<'_, T>,
    segment: SegmentEdge,
    between: HalfEdgeKey,
    face: FaceKey,
    band: Band,
) -> Result<bool, SplitJoinError> {
    match segment {
        SegmentEdge::Is(edge) => {
            let between = body
                .get_half_edge(between)
                .ok_or_else(|| corrupt_he(between))?;
            Ok(edge == Some(between.edge))
        }
        SegmentEdge::InPlane => match between_edge_is_section(body, lane, between, band)? {
            Some(in_plane) => Ok(in_plane),
            None => Err(SplitJoinError::SectionInvariant {
                face,
                what: "section classification of the join-adjacent edge escalated",
            }),
        },
    }
}

/// Whether the (real) edge under `he` IS the section segment over its
/// whole span, the split lane's adjacency-skip question
/// ([`SegmentEdge::InPlane`]): `None` = escalated. Null scaffolding
/// answers `true`. A line answers `true` with NO predicate evaluation,
/// since a line between two ON copies lies in the plane. A conic asks
/// the trilean `split_conic_inplane_mid`, margin its mid-parameter plane
/// distance (metres): a conic not lying in the plane meets it in at most
/// two points and both endpoints are ON, so the interior is
/// sign-constant. Zero pins the whole arc in-plane; a definite midpoint
/// is a belly arc, whose chord MUST be minted or the section face
/// inherits an off-plane boundary edge. The boolean lanes read the
/// segment's locus instead and never ask this.
fn between_edge_is_section<T: Decide>(
    body: &Body<T>,
    lane: &JoinLane<'_, T>,
    he: HalfEdgeKey,
    band: Band,
) -> Result<Option<bool>, SplitJoinError> {
    let he_data = body.get_half_edge(he).ok_or_else(|| corrupt_he(he))?;
    let edge = body
        .get_edge(he_data.edge)
        .ok_or_else(|| corrupt_edge(he_data.edge))?;
    // Named only by the invariant arms below, which are off the hot
    // path.
    let owning_face = || {
        body.get_loop(he_data.parent_loop)
            .map(|l| l.face)
            .ok_or_else(|| corrupt_loop(he_data.parent_loop))
    };
    let Some(CurveGeom::Certified(curve)) = body.get_curve_geom(edge.curve) else {
        return Ok(Some(true)); // null scaffolding: zero-length, ON
    };
    match curve.carrier() {
        geom::Curve3::Line { .. } => match lane {
            JoinLane::Split(_) => Ok(Some(true)),
            _ => Err(boolean_lane_asked(owning_face()?)),
        },
        // The join lanes are fenced against the spiric and the spline
        // (both operand gates refuse the kinds), so a run edge carrying
        // one is an invariant break, never assumed ON.
        geom::Curve3::Spiric { .. } | geom::Curve3::Nurbs(_) => {
            Err(SplitJoinError::SectionInvariant {
                face: owning_face()?,
                what: "a join lane reached a spiric or spline run edge (the operand gates \
                       refuse the kinds)",
            })
        }
        geom::Curve3::Circle { .. } | geom::Curve3::Ellipse { .. } => {
            let mid = curve.mid_point();
            match lane {
                JoinLane::Split(SectionCtx { origin, normal, .. }) => {
                    let margin = Margin::of((mid - *origin).dot(normal.get()));
                    match decide("split_conic_inplane_mid", margin, band) {
                        Ok(Sign::Zero) => Ok(Some(true)),
                        Ok(Sign::Positive | Sign::Negative) => Ok(Some(false)),
                        Err(_) => Ok(None),
                    }
                }
                _ => Err(boolean_lane_asked(owning_face()?)),
            }
        }
    }
}

/// The in-plane skip test asked on a boolean lane, whose skip reads the
/// segment's locus ([`SegmentEdge::Is`]): no caller does.
fn boolean_lane_asked(face: FaceKey) -> SplitJoinError {
    SplitJoinError::SectionInvariant {
        face,
        what: "the in-plane skip test was asked on a boolean lane, whose skip reads the \
               segment's locus",
    }
}

/// Branch-stabilized chart azimuth `atan2(y, x)` (M5 S13): `atan2`'s
/// cut sits on the negative-`x` axis, and an interval `y` touching
/// zero there (a crossing vertex exactly on the chart seam's angle-π
/// copy) explodes the enclosure to a full period even though every
/// consumer reads the value mod τ. On a definitely-negative-`x` frame
/// the same azimuth is `atan2(−y, −x) + π` — the identical angle mod
/// τ with the cut on the benign axis. The frame trilean is a
/// computation choice between two identical formulas; its degenerate
/// and in-band arms keep the direct one (deterministic tie-break, D9).
fn stable_azimuth<T: Decide>(y: T, x: T, band: Band) -> T {
    match decide("split_chart_azimuth_frame", Margin::of(x), band) {
        Ok(Sign::Negative) => (T::zero() - y).atan2(T::zero() - x) + T::pi(),
        Ok(Sign::Positive | Sign::Zero) | Err(_) => y.atan2(x),
    }
}

/// The **exact** chart-azimuth extent of a pcurve over `[t₀, t₁]`.
///
/// On a cylinder chart [`geom_brep::chart_pcurve`] produces an azimuth
/// channel that is exactly `α + β·t` (it writes `pa.x = pb.x = 0` and
/// `pl.x = β ∈ {−1, 0, +1}` in both of its arms), so the two endpoint
/// evaluations ARE the range — this is closed-form structure, not a
/// sampled bound. A cone-section ellipse's azimuth is strictly
/// monotone, so its endpoints are its range too. It is read off
/// [`Pcurve::closed_form_span_box`], whose trigonometric widening is
/// exactly zero for every pcurve this lane derives and keeps the
/// statement true if the family ever widens.
///
/// **The closed-form lane only, and it says so with `None`.** The
/// window walk reads a chart image's azimuth through its closed form; a
/// fitted (rung-3) image has none, so it gets no answer rather than a
/// sentinel. A sentinel would be actively wrong here: the caller hulls
/// with `(a.min(lo), b.max(hi))`, which ABSORBS an inverted range
/// silently instead of propagating it, so "the empty range makes the
/// window refuse" would have been false. `None` propagates through the
/// caller as a typed `SectionInvariant` refusal.
///
/// The arm is unreachable today — `chart_pcurve` refuses a `Nurbs`
/// carrier before a fitted image can reach this function — and it is
/// written anyway because a face bounded by a fitted cyl×sphere edge is
/// exactly what would make it live.
fn chart_azimuth_range<T: Real>(p: &Pcurve<T>, t0: T, t1: T) -> Option<(T, T)> {
    p.closed_form_span_box(t0, t1).map(|b| (b.u_min, b.u_max))
}

/// One boundary half-edge's chart azimuth image, on the branch the
/// walk pinned: the azimuth where the walk ENTERS it and where it
/// EXITS it (the half-edge's own start and end), and the hull
/// [`face_azimuth_window`] folds.
#[derive(Clone, Debug)]
pub(crate) struct AzimuthImage<T: geom_core::Real> {
    /// The half-edge.
    pub(crate) he: HalfEdgeKey,
    /// The azimuth at its start vertex.
    pub(crate) entry: T,
    /// The azimuth at its end vertex.
    pub(crate) exit: T,
    /// Its azimuth extent (padded by the image's harmonic amplitude).
    pub(crate) range: (T, T),
    /// The chart's second coordinate at its start and end vertices.
    pub(crate) v: (T, T),
    /// The carrier parameter at its start and end vertices.
    pub(crate) t: (T, T),
    /// The half-edge's chart image when it is harmonic (a closed form,
    /// read by [`chart_v_du`] and evaluated directly); `None` for a
    /// fitted image.
    pub(crate) harmonic: Option<Pcurve<T>>,
}

/// Every charted half-edge of `face`'s outer loop, in loop order, with
/// its azimuth image on the branch the walk pins — the same walk
/// [`face_azimuth_window`] hulls, so each image's entry and exit sit on
/// the window's own branch. `None` for a loop that is not a cycle.
///
/// # Errors
///
/// As [`face_azimuth_window`].
pub(crate) fn face_azimuth_images<T: Decide>(
    body: &Body<T>,
    surface: &geom::Surface<T>,
    face: FaceKey,
    band: Band,
) -> Result<Option<Vec<AzimuthImage<T>>>, SplitJoinError> {
    let Some(halves) = outer_cycle(body, face)? else {
        return Ok(None);
    };
    run_azimuth_images(body, surface, face, &halves, band).map(Some)
}

/// The half-edges of `face`'s outer loop in cycle order; `None` for a
/// loop that is not a cycle.
fn outer_cycle<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
) -> Result<Option<Vec<HalfEdgeKey>>, SplitJoinError> {
    let outer = body.get_face(face).ok_or_else(|| corrupt_face(face))?.outer;
    let crate::entity::LoopBoundary::Cycle { first } = body
        .get_loop(outer)
        .ok_or_else(|| corrupt_loop(outer))?
        .boundary
    else {
        return Ok(None);
    };
    body.loop_cycle(first)
        .ok_or_else(|| corrupt_he(first))
        .map(Some)
}

/// **Is a walk's chart polygon its own bounding box?** Every image is a
/// chart segment on the cone's two iso families — a rim holds `v`, a
/// generator holds `u` — so the lifted boundary, closed by the segment
/// from its last exit to its first entry (the apex jump, when the walk
/// was closed there), is a rectilinear chart polygon. Such a polygon is
/// its bounding box exactly when its enclosed area is the box's; an L or
/// a notch has strictly less, and every one of them has the same hull,
/// so a window read off the hull would cover the notch. `None` for an
/// empty walk.
pub(crate) fn chart_box_defect<T: Real>(images: &[AzimuthImage<T>]) -> Option<ChartBox<T>> {
    let (first, last) = (images.first()?, images.last()?);
    let u = azimuth_hull(images)?;
    let mut v = (first.v.0, first.v.0);
    let mut twice = T::zero();
    let closing = [(last.exit, last.v.1, first.entry, first.v.0)];
    for (u0, v0, u1, v1) in images
        .iter()
        .map(|i| (i.entry, i.v.0, i.exit, i.v.1))
        .chain(closing)
    {
        v = (v.0.min(v0).min(v1), v.1.max(v0).max(v1));
        twice = twice + (u0 * v1 - u1 * v0);
    }
    let area = twice.abs() / T::from_f64(2.0);
    Some(ChartBox {
        u,
        v,
        defect: (u.1 - u.0) * (v.1 - v.0) - area,
    })
}

/// A walk's chart bounding box, and how far its polygon falls short of
/// it ([`chart_box_defect`]).
#[derive(Clone, Copy, Debug)]
pub(crate) struct ChartBox<T: Real> {
    /// The azimuth window.
    pub(crate) u: (T, T),
    /// The second coordinate's window.
    pub(crate) v: (T, T),
    /// The box's area less the polygon's: zero exactly when the polygon
    /// is the box.
    pub(crate) defect: T,
}

/// What the apex closure makes of a cone face's outer cycle.
#[derive(Clone, Debug)]
pub(crate) enum ApexClosure<T: Real> {
    /// The cycle never reaches the apex; every junction is a chart
    /// point, where the nearest-branch walk is exact.
    Clear,
    /// One apex visit and no ring: the lifted loop, closed at the apex.
    Closed {
        /// The lift's images, starting at the half-edge that leaves the
        /// apex.
        images: Vec<AzimuthImage<T>>,
        /// The azimuth hull of the lift.
        window: (T, T),
        /// The farthest boundary vertex from the apex, in metres: the
        /// lever for an angle read off the window.
        reach: T,
    },
    /// Two or more apex visits, or one on a ringed face: no single lift
    /// describes the face.
    Open,
}

/// **The apex closure of a cone face's outer cycle.**
///
/// The apex is not a chart point: every azimuth maps to it, so the
/// nearest-branch pin carries no information across it, and an
/// apex-closed sector wider than π reads there as a full period. The
/// face's chart region lies in the lifted half-strip `v ∈ (0, V]` (or
/// its mirror), and its closure meets `v = 0` — the apex blown up — in
/// the segment between the incoming and outgoing azimuths. The lifted
/// boundary is a closed curve, so its net azimuth change is zero, and
/// the jump at a single apex visit is `J = −Σ Δθ(non-apex edges)`
/// exactly. The walk realises that by starting at the half-edge that
/// leaves the apex: every junction it crosses is then a chart point,
/// pinned by nearest-branch continuity as everywhere else, and the one
/// junction it does not cross is the apex, whose jump the closure
/// supplies.
///
/// The preconditions are the rule's own: exactly one apex visit (a
/// cycle through the apex twice bounds two sectors, whose gap no hull
/// of a single lift can exclude), no ring, and every other boundary
/// vertex on one nappe.
///
/// # Errors
///
/// As [`face_azimuth_window`], and
/// [`SplitJoinError::SectionInvariant`] when `surface` is not a cone.
pub(crate) fn cone_apex_closure<T: Decide>(
    body: &Body<T>,
    surface: &geom::Surface<T>,
    face: FaceKey,
    band: Band,
) -> Result<ApexClosure<T>, SplitJoinError> {
    let geom::Surface::Cone { apex, axis, .. } = *surface else {
        return Err(SplitJoinError::SectionInvariant {
            face,
            what: "the apex closure was asked of a face that is not on a cone",
        });
    };
    let esc = |diag| SplitJoinError::Escalated { face, diag };
    let fd = body.get_face(face).ok_or_else(|| corrupt_face(face))?;
    let Some(halves) = outer_cycle(body, face)? else {
        return Ok(ApexClosure::Open);
    };
    let mut leaving = Vec::new();
    let mut reach = T::zero();
    let mut nappes = [false; 2];
    for (i, &he) in halves.iter().enumerate() {
        let v = body.get_half_edge(he).ok_or_else(|| corrupt_he(he))?.start;
        let q = vertex_point(body, v)? - apex;
        let d = q.norm();
        reach = reach.max(d);
        if decide("bool_cone_apex_visit", Margin::of(d), band).map_err(esc)? == Sign::Zero {
            leaving.push(i);
            continue;
        }
        match decide("bool_cone_apex_nappe", Margin::of(q.dot(axis)), band).map_err(esc)? {
            Sign::Positive => nappes[0] = true,
            Sign::Negative => nappes[1] = true,
            Sign::Zero => {}
        }
    }
    let [start] = leaving[..] else {
        return Ok(if leaving.is_empty() {
            ApexClosure::Clear
        } else {
            ApexClosure::Open
        });
    };
    // A boundary on both nappes meets the apex from two sheets: no one
    // lift describes it.
    if !fd.rings.is_empty() || nappes == [true, true] {
        return Ok(ApexClosure::Open);
    }
    let lifted: Vec<HalfEdgeKey> = halves[start..]
        .iter()
        .chain(&halves[..start])
        .copied()
        .collect();
    let images = run_azimuth_images(body, surface, face, &lifted, band)?;
    Ok(match azimuth_hull(&images) {
        Some(window) => ApexClosure::Closed {
            images,
            window,
            reach,
        },
        None => ApexClosure::Open,
    })
}

/// The walk behind [`face_azimuth_images`] and [`cone_apex_closure`]:
/// each charted half-edge's image, its branch pinned to the previous
/// edge's exit.
fn run_azimuth_images<T: Decide>(
    body: &Body<T>,
    surface: &geom::Surface<T>,
    face: FaceKey,
    halves: &[HalfEdgeKey],
    band: Band,
) -> Result<Vec<AzimuthImage<T>>, SplitJoinError> {
    let tau = T::tau();
    let mut images = Vec::new();
    let mut prev_exit: Option<T> = None;
    for &he in halves {
        let he_data = body.get_half_edge(he).ok_or_else(|| corrupt_he(he))?;
        let edge = body
            .get_edge(he_data.edge)
            .ok_or_else(|| corrupt_edge(he_data.edge))?;
        let Some(CurveGeom::Certified(curve)) = body.get_curve_geom(edge.curve) else {
            continue; // null scaffolding: zero-length, no azimuth extent
        };
        let (t0, t1) = curve.params();
        let base = chart_pcurve(curve.carrier(), surface, band).map_err(|e| match e {
            geom_brep::PcurveCertifyError::Escalated { cause, .. } => {
                SplitJoinError::Escalated { face, diag: cause }
            }
            _ => SplitJoinError::SectionInvariant {
                face,
                what: "a run edge has no closed-form chart image on the divided face's chart",
            },
        })?;
        let plus = edge.he_plus == he;
        let (entry_t, exit_t) = if plus { (t0, t1) } else { (t1, t0) };
        let pcurve = match prev_exit {
            None => base,
            Some(prev) => {
                let raw = base.eval(entry_t).x;
                // The branch pin. Default: nearest-branch continuity —
                // the PR 6 loop-walk rule, exact wherever the previous
                // edge's exit and this edge's entry share a chart
                // point.
                //
                // **Sphere-chart POLE junctions (M5 S13)** carry no
                // azimuth continuity at all — the chart is singular at
                // a pole, and a face bounded by two meridians exactly
                // half a period apart (the two-band ball's shape) makes
                // the nearest-branch pin a knife-edge tie that hands
                // BOTH bands the same window. The loop's own
                // orientation carries the missing bit exactly: walking
                // an outward (`sense: true`) face's cycle with the face
                // on the left, azimuth ADVANCES through the south pole
                // (the next boundary azimuth is the unique branch in
                // `(prev, prev + τ)`) and RETURNS through the north
                // pole (`(prev − τ, prev)`); a reversed face swaps the
                // poles' roles. Exact structure — nothing sampled; the
                // pole test and its side are named trileans and an
                // in-band junction escalates (F6).
                let mut k = (prev - raw).periodic_branch(tau);
                // A cone APEX junction carries no azimuth at all, and
                // unlike a sphere pole no orientation bit recovers it:
                // the jump there is the whole lift's
                // ([`cone_apex_closure`]), which never pins across it.
                // A walk that would is refused rather than pinned.
                if let geom::Surface::Cone { apex, .. } = surface {
                    let entry_v = he_data.start;
                    let p = vertex_point(body, entry_v).map_err(|_| corrupt_vertex(entry_v))?;
                    if decide("bool_cone_apex_visit", Margin::of((p - *apex).norm()), band)
                        .map_err(|diag| SplitJoinError::Escalated { face, diag })?
                        == Sign::Zero
                    {
                        return Err(SplitJoinError::ApexUnlifted { face });
                    }
                }
                if let geom::Surface::Sphere {
                    center,
                    radius,
                    axis,
                    ..
                } = surface
                {
                    let entry_v = he_data.start;
                    let p = vertex_point(body, entry_v).map_err(|_| corrupt_vertex(entry_v))?;
                    let d = (p - *center).dot(*axis);
                    match decide(
                        "split_sphere_window_pole",
                        Margin::of(*radius - d.abs()),
                        band,
                    )
                    .map_err(|diag| SplitJoinError::Escalated { face, diag })?
                    {
                        Sign::Positive => {}
                        Sign::Negative => {
                            return Err(SplitJoinError::SectionInvariant {
                                face,
                                what: "a run vertex lies off its sphere face's carrier (its \
                                       axial offset exceeds the radius)",
                            });
                        }
                        Sign::Zero => {
                            let south =
                                match decide("split_sphere_window_pole_side", Margin::of(d), band)
                                    .map_err(|diag| SplitJoinError::Escalated { face, diag })?
                                {
                                    Sign::Negative => true,
                                    Sign::Positive => false,
                                    Sign::Zero => {
                                        return Err(SplitJoinError::SectionInvariant {
                                            face,
                                            what: "a run vertex reads as both a pole and an \
                                                   equator point — a zero-radius sphere face",
                                        });
                                    }
                                };
                            let sense =
                                body.get_face(face).ok_or_else(|| corrupt_face(face))?.sense;
                            let advancing = south == sense;
                            // STRICTLY next / strictly previous, so
                            // `floor`/`ceil` and not the nearest-branch
                            // index above: the branch wanted is the
                            // unique one in an OPEN interval, and the
                            // nearest one is exactly what carries no
                            // information here.
                            //
                            // The selection jumps at an integer `q`: the
                            // entry meridian leaves the pole along the
                            // half-meridian the previous one arrived on —
                            // a slit, two meridian edges overlapping, or a
                            // wedge inside rounding. An enclosure
                            // straddling the jump spans both branches,
                            // `prev` and `prev ± τ`, so the window's width
                            // encloses τ or more. What makes that sound
                            // is the property a window reader relies on:
                            // it reads the window as a region only once
                            // its period gate has decided the width
                            // definitely under τ, so on this window it
                            // declines or escalates and never reads it.
                            let q = (prev - raw) / tau;
                            k = if advancing {
                                q.floor() + T::one()
                            } else {
                                T::zero() - (T::zero() - q).floor() - T::one()
                            };
                        }
                    }
                }
                base.shift_branch(k, tau)
            }
        };
        let (lo, hi) =
            chart_azimuth_range(&pcurve, t0, t1).ok_or(SplitJoinError::SectionInvariant {
                face,
                what: "a boundary edge's chart image is FITTED — the window walk reads a \
                       closed-form azimuth",
            })?;
        let exit = pcurve.eval(exit_t).x;
        images.push(AzimuthImage {
            he,
            entry: pcurve.eval(entry_t).x,
            exit,
            range: (lo, hi),
            v: (pcurve.eval(entry_t).y, pcurve.eval(exit_t).y),
            t: (entry_t, exit_t),
            harmonic: matches!(pcurve, Pcurve::Harmonic { .. }).then(|| pcurve.clone()),
        });
        prev_exit = Some(exit);
    }
    Ok(images)
}

/// `∫ (v − anchor) du` of a harmonic chart image from `t0` to `t1`, and
/// an upper bound on `∫ |dv|` there; `None` for an image that is not
/// harmonic. The anchor is subtracted from the constant term before
/// integrating, so a short island far along the axis does not cancel
/// `v·Δu` against `v·Δu` (the precision note at `geom_brep::props`'
/// cylinder Green form, which this shares).
///
/// Read for a cylinder chart, whose every harmonic image
/// [`chart_pcurve`] writes with a LINEAR azimuth channel (`pa.x = pb.x
/// = 0`, `u = p0.x + pl.x·t`): `v` is then
/// `p0.y + pa.y·cos t + pb.y·sin t + pl.y·t` and integrates term by
/// term. The precondition is decided, not assumed
/// (`split_chart_azimuth_linear`, the harmonic azimuth amplitude
/// levered by the radius): an image that breaks it is the chart
/// writer's invariant broken, and refuses.
///
/// # Errors
///
/// [`SplitJoinError::SectionInvariant`] for a harmonic azimuth term;
/// [`SplitJoinError::Escalated`] when that decision escalates.
fn chart_v_du<T: Decide>(
    face: FaceKey,
    p: &Pcurve<T>,
    (t0, t1): (T, T),
    anchor: T,
    radius: T,
    band: Band,
) -> Result<Option<(T, T)>, SplitJoinError> {
    let Pcurve::Harmonic { p0, pa, pb, pl } = *p else {
        return Ok(None);
    };
    match decide(
        "split_chart_azimuth_linear",
        Margin::levered(pa.x.abs() + pb.x.abs(), radius),
        band,
    )
    .map_err(|diag| SplitJoinError::Escalated { face, diag })?
    {
        Sign::Zero => {}
        Sign::Positive | Sign::Negative => {
            return Err(SplitJoinError::SectionInvariant {
                face,
                what: "a cylinder chart image carries a harmonic azimuth term (the chart \
                       writes the azimuth linear in the carrier parameter)",
            });
        }
    }
    let v_var = (pa.y.abs() + pb.y.abs() + pl.y.abs()) * (t1 - t0).abs();
    let half = T::from_f64(0.5);
    let dt = t1 - t0;
    let integral = (p0.y - anchor) * dt + pa.y * (t1.sin() - t0.sin())
        - pb.y * (t1.cos() - t0.cos())
        + pl.y * (t1 + t0) * dt * half;
    Ok(Some((pl.x * integral, v_var)))
}

/// A cylinder face's chart frame: the one destructure the wall-chart
/// readers below share.
#[derive(Clone, Copy)]
struct WallChart<T: geom_core::Real> {
    origin: Point3<T>,
    axis: Vec3<T>,
    radius: T,
    u_ref: Vec3<T>,
}

/// The cylinder frame [`WallChart`] of `surface`; `None` for any other
/// kind.
fn wall_chart<T: Real>(surface: &geom::Surface<T>) -> Option<WallChart<T>> {
    match *surface {
        geom::Surface::Cylinder {
            origin,
            axis,
            radius,
            u_ref,
        } => Some(WallChart {
            origin,
            axis,
            radius,
            u_ref,
        }),
        _ => None,
    }
}

/// The refusal of a ring-lane reading on a curved face that is not a
/// cylinder wall ([`SplitJoinError::RingOffCylinderChart`]).
fn no_wall_chart<T: Real>(face: FaceKey, surface: &geom::Surface<T>) -> SplitJoinError {
    SplitJoinError::RingOffCylinderChart {
        face,
        kind: surface.kind(),
    }
}

/// **The winding of a ring-lane island on a cylinder wall's chart**,
/// as a decided sign about the face's OUTWARD normal: the open run
/// `h1 → h2` (`next` order, through `h2`) closed by the chord from
/// `h2`'s end back to `h1`'s start, which lies in the section plane
/// `closure`.
///
/// The planar ring lane's statement ([`crate::loop_winding`]) asked on
/// the face's own chart, where the region's signed area is the chart
/// Green form `A = −∮ v du` (its home, and the chart's orientation
/// premise, are `geom_brep::props`' cylinder arm and
/// [`geom::Surface::Cylinder`]; this reads the same form off the run's
/// chart images rather than off rim carriers):
///
/// - each run edge contributes its exact `∫ v du` ([`chart_v_du`]); the
///   walk pins every image on one branch, so a junction is a chart
///   point and the straight term between images is the walk's own
///   junction gap;
/// - the closing chord is the section of the face by `closure`:
///   `v(u) = (n·(o − c) − R·n·r̂(u)) / (n·â)`, integrated in closed
///   form, or a ruling (`n·â = 0`, no `du`). **The exact closure is
///   load-bearing**: a straight chart segment differs by the lens
///   between the sinusoid and its chord, and on a thin bar turned about
///   two axes that lens outweighs the island and reverses its sign
///   (38 of 658 islands in the delta review's scan, −13.4× to 35×;
///   pinned by `verbs_germarms::a_thin_bar_turned_about_two_axes_gets_through_the_join`).
///
/// Every `v` is read against an anchor on the island (its first entry):
/// the island is closed, so `Σ du = 0` and the anchor drops out in
/// exact arithmetic, and the sum keeps the conditioning of the island's
/// own height rather than its distance along the axis.
///
/// The chart sign is the winding about `+r̂`, and the face's sense bit
/// turns it into the winding about the outward normal.
///
/// The margin is the planar arm's mean width `2A/P` (F4): `A` in m²
/// (`R·A_chart`) and `P` an upper bound on the boundary length in
/// metres (`R·|Δu|` plus the axial variation, per piece).
///
/// A sphere face, or any other curved kind, refuses typed
/// ([`no_wall_chart`]).
pub(crate) fn chart_island_winding<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    (h1, h2): (HalfEdgeKey, HalfEdgeKey),
    closure: (Point3<T>, UnitVec3<T>),
    band: Band,
) -> Result<Result<Sign, Indeterminate>, SplitJoinError> {
    let face_data = body.get_face(face).ok_or_else(|| corrupt_face(face))?;
    let sense = face_data.sense;
    let surface = body
        .get_surface(face_data.surface)
        .cloned()
        .ok_or_else(|| corrupt_face(face))?;
    let WallChart {
        origin: centre,
        axis,
        radius,
        u_ref,
    } = wall_chart(&surface).ok_or_else(|| no_wall_chart(face, &surface))?;
    let cycle = body.loop_cycle(h1).ok_or_else(|| corrupt_he(h1))?;
    let end = cycle
        .iter()
        .position(|&he| he == h2)
        .ok_or_else(|| corrupt_he(h2))?;
    let images = run_azimuth_images(body, &surface, face, &cycle[..=end], band)?;
    let (Some(first), Some(last)) = (images.first(), images.last()) else {
        return Err(SplitJoinError::SectionInvariant {
            face,
            what: "a ring-lane island carries no charted edge",
        });
    };
    let half = T::from_f64(0.5);
    let anchor = first.v.0;
    let mut area = T::zero();
    let mut length = T::zero();
    for (i, image) in images.iter().enumerate() {
        let harmonic = match image.harmonic.as_ref() {
            Some(p) => chart_v_du(face, p, image.t, anchor, radius, band)?,
            None => None,
        };
        let (v_du, v_var) = harmonic.ok_or(SplitJoinError::SectionInvariant {
            face,
            what: "a ring-lane island edge's chart image is fitted — the chart winding \
                       reads a linear azimuth channel",
        })?;
        area = area - v_du;
        length = length + radius * (image.exit - image.entry).abs() + v_var;
        if let Some(next) = images.get(i + 1) {
            let du = next.entry - image.exit;
            area = area - ((image.v.1 - anchor) + (next.v.0 - anchor)) * half * du;
            length = length + radius * du.abs() + (next.v.0 - image.v.1).abs();
        }
    }
    let (from, to) = ((last.exit, last.v.1), (first.entry, first.v.0));
    let (origin, normal) = closure;
    let n = normal.get();
    let n_a = n.dot(axis);
    match decide(
        "split_ring_closure_ruling",
        Margin::levered(n_a, radius),
        band,
    )
    .map_err(|diag| SplitJoinError::Escalated { face, diag })?
    {
        // A ruling: no azimuth travel, so no `v du`.
        Sign::Zero => length = length + (to.1 - from.1).abs(),
        Sign::Positive | Sign::Negative => {
            let n_u = n.dot(u_ref);
            let n_v = n.dot(axis.cross(u_ref));
            let k = n.dot(origin - centre) / n_a - anchor;
            let lever = radius / n_a;
            let du = to.0 - from.0;
            // `∫ (v − anchor) du` along the section, as differences.
            let g = k * du
                - lever * (n_u * (to.0.sin() - from.0.sin()) - n_v * (to.0.cos() - from.0.cos()));
            area = area - g;
            length = length
                + radius * du.abs()
                + lever.abs() * (n_u.powi(2) + n_v.powi(2)).sqrt() * du.abs();
        }
    }
    let wound = crate::validate::decide_reported(
        crate::loop_winding::WINDING_PREDICATE,
        Margin::over_lever(area * radius * T::from_f64(2.0), length),
        band,
    );
    Ok(wound.map(|d| if sense { d.sign } else { d.sign.flip() }))
}

/// **The chord of a section segment that is an edge of BOTH solids**
/// ([`JoinLane::AlongEdge`], `segment` naming this solid's edge): the
/// other copy of that edge, so its curve is the edge's own, from `u1`
/// to `u2` (the edge's endpoints' copies, at its endpoints' points) —
/// never a section the lane would compute from the germ's face pair,
/// which may be two faces on one carrier with no section between them.
/// A line is the straight chord; a circle is its own arc, on the
/// carrier reversed when the chord runs against it. `None` on every
/// other lane: there a segment along an edge of ONE solid lies in a
/// face of the other, whose lane computes the section the chord takes
/// (the rod's ruling, a lens rim on a wall).
fn along_edge_spec<T: Decide>(
    body: &Body<T>,
    lane: &JoinLane<'_, T>,
    segment: SegmentEdge,
    face: FaceKey,
    u1: VertexKey,
    u2: VertexKey,
) -> Result<Option<EdgeCurveSpec<T>>, SplitJoinError> {
    let (JoinLane::AlongEdge, SegmentEdge::Is(Some(edge))) = (lane, segment) else {
        return Ok(None);
    };
    let point = |v: VertexKey| {
        body.get_vertex(v)
            .and_then(|d| body.get_point(d.point))
            .copied()
            .ok_or(SplitJoinError::SectionInvariant {
                face,
                what: "a chord end along the segment's edge has no point",
            })
    };
    let (p1, p2) = (point(u1)?, point(u2)?);
    let curve = match body
        .get_edge(edge)
        .and_then(|e| body.get_curve_geom(e.curve))
    {
        Some(CurveGeom::Certified(c)) => c,
        _ => {
            return Err(SplitJoinError::SectionInvariant {
                face,
                what: "the segment's edge carries no certified curve",
            });
        }
    };
    let (t0, t1) = curve.params();
    match *curve.carrier() {
        geom::Curve3::Line { .. } => Ok(Some(EdgeCurveSpec::line_between(p1, p2))),
        geom::Curve3::Circle {
            center,
            axis,
            radius,
            u_ref,
        } => {
            // The chord starts at the copy of whichever end of the edge
            // null edges tie `u1` to; the curve runs from its `he_plus`
            // start.
            let e = body.get_edge(edge).ok_or_else(|| corrupt_edge(edge))?;
            let e_start = body
                .get_half_edge(e.he_plus)
                .ok_or_else(|| corrupt_he(e.he_plus))?
                .start;
            let e_end = body
                .half_edge_end(e.he_plus)
                .ok_or_else(|| corrupt_he(e.he_plus))?;
            let tied = null_site(body, &[u1]);
            let forward = match (tied.contains(&e_start), tied.contains(&e_end)) {
                (true, false) => true,
                (false, true) => false,
                _ => {
                    return Err(SplitJoinError::SectionInvariant {
                        face,
                        what: "a chord end along the segment's edge is tied to neither or both \
                               of the edge's ends",
                    });
                }
            };
            let carrier = curve.carrier().clone();
            let spec = if forward {
                EdgeCurveSpec::arc_of_circle(carrier, t0, t1)
            } else {
                // θ ↦ −θ about the flipped axis runs the same arc back.
                let back = geom::Curve3::Circle {
                    center,
                    axis: -axis,
                    radius,
                    u_ref,
                };
                EdgeCurveSpec::arc_of_circle(back, -t1, -t0)
            };
            Ok(spec)
        }
        _ => Err(SplitJoinError::SectionInvariant {
            face,
            what: "a section segment along an edge whose carrier has no chord lane (only lines \
                   and circles are copied)",
        }),
    }
}

/// **The site of a vertex**: every vertex null edges tie it to, itself
/// included — the copies one crossing site was split into, however
/// many null edges it holds. A null edge's two ends are the same point,
/// so the site is one point held by several vertices.
pub(crate) fn null_site<T: Decide>(body: &Body<T>, from: &[VertexKey]) -> Vec<VertexKey> {
    let mut site: Vec<VertexKey> = from.to_vec();
    let mut i = 0;
    while i < site.len() {
        let v = site[i];
        for k in body.edges_of_vertex(v).unwrap_or_default() {
            let Some(attr) = body
                .get_edge(k)
                .and_then(|e| body.get_curve_geom(e.curve))
                .and_then(CurveGeom::null_scaffold)
            else {
                continue;
            };
            for w in [attr.below_end, attr.above_end] {
                if !site.contains(&w) {
                    site.push(w);
                }
            }
        }
        i += 1;
    }
    site
}

impl ChordJoiner {
    /// `join` (module docs): connect the old loose end `h1` and the
    /// new half `h2` with up to two chord edges, each a curved face's
    /// conic arc leaving its first site along `leave`; the minted chord
    /// edges come back (the boolean joining records their germ — M3
    /// PR 5).
    pub(crate) fn join<T: Decide + crate::props::AtRestPolicy>(
        &mut self,
        body: &mut Body<T>,
        h1: HalfEdgeKey,
        h2: HalfEdgeKey,
        mut lane: JoinLane<'_, T>,
        segment: SegmentEdge,
        leave: Leave<T>,
        tol: Tol,
    ) -> Result<Vec<EdgeKey>, SplitJoinError> {
        let l1 = body
            .get_half_edge(h1)
            .ok_or_else(|| corrupt_he(h1))?
            .parent_loop;
        let l2 = body
            .get_half_edge(h2)
            .ok_or_else(|| corrupt_he(h2))?
            .parent_loop;
        let oldf = body.get_loop(l1).ok_or_else(|| corrupt_loop(l1))?.face;
        let next = |body: &Body<T>, he: HalfEdgeKey| -> Result<HalfEdgeKey, SplitJoinError> {
            Ok(body.get_half_edge(he).ok_or_else(|| corrupt_he(he))?.next)
        };
        let prev = |body: &Body<T>, he: HalfEdgeKey| -> Result<HalfEdgeKey, SplitJoinError> {
            Ok(body.get_half_edge(he).ok_or_else(|| corrupt_he(he))?.prev)
        };
        let start_of = |body: &Body<T>, he: HalfEdgeKey| -> Result<VertexKey, SplitJoinError> {
            Ok(body.get_half_edge(he).ok_or_else(|| corrupt_he(he))?.start)
        };

        let mut chords = Vec::new();
        let mut newf = None;
        // Adjacency of the two null halves on the prev side of h1
        // (h2 → between → h1), which the first chord's guard reads.
        let prev_adjacent = l1 == l2 && prev(body, prev(body, h1)?)? == h2;
        if l1 == l2 {
            // Adjacency skip (M3): when exactly one edge sits between
            // h2 and h1 AND it lies in the plane, that edge IS the
            // section segment — no chord needed. A belly conic between
            // them (M1 fix) is NOT a section segment: the chord must
            // be minted or the section face inherits an off-plane
            // boundary; an escalated in-plane verdict refuses typed.
            let skip_first = if prev_adjacent {
                skip_adjacent_chord(body, &lane, segment, prev(body, h1)?, oldf, self.band)?
            } else {
                false
            };
            if !skip_first {
                // `outside` is the first half past the run; after the
                // mef its parent loop is the split's REMAINDER — the
                // loop ring re-homing must skip (a ring-lane remainder
                // is geometrically coincident with the run and would
                // land OnBoundary; issue #93).
                let outside = next(body, h2)?;
                // Curved faces get their C5 section carrier (M5 PR 5);
                // planar faces keep the straight mef_chord lane
                // BIT-IDENTICALLY (chord_spec returns None for planes).
                let site = MefSite::Chords {
                    he1: h1,
                    he2: outside,
                };
                let (u1, u2) = (start_of(body, h1)?, start_of(body, outside)?);
                let spec = match along_edge_spec(body, &lane, segment, oldf, u1, u2)? {
                    Some(spec) => Some(spec),
                    None => chord_spec(body, self.band, lane.reborrow(), oldf, u1, u2, leave.h1)?,
                };
                // Both arms hand `mef` the parent's surface, so the
                // fragment takes `oldf`'s bit (`Body::resolve_face_surface`).
                // Guard: sweep's `m5_s12_curved_ops.rs`, the row named
                // `a_boolean_that_splits_a_reversed_wall_inherits_the_parent_bit`.
                let created = match spec {
                    None => body.mef_chord(site, tol)?,
                    Some(spec) => body.mef(site, spec, FaceSurface::Inherit, tol)?,
                };
                self.slivers.insert(created.face, ());
                self.fragments.push((created.face, oldf));
                chords.push(created.edge);
                newf = Some((created.face, outside));
            }
        } else {
            // Structural ring choice (module docs): kill the loop that
            // is not the face's outer; if both are rings, keep the
            // book's order (kill h2's loop).
            let outer = body.get_face(oldf).ok_or_else(|| corrupt_face(oldf))?.outer;
            let (target, ring, leave_target) = if l2 == outer {
                (next(body, h2)?, h1, leave.h2)
            } else {
                (h1, next(body, h2)?, leave.h1)
            };
            let site = MekrSite::Cycles { target, ring };
            let (u1, u2) = (start_of(body, target)?, start_of(body, ring)?);
            let spec = match along_edge_spec(body, &lane, segment, oldf, u1, u2)? {
                Some(spec) => Some(spec),
                None => chord_spec(body, self.band, lane.reborrow(), oldf, u1, u2, leave_target)?,
            };
            let made = match spec {
                None => body.mekr_chord(site, tol)?,
                Some(spec) => body.mekr(site, spec, tol)?,
            };
            chords.push(made.edge);
        }
        // Second-chord guard: when the two halves are already adjacent
        // AND the between edge is in-plane, the chord already exists
        // (M3); a belly conic between them still needs its chord (M1
        // fix — same rule as the first guard).
        let adjacent2 = next(body, next(body, h1)?)? == h2;
        let skip_second = if adjacent2 {
            skip_adjacent_chord(body, &lane, segment, next(body, h1)?, oldf, self.band)?
        } else {
            false
        };
        if !skip_second {
            // The second chord divides the face `h2` sits on NOW —
            // after a first mef that is not necessarily `oldf` (`h2`
            // may have landed in the new face). Capture the owner at
            // call time, BEFORE the surgery moves loops.
            let l2_now = body
                .get_half_edge(h2)
                .ok_or_else(|| corrupt_he(h2))?
                .parent_loop;
            let owner = body
                .get_loop(l2_now)
                .ok_or_else(|| corrupt_loop(l2_now))?
                .face;
            let site = MefSite::Chords {
                he1: h2,
                he2: next(body, h1)?,
            };
            // The second chord runs from h2's site back to h1's: the
            // same segment as the first (the two null edges are
            // zero-length), taken the other way.
            let (u1, u2) = (start_of(body, h2)?, start_of(body, next(body, h1)?)?);
            let spec = match along_edge_spec(body, &lane, segment, owner, u1, u2)? {
                Some(spec) => Some(spec),
                None => chord_spec(body, self.band, lane, owner, u1, u2, leave.h2)?,
            };
            let created = match spec {
                None => body.mef_chord(site, tol)?,
                Some(spec) => body.mef(site, spec, FaceSurface::Inherit, tol)?,
            };
            self.slivers.insert(created.face, ());
            self.fragments.push((created.face, owner));
            chords.push(created.edge);
        }
        // Ring re-homing (`laringmv`, the lkemr/ring-placement mirror
        // site): whenever the FIRST mef divided a face that still owns
        // rings. PR 3 shipped this call *inside* the second-chord
        // guard (Program 14.10's literal placement) and flagged the
        // skip window as a watch item; the re-homing need depends only
        // on the first mef having divided the face, not on whether the
        // second chord was minted, so the call now runs unconditionally
        // on `newf` (M3 PR 5 — the boolean joining reaches face-
        // dividing joins with unrelated rings present). The book never
        // re-homes after the second mef alone (its face is the sliver
        // between the two chords, which bounds no ring-holding region);
        // that placement is kept.
        if let Some((newf, outside)) = newf {
            let remainder = body
                .get_half_edge(outside)
                .ok_or_else(|| corrupt_he(outside))?
                .parent_loop;
            self.rehome_rings(body, oldf, newf, remainder)?;
        }
        Ok(chords)
    }

    /// `laringmv(oldf, newf)`: move every bystander ring of `oldf`
    /// enclosed by the mef run (`newf`'s outer) into `newf` — decided
    /// on the run's own edge carriers ([`point_in_loop`]),
    /// since a run bearing an arc does not bound the polygon through
    /// its vertices.
    ///
    /// The test is against the RUN, not `oldf`'s outer (issue #93):
    /// when the split loop was a RING of `oldf` (an island seam),
    /// `oldf`'s outer is untouched and still encloses everything — the
    /// old-outer test kept nested rings (an island inside an island)
    /// on the wrong face, silently. For an outer-loop split the two
    /// tests agree (the run and the remainder partition the old area).
    /// `remainder` — the split's own leftover cycle — is skipped, not
    /// tested: a ring-lane remainder is geometrically coincident with
    /// the run and would land `OnBoundary`.
    fn rehome_rings<T: Decide>(
        &mut self,
        body: &mut Body<T>,
        oldf: FaceKey,
        newf: FaceKey,
        remainder: LoopKey,
    ) -> Result<(), SplitJoinError> {
        let rings = body
            .get_face(oldf)
            .ok_or_else(|| corrupt_face(oldf))?
            .rings
            .clone();
        if rings.iter().all(|&r| r == remainder) {
            return Ok(());
        }
        let run = body.get_face(newf).ok_or_else(|| corrupt_face(newf))?.outer;
        let surface = body
            .get_face(oldf)
            .and_then(|f| body.get_surface(f.surface))
            .cloned()
            .ok_or_else(|| corrupt_face(oldf))?;
        let chart = matches!(
            surface,
            geom::Surface::Cylinder { .. } | geom::Surface::Sphere { .. }
        );
        let normal = if chart {
            None
        } else {
            Some(face_plane_normal(body, oldf)?)
        };
        for ring in rings {
            if ring == remainder {
                continue;
            }
            let side = match normal {
                Some(normal) => ring_side(body, ring, run, normal, self.band)?,
                None => chart_ring_side(body, &surface, newf, ring, self.band)?,
            };
            match side {
                LoopContainment::In => body.ring_move(ring, newf)?,
                LoopContainment::Out => {}
                LoopContainment::OnBoundary => {
                    return Err(SplitJoinError::RingHomingAmbiguous { ring });
                }
            }
        }
        Ok(())
    }

    /// `cut` (module docs): retire a fully-joined null edge. The
    /// completion outcome comes back unresolved — the caller assigns
    /// roles from its own side data.
    pub(crate) fn cut_core<T: Decide>(
        &mut self,
        body: &mut Body<T>,
        edge: EdgeKey,
    ) -> Result<CutOutcome, SplitJoinError> {
        let edge_data = body
            .get_edge(edge)
            .ok_or_else(|| corrupt_edge(edge))?
            .clone();
        let loop_of = |body: &Body<T>, he: HalfEdgeKey| -> Result<LoopKey, SplitJoinError> {
            Ok(body
                .get_half_edge(he)
                .ok_or_else(|| corrupt_he(he))?
                .parent_loop)
        };
        let l_plus = loop_of(body, edge_data.he_plus)?;
        let l_minus = loop_of(body, edge_data.he_minus)?;
        if l_plus == l_minus {
            // The last null edge of a section polygon: kemr leaves the
            // 2-loop null face.
            let face = body
                .get_loop(l_plus)
                .ok_or_else(|| corrupt_loop(l_plus))?
                .face;
            let result = body.kemr(edge_data.he_plus, edge_data.he_minus)?;
            Ok(CutOutcome::Completed {
                face,
                ring: result.ring,
            })
        } else {
            // Interior null edge: kef merges the two slivers. Kill a
            // sliver side (never a real face), deterministically
            // preferring he_plus's side.
            let f_plus = body
                .get_loop(l_plus)
                .ok_or_else(|| corrupt_loop(l_plus))?
                .face;
            let f_minus = body
                .get_loop(l_minus)
                .ok_or_else(|| corrupt_loop(l_minus))?
                .face;
            let victim = if self.slivers.contains_key(f_plus) {
                edge_data.he_plus
            } else if self.slivers.contains_key(f_minus) {
                edge_data.he_minus
            } else {
                return Err(SplitJoinError::CutInvariant { edge });
            };
            let killed = body.kef(victim)?;
            self.slivers.remove(killed.killed_face);
            Ok(CutOutcome::Merged)
        }
    }
}

/// The face's **chart** plane normal (F5-gated: always a `Plane`),
/// deliberately without the face's sense folded in.
///
/// Its one consumer is [`point_in_loop`], which reads the normal only
/// to recover the loop's PLANE, and whose verdict is exactly invariant
/// under `n̂ ↦ −n̂` over lines and conics alike. **That derivation lives
/// at [`point_in_loop`]**, under the function whose property it is
/// rather than under the five-line producer that relies on it; the
/// consequence here is that ring re-homing cannot move a ring on the
/// sense bit. `tests/review_m3_pr3_pil.rs` pins it for the straight
/// rows and `validate.rs`'s
/// `point_in_loop_is_blind_to_the_normals_sign_on_an_arc_bearing_loop`
/// for the conic rows.
///
/// The contrast with [`crate::boolean::solid_contain`]'s `face_plane`,
/// which multiplies although its own consumer is equally sign-blind,
/// is a naming contract rather than a correctness one: that door
/// promises an OUTWARD normal to whoever calls it next. This one
/// promises a chart normal and is named for it, so it is not a site
/// of [`crate::face_normal`]'s hand-multiply inventory.
fn face_plane_normal<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
) -> Result<geom_core::Vec3<T>, SplitJoinError> {
    let f = body.get_face(face).ok_or_else(|| corrupt_face(face))?;
    match body.get_surface(f.surface) {
        Some(geom::Surface::Plane { normal, .. }) => Ok(*normal),
        // Corruption and an unwired arm are different refusals.
        None => Err(corrupt_face(face)),
        Some(_) => Err(SplitJoinError::SectionInvariant {
            face,
            what: "ring re-homing reads the divided face's plane; this face's carrier is not \
                   a plane (arm not wired)",
        }),
    }
}

/// Which side of `run` a bystander ring lies on, read at its first
/// vertex off `run`'s boundary: a ring disjoint from the run cannot
/// cross it, but it may touch it at a vertex — a pinch, where two
/// sections meet at one point — so its anchor alone can land `OnBoundary`
/// on a ring that is plainly on one side. `OnBoundary` only when every
/// vertex does.
fn ring_side<T: Decide>(
    body: &Body<T>,
    ring: LoopKey,
    run: LoopKey,
    normal: Vec3<T>,
    band: Band,
) -> Result<LoopContainment, SplitJoinError> {
    let vertices = match body
        .get_loop(ring)
        .ok_or_else(|| corrupt_loop(ring))?
        .boundary
    {
        LoopBoundary::Cycle { first } => body
            .loop_cycle(first)
            .ok_or_else(|| corrupt_he(first))?
            .into_iter()
            .map(|he| Ok(body.get_half_edge(he).ok_or_else(|| corrupt_he(he))?.start))
            .collect::<Result<Vec<_>, SplitJoinError>>()?,
        LoopBoundary::Empty { vertex } => vec![vertex],
    };
    for v in vertices {
        let p = vertex_point(body, v)?;
        match point_in_loop(body, run, normal, p, band)? {
            LoopContainment::OnBoundary => {}
            side => return Ok(side),
        }
    }
    Ok(LoopContainment::OnBoundary)
}

/// [`ring_side`] on a cylinder wall's chart: which side of
/// `newf`'s outer loop (the run a `mef` just walled off) `ring` lies on,
/// by the parity of a ray from each ring vertex toward `+v` at constant
/// azimuth.
///
/// Exact on the chart: every run edge's azimuth channel is linear in its
/// carrier parameter ([`chart_v_du`]), so the ray meets an edge at most
/// once, where its azimuth reaches the ray's, and the edge's own `v`
/// there is read from its harmonic form; the straight chart rows between
/// images (the walk's junction gaps) are segments. A ring vertex is placed
/// on the run's branch, which is one branch because the run's window is
/// decided under a period. Each comparison is a named trilean metered in
/// metres; a ring vertex on the ray's degenerate rows (the run passes
/// through its azimuth at a vertex, or along it) says nothing and the
/// next vertex is asked, as [`ring_side`] does for a vertex on the run.
/// A sphere face refuses typed ([`no_wall_chart`]).
///
/// This is the third point-in-region routine beside [`ring_side`] (on a
/// plane) and `solid_contain`'s wall outline (a point against a whole
/// wall face's outline, inside the containment gate): a known split,
/// each reading the region it is handed in its own chart.
fn chart_ring_side<T: Decide>(
    body: &Body<T>,
    surface: &geom::Surface<T>,
    newf: FaceKey,
    ring: LoopKey,
    band: Band,
) -> Result<LoopContainment, SplitJoinError> {
    let WallChart {
        origin: centre,
        axis,
        radius,
        u_ref,
    } = wall_chart(surface).ok_or_else(|| no_wall_chart(newf, surface))?;
    let tau = T::tau();
    let invariant = |what| SplitJoinError::SectionInvariant { face: newf, what };
    let images = face_azimuth_images(body, surface, newf, band)?.ok_or(invariant(
        "ring re-homing on a chart: the run is not a cycle",
    ))?;
    let (lo, hi) = azimuth_hull(&images).ok_or(invariant(
        "ring re-homing on a chart: the run carries no charted edge",
    ))?;
    let decide_m = |name, margin| {
        decide(name, margin, band).map_err(|diag| SplitJoinError::Escalated { face: newf, diag })
    };
    if decide_m(
        "split_ring_chart_window",
        Margin::levered(tau - (hi - lo), radius),
    )? != Sign::Positive
    {
        return Err(invariant(
            "ring re-homing on a chart: the run's azimuth window spans a full period, so a \
             ring vertex has no single branch on it",
        ));
    }
    let mid = (lo + hi) * T::from_f64(0.5);
    let vertices = match body
        .get_loop(ring)
        .ok_or_else(|| corrupt_loop(ring))?
        .boundary
    {
        LoopBoundary::Cycle { first } => body
            .loop_cycle(first)
            .ok_or_else(|| corrupt_he(first))?
            .into_iter()
            .map(|he| Ok(body.get_half_edge(he).ok_or_else(|| corrupt_he(he))?.start))
            .collect::<Result<Vec<_>, SplitJoinError>>()?,
        LoopBoundary::Empty { vertex } => vec![vertex],
    };
    // The chart segments of the run: each edge (`Some(image)`), then the
    // straight row to the next image's entry.
    let n = images.len();
    'vertex: for v in vertices {
        let w = vertex_point(body, v)? - centre;
        let raw = stable_azimuth(w.dot(axis.cross(u_ref)), w.dot(u_ref), band);
        let u_p = raw + (mid - raw).periodic_branch(tau) * tau;
        let v_p = w.dot(axis);
        let mut crossings = 0usize;
        for (i, image) in images.iter().enumerate() {
            let next = &images[(i + 1) % n];
            let rows = [
                (image.entry, image.exit, Some(image)),
                (image.exit, next.entry, None),
            ];
            for (u0, u1, edge) in rows {
                let s0 = decide_m(
                    "split_ring_chart_ray_azimuth",
                    Margin::levered(u_p - u0, radius),
                )?;
                let s1 = decide_m(
                    "split_ring_chart_ray_azimuth",
                    Margin::levered(u_p - u1, radius),
                )?;
                if s0 == Sign::Zero || s1 == Sign::Zero {
                    continue 'vertex;
                }
                if s0 == s1 {
                    continue;
                }
                let f = (u_p - u0) / (u1 - u0);
                let v_x = match edge {
                    Some(image) => {
                        image
                            .harmonic
                            .as_ref()
                            .ok_or(invariant(
                                "ring re-homing on a chart: a run edge's chart image is fitted",
                            ))?
                            .eval(image.t.0 + f * (image.t.1 - image.t.0))
                            .y
                    }
                    None => image.v.1 + f * (next.v.0 - image.v.1),
                };
                match decide_m("split_ring_chart_ray_height", Margin::of(v_x - v_p))? {
                    Sign::Positive => crossings += 1,
                    Sign::Negative => {}
                    Sign::Zero => continue 'vertex,
                }
            }
        }
        return Ok(if crossings % 2 == 1 {
            LoopContainment::In
        } else {
            LoopContainment::Out
        });
    }
    Ok(LoopContainment::OnBoundary)
}

/// A representative point of a loop (its anchor vertex).
pub(crate) fn ring_representative<T: Decide>(
    body: &Body<T>,
    ring: LoopKey,
) -> Result<Point3<T>, SplitJoinError> {
    let v = match body
        .get_loop(ring)
        .ok_or_else(|| corrupt_loop(ring))?
        .boundary
    {
        LoopBoundary::Cycle { first } => {
            body.get_half_edge(first)
                .ok_or_else(|| corrupt_he(first))?
                .start
        }
        LoopBoundary::Empty { vertex } => vertex,
    };
    vertex_point(body, v)
}
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use crate::entity::FaceKey;
    use geom_core::Tol;

    use geom_core::{Point3, Vec3};

    /// A body with one cylinder face (unit radius about z) and two
    /// vertices on the tilted section ellipse this fixture's own plane
    /// cuts (tilt φ = 0.5 about y), at conic parameters θ = 0 and
    /// θ = π/2 — the chord endpoints `chord_spec` connects.
    fn cyl_fixture() -> (
        crate::Body<f64>,
        crate::entity::FaceKey,
        crate::entity::VertexKey,
        crate::entity::VertexKey,
        SectionCtx<f64>,
    ) {
        let phi = 0.5f64;
        let plane = crate::test_support::split_plane(
            Point3::new(0.0, 0.0, 0.0),
            Vec3::new(phi.sin(), 0.0, phi.cos()),
            Tol::witness(),
        );
        let normal = plane.normal.get();
        // The section ellipse of (plane × unit cylinder about z):
        // center at the axis piercing (origin), minor dir ŵ =
        // normalize(ẑ×n̂) = ŷ… compute points directly from the
        // closed form: v̂ = ŷ, û = v̂×n̂; a = 1/cos φ, b = 1.
        let v_e = Vec3::new(0.0, 1.0, 0.0);
        let u_e = v_e.cross(normal);
        let a = 1.0 / phi.cos();
        let at = |theta: f64| -> Point3<f64> {
            let (s, c) = geom_core::Real::sin_cos(theta);
            Point3::origin() + u_e * (a * c) + v_e * s
        };
        let p1 = at(0.0);
        let p2 = at(core::f64::consts::FRAC_PI_2);
        let mut body = crate::Body::<f64>::new();
        let seed = body.mvfs(p1, true).unwrap();
        body.set_face_surface(
            seed.face,
            crate::FaceSurface::New {
                surface: geom::Surface::Cylinder {
                    origin: Point3::origin(),
                    axis: Vec3::unit_z(),
                    radius: 1.0,
                    u_ref: Vec3::unit_x(),
                },
                sense: true,
            },
        )
        .unwrap();
        let mev = body
            .mev_line(
                crate::MevSite::Lone {
                    r#loop: seed.r#loop,
                },
                p2,
                Tol::witness(),
            )
            .unwrap();
        let ctx = SectionCtx {
            origin: plane.origin,
            normal: plane.normal,
            plane_key: None,
        };
        (body, seed.face, seed.vertex, mev.vertex, ctx)
    }

    /// Mints a certified rim arc (unit circle at z = 0 on the fixture's
    /// cylinder, parameter = chart azimuth because the circle's `u_ref`
    /// is the chart's) as an independent edge of `body`, and returns its
    /// forward half-edge — the RUN the window is derived from. The
    /// interval must be forward and shorter than a period (the ordinary
    /// certification gates).
    fn rim_run(body: &mut crate::Body<f64>, t0: f64, t1: f64) -> crate::entity::HalfEdgeKey {
        let carrier = geom::Curve3::Circle {
            center: Point3::origin(),
            axis: Vec3::unit_z(),
            radius: 1.0,
            u_ref: Vec3::unit_x(),
        };
        // The seed solid first: `mvfs` asserts tier-1 validity, and a
        // surface nothing references yet is an orphan.
        let seed = body.mvfs(carrier.eval(t0), true).unwrap();
        let cyl = body.add_surface(geom::Surface::Cylinder {
            origin: Point3::origin(),
            axis: Vec3::unit_z(),
            radius: 1.0,
            u_ref: Vec3::unit_x(),
        });
        let plane = body.add_surface(geom::Surface::Plane {
            origin: Point3::origin(),
            normal: Vec3::unit_z(),
            u_ref: Vec3::unit_x(),
        });
        let made = body
            .mev(
                crate::MevSite::Lone {
                    r#loop: seed.r#loop,
                },
                carrier.eval(t1),
                EdgeCurveSpec {
                    description: geom_brep::EdgeDescriptionSpec::Intersection {
                        s1: cyl,
                        s2: plane,
                        witness: carrier.mid_point(t0, t1),
                    },
                    carrier,
                    param_start: t0,
                    param_end: t1,
                },
                Tol::witness(),
            )
            .unwrap();
        body.get_edge(made.edge).unwrap().he_plus
    }

    /// The split lane's adjacency question on a conic between edge (a
    /// cylinder cap's rim, which a planar divided face carries): the
    /// belly verdict and the coplanar verdict. The rim is the upper
    /// semicircle of the unit circle in z = 0, from (1, 0, 0) to
    /// (−1, 0, 0). A boolean lane asking it is refused: their skip reads
    /// the segment's locus.
    #[test]
    fn split_lane_decides_a_conic_between_edge_against_its_section_plane() {
        let band = Band::new(1e-9, 1e-8).unwrap();
        let mut body = crate::Body::<f64>::new();
        let rim = rim_run(&mut body, 0.0, core::f64::consts::PI);
        let verdict = |normal: Vec3<f64>| {
            let plane = crate::test_support::split_plane(Point3::origin(), normal, Tol::witness());
            let mut ctx = SectionCtx {
                origin: plane.origin,
                normal: plane.normal,
                plane_key: None,
            };
            between_edge_is_section(&body, &JoinLane::Split(&mut ctx), rim, band).unwrap()
        };
        // A section plane through the rim's two ends and the cap's
        // centre (y = 0): the rim bellies to y = 1, so it is no section
        // segment and the chord is minted.
        assert_eq!(verdict(Vec3::unit_y()), Some(false));
        // The section plane holding the whole rim (z = 0): Zero → skip.
        assert_eq!(verdict(Vec3::unit_z()), Some(true));
        let asked = between_edge_is_section(&body, &JoinLane::Planar, rim, band);
        assert!(
            matches!(asked, Err(SplitJoinError::SectionInvariant { .. })),
            "{asked:?}"
        );
    }

    /// `chord_spec` on the fixture's chord, θ = 0 to θ = π/2, with the
    /// section leaving θ = 0 along `leave`.
    fn spec_leaving(leave: Vec3<f64>) -> Result<Option<EdgeCurveSpec<f64>>, SplitJoinError> {
        let band = Band::new(1e-9, 1e-8).unwrap();
        let (mut body, face, u1, u2, mut ctx) = cyl_fixture();
        chord_spec(
            &mut body,
            band,
            JoinLane::Split(&mut ctx),
            face,
            u1,
            u2,
            leave,
        )
    }

    /// **The datum orients the chord.** The fixture's ellipse is
    /// `û·a·cos θ + ŷ·sin θ`, so its tangent at θ = 0 is `ŷ`: leaving
    /// along `+ŷ` the chord is the quarter through θ = π/4, leaving along
    /// `−ŷ` the three quarters through θ = −3π/4. Either way it runs from
    /// the first vertex to the second, and a datum of any positive
    /// length reads the same.
    #[test]
    fn the_datum_orients_the_chord() {
        let phi = 0.5f64;
        let n = Vec3::new(phi.sin(), 0.0, phi.cos());
        let u_e = Vec3::unit_y().cross(n);
        let at = |theta: f64| -> Point3<f64> {
            Point3::origin() + u_e * (theta.cos() / phi.cos()) + Vec3::unit_y() * theta.sin()
        };
        let pi = core::f64::consts::PI;
        for (what, leave, mid) in [
            ("the quarter", Vec3::unit_y(), at(pi / 4.0)),
            (
                "the quarter, a short datum",
                Vec3::unit_y() * 1e-3,
                at(pi / 4.0),
            ),
            ("the three quarters", -Vec3::unit_y(), at(-0.75 * pi)),
        ] {
            let spec = spec_leaving(leave).unwrap().expect("a conic chord");
            let c = &spec.carrier;
            let close = |p: Point3<f64>, q: Point3<f64>| (p - q).norm() < 1e-12;
            assert!(
                close(c.eval(spec.param_start), at(0.0)),
                "{what}: starts at u1"
            );
            assert!(
                close(c.eval(spec.param_end), at(pi / 2.0)),
                "{what}: ends at u2"
            );
            assert!(
                close(c.mid_point(spec.param_start, spec.param_end), mid),
                "{what}: passes {mid:?}"
            );
        }
    }

    /// **A datum with no component along the section refuses**, typed,
    /// and one whose component is in the band escalates naming
    /// `chord_arc_leave`: the section's normal (the plane's own, which
    /// no section tangent has a component along), and the tangent
    /// scaled to put the levered margin `leave·Ĉ′ × a` at 5e-9.
    #[test]
    fn a_datum_off_the_section_refuses() {
        let phi = 0.5f64;
        let n = Vec3::new(phi.sin(), 0.0, phi.cos());
        let err = spec_leaving(n).unwrap_err();
        assert!(
            matches!(err, SplitJoinError::SectionInvariant { .. }),
            "{err:?}"
        );
        let err = spec_leaving(n + Vec3::unit_y() * (5e-9 * phi.cos())).unwrap_err();
        let SplitJoinError::Escalated { diag, .. } = err else {
            panic!("expected an escalation, got {err:?}");
        };
        assert_eq!(diag.predicate, Some("chord_arc_leave"));
    }

    #[test]
    fn section_area_pair_carries_the_shared_recourse() {
        let face = FaceKey::default();
        let msg = SplitJoinError::DegenerateSection { face }.to_string();
        assert_eq!(msg.matches(JOIN_RECOURSE).count(), 1, "{msg}");
        assert!(!msg.contains("declare"), "{msg}");

        // The spur arm carries the same recourse, and it does not claim
        // the zero area its section does not have.
        let msg = SplitJoinError::SectionSpur { face }.to_string();
        assert_eq!(msg.matches(JOIN_RECOURSE).count(), 1, "{msg}");
        assert!(!msg.contains("declare"), "{msg}");
        assert!(!msg.contains("zero area"), "{msg}");

        let msg = SplitJoinError::Escalated {
            face,
            diag: Indeterminate {
                margin: geom_core::MarginDiag::value(5e-9),
                band: Band::new(1e-9, 1e-8).unwrap(),
                predicate: Some("split_section_area"),
                terminal_sliver: false,
            },
        }
        .to_string();
        assert_eq!(msg.matches(JOIN_RECOURSE).count(), 1, "{msg}");
        assert!(!msg.contains("declare"), "{msg}");
    }

    /// **The anti-re-fork row for the arc a chord takes.** Its one
    /// rung, `chord_arc_leave`, is decided in exactly ONE place in this
    /// crate — counted, not merely located: for most of this module's
    /// life `chord_spec` and `bool_planar_chord_spec` sat 500 lines
    /// apart carrying line-identical copies of their arc selection, and
    /// a cross-file guard would have been green throughout.
    ///
    /// **What it cannot match** — three shapes:
    ///
    /// 1. **A second rule under a FRESH predicate name**, re-deriving the
    ///    arc from the face. It surfaces as new rows in
    ///    `docs/K-REPORT.md`'s census, which is the mechanism that
    ///    already exists for that.
    /// 2. **A reading of the arc from `arc_leaving`'s RESULT** —
    ///    recomputing it from the returned carrier's axis, say. The rung
    ///    still fires once, and no string search can see it.
    /// 3. **A copy in another crate.** The count is scoped to
    ///    `topo/src`; `crate::validate::decide` is `pub(crate)`, so a
    ///    foreign crate would have to call `geom_core`'s directly.
    #[test]
    fn the_chord_arc_rung_is_decided_in_one_place() {
        // Assembled rather than spelled, so this file is subject to the
        // count like any other.
        let rung = format!("\"chord_{}\"", "arc_leave");
        let home = crate::source_walk::src_root().join("chord_join.rs");
        let files = crate::source_walk::crate_sources();
        assert!(files.contains(&home), "the walk did not find chord_join.rs");
        let mut sites = 0;
        for path in &files {
            let text = std::fs::read_to_string(path).expect("a readable source file");
            // DECIDE sites, counted on a whitespace-stripped copy so a
            // call broken across lines counts the same as an inline one.
            let stripped: String = text.chars().filter(|c| !c.is_whitespace()).collect();
            sites += stripped.matches(&format!("decide({rung}")).count();
            assert!(
                path == &home || !text.contains(rung.as_str()),
                "{} names {rung}: the chord's arc has been re-forked out of chord_join.rs, \
                 which must hold the only one. Call `arc_leaving` instead.",
                path.display()
            );
        }
        assert_eq!(sites, 1, "{rung} is decided at {sites} site(s)");
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod section_case_pair_tests {
    use geom_core::{Band, Point3, Tol, Vec3};

    use super::{SectionCase, section_case};
    use crate::entity::FaceKey;
    use crate::splitting::SplitJoinError;

    fn band() -> Band {
        Band::linear(Tol::witness()).expect("a linear band")
    }

    fn plane() -> geom::Surface<f64> {
        geom::Surface::Plane {
            origin: Point3::new(0.0, 0.0, 0.5),
            normal: Vec3::new(0.0, 0.0, 1.0),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        }
    }

    fn cylinder() -> geom::Surface<f64> {
        geom::Surface::Cylinder {
            origin: Point3::new(0.0, 0.0, 0.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
            radius: 1.0,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        }
    }

    fn sphere() -> geom::Surface<f64> {
        geom::Surface::Sphere {
            center: Point3::new(0.0, 0.0, 0.0),
            radius: 2.0,
            axis: Vec3::new(0.0, 0.0, 1.0),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        }
    }

    /// The dispatch reads a PAIR: which member carries the plane is
    /// this table's question, not the caller's, so both orders name the
    /// same conic.
    #[test]
    fn the_pair_is_order_free() {
        let f = FaceKey::default();
        for (a, b) in [(plane(), cylinder()), (cylinder(), plane())] {
            let got = section_case(f, band(), &a, &b, 4.0).expect("the rim arm is wired");
            let SectionCase::Conic(c) = got else {
                panic!("a square cut names a rim circle");
            };
            assert!((c.sa - 1.0).abs() < 1e-12 && (c.sb - 1.0).abs() < 1e-12);
        }
    }

    /// A pair with no arm refuses TYPED. It must never fall through to
    /// `Straight`, which the callers mint a straight chord from.
    #[test]
    fn a_pair_without_a_plane_refuses_typed() {
        let f = FaceKey::default();
        for (a, b) in [
            (cylinder(), cylinder()),
            (cylinder(), sphere()),
            (sphere(), sphere()),
        ] {
            match section_case(f, band(), &a, &b, 4.0) {
                Err(SplitJoinError::SectionInvariant { .. }) => {}
                Err(e) => panic!("a curved pair must refuse SectionInvariant, got {e:?}"),
                Ok(_) => panic!("a curved pair must refuse typed, never classify"),
            }
        }
        match section_case(f, band(), &plane(), &plane(), 4.0) {
            Err(SplitJoinError::SectionInvariant { .. }) => {}
            Err(e) => panic!("a planar pair must refuse SectionInvariant, got {e:?}"),
            Ok(_) => panic!("a planar pair must refuse typed here, never classify"),
        }
    }
}
