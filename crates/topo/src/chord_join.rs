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
//!   mirror test pins the outcome, not the citation, tol), skipped
//!   where `prev(prev(h1)) == h2` AND the one edge between them is the
//!   section segment ([`SegmentEdge`]) — any other edge between needs
//!   its chord;
//! - different loops ⇒ `mekr` with the **ring chosen structurally**
//!   (the loop that is not the face's outer; the book's fixed
//!   `lmekr(h1, h2->nxt)` argument order assumes GWB's list layout —
//!   ours is explicit outer/ring data);
//! - then the second chord `mef(Chords { he1: h2, he2: next(h1) })`,
//!   skipped where `next(next(h1)) == h2` and the edge between is the
//!   segment; both chords are planned before either is minted
//!   ([`JoinPlan`]) and minted on the segment's one curve
//!   ([`SegmentCurve`]); if the first `mef` split a
//!   face that still owns rings, the rings are re-homed by trilean
//!   containment ([`crate::splitting::containment`] on a plane,
//!   [`path_ring_side`] on a cylinder, a sphere, a cone or a torus) +
//!   [`Body::ring_move`](crate::construction::Construct::ring_move) — the `laringmv` step (lkemr/ring-placement
//!   mirror site);
//! - two halves of one null edge making up a ring alone (a one-site
//!   section loop's pierce of a planar or spherical face), or with the
//!   closed edge the segment runs along ⇒ the ring is promoted to a face
//!   and joined there, and the conic whose region holds the old face's
//!   outer loop goes back to it as a ring
//!   ([`ChordJoiner::join_lone_ring`]).
//!
//! `cut(edge)` retires a fully-joined null edge: halves in different
//! loops ⇒ `kef` (merge the two sliver faces — the killed side must be
//! a join-minted sliver, asserted); same loop ⇒ the section polygon is
//! COMPLETE and `kemr` leaves the 2-loop null face, which the core
//! hands back as [`CutOutcome::Completed`] with the roles UNRESOLVED.
//!
//! **Role resolution and side-specific certification are the callers'**
//! — that is what makes this core side-agnostic. The split lane
//! resolves roles by membership of its null edges' above-end set, reads
//! a slit trace from the loops' topology, and certifies every other
//! section's area (`split_section_area`, refusing a zero-area polygon
//! as [`SplitJoinError::DegenerateSection`]); the
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
//! (`chord_arc_leave_germ`, `chord_arc_leave_section`), so a chord reads
//! no chart: every conic that reaches it takes its arc the same way. A
//! copy of an edge is the exception: it is that edge's curve, not a
//! section, oriented by the edge rather than by [`Leave`], and stated
//! in the chart of the face it divides where the edge's description is
//! not one a chord states ([`chord_description`]). An open edge's copy
//! ([`along_edge_spec`]) runs from whichever of the edge's ends null
//! edges tie the chord's start to. A closed edge's copy
//! ([`closed_edge_spec`]) runs beside the edge it copies, in a loop that
//! bounds no area, so it has to run against that edge's half there,
//! which is a question about the `mef` site, not the section
//! ([`closed_chord_runs_forward`]); it is always stated in the chart of
//! the face it divides. A transverse
//! whole-turn chord (a wrap edge's pierce) keeps [`Leave`]: each of its
//! two chords walls off a face bounded by one conic, and which of them
//! is the hole is read after the join, from the geometry ([`RimSide`]),
//! so either direction joins. A
//! section of two loops (a torus's) is first narrowed to the loop the
//! chord's sites lie on. What reaches it is bounded upstream, not here:
//! the boolean's planar side wires a cylinder, sphere, cone or torus
//! partner, and the split's reduce refuses a sphere and a torus.
//!
//! A **self-loop chord** (both ends one vertex) is a one-site section
//! loop's: a one-face closed wall cut across its one wrap edge, in the
//! split or the boolean, and in the boolean also the partner's planar
//! face, whose interior that edge pierces
//! ([`ChordJoiner::join_lone_ring`]); and in the boolean a closed edge
//! (one vertex, both its ends) lying in the other operand, which copies
//! that edge whole on its own side ([`closed_edge_spec`]) and is the
//! section conic in the partner's planar or spherical face. There the
//! chord is the whole section conic, from the vertex round to itself.
//! A chord between two copies of one site, which the null edges tie
//! ([`same_site`]), runs round the same way: a section conic that meets
//! a face's boundary at one site alone reaches it under whichever of
//! the site's copies the split's null edges leave there.
//! In the plane × plane
//! lane, on a planar face the split divides, or on a ruling section, a
//! self-loop is a lone site, and rides `mef`'s placeholder circle
//! (`EdgeCurveSpec::self_loop_circle_at`), which bounds nothing. A
//! tangent ruling has no self-loop: its two ends coincide along the
//! ruling, which refuses.

use geom_brep::{EdgeCurveSpec, Pcurve, chart_pcurve_over};
use geom_core::{
    Band, BandError, Decide, Indeterminate, InfSpeed, Margin, Point3, Real, Sign, UnitVec3, Vec3,
};
use slotmap::SecondaryMap;

use crate::body::Body;
use crate::entity::{EdgeKey, EntityId, FaceKey, HalfEdgeKey, LoopBoundary, LoopKey, VertexKey};
use crate::euler::{EulerOpError, FaceSurface, MefSite};
use crate::euler_ring::MekrSite;
use crate::geometry::SurfaceKey;
use crate::live::{Proven, linked, proven};
use crate::null::CurveGeom;
use crate::ring_path::{LoopArc, LoopSplit, Path, Quadric, SplineArc, path_parity};
use crate::splitting::containment::{LoopContainment, PointInLoopError, point_in_loop};
use crate::splitting::order::{FRAME_ARM, FRAME_AXIS};
use crate::splitting::rules::{face_axial_range, face_extent};
use crate::validate::decide;
use geom_core::Tol;

/// Why a curved face's crossings could not be paired along the face's
/// section (`splitting::join`'s conic and ruling pairings): the
/// section's heading at a crossing — which way along it runs into the
/// face — is what pairs them, and here it did not.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConicCrossingsCase {
    /// At a crossing the plane runs within the band of tangent to the
    /// face's wall (**`split_join_conic_heading`** Zero), so the
    /// crossing neither enters nor leaves the face definitely. No
    /// shipped fixture reaches it; the same sine read Zero at a chord's
    /// start is [`SplitJoinError::SectionDepartureGrazes`].
    Grazing,
    /// Taken along the conic, the crossings do not alternate between
    /// entering the face and leaving it. Crossings closer along the
    /// conic than the band read as one point and keep their insertion
    /// order (`split_join_line_gap` Zero), so two crossings of the face
    /// at one point — the plane through a vertex of it — can land
    /// here, as can every crossing of the face at one point, or up/down
    /// senses that disagree with the geometry. On a section of two
    /// rulings, a ruling whose crossings do not pair off along it lands
    /// here too. No shipped fixture reaches it.
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

/// Which reading of the input puts a section chord in band of what it is
/// pinned to ([`SplitJoinError::SectionEndEscalated`]), by how the
/// section table read the plane.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SectionEndReading {
    /// The table cut the plane's own section: the miss is the offsets of
    /// the vertices the input put on the plane, each read in its zero
    /// band, at the end they meet. An edge on its faces: the Boolean
    /// refuses it as its own `EdgeOnFaces` decision, so this arm's words
    /// are the split's.
    OnPlane,
    /// The table read the plane square to a curved face's axis and stood
    /// the axis-normal circle in for its section, so the plane's tilt off
    /// square, read in its zero band, adds to the vertex's offset at an
    /// end, and is the circle's whole miss off the plane.
    SquareToAxis,
}

/// The lever of a section read square to a curved face's axis that
/// misses its vertex in band, at both doors that join sections: either
/// reading of the input it adds clears it.
const SQUARE_TO_AXIS_LEVER: &str = concat!(
    geom_core::coincidence_move_arm!(),
    " so each plane that cuts a curved face runs clearly square to its axis or clearly off \
     square, and each vertex that meets it lies clearly on it or clearly off it"
);

/// What a section chord's pin read where its carrier's own rounding
/// leaves it no reading of the input ([`SplitJoinError::SectionUnresolved`]).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SectionPinMiss {
    /// The pin read in band, on a carrier whose resolution is past the
    /// `ε/K` share of the band that rounding may take.
    Undecided {
        /// The pin.
        check: geom_brep::CertCheck,
        /// The pin's diagnostics.
        diag: Indeterminate,
        /// The carrier's resolution `K` times over, as the band read it.
        resolution: geom_core::MarginDiag,
    },
    /// An end pin read past the band, by a miss the carrier's
    /// resolution covers to within the zero band.
    Past {
        /// The end.
        check: geom_brep::CertCheck,
        /// The sample the gate read.
        sample: u32,
        /// The miss, as the gate read it.
        margin: geom_core::MarginDiag,
    },
}

/// The cause an in-band [`SectionPinMiss::Undecided`] states before its
/// last resort.
const SECTION_UNRESOLVED_CAUSE: &str = "The section curve's own rounding over this edge is too coarse to read this miss as the model's.";

/// The cause a [`SectionPinMiss::Past`] states before its last resort.
const SECTION_WITHIN_CAUSE: &str =
    "The section curve's own rounding over this edge can account for this miss.";

/// Typed failure of the joining step.
#[derive(Debug)]
pub enum SplitJoinError {
    /// The split's join order escalated (interval lane only): building
    /// its frame — whether the plane is an axis plane
    /// (`split_join_frame_axis`), or whether a schedule member projects
    /// into it (`split_join_frame_arm`) — or comparing two section
    /// points in it (`split_join_order_u` / `_v`). The sentence names
    /// which ([`order_decision`]).
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
    /// A section loop that closes on one vertex bounds nothing however
    /// large the section it should be
    /// (`work/cleave/a-revolved-tube-split-across-its-axis-refuses-a-degenerate-section.md`).
    /// A pinch at a vertex does not reach it: the split's reduction keeps
    /// one vertex per piece round a vertex on each side, so pieces
    /// meeting there do not share one. A pinch along an edge in the
    /// plane, wrapped round at both ends by the other side's material,
    /// would: its trace is a polygon of its own, which the split reads as
    /// a slit trace from its topology before its area, and builds. A
    /// plane tangent to a curved wall does not reach it either: rule (b)
    /// classifies a convex graze with its material, and rule (a) refuses
    /// a concave one ([`crate::SplitReduceError::KnifeEdge`]).
    DegenerateSection {
        /// The completed null face.
        face: FaceKey,
    },
    /// A slit trace whose tree holds an edge that is not a line: its
    /// slit would be two coincident curved edges, which no contact record
    /// backs and tier 3′ refuses
    /// (`work/cleave/a-curved-slit-fails-tier-3-prime-on-its-two-coincident-arcs.md`).
    CurvedSlitTrace {
        /// The completed null face.
        face: FaceKey,
    },
    /// Ring re-homing could not decide: the walk escalated, exhausted
    /// its schedule, or met an edge of the divided face's outline it
    /// cannot cross ([`PointInLoopError::Uncrossable`]).
    RingHoming(PointInLoopError),
    /// No vertex of a ring says which side of the run dividing its face
    /// off it is on: every vertex is ON the run (a ring an
    /// ill-conditioned operand put there), or, on a wall's chart, no
    /// vertex's ray is decided. A pierce ring on a plane with every
    /// vertex on the run is deferred instead ([`ChordJoiner`]'s pending
    /// rings), and refuses this way only if its join cannot place it
    /// either: it meets a pending ring in another face, a polygon
    /// completes inside it, or it is still pending when either sweep is
    /// done.
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
    /// A closed section loop has ONE site that the join does not close
    /// a loop at: the one vertex on the loop holds both its ends, and
    /// the join closes a loop on itself only at a wrap edge crossing a
    /// planar face, or along a closed edge lying in a planar or a
    /// spherical face or along another closed edge
    /// (`boolean::join::one_site_segments`). Any other is refused typed,
    /// before the loose ends are counted.
    SingleSiteSectionLoop {
        /// How many such loops.
        count: usize,
    },
    /// A section loop mixed above ends with below ends —
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
    /// band of it. A crossing's two flanks can both read that way only
    /// where no face of either offers a point of its interior: a
    /// curved face, the frontier of
    /// `work/cleave/the-uncut-shell-witness-reads-no-curved-face-interior`,
    /// or a planar one none of whose interior candidates certifies
    /// (`crate::stands`, rung 3) — or where the two solids' faces lie
    /// within the band of each other (a settled in-band coincidence).
    /// No kernel defect.
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
    /// The join's face division list refused a row where a chord added
    /// it ([`crate::lineage::FaceDivisions::push`]): a kernel defect.
    LineageRow(crate::lineage::RowRefusal),
    /// The exact-order band's constants did not construct —
    /// structurally impossible for the two literal bit patterns, typed
    /// rather than panicked (no panic paths in operator code).
    Band(BandError),
    /// An underlying Euler operation refused.
    Euler(EulerOpError),
    /// A section chord's end reads within the band of the vertex it
    /// meets, and the miss is a reading of the input's
    /// ([`SectionEndReading`]): its carrier resolves the pin to within
    /// the share of the band rounding may take, so the miss is the
    /// input's.
    SectionEndEscalated {
        /// The face the chord divides.
        face: FaceKey,
        /// The reading that puts the pin in band.
        reading: SectionEndReading,
        /// The pin's diagnostics.
        diag: Indeterminate,
    },
    /// A section chord's attachment pin misses where its carrier's own
    /// resolution over the edge
    /// ([`geom_brep::carrier_resolution::resolution`]) leaves no reading
    /// of the input ([`SectionPinMiss`]): a kernel limitation (D4 ¶1
    /// (i)'s last resort), with no lever.
    SectionUnresolved {
        /// The face the chord divides.
        face: FaceKey,
        /// What the pin read.
        miss: SectionPinMiss,
    },
    /// The C5 section classification refused while minting a curved
    /// face's section chord (M5 PR 5) — the typed table verdict nested
    /// whole (escalations carry the shared recourse through it).
    Section {
        /// The face being divided.
        face: FaceKey,
        /// The table's refusal.
        source: geom_brep::SectionError,
    },
    /// The plane cuts a cone face in a parabola or a hyperbola. The
    /// table serves either in closed form and mints its arcs per window
    /// as rational-quadratic NURBS (`geom_brep`'s C1), but the split
    /// does not carry such a chord, so the face is not divided along it.
    OpenConicChord {
        /// The face being divided.
        face: FaceKey,
        /// The conic.
        conic: geom_brep::OpenConic,
    },
    /// A cone face's chart window would be read across its APEX, where
    /// every azimuth maps to one point and no branch pin carries: the
    /// walk reached the apex, or the face meets it in a way no single
    /// chart lift closes (twice, from both nappes, or around a ring). A
    /// window guessed there reads
    /// the face's complement without any later check seeing it.
    ApexUnlifted {
        /// The cone face whose window was read.
        face: FaceKey,
    },
    /// Two distinct edges of a sphere face meet at its pole at an angle
    /// that, levered by the shorter one's reach from the axis, is within
    /// ε of a whole turn: they overlap there, meet in a cusp, or the
    /// shorter is too short for any angle to separate them by ε. The
    /// face's azimuth walk has no strict branch to read at the pole
    /// ([`pole_branch`]).
    PoleEdgesCoincide {
        /// The sphere face whose window was read.
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
    ///   tangent germ pair inside the boolean join — a touching
    ///   configuration, the M5 envelope's frontier).
    SectionInvariant {
        /// The face being divided.
        face: FaceKey,
        /// What failed.
        what: &'static str,
    },
    /// A plane's section of a curved face the chord layer has no arm
    /// for: valid input whose lane is not yet built (D2 addendum row 2),
    /// refused typed. The chord is minted in the curved face itself
    /// (`divided` is `wall`), or in a plane face against it (`divided`
    /// is [`geom::SurfaceKind::Plane`]). The chord layer serves a plane
    /// against a cylinder, a sphere, a cone or a torus; every other curved
    /// kind is unarmed.
    SectionUnarmed {
        /// The face being divided.
        face: FaceKey,
        /// Its surface kind.
        divided: geom::SurfaceKind,
        /// The curved kind whose section has no arm.
        wall: geom::SurfaceKind,
    },
    /// A run edge of a curved face whose image on the face's chart is
    /// none of the chart's own curves: its class reading took the
    /// general lane, read in band or definite, or found none written
    /// for the pair ([`geom_brep::PcurveCertifyError::NoGeneralLane`]).
    /// The window walk reads a closed-form azimuth, so a general image
    /// is a lane it does not read yet: a lane choice (D3), refused
    /// lever-only "not yet", never an invariant break or a tolerance.
    RunImageNotYet {
        /// The face being divided.
        face: FaceKey,
        /// Its surface kind.
        kind: geom::SurfaceKind,
    },
    /// A run edge's image refused on its incidence on the divided face's
    /// chart (`geom_brep::IncidenceRead::of`): decided off it, read at a
    /// pose, or undecided. It ends by the one map
    /// (`geom_brep::IncidenceRead::ending`): a definite miss is the
    /// kernel's defect (the join reads a body the kernel built), a read in
    /// band the geometry's lever.
    RunImageIncidence {
        /// The face being divided.
        face: FaceKey,
        /// Its surface kind.
        kind: geom::SurfaceKind,
        /// The image's refusal, verbatim (`Debug`), read for its ending.
        refusal: geom_brep::PcurveCertifyError,
    },
    /// A ring whose island the ring lane does not read: on a curved face
    /// it winds an island, and re-homes a ring, on a cylinder wall's
    /// chart and on a sphere, a cone or a torus by a path's parity; on a
    /// plane, a run on spiric ovals at a scalar that does not certify
    /// (`crate::loop_winding::Unwound::Uncertified`). Valid input
    /// whose lane is not yet built (D2 addendum row 2), refused typed.
    RingIslandUnread {
        /// The face carrying the ring.
        face: FaceKey,
        /// Its surface kind.
        kind: geom::SurfaceKind,
    },
    /// A run edge between two chords on a plane face is a spiric or a
    /// spline, and the join has no row reading whether such an edge lies
    /// in the cut's plane: its midpoint's side says nothing of the rest.
    RunEdgeCarrierUnsupported {
        /// The face whose loop the run is on.
        face: FaceKey,
        /// The edge: its split pieces' root.
        edge: EdgeKey,
    },
    /// Whether a split plane is tangent to a curved face along the
    /// ruling it cuts the face on is in band, at a station of the
    /// must-carry rule over the ruling
    /// ([`geom_brep::MustCarryEscalation::FirstOrder`]).
    ///
    /// No shipped pose reaches it: a cut turned off tangent by an
    /// angle the rule's first order would leave in band refuses at the
    /// reduction first (`wedge_end_doors.rs`,
    /// `a_cut_turned_about_the_ruling_refuses_before_the_chord`).
    TangentChordEscalated {
        /// The curved face the ruling divides.
        face: FaceKey,
        /// The deciding station's diagnostic.
        diag: Indeterminate,
    },
    /// Whether a curved face curves away from a split plane tangent to
    /// it along a ruling, or shares its curvature there, is in band
    /// (`"tangent_second_order"`,
    /// [`geom_brep::MustCarryEscalation::SecondOrder`]): the ruling is
    /// certifiable as neither the intrinsic tangency nor a conventional
    /// image (D4 ¶3).
    TangentChordBendEscalated {
        /// The curved face the ruling divides.
        face: FaceKey,
        /// The sagitta's diagnostic.
        diag: Indeterminate,
    },
    /// A station of the must-carry rule read the tangent ruling a
    /// corner ([`geom_brep::MustCarryRefusal::Refuted`]), against the
    /// section table's tangent verdict: the two readings disagree, so
    /// the split refuses rather than choose a description.
    ///
    /// Unreached like [`Self::TangentChordEscalated`], for the same
    /// reason.
    TangentChordRefuted {
        /// The curved face the ruling divides.
        face: FaceKey,
        /// The two neighbouring stations that disagree, where the ruling
        /// reads smooth at one ([`geom_brep::MustCarryRefusal::Mixed`]);
        /// `None` where every station reads a corner.
        mixed: Option<geom_brep::MixedStations>,
    },
    /// A curved face crossed more than twice could not have its
    /// crossings paired along its section conic, or along each ruling
    /// of a two-ruling section, which is the only pairing that keeps
    /// each chord on an arc inside the face. The case says which way
    /// the pairing failed.
    SectionCrossings {
        /// The curved face whose crossings were being paired.
        face: FaceKey,
        /// Why they could not be.
        case: ConicCrossingsCase,
        /// The band the headings were decided against.
        band: Band,
    },
    /// A section chord's start could not say which way the section runs
    /// into the face: the plane lies within the band of tangent to the
    /// wall there, read as the sine between them times the wall's
    /// [`geom_brep::curvature_lever_arm`] (`chord_arc_leave_section`
    /// Zero, its datum along the section). On a cone that lever is the
    /// distance from the axis, so a cut a few band widths of radius from
    /// the apex reaches it at any angle the band cannot resolve there.
    SectionDepartureGrazes {
        /// The curved face the chord divides.
        face: FaceKey,
        /// The band the departure was decided against.
        band: Band,
    },
}

impl From<EulerOpError> for SplitJoinError {
    fn from(e: EulerOpError) -> Self {
        Self::Euler(e.from_driver())
    }
}

impl From<PointInLoopError> for SplitJoinError {
    fn from(e: PointInLoopError) -> Self {
        Self::RingHoming(e)
    }
}

/// The lever the section join's refusals carry, under a split and a
/// Boolean alike: the geometry.
use geom_core::COINCIDENCE_RECOURSE as JOIN_RECOURSE;

/// What a [`SplitJoinError::OrderEscalated`] could not decide, by its
/// predicate: the join order's frame, or the order of two points in it.
fn order_decision(predicate: Option<&str>) -> &'static str {
    match predicate {
        Some(FRAME_AXIS) => "whether the section plane's normal lies along a coordinate axis",
        Some(FRAME_ARM) => "which direction in the section plane to order points along",
        _ => "the order of two section points",
    }
}

/// The ending of a join order escalation. The frame's two decisions
/// (an axis component, an arm's length in the plane) are no size the
/// user chose, so they end in the join's lever alone (D4 ¶1 (i)); the
/// order of two points compares metred coordinates, a size, and takes
/// the escalation's whole ending.
fn order_ending(diag: &Indeterminate) -> String {
    match diag.predicate {
        Some(FRAME_AXIS | FRAME_ARM) => diag.lever_ending(JOIN_RECOURSE),
        _ => diag.ending(JOIN_RECOURSE),
    }
}

impl core::fmt::Display for SplitJoinError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::OrderEscalated { diag } => write!(
                f,
                "{}",
                diag.undecided(order_decision(diag.predicate), order_ending(diag))
            ),
            // The section's sized decisions (a pose's gap, an apex's
            // offset, a tangent chord's advance) end as geom-brep ends
            // them: the escalation's whole ending.
            Self::Escalated { diag, .. } => write!(
                f,
                "{}",
                diag.undecided(
                    "where a section runs across a face",
                    diag.ending(JOIN_RECOURSE)
                )
            ),
            Self::DegenerateSection { .. } => write!(
                f,
                "a section is degenerate: it bounds zero area. Recourse: {JOIN_RECOURSE}"
            ),
            Self::CurvedSlitTrace { .. } => write!(
                f,
                "an arc lying in the cut, wrapped round at both ends by the material \
                 across it, would leave two coincident arcs that no contact record backs \
                 (`work/cleave/a-curved-slit-fails-tier-3-prime-on-its-two-coincident-arcs.md`). \
                 {}",
                geom_core::NOT_YET_ENDING
            ),
            Self::RingHoming(e) => e.render_as(
                f,
                "re-homing a hole loop",
                "which piece holds a hole loop",
                JOIN_RECOURSE,
            ),
            Self::RingHomingAmbiguous { .. } => write!(
                f,
                "every vertex of a hole loop lies on the boundary of the piece being \
                 divided off, so which piece holds it cannot be decided. Recourse: {JOIN_RECOURSE}"
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
                "{count} section loop(s) close through a single vertex where neither a wrap \
                 edge crosses a planar face nor a closed edge lies in a planar or spherical \
                 face or along another closed edge, and such a loop joined at one site is not \
                 built. {}",
                geom_core::NOT_YET_ENDING
            ),
            Self::SectionLoopUndecided { .. } => write!(
                f,
                "which of a section's two loops bounds the result cannot be read: every \
                 point it is read at lies on a face's boundary or too near the other \
                 part. {}",
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
            Self::LineageRow(refusal) => write!(
                f,
                "{}. {}",
                refusal.refused_by("join"),
                geom_core::KERNEL_DEFECT_ENDING
            ),
            Self::Band(e) => write!(f, "{e}"),
            Self::Euler(e) => write!(f, "an Euler operation refused: {e}"),
            Self::SectionEndEscalated {
                reading: SectionEndReading::OnPlane,
                diag,
                ..
            } => crate::merge_faces::edge_on_plane_refusal(diag).fmt(f),
            Self::SectionEndEscalated {
                reading: SectionEndReading::SquareToAxis,
                diag,
                ..
            } => diag
                .undecided(
                    "whether a section read square to a curved face's axis ends at its vertex",
                    diag.lever_ending(SQUARE_TO_AXIS_LEVER),
                )
                .fmt(f),
            Self::SectionUnresolved {
                miss: SectionPinMiss::Undecided { check, diag, .. },
                ..
            } => {
                let subject = match check {
                    geom_brep::CertCheck::EndpointStart | geom_brep::CertCheck::EndpointEnd => {
                        "whether a section curve's end meets its vertex"
                    }
                    geom_brep::CertCheck::WitnessMidpoint => {
                        "whether a section curve passes the witness it was minted through"
                    }
                    _ => "whether a section curve lies on the surfaces it is cut from",
                };
                let ending = geom_brep::recourse::Unsized::Fit.recourse(
                    geom_brep::recourse::RefusedArm::Undecided(diag),
                    geom_brep::recourse::Reading::Build,
                );
                diag.undecided(subject, format!("{SECTION_UNRESOLVED_CAUSE} {ending}"))
                    .fmt(f)
            }
            Self::SectionUnresolved {
                miss:
                    SectionPinMiss::Past {
                        check,
                        sample,
                        margin,
                    },
                ..
            } => {
                let ending = geom_brep::recourse::Unsized::Fit.recourse(
                    geom_brep::recourse::RefusedArm::SignCertain(Some(*margin)),
                    geom_brep::recourse::Reading::Build,
                );
                write!(
                    f,
                    "a section curve's end misses its vertex: {check} at sample {sample} is \
                     {margin:e}, definitely past the tolerance. {SECTION_WITHIN_CAUSE} {ending}"
                )
            }
            // The join keeps the carrier constructor's subject and
            // offers its own door's levers.
            Self::Section {
                source: geom_brep::SectionError::Carrier(geom::EllipseInvalid::Escalated(diag)),
                ..
            } => write!(
                f,
                "{} is undecided for the section through a curved face: {}. Recourse: \
                 {JOIN_RECOURSE}",
                geom::EllipseInvalid::escalated_subject(diag),
                diag.payload()
            ),
            Self::Section { source, .. } => {
                write!(f, "the section through a curved face refused: {source}")
            }
            Self::OpenConicChord { conic, .. } => write!(
                f,
                "the plane cuts the cone face along {}, an arc the split cannot yet \
                 divide a face along. Recourse: tilt the plane so it cuts the cone all the \
                 way round",
                conic.named()
            ),
            Self::ApexUnlifted { .. } => write!(
                f,
                "a cone face's azimuth window would be read across its apex, where the chart \
                 has no azimuth: the walk reached the apex, or the face meets it in a way no \
                 single chart lift closes (twice, from both nappes, or around a ring). \
                 Recourse: divide the cone face so each piece meets its apex at most once, \
                 from one nappe"
            ),
            Self::PoleEdgesCoincide { .. } => write!(
                f,
                "two edges of a sphere face meet at its pole within tolerance of one direction \
                 over the shorter one's reach: they overlap, meet in a cusp, or the shorter is \
                 too short to tell apart, so the face's azimuth window has no branch there. \
                 Recourse: merge the overlapping edges, or move one off the other at the pole"
            ),
            Self::SectionInvariant { face, what } => {
                write!(f, "curved-section invariant at face {face:?}: {what}")
            }
            Self::SectionUnarmed { divided, wall, .. } => {
                let wall = crate::boolean::kind_word(*wall);
                write!(
                    f,
                    "the join cannot yet trace where a plane meets a {wall} face, so it cannot \
                     divide the {} face there. {}",
                    crate::boolean::kind_word(*divided),
                    crate::boolean::meeting_recourse(wall),
                )
            }
            Self::RunImageIncidence { kind, refusal, .. } => {
                use geom_brep::IncidenceRead as R;
                let kind = crate::boolean::kind_word(*kind);
                // Worded by what was read and ended by the one map; a
                // refusal that carried no read is no incidence refusal.
                let Some(read) = R::of(refusal) else {
                    return write!(f, "{refusal}");
                };
                let what = match read {
                    _ if read.is_definite() && !matches!(read, R::Off(_)) => {
                        "could not be read against it"
                    }
                    R::Off(_) => "does not lie on it",
                    R::AtPose(_) => "sits, within the band, where it holds no curve of its kind",
                    R::Undecided(_) => "is not shown on it",
                };
                write!(
                    f,
                    "an edge of a {kind} face the join divides {what}. {}",
                    read.ending(geom_brep::recourse::Reading::Build)
                )
            }
            Self::RunImageNotYet { kind, .. } => write!(
                f,
                "an edge of a {} face sits on the face as none of its own curves, and the \
                 join does not yet read such an edge's reach around the face. {}",
                crate::boolean::kind_word(*kind),
                geom_core::NOT_YET_ENDING
            ),
            Self::RingIslandUnread { kind, .. } => write!(
                f,
                "a cut passes through a {kind:?} face without reaching its boundary, leaving \
                 a ring whose island the join does not read there. Recourse: move the cut so \
                 it crosses the face's edge, or divide the face there first"
            ),
            Self::RunEdgeCarrierUnsupported { .. } => write!(
                f,
                "a cut joins two points of a face's boundary across a spiric or spline \
                 (NURBS) edge, and the join cannot yet tell whether such an edge lies in the \
                 cut. Recourse: {}",
                crate::boolean::CONIC_EDGES_RECOURSE
            ),
            Self::TangentChordEscalated { diag, .. } => write!(
                f,
                "{}",
                diag.undecided(
                    "whether a cut is tangent to a curved face along a ruling",
                    diag.ending(JOIN_RECOURSE)
                )
            ),
            Self::TangentChordBendEscalated { diag, .. } => write!(
                f,
                "{}",
                diag.undecided(
                    "whether a curved face the cut touches along a ruling curves away from the \
                     cut there or shares its curvature",
                    diag.ending(JOIN_RECOURSE)
                )
            ),
            Self::TangentChordRefuted {
                mixed: Some(stations),
                ..
            } => write!(
                f,
                "along a cut's tangent ruling of a curved face, {stations}. Recourse: \
                 {JOIN_RECOURSE}"
            ),
            Self::TangentChordRefuted { mixed: None, .. } => write!(
                f,
                "a cut reads tangent to a curved face along a ruling but a corner at every \
                 station along it, so what that ruling is cannot be said. Recourse: \
                 {JOIN_RECOURSE}"
            ),
            Self::SectionCrossings { case, band, .. } => write!(
                f,
                "a curved face's crossings cannot be paired along the section: {case} \
                 ('split_join_conic_heading', band ({:e}, {:e})). Recourse: {JOIN_RECOURSE}",
                band.zero(),
                band.escalate(),
            ),
            Self::SectionDepartureGrazes { band, .. } => write!(
                f,
                "a section chord's start reads the plane within the band of tangent to \
                 the wall: its angle to the wall times the wall's radius of curvature \
                 ('chord_arc_leave_section', band ({:e}, {:e})). Recourse: cut where the \
                 wall curves less (on a cone, farther from its apex), or tighten the \
                 tolerance",
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

/// Where the section table reads a wall's pose, and how far `face`
/// reaches from there, measured from the base vertex `at`'s point: on a
/// cone the [`geom_brep::Reach::Face`] the cone lane reads
/// (`splitting::rules::cone_face_reach`), and on a cylinder the
/// [`WallReach`] [`agreed_section`] reads: the face's axial range from
/// the vertex ([`face_axial_range`], the curved edges' bulge included)
/// and its distance from it, each edge read over its span
/// ([`face_extent`]) and round its whole carrier
/// (`face_reach_round_from`). A sphere's lane reads neither. `within`
/// is a window along a cone's axis the consumed region also lies in:
/// the partner wall face's, where `face` is the plane the boolean cuts
/// it with.
fn section_reach<T: Decide>(
    body: &Body<T>,
    at: VertexKey,
    face: FaceKey,
    (wall, within): (&geom::Surface<T>, Option<(T, T)>),
    band: Band,
) -> Result<SectionReach<T>, SplitJoinError> {
    let p = body.resolve_vertex_point(at, Proven);
    Ok(match wall {
        geom::Surface::Cylinder { axis, .. } => {
            let extent = face_extent(body, at, face).map_err(unbounded)?;
            let (below, above) = face_axial_range(body, face, p, *axis).map_err(unbounded)?;
            let round =
                crate::splitting::rules::face_reach_round_from(body, face, p).map_err(unbounded)?;
            SectionReach::Cylinder(WallReach {
                at: p,
                below,
                above,
                span: extent,
                round,
            })
        }
        geom::Surface::Cone { .. } => SectionReach::Cone(
            crate::splitting::rules::cone_face_reach(body, face, p, (wall, within), band)
                .map_err(unbounded)?,
        ),
        geom::Surface::Torus { .. } => SectionReach::Torus(
            crate::splitting::rules::face_reach_from(body, face, p).map_err(unbounded)?,
        ),
        _ => SectionReach::Unmeasured,
    })
}

/// [`face_extent`]'s refusal as the join's typed frontier: a face with
/// no outer boundary has no extent to meter a section across.
fn unbounded(e: crate::splitting::rules::UnboundedFace) -> SplitJoinError {
    SplitJoinError::SectionInvariant {
        face: e.face,
        what: "the face's outer loop is a lone vertex, so it has no extent to meter the \
               section across",
    }
}

/// The loop and face of `he`, a half the join reads out of its
/// working body: a miss of the half or of its loop refuses typed,
/// naming the key that went stale.
pub(crate) fn he_loop<T: Real>(
    body: &Body<T>,
    he: HalfEdgeKey,
) -> Result<(LoopKey, FaceKey), SplitJoinError> {
    let l = body
        .get_half_edge(he)
        .ok_or_else(|| corrupt_he(he))?
        .parent_loop;
    Ok((l, body.get_loop(l).ok_or_else(|| corrupt_loop(l))?.face))
}

/// The face of `he` ([`he_loop`]).
pub(crate) fn he_face<T: Real>(body: &Body<T>, he: HalfEdgeKey) -> Result<FaceKey, SplitJoinError> {
    Ok(he_loop(body, he)?.1)
}

/// The corruption refusal naming the edge the join was reading.
pub(crate) fn corrupt_edge(edge: EdgeKey) -> SplitJoinError {
    SplitJoinError::Corrupt {
        entity: EntityId::Edge(edge),
    }
}

/// Chord-mef fragment rows: `(new face, divided-from face)` in mint
/// order (naming emission, M4 PR 3), checked as each is added
/// ([`crate::lineage::FaceDivisions`]).
pub(crate) type FragmentRows = crate::lineage::FaceDivisions;

/// The point of `v`, a vertex the join read out of its working body: a
/// miss of the vertex or its point panics naming it (D2 row 4).
#[track_caller]
pub(crate) fn vertex_point<T: Decide>(body: &Body<T>, v: VertexKey) -> Point3<T> {
    body.point_of(v, crate::live::proven(&body.vertices, v, EntityId::Vertex))
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
    /// Naming emission (M4 PR 3): every face the chord mefs minted, and
    /// the face [`Self::join_lone_ring`] promotes a ring to, paired with
    /// the face it was divided from, in mint order —
    /// `(new face, divided-from face)` at CALL-TIME keys. Rows are
    /// historical (a recorded face may later die — slivers killed by
    /// `cut`, discarded material at finish); consumers filter to the
    /// entities alive in the body they hold. This is mint-time wiring
    /// knowledge, recorded so the naming layer never reconstructs
    /// parentage by inspection (NAMING-DESIGN N4: no post-hoc scans).
    fragments: FragmentRows,
    /// Every face the join killed, paired with the face that holds its
    /// region now, at call-time keys: a sliver [`Self::cut_core`]'s kef
    /// merged into its neighbour, and the hole [`Self::join_lone_ring`]
    /// made a ring of the face it was divided from. With
    /// [`Self::fragments`] it states every face the join made and
    /// unmade, so a reader never takes a killed fragment for one the
    /// op lost. Checked where each row is added: the killed face dies
    /// with its row and its holder is live, so the list runs one way.
    absorbed: crate::lineage::Substitutions<FaceKey>,
    /// The run band (ring re-homing containment).
    band: Band,
    /// Pierce rings a division left unplaced: every vertex on the run,
    /// every edge a null edge (a strut at a pinch on a plane). Each stays
    /// in the face it was in until a join connects it to a ring that is
    /// placed, and it moves to that ring's face.
    pending: SecondaryMap<LoopKey, ()>,
    /// The 2-loop null faces [`Self::cut_core`] completed, which no later
    /// kef takes as a side.
    completed: SecondaryMap<FaceKey, ()>,
}

impl ChordJoiner {
    /// A fresh core.
    pub(crate) fn new(band: Band) -> Self {
        Self {
            slivers: SecondaryMap::new(),
            fragments: FragmentRows::new(crate::lineage::LineageRows::FaceDivisions),
            absorbed: crate::lineage::Substitutions::new(
                crate::lineage::LineageRows::Substitutions,
            ),
            band,
            pending: SecondaryMap::new(),
            completed: SecondaryMap::new(),
        }
    }

    /// A pierce ring still unplaced, if any: asked once the sweep is
    /// quiescent, when every join that could have placed it has run.
    /// A pending key whose loop has died (a `mekr` merged it away) names
    /// nothing, and slotmap versions keep it from naming a new loop.
    pub(crate) fn unplaced_ring<T: Real>(&self, body: &Body<T>) -> Option<LoopKey> {
        self.pending
            .keys()
            .find(|&ring| body.get_loop(ring).is_some())
    }

    /// The quiescence check both sweeps end with: a ring still pending
    /// was never reached by a join, so nothing placed it.
    pub(crate) fn finish<T: Real>(&self, body: &Body<T>) -> Result<(), SplitJoinError> {
        match self.unplaced_ring(body) {
            Some(ring) => Err(SplitJoinError::RingHomingAmbiguous { ring }),
            None => Ok(()),
        }
    }

    /// Places a pending ring before `h1` and `h2` are joined: the join
    /// connects it to its own polygon, so it belongs in the face of
    /// the ring it meets. Refuses when that ring is pending in another
    /// face, or when the two halves share a pending loop (a polygon
    /// completing inside a ring nothing has placed).
    ///
    /// Every lane runs it on a match's halves before anything reads
    /// which loop or face they are in: the role order, the
    /// [`JoinPlan`], the segment's curve.
    pub(crate) fn place_pending<T: Decide>(
        &mut self,
        body: &mut Body<T>,
        (h1, h2): (HalfEdgeKey, HalfEdgeKey),
    ) -> Result<(), SplitJoinError> {
        let ((l1, f1), (l2, f2)) = (he_loop(body, h1)?, he_loop(body, h2)?);
        let (p1, p2) = (self.pending.contains_key(l1), self.pending.contains_key(l2));
        match (p1, p2) {
            // Halves on two faces with no pending ring between them are
            // the plan's to read ([`JoinPlan::of`]).
            (false, false) => {}
            (true, true) if l1 == l2 || f1 != f2 => {
                return Err(SplitJoinError::RingHomingAmbiguous { ring: l1 });
            }
            // Two pending rings of one face merge into a ring still
            // pending: the join's mekr keeps one of the two loops, the
            // second chord walls a sliver off that ring's face, and the
            // dead loop's key stays in `pending` but resolves to nothing
            // ([`Self::unplaced_ring`]).
            (true, true) => {}
            (true, false) | (false, true) => {
                let (ring, to) = if p1 { (l1, f2) } else { (l2, f1) };
                if f1 != f2 {
                    body.ring_move(ring, to)?;
                }
                self.pending.remove(ring);
            }
        }
        Ok(())
    }

    /// Consumes the recorded `(new face, divided-from face)` rows
    /// (naming emission; see the field docs).
    pub(crate) fn take_fragments(&mut self) -> FragmentRows {
        core::mem::replace(
            &mut self.fragments,
            FragmentRows::new(crate::lineage::LineageRows::FaceDivisions),
        )
    }

    /// Consumes the recorded `(killed face, face holding its region)`
    /// rows (see the field docs).
    pub(crate) fn take_absorbed(&mut self) -> crate::lineage::Substitutions<FaceKey> {
        core::mem::replace(
            &mut self.absorbed,
            crate::lineage::Substitutions::new(crate::lineage::LineageRows::Substitutions),
        )
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
    /// The loop of a two-loop section the chord rides, where the caller
    /// decided it: a boolean germ's branch. `None` in the split lane,
    /// whose chord reads it at its sites ([`section_loop`]).
    pub(crate) branch: Option<crate::boolean::Branch>,
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
        /// The partner wall face's window along a cone's axis
        /// (`splitting::rules::cone_axial_window`), which the section
        /// this face shares with it lies in; `None` off a cone.
        window: Option<(T, T)>,
        /// The aux wall key in THIS body (minted once, caller-cached).
        partner_key: &'a mut Option<SurfaceKey>,
        /// The germ's branch: the loop of a two-loop section the chord
        /// rides.
        branch: crate::boolean::Branch,
    },
    /// The boolean's chord on a germ pair whose section the operation's
    /// section table carries (`boolean::section_table`): an arc of the
    /// component's carrier between the two sites, on both sides
    /// ([`carried_chord_spec`]).
    Carried(&'a mut CarriedCtx<T>),
    /// A section segment that is an edge of this solid: this side's
    /// chord is a copy of that edge ([`along_edge_spec`]) and reads no
    /// section, since the germ's face pair there may be two faces on
    /// one carrier, or meet in no conic. Where the edge is this solid's
    /// alone, the other solid's chord may read one (the boolean's
    /// edge-plane lane cuts its face by the edge's plane).
    AlongEdge,
}

/// **The component a carried chord is an arc of** ([`JoinLane::Carried`]):
/// the component as the section table seated it, the partner face's
/// surface, the copy of it this body holds once one is minted, and the
/// length a direction's angle to the component is levered by.
pub(crate) struct CarriedCtx<T: Real> {
    /// The component, seated: a closed one opened at its seat.
    pub(crate) component: geom_brep::CarriedComponent<T>,
    /// The other operand's germ face's surface.
    pub(crate) partner: geom::Surface<T>,
    /// The copy of `partner` in this body, once minted.
    pub(crate) partner_key: Option<SurfaceKey>,
    /// The section's own scale ([`geom_brep::CarriedSection::lever`]).
    pub(crate) lever: T,
}

/// The section conic's frame — the curve both chord lanes take an arc
/// of.
#[derive(Clone)]
pub(crate) struct SectionConic<T: Real> {
    /// The conic's centre.
    center: Point3<T>,
    /// The section plane's normal — the conic's own axis, about which
    /// its parameter runs ccw.
    normal: Vec3<T>,
    /// The seam direction, a principal direction: `θ = 0` is at
    /// `center + seam·sa`.
    seam: Vec3<T>,
    /// The semi-axis along `seam` (the radius, for a circle).
    sa: T,
    /// The semi-axis along `normal × seam`.
    sb: T,
    /// The carrier itself.
    carrier: geom::Curve3<T>,
    /// Whether the table stood this circle in for the plane's own
    /// section, square to the wall's axis by a Zero of its trilean
    /// (`pc_rim_alignment`, `pn_axis_normal`, `pt_axis_normal`): the
    /// circle lies in the plane through `center` square to `normal`,
    /// which is the section plane only to within that Zero.
    square: bool,
}

impl<T: Real> SectionConic<T> {
    /// The eccentric anomaly of a point on the conic: `θ` with
    /// `p = center + sa·cos θ·seam + sb·sin θ·(normal × seam)`.
    pub(crate) fn param(&self, p: Point3<T>) -> T {
        let d = p - self.center;
        (d.dot(self.normal.cross(self.seam)) / self.sb).atan2(d.dot(self.seam) / self.sa)
    }

    /// The conic's derivative in `θ` (metres per radian).
    pub(crate) fn tangent(&self, theta: T) -> Vec3<T> {
        self.normal.cross(self.seam) * (self.sb * theta.cos()) - self.seam * (self.sa * theta.sin())
    }

    /// **The same conic seamed at the principal vertex nearest `theta`**,
    /// in this frame's eccentric anomaly ([`geom::Curve3::seamed_near`]).
    /// A circle is returned as it is: its parameter resolves to its
    /// radius everywhere.
    fn seamed_near(self, theta: T) -> Self {
        match self.carrier.seamed_near(theta).and_then(section_conic) {
            Some(seamed) => Self {
                square: self.square,
                ..seamed
            },
            None => self,
        }
    }
}

/// **A spiric oval of a plane × torus section**, as a chord takes an arc
/// of it: a `Curve3::Spiric`, whose parameter is the torus's own minor
/// angle `v`. The oval turns once in `v`, at a speed bounded away from
/// zero (the variant's docs), so its points and its parameters are one
/// to one, as a conic's are in its eccentric anomaly.
#[derive(Clone)]
pub(crate) struct SectionOval<T: Real> {
    /// The carrier.
    carrier: geom::Curve3<T>,
    /// The oval's own extent: the larger of its half-widths across the
    /// plane, `r` along the axis and `(f_max − f_min)/2` along the trace
    /// (`geom::spiric_f_range`). At `offset = 0` it is the meridian
    /// circle's radius, as a conic's is its semi-major axis.
    extent: T,
}

/// The closed curve of a section a chord takes an arc of: a conic, or a
/// spiric oval. [`arc_leaving`] and [`oriented_arc`] read it through its
/// parameter, its tangent and its lever, the same rule for each.
#[derive(Clone)]
pub(crate) enum SectionCurve<T: Real> {
    /// A circle or an ellipse.
    Conic(SectionConic<T>),
    /// A spiric oval.
    Oval(SectionOval<T>),
}

impl<T: Decide> SectionCurve<T> {
    /// The parameter of a point on the curve: a conic's eccentric
    /// anomaly, an oval's minor angle `atan2(h, ρ − R)`.
    pub(crate) fn param(&self, p: Point3<T>) -> T {
        match self {
            Self::Conic(c) => c.param(p),
            Self::Oval(o) => match o.carrier.param_near(p, T::zero()) {
                Some(v) => v,
                None => unreachable!(
                    "{:?} is the spiric an oval is built of, and a spiric has a closed-form \
                     parameter",
                    o.carrier
                ),
            },
        }
    }

    /// The curve's derivative in its parameter (metres per radian).
    pub(crate) fn tangent(&self, t: T) -> Vec3<T> {
        match self {
            Self::Conic(c) => c.tangent(t),
            Self::Oval(o) => o.carrier.deriv(t),
        }
    }
}

impl<T: Real> SectionCurve<T> {
    /// The length a departure's cosine is levered by: a conic's larger
    /// semi-axis, whichever its seam lies along, an oval's own extent.
    fn lever(&self) -> T {
        match self {
            Self::Conic(c) => geom::Curve3::semi_axes_by_size(c.sa, c.sb).0,
            Self::Oval(o) => o.extent,
        }
    }

    /// The carrier itself.
    fn carrier(&self) -> &geom::Curve3<T> {
        match self {
            Self::Conic(c) => &c.carrier,
            Self::Oval(o) => &o.carrier,
        }
    }

    /// A direction in the section plane, for a minted plane's `u_ref`: a
    /// conic's seam direction (a principal one), an oval's torus axis.
    fn in_plane(&self) -> Vec3<T> {
        match self {
            Self::Conic(c) => c.seam,
            Self::Oval(o) => match o.carrier {
                geom::Curve3::Spiric { axis, .. } => axis,
                _ => unreachable!(
                    "{:?} is the spiric an oval is built of, and a spiric's axis lies in its \
                     plane",
                    o.carrier
                ),
            },
        }
    }
}

/// What the C5 table made of `plane × wall`: the curve a chord rides,
/// and what the split pairs a curved face's crossings along.
///
/// `Straight` and `Tangent` are handed BACK rather than decided here:
/// the two chord lanes mean different things by them — the split lane
/// mints a tangent chord along the ruling, the boolean join refuses a
/// tangent germ pair as a touching frontier — and that difference is
/// the whole of what the two lanes do not share.
pub(crate) enum SectionCase<T: Real> {
    /// A conic to select an arc of.
    Conic(SectionConic<T>),
    /// Two rulings, each a `Line`: a cone's pair through its apex or a
    /// cylinder's parallel pair. The split pairs a face's crossings
    /// along each, and a chord between two of them is straight, so the
    /// caller mints no spec.
    Straight([geom::Curve3<T>; 2]),
    /// The tangent locus, as the table constructed it.
    Tangent(geom::Curve3<T>),
    /// Two closed curves, each a loop of its own: a torus's two meridian
    /// circles, its two concentric ones, or its two spiric ovals. A
    /// chord is an arc of the loop its sites lie on ([`section_loop`]).
    Loops {
        /// The loop on the split's positive side, then its negative one.
        loops: [SectionCurve<T>; 2],
        /// The margin that tells them apart: the boolean join's split
        /// for the same section (`boolean::join`'s `pt_germ_frame`).
        split: LoopSplit<T>,
    },
}

/// **The loop of a two-loop section a chord rides**: the branch the
/// caller decided (a boolean germ's, decided where the germ was minted),
/// or, in the split lane, the loop its sites lie on
/// (**`chord_section_loop`**: the split's margin at each site, in
/// metres). The split lane reads which side of the split a site is on,
/// not whether the site lies on that side's loop: the sites are
/// crossings placed on the section, and certification holds the arc
/// minted between them to both surfaces. On the meridian circles and the spiric ovals
/// the margin is `√(ρ² − d²) ≥ √((R − r)² − d²)` at every point of the
/// section, which the table decided positive (`d = 0` on the circles),
/// and on the concentric circles `√(r² − h²)`, likewise decided, so
/// a site whose margin falls in the band is no site of the section, an
/// invariant; an undecided one escalates. Sites on two sides are an
/// invariant too.
fn section_loop<T: Decide>(
    face: FaceKey,
    [positive, negative]: [SectionCurve<T>; 2],
    (split, branch): (LoopSplit<T>, Option<crate::boolean::Branch>),
    sites: [Point3<T>; 2],
    band: Band,
) -> Result<SectionCurve<T>, SplitJoinError> {
    // A boolean germ along an edge rides the edge's own curve and
    // carries no branch of the face pair's section: its chord reads the
    // loop at its sites, as the split lane's does.
    match branch {
        Some(crate::boolean::Branch::Positive) => return Ok(positive),
        Some(crate::boolean::Branch::Negative) => return Ok(negative),
        Some(crate::boolean::Branch::Only) | None => {}
        Some(crate::boolean::Branch::Component(_)) => {
            return Err(SplitJoinError::SectionInvariant {
                face,
                what: "a germ riding a carried section's component reached a two-loop frame",
            });
        }
    }
    let side = |p| match decide("chord_section_loop", Margin::of(split.margin(p)), band) {
        Ok(Sign::Zero) => Err(SplitJoinError::SectionInvariant {
            face,
            what: "a chord's site lies on neither loop of its section (its loop margin is in \
                   the band)",
        }),
        Ok(sign) => Ok(sign),
        Err(diag) => Err(SplitJoinError::Escalated { face, diag }),
    };
    match (side(sites[0])?, side(sites[1])?) {
        (Sign::Positive, Sign::Positive) => Ok(positive),
        (Sign::Negative, Sign::Negative) => Ok(negative),
        _ => Err(SplitJoinError::SectionInvariant {
            face,
            what: "a chord's two sites lie on different loops of its section",
        }),
    }
}

/// What [`section_reach`] measured of a face, by its wall's kind.
#[derive(Clone, Debug)]
pub(crate) enum SectionReach<T: Real> {
    /// A cone wall's face, as the plane×cone arm reads it.
    Cone(geom_brep::Reach<T>),
    /// A cylinder wall's face, as [`agreed_section`] reads it.
    Cylinder(WallReach<T>),
    /// A torus wall's face, or a plane face against a torus: the
    /// divided face's farthest distance from the chord's site, which the
    /// plane×torus arm levers its axis tilt by.
    Torus(T),
    /// Any other wall: the sphere lane reads no reach.
    Unmeasured,
}

/// **A plane×cylinder wall face's reach from the one point it is read
/// from**: how far it reaches either way along the cylinder's axis from
/// `at`, and its farthest distance from `at` measured two ways — each
/// edge read over the span it holds (`splitting::rules::face_reach_from`)
/// and round its whole carrier (`face_reach_round_from`).
#[derive(Clone, Copy, Debug)]
pub(crate) struct WallReach<T: geom_core::Real> {
    /// The point both measures are taken from.
    pub(crate) at: Point3<T>,
    /// How far the face reaches from `at` against the axis.
    pub(crate) below: T,
    /// How far the face reaches from `at` along the axis.
    pub(crate) above: T,
    /// The farthest distance, each edge over its span.
    pub(crate) span: T,
    /// The farthest distance, each edge round its whole carrier.
    pub(crate) round: T,
}

/// **A plane×cylinder section, served only where a wall face's two
/// measures of its reach across agree** ([`WallReach`]). Both the chord
/// lane ([`section_case`]) and the germ frame
/// (`boolean::join::pair_section_frame_at`) read the section on the
/// declared-tangency path, which a shorter lever must not widen: an
/// escalation under either reading escalates, and the two readings
/// parting the conic lane from the rulings' lane are two sound bounds on
/// the axis' tilt over the face, straddling the band
/// (`pc_axis_plane_parallel_straddle`, [`geom_core::k_stats::straddle`]).
/// Nothing else parts them: both are read at the one point `at`, so the
/// rulings' gap datum is one number, whose two swings cannot put it on
/// two sides of the band, and the conic lane's rim and tilted ellipse
/// read no reach.
pub(crate) fn agreed_section<T: Decide>(
    plane: &impl geom_brep::SectionPlane<T>,
    wall: &geom::Surface<T>,
    reach: WallReach<T>,
    band: Band,
) -> Result<geom_brep::PlaneCylinderSection<T>, geom_brep::SectionError> {
    use geom_brep::PlaneCylinderSection as S;
    let read = |across| {
        let face = geom_brep::Reach::Face {
            at: reach.at,
            below: reach.below,
            above: reach.above,
            across,
        };
        geom_brep::plane_cylinder_section_read(plane, wall, &face, band)
    };
    let conic = |s: &S<T>| matches!(s, S::Rim(_) | S::TiltedEllipse(_));
    match (read(reach.span), read(reach.round)) {
        (Err(e), _) | (Ok(_), Err(e)) => Err(e),
        (Ok((s, _)), Ok((r, _))) if core::mem::discriminant(&s) == core::mem::discriminant(&r) => {
            Ok(s)
        }
        (Ok((s, span)), Ok((r, round))) if conic(&s) != conic(&r) => {
            let (lower, upper) = if span.sign < round.sign {
                (span, round)
            } else {
                (round, span)
            };
            Err(geom_brep::SectionError::Escalated(
                geom_core::k_stats::straddle("pc_axis_plane_parallel_straddle", lower, upper, band),
            ))
        }
        (Ok((s, span)), Ok((r, round))) => unreachable!(
            "one plane and wall served {s:?} over the span reach and {r:?} round it, both in one \
             lane (`pc_axis_plane_parallel` {:?} on {:e} and {:?} on {:e}): both reaches are read \
             at {:?}, so the rulings' gap datum is one number and only its swing differs, which \
             cannot place it on two sides of the band, and `pc_rim_alignment` and the ellipse \
             constructor, which part the conic lane, read no reach",
            span.sign, span.margin, round.sign, round.margin, reach.at
        ),
    }
}

/// The section of the surface PAIR `(s1, s2)` under THE C5 table, as
/// the conic a chord takes an arc of — **one implementation for both
/// chord lanes** — this classification was written twice in this file,
/// once per lane, differing only in the wording of its refusals and in
/// what it did with the tangent arm.
///
/// **Pair-general, not plane-first.** The arms wired today are
/// plane×cylinder, plane×cone, plane×sphere and plane×torus (its
/// meridian and concentric circles), and either order is accepted: the
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
/// `plane_cylinder_section`; the cone lane is `plane_cone_section`; the
/// torus lane is `plane_torus_section`: its meridian circles, its
/// concentric circles and its spiric ovals.
fn section_case<T: Decide>(
    face: FaceKey,
    band: Band,
    s1: &geom::Surface<T>,
    s2: &geom::Surface<T>,
    reach: SectionReach<T>,
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
                 plane×cylinder, plane×cone, plane×sphere and plane×torus, and a curved×curved pair has no conic \
                 to take an arc of; refused typed rather than defaulted to a straight chord. A \
                 sphere pair never arrives here: its section lies in the pair's radical \
                 plane, which the boolean's join hands each side as a plane×sphere pair",
            ));
        }
    };
    plane_wall_case(face, band, plane_s, wall, reach)
}

/// [`section_case`] past its pair normalization: the section of `wall`
/// by `plane`, of which the table reads a point and the normal and
/// never a chart ([`geom_brep::SectionPlane`]).
fn plane_wall_case<T: Decide>(
    face: FaceKey,
    band: Band,
    plane_s: &impl geom_brep::SectionPlane<T>,
    wall: &geom::Surface<T>,
    reach: SectionReach<T>,
) -> Result<SectionCase<T>, SplitJoinError> {
    let invariant = |what: &'static str| SplitJoinError::SectionInvariant { face, what };
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
            seam: u_ref,
            sa: radius,
            sb: radius,
            carrier: circle,
            square: false,
        }));
    }
    let conic = |carrier: geom::Curve3<T>, square: bool| {
        section_conic(carrier)
            .map(|c| SectionCase::Conic(SectionConic { square, ..c }))
            .ok_or_else(|| {
                invariant(
                    "a conic classification carried a carrier that is neither ellipse nor circle",
                )
            })
    };
    // The torus (C5's plane×torus arm): the meridian circles, the
    // concentric circles and the spiric ovals are two loops, a chord an
    // arc of the one its sites lie on,
    // told apart by the boolean join's own split for the section
    // ([`LoopSplit::plane_torus`]). The lever is the divided face's reach
    // (a torus face, or the plane face against it), which bounds where
    // that side's chord is consumed. It enters only the axis tilt's fork:
    // each served row decides its circles' own displacement off the cut
    // beside its position datum, at any lever.
    if let geom::Surface::Torus {
        center,
        axis,
        major_radius,
        ..
    } = *wall
    {
        let SectionReach::Torus(reach) = reach else {
            return Err(invariant(
                "a torus wall's chord reached the section table with no reach measured of the \
                 divided face",
            ));
        };
        let Some((_, normal)) = plane_s.site() else {
            return Err(invariant("the pair normalization put no plane first"));
        };
        let section = geom_brep::plane_torus_section(plane_s, wall, reach, band).map_err(table)?;
        let carried = || {
            invariant(
                "a plane×torus classification carried a carrier that is not the circle or the \
                 spiric its row names",
            )
        };
        let loops = |l1, l2| {
            Ok(SectionCase::Loops {
                loops: [l1, l2],
                split: LoopSplit::plane_torus(&section, (center, axis, major_radius), normal)
                    .ok_or_else(carried)?,
            })
        };
        // The concentric circles are stood in for a plane the table read
        // square to the axis, as a cylinder's rim is.
        let circle = |c: &geom::Curve3<T>, square: bool| {
            section_conic(c.clone())
                .map(|c| SectionCurve::Conic(SectionConic { square, ..c }))
                .ok_or_else(carried)
        };
        return match &section {
            geom_brep::PlaneTorusSection::MeridianCircles { c1, c2 } => {
                loops(circle(c1, false)?, circle(c2, false)?)
            }
            geom_brep::PlaneTorusSection::ConcentricCircles { c1, c2 } => {
                loops(circle(c1, true)?, circle(c2, true)?)
            }
            geom_brep::PlaneTorusSection::SpiricOvals { s1, s2 } => {
                let oval = |s: &geom::Curve3<T>| match *s {
                    geom::Curve3::Spiric {
                        major_radius,
                        minor_radius,
                        offset,
                        ..
                    } => {
                        let (f_min, f_max) =
                            geom::spiric_f_range(major_radius, minor_radius, offset);
                        Ok(SectionCurve::Oval(SectionOval {
                            carrier: s.clone(),
                            extent: minor_radius.max((f_max - f_min) * T::from_f64(0.5)),
                        }))
                    }
                    _ => Err(carried()),
                };
                loops(oval(s1)?, oval(s2)?)
            }
            // C7: the tangent locus is the tube's top or bottom circle,
            // classification data — a touching configuration.
            geom_brep::PlaneTorusSection::TangentCircle(_) => Err(invariant(
                "tangent plane×torus germ pair under a minted chord — a touching configuration, \
                 the typed frontier of the supported envelope",
            )),
            geom_brep::PlaneTorusSection::Empty => Err(invariant(
                "empty plane×torus classification under a minted chord",
            )),
        };
    }
    // The cone (C5's plane×cone arm): the tilted ellipse and the
    // axis-normal circle are conics to select an arc of; the apex lane's
    // generator pair is the ruling case, its tangent generator the
    // tangent one — the cylinder's parallel-axis lane, one kind over.
    // The reach names the wall it measured, and each arm refuses a wall
    // of another kind as its own wrong lane.
    let reach = match reach {
        SectionReach::Cylinder(reach) => reach,
        SectionReach::Unmeasured | SectionReach::Torus(_) => {
            return Err(invariant(
                "a cone or cylinder wall's chord reached the section table with no reach \
                 measured of its face",
            ));
        }
        SectionReach::Cone(cone_reach) => {
            return match geom_brep::plane_cone_section(plane_s, wall, &cone_reach, band)
                .map_err(table)?
            {
                geom_brep::PlaneConeSection::TiltedEllipse(c) => conic(c, false),
                geom_brep::PlaneConeSection::AxisNormalCircle(c) => conic(c, true),
                geom_brep::PlaneConeSection::ApexLinePair { l1, l2 } => {
                    Ok(SectionCase::Straight([l1, l2]))
                }
                geom_brep::PlaneConeSection::ApexTangentLine(line) => {
                    Ok(SectionCase::Tangent(line))
                }
                geom_brep::PlaneConeSection::ApexPoint(_) => Err(invariant(
                    "apex-point plane×cone classification under a minted chord — the plane \
                 touches the cone at its apex alone",
                )),
                geom_brep::PlaneConeSection::OpenConic(section) => {
                    Err(SplitJoinError::OpenConicChord {
                        face,
                        conic: section.conic(),
                    })
                }
            };
        }
    };
    let sec = agreed_section(plane_s, wall, reach, band).map_err(table)?;
    match sec {
        geom_brep::PlaneCylinderSection::TiltedEllipse(c) => conic(c, false),
        geom_brep::PlaneCylinderSection::Rim(c) => conic(c, true),
        geom_brep::PlaneCylinderSection::ParallelLines { l1, l2 } => {
            Ok(SectionCase::Straight([l1, l2]))
        }
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
pub(crate) fn section_conic<T: Real>(carrier: geom::Curve3<T>) -> Option<SectionConic<T>> {
    let (center, normal, seam, sa, sb) = match carrier {
        geom::Curve3::Ellipse {
            center,
            axis,
            semi_u,
            semi_v,
            u_ref,
        } => (center, axis, u_ref, semi_u, semi_v),
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
        seam,
        sa,
        sb,
        carrier,
        square: false,
    })
}

/// Whether an edge from `start` to `end` on `curve` is a lone site's
/// placeholder: a self-loop on a scaffold carrier, which `mef` certifies
/// at a lone site as `EdgeCurveSpec::self_loop_circle_at` and
/// [`chord_spec`] leaves on a self-loop chord in the plane × plane and
/// along-edge lanes, where no conic is sectioned (module docs, "The
/// section-chord geometry"; a curved face's self-loop and the boolean's
/// planar pierce are the whole conic). Its circle is arbitrary, so it
/// bounds nothing.
///
/// Nothing in the edge marks a placeholder apart from an honest
/// whole-turn scaffold, so this reading holds only in a body whose
/// self-loop scaffolds are all the operation's own lone sites: the
/// split's scratch body, made from an operand at rest, where tier 3
/// has refused every scaffold.
pub(crate) fn lone_site_placeholder<T: Real>(
    start: VertexKey,
    end: VertexKey,
    curve: &geom_brep::EdgeCurve<T>,
) -> bool {
    start == end && curve.description().is_scaffold()
}

/// Where a chord's arc ends on its section conic.
#[derive(Clone, Copy, Debug)]
enum ArcEnd<T> {
    /// At the conic parameter of a vertex at a distinct site.
    At(T),
    /// Back at its start, a whole turn on: a self-loop chord's.
    WholeTurn,
}

/// The candidate arc of `conic` from `th1` (the exact conic parameter
/// of the chord's start) to `to`, running forward from the start: the
/// ccw arc as it stands, the cw arc on the axis-flipped carrier.
fn oriented_arc<T: Real>(
    conic: &SectionCurve<T>,
    face: FaceKey,
    th1: T,
    to: ArcEnd<T>,
    ccw: bool,
) -> Result<(geom::Curve3<T>, T, T), SplitJoinError> {
    let tau = T::tau();
    // The span forward from `th1` to a distinct end, in `[0, τ)`: a
    // chord may span more than half the conic. Its window jumps at a
    // span of zero, two ends sharing a conic parameter, which is either
    // a zero-length arc or a whole turn. A chord's two ends at one site
    // say which ([`far_end`]: `ArcEnd::WholeTurn`). That ends at distinct
    // sites lie at distinct points nothing here enforces: two that share
    // a parameter read a zero span, which the chord's mint refuses.
    // Each span is read forward from its own start, never as a turn
    // less the other's, which would carry `ulp(τ)` in a short arc.
    if ccw {
        let span = match to {
            ArcEnd::At(th2) => (th2 - th1).reduce_periodic(tau),
            ArcEnd::WholeTurn => tau,
        };
        Ok((conic.carrier().clone(), th1, th1 + span))
    } else {
        // The cw arc: the carrier run back runs forward from p1.
        let span = match to {
            ArcEnd::At(th2) => (th1 - th2).reduce_periodic(tau),
            ArcEnd::WholeTurn => tau,
        };
        let flipped = conic
            .carrier()
            .reversed()
            .ok_or(SplitJoinError::SectionInvariant {
                face,
                what: "a section curve that does not reverse (a section curve is a circle, an \
                       ellipse or a spiric)",
            })?;
        let th1f = T::zero() - th1;
        Ok((flipped, th1f, th1f + span))
    }
}

/// **A chord's arc of `curve`**, from `p1` to `p2` (to `p1` again, a
/// whole turn, where `p2` is `None`), leaving `p1` counterclockwise
/// about the section's normal or against it: [`oriented_arc`] on a
/// conic seamed at the principal vertex nearest the arc's middle
/// ([`SectionConic::seamed_near`]), so its window is a short signed span
/// about the seam; a whole turn is seamed nearest its site. An oval is
/// taken as it stands, in the torus's minor angle.
fn chord_arc<T: Decide>(
    curve: SectionCurve<T>,
    face: FaceKey,
    p1: Point3<T>,
    p2: Option<Point3<T>>,
    ccw: bool,
) -> Result<(geom::Curve3<T>, T, T), SplitJoinError> {
    let seamed = match curve {
        SectionCurve::Conic(conic) => {
            let (tau, half) = (T::tau(), T::from_f64(0.5));
            let th1 = conic.param(p1);
            let middle = match p2 {
                None => th1,
                Some(p2) if ccw => th1 + (conic.param(p2) - th1).reduce_periodic(tau) * half,
                Some(p2) => th1 - (th1 - conic.param(p2)).reduce_periodic(tau) * half,
            };
            SectionCurve::Conic(conic.seamed_near(middle))
        }
        oval @ SectionCurve::Oval(_) => oval,
    };
    let to = p2.map_or(ArcEnd::WholeTurn, |p2| ArcEnd::At(seamed.param(p2)));
    oriented_arc(&seamed, face, seamed.param(p1), to, ccw)
}

/// **Where a chord from `u1` to `u2` ends on its closed section**:
/// `u2`'s point, or `None`, back at its start a whole turn on, where the
/// two are one site ([`same_site`]). A chord leaves its start, and the
/// null edges already join a site's copies, so a chord between two of
/// them runs round its whole closed section.
fn far_end<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    (u1, u2): (VertexKey, VertexKey),
) -> Result<Option<Point3<T>>, SplitJoinError> {
    Ok((!same_site(body, face, (u1, u2))?).then(|| vertex_point(body, u2)))
}

/// **Whether `u2` is one site with `u1`**: `u1` itself or a copy of it
/// the null edges tie ([`null_site`]), the join's one reading of
/// site-ness. A site that does not resolve is the join's broken
/// invariant.
fn same_site<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    (u1, u2): (VertexKey, VertexKey),
) -> Result<bool, SplitJoinError> {
    let tied = null_site(body, &[u1]).map_err(|StaleSite| SplitJoinError::SectionInvariant {
        face,
        what: StaleSite::WHAT,
    })?;
    Ok(tied.contains(&u2))
}

/// The direction the section leaves each of a join's two sites in,
/// toward the other: the datum the lane that paired the sites decided
/// the pairing on, handed to the chord so that it takes the arc the
/// pairing saw rather than deriving it again.
///
/// The boolean hands each matched half's germ direction
/// ([`crate::boolean::HalfGerm::dir`], [`Datum::Germ`]); the split hands
/// `±(n_plane × n_out)`, the way its conic walk enters the face at a
/// down crossing and leaves it at an up one (`splitting::join`'s
/// `split_leave`, [`Datum::Section`]). Either is tangent to the section
/// at the site, of any positive length.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Leave<T: Real> {
    /// Each matched half and the direction at its site.
    pub(crate) at: [(HalfEdgeKey, Vec3<T>); 2],
    /// Which quantity the directions are.
    pub(crate) datum: Datum,
}

/// What a [`Leave`]'s directions measure, and so how the chord decides
/// along them: two quantities, each under its own name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Datum {
    /// A boolean germ's unit direction: its component along the
    /// section's unit tangent is the cosine between them.
    Germ,
    /// The split's `±(n_plane × n_out)`: its component along the unit
    /// tangent is the sine between the plane and the wall times that
    /// cosine — the quantity, on the lever, its conic walk's
    /// `split_join_conic_heading` decides.
    Section,
}

/// One end's departure, as a chord reads it.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Departure<T: Real> {
    /// The section's direction at the chord's start.
    pub(crate) dir: Vec3<T>,
    /// What `dir` measures.
    pub(crate) datum: Datum,
}

impl<T: Real> Leave<T> {
    /// The departure at the site of matched half `he`; a half that is
    /// neither is the caller's broken invariant.
    fn from(&self, he: HalfEdgeKey, face: FaceKey) -> Result<Departure<T>, SplitJoinError> {
        self.at
            .iter()
            .find(|(h, _)| *h == he)
            .map(|&(_, dir)| Departure {
                dir,
                datum: self.datum,
            })
            .ok_or(SplitJoinError::SectionInvariant {
                face,
                what: "a chord starts at neither of the halves its departure datum names",
            })
    }
}

/// Whether the chord `p1 → p2` takes the conic's ccw candidate: the one
/// whose tangent at `p1` agrees with `leave`, the section's direction of
/// departure there ([`Leave`]). Decided as `leave · Ĉ′(θ₁)`, per datum:
/// a germ's cosine levered by the curve's extent, a conic's larger
/// semi-axis or an oval's own (`chord_arc_leave_germ`); the split's sine on the wall's
/// [`geom_brep::curvature_lever_arm`] at `p1`
/// (`chord_arc_leave_section`), the lever its conic walk decides the
/// same sine on, so the two cannot disagree outside the band.
///
/// **A split Zero has two readings**, told apart by the datum's own
/// direction against the tangent (`chord_arc_leave_section_aligned`: the
/// cosine levered by the conic's extent, as the germ's is). Along the
/// section, the small product is the plane within the band of tangent to
/// the wall at `p1` — the sine small, or the lever short, as near a
/// cone's apex — and the chord refuses the graze. Across it, the datum
/// is not the section's departure at all: `±(n_plane × n_out)` is the
/// section's tangent wherever `p1` lies on the conic, so that is a
/// broken invariant, as is a germ with no component along the section.
///
/// # Errors
///
/// [`SplitJoinError::SectionDepartureGrazes`] where the split's sine reads
/// Zero along the section; [`SplitJoinError::SectionInvariant`] for a
/// datum normal to the section; [`SplitJoinError::Escalated`] in the band.
fn arc_leaving<T: Decide>(
    face: FaceKey,
    band: Band,
    conic: &SectionCurve<T>,
    wall: &geom::Surface<T>,
    p1: Point3<T>,
    leave: Departure<T>,
) -> Result<bool, SplitJoinError> {
    let escalated = |diag| SplitJoinError::Escalated { face, diag };
    let off_section = SplitJoinError::SectionInvariant {
        face,
        what: "the section's direction of departure handed to a chord end has no \
               component along the section there",
    };
    let tangent = conic.tangent(conic.param(p1));
    let along = leave.dir.dot(tangent) / tangent.norm();
    let (name, margin) = match leave.datum {
        Datum::Germ => (
            "chord_arc_leave_germ",
            Margin::levered(along, conic.lever()),
        ),
        Datum::Section => (
            "chord_arc_leave_section",
            Margin::levered(along, geom_brep::curvature_lever_arm(wall, p1)),
        ),
    };
    match decide(name, margin, band).map_err(escalated)? {
        Sign::Positive => Ok(true),
        Sign::Negative => Ok(false),
        Sign::Zero => match leave.datum {
            Datum::Section => {
                let cosine = along / leave.dir.norm();
                match decide(
                    "chord_arc_leave_section_aligned",
                    Margin::levered(cosine, conic.lever()),
                    band,
                )
                .map_err(escalated)?
                {
                    Sign::Zero => Err(off_section),
                    Sign::Positive | Sign::Negative => {
                        Err(SplitJoinError::SectionDepartureGrazes { face, band })
                    }
                }
            }
            Datum::Germ => Err(off_section),
        },
    }
}

/// A chord's curve as [`chord_spec`] mints it, with how the section
/// table read the plane where it is a section conic
/// ([`section_end_reading`]).
pub(crate) struct SectionChord<T: Real> {
    spec: EdgeCurveSpec<T>,
    ends: Option<SectionEndReading>,
    /// A carried chord's arc, with its evidence of lying on the face.
    arc: Option<SplineArc<T>>,
}

impl<T: Real> SectionChord<T> {
    /// A chord that is no section conic: a ruling, or an edge's copy.
    const fn bare(spec: EdgeCurveSpec<T>) -> Self {
        Self {
            spec,
            ends: None,
            arc: None,
        }
    }
}

/// K name: a section chord's carrier's own resolution over its span
/// ([`geom_brep::carrier_resolution::resolution`]), `K` times over,
/// against the zero band (metres), where the gate refuses one of its
/// attachment pins: Zero reads an in-band pin as the input's
/// ([`crate::validate::resolved`]).
pub(crate) const SECTION_PIN_RESOLUTION: &str = "section_pin_resolution";

/// K name: a section chord's end miss past the band, moved toward zero
/// by its carrier's resolution (metres): Zero says the carrier's own
/// rounding can account for the whole miss.
pub(crate) const SECTION_PIN_WITHIN: &str = "section_pin_within";

/// **How the section table read the plane a chord on `curve` lies in**:
/// a stood-in circle's reading where the table read the plane square to
/// the wall's axis, the plane's own section otherwise; `None` for an
/// oval, whose carrier has no resolution bound.
fn section_end_reading<T: Real>(curve: &SectionCurve<T>) -> Option<SectionEndReading> {
    let SectionCurve::Conic(conic) = curve else {
        return None;
    };
    Some(if conic.square {
        SectionEndReading::SquareToAxis
    } else {
        SectionEndReading::OnPlane
    })
}

/// **The join's refusal of a section chord on `spec`, between vertices
/// at `at`, dividing `face`, whose minting the gate refused**, read by
/// how the table read the plane (`reading`) and against the carrier's
/// own resolution over the edge's span, `ρ`.
///
/// - **In band.** Where `ρ` is within the `ε/K` share of the band
///   ([`crate::validate::resolved`]), the miss is the input's: an end
///   pin by either reading, a residual off the surfaces only on a
///   stood-in circle, whose plane is the section plane only to within
///   the tilt read Zero ([`SplitJoinError::SectionEndEscalated`], with
///   its lever). A residual on the plane's own section, or a witness
///   missed, has no reading of the input in it, and keeps the gate's
///   refusal. Past that share, the carrier's rounding is more than the
///   band was sized for: a kernel limitation
///   ([`SplitJoinError::SectionUnresolved`]), with no lever.
/// - **Past the band**, an end pin whose miss, moved toward zero by
///   `ρ`, reads Zero is the carrier's rounding: a kernel limitation
///   too. Any other definite miss keeps the gate's refusal.
///
/// Every other check, a carrier with no bound, and an escalation that is
/// no statement about the input keep the gate's refusal: a margin nobody
/// could read, or an enclosure that straddles a boundary, which
/// subdividing may still decide (D9 row 1).
fn section_pin_refusal<T: Decide>(
    reading: SectionEndReading,
    face: FaceKey,
    (spec, at): (&EdgeCurveSpec<T>, (Point3<T>, Point3<T>)),
    refusal: EulerOpError,
    band: Band,
) -> SplitJoinError {
    use geom_brep::CertCheck as C;
    let EulerOpError::Certification { error } = &refusal else {
        return refusal.into();
    };
    let Some(rho) =
        geom_brep::carrier_resolution::resolution(&spec.carrier, spec.param_start, spec.param_end)
    else {
        return refusal.into();
    };
    match *error {
        geom_brep::CertifyError::Escalated { check, cause, .. }
            if cause.margin.kind() == geom_core::MarginKind::Value || cause.terminal_sliver =>
        {
            let typed = match check {
                C::EndpointStart | C::EndpointEnd => Some(reading),
                C::SurfaceResidual { .. } | C::WitnessSurfaceResidual { .. } => {
                    (reading == SectionEndReading::SquareToAxis).then_some(reading)
                }
                C::WitnessMidpoint => None,
                _ => return refusal.into(),
            };
            match (
                crate::validate::resolved(SECTION_PIN_RESOLUTION, rho, band),
                typed,
            ) {
                (Err(resolution), _) => SplitJoinError::SectionUnresolved {
                    face,
                    miss: SectionPinMiss::Undecided {
                        check,
                        diag: cause,
                        resolution,
                    },
                },
                (Ok(()), Some(reading)) => SplitJoinError::SectionEndEscalated {
                    face,
                    reading,
                    diag: cause,
                },
                (Ok(()), None) => refusal.into(),
            }
        }
        geom_brep::CertifyError::ResidualExceeded {
            check: check @ (C::EndpointStart | C::EndpointEnd),
            sample,
            margin,
        } => {
            let (t, vertex) = if check == C::EndpointStart {
                (spec.param_start, at.0)
            } else {
                (spec.param_end, at.1)
            };
            let inward = (spec.carrier.eval(t).distance(vertex) - rho).max(T::zero());
            match decide(SECTION_PIN_WITHIN, Margin::of(inward), band) {
                Ok(Sign::Zero) => SplitJoinError::SectionUnresolved {
                    face,
                    miss: SectionPinMiss::Past {
                        check,
                        sample,
                        margin,
                    },
                },
                _ => refusal.into(),
            }
        }
        _ => refusal.into(),
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
///    a ruling falls back to the straight chord, a tangent ruling
///    stores what the must-carry rule over it demands
///    ([`geom_brep::must_carry_over_edge`]: the intrinsic tangency, or
///    an image in the section chart), escalations pass through whole.
/// 2. Orient: of the conic's two arcs from `u1` to `u2`, take the one
///    leaving `u1` along `leave`, the section's direction of departure
///    there as the lane that paired the chord's ends decided it
///    ([`Leave`], [`arc_leaving`]). Which arc lies in the face is the
///    pairing's answer; the chord does not ask the face again. A
///    chord from a vertex to itself or to a copy of its site
///    ([`same_site`]) is the whole conic, run the same way; on
///    a ruling section, and in the plane × plane and along-edge lanes,
///    a self-loop stays a lone site (`None`, the placeholder circle). A
///    self-loop along a closed edge never reaches here: it copies the
///    edge ([`closed_edge_spec`]).
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
    leave: Departure<T>,
) -> Result<Option<SectionChord<T>>, SplitJoinError> {
    // A self-loop chord that copies no closed edge is a lone site
    // (module docs) in the lanes that section no conic.
    if u1 == u2 && matches!(lane, JoinLane::Planar | JoinLane::AlongEdge) {
        return Ok(None);
    }
    body.get_vertex(u1).ok_or(SplitJoinError::Corrupt {
        entity: EntityId::Vertex(u1),
    })?;
    let face_data = body.get_face(face).ok_or_else(|| corrupt_face(face))?;
    let wall_key = face_data.surface;
    // The surface is a link of the face the join divides, mid-operation
    // ([`crate::live::OPERATORS_KEEP_LINKS`]): a torn one is not curved.
    if let geom::Surface::Plane { .. } = body.face_surface_linked(face, face_data) {
        // The boolean's planar-side chord of a curved germ pair
        // takes its own lane (M5 PR 9); every other lane keeps
        // the straight chord BIT-IDENTICALLY.
        return match lane {
            JoinLane::BoolPlanar {
                wall,
                window,
                partner_key,
                branch,
            } => bool_planar_chord_spec(
                body,
                band,
                face,
                wall_key,
                (&wall, window),
                (partner_key, branch),
                u1,
                u2,
                leave,
            ),
            JoinLane::Planar | JoinLane::Split(_) => Ok(None),
            JoinLane::Carried(_) => Err(SplitJoinError::SectionInvariant {
                face,
                what: "a carried chord asked of a planar face (a carried section's faces are a \
                       cylinder's and a sphere's)",
            }),
            JoinLane::AlongEdge => Err(SplitJoinError::SectionInvariant {
                face,
                what: "a chord along an edge asked for a section: it copies the edge",
            }),
        };
    }
    let ctx = match lane {
        JoinLane::Split(ctx) => ctx,
        JoinLane::Carried(ctx) => {
            return carried_chord_spec(body, band, face, ctx, (u1, u2), leave).map(Some);
        }
        JoinLane::Planar | JoinLane::BoolPlanar { .. } | JoinLane::AlongEdge => {
            return Err(SplitJoinError::SectionInvariant {
                face,
                what: "curved section chord outside the split/wall lane (the plane×plane lane \
                       divides only planar faces)",
            });
        }
    };
    let Some(WallSection { wall, case }) =
        wall_section(body, band, ctx.origin, ctx.normal, face, u1)?
    else {
        return Err(SplitJoinError::SectionInvariant {
            face,
            what: "a face read planar by its key reads curved by its section",
        });
    };
    let conic = match case {
        // A two-ruling section: the straight chord is the honest carrier.
        SectionCase::Straight(_) => return Ok(None),
        // The tangent ruling stores what the must-carry rule over it
        // demands, and is pushed through the ordinary certification
        // gate by the mef/mekr caller. No arc-side rule applies: a line
        // has no complementary candidate.
        SectionCase::Tangent(line) => {
            let (geom::Curve3::Line { origin, dir }, Some(back)) = (line.clone(), line.reversed())
            else {
                return Err(SplitJoinError::SectionInvariant {
                    face,
                    what: "tangent classification carried a non-line",
                });
            };
            let p1 = vertex_point(body, u1);
            let p2 = vertex_point(body, u2);
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
                Sign::Positive => (line.clone(), t1, t2),
                Sign::Negative => (back, T::zero() - t1, T::zero() - t2),
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
            // Jet-determinate ⇒ the intrinsic tangency; under-determined
            // (a sagitta decided Zero over a short ruling, or a carrier
            // outside the tangent certificate's lane) ⇒ the conventional
            // pair, as `describe_section_boundary` restates a smooth
            // section-boundary edge.
            let plane = body.get_surface(plane_key).unwrap_or_else(|| {
                unreachable!(
                    "{plane_key:?} is the section plane this split minted into the body it is \
                     dividing, and no operator removes a surface a later chord still names"
                )
            });
            // The section face is not minted yet: only the face the
            // ruling divides names its region.
            let rest = crate::rest_locus(
                body,
                [(wall_key, Some(face)), (plane_key, None)],
                &carrier,
                (s1, s2),
                band,
            );
            let description = geom_brep::must_carry_description(
                geom_brep::SmoothJoin {
                    surfaces: [(wall_key, &wall), (plane_key, plane)],
                    carrier: &carrier,
                    interval: (s1, s2),
                    // The section face is not minted yet, so the wedge
                    // reads the face the ruling divides.
                    reach: crate::wedge_reach(body, [face; 2], &carrier, (s1, s2)),
                    witness,
                },
                band,
                |_, diag| SplitJoinError::TangentChordEscalated { face, diag },
                |_, diag| SplitJoinError::TangentChordBendEscalated { face, diag },
                |mixed| SplitJoinError::TangentChordRefuted { face, mixed },
            )?
            .spec(rest);
            return Ok(Some(SectionChord::bare(EdgeCurveSpec {
                description,
                carrier,
                param_start: s1,
                param_end: s2,
            })));
        }
        SectionCase::Conic(c) => SectionCurve::Conic(c),
        SectionCase::Loops { loops, split } => section_loop(
            face,
            loops,
            (split, ctx.branch),
            [vertex_point(body, u1), vertex_point(body, u2)],
            band,
        )?,
    };
    let p1 = vertex_point(body, u1);
    let ccw = arc_leaving(face, band, &conic, &wall, p1, leave)?;
    // The aux plane surface (honest u_ref: a direction in the section
    // plane, ⊥ normal by construction), minted once per split.
    let plane_key = match ctx.plane_key {
        Some(k) => k,
        None => {
            let k = body.add_surface(geom::Surface::Plane {
                origin: ctx.origin,
                normal: ctx.normal.get(),
                u_ref: conic.in_plane(),
            });
            ctx.plane_key = Some(k);
            k
        }
    };
    let ends = section_end_reading(&conic);
    let (carrier, t_start, t_end) =
        chord_arc(conic, face, p1, far_end(body, face, (u1, u2))?, ccw)?;
    let witness = carrier.mid_point(t_start, t_end);
    Ok(Some(SectionChord {
        spec: EdgeCurveSpec {
            description: geom_brep::EdgeDescriptionSpec::Intersection {
                pair: geom_brep::SurfacePair::new(wall_key, plane_key),
                selector: geom_brep::CurveSelector::Witness(witness),
            },
            carrier,
            param_start: t_start,
            param_end: t_end,
        },
        ends,
        arc: None,
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
/// plane, and nothing here knows which lane asks. So a cylinder wall is
/// read as the most guarded lane needs it, the Boolean's germ join on the
/// declared-tangency path: served only where the face's reach over its
/// edges' spans and round their whole turn agree ([`agreed_section`]).
///
/// # Errors
///
/// [`SplitJoinError::SectionUnarmed`] for a kind no section arm
/// reads; the table's refusals ([`section_case`]).
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
        | geom::Surface::Cone { .. }
        | geom::Surface::Torus { .. } => {}
        _ => {
            return Err(SplitJoinError::SectionUnarmed {
                face,
                divided: wall.kind(),
                wall: wall.kind(),
            });
        }
    }
    let plane_s = geom_brep::PlaneSite {
        origin,
        normal: normal.get(),
    };
    let case = plane_wall_case(
        face,
        band,
        &plane_s,
        &wall,
        section_reach(body, at, face, (&wall, None), band)?,
    )?;
    Ok(Some(WallSection { wall, case }))
}

/// [`wall_section`]'s answer.
pub(crate) struct WallSection<T: Real> {
    /// The face's wall surface.
    pub(crate) wall: geom::Surface<T>,
    /// What the table made of the plane against it.
    pub(crate) case: SectionCase<T>,
}

/// The azimuth window of `face`'s outer loop on `surface`'s chart: the
/// hull of [`face_azimuth_images`]. `None` for a loop that is not a
/// cycle, or a walk with no charted edge.
///
/// # Errors
///
/// As [`face_azimuth_images`].
///
/// # Panics
///
/// As [`face_azimuth_images`].
pub(crate) fn face_azimuth_window<T: Decide>(
    body: &Body<T>,
    surface: &geom::Surface<T>,
    face: FaceKey,
    band: Band,
) -> Result<Option<(T, T)>, SplitJoinError> {
    Ok(face_azimuth_images(body, surface, face, band)?.and_then(|images| azimuth_hull(&images)))
}

/// The azimuth window of each cycle loop of `face` on `surface`'s chart,
/// the outer loop and its rings alike: the hull of the loop's images,
/// `None` for a walk with no charted edge. A lone-vertex loop winds
/// nothing and is not listed.
///
/// # Errors
///
/// As [`face_azimuth_images`], for any loop.
///
/// # Panics
///
/// As [`face_azimuth_images`].
pub(crate) fn face_loop_azimuth_windows<T: Decide>(
    body: &Body<T>,
    surface: &geom::Surface<T>,
    face: FaceKey,
    band: Band,
) -> Result<Vec<Option<(T, T)>>, SplitJoinError> {
    let data = body.get_face(face).ok_or_else(|| corrupt_face(face))?;
    body.face_loops_linked(face, data)
        .filter_map(|(_, l)| match l.boundary {
            LoopBoundary::Cycle { first } => Some(first),
            _ => None,
        })
        .map(|first| {
            let halves = body.loop_walk(first).closed("loop", first);
            run_azimuth_images(body, surface, face, &halves, band)
                .map(|images| azimuth_hull(&images))
        })
        .collect()
}

/// [`face_azimuth_window`] on `face`'s own surface: the window every
/// reader folds, read from outside the crate so a suite can hold the
/// interval lane's window against the `f64` replay's. `sweep-testing`
/// only, never production surface.
///
/// # Errors
///
/// As [`face_azimuth_window`]: [`SplitJoinError::Corrupt`] for a face
/// key that does not resolve.
///
/// # Panics
///
/// As [`face_azimuth_window`], and where the face's surface does not
/// resolve: a torn link past the caller's key (D2 row 4).
#[cfg(feature = "sweep-testing")]
pub fn face_azimuth_window_traces<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    band: Band,
) -> Result<Option<(T, T)>, SplitJoinError> {
    let data = body.get_face(face).ok_or_else(|| corrupt_face(face))?;
    face_azimuth_window(body, body.face_surface_linked(face, data), face, band)
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
    (wall, partner_window): (&geom::Surface<T>, Option<(T, T)>),
    (partner_key, branch): (&mut Option<SurfaceKey>, crate::boolean::Branch),
    u1: VertexKey,
    u2: VertexKey,
    leave: Departure<T>,
) -> Result<Option<SectionChord<T>>, SplitJoinError> {
    if !matches!(
        wall,
        geom::Surface::Cylinder { .. }
            | geom::Surface::Sphere { .. }
            | geom::Surface::Cone { .. }
            | geom::Surface::Torus { .. }
    ) {
        return Err(SplitJoinError::SectionUnarmed {
            face,
            divided: geom::SurfaceKind::Plane,
            wall: wall.kind(),
        });
    }
    let plane_s = match body.get_surface(plane_key) {
        Some(plane_s @ geom::Surface::Plane { .. }) => plane_s.clone(),
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
    let conic = match section_case(
        face,
        band,
        &plane_s,
        wall,
        section_reach(body, u1, face, (wall, partner_window), band)?,
    )? {
        // A two-ruling section's chords are straight on the plane too.
        SectionCase::Straight(_) => return Ok(None),
        // A tangent germ pair in the boolean join means TOUCHING
        // operands: a germ tangent to a bound of its sector is read in
        // the face across it (`boolean::insert`), so reaching here is a
        // frontier configuration, refused typed. (The split lane mints
        // a chord on the same ruling; that difference is why
        // `section_case` hands the arm back instead of deciding.)
        SectionCase::Tangent(_) => {
            return Err(SplitJoinError::SectionInvariant {
                face,
                what: "tangent plane×wall germ pair in the boolean join — a touching \
                       configuration, the typed frontier of the supported envelope",
            });
        }
        SectionCase::Conic(c) => SectionCurve::Conic(c),
        SectionCase::Loops { loops, split } => section_loop(
            face,
            loops,
            (split, Some(branch)),
            [vertex_point(body, u1), vertex_point(body, u2)],
            band,
        )?,
    };
    let p1 = vertex_point(body, u1);
    let ccw = arc_leaving(face, band, &conic, wall, p1, leave)?;
    let ends = section_end_reading(&conic);
    let (carrier, t_start, t_end) =
        chord_arc(conic, face, p1, far_end(body, face, (u1, u2))?, ccw)?;
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
    Ok(Some(SectionChord {
        spec: EdgeCurveSpec {
            description: geom_brep::EdgeDescriptionSpec::Intersection {
                pair: geom_brep::SurfacePair::new(plane_key, wall_aux),
                selector: geom_brep::CurveSelector::Witness(witness),
            },
            carrier,
            param_start: t_start,
            param_end: t_end,
        },
        ends,
        arc: None,
    }))
}

/// **A carried chord** ([`JoinLane::Carried`]): the arc of the
/// component's carrier from `u1`'s site to `u2`'s, the way the section
/// leaves `u1` (`chord_carried_leave`, levered at the section's scale),
/// described as the intersection of the divided face's surface and this
/// body's copy of the partner's. A site's parameter is its foot on the
/// carrier, read on it. A closed component is seated at one of its
/// sites (`boolean::section_table`): its carrier runs from that site
/// round to it, so a chord leaving the seat starts at the span's start
/// when it runs forward and at its end when it runs back, one arriving
/// there the other way round, and a one-site loop's chord is the whole
/// loop. No chord straddles the seat: the arc runs forward from its
/// start's parameter to its end's (`chord_carried_forward`), cut from
/// the carrier's net over that interval ([`cut_arc`]) and reflected
/// where it leaves backward ([`geom_brep::reversed_column`]), so the two
/// solids' chords are cut from one carrier's bits. The chord carries
/// its arc with the component's evidence of lying on both surfaces
/// ([`SplineArc::of_section`]), which the ring lane's closing reads.
///
/// # Errors
///
/// [`SplitJoinError::Escalated`] for a reading in the band;
/// [`SplitJoinError::SectionInvariant`] for a component whose carrier is
/// no spline or holds no certificate, a site off the component, a
/// departure across it or of the split's datum, and an arc that would
/// cross the seat.
fn carried_chord_spec<T: Decide>(
    body: &mut Body<T>,
    band: Band,
    face: FaceKey,
    ctx: &mut CarriedCtx<T>,
    (u1, u2): (VertexKey, VertexKey),
    leave: Departure<T>,
) -> Result<SectionChord<T>, SplitJoinError> {
    let invariant = |what| SplitJoinError::SectionInvariant { face, what };
    let escalated = |diag| SplitJoinError::Escalated { face, diag };
    let own = body
        .get_face(face)
        .ok_or_else(|| corrupt_face(face))?
        .surface;
    let component = &ctx.component;
    let geom::Curve3::Nurbs(net) = &component.carrier else {
        return Err(invariant("a carried component's carrier is not a spline"));
    };
    let carrier = &component.carrier;
    let (lo, hi) = component.span;
    let seat = carrier.eval(lo);
    let at_seat = |p: Point3<T>| -> Result<bool, SplitJoinError> {
        if !component.closed() {
            return Ok(false);
        }
        Ok(
            decide("chord_carried_at_seat", Margin::norm3(p - seat), band).map_err(escalated)?
                == Sign::Zero,
        )
    };
    let foot = |p: Point3<T>| -> Result<T, SplitJoinError> {
        let foot = carrier.foot(p, lo);
        match foot.on_carrier(band) {
            Ok(geom::OnCarrier::On { .. }) => Ok(foot.t()),
            Ok(geom::OnCarrier::Off) => {
                Err(invariant("a carried chord's end lies off its component"))
            }
            Err(geom::CarrierRefusal::Escalated(diag) | geom::CarrierRefusal::Straddle(diag)) => {
                Err(escalated(diag))
            }
            Err(geom::CarrierRefusal::Exhausted | geom::CarrierRefusal::Unbounded) => Err(
                invariant("a carried chord end's distance from its component could not be read"),
            ),
        }
    };
    let (p1, p2) = (vertex_point(body, u1), vertex_point(body, u2));
    let seat1 = at_seat(p1)?;
    let from = if seat1 { lo } else { foot(p1)? };
    let tangent = carrier.deriv(from);
    if leave.datum != Datum::Germ {
        return Err(invariant(
            "a carried chord's departure is not a germ's direction",
        ));
    }
    let along = leave.dir.dot(tangent) / tangent.norm();
    let up = match decide(
        "chord_carried_leave",
        Margin::levered(along, ctx.lever),
        band,
    )
    .map_err(escalated)?
    {
        Sign::Positive => true,
        Sign::Negative => false,
        Sign::Zero => {
            return Err(invariant(
                "the section's direction of departure handed to a chord end has no component \
                 along the section there",
            ));
        }
    };
    let (start, stop) = if up { (lo, hi) } else { (hi, lo) };
    let t1 = if seat1 { start } else { from };
    let t2 = if at_seat(p2)? { stop } else { foot(p2)? };
    // The travel in parameter, metered by the carrier's speed at the
    // chord's start. Only its sign is consumed, and the margin's sign is
    // the travel's own (the speed is positive): the point speed sets only
    // where the band's edge falls in parameter, so a speed off the
    // carrier's elsewhere can move a travel near zero across the band's
    // edge, a refusal or a chord a band long, and never turns a chord
    // round.
    let forward = if up { t2 - t1 } else { t1 - t2 };
    if decide(
        "chord_carried_forward",
        Margin::metered(forward, InfSpeed::new(tangent.norm())),
        band,
    )
    .map_err(escalated)?
        != Sign::Positive
    {
        return Err(invariant(
            "a carried chord would cross its component's seat",
        ));
    }
    let (lo_cut, hi_cut) = if up { (t1, t2) } else { (t2, t1) };
    let piece = cut_arc(net, (lo_cut, hi_cut)).ok_or(invariant(
        "a carried chord's arc could not be cut from its component's carrier",
    ))?;
    let forward_arc = geom::Curve3::Nurbs(std::sync::Arc::new(piece.clone()));
    let arc = SplineArc::of_section(component, &forward_arc, (lo_cut, hi_cut)).ok_or(invariant(
        "a carried chord's component holds no certificate of lying on its surfaces",
    ))?;
    let (carrier, t_start, t_end) = if up {
        (forward_arc, t1, t2)
    } else {
        (
            geom::Curve3::Nurbs(std::sync::Arc::new(geom_brep::reversed_column(&piece))),
            T::zero() - t1,
            T::zero() - t2,
        )
    };
    let partner = match ctx.partner_key {
        Some(k) => k,
        None => {
            let k = body.add_surface(ctx.partner.clone());
            ctx.partner_key = Some(k);
            k
        }
    };
    let witness = carrier.mid_point(t_start, t_end);
    Ok(SectionChord {
        spec: EdgeCurveSpec {
            description: geom_brep::EdgeDescriptionSpec::Intersection {
                pair: geom_brep::SurfacePair::new(own, partner),
                selector: geom_brep::CurveSelector::Witness(witness),
            },
            carrier,
            param_start: t_start,
            param_end: t_end,
        },
        ends: None,
        arc: Some(arc),
    })
}

/// **The arc of `net` over `[lo, hi]`, cut from its bits**: the net
/// split at each end inside its domain ([`geom::NurbsCurve3::split_at`],
/// which keeps the parent's parameter), so the arc is the carrier's own
/// net over the interval and every reader of the chord reads only that.
/// `None` where an end is no point, or the split refuses.
fn cut_arc<T: Decide>(
    net: &geom::NurbsCurve3<T>,
    (lo, hi): (T, T),
) -> Option<geom::NurbsCurve3<T>> {
    let (d0, d1) = net.knot_values().domain();
    let (a, b) = (lo.point_value()?, hi.point_value()?);
    let (e0, e1) = (d0.point_value()?, d1.point_value()?);
    let mut piece = net.clone();
    if a > e0 {
        piece = piece.split_at(a).ok()?.1;
    }
    if b < e1 {
        piece = piece.split_at(b).ok()?.0;
    }
    Some(piece)
}

/// **A section segment's chord curve**: the curve both of its chords
/// are minted on, computed ONCE per join ([`ChordJoiner::segment_curve`])
/// and read by everything that needs the chord's geometry — the joiner's
/// `mef`/`mekr` ([`ChordJoiner::join`]) and the boolean join's role
/// resolution, whose ring lane winds the run the first chord closes
/// (`boolean::join::ring_run_ccw`). A curve read in one place and
/// assumed in another is two answers to one question.
///
/// `spec` runs from the site of `halves.0` to the site of `halves.1` (the
/// two matched null halves, whose null edges tie each site's copies);
/// `None` is the straight chord, which `mef_chord`/`mekr_chord` mint
/// between the chord's own ends.
pub(crate) struct SegmentCurve<T: Real> {
    /// The matched halves the curve runs between, from the first's site.
    halves: (HalfEdgeKey, HalfEdgeKey),
    /// The curve, or `None` for the straight chord.
    spec: Option<EdgeCurveSpec<T>>,
    /// How the section table read the plane, where the curve is a
    /// section conic.
    ends: Option<SectionEndReading>,
    /// A carried chord's arc, with its evidence of lying on the face
    /// ([`SplineArc::of_section`]): the closing a ring-lane island on the
    /// carried lane reads.
    arc: Option<SplineArc<T>>,
}

#[cfg(test)]
impl<T: Real> SegmentCurve<T> {
    /// A segment curve as `segment_curve` would hand it over, for a row
    /// that winds a closing without a join.
    pub(crate) fn of(halves: (HalfEdgeKey, HalfEdgeKey), spec: Option<EdgeCurveSpec<T>>) -> Self {
        Self {
            halves,
            spec,
            ends: None,
            arc: None,
        }
    }
}

impl<T: Real> SegmentCurve<T> {
    /// The curve from the site of `halves.0`, or `None` for the straight
    /// chord.
    pub(crate) fn spec(&self) -> Option<&EdgeCurveSpec<T>> {
        self.spec.as_ref()
    }

    /// The carried chord's arc with its evidence of lying on the face,
    /// where the curve is one.
    pub(crate) fn carried_arc(&self) -> Option<&SplineArc<T>> {
        self.arc.as_ref()
    }
}

impl<T: Decide> SegmentCurve<T> {
    /// The curve closing the ring-lane run `[h1 .. h2]` — the chord the
    /// joiner's first `mef` mints, run from `h2`'s site back to `h1`'s,
    /// as the run's new face walks it (`None`: the straight chord).
    pub(crate) fn run_closing(
        &self,
        h1: HalfEdgeKey,
        face: FaceKey,
    ) -> Result<Option<EdgeCurveSpec<T>>, SplitJoinError> {
        let h2 = match self.halves {
            (a, b) if h1 == a => b,
            (a, b) if h1 == b => a,
            _ => {
                return Err(SplitJoinError::SectionInvariant {
                    face,
                    what: "a ring run opens at neither of its segment's matched halves",
                });
            }
        };
        self.running_from(h2, face)
    }

    /// The join's refusal of a chord on `spec`, this curve run from one of
    /// its ends between the vertices `ends`, dividing `face`, where the
    /// gate refuses minting it: a section conic's own reading of its pin
    /// ([`section_pin_refusal`]), else the gate's.
    fn refusal(
        &self,
        body: &Body<T>,
        face: FaceKey,
        ends: (VertexKey, VertexKey),
        spec: Option<&EdgeCurveSpec<T>>,
        band: Band,
    ) -> impl FnOnce(EulerOpError) -> SplitJoinError + use<T> {
        let read = self.ends.zip(spec.cloned());
        let at = (vertex_point(body, ends.0), vertex_point(body, ends.1));
        move |refusal| match read {
            Some((reading, spec)) => section_pin_refusal(reading, face, (&spec, at), refusal, band),
            None => refusal.into(),
        }
    }

    /// The chord's spec running from the site of matched half `at` to the
    /// other's: the curve as computed, or the same curve run back. A half
    /// that is neither of the two is the caller's broken invariant.
    fn running_from(
        &self,
        at: HalfEdgeKey,
        face: FaceKey,
    ) -> Result<Option<EdgeCurveSpec<T>>, SplitJoinError> {
        let invariant = |what| SplitJoinError::SectionInvariant { face, what };
        let Some(spec) = &self.spec else {
            return Ok(None);
        };
        if at == self.halves.0 {
            return Ok(Some(spec.clone()));
        }
        if at != self.halves.1 {
            return Err(invariant(
                "a chord starts at neither of its segment's matched halves",
            ));
        }
        run_back(spec, face, None).map(Some)
    }
}

/// `spec` run back: the same locus from its other end. A spline runs
/// back by its net reflected through 0, exact
/// ([`geom_brep::reversed_column`]); every closed form has its own
/// reversal. A scaffold names its start point's trajectory, so it is
/// re-stated from the other end; any other description is read as
/// [`chord_description`] reads it.
fn run_back<T: Decide>(
    spec: &EdgeCurveSpec<T>,
    face: FaceKey,
    chart: Option<SurfaceKey>,
) -> Result<EdgeCurveSpec<T>, SplitJoinError> {
    let invariant = |what| SplitJoinError::SectionInvariant { face, what };
    let carrier = match &spec.carrier {
        geom::Curve3::Nurbs(net) => {
            geom::Curve3::Nurbs(std::sync::Arc::new(geom_brep::reversed_column(net)))
        }
        other => other
            .reversed()
            .ok_or(invariant("a segment curve with no reversal"))?,
    };
    let (t0, t1) = (T::zero() - spec.param_end, T::zero() - spec.param_start);
    match &spec.description {
        geom_brep::EdgeDescriptionSpec::Scaffold(_) => match carrier {
            geom::Curve3::Line { .. } => EdgeCurveSpec::segment_of_line(carrier, t0, t1),
            back => EdgeCurveSpec::arc_of_circle(back, t0, t1),
        }
        .ok_or(invariant(
            "a scaffold segment curve that is neither a line nor a circle (a body at rest \
             carries no scaffold, and only lines and circles are minted as one)",
        )),
        description => Ok(EdgeCurveSpec {
            description: chord_description(description, face, chart)?,
            carrier,
            param_start: t0,
            param_end: t1,
        }),
    }
}

/// `description` as a chord on the same locus may state it: one naming
/// its surface pair, a chart image the door derives, or a scaffold
/// (stated forward). A chord bounds two faces, so it is never a wrap
/// edge (D1), and no chord lane states a declaration or carries an image
/// of another curve's parameter. A copy of an edge stating any of those
/// lies in the face it divides, whose surface is `chart`: it is
/// re-stated there, its image derived from its own carrier. A section
/// chord (`chart` `None`) is computed, so it states none of them.
fn chord_description<T: Real>(
    description: &geom_brep::EdgeDescriptionSpec<T>,
    face: FaceKey,
    chart: Option<SurfaceKey>,
) -> Result<geom_brep::EdgeDescriptionSpec<T>, SplitJoinError> {
    match (description, chart) {
        (
            geom_brep::EdgeDescriptionSpec::Intersection { .. }
            | geom_brep::EdgeDescriptionSpec::TangentIntersection { .. }
            | geom_brep::EdgeDescriptionSpec::OnPair { declared: None, .. }
            | geom_brep::EdgeDescriptionSpec::Chart {
                image: None,
                wrap: false,
                declared: None,
                ..
            }
            | geom_brep::EdgeDescriptionSpec::Scaffold(_),
            _,
        ) => Ok(description.clone()),
        (
            geom_brep::EdgeDescriptionSpec::OnPair { .. }
            | geom_brep::EdgeDescriptionSpec::Chart { .. },
            Some(surface),
        ) => Ok(geom_brep::EdgeDescriptionSpec::chart(surface)),
        (geom_brep::EdgeDescriptionSpec::OnPair { .. }, None) => {
            Err(SplitJoinError::SectionInvariant {
                face,
                what: "a section curve stating a declaration on a pair (no section lane states \
                       one)",
            })
        }
        (geom_brep::EdgeDescriptionSpec::Chart { .. }, None) => {
            Err(SplitJoinError::SectionInvariant {
                face,
                what: "a section curve stating a chart image, a declaration or a wrap (no \
                       section lane states one)",
            })
        }
    }
}

/// How a join knows the section segment it chords is an edge the face
/// already has: the adjacency skip's question, asked of the one edge
/// between two halves when a join is planned ([`JoinPlan::of`]).
#[derive(Clone, Copy)]
pub(crate) enum SegmentEdge<'a, T: Real> {
    /// A boolean match: the edge the matched germs' locus names on this
    /// solid, or `None` where the segment lies inside a face. The skip
    /// is structural ([`between_is_segment`]).
    Locus(Option<EdgeKey>),
    /// The plane split: an edge lying in the section plane
    /// ([`between_edge_is_section`]) — the plane the split's chords are
    /// curved against ([`JoinLane::Split`]). An escalated verdict refuses
    /// typed rather than guessing either way; a belly conic between the
    /// halves is no section segment, and its chord is minted.
    InPlane(&'a SectionCtx<T>),
}

impl<T: Decide> SegmentEdge<'_, T> {
    /// The edge this segment is, where a locus names one.
    fn locus(self) -> Option<EdgeKey> {
        match self {
            Self::Locus(edge) => edge,
            Self::InPlane(_) => None,
        }
    }

    /// Whether the edge under `between` IS the section segment.
    fn is(
        self,
        body: &Body<T>,
        between: HalfEdgeKey,
        face: FaceKey,
        band: Band,
    ) -> Result<bool, SplitJoinError> {
        match self {
            Self::Locus(edge) => between_is_segment(body, edge, between),
            Self::InPlane(ctx) => between_edge_is_section(body, ctx, between, band)?.ok_or(
                SplitJoinError::SectionInvariant {
                    face,
                    what: "section classification of the join-adjacent edge escalated",
                },
            ),
        }
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
/// inherits an off-plane boundary edge. A boolean match reads the
/// segment's locus instead ([`SegmentEdge::Locus`]).
///
/// Past `he`, the split's join's key, its edge and curve are links,
/// mid-operation ([`crate::live::OPERATORS_KEEP_LINKS`]): a miss panics,
/// and a torn curve is not null scaffolding.
fn between_edge_is_section<T: Decide>(
    body: &Body<T>,
    ctx: &SectionCtx<T>,
    he: HalfEdgeKey,
    band: Band,
) -> Result<Option<bool>, SplitJoinError> {
    let he_data = body.get_half_edge(he).ok_or_else(|| corrupt_he(he))?;
    let edge = linked(
        &body.edges,
        he_data.edge,
        EntityId::Edge,
        EntityId::HalfEdge(he),
        "edge",
    );
    let CurveGeom::Certified(curve) = body.edge_curve_linked(he_data.edge, edge) else {
        return Ok(Some(true)); // null scaffolding: zero-length, ON
    };
    match curve.carrier() {
        geom::Curve3::Line { .. } => Ok(Some(true)),
        geom::Curve3::Spiric { .. } | geom::Curve3::Nurbs(_) => {
            Err(SplitJoinError::RunEdgeCarrierUnsupported {
                face: body
                    .get_loop(he_data.parent_loop)
                    .map(|l| l.face)
                    .ok_or_else(|| corrupt_loop(he_data.parent_loop))?,
                edge: body
                    .split_root(he_data.edge, |_| false)
                    .map_err(|_| corrupt_edge(he_data.edge))?,
            })
        }
        geom::Curve3::Circle { .. } | geom::Curve3::Ellipse { .. } => {
            let margin = Margin::of((curve.mid_point() - ctx.origin).dot(ctx.normal.get()));
            match decide("split_conic_inplane_mid", margin, band) {
                Ok(Sign::Zero) => Ok(Some(true)),
                Ok(Sign::Positive | Sign::Negative) => Ok(Some(false)),
                Err(_) => Ok(None),
            }
        }
    }
}

/// The **exact** chart-azimuth extent of a pcurve over `[t₀, t₁]`.
///
/// On a cylinder chart [`geom_brep::chart_pcurve`] produces an azimuth
/// channel that is exactly `α + β·t` (it writes `pa.x = pb.x = 0` and
/// `pl.x = β ∈ {−1, 0, +1}` in both of its arms), so the two endpoint
/// evaluations ARE the range — this is closed-form structure, not a
/// sampled bound. A focal section's azimuth is strictly
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
/// window refuse" would have been false. The caller reads `None` by the
/// image: a projected image (a circle's lane choice on a sphere, a
/// spline's only image) refuses `RunImageNotYet`, and a spiric image
/// `SectionInvariant`. A fitted image never reaches this function:
/// `chart_pcurve` derives none.
fn chart_azimuth_range<T: Real>(p: &Pcurve<T>, t0: T, t1: T) -> Option<(T, T)> {
    p.closed_form_span_box(t0, t1).map(|b| (b.u_min, b.u_max))
}

/// One boundary half-edge's chart azimuth image, on the branch the
/// walk pinned: the azimuth where the walk ENTERS it and where it
/// EXITS it (the half-edge's own start and end), and the hull
/// [`face_azimuth_window`] folds.
#[derive(Clone, Debug)]
pub(crate) struct AzimuthImage<T: geom_core::Real> {
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
    /// Its edge's certified carrier, as the walk read it.
    pub(crate) carrier: geom::Curve3<T>,
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
/// [`SplitJoinError::Corrupt`] where `face`, the caller's key, does not
/// resolve; otherwise the walk's geometric refusals: a run edge with no
/// closed-form chart image, a fitted image, a vertex off the carrier, an
/// apex junction ([`SplitJoinError::ApexUnlifted`]), or an escalation.
///
/// # Panics
///
/// On a torn hop past `face` ([`outer_cycle`], [`run_azimuth_images`]).
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
///
/// `face` is the caller's key, and [`SplitJoinError::Corrupt`] where it
/// does not resolve. Past it the outer loop is a link and its walk
/// closes, on a body at rest by tier 1 and mid-operation by
/// [`crate::live::OPERATORS_KEEP_LINKS`]: a miss panics, and the walk's
/// members are proven for [`run_azimuth_images`].
fn outer_cycle<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
) -> Result<Option<Vec<HalfEdgeKey>>, SplitJoinError> {
    let outer = body.get_face(face).ok_or_else(|| corrupt_face(face))?.outer;
    let LoopBoundary::Cycle { first } = linked(
        &body.loops,
        outer,
        EntityId::Loop,
        EntityId::Face(face),
        "outer",
    )
    .boundary
    else {
        return Ok(None);
    };
    Ok(Some(body.loop_walk(first).closed("loop", first)))
}

/// **How a run edge's chart-image refusal ends at the join** (D3): a
/// class reading or a lane not built "not yet" ([`SplitJoinError::RunImageNotYet`]);
/// an incidence refusal by its read, through the one map
/// ([`SplitJoinError::RunImageIncidence`]); another escalation as the
/// join's; anything else an invariant the shipped carriers keep.
fn run_image_refusal(
    face: FaceKey,
    kind: geom::SurfaceKind,
    e: geom_brep::PcurveCertifyError,
) -> SplitJoinError {
    match e {
        // The class reading is a lane choice (D3): in band, and
        // with no general lane written, it ends "not yet".
        geom_brep::PcurveCertifyError::Escalated {
            check: geom_brep::PcurveCheck::ChartClass | geom_brep::PcurveCheck::GeneralLane,
            ..
        }
        | geom_brep::PcurveCertifyError::NoGeneralLane { .. } => {
            SplitJoinError::RunImageNotYet { face, kind }
        }
        // An incidence refusal ends by its read, through the one
        // map, never as an invariant break or the join's own lever.
        refusal if geom_brep::IncidenceRead::of(&refusal).is_some() => {
            SplitJoinError::RunImageIncidence {
                face,
                kind,
                refusal,
            }
        }
        geom_brep::PcurveCertifyError::Escalated { cause, .. } => {
            SplitJoinError::Escalated { face, diag: cause }
        }
        _ => SplitJoinError::SectionInvariant {
            face,
            what: "a run edge has no closed-form chart image on the divided face's chart",
        },
    }
}

/// The walk behind [`face_azimuth_images`]:
/// each charted half-edge's image, its branch pinned to the previous
/// edge's exit.
///
/// `halves` are members of a loop walk the caller closed, so each is
/// proven, and its edge and that edge's curve are links: a miss panics
/// (tier 1 at rest, [`crate::live::OPERATORS_KEEP_LINKS`]
/// mid-operation), and only null scaffolding is skipped.
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
    let mut prev_edge: Option<EdgeKey> = None;
    let mut prev_k = 0;
    for &he in halves {
        let he_data = proven(&body.half_edges, he, EntityId::HalfEdge);
        let edge = linked(
            &body.edges,
            he_data.edge,
            EntityId::Edge,
            EntityId::HalfEdge(he),
            "edge",
        );
        let CurveGeom::Certified(curve) = body.edge_curve_linked(he_data.edge, edge) else {
            continue; // null scaffolding: zero-length, no azimuth extent
        };
        let (t0, t1) = curve.params();
        let base = chart_pcurve_over(curve.carrier(), t0, t1, surface, band)
            .map_err(|e| run_image_refusal(face, surface.kind(), e))?;
        let plus = crate::pcurves::is_plus(body, he);
        let (entry_t, exit_t) = if plus { (t0, t1) } else { (t1, t0) };
        let pcurve = match prev_exit {
            None => base,
            Some(prev) => {
                let raw = base.eval(entry_t).x;
                // The branch pin. Default: nearest-branch continuity —
                // the PR 6 loop-walk rule, exact wherever the previous
                // edge's exit and this edge's entry share a chart
                // point, decided as structure ([`nearest_branch`]).
                //
                // A sphere POLE carries no azimuth continuity (the
                // chart is singular there, and two meridians half a
                // period apart make the nearest pin a tie), so the
                // face's orientation picks the branch ([`pole_branch`]).
                let mut pole_k = None;
                // A cone APEX junction carries no azimuth at all, and
                // unlike a sphere pole no orientation bit recovers it:
                // the jump there is the whole lift's, which no pin
                // crosses. A walk that would is refused rather than
                // pinned.
                if let geom::Surface::Cone { apex, .. } = surface {
                    let entry_v = he_data.start;
                    let p = vertex_point(body, entry_v);
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
                    let p = vertex_point(body, entry_v);
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
                            let prior: &AzimuthImage<T> =
                                images.last().ok_or(SplitJoinError::SectionInvariant {
                                    face,
                                    what: "a pole joint has no previous image",
                                })?;
                            pole_k = Some(pole_branch(
                                PoleJoint {
                                    face,
                                    surface: (*center, *radius, *axis),
                                    pole_height: d,
                                    advancing,
                                    gap: prev - raw,
                                    prev_k,
                                    same_edge: prev_edge == Some(he_data.edge),
                                },
                                (&prior.carrier, prior.t.1, prior.t.0),
                                (curve.carrier(), entry_t, exit_t),
                                band,
                            )?);
                        }
                    }
                }
                let k = match pole_k {
                    Some(k) => k,
                    None => nearest_branch(
                        surface,
                        face,
                        vertex_point(body, he_data.start),
                        prev - raw,
                        band,
                    )?,
                };
                prev_k = k;
                base.shift_branch(k, geom_brep::Period::new(tau))
            }
        };
        // A general-lane image (a circle's class read in band on a sphere,
        // or a spline's only image: the projected lane) has no closed-form
        // azimuth box: the walk's lane for it is not written, which is no
        // invariant break. The other image the derivation above returns
        // without a box is a spiric carrier's exact `Pcurve::Spiric` (a
        // torus wall's plane section); it ends `SectionInvariant` today,
        // though it is one of the chart's own curves and the walk's lane
        // for it is simply not written
        // (`work/pcert/a-spiric-run-edge-ends-the-window-walk-as-an-invariant.md`).
        // A fitted image never comes out of that derivation.
        let (lo, hi) = chart_azimuth_range(&pcurve, t0, t1).ok_or(match &pcurve {
            Pcurve::Projected(_) | Pcurve::General(_) => SplitJoinError::RunImageNotYet {
                face,
                kind: surface.kind(),
            },
            _ => SplitJoinError::SectionInvariant {
                face,
                what: "a boundary edge's chart image has no closed-form azimuth box (a spiric \
                       image) — the window walk reads a closed-form azimuth",
            },
        })?;
        let exit = pcurve.eval(exit_t).x;
        images.push(AzimuthImage {
            entry: pcurve.eval(entry_t).x,
            exit,
            range: (lo, hi),
            v: (pcurve.eval(entry_t).y, pcurve.eval(exit_t).y),
            t: (entry_t, exit_t),
            carrier: curve.carrier().clone(),
            harmonic: matches!(pcurve, Pcurve::Harmonic { .. }).then(|| pcurve.clone()),
        });
        prev_exit = Some(exit);
        prev_edge = Some(he_data.edge);
    }
    Ok(images)
}

/// **The nearest branch, as structure**: the whole number of periods
/// `k` that lands an image's entry within half a period of the previous
/// exit, `gap` apart, at the joint's `vertex`. Decided by sign decisions
/// at the half-period marks, metered at the vertex's own lever
/// (`pcurves::joint_arm`), with the loop walk's near-pole read where the
/// marks miss (`pcurves::lever_periods`), so `k` is a literal at every
/// scalar. The walk answers a sphere pole by orientation and refuses a
/// cone apex before it gets here; a vertex decided clear of the apex can
/// still have a lever under the band (`d·sin α` on a narrow cone), which
/// the near-pole read serves.
///
/// # Errors
///
/// [`SplitJoinError::Escalated`] where a mark is in band, or the gap
/// sits on a mark with a branch the near-pole read tries in band;
/// [`SplitJoinError::SectionInvariant`] where the gap sits on a mark off
/// every branch that read tries, or the chart
/// is the placeholder.
fn nearest_branch<T: Decide>(
    surface: &geom::Surface<T>,
    face: FaceKey,
    vertex: geom_core::Point3<T>,
    gap: T,
    band: Band,
) -> Result<i32, SplitJoinError> {
    let chart = crate::pcurves::DescribedChart::of(surface).ok_or(SplitJoinError::SectionInvariant {
        face,
        what: "a divided face's chart is the placeholder — it has no lever to decide a branch by",
    })?;
    let arm = crate::pcurves::joint_arm(chart, vertex);
    crate::pcurves::lever_periods(
        "split_window_branch",
        arm,
        gap,
        geom_brep::Period::new(T::tau()),
        crate::pcurves::Orbit::Periods,
        band,
    )
    .map_err(|miss| match miss {
        geom_brep::BranchMiss::Undecided(diag) => SplitJoinError::Escalated { face, diag },
        geom_brep::BranchMiss::OnMark => SplitJoinError::SectionInvariant {
            face,
            what: "a run edge's chart entry sits half a period from the previous edge's exit — \
                   the two images name no common branch",
        },
    })
}

/// A run vertex on a sphere pole, as [`pole_branch`] reads it.
struct PoleJoint<T: Decide> {
    face: FaceKey,
    /// The sphere's centre, radius and axis.
    surface: (geom_core::Point3<T>, T, geom_core::Vec3<T>),
    /// The pole's axial offset from the centre: `±radius`.
    pole_height: T,
    /// Whether azimuth advances through this pole on the face's walk.
    advancing: bool,
    /// The previous exit less this entry's unshifted azimuth.
    gap: T,
    /// The previous image's branch.
    prev_k: i32,
    /// Whether the walk leaves the pole along the edge it arrived on.
    same_edge: bool,
}

/// **The strict branch at a sphere pole**: the chart carries no azimuth
/// continuity there, so the face's orientation carries the bit. Walking
/// an outward face's cycle with the face on the left, azimuth ADVANCES
/// through the south pole (the entry is the unique branch in
/// `(prev, prev + τ)`) and RETURNS through the north pole
/// (`(prev − τ, prev)`); a reversed face swaps the poles' roles.
///
/// The interval is open, and its ends are where the two edges leave the
/// pole along one direction. That is structure, never a number:
/// - the walk leaves along the edge it arrived on, so the face wraps the
///   pole: the previous branch one whole turn on, `prev_k ± 1`, a
///   literal at every scalar;
/// - two distinct edges: [`geom_brep::Period::decide`] of
///   `gap ± τ/2`, its marks at whole periods, metered by D4 ¶1's
///   parallelism lever ([`pole_lever`]). A mark decided Zero is two
///   distinct edges leaving along one direction, an overlap or a cusp,
///   and refuses ([`SplitJoinError::PoleEdgesCoincide`]); one in band
///   escalates.
///
/// `arrived` and `leaving` are the two edges' carriers with the
/// parameters of each one's pole end and far end.
///
/// Where the lever is so short that both marks either side of the gap
/// lie under the escalation band, which is read first decides between
/// the refusal and the escalation; both fail loud.
fn pole_branch<T: Decide>(
    joint: PoleJoint<T>,
    arrived: (&geom::Curve3<T>, T, T),
    leaving: (&geom::Curve3<T>, T, T),
    band: Band,
) -> Result<i32, SplitJoinError> {
    let PoleJoint {
        face,
        surface,
        pole_height,
        advancing,
        gap,
        prev_k,
        same_edge,
    } = joint;
    let step = if advancing { 1 } else { -1 };
    if same_edge {
        return Ok(prev_k + step);
    }
    let lever = match (
        pole_lever(arrived, surface, pole_height),
        pole_lever(leaving, surface, pole_height),
    ) {
        (Some(a), Some(b)) => a.min(b),
        _ => {
            return Err(SplitJoinError::SectionInvariant {
                face,
                what: "an edge at a sphere pole is not a circle — the pole's branch has no \
                       lever to decide it by",
            });
        }
    };
    let half = T::tau() * T::from_f64(0.5);
    let shifted = if advancing { gap + half } else { gap - half };
    // **What the lever is.** Not a rate either edge has throughout: a
    // point's azimuth rate is its distance from the axis, zero at the pole
    // and at most the edge's reach, so over either edge the lever is that
    // rate's sup, and its inf is zero. What the marks decide is the two
    // edges' largest separation, which on the meridians that reach here is
    // the chord `2ρ·sin(g/2)` at the reach `ρ` both share: `g·lever`
    // overstates it by a relative `g²/24` at most, and where `g·ρ` is
    // within a few bands of a mark that is a few bands squared over `ρ²`,
    // beneath any band's resolution. So the lever is
    // the decision's inf to that order, and the decision is the side of
    // the mark the pair's separation lies on. It is no inf on a curved
    // edge bending back (the lens, [`pole_lever`]'s four times), which
    // never gets here: a non-meridian circle's chart image is projected,
    // and the walk refuses it as FITTED.
    geom_brep::Period::new(T::tau())
        .decide(
            "split_sphere_pole_branch",
            shifted,
            |g| Margin::metered(g, InfSpeed::new(lever)),
            band,
        )
        .map_err(|stop| match stop.miss {
            geom_brep::BranchMiss::Undecided(diag) => SplitJoinError::Escalated { face, diag },
            geom_brep::BranchMiss::OnMark => SplitJoinError::PoleEdgesCoincide { face },
        })
}

/// **The length over which an angle at a sphere pole displaces an
/// edge**: the edge's furthest distance from the axis, capped at its
/// curvature radius. The ruling's "relative curvature radius" is read as
/// the edge's own `1/κ_g`, its geodesic curvature against the sphere,
/// which a meridian lacks.
///
/// The pole is where the edge's circle peaks in axial height, so the
/// height falls monotonically for half the circle from it. The arc's
/// furthest point from the axis is therefore where it has swept half its
/// circle from the pole, `pole_t ± π`, or its far end `far_t` if that
/// comes first; and it is the sphere's radius where that point lies
/// across the equator. An edge whose far end is back at the pole (a
/// closed circle, a near-full sweep) reads its antipode, not its end.
///
/// **What it bounds.** Two meridians `θ` apart separate by at most
/// `2ρ·sin(θ/2) ≤ θ·ρ` at reach `ρ`, so for them the meter `θ·lever`
/// is exact to first order. Against a curved edge bending back toward
/// its partner (the lens between a meridian and a small circle) it
/// overstates the pair's largest separation about four times
/// (`work/pcert/the-pole-lever-overstates-a-lens-about-four-times.md`),
/// which a band with `K` under 4 would decide apart. No answer rides on
/// that today: a non-meridian circle's chart image is projected, and the
/// walk refuses it as FITTED.
///
/// `None` for a carrier other than a circle, the only one a sphere
/// section draws through a pole.
fn pole_lever<T: Decide>(
    (carrier, pole_t, far_t): (&geom::Curve3<T>, T, T),
    (center, radius, axis): (geom_core::Point3<T>, T, geom_core::Vec3<T>),
    pole_height: T,
) -> Option<T> {
    let geom::Curve3::Circle { radius: r, .. } = *carrier else {
        return None;
    };
    let swept = (far_t - pole_t).max(T::zero() - T::pi()).min(T::pi());
    let w = carrier.eval(pole_t + swept) - center;
    let height = w.dot(axis);
    let far = (w - axis * height).norm();
    let reach = (height * pole_height).select_le_zero(radius, far);
    let bend = (radius.powi(2) - r.powi(2)).max(T::zero()).sqrt();
    Some((reach * bend - r * radius).select_le_zero(reach, r * radius / bend))
}

/// `∫ (v − anchor) du` of a harmonic chart image from `t0` to `t1`, and
/// an upper bound on `∫ |dv|` there; `None` for an image that is not
/// harmonic. The anchor is subtracted from the constant term before
/// integrating, so a short island far along the axis does not cancel
/// `v·Δu` against `v·Δu` (the precision note at `geom_brep::props`'
/// cylinder Green form, which this shares).
///
/// Read for a cylinder chart, whose azimuth is an angle channel
/// ([`geom_brep::Channel::Angle`]): `u = phase + w·t` with no harmonic
/// part by type, so `du = w·dt`, and `v` is
/// `p0.y + pa.y·cos t + pb.y·sin t + pl.y·t`, which integrates term by
/// term.
///
/// # Errors
///
/// [`SplitJoinError::SectionInvariant`] for a harmonic image whose
/// azimuth is a free channel, which is no cylinder chart's.
fn chart_v_du<T: Decide>(
    face: FaceKey,
    p: &Pcurve<T>,
    (t0, t1): (T, T),
    anchor: T,
) -> Result<Option<(T, T)>, SplitJoinError> {
    let Pcurve::Harmonic { u, v } = *p else {
        return Ok(None);
    };
    if u.winding().is_none() {
        return Err(SplitJoinError::SectionInvariant {
            face,
            what: "a cylinder chart image's azimuth is a free channel, and the chart's \
                   azimuth is an angle",
        });
    }
    let (p0, pa, pb, pl) = geom_brep::harmonic_form_of(u, v);
    // `sup |x·cos t + y·sin t| = √(x² + y²)`, spelled as
    // `geom_brep::harmonic` spells it, whose helpers are crate-private.
    let pair = |_site: &str, x: T, y: T| {
        let norm = (x.powi(2) + y.powi(2)).sqrt();
        #[cfg(test)]
        let norm = mutants::pair(_site, x, y).unwrap_or(norm);
        norm
    };
    let v_var = (pair("topo-v-var", pa.y, pb.y) + pl.y.abs()) * (t1 - t0).abs();
    let half = T::from_f64(0.5);
    let dt = t1 - t0;
    let integral = (p0.y - anchor) * dt + pa.y * (t1.sin() - t0.sin())
        - pb.y * (t1.cos() - t0.cos())
        + pl.y * (t1 + t0) * dt * half;
    Ok(Some((pl.x * integral, v_var)))
}

/// [`chart_v_du`]'s harmonic pairs read as the sum of their
/// coefficients again, one site at a time: a test build takes the sum
/// at the site `PCERT_RADIAL_PAIR_MUTANT` names at compile time
/// ([`mutants::SITES`]; `option_env!` because
/// `scripts/gates/no-ambient-env.sh` forbids a runtime read). The rows
/// that refuse each are in `v_du_rows`.
#[cfg(test)]
mod mutants {
    use geom_core::Real;

    pub(super) const SITES: [&str; 1] = ["topo-v-var"];

    pub(super) fn pair<T: Real>(site: &str, x: T, y: T) -> Option<T> {
        debug_assert!(SITES.contains(&site), "an unlisted mutant site: {site}");
        (option_env!("PCERT_RADIAL_PAIR_MUTANT") == Some(site)).then(|| x.abs() + y.abs())
    }
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

/// The refusal of a ring-lane reading the lane has no arm for
/// ([`SplitJoinError::RingIslandUnread`]).
fn ring_island_unread<T: Real>(face: FaceKey, surface: &geom::Surface<T>) -> SplitJoinError {
    SplitJoinError::RingIslandUnread {
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
/// A sphere, a cone or a torus face winds without a chart
/// ([`path_island_winding`]), and so does a cylinder face a carried
/// chord closes; any other curved kind refuses typed
/// ([`ring_island_unread`]).
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
    } = wall_chart(&surface).ok_or_else(|| ring_island_unread(face, &surface))?;
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
            Some(p) => chart_v_du(face, p, image.t, anchor)?,
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

/// The loop arc of a closed-form edge on `quadric` over `params`; `None`
/// on a carrier its loops do not carry: on a sphere a line, an ellipse
/// or a spiric, on a cylinder or a cone a spiric, on a torus any but a
/// meridian, a parallel ([`Quadric::torus_circle_arc`]) or a spiric oval
/// ([`Quadric::torus_spiric_arc`]). A spline is read with its evidence
/// of lying on the face ([`SplineArc`], [`run_loop_arcs`]), never here.
fn quadric_loop_arc<T: Decide>(
    quadric: &Quadric<T>,
    carrier: &geom::Curve3<T>,
    params: (T, T),
    band: Band,
) -> Result<Option<LoopArc<T>>, Indeterminate> {
    Ok(match (quadric, carrier) {
        (Quadric::Sphere { .. }, geom::Curve3::Circle { .. })
        | (
            Quadric::Cylinder { .. } | Quadric::Cone { .. },
            geom::Curve3::Circle { .. } | geom::Curve3::Ellipse { .. } | geom::Curve3::Line { .. },
        ) => LoopArc::of(carrier, params),
        (Quadric::Torus { .. }, geom::Curve3::Spiric { .. }) => {
            quadric.torus_spiric_arc(carrier, params)
        }
        (Quadric::Torus { .. }, _) => quadric.torus_circle_arc(carrier, params, band)?,
        _ => None,
    })
}

/// The loop arcs of the half-edges `run` on the face `face` of
/// `surface`: a null edge (a pierce point's scaffolding, no length)
/// contributes none, and an edge on a carrier the surface's loops do
/// not carry ([`quadric_loop_arc`]) is a ring the lane does not read
/// ([`SplitJoinError::RingIslandUnread`]). A spline edge is read on a
/// cylinder, a sphere or a torus face where it is the certified
/// intersection of the face's surface with another
/// ([`SplineArc::of_edge`]), and is unread on a cone, whose pairs the
/// section table does not carry.
fn run_loop_arcs<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    (surface, quadric): (&geom::Surface<T>, &Quadric<T>),
    run: &[HalfEdgeKey],
    band: Band,
) -> Result<Vec<LoopArc<T>>, SplitJoinError> {
    let own = body
        .get_face(face)
        .ok_or_else(|| corrupt_face(face))?
        .surface;
    let mut arcs = Vec::with_capacity(run.len());
    for &he in run {
        let edge = body.get_half_edge(he).ok_or_else(|| corrupt_he(he))?.edge;
        let data = body.get_edge(edge).ok_or_else(|| corrupt_edge(edge))?;
        let Some(curve) = body.edge_curve_linked(edge, data).certified() else {
            continue;
        };
        let arc = match (quadric, curve.carrier()) {
            (
                Quadric::Cylinder { .. } | Quadric::Sphere { .. } | Quadric::Torus { .. },
                geom::Curve3::Nurbs(_),
            ) => SplineArc::of_edge(curve, own).map(LoopArc::Spline),
            (Quadric::Cone { .. }, geom::Curve3::Nurbs(_)) => None,
            _ => quadric_loop_arc(quadric, curve.carrier(), curve.params(), band)
                .map_err(|diag| SplitJoinError::Escalated { face, diag })?,
        };
        arcs.push(arc.ok_or_else(|| ring_island_unread(face, surface))?);
    }
    Ok(arcs)
}

/// The reference points a path may run to from `face`'s outer loop:
/// its vertices, then the midpoints of its edges other than `skip`. A
/// midpoint lies off a run that shares its edge's vertices but not the
/// edge, where every vertex reads on the run.
fn outer_references<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    skip: &[EdgeKey],
) -> Result<Vec<Point3<T>>, SplitJoinError> {
    let outer = body.get_face(face).ok_or_else(|| corrupt_face(face))?.outer;
    let mut points = loop_points(body, outer)?;
    for he in outer_cycle(body, face)?.unwrap_or_default() {
        let edge = body.get_half_edge(he).ok_or_else(|| corrupt_he(he))?.edge;
        if skip.contains(&edge) {
            continue;
        }
        let data = body.get_edge(edge).ok_or_else(|| corrupt_edge(edge))?;
        if let Some(curve) = body.edge_curve_linked(edge, data).certified() {
            let (t0, t1) = curve.params();
            points.push(curve.carrier().mid_point(t0, t1));
        }
    }
    Ok(points)
}

/// The points of a loop's vertices, in cycle order (an empty loop's
/// lone vertex).
fn loop_points<T: Decide>(body: &Body<T>, l: LoopKey) -> Result<Vec<Point3<T>>, SplitJoinError> {
    Ok(ring_vertices(body, l)?
        .into_iter()
        .map(|v| vertex_point(body, v))
        .collect())
}

/// **The winding of a ring-lane island on a cylinder, a sphere, a cone or
/// a torus face**,
/// about the face's OUTWARD normal, read without a chart: whether the
/// open run `h1 → h2` (`next` order, through `h2`), closed by `closing`
/// (the chord from `h2`'s site back to `h1`'s; `None`, the straight
/// chord, which only a cone's ruling is), bounds on its left the patch
/// holding none of the face's outer loop — the island, since the ring is
/// a hole and the run's left is the face's side of it.
///
/// A path ([`Quadric::paths`]) runs from an outer-loop point `w` (a
/// vertex, or an edge midpoint: [`outer_references`]) to the closing
/// chord's midpoint `q`, where it arrives across the chord. Its
/// crossings of the curve before `q` ([`crate::ring_path::path_parity`])
/// say whether `w` lies on the side it arrives from, and the arrival
/// says which side that is: the left one exactly when the path arrives
/// against the chord's left normal `N × t` (`N` the outward normal at
/// `q`, `t` the chord's direction of travel), the lean
/// **`split_ring_path_lean`** (the cosine between the arrival and the
/// left normal, levered at [`Quadric::lever`]). CCW is `w` off the left.
///
/// Positive for CCW, as the cylinder arm's chart sign. A path whose lean
/// or crossings land in the zero band or the escalation band says
/// nothing, and the next path or `w` is asked: the parity is the same on
/// every path, so the first decided one is the reading. The first
/// escalation escalates only where no path decides. A run
/// bounded by an edge the face's loops do not carry, or a chord through
/// a cone's apex, is a ring the lane does not read
/// ([`SplitJoinError::RingIslandUnread`]); an outer loop no point of
/// which a path decides refuses typed.
pub(crate) fn path_island_winding<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    run: (HalfEdgeKey, HalfEdgeKey),
    closing: Option<&EdgeCurveSpec<T>>,
    band: Band,
) -> Result<Result<Sign, Indeterminate>, SplitJoinError> {
    path_island_winding_closed_by(body, face, run, (closing, None), band)
}

/// [`path_island_winding`] closed by `closing`, whose arc, where it is a
/// spline, is `arc`: the carried chord's, with its evidence of lying on
/// the face ([`SplineArc::of_section`]).
pub(crate) fn path_island_winding_closed_by<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    run: (HalfEdgeKey, HalfEdgeKey),
    closing: (Option<&EdgeCurveSpec<T>>, Option<&SplineArc<T>>),
    band: Band,
) -> Result<Result<Sign, Indeterminate>, SplitJoinError> {
    let island = match IslandReading::closed_by(body, face, run, closing, band)? {
        Ok(island) => island,
        Err(diag) => return Ok(Err(diag)),
    };
    let readings = island
        .paths(body, face, band)?
        .into_iter()
        .map(|path| Ok::<_, SplitJoinError>(path.and_then(|path| island.read(path, band))));
    match first_decided(readings)? {
        Ok(Some(sign)) => Ok(Ok(sign)),
        Err(diag) => Ok(Err(diag)),
        Ok(None) => Err(SplitJoinError::SectionInvariant {
            face,
            what: "no outer-loop point of a cylinder, sphere, cone or torus face reads which side \
                   of a ring-lane island it is on (every path to the closing chord meets the \
                   island's boundary in the zero band)",
        }),
    }
}

/// [`path_island_winding`]'s question, asked of one path at a time: the
/// run's loop arcs closed by the chord, the chord's midpoint `q` the
/// paths arrive at, and its left normal there.
pub(crate) struct IslandReading<T: Real> {
    quadric: Quadric<T>,
    arcs: Vec<LoopArc<T>>,
    q: Point3<T>,
    left: Vec3<T>,
}

impl<T: Decide> IslandReading<T> {
    /// [`Self::closed_by`] with no spline arc.
    #[cfg(test)]
    pub(crate) fn new(
        body: &Body<T>,
        face: FaceKey,
        run: (HalfEdgeKey, HalfEdgeKey),
        closing: Option<&EdgeCurveSpec<T>>,
        band: Band,
    ) -> Result<Result<Self, Indeterminate>, SplitJoinError> {
        Self::closed_by(body, face, run, (closing, None), band)
    }

    /// The reading of the run `h1 → h2` closed by `closing`, whose arc,
    /// where it is a spline, is `arc` ([`path_island_winding_closed_by`]);
    /// an escalation where `q` is not decided off a cone's apex or a
    /// torus's axis.
    pub(crate) fn closed_by(
        body: &Body<T>,
        face: FaceKey,
        (h1, h2): (HalfEdgeKey, HalfEdgeKey),
        (closing, arc): (Option<&EdgeCurveSpec<T>>, Option<&SplineArc<T>>),
        band: Band,
    ) -> Result<Result<Self, Indeterminate>, SplitJoinError> {
        let invariant = |what| SplitJoinError::SectionInvariant { face, what };
        let face_data = body.get_face(face).ok_or_else(|| corrupt_face(face))?;
        let surface = body
            .get_surface(face_data.surface)
            .ok_or_else(|| corrupt_face(face))?;
        let quadric = Quadric::of(surface).ok_or_else(|| ring_island_unread(face, surface))?;
        let cycle = body.loop_cycle(h1).ok_or_else(|| corrupt_he(h1))?;
        let end = cycle
            .iter()
            .position(|&he| he == h2)
            .ok_or_else(|| corrupt_he(h2))?;
        let mut arcs = run_loop_arcs(body, face, (surface, &quadric), &cycle[..=end], band)?;
        let (chord, q, travel) = match closing {
            Some(spec) => {
                let (t0, t1) = (spec.param_start, spec.param_end);
                let chord = match &spec.carrier {
                    // The chord's locus, read in either sense: the arc
                    // its evidence was taken on.
                    geom::Curve3::Nurbs(_) => arc.cloned().map(LoopArc::Spline),
                    _ => quadric_loop_arc(&quadric, &spec.carrier, (t0, t1), band)
                        .map_err(|diag| SplitJoinError::Escalated { face, diag })?,
                }
                .ok_or_else(|| ring_island_unread(face, surface))?;
                let travel = spec.carrier.deriv(geom::mid_param(t0, t1)) * (t1 - t0);
                (chord, spec.carrier.mid_point(t0, t1), travel)
            }
            None if matches!(quadric, Quadric::Sphere { .. } | Quadric::Torus { .. }) => {
                return Err(invariant(
                    "a sphere or torus ring run is closed by a straight chord (the section lanes \
                     mint an arc on a sphere or a torus)",
                ));
            }
            None => {
                let after = cycle[(end + 1) % cycle.len()];
                let at = |he: HalfEdgeKey| {
                    Ok::<_, SplitJoinError>(vertex_point(
                        body,
                        body.get_half_edge(he).ok_or_else(|| corrupt_he(he))?.start,
                    ))
                };
                let (from, to) = (at(after)?, at(h1)?);
                (
                    LoopArc::segment(from, to),
                    from + (to - from) * T::from_f64(0.5),
                    to - from,
                )
            }
        };
        match quadric.off_apex(q, band) {
            Ok(true) => {}
            Ok(false) => return Err(ring_island_unread(face, surface)),
            Err(diag) => return Ok(Err(diag)),
        }
        arcs.push(chord);
        let left = quadric.outward(q, face_data.sense).cross(travel);
        Ok(Ok(Self {
            quadric,
            arcs,
            q,
            left,
        }))
    }

    /// The paths to `q` from every outer-loop point of `face`
    /// ([`outer_references`]), in the order they are asked.
    pub(crate) fn paths(
        &self,
        body: &Body<T>,
        face: FaceKey,
        band: Band,
    ) -> Result<Vec<Result<Path<T>, Indeterminate>>, SplitJoinError> {
        Ok(outer_references(body, face, &[])?
            .into_iter()
            .flat_map(|w| quadric_paths(&self.quadric, (w, self.q), band))
            .collect())
    }

    /// The winding `path` reads: its lean against the left normal
    /// (**`split_ring_path_lean`**) and its parity across the closed
    /// curve; `None` where either is in the zero band.
    pub(crate) fn read(&self, path: Path<T>, band: Band) -> Result<Option<Sign>, Indeterminate> {
        let lean = decide(
            "split_ring_path_lean",
            Margin::levered(
                -path.arrival.dot(self.left) / self.left.norm(),
                self.quadric.lever(self.q),
            ),
            band,
        )?;
        if lean == Sign::Zero {
            return Ok(None);
        }
        let arrives_on = self.arcs.len() - 1;
        Ok(
            path_parity(&path, &self.arcs, Some(arrives_on), band)?.map(|odd| {
                let w_left = (lean == Sign::Positive) != odd;
                if w_left {
                    Sign::Negative
                } else {
                    Sign::Positive
                }
            }),
        )
    }
}

/// **The first decided reading** of `readings`, asked in order and
/// lazily. Every reading answers one question — which side of a curve a
/// point lies on, the same whatever path or point reads it — so one that
/// says nothing (`Ok(None)`: the zero band) or escalates moves to the
/// next. The first escalation escalates only where no reading decides; a
/// hard error stops the walk.
///
/// **The premise**: the decided readings agree. For the winding's
/// references and a ring's paths it holds by construction (each reads one
/// point's side). For a ring's vertices it holds because the ring does
/// not cross the curve it is read against: a ring crossing it would have
/// decided vertices on both sides, and the walk would answer with the
/// first. That is the premise `validate::ring_nesting` rests on, checked
/// by check 9's contact arms on `Line` and `Circle` edges and assumed on
/// `Ellipse`, `Spiric` and NURBS ones
/// (`work/restfront/check-9-meeting-arms-silent-off-a-plane-and-on-ellipse-spiric-nurbs-edges.md`).
/// Under it, which decided reading is taken changes nothing.
fn first_decided<R, D, E>(
    readings: impl IntoIterator<Item = Result<Result<Option<R>, D>, E>>,
) -> Result<Result<Option<R>, D>, E> {
    let mut escalated = None;
    for reading in readings {
        match reading? {
            Ok(Some(r)) => return Ok(Ok(Some(r))),
            Ok(None) => {}
            Err(diag) => {
                escalated.get_or_insert(diag);
            }
        }
    }
    Ok(escalated.map_or(Ok(None), Err))
}

/// The paths [`Quadric::paths`] offers between two points, each as a
/// reading of [`first_decided`]'s: an escalation choosing them is one
/// escalated reading.
fn quadric_paths<T: Decide>(
    quadric: &Quadric<T>,
    ends: (Point3<T>, Point3<T>),
    band: Band,
) -> Vec<Result<Path<T>, Indeterminate>> {
    match quadric.paths(ends, band) {
        Ok(paths) => paths.into_iter().map(Ok).collect(),
        Err(diag) => vec![Err(diag)],
    }
}

/// [`ring_side`] on a cylinder, a sphere, a cone or a torus face, without a chart:
/// whether `ring` lies inside `newf`'s outer loop (the run a `mef` just
/// walled off `oldf`), by the parity of a path ([`Quadric::paths`]) from a ring
/// vertex to a point outside the new face: a vertex of `oldf`'s outer
/// loop, or the midpoint of one of its edges the run does not share,
/// which lies off the run even where every vertex of that loop is a
/// copy of a run vertex ([`crate::ring_path::path_parity`]). A path with
/// a reading in the zero band says nothing and the next path or pair is
/// asked: one ending on the run does, so a ring vertex on the run never
/// decides. One with a reading in the escalation band says nothing
/// either; the first escalation escalates only where no pair decides.
/// The decided pairs agree because the ring does not cross the run
/// ([`first_decided`]'s premise). A ring no pair decides otherwise is
/// [`RingSide::Undecided`]. The parity does not depend on the path on any
/// of these kinds (`crate::ring_path`'s module docs: on a cylinder, an
/// annulus, two closed curves cross an even number of times), so a run
/// edge of any carrier the lane reads, a spline chord's included, is read
/// as a loop arc and never through a chart image.
fn path_ring_side<T: Decide>(
    body: &Body<T>,
    (surface, quadric): (&geom::Surface<T>, &Quadric<T>),
    (oldf, newf): (FaceKey, FaceKey),
    ring: LoopKey,
    band: Band,
) -> Result<RingSide, SplitJoinError> {
    let cycle = outer_cycle(body, newf)?.ok_or(SplitJoinError::SectionInvariant {
        face: newf,
        what: "ring re-homing on a curved face: the run is not a cycle",
    })?;
    let arcs = run_loop_arcs(body, newf, (surface, quadric), &cycle, band)?;
    let run_edges = cycle
        .iter()
        .map(|&he| Ok(body.get_half_edge(he).ok_or_else(|| corrupt_he(he))?.edge))
        .collect::<Result<Vec<_>, SplitJoinError>>()?;
    let outside = outer_references(body, oldf, &run_edges)?;
    let paths = loop_points(body, ring)?.into_iter().flat_map(|p| {
        outside
            .iter()
            .flat_map(move |&w| quadric_paths(quadric, (p, w), band))
    });
    let read = |path: Result<Path<T>, Indeterminate>| {
        Ok::<_, SplitJoinError>(path.and_then(|path| path_parity(&path, &arcs, None, band)))
    };
    match first_decided(paths.map(read))? {
        Ok(Some(odd)) => Ok(if odd { RingSide::In } else { RingSide::Out }),
        Ok(None) => Ok(RingSide::Undecided),
        Err(diag) => Err(SplitJoinError::Escalated { face: newf, diag }),
    }
}

/// **The chord of a section segment that is an edge of this solid**
/// ([`JoinLane::AlongEdge`], `segment` naming that edge): the other
/// copy of the edge, so its curve is the edge's own, from `u1` to `u2`
/// (the edge's endpoints' copies, at its endpoints' points) — never a
/// section the lane would compute from the germ's face pair, which may
/// be two faces on one carrier, or meet in no conic. A line is the
/// straight chord; a circle is its own arc; any other curve is the
/// edge's own spec. Either is run back where the chord runs against the
/// edge ([`run_back`]), and a description no chord states is re-stated
/// in the chart of the face it divides ([`chord_description`]) (a
/// closed edge's copy is [`closed_edge_spec`]'s, in every lane). The boolean takes this lane on both
/// solids for an edge of both, and on the edge's solid alone for a
/// conic edge of one lying in a face of the other whose pair has no
/// section lane (a tube's rim on a ball). `None` on every other lane:
/// there a segment along an edge of one solid lies in a face of the
/// other, whose lane computes the section the chord takes (the rod's
/// ruling, a lens rim on a wall).
///
/// `segment`, `u1` and `u2` are keys the join carries, so one that no
/// longer resolves refuses typed. Past them, the ends' points and the
/// segment's curve and halves are links, mid-operation
/// ([`crate::live::OPERATORS_KEEP_LINKS`]): a miss panics, and a torn
/// curve is not an uncertified one.
fn along_edge_spec<T: Decide>(
    body: &Body<T>,
    lane: &JoinLane<'_, T>,
    segment: Option<EdgeKey>,
    face: FaceKey,
    u1: VertexKey,
    u2: VertexKey,
) -> Result<Option<EdgeCurveSpec<T>>, SplitJoinError> {
    let (JoinLane::AlongEdge, Some(edge)) = (lane, segment) else {
        return Ok(None);
    };
    let point = |v: VertexKey| {
        body.get_vertex(v)
            .map(|d| body.point_of(v, d))
            .ok_or(SplitJoinError::SectionInvariant {
                face,
                what: "a chord end along the segment's edge no longer resolves",
            })
    };
    let (p1, p2) = (point(u1)?, point(u2)?);
    let e = body
        .get_edge(edge)
        .ok_or(SplitJoinError::SectionInvariant {
            face,
            what: "the segment's edge no longer resolves",
        })?;
    let CurveGeom::Certified(curve) = body.edge_curve_linked(edge, e) else {
        return Err(SplitJoinError::SectionInvariant {
            face,
            what: "the segment's edge carries no certified curve",
        });
    };
    if let geom::Curve3::Line { .. } = curve.carrier() {
        return Ok(Some(EdgeCurveSpec::line_between(p1, p2)));
    }
    // The chord starts at the copy of whichever end of the edge null
    // edges tie `u1` to; the curve runs from its `he_plus` start.
    let e_start = linked(
        &body.half_edges,
        e.he_plus,
        EntityId::HalfEdge,
        EntityId::Edge(edge),
        "he_plus",
    )
    .start;
    let e_end = body.proven_half_edge_end(e.he_plus);
    let forward = match (
        same_site(body, face, (u1, e_start))?,
        same_site(body, face, (u1, e_end))?,
    ) {
        (true, false) => true,
        (false, true) => false,
        _ => {
            return Err(SplitJoinError::SectionInvariant {
                face,
                what: "a chord end along the segment's edge is tied to neither or both of the \
                       edge's ends",
            });
        }
    };
    let surface = body
        .get_face(face)
        .ok_or_else(|| corrupt_face(face))?
        .surface;
    let copy = match curve.carrier() {
        geom::Curve3::Circle { .. } => {
            let (t0, t1) = curve.params();
            EdgeCurveSpec::arc_of_circle(curve.carrier().clone(), t0, t1).ok_or(
                SplitJoinError::SectionInvariant {
                    face,
                    what: "a circle segment edge's arc does not state",
                },
            )?
        }
        _ => curve.restated_spec(),
    };
    if forward {
        Ok(Some(EdgeCurveSpec {
            description: chord_description(&copy.description, face, Some(surface))?,
            ..copy
        }))
    } else {
        run_back(&copy, face, Some(surface)).map(Some)
    }
}

/// **A segment's chord along a closed edge** ([`JoinPlan`]'s `closed`):
/// the edge's whole turn, in every lane — the segment is the edge, so no
/// section is asked — run the way the chord's `mef` `site` needs
/// ([`closed_chord_runs_forward`]). It is stated in the chart of the
/// face it divides, on which it lies: a whole-turn scaffold circle reads
/// as a null edge's placeholder (`containment::carrier_loop`).
fn closed_edge_spec<T: Decide>(
    body: &Body<T>,
    edge: EdgeKey,
    face: FaceKey,
    site: ChordSite,
) -> Result<Option<EdgeCurveSpec<T>>, SplitJoinError> {
    let invariant = |what| SplitJoinError::SectionInvariant { face, what };
    let e = body
        .get_edge(edge)
        .ok_or(invariant("the segment's edge no longer resolves"))?;
    let CurveGeom::Certified(curve) = body.edge_curve_linked(edge, e) else {
        return Err(invariant("the segment's edge carries no certified curve"));
    };
    let geom::Curve3::Circle { .. } = curve.carrier() else {
        return Err(invariant("a closed segment edge that is not a circle"));
    };
    let forward = closed_chord_runs_forward(body, edge, site, face)?;
    let surface = body
        .get_face(face)
        .ok_or_else(|| corrupt_face(face))?
        .surface;
    let (t0, t1) = curve.params();
    let turn = EdgeCurveSpec {
        description: geom_brep::EdgeDescriptionSpec::chart(surface),
        ..EdgeCurveSpec::arc_of_circle(curve.carrier().clone(), t0, t1).ok_or(invariant(
            "a closed circle segment edge's turn does not state",
        ))?
    };
    if forward {
        Ok(Some(turn))
    } else {
        run_back(&turn, face, Some(surface)).map(Some)
    }
}

/// **Whether `edge` is closed at the site of `u`**: both of its ends
/// are copies of that site ([`same_site`]). The one test of a closed
/// segment edge ([`JoinPlan::of`]).
fn closed_at<T: Decide>(
    body: &Body<T>,
    edge: EdgeKey,
    u: VertexKey,
    face: FaceKey,
) -> Result<bool, SplitJoinError> {
    let Some((a, b)) = body.edge_vertices(edge) else {
        return Err(SplitJoinError::SectionInvariant {
            face,
            what: "the segment's edge no longer resolves",
        });
    };
    Ok(same_site(body, face, (u, a))? && same_site(body, face, (u, b))?)
}

/// **Which way a closed edge's chord runs**, where both ends of the
/// segment's `edge` are the chord's one site: the chord is the whole
/// circle, and `mef` decides only which loop each of its halves joins
/// ([`MefSite::Chords`]: the minus half bounds the new face, the run
/// from `he1` up to `he2`, and the plus half, which runs along the
/// spec, stays in the old loop). The edge's own half is the one between
/// the null edge's two halves. The loop that keeps it bounds no area:
/// it is the edge's half and the chord's beside it, and, where the
/// chord's site is the copy across the null edge from the edge's
/// vertex, the null edge's two halves too. So the chord's half there
/// runs against the edge's half. The spec therefore runs along the
/// edge's half where that half is in the new face's run, and against it
/// where it stays in the old loop: forward along the carrier where that
/// agrees with the half being `he_plus`.
fn closed_chord_runs_forward<T: Decide>(
    body: &Body<T>,
    edge: EdgeKey,
    site: ChordSite,
    face: FaceKey,
) -> Result<bool, SplitJoinError> {
    let invariant = |what| SplitJoinError::SectionInvariant { face, what };
    let ChordSite::Mef(MefSite::Chords { he1, he2 }) = site else {
        return Err(invariant(
            "a closed edge's chord planned at a site that is not two halves of one loop",
        ));
    };
    let e = body.get_edge(edge).ok_or_else(|| corrupt_edge(edge))?;
    let loop_of = |he: HalfEdgeKey| {
        body.get_half_edge(he)
            .map(|h| h.parent_loop)
            .ok_or_else(|| corrupt_he(he))
    };
    let split = loop_of(he1)?;
    // The edge's half in the split loop between the null edge's two
    // halves: a wrap edge has both of its halves in that loop.
    let between_nulls = |h: HalfEdgeKey| -> Result<bool, SplitJoinError> {
        let d = body.get_half_edge(h).ok_or_else(|| corrupt_he(h))?;
        let (prev, next) = (d.prev, d.next);
        let edge_of = |x: HalfEdgeKey| {
            body.get_half_edge(x)
                .map(|x| x.edge)
                .ok_or_else(|| corrupt_he(x))
        };
        let null = edge_of(next)?;
        let is_null = body
            .get_edge(null)
            .is_some_and(|n| body.edge_curve_linked(null, n).null_scaffold().is_some());
        Ok(d.parent_loop == split && is_null && edge_of(prev)? == null)
    };
    let own = match (between_nulls(e.he_plus)?, between_nulls(e.he_minus)?) {
        (true, false) => e.he_plus,
        (false, true) => e.he_minus,
        _ => {
            return Err(invariant(
                "a closed edge's chord splits a loop holding other than one of its halves \
                 between the null edge's",
            ));
        }
    };
    // The run from `he1` up to `he2`, empty where they are one half.
    let mut in_run = false;
    let mut he = he1;
    for _ in 0..=body.half_edges.len() {
        if he == he2 {
            return Ok(in_run == (own == e.he_plus));
        }
        in_run |= he == own;
        he = body.get_half_edge(he).ok_or_else(|| corrupt_he(he))?.next;
    }
    Err(invariant("a closed edge's chord run never reaches its end"))
}

/// **The site of a vertex**: every vertex null edges tie it to, itself
/// included — the copies one crossing site was split into, however
/// many null edges it holds. A null edge's two ends are the same point,
/// so the site is one point held by several vertices.
///
/// The split states the same relation a second way once its null edges
/// are gone: each copy's null-pair row names the vertex it was minted
/// at, and [`crate::splitting::SplitNaming::original_of`] chases a copy
/// to the operand vertex. Two vertices one site ties here are two that
/// read back to one original there.
///
/// Runs on bodies mid-operation. [`StaleSite`]: a vertex of the site
/// does not resolve, one of `from`, the caller's keys, or a copy a null
/// edge's attribute names, whose currency is the minting and consuming
/// operators' and no tier-1 rule's ([`crate::null::NullEdge`]), so
/// neither is proven. Past a vertex that resolves, its orbit's edges
/// ([`Body::edges_of_vertex_linked`]) and their curves are links, and a
/// miss panics ([`crate::live::OPERATORS_KEEP_LINKS`]).
pub(crate) fn null_site<T: Decide>(
    body: &Body<T>,
    from: &[VertexKey],
) -> Result<Vec<VertexKey>, StaleSite> {
    let mut site: Vec<VertexKey> = from.to_vec();
    let mut i = 0;
    while i < site.len() {
        let v = site[i];
        body.get_vertex(v).ok_or(StaleSite)?;
        for k in body.edges_of_vertex_linked(v) {
            let e = proven(&body.edges, k, EntityId::Edge);
            let Some(attr) = body.edge_curve_linked(k, e).null_scaffold() else {
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
    Ok(site)
}

/// A site holds a vertex that does not resolve ([`null_site`]): a key
/// its caller carried, or a copy a null edge's attribute names. Either
/// is the minting and consuming operators' bookkeeping, so it is a
/// kernel bug, refused typed under each caller's invariant kind with
/// [`StaleSite::WHAT`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct StaleSite;

impl StaleSite {
    /// The refusal text every reader of a site gives it.
    pub(crate) const WHAT: &'static str = "a null site holds a vertex that no longer resolves";
}

/// Where one of [`ChordJoiner::join`]'s two chords is minted: the
/// `mef` or `mekr` site, the matched half whose site its spec runs
/// from, and its two end vertices as that site names them.
struct ChordPlan {
    site: ChordSite,
    from: HalfEdgeKey,
    ends: (VertexKey, VertexKey),
}

/// A chord's Euler site.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ChordSite {
    /// A same-loop `mef`.
    Mef(MefSite),
    /// A cross-loop `mekr`.
    Mekr(MekrSite),
}

/// What a [`JoinPlan`] reads of `(h1, h2)`:
/// whether they share a loop, and whether `h2 → between → h1` holds.
struct JoinShape {
    same_loop: bool,
    prev_adjacent: bool,
}

/// Whether the edge under a half IS the section segment: the adjacency
/// skip's question ([`SegmentEdge::is`]).
type SkipTest<'s, T> = &'s mut dyn FnMut(&Body<T>, HalfEdgeKey) -> Result<bool, SplitJoinError>;

fn join_shape<T: Decide>(
    body: &Body<T>,
    (h1, h2): (HalfEdgeKey, HalfEdgeKey),
) -> Result<JoinShape, SplitJoinError> {
    let half = |he: HalfEdgeKey| body.get_half_edge(he).ok_or_else(|| corrupt_he(he));
    let same_loop = half(h1)?.parent_loop == half(h2)?.parent_loop;
    Ok(JoinShape {
        same_loop,
        prev_adjacent: same_loop && half(half(h1)?.prev)?.prev == h2,
    })
}

/// The first chord's plan, or `None` where the adjacency skip drops
/// it. Same loop: `mef(Chords { he1: h1, he2: next(h2) })`, from `h1`.
/// Two loops: the `mekr` that kills the loop which is not the face's
/// outer (both rings: `h2`'s, the book's order), from the target's
/// site.
fn first_chord<T: Decide>(
    body: &Body<T>,
    (h1, h2): (HalfEdgeKey, HalfEdgeKey),
    face: FaceKey,
    shape: &JoinShape,
    skip: SkipTest<'_, T>,
) -> Result<Option<ChordPlan>, SplitJoinError> {
    let half = |he: HalfEdgeKey| body.get_half_edge(he).ok_or_else(|| corrupt_he(he));
    let (d1, d2) = (half(h1)?, half(h2)?);
    if shape.same_loop {
        if shape.prev_adjacent && skip(body, d1.prev)? {
            return Ok(None);
        }
        return Ok(Some(ChordPlan {
            site: ChordSite::Mef(MefSite::Chords {
                he1: h1,
                he2: d2.next,
            }),
            from: h1,
            ends: (d1.start, half(d2.next)?.start),
        }));
    }
    let outer = body.get_face(face).ok_or_else(|| corrupt_face(face))?.outer;
    let (target, ring, from) = if d2.parent_loop == outer {
        (d2.next, h1, h2)
    } else {
        (h1, d2.next, h1)
    };
    Ok(Some(ChordPlan {
        site: ChordSite::Mekr(MekrSite::Cycles { target, ring }),
        from,
        ends: (half(target)?.start, half(ring)?.start),
    }))
}

/// The second chord's plan on the body as it stands — after the first
/// chord's surgery, or before it where the first was skipped — or `None`
/// where the adjacency skip drops it: `mef(Chords { he1: h2, he2:
/// next(h1) })`, from `h2`: the first chord's segment taken the other
/// way (the null edges are zero-length).
fn second_chord<T: Decide>(
    body: &Body<T>,
    (h1, h2): (HalfEdgeKey, HalfEdgeKey),
    skip: SkipTest<'_, T>,
) -> Result<Option<ChordPlan>, SplitJoinError> {
    let half = |he: HalfEdgeKey| body.get_half_edge(he).ok_or_else(|| corrupt_he(he));
    let d1 = half(h1)?;
    let adjacent2 = half(d1.next)?.next == h2;
    if adjacent2 && skip(body, d1.next)? {
        return Ok(None);
    }
    Ok(Some(ChordPlan {
        site: ChordSite::Mef(MefSite::Chords {
            he1: h2,
            he2: d1.next,
        }),
        from: h2,
        ends: (half(h2)?.start, half(d1.next)?.start),
    }))
}

/// **A join's two chords, planned once** — each one's site, end
/// vertices and the half its curve runs from, or `None` where the
/// adjacency skip drops it — read by everything that curves or mints
/// them: [`ChordJoiner::segment_curve`], [`ChordJoiner::join`] and the
/// boolean's role order ([`Self::sites`]).
///
/// Both chords are planned on the body before any surgery. The first
/// chord's surgery rewires only `prev(h1)` and `next(h2)`; the second's
/// plan reads `next(h1)`, the edge under it, and the half after it,
/// which is `next(h2)` only where `next(h1)` is `h2` itself — not
/// adjacent before the surgery or after it.
pub(crate) struct JoinPlan {
    halves: (HalfEdgeKey, HalfEdgeKey),
    /// The first half's face, which the plan, the curve and the join
    /// all read.
    face: FaceKey,
    first: Option<ChordPlan>,
    second: Option<ChordPlan>,
    /// The edge the segment is, where a locus names one.
    locus: Option<EdgeKey>,
    /// That edge, where it is closed at the site of the first half's
    /// start ([`closed_at`]): its chord is the whole edge
    /// ([`closed_edge_spec`]), and a ring that holds nothing but it and
    /// the null edge joins as a lone ring ([`lone_ring`]).
    closed: Option<EdgeKey>,
}

impl JoinPlan {
    /// The plan for joining `halves` in that order, `segment` answering
    /// the adjacency skip.
    pub(crate) fn of<T: Decide>(
        body: &Body<T>,
        halves: (HalfEdgeKey, HalfEdgeKey),
        segment: SegmentEdge<'_, T>,
        band: Band,
    ) -> Result<Self, SplitJoinError> {
        // The second half can sit on another face: a boolean match can
        // take a germ's half from a sector on a face its ends do not
        // share (`a-boolean-match-takes-a-half-from-a-sector-on-a-face-its-ends-do-not-share`, JOIN, closed by PR 4364).
        // Such a plan stays on the first half's face, and what runs on
        // it answers: the curve's lane there, or the `mekr` across the
        // two faces (`NotSameFace`).
        let face = he_face(body, halves.0)?;
        let shape = join_shape(body, halves)?;
        let locus = segment.locus();
        let closed = match locus {
            Some(edge) => {
                let u = body
                    .get_half_edge(halves.0)
                    .ok_or_else(|| corrupt_he(halves.0))?
                    .start;
                closed_at(body, edge, u, face)?.then_some(edge)
            }
            None => None,
        };
        let mut skip = |b: &Body<T>, he| segment.is(b, he, face, band);
        Ok(Self {
            halves,
            face,
            first: first_chord(body, halves, face, &shape, &mut skip)?,
            second: second_chord(body, halves, &mut skip)?,
            locus,
            closed,
        })
    }

    /// The halves joined, in the plan's order.
    pub(crate) fn halves(&self) -> (HalfEdgeKey, HalfEdgeKey) {
        self.halves
    }

    /// The first half's face, the one the plan is made on.
    pub(crate) fn face(&self) -> FaceKey {
        self.face
    }

    /// The chords' sites, `(first, second)`.
    pub(crate) fn sites(&self) -> (Option<ChordSite>, Option<ChordSite>) {
        (
            self.first.as_ref().map(|p| p.site),
            self.second.as_ref().map(|p| p.site),
        )
    }
}

/// What a ring a one-site section loop's null edge lies in holds
/// besides that edge ([`lone_ring`]).
#[derive(Clone, Copy, PartialEq, Eq)]
enum LoneRing {
    /// Nothing: a pierce of the face's interior, joined to itself.
    Pierce,
    /// The segment's own closed edge: the loop runs along an edge of the
    /// face's ring, whose ends are both the site.
    Closed,
}

/// The ring `halves` make up, where they are the two halves of one null
/// edge and their loop is a ring of its face holding nothing else, or
/// nothing else but a half of the segment's `closed` edge
/// ([`ChordJoiner::join_lone_ring`]).
fn lone_ring<T: Real>(
    body: &Body<T>,
    (h1, h2): (HalfEdgeKey, HalfEdgeKey),
    closed: Option<EdgeKey>,
) -> Result<Option<(LoopKey, LoneRing)>, SplitJoinError> {
    let half = |he: HalfEdgeKey| body.get_half_edge(he).ok_or_else(|| corrupt_he(he));
    let (d1, d2) = (half(h1)?, half(h2)?);
    if d1.edge != d2.edge || d1.parent_loop != d2.parent_loop {
        return Ok(None);
    }
    let kind = if d1.next == h2 && d2.next == h1 {
        LoneRing::Pierce
    } else {
        // One of the two runs between the halves is empty and the other
        // is the closed edge alone.
        let between =
            |from: &crate::entity::HalfEdge, to: HalfEdgeKey| -> Result<bool, SplitJoinError> {
                let mid = half(from.next)?;
                Ok(Some(mid.edge) == closed && mid.next == to)
            };
        let closed = (d1.next == h2 && between(d2, h1)?) || (d2.next == h1 && between(d1, h2)?);
        if !closed {
            return Ok(None);
        }
        LoneRing::Closed
    };
    let l = d1.parent_loop;
    let face = body.get_loop(l).ok_or_else(|| corrupt_loop(l))?.face;
    let outer = body.get_face(face).ok_or_else(|| corrupt_face(face))?.outer;
    Ok((l != outer).then_some((l, kind)))
}

/// **How [`ChordJoiner::join_lone_ring`] reads which side of a
/// walled-off face's conic its region lies on**, by the pierced face's
/// surface ([`Self::of`]). A face it has no arm for (a cone, a cylinder,
/// a torus, which a plane can cut in more than the conic) is not joined
/// at a one-site pierce: `boolean::join`'s `closes_at_its_site` reads
/// this same table.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum RimSide {
    /// A plane: a conic loop winding with the old face's outer loop, on
    /// the plane's chart normal, bounds the disc inside it, and one
    /// winding against it the old face's region outside it.
    Plane,
    /// A sphere, cut by the conic's plane in the conic alone, into two
    /// caps. The old face's outer loop does not meet the conic, so it
    /// lies in one cap, read at its first point definitely off the
    /// plane. A loop's region is the cap its left hand points into
    /// (`n × t` at its start, `n` the face's outward normal and `t` the
    /// loop's direction).
    Sphere,
}

impl RimSide {
    /// The arm for a face on `surface`, if there is one.
    pub(crate) fn of<T: Real>(surface: &geom::Surface<T>) -> Option<Self> {
        match surface {
            geom::Surface::Plane { .. } => Some(Self::Plane),
            geom::Surface::Sphere { .. } => Some(Self::Sphere),
            _ => None,
        }
    }

    /// Whether the region of `face`, walled off by one conic, holds the
    /// outer loop `outer` of `oldf`, the face the conic pierced.
    fn holds_outer<T: Decide + crate::props::AtRestPolicy>(
        self,
        body: &Body<T>,
        oldf: FaceKey,
        outer: LoopKey,
        face: FaceKey,
        band: Band,
    ) -> Result<bool, SplitJoinError> {
        let invariant = |what| SplitJoinError::SectionInvariant { face: oldf, what };
        let escalated = |diag| SplitJoinError::Escalated { face: oldf, diag };
        let face_data = body.get_face(face).ok_or_else(|| corrupt_face(face))?;
        let winding = |l: LoopKey, normal: Vec3<T>| match body.planar_loop_winding(l, normal, band)
        {
            Some(Ok(sign @ (Sign::Positive | Sign::Negative))) => Ok(sign),
            Some(Err(diag)) => Err(escalated(diag)),
            _ => Err(invariant(
                "a one-site loop's conic in a pierce ring does not wind definitely on its plane",
            )),
        };
        match self {
            Self::Plane => {
                let normal = face_plane_normal(body, oldf)?;
                Ok(winding(face_data.outer, normal)? != winding(outer, normal)?)
            }
            Self::Sphere => {
                let Some(&[he]) = outer_cycle(body, face)?.as_deref() else {
                    return Err(invariant(
                        "a one-site loop's walled-off face is bounded by other than one conic",
                    ));
                };
                let h = body.get_half_edge(he).ok_or_else(|| corrupt_he(he))?;
                let edge = body.get_edge(h.edge).ok_or_else(|| corrupt_edge(h.edge))?;
                let curve = body
                    .edge_curve_linked(h.edge, edge)
                    .certified()
                    .ok_or(invariant(
                        "a one-site loop's conic carries no certified curve",
                    ))?;
                let geom::Curve3::Circle {
                    center,
                    axis,
                    radius,
                    ..
                } = *curve.carrier()
                else {
                    return Err(invariant(
                        "a one-site loop's conic on a sphere is not a circle",
                    ));
                };
                let (t0, _) = curve.params();
                let forward = if he == edge.he_plus {
                    T::one()
                } else {
                    -T::one()
                };
                let tangent = curve.carrier().deriv(t0) * (forward / radius);
                let p = vertex_point(body, h.start);
                let normal = crate::face_normal::face_outward_normal_at(body, face, p, band)
                    .map_err(|_| invariant("a one-site loop's site reads no normal on its sphere"))?
                    .ok_or_else(|| corrupt_face(face))?;
                let left = normal.vec().cross(tangent).dot(axis);
                let region = decide(
                    "bool_lone_ring_region_side",
                    Margin::levered(left, radius),
                    band,
                )
                .map_err(escalated)?;
                let mut reads = loop_points(body, outer)?.into_iter().map(|w| {
                    decide(
                        "bool_lone_ring_outer_side",
                        Margin::of((w - center).dot(axis)),
                        band,
                    )
                });
                // The first point read definitely off the plane decides;
                // an escalation escalates only where none does.
                let mut first_escalation = None;
                let outer_side = loop {
                    match reads.next() {
                        Some(Ok(sign @ (Sign::Positive | Sign::Negative))) => break sign,
                        Some(Ok(Sign::Zero)) => {}
                        Some(Err(diag)) => {
                            first_escalation.get_or_insert(diag);
                        }
                        None => {
                            return Err(match first_escalation {
                                Some(diag) => escalated(diag),
                                None => invariant(
                                    "the old face's outer loop lies on its pierce ring's plane",
                                ),
                            });
                        }
                    }
                };
                match region {
                    Sign::Zero => Err(invariant(
                        "a one-site loop's conic on a sphere leaves its plane's sides undecided",
                    )),
                    sign => Ok(sign == outer_side),
                }
            }
        }
    }
}

/// A boolean match's adjacency skip: the between edge is the edge the
/// segment's locus names.
fn between_is_segment<T: Real>(
    body: &Body<T>,
    segment: Option<EdgeKey>,
    between: HalfEdgeKey,
) -> Result<bool, SplitJoinError> {
    let between = body
        .get_half_edge(between)
        .ok_or_else(|| corrupt_he(between))?;
    Ok(segment == Some(between.edge))
}

impl ChordJoiner {
    /// The [`JoinPlan`] for joining `halves` in that order.
    pub(crate) fn plan<T: Decide>(
        &self,
        body: &Body<T>,
        halves: (HalfEdgeKey, HalfEdgeKey),
        segment: SegmentEdge<'_, T>,
    ) -> Result<JoinPlan, SplitJoinError> {
        JoinPlan::of(body, halves, segment, self.band)
    }

    /// **The segment's chord curve** ([`SegmentCurve`]) for a planned
    /// join, computed once, in the lane the caller selects: the edge's
    /// own copy on [`JoinLane::AlongEdge`] ([`along_edge_spec`]), else
    /// the section chord [`chord_spec`] mints in the plan's face. Both
    /// lanes that join — a boolean match and the plane split — take
    /// their chords' geometry here.
    ///
    /// It is computed for the chord the plan mints first: from that
    /// chord's end, leaving it along `leave`. The curve is computed only
    /// for a chord that is minted, so an aux surface it mints is always
    /// referenced; `None` is a join whose two chords are both skipped,
    /// which mints nothing.
    pub(crate) fn segment_curve<T: Decide>(
        &self,
        body: &mut Body<T>,
        plan: &JoinPlan,
        lane: JoinLane<'_, T>,
        leave: Leave<T>,
    ) -> Result<Option<SegmentCurve<T>>, SplitJoinError> {
        let (halves, face) = (plan.halves, plan.face);
        let Some(chord) = plan.first.as_ref().or(plan.second.as_ref()) else {
            return Ok(None);
        };
        let to = if chord.from == halves.0 {
            halves.1
        } else {
            halves.0
        };
        let (u1, u2) = chord.ends;
        let section = match plan.closed {
            Some(edge) => closed_edge_spec(body, edge, face, chord.site)?.map(SectionChord::bare),
            None => match along_edge_spec(body, &lane, plan.locus, face, u1, u2)? {
                Some(spec) => Some(SectionChord::bare(spec)),
                None => chord_spec(
                    body,
                    self.band,
                    lane,
                    face,
                    u1,
                    u2,
                    leave.from(chord.from, face)?,
                )?,
            },
        };
        let (spec, ends, arc) =
            section.map_or((None, None, None), |c| (Some(c.spec), c.ends, c.arc));
        Ok(Some(SegmentCurve {
            arc,
            halves: (chord.from, to),
            spec,
            ends,
        }))
    }

    /// `join` (module docs): connect the old loose end `h1` and the
    /// new half `h2` — `plan`'s halves — with the chords `plan` mints,
    /// both on `curve` (the segment's, [`Self::segment_curve`]); the
    /// minted chord edges come back (the boolean joining records their
    /// germ — M3 PR 5).
    pub(crate) fn join<T: Decide + crate::props::AtRestPolicy>(
        &mut self,
        body: &mut Body<T>,
        plan: &JoinPlan,
        curve: &SegmentCurve<T>,
        tol: Tol,
    ) -> Result<Vec<EdgeKey>, SplitJoinError> {
        if let Some(ring) = lone_ring(body, plan.halves, plan.closed)? {
            return self.join_lone_ring(body, plan, ring, curve, tol);
        }
        let (h2, oldf) = (plan.halves.1, plan.face);
        let mut minted = Vec::new();
        let mut newf = None;
        if let Some(chord) = &plan.first {
            let spec = curve.running_from(chord.from, oldf)?;
            let refused = curve.refusal(body, oldf, chord.ends, spec.as_ref(), self.band);
            match chord.site {
                ChordSite::Mef(site) => {
                    // `outside` is the first half past the run; after the
                    // mef its parent loop is the split's REMAINDER — the
                    // loop ring re-homing must skip (a ring-lane remainder
                    // is geometrically coincident with the run and would
                    // land OnBoundary; issue #93).
                    let MefSite::Chords { he2: outside, .. } = site else {
                        return Err(SplitJoinError::SectionInvariant {
                            face: oldf,
                            what: "a same-loop chord planned at a site that is not two halves",
                        });
                    };
                    // Both arms hand `mef` the parent's surface, so the
                    // fragment takes `oldf`'s bit (`Body::resolve_face_surface`).
                    // Guard: sweep's `m5_s12_curved_ops.rs`, the row named
                    // `a_boolean_that_splits_a_reversed_wall_inherits_the_parent_bit`.
                    let created = match spec {
                        None => body.mef_chord(site, tol)?,
                        Some(spec) => body
                            .mef(site, spec, FaceSurface::Inherit, tol)
                            .map_err(refused)?,
                    };
                    self.slivers.insert(created.face, ());
                    self.fragments
                        .push((created.face, oldf))
                        .map_err(SplitJoinError::LineageRow)?;
                    minted.push(created.edge);
                    newf = Some((created.face, outside));
                }
                ChordSite::Mekr(site) => {
                    let made = match spec {
                        None => body.mekr_chord(site, tol)?,
                        Some(spec) => body.mekr(site, spec, tol).map_err(refused)?,
                    };
                    minted.push(made.edge);
                }
            }
        }
        // The second chord divides the face `h2` sits on NOW — after a
        // first mef that is not necessarily `oldf` (`h2` may have landed
        // in the new face). Capture the owner at call time, BEFORE the
        // surgery moves loops.
        if let Some(chord) = &plan.second {
            let owner = he_face(body, h2)?;
            let ChordSite::Mef(site) = chord.site else {
                return Err(SplitJoinError::SectionInvariant {
                    face: owner,
                    what: "a second chord planned as a mekr",
                });
            };
            let spec = curve.running_from(chord.from, owner)?;
            let refused = curve.refusal(body, owner, chord.ends, spec.as_ref(), self.band);
            let created = match spec {
                None => body.mef_chord(site, tol)?,
                Some(spec) => body
                    .mef(site, spec, FaceSurface::Inherit, tol)
                    .map_err(refused)?,
            };
            self.slivers.insert(created.face, ());
            self.fragments
                .push((created.face, owner))
                .map_err(SplitJoinError::LineageRow)?;
            minted.push(created.edge);
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
        Ok(minted)
    }

    /// [`Self::join`] of a one-site section loop whose site is a pierce
    /// of a planar or spherical face's interior: the two halves are one
    /// null edge, and `ring`, a ring of the face, holds nothing else.
    /// Each chord is the whole conic at one copy of the site, and the
    /// face they wall off is the null face, which holds both halves.
    /// `mef` walls off an empty run when its two halves are one, so the
    /// ring is first promoted to a face of its own (`mfkrh`) and joined
    /// there: the promoted face keeps both halves and each chord walls
    /// off a face bounded by one conic. The one of those two whose
    /// region holds the old face's outer loop ([`RimSide`]) bounds the
    /// hole the conic cuts in the old face, and becomes its ring
    /// (`kfmrh_minting`, which re-mints the old face where the hole's
    /// rows do not stand on it); the other is the disc inside the conic, which takes the
    /// old face's rings it encloses.
    fn join_lone_ring<T: Decide + crate::props::AtRestPolicy>(
        &mut self,
        body: &mut Body<T>,
        plan: &JoinPlan,
        (ring, kind): (LoopKey, LoneRing),
        curve: &SegmentCurve<T>,
        tol: Tol,
    ) -> Result<Vec<EdgeKey>, SplitJoinError> {
        let oldf = plan.face;
        let invariant = |what| SplitJoinError::SectionInvariant { face: oldf, what };
        let side = match kind {
            LoneRing::Closed => None,
            LoneRing::Pierce => {
                let surface = body
                    .get_face(oldf)
                    .and_then(|f| body.get_surface(f.surface))
                    .ok_or_else(|| corrupt_face(oldf))?;
                Some(RimSide::of(surface).ok_or(invariant(
                    "a one-site loop's pierce ring on a face no whole conic is sided on",
                ))?)
            }
        };
        let old = body.get_face(oldf).ok_or_else(|| corrupt_face(oldf))?;
        let (outer, sense) = (old.outer, old.sense);
        // The promoted face lies in the old face's region, its material
        // on the same side, where `mfkrh` derives a promoted ring's face
        // as facing the other way. No door states the bit: `mfkrh`
        // refuses a stated sense on the old face's chart that disagrees
        // with the one it derives (`SenseContradictsChart`), so the bit
        // is set after. The loop is the null edge alone, which winds no
        // area to disagree with it, and the seam zip reads each wall-off
        // face's conic against its partner's
        // (`boolean::zip::one_vertex_sense`).
        let promoted = body.mfkrh(ring, FaceSurface::Inherit)?.face;
        body.set_face_sense(promoted, sense)?;
        self.slivers.insert(promoted, ());
        self.fragments
            .push((promoted, oldf))
            .map_err(SplitJoinError::LineageRow)?;
        let replan = JoinPlan::of(body, plan.halves, SegmentEdge::Locus(plan.locus), self.band)?;
        let walled_from = self.fragments.rows().len();
        // The halves' loop is now the promoted face's outer, which
        // `lone_ring` never takes, so this `join` mints its chords: two
        // in a pierce, and in a closed edge's ring the one the edge does
        // not stand in for.
        let minted = self.join(body, &replan, curve, tol)?;
        let walled: Vec<FaceKey> = self.fragments.rows()[walled_from..]
            .iter()
            .map(|&(f, _)| f)
            .collect();
        let (disc, hole) = match (side, &walled[..]) {
            (Some(side), &[a, b]) => match (
                side.holds_outer(body, oldf, outer, a, self.band)?,
                side.holds_outer(body, oldf, outer, b, self.band)?,
            ) {
                (false, true) => (Some(a), b),
                (true, false) => (Some(b), a),
                _ => {
                    return Err(invariant(
                        "a one-site loop's two conics in a pierce ring bound one side alike",
                    ));
                }
            },
            // The chord copies the ring's own edge, and the face it walls
            // off holds it alone: the hole the edge bounded.
            (None, &[hole]) => (None, hole),
            _ => {
                return Err(invariant(
                    "a one-site loop's join in a ring walled off other than its conics",
                ));
            }
        };
        let cut = body.kfmrh_minting(oldf, hole, tol)?.ring;
        self.slivers.remove(hole);
        self.absorbed
            .push((hole, oldf))
            .map_err(SplitJoinError::LineageRow)?;
        if let Some(disc) = disc {
            self.rehome_rings(body, oldf, disc, cut)?;
        }
        Ok(minted)
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
    ///
    /// A pierce ring on a plane every vertex of which is decided ON the
    /// run ([`RingSide::OnRun`]) is a strut at a pinch the run passes
    /// through: no point of it says which side it is on, but its own
    /// polygon will, so it is left pending ([`Self::place_pending`]). Its
    /// point stays on the boundary of the face holding it, so a later
    /// division of that face reads it on the run again or out, and
    /// never moves it. Any other ring on the run, and any ring no vertex
    /// of which is decided (a curved face's ring whose every path ends on
    /// the run or grazes it), refuses.
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
        // How a ring's side is read: off the plane, or by a path on a
        // cylinder, a sphere, a cone or a torus.
        enum Homing<T: geom_core::Real> {
            Plane(Vec3<T>),
            Path(Quadric<T>),
        }
        let homing = match &surface {
            geom::Surface::Plane { .. } => Homing::Plane(face_plane_normal(body, oldf)?),
            other => {
                Homing::Path(Quadric::of(other).ok_or_else(|| ring_island_unread(newf, other))?)
            }
        };
        for ring in rings {
            if ring == remainder {
                continue;
            }
            let side = match &homing {
                Homing::Plane(normal) => ring_side(body, ring, run, *normal, self.band)?,
                Homing::Path(quadric) => {
                    path_ring_side(body, (&surface, quadric), (oldf, newf), ring, self.band)?
                }
            };
            match side {
                RingSide::In => body.ring_move(ring, newf)?,
                RingSide::Out => {}
                RingSide::OnRun if is_pierce_ring(body, ring)? => {
                    self.pending.insert(ring, ());
                }
                RingSide::OnRun | RingSide::Undecided => {
                    return Err(SplitJoinError::RingHomingAmbiguous { ring });
                }
            }
        }
        Ok(())
    }

    /// `cut` (module docs): retire a fully-joined null edge. The
    /// completion outcome comes back unresolved — the caller assigns
    /// roles from its own side data. The kill is the band door, so a
    /// loop it releases from its last null edge is minted at `tol`.
    pub(crate) fn cut_core<T: Decide>(
        &mut self,
        body: &mut Body<T>,
        edge: EdgeKey,
        tol: Tol,
    ) -> Result<CutOutcome, SplitJoinError> {
        let edge_data = body
            .get_edge(edge)
            .ok_or_else(|| corrupt_edge(edge))?
            .clone();
        let (l_plus, f_plus) = he_loop(body, edge_data.he_plus)?;
        let (l_minus, f_minus) = he_loop(body, edge_data.he_minus)?;
        if l_plus == l_minus {
            // The last null edge of a section polygon: kemr leaves the
            // 2-loop null face.
            let result = body.kemr_minting(edge_data.he_plus, edge_data.he_minus, tol)?;
            self.completed.insert(f_plus, ());
            Ok(CutOutcome::Completed {
                face: f_plus,
                ring: result.ring,
            })
        } else {
            // Interior null edge: kef merges the two slivers. Kill a
            // sliver side (never a real face), deterministically
            // preferring he_plus's side.
            if self.completed.contains_key(f_plus) || self.completed.contains_key(f_minus) {
                unreachable!(
                    "{} would kef a completed null face: the edge's sides are the slivers its \
                     own polygon's chords walled off at its two ends, and a completed face is \
                     bounded by another polygon's two copies, which kemr left once that \
                     polygon's last null edge was cut",
                    EntityId::Edge(edge)
                );
            }
            let (victim, survivor) = if self.slivers.contains_key(f_plus) {
                (edge_data.he_plus, f_minus)
            } else if self.slivers.contains_key(f_minus) {
                (edge_data.he_minus, f_plus)
            } else {
                return Err(SplitJoinError::CutInvariant { edge });
            };
            let killed = body.kef_minting(victim, tol)?;
            self.slivers.remove(killed.killed_face);
            self.absorbed
                .push((killed.killed_face, survivor))
                .map_err(SplitJoinError::LineageRow)?;
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
            what: "ring re-homing read a plane off a face whose carrier is not a plane (re-homing \
                   sends every other kind to its own reading)",
        }),
    }
}

/// Whether every edge of `ring` is a null edge: a pierce ring no join
/// has reached yet, whose vertices are all copies of its pierce point.
/// Only the boolean's vertex-on-face insertion mints one; the split's
/// null edges hang off vertices of existing loops.
fn is_pierce_ring<T: Decide>(body: &Body<T>, ring: LoopKey) -> Result<bool, SplitJoinError> {
    let first = match body
        .get_loop(ring)
        .ok_or_else(|| corrupt_loop(ring))?
        .boundary
    {
        LoopBoundary::Cycle { first } => first,
        // A lone vertex: no edge yet, so nothing it is joined by.
        LoopBoundary::Empty { .. } => return Ok(false),
    };
    for he in body.loop_cycle(first).ok_or_else(|| corrupt_he(first))? {
        let edge = body.get_half_edge(he).ok_or_else(|| corrupt_he(he))?.edge;
        let curve = body.get_edge(edge).ok_or_else(|| corrupt_edge(edge))?.curve;
        if body
            .get_curve_geom(curve)
            .and_then(CurveGeom::null_scaffold)
            .is_none()
        {
            return Ok(false);
        }
    }
    Ok(true)
}

/// Which side of `run` a bystander ring lies on, read at its first
/// vertex off `run`'s boundary: a ring disjoint from the run cannot
/// cross it, but it may touch it at a vertex — a pinch, where two
/// sections meet at one point — so its anchor alone can land `OnBoundary`
/// on a ring that is plainly on one side. [`RingSide::OnRun`] only when
/// every vertex does: each such verdict is decided, so the ring is on
/// the run. A vertex whose reading escalates says nothing either, and
/// the next is asked; the first escalation escalates only where no
/// vertex decides ([`first_decided`]). The vertices agree because the
/// ring does not cross the run ([`first_decided`]'s premise).
fn ring_side<T: Decide>(
    body: &Body<T>,
    ring: LoopKey,
    run: LoopKey,
    normal: Vec3<T>,
    band: Band,
) -> Result<RingSide, SplitJoinError> {
    let read = |v: VertexKey| match point_in_loop(body, run, normal, vertex_point(body, v), band) {
        Ok(LoopContainment::In) => Ok(Ok(Some(RingSide::In))),
        Ok(LoopContainment::Out) => Ok(Ok(Some(RingSide::Out))),
        Ok(LoopContainment::OnBoundary) => Ok(Ok(None)),
        Err(e @ PointInLoopError::Escalated { .. }) => Ok(Err(e)),
        Err(e) => Err(SplitJoinError::from(e)),
    };
    match first_decided(ring_vertices(body, ring)?.into_iter().map(read))? {
        Ok(Some(side)) => Ok(side),
        Ok(None) => Ok(RingSide::OnRun),
        Err(e) => Err(SplitJoinError::RingHoming(e)),
    }
}

/// Where ring re-homing puts a bystander ring.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RingSide {
    /// Inside the run: it moves to the new face.
    In,
    /// Outside the run: it stays.
    Out,
    /// Every vertex is ON the run ([`ring_side`]'s decided verdict).
    OnRun,
    /// No vertex was decided: on a curved face, a vertex on the run
    /// says nothing ([`path_ring_side`]).
    Undecided,
}

/// The vertices of `ring`, in cycle order (an empty ring's lone one).
fn ring_vertices<T: Decide>(
    body: &Body<T>,
    ring: LoopKey,
) -> Result<Vec<VertexKey>, SplitJoinError> {
    match body
        .get_loop(ring)
        .ok_or_else(|| corrupt_loop(ring))?
        .boundary
    {
        LoopBoundary::Cycle { first } => body
            .loop_cycle(first)
            .ok_or_else(|| corrupt_he(first))?
            .into_iter()
            .map(|he| Ok(body.get_half_edge(he).ok_or_else(|| corrupt_he(he))?.start))
            .collect(),
        LoopBoundary::Empty { vertex } => Ok(vec![vertex]),
    }
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
    Ok(vertex_point(body, v))
}
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod cone_ring_rows;
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod first_image_rows;
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod seam_rows;
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod section_end_rows;
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod sibling_escalation_rows;
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod sphere_island_rows;
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod torus_chord_rows;
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod torus_ring_rows;
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod v_du_rows;
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod wall_ring_rows;

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
            crate::construction::FaceSurface::New {
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
                crate::construction::MevSite::Lone {
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
            branch: None,
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
                crate::construction::MevSite::Lone {
                    r#loop: seed.r#loop,
                },
                carrier.eval(t1),
                EdgeCurveSpec {
                    description: geom_brep::EdgeDescriptionSpec::Intersection {
                        pair: geom_brep::SurfacePair::new(cyl, plane),
                        selector: geom_brep::CurveSelector::Witness(carrier.mid_point(t0, t1)),
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

    /// **A wall's section reads at its base vertex, wherever the wall's
    /// origin is stored.** Row C's unit wall and the plane `x = 1`
    /// tangent to it along the base vertex's ruling, the axis tilted
    /// half the zero band toward the plane and the wall's origin stored
    /// 1000 m out along it either way. Read at the base vertex's foot,
    /// the tilt is in the zero band at the face extent and the gap is the
    /// radius: the tangent ruling. Read 1000 m away, the gap moved by
    /// `1000·θ`, five hundred times the band: two rulings on one side, no
    /// section on the other.
    #[test]
    fn a_walls_section_reads_at_its_base_vertex_wherever_the_origin_is_stored() {
        let band = geom_core::Band::linear(Tol::witness()).expect("a linear band");
        let base = Point3::new(1.0, 0.0, 0.0);
        let sin_beta: f64 = 0.5 * band.zero();
        let axis = Vec3::new(-sin_beta, 0.0, (1.0 - sin_beta * sin_beta).sqrt());
        let normal = UnitVec3::new(Vec3::new(1.0, 0.0, 0.0), "stored-origin row", band).unwrap();
        for along in [1000.0, -1000.0] {
            let mut body = crate::Body::<f64>::new();
            let seed = body.mvfs(base, true).unwrap();
            body.set_face_surface(
                seed.face,
                crate::construction::FaceSurface::New {
                    surface: geom::Surface::Cylinder {
                        origin: Point3::origin() + axis * along,
                        axis,
                        radius: 1.0,
                        u_ref: Vec3::unit_x(),
                    },
                    sense: true,
                },
            )
            .unwrap();
            body.mev_line(
                crate::construction::MevSite::Lone {
                    r#loop: seed.r#loop,
                },
                Point3::new(1.0, 0.0, 1.0),
                Tol::witness(),
            )
            .unwrap();
            let got = wall_section(&body, band, base, normal, seed.face, seed.vertex);
            assert!(
                matches!(
                    got,
                    Ok(Some(WallSection {
                        case: SectionCase::Tangent(_),
                        ..
                    }))
                ),
                "stored {along} m along: the tangent ruling, got {:?}",
                got.map(|w| w.map(|w| match w.case {
                    SectionCase::Straight(_) => "straight",
                    SectionCase::Tangent(_) => "tangent",
                    SectionCase::Conic(_) => "conic",
                    SectionCase::Loops { .. } => "loops",
                }))
            );
        }
    }

    /// **A face shorter than the radius is levered at its face extent,
    /// not the radius.** Row C's wall and plane with the second vertex at
    /// `(1, 0, h)`, `h < r`, the plane tilted so the axis meets it at
    /// `sin β = k·ε/h`. A tilt moves the section by the tilt times the
    /// AXIAL distance from the base vertex's foot, at most `h`, so
    /// `pc_axis_plane_parallel` reads `k·ε`: in the band, and the table
    /// escalates. Floored at the foot's distance from the vertex (the
    /// radius), the lever read `k·ε·r/h`, definite from `k = 6` at
    /// `h = 0.5` and from `k = 3` at `h = 0.2`, and served a tilted
    /// ellipse. `k = 1.2` reads Zero if the lever is cut below `h`.
    #[test]
    fn a_short_faces_pose_is_levered_at_its_face_extent_not_the_radius() {
        let band = geom_core::Band::linear(Tol::witness()).expect("a linear band");
        let base = Point3::new(1.0, 0.0, 0.0);
        for h in [0.5, 0.2] {
            let mut body = crate::Body::<f64>::new();
            let seed = body.mvfs(base, true).unwrap();
            body.set_face_surface(
                seed.face,
                crate::construction::FaceSurface::New {
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
            body.mev_line(
                crate::construction::MevSite::Lone {
                    r#loop: seed.r#loop,
                },
                Point3::new(1.0, 0.0, h),
                Tol::witness(),
            )
            .unwrap();
            for k in [1.2, 3.0, 6.0, 8.0, 9.0, 9.9] {
                let sin_beta: f64 = k * band.zero() / h;
                let normal = UnitVec3::new(
                    Vec3::new((1.0 - sin_beta * sin_beta).sqrt(), 0.0, sin_beta),
                    "short-face row",
                    band,
                )
                .unwrap();
                let got = wall_section(&body, band, base, normal, seed.face, seed.vertex);
                assert!(
                    matches!(
                        got,
                        Err(SplitJoinError::Escalated { ref diag, .. })
                            if diag.predicate == Some("pc_axis_plane_parallel")
                    ),
                    "h = {h}, k = {k}: an in-band tilt must escalate, got {:?}",
                    got.map(|w| w.map(|w| match w.case {
                        SectionCase::Straight(_) => "straight",
                        SectionCase::Tangent(_) => "tangent",
                        SectionCase::Conic(_) => "conic",
                        SectionCase::Loops { .. } => "loops",
                    }))
                );
            }
        }
    }

    /// **A wall's pose is levered at its face extent, not a ball about
    /// the base vertex** (row C). A unit cylinder face about `z` whose
    /// base vertex `(1, 0, 0)` lies on the ruling the plane `x = 1`
    /// touches, its other vertex `(1, 0, 2)` a face extent of 2 away, past
    /// the radius; the plane through the base vertex tilted so the axis
    /// meets it at `sin β = k·ε/2`. Levered at the face extent,
    /// `pc_axis_plane_parallel` reads `k·ε`, in the band, and the table
    /// escalates. Levered at a ball of that radius about the base vertex,
    /// the axis's foot stood `r` inside it and the lever read
    /// `r + 2 = 3`: `1.5·k·ε`, a definite tilt for `k ≥ 7`, and a tilted
    /// ellipse. `k = 1.2` is in the band too, and reads Zero if the lever
    /// is cut below the face extent (the extent is past the radius, so the
    /// pivot's distance from the vertex, the lever's floor, does not hide
    /// the cut).
    #[test]
    fn a_walls_pose_is_levered_at_its_face_extent() {
        let band = geom_core::Band::linear(Tol::witness()).expect("a linear band");
        let base = Point3::new(1.0, 0.0, 0.0);
        let mut body = crate::Body::<f64>::new();
        let seed = body.mvfs(base, true).unwrap();
        body.set_face_surface(
            seed.face,
            crate::construction::FaceSurface::New {
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
        body.mev_line(
            crate::construction::MevSite::Lone {
                r#loop: seed.r#loop,
            },
            Point3::new(1.0, 0.0, 2.0),
            Tol::witness(),
        )
        .unwrap();
        for k in [1.2, 6.0, 8.0, 9.0, 9.9] {
            let sin_beta: f64 = k * band.zero() / 2.0;
            let normal = UnitVec3::new(
                Vec3::new((1.0 - sin_beta * sin_beta).sqrt(), 0.0, sin_beta),
                "row C",
                band,
            )
            .unwrap();
            let got = wall_section(&body, band, base, normal, seed.face, seed.vertex);
            assert!(
                matches!(
                    got,
                    Err(SplitJoinError::Escalated { ref diag, .. })
                        if diag.predicate == Some("pc_axis_plane_parallel")
                ),
                "k = {k}: an in-band tilt must escalate, got {:?}",
                got.map(|w| w.map(|w| match w.case {
                    SectionCase::Straight(_) => "straight",
                    SectionCase::Tangent(_) => "tangent",
                    SectionCase::Conic(_) => "conic",
                    SectionCase::Loops { .. } => "loops",
                }))
            );
        }
    }

    /// **A wall's pose is levered at its axial extent, its rim's bulge
    /// included, not the distance round it.** A unit wall about `z`
    /// trimmed at `φ = ±45°`, its rim one closed ellipse on the seam vertex
    /// `(1, 0, ±1)` (`oblique_rim_wall`), the rim's highest point or its
    /// lowest, and the plane through the vertex tilted so the axis meets
    /// it at `sin β = k·ε/2`. The rim reaches `2·tan |φ| = 2` down or up
    /// the axis from the vertex's foot, at its low crest or its high one, so
    /// `pc_axis_plane_parallel` reads `k·ε` and escalates at every `k` in
    /// the band. The vertex alone levers nothing and reads Zero (the
    /// tangent ruling); the rim's Euclidean reach from the vertex,
    /// `2/cos φ`, reads `√2·k·ε`, definite from `k = K/√2`, and serves a
    /// tilted ellipse.
    #[test]
    fn a_rims_bulge_levers_the_pose_along_the_axis() {
        let band = geom_core::Band::linear(Tol::witness()).expect("a linear band");
        let quarter = core::f64::consts::FRAC_PI_4;
        for (phi, frac) in [quarter, -quarter]
            .into_iter()
            .flat_map(|phi| [0.12, 0.8, 0.9, 0.99].map(|frac| (phi, frac)))
        {
            let (body, face, vertex) = crate::test_support_fixtures::oblique_rim_wall(phi);
            let base = body.resolve_vertex_point(vertex, Proven);
            let k = frac * Tol::witness().k();
            let sin_beta: f64 = k * band.zero() / 2.0;
            let normal = UnitVec3::new(
                Vec3::new((1.0 - sin_beta * sin_beta).sqrt(), 0.0, sin_beta),
                "rim row",
                band,
            )
            .unwrap();
            let got = wall_section(&body, band, base, normal, face, vertex);
            assert!(
                matches!(
                    got,
                    Err(SplitJoinError::Escalated { ref diag, .. })
                        if diag.predicate == Some("pc_axis_plane_parallel")
                ),
                "φ = {phi}, k = {k}: an in-band tilt over the rim must escalate, got {:?}",
                got.map(|w| w.map(|w| match w.case {
                    SectionCase::Straight(_) => "straight",
                    SectionCase::Tangent(_) => "tangent",
                    SectionCase::Conic(_) => "conic",
                    SectionCase::Loops { .. } => "loops",
                }))
            );
        }
    }

    /// **A short face is never turned definite by its wall's size.** A
    /// wall of radius `r` whose face is the lone ruling `(r, 0, 0)` to
    /// `(r, 0, 10 µm)`, cut by the plane through the vertex and the axis
    /// tilted so the axis meets it at `sin β = k·ε/10 µm`: the ruling
    /// leaves the plane by at most `k·ε`, so the section over it is the
    /// ruling pair (`Straight`) at every `k < 1`, on a 1 km wall and a
    /// 1 m one. Levered across the whole cylinder (`r + |gap|`) the tilt's
    /// second-order turn read `r·sin² β`, which served an ellipse on the
    /// 1 km wall and escalated on the 1 m one; the face reaches 10 µm
    /// across the wall and reads nothing.
    #[test]
    fn a_short_face_is_never_turned_definite_by_its_walls_size() {
        let band = geom_core::Band::linear(Tol::witness()).expect("a linear band");
        let e = 1e-5;
        for r in [1000.0, 1.0] {
            let base = Point3::new(r, 0.0, 0.0);
            let mut body = crate::Body::<f64>::new();
            let seed = body.mvfs(base, true).unwrap();
            body.set_face_surface(
                seed.face,
                crate::construction::FaceSurface::New {
                    surface: geom::Surface::Cylinder {
                        origin: Point3::origin(),
                        axis: Vec3::unit_z(),
                        radius: r,
                        u_ref: Vec3::unit_x(),
                    },
                    sense: true,
                },
            )
            .unwrap();
            body.mev_line(
                crate::construction::MevSite::Lone {
                    r#loop: seed.r#loop,
                },
                Point3::new(r, 0.0, e),
                Tol::witness(),
            )
            .unwrap();
            for k in [0.5, 0.8, 0.95] {
                let c: f64 = k * band.zero() / e;
                let normal =
                    UnitVec3::new(Vec3::new(0.0, (1.0 - c * c).sqrt(), c), "short face", band)
                        .unwrap();
                let got = wall_section(&body, band, base, normal, seed.face, seed.vertex);
                assert!(
                    matches!(
                        got,
                        Ok(Some(WallSection {
                            case: SectionCase::Straight(_),
                            ..
                        }))
                    ),
                    "r = {r}, k = {k}: the ruling pair, got {:?}",
                    got.map(|w| w.map(|w| match w.case {
                        SectionCase::Straight(_) => "straight",
                        SectionCase::Tangent(_) => "tangent",
                        SectionCase::Conic(_) => "conic",
                        SectionCase::Loops { .. } => "loops",
                    }))
                );
            }
        }
    }

    /// **A short patch on a large wall never takes its rims' whole turn
    /// for a conic.** A `1e5·ε × 1e3·ε`
    /// patch (100 µm × 1 µm at ε = 1e-9) of a 1 km wall, bounded by two rim
    /// arcs and two rulings, read at a corner and cut by the plane through that corner and the axis
    /// tilted by `sin β = k·zero/e` about the radial: the patch stands
    /// within `k·zero` of the corner's ruling. Read over its arcs' spans
    /// the turn is Zero (the rulings); round the arcs' whole turn (2 km
    /// across) it reads definite (a conic). The section serves only what
    /// both readings serve ([`agreed_section`]), so it escalates on the
    /// split, and never serves the whole turn's conic.
    #[test]
    fn a_rim_patchs_turn_is_levered_at_its_arcs_not_their_whole_turn() {
        // Pinned to limb 2's kernel limit at ε = 1e-12: the 1 km-radius rim rounds at its radius
        // (`work/offset/the-closed-form-net-builder-rounds-at-a-km-radius-not-at-the-span.md`).
        test_utils::kernel_limit::pinned_row(
            geom_core::Tol::witness().eps(),
            &[1e-12],
            "a_rim_patchs_turn_is_levered_at_its_arcs_not_their_whole_turn",
            || {
                let band = geom_core::Band::linear(Tol::witness()).expect("a linear band");
                let eps = Tol::witness().eps();
                let (r, e, w) = (1000.0, 1e3 * eps, 1e5 * eps);
                let mut body = crate::Body::<f64>::new();
                let face = crate::test_support_fixtures::cyl_wall_sheet(
                    &mut body,
                    crate::test_support_fixtures::CylFrame::canonical(r),
                    (0.0, w / r),
                    (0.0, e),
                    Tol::witness(),
                );
                let base = Point3::new(r, 0.0, 0.0);
                let corner = body
                    .vertex_points()
                    .find(|(_, p)| (*p - base).norm() < 1e-9)
                    .map(|(v, _)| v)
                    .expect("the patch's corner");
                for k in [0.5, 0.8, 0.95] {
                    let c: f64 = k * band.zero() / e;
                    let normal =
                        UnitVec3::new(Vec3::new(0.0, (1.0 - c * c).sqrt(), c), "rim patch", band)
                            .unwrap();
                    let bracket = geom_core::k_stats::Bracket::open();
                    let got = wall_section(&body, band, base, normal, face, corner);
                    let recorded = bracket.finish();
                    let err = match got {
                        Err(err @ SplitJoinError::Escalated { .. }) => err,
                        got => panic!(
                            "k = {k}: the span's rulings against the whole turn's conic, got {:?}",
                            got.map(|w| w.map(|w| match w.case {
                                SectionCase::Straight(_) => "straight",
                                SectionCase::Tangent(_) => "tangent",
                                SectionCase::Conic(_) => "conic",
                                SectionCase::Loops { .. } => "loops",
                            }))
                        ),
                    };
                    let SplitJoinError::Escalated { diag, .. } = &err else {
                        unreachable!("matched as an escalation")
                    };
                    assert_eq!(
                        diag.predicate,
                        Some("pc_axis_plane_parallel_straddle"),
                        "k = {k}"
                    );
                    // The two readings' enclosure, on the frame: never poison.
                    assert_eq!(
                        diag.margin.kind(),
                        geom_core::MarginKind::Enclosure,
                        "k = {k}"
                    );
                    assert_eq!(
                        recorded.escalations.last().map(|e| e.source),
                        Some(*diag),
                        "k = {k}: on the frame"
                    );
                    // The section's escalation ends as geom-brep ends it: the
                    // enclosure's whole ending, kind lever first. (That a
                    // straddle's enclosure reads as subdividable is open on the
                    // cleave row `split-escalations-end-a-poisoned-margin-in-the-plane-lever`.)
                    assert!(
                        err.to_string().contains(&format!(
                            "{}. Recourse: subdivide the parameter box for a tighter enclosure, or \
                         move the geometry",
                            diag.payload()
                        )),
                        "k = {k}: {err}"
                    );
                }
            },
        );
    }

    /// **The Boolean's germ join never serves a rim patch where main
    /// escalated.** The germ join's wall side mints its chord through the
    /// split lane ([`JoinLane::Split`], `boolean::join`'s `split_curve`).
    /// A `1e5·ε × 1e3·ε` patch on a wall of radius `3·zero/c²` is cut by
    /// the plane through a corner and the axis tilted by
    /// `c = 0.5·zero/e`. Round the rims' whole turn the turn moves the
    /// patch by about `c²·r = 3·zero`, in the band, so main escalates.
    /// Over the arcs' spans it moves the patch by about `0.5·zero`, so the
    /// span alone serves the rulings. The chord escalates.
    #[test]
    fn the_germ_join_escalates_a_rim_patch_main_escalated() {
        let band = geom_core::Band::linear(Tol::witness()).expect("a linear band");
        let eps = Tol::witness().eps();
        let (e, w) = (1e3 * eps, 1e5 * eps);
        let c: f64 = 0.5 * band.zero() / e;
        let r = 3.0 * band.zero() / (c * c);
        let mut body = crate::Body::<f64>::new();
        let face = crate::test_support_fixtures::cyl_wall_sheet(
            &mut body,
            crate::test_support_fixtures::CylFrame::canonical(r),
            (0.0, w / r),
            (0.0, e),
            Tol::witness(),
        );
        let base = Point3::new(r, 0.0, 0.0);
        let corners: Vec<_> = body.vertex_points().collect();
        let corner = corners
            .iter()
            .find(|(_, p)| (*p - base).norm() < 1e-9 * r)
            .map(|(v, _)| *v)
            .expect("the patch's corner");
        let other = corners
            .iter()
            .find(|(v, _)| *v != corner)
            .map(|(v, _)| *v)
            .expect("a second corner");
        let normal = Vec3::new(0.0, (1.0 - c * c).sqrt(), c);
        let plane = geom::Surface::Plane {
            origin: base,
            normal,
            u_ref: Vec3::unit_x(),
        };
        let wall = crate::test_support_fixtures::CylFrame::canonical(r).surface::<f64>();
        let (below, above) =
            crate::splitting::rules::face_axial_range(&body, face, base, Vec3::unit_z()).unwrap();
        let read = |across| {
            let reach = geom_brep::Reach::Face {
                at: base,
                below,
                above,
                across,
            };
            geom_brep::plane_cylinder_section(&plane, &wall, &reach, band)
        };
        let span = crate::splitting::rules::face_reach_from(&body, face, base).unwrap();
        let round = crate::splitting::rules::face_reach_round_from(&body, face, base).unwrap();
        assert!(
            matches!(read(round), Err(geom_brep::SectionError::Escalated(_))),
            "main's whole-turn reading escalates"
        );
        assert!(
            matches!(
                read(span),
                Ok(geom_brep::PlaneCylinderSection::ParallelLines { .. }
                    | geom_brep::PlaneCylinderSection::TangentLine(_))
            ),
            "the span alone serves the rulings"
        );
        let mut ctx = SectionCtx {
            origin: base,
            normal: UnitVec3::new(normal, "rim patch", band).unwrap(),
            plane_key: None,
            branch: None,
        };
        let leave = Departure {
            dir: Vec3::unit_z(),
            datum: Datum::Germ,
        };
        let got = chord_spec(
            &mut body,
            band,
            JoinLane::Split(&mut ctx),
            face,
            corner,
            other,
            leave,
        );
        assert!(
            matches!(got, Err(SplitJoinError::Escalated { .. })),
            "the germ join's chord escalates, got {:?}",
            got.map(|s| s.is_some())
        );
    }

    /// **A face at one station is cut by a plane across the axis in a
    /// conic.** A unit wall about `z` whose face is the rim arc at `z = 0`
    /// from `(1, 0, 0)` a quarter turn round: it reaches nothing along the
    /// axis, so a tilt levered along the axis alone moves it by nothing,
    /// read Zero and minted rulings for the plane `z = 0` (the merge
    /// later refused them). The plane turns about the rulings' hinge, and
    /// the face reaches across the wall from there by up to its distance
    /// from the base vertex: the plane `z = 0` and the planes 30° and 60°
    /// off it are conics.
    #[test]
    fn a_face_at_one_station_is_cut_by_a_plane_across_the_axis_in_a_conic() {
        let band = geom_core::Band::linear(Tol::witness()).expect("a linear band");
        let carrier = geom::Curve3::Circle {
            center: Point3::origin(),
            axis: Vec3::unit_z(),
            radius: 1.0,
            u_ref: Vec3::unit_x(),
        };
        let base = carrier.eval(0.0);
        let mut body = crate::Body::<f64>::new();
        let seed = body.mvfs(base, true).unwrap();
        body.set_face_surface(
            seed.face,
            crate::construction::FaceSurface::New {
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
        let cyl = body.get_face(seed.face).unwrap().surface;
        let rim_plane = body.add_surface(geom::Surface::Plane {
            origin: Point3::origin(),
            normal: Vec3::unit_z(),
            u_ref: Vec3::unit_x(),
        });
        let quarter = core::f64::consts::FRAC_PI_2;
        body.mev(
            crate::construction::MevSite::Lone {
                r#loop: seed.r#loop,
            },
            carrier.eval(quarter),
            EdgeCurveSpec {
                description: geom_brep::EdgeDescriptionSpec::Intersection {
                    pair: geom_brep::SurfacePair::new(cyl, rim_plane),
                    selector: geom_brep::CurveSelector::Witness(carrier.mid_point(0.0, quarter)),
                },
                carrier,
                param_start: 0.0,
                param_end: quarter,
            },
            Tol::witness(),
        )
        .unwrap();
        for degrees in [90.0_f64, 60.0, 30.0] {
            let tilt = degrees.to_radians();
            let normal =
                UnitVec3::new(Vec3::new(0.0, tilt.cos(), tilt.sin()), "one station", band).unwrap();
            let got = wall_section(&body, band, base, normal, seed.face, seed.vertex);
            assert!(
                matches!(
                    got,
                    Ok(Some(WallSection {
                        case: SectionCase::Conic(_),
                        ..
                    }))
                ),
                "{degrees}° off the axis: a conic, got {:?}",
                got.map(|w| w.map(|w| match w.case {
                    SectionCase::Straight(_) => "straight",
                    SectionCase::Tangent(_) => "tangent",
                    SectionCase::Conic(_) => "conic",
                    SectionCase::Loops { .. } => "loops",
                }))
            );
        }
    }

    /// **A one-sided face is levered from the rulings' hinge station
    /// either way.** A wall of radius `r` whose face is the lone ruling
    /// `(r, 0, 0)` to `(r, 0, e)`, `e` = 1 mm, cut by the plane through the
    /// vertex tilted to `sin β = c`. The rulings this lane mints stand on
    /// the hinge through the foot's projection, `r·c` up the axis: with
    /// `r·c = e/2` the face reaches `e/2` from that station either way, so
    /// the tilt moves it by `c·e/2 = 0.6·Kε`, in the band, and the table
    /// escalates. Read as reaching `e` both ways from the vertex, the
    /// lever was `e + r·c` and served an ellipse (main's face extent, `e`,
    /// read `1.2·Kε` and served one too).
    #[test]
    fn a_one_sided_face_is_levered_from_the_hinge_station_either_way() {
        let band = geom_core::Band::linear(Tol::witness()).expect("a linear band");
        let e = 1e-3;
        let c: f64 = 1.2 * band.escalate() / e;
        let r = e / (2.0 * c);
        let base = Point3::new(r, 0.0, 0.0);
        let mut body = crate::Body::<f64>::new();
        let seed = body.mvfs(base, true).unwrap();
        body.set_face_surface(
            seed.face,
            crate::construction::FaceSurface::New {
                surface: geom::Surface::Cylinder {
                    origin: Point3::origin(),
                    axis: Vec3::unit_z(),
                    radius: r,
                    u_ref: Vec3::unit_x(),
                },
                sense: true,
            },
        )
        .unwrap();
        body.mev_line(
            crate::construction::MevSite::Lone {
                r#loop: seed.r#loop,
            },
            Point3::new(r, 0.0, e),
            Tol::witness(),
        )
        .unwrap();
        let normal = UnitVec3::new(
            Vec3::new((1.0 - c * c).sqrt(), 0.0, c),
            "one-sided face",
            band,
        )
        .unwrap();
        let got = wall_section(&body, band, base, normal, seed.face, seed.vertex);
        assert!(
            matches!(
                got,
                Err(SplitJoinError::Escalated { ref diag, .. })
                    if diag.predicate == Some("pc_axis_plane_parallel")
            ),
            "the tilt over the face from the hinge station is in the band, got {:?}",
            got.map(|w| w.map(|w| match w.case {
                SectionCase::Straight(_) => "straight",
                SectionCase::Tangent(_) => "tangent",
                SectionCase::Conic(_) => "conic",
                SectionCase::Loops { .. } => "loops",
            }))
        );
    }

    /// **A hole loop's re-homing ends in the walk decision's lever
    /// alone.** The join asks the walk of many vertices, so no margin it
    /// carries binds the operation: no tolerance is offered on a read
    /// margin or a straddle, and no box to subdivide.
    #[test]
    fn ring_homing_ends_in_the_walks_decision() {
        use crate::splitting::{Escalation, LoopDecision};
        use geom_core::{Decided, MarginDiag};
        let band = Band::new(1e-9, 1e-8).unwrap();
        let read = 3e-9_f64.sign_within(band).unwrap_err();
        let straddle = geom_core::k_stats::straddle(
            "rh_straddle",
            Decided {
                sign: Sign::Zero,
                margin: MarginDiag::value(0.5e-9),
            },
            Decided {
                sign: Sign::Positive,
                margin: MarginDiag::value(3e-8),
            },
            band,
        );
        let cases = [
            (
                LoopDecision::Ray,
                "where a test ray from a point meets the loop's boundary",
                "nudge the point so no boundary corner lines up with it",
            ),
            (
                LoopDecision::Boundary,
                "whether a point lies on the loop's boundary or clear of it",
                "move the point exactly onto the boundary or clearly off it",
            ),
        ];
        for (decision, subject, lever) in cases {
            for (escalation, diag) in [(Escalation::Margin, read), (Escalation::Straddle, straddle)]
            {
                let text = SplitJoinError::RingHoming(PointInLoopError::Escalated {
                    r#loop: LoopKey::default(),
                    decision,
                    escalation,
                    diag,
                })
                .to_string();
                assert_eq!(
                    text,
                    format!(
                        "re-homing a hole loop: {subject} is undecided: {}. Recourse: {lever}",
                        diag.payload()
                    ),
                    "{decision:?}, {escalation:?}"
                );
            }
        }
    }

    /// The split lane's adjacency question on a conic between edge (a
    /// cylinder cap's rim, which a planar divided face carries): the
    /// belly verdict and the coplanar verdict. The rim is the upper
    /// semicircle of the unit circle in z = 0, from (1, 0, 0) to
    /// (−1, 0, 0).
    #[test]
    fn split_lane_decides_a_conic_between_edge_against_its_section_plane() {
        let band = Band::new(1e-9, 1e-8).unwrap();
        let mut body = crate::Body::<f64>::new();
        let rim = rim_run(&mut body, 0.0, core::f64::consts::PI);
        let verdict = |normal: Vec3<f64>| {
            let plane = crate::test_support::split_plane(Point3::origin(), normal, Tol::witness());
            let ctx = SectionCtx {
                origin: plane.origin,
                normal: plane.normal,
                plane_key: None,
                branch: None,
            };
            between_edge_is_section(&body, &ctx, rim, band).unwrap()
        };
        // A section plane through the rim's two ends and the cap's
        // centre (y = 0): the rim bellies to y = 1, so it is no section
        // segment and the chord is minted.
        assert_eq!(verdict(Vec3::unit_y()), Some(false));
        // The section plane holding the whole rim (z = 0): Zero → skip.
        assert_eq!(verdict(Vec3::unit_z()), Some(true));
    }

    /// `chord_spec` on the fixture's chord, θ = 0 to θ = π/2, with the
    /// section leaving θ = 0 along `leave`.
    fn spec_leaving(leave: Vec3<f64>) -> Result<Option<EdgeCurveSpec<f64>>, SplitJoinError> {
        let leave = super::Departure {
            dir: leave,
            datum: super::Datum::Section,
        };
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
        .map(|chord| chord.map(|c| c.spec))
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

    /// **A split datum's Zero is a graze only along the section.** The
    /// fixture's tangent at θ = 0 is `ŷ` and its lever the unit
    /// cylinder's radius. `ŷ·5e-10` lies along the section and reads half
    /// the zero band — the needle cone's shape, a short product, not a
    /// crooked datum — so it grazes; `û_e = ŷ × n`, in the plane and
    /// normal to the section, and `û_e + ŷ·5e-10`, nearly so, are no
    /// departure along the section and name the invariant; `ŷ·5e-9`
    /// escalates naming `chord_arc_leave_section`.
    #[test]
    fn a_datum_off_the_section_refuses() {
        let phi = 0.5f64;
        let n = Vec3::new(phi.sin(), 0.0, phi.cos());
        let u_e = Vec3::unit_y().cross(n);
        let err = spec_leaving(Vec3::unit_y() * 5e-10).unwrap_err();
        assert!(
            matches!(err, SplitJoinError::SectionDepartureGrazes { .. }),
            "along the section, inside the zero band: {err:?}"
        );
        for (what, leave) in [
            ("normal to the section", u_e),
            ("nearly normal to the section", u_e + Vec3::unit_y() * 5e-10),
        ] {
            let err = spec_leaving(leave).unwrap_err();
            assert!(
                matches!(err, SplitJoinError::SectionInvariant { .. }),
                "{what}: {err:?}"
            );
        }
        let err = spec_leaving(Vec3::unit_y() * 5e-9).unwrap_err();
        let SplitJoinError::Escalated { diag, .. } = err else {
            panic!("expected an escalation, got {err:?}");
        };
        assert_eq!(diag.predicate, Some("chord_arc_leave_section"));
    }

    #[test]
    fn section_area_pair_carries_the_shared_recourse() {
        let face = FaceKey::default();
        let msg = SplitJoinError::DegenerateSection { face }.to_string();
        assert_eq!(msg.matches(JOIN_RECOURSE).count(), 1, "{msg}");
        assert!(!msg.contains("declare"), "{msg}");

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

    /// **The anti-re-fork row for the arc a chord takes.** Its rungs —
    /// `chord_arc_leave_germ` and `chord_arc_leave_section`, one per
    /// datum, in this file — and the split's own walk heading,
    /// `split_join_conic_heading` in `splitting/join.rs`, are each
    /// decided in exactly ONE place in this crate, counted, not merely
    /// located: for most of this module's life `chord_spec` and
    /// `bool_planar_chord_spec` sat 500 lines apart carrying
    /// line-identical copies of their arc selection, and a cross-file
    /// guard would have been green throughout.
    ///
    /// On the split the walk and the chord read one sine on one lever
    /// ([`arc_leaving`]): that is the one duplicate this row admits, and
    /// it names both halves so a third cannot join them unseen.
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
        let files = crate::source_walk::crate_sources();
        // Assembled rather than spelled, so this file is subject to the
        // count like any other.
        for (rung, home) in [
            (format!("\"chord_{}_germ\"", "arc_leave"), "chord_join.rs"),
            (
                format!("\"chord_{}_section\"", "arc_leave"),
                "chord_join.rs",
            ),
            (
                format!("\"split_join_{}\"", "conic_heading"),
                "splitting/join.rs",
            ),
        ] {
            let home = crate::source_walk::src_root().join(home);
            assert!(
                files.contains(&home),
                "the walk did not find {}",
                home.display()
            );
            let mut sites = 0;
            for path in &files {
                let text = std::fs::read_to_string(path).expect("a readable source file");
                // DECIDE sites, counted on a whitespace-stripped copy so
                // a call broken across lines counts the same as an
                // inline one.
                let stripped: String = text.chars().filter(|c| !c.is_whitespace()).collect();
                sites += stripped.matches(&format!("decide({rung}")).count();
                // A rung chosen from a `match` and decided under a bound
                // name still counts: its literal lives in its home.
                sites += stripped.matches(&format!("=>({rung},")).count();
                assert!(
                    path == &home || !text.contains(rung.as_str()),
                    "{} names {rung}: the chord's arc has been re-forked out of {}, which \
                     must hold the only one. Call `arc_leaving` instead.",
                    path.display(),
                    home.display()
                );
            }
            assert_eq!(sites, 1, "{rung} is decided at {sites} site(s)");
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod section_case_pair_tests {
    use geom_core::{Band, Point3, Tol, Vec3};

    use super::{SectionCase, SectionReach, WallReach, section_case};
    use crate::entity::FaceKey;
    use crate::splitting::SplitJoinError;

    fn band() -> Band {
        Band::linear(Tol::witness()).expect("a linear band")
    }

    fn reach() -> SectionReach<f64> {
        let reach = WallReach {
            at: Point3::origin(),
            below: 4.0,
            above: 4.0,
            span: 4.0,
            round: 4.0,
        };
        SectionReach::Cylinder(reach)
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
            let got = section_case(f, band(), &a, &b, reach()).expect("the rim arm is wired");
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
            match section_case(f, band(), &a, &b, reach()) {
                Err(SplitJoinError::SectionInvariant { .. }) => {}
                Err(e) => panic!("a curved pair must refuse SectionInvariant, got {e:?}"),
                Ok(_) => panic!("a curved pair must refuse typed, never classify"),
            }
        }
        match section_case(f, band(), &plane(), &plane(), reach()) {
            Err(SplitJoinError::SectionInvariant { .. }) => {}
            Err(e) => panic!("a planar pair must refuse SectionInvariant, got {e:?}"),
            Ok(_) => panic!("a planar pair must refuse typed here, never classify"),
        }
    }
}

/// The pending-ring lifecycle on a hand-built face: a 2×2 top face
/// divided along its diagonal, with rings placed on and off that run.
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod pending_ring_tests {
    use super::*;
    use crate::euler::{MefSite, MevSite};
    use crate::null::NewVertexSide;
    use crate::test_support_fixtures::prism_z;
    use geom_brep::EdgeCurveSpec;
    use geom_core::{Point3, Tol};

    fn tol() -> Tol {
        Tol::witness()
    }

    fn joiner() -> ChordJoiner {
        ChordJoiner::new(Band::linear(tol()).unwrap())
    }

    /// The slab `[0,2]² × [0,1]`, its top face and that face's four
    /// corners, counterclockwise from (0, 0).
    fn slab() -> (Body<f64>, FaceKey, Vec<VertexKey>) {
        let p = prism_z::<f64>(
            &[(0.0, 0.0), (2.0, 0.0), (2.0, 2.0), (0.0, 2.0)],
            0.0,
            1.0,
            tol(),
        );
        (p.body, p.top_face, p.top)
    }

    /// The half of `face`'s outer starting at `v`.
    fn outer_from(body: &Body<f64>, face: FaceKey, v: VertexKey) -> HalfEdgeKey {
        let outer = body.get_face(face).unwrap().outer;
        let LoopBoundary::Cycle { first } = body.get_loop(outer).unwrap().boundary else {
            panic!("outer is a cycle");
        };
        body.loop_cycle(first)
            .unwrap()
            .into_iter()
            .find(|&he| body.get_half_edge(he).unwrap().start == v)
            .unwrap()
    }

    /// An empty ring of `face` at `p`, the pierce ring's first step.
    fn empty_ring(body: &mut Body<f64>, face: FaceKey, p: Point3<f64>) -> LoopKey {
        let outer = body.get_face(face).unwrap().outer;
        let LoopBoundary::Cycle { first } = body.get_loop(outer).unwrap().boundary else {
            panic!("outer is a cycle");
        };
        let u = body.half_edge_start_point(first).unwrap();
        let chord = body
            .mev(
                MevSite::Fan {
                    he1: first,
                    he2: first,
                },
                p,
                EdgeCurveSpec::line_between(u, p),
                tol(),
            )
            .unwrap();
        body.kemr(chord.he_plus, chord.he_minus).unwrap().ring
    }

    /// A pierce ring of `face` at `p` with `struts` null edges, and the
    /// minus half of each strut.
    fn pierce_ring(
        body: &mut Body<f64>,
        face: FaceKey,
        p: Point3<f64>,
        struts: usize,
    ) -> (LoopKey, Vec<HalfEdgeKey>) {
        let ring = empty_ring(body, face, p);
        let mut anchor = None;
        let mut halves = Vec::new();
        for _ in 0..struts {
            let site = match anchor {
                None => MevSite::Lone { r#loop: ring },
                Some(he) => MevSite::Fan { he1: he, he2: he },
            };
            let s = body.mev_null(site, NewVertexSide::Above).unwrap();
            anchor.get_or_insert(s.he_plus);
            halves.push(s.he_minus);
        }
        (ring, halves)
    }

    /// Divides `face` along the chord from its corner `a` to its corner
    /// `b`, re-homing its rings, and returns the new face.
    fn divide(
        j: &mut ChordJoiner,
        body: &mut Body<f64>,
        face: FaceKey,
        (a, b): (VertexKey, VertexKey),
    ) -> Result<FaceKey, SplitJoinError> {
        let (he1, he2) = (outer_from(body, face, a), outer_from(body, face, b));
        let made = body.mef_chord(MefSite::Chords { he1, he2 }, tol()).unwrap();
        let remainder = body.get_half_edge(he2).unwrap().parent_loop;
        j.rehome_rings(body, face, made.face, remainder)?;
        Ok(made.face)
    }

    /// The vertices of `face`'s outer loop.
    fn face_corners(body: &Body<f64>, face: FaceKey) -> Vec<VertexKey> {
        ring_vertices(body, body.get_face(face).unwrap().outer).unwrap()
    }

    /// Joins `h1` to `h2` as a planar boolean match does: places a
    /// pending ring, plans, curves and mints.
    fn join(
        j: &mut ChordJoiner,
        body: &mut Body<f64>,
        h1: HalfEdgeKey,
        h2: HalfEdgeKey,
    ) -> Result<Vec<EdgeKey>, SplitJoinError> {
        j.place_pending(body, (h1, h2))?;
        let plan = j.plan(body, (h1, h2), SegmentEdge::Locus(None))?;
        let (p1, p2) = (
            body.half_edge_start_point(h1).unwrap(),
            body.half_edge_start_point(h2).unwrap(),
        );
        let d = (p2 - p1) * (1.0 / (p2 - p1).norm());
        let leave = Leave {
            at: [(h1, d), (h2, -d)],
            datum: Datum::Germ,
        };
        match j.segment_curve(body, &plan, JoinLane::Planar, leave)? {
            Some(curve) => j.join(body, &plan, &curve, tol()),
            None => Ok(Vec::new()),
        }
    }

    /// Asserts `got` is the ambiguous-homing refusal naming `ring`.
    fn refuses_at<R: core::fmt::Debug>(got: Result<R, SplitJoinError>, ring: LoopKey) {
        match got {
            Err(SplitJoinError::RingHomingAmbiguous { ring: r }) => assert_eq!(r, ring),
            other => panic!("expected RingHomingAmbiguous at {ring:?}, got {other:?}"),
        }
    }

    fn face_of(body: &Body<f64>, ring: LoopKey) -> FaceKey {
        body.get_loop(ring).unwrap().face
    }

    /// **A strut on the run is left pending, not refused, and is still
    /// unplaced when nothing joins it** (the quiescence refusal's
    /// reading); a ring off the run is placed as before.
    #[test]
    fn a_strut_on_the_run_is_pending_until_a_join_places_it() {
        let (mut body, top, c) = slab();
        let mut j = joiner();
        let (strut, _) = pierce_ring(&mut body, top, Point3::new(1.0, 1.0, 1.0), 1);
        let (off, _) = pierce_ring(&mut body, top, Point3::new(1.5, 0.5, 1.0), 1);
        let newf = divide(&mut j, &mut body, top, (c[0], c[2])).unwrap();
        assert_eq!(
            face_of(&body, strut),
            top,
            "the pending strut stays where it was"
        );
        assert_eq!(j.unplaced_ring(&body), Some(strut), "the strut is pending");
        let other = if face_of(&body, off) == newf {
            newf
        } else {
            top
        };
        assert!(
            body.get_face(other).unwrap().rings.contains(&off),
            "the ring off the run is homed by its own vertex"
        );
    }

    /// **A ring with a real edge, every vertex on the run, still
    /// refuses**: only a ring of null edges is deferred.
    #[test]
    fn a_real_ring_on_the_run_still_refuses() {
        let (mut body, top, c) = slab();
        let mut j = joiner();
        let p = Point3::new(0.5, 0.5, 1.0);
        let q = Point3::new(1.5, 1.5, 1.0);
        let ring = empty_ring(&mut body, top, p);
        body.mev(
            MevSite::Lone { r#loop: ring },
            q,
            EdgeCurveSpec::line_between(p, q),
            tol(),
        )
        .unwrap();
        refuses_at(divide(&mut j, &mut body, top, (c[0], c[2])), ring);
        // An empty ring (a lone vertex, no edge) refuses the same way.
        let (mut body, top, c) = slab();
        let ring = empty_ring(&mut body, top, Point3::new(1.0, 1.0, 1.0));
        refuses_at(divide(&mut joiner(), &mut body, top, (c[0], c[2])), ring);
    }

    /// **A join moves a pending strut into the face of the ring it meets.**
    #[test]
    fn a_join_places_a_pending_strut_with_its_partner() {
        let (mut body, top, c) = slab();
        let mut j = joiner();
        let (strut, s) = pierce_ring(&mut body, top, Point3::new(1.0, 1.0, 1.0), 1);
        let (lower, l) = pierce_ring(&mut body, top, Point3::new(1.5, 0.5, 1.0), 1);
        divide(&mut j, &mut body, top, (c[0], c[2])).unwrap();
        let home = face_of(&body, lower);
        assert_ne!(
            face_of(&body, strut),
            home,
            "the fixture puts the two apart"
        );
        join(&mut j, &mut body, s[0], l[0]).unwrap();
        assert_eq!(j.unplaced_ring(&body), None, "the join placed the strut");
        // The join's second chord walls a sliver, holding the strut's
        // half, off the partner's face.
        let joined = face_of(&body, body.get_half_edge(s[0]).unwrap().parent_loop);
        assert_eq!(
            j.take_fragments().rows(),
            [(joined, home)],
            "the join divided the partner's face"
        );
    }

    /// **Two pending rings in different faces refuse when joined.**
    #[test]
    fn two_pending_rings_in_different_faces_refuse() {
        let (mut body, top, c) = slab();
        let mut j = joiner();
        let (a, ha) = pierce_ring(&mut body, top, Point3::new(1.0, 1.0, 1.0), 1);
        let (b, hb) = pierce_ring(&mut body, top, Point3::new(0.5, 0.5, 1.0), 1);
        let newf = divide(&mut j, &mut body, top, (c[0], c[2])).unwrap();
        // Both are pending in `top`. No sweep reaches two pending rings
        // in different faces, so the state is fabricated: one is carried
        // across by hand.
        body.ring_move(b, newf).unwrap();
        assert!(j.unplaced_ring(&body).is_some());
        refuses_at(join(&mut j, &mut body, ha[0], hb[0]), a);
    }

    /// **A polygon completing inside a pending loop refuses**: the mef
    /// would divide a face nothing has placed the loop in.
    #[test]
    fn a_mef_inside_a_pending_loop_refuses() {
        let (mut body, top, c) = slab();
        let mut j = joiner();
        let (ring, h) = pierce_ring(&mut body, top, Point3::new(1.0, 1.0, 1.0), 2);
        divide(&mut j, &mut body, top, (c[0], c[2])).unwrap();
        assert_eq!(j.unplaced_ring(&body), Some(ring));
        refuses_at(join(&mut j, &mut body, h[0], h[1]), ring);
    }

    /// **Two pending struts of one face merge into a ring still pending,
    /// and the join that reaches it from a placed ring places it.**
    #[test]
    fn two_pending_struts_merge_and_are_placed_together() {
        let (mut body, top, c) = slab();
        let mut j = joiner();
        let (a, ha) = pierce_ring(&mut body, top, Point3::new(1.0, 1.0, 1.0), 2);
        let (b, hb) = pierce_ring(&mut body, top, Point3::new(0.5, 0.5, 1.0), 1);
        let (lower, l) = pierce_ring(&mut body, top, Point3::new(1.5, 0.5, 1.0), 1);
        divide(&mut j, &mut body, top, (c[0], c[2])).unwrap();
        let home = face_of(&body, lower);
        assert_eq!((face_of(&body, a), face_of(&body, b)), (top, top));
        assert_ne!(top, home, "the fixture puts the partner apart");
        join(&mut j, &mut body, ha[0], hb[0]).unwrap();
        let merged = body.get_half_edge(ha[1]).unwrap().parent_loop;
        assert_eq!(
            j.unplaced_ring(&body),
            Some(merged),
            "the merged ring is still pending"
        );
        assert_eq!(face_of(&body, merged), top, "and still where it was");
        let merge = j.take_fragments();
        assert!(
            merge.rows().iter().all(|&(_, from)| from == top),
            "the merge's chords divided only the face it was in: {merge:?}"
        );
        join(&mut j, &mut body, ha[1], l[0]).unwrap();
        assert_eq!(j.unplaced_ring(&body), None, "the partner's join placed it");
        let joined = face_of(&body, body.get_half_edge(ha[1]).unwrap().parent_loop);
        assert_eq!(
            j.take_fragments().rows(),
            [(joined, home)],
            "its chords walled a sliver off the partner's face"
        );
    }

    /// **A pending strut stays pending through a later division whose
    /// run misses it.**
    #[test]
    fn a_pending_strut_stays_pending_through_a_later_division() {
        let p = prism_z::<f64>(
            &[
                (0.0, 0.0),
                (2.0, 0.0),
                (3.0, 1.0),
                (2.0, 2.0),
                (0.0, 2.0),
                (-1.0, 1.0),
            ],
            0.0,
            1.0,
            tol(),
        );
        let (mut body, top, c) = (p.body, p.top_face, p.top);
        let mut j = joiner();
        let (strut, _) = pierce_ring(&mut body, top, Point3::new(1.0, 1.0, 1.0), 1);
        // The run from (-1, 1) to (3, 1) through the strut.
        divide(&mut j, &mut body, top, (c[5], c[2])).unwrap();
        assert_eq!(j.unplaced_ring(&body), Some(strut));
        assert_eq!(face_of(&body, strut), top);
        // Cut a corner off whichever half `top` kept, along a chord
        // that passes above or below the strut.
        let corner = if face_corners(&body, top).contains(&c[3]) {
            (c[3], c[5])
        } else {
            (c[0], c[2])
        };
        divide(&mut j, &mut body, top, corner).unwrap();
        assert_eq!(j.unplaced_ring(&body), Some(strut), "still pending");
        assert_eq!(face_of(&body, strut), top, "and still where it was");
    }

    /// **A strut still pending when a sweep is done refuses**: the
    /// check both sweeps end with.
    #[test]
    fn a_strut_still_pending_at_the_end_refuses() {
        let (mut body, top, c) = slab();
        let mut j = joiner();
        assert!(j.finish(&body).is_ok(), "nothing pending");
        let (strut, _) = pierce_ring(&mut body, top, Point3::new(1.0, 1.0, 1.0), 1);
        divide(&mut j, &mut body, top, (c[0], c[2])).unwrap();
        refuses_at(j.finish(&body), strut);
    }
}

/// **[`null_site`] answers a site vertex that does not resolve typed,
/// and panics on a torn link past one that does.**
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod null_site_rows {
    use super::*;
    use crate::null::NewVertexSide;
    use crate::review_d18::{ROW_FOUR, assert_torn_op_panics};

    /// A declined cube with one null strut at its seed vertex: the
    /// body, the seed, the strut's copy and its curve.
    fn strut() -> (Body<f64>, VertexKey, VertexKey, crate::geometry::CurveKey) {
        let cube = crate::test_support_fixtures::declined_cube::<f64>(Tol::witness());
        let mut body = cube.body;
        let seed = cube.seed.vertex;
        let he = body.get_vertex(seed).unwrap().emanating.unwrap();
        let created = body
            .mev_null(
                crate::construction::MevSite::Fan { he1: he, he2: he },
                NewVertexSide::Above,
            )
            .unwrap();
        (body, seed, created.vertex, created.curve)
    }

    /// The root the caller passed, and a copy a null edge's attribute
    /// names, each answer `Err` when they do not resolve; a sound site is the seed and its copy. Read as absent,
    /// either would answer a site short of the vertex.
    #[test]
    fn a_site_vertex_that_does_not_resolve_answers_typed() {
        let (mut body, seed, copy, curve) = strut();
        assert_eq!(
            null_site(&body, &[seed]),
            Ok(vec![seed, copy]),
            "sound site"
        );
        let data = body.get_vertex(seed).unwrap().clone();
        let stale = body.vertices.insert(data);
        body.vertices.remove(stale);
        assert_eq!(null_site(&body, &[stale]), Err(StaleSite), "a stale root");
        let Some(CurveGeom::NullScaffold(attr)) = body.curves.get_mut(curve) else {
            panic!("the strut's curve is null scaffolding");
        };
        attr.above_end = stale;
        assert_eq!(null_site(&body, &[seed]), Err(StaleSite), "a stale copy");
    }

    /// An edge at a resolved site vertex whose curve does not resolve
    /// panics naming it, where the read skipped it as no strut.
    #[test]
    fn a_torn_curve_at_a_site_vertex_panics() {
        let (mut body, seed, _, strut_curve) = strut();
        let (edge, curve) = body
            .edges_of_vertex_linked(seed)
            .into_iter()
            .map(|e| (e, body.get_edge(e).unwrap().curve))
            .find(|&(_, c)| c != strut_curve)
            .unwrap();
        body.curves.remove(curve);
        let named = format!("{}'s curve names", EntityId::Edge(edge));
        assert_torn_op_panics("null_site", &mut body, &[&named, ROW_FOUR], |b| {
            null_site(b, &[seed])
        });
    }

    /// A resolved site vertex whose `emanating` half was dropped panics
    /// naming it, where a read of the orbit as empty would answer a site
    /// short of the strut's copy.
    #[test]
    fn a_torn_orbit_at_a_site_vertex_panics() {
        let (mut body, seed, _, _) = strut();
        let first = body.get_vertex(seed).unwrap().emanating.unwrap();
        body.half_edges.remove(first);
        let named = format!(
            "{}'s emanating names {}",
            EntityId::Vertex(seed),
            EntityId::HalfEdge(first)
        );
        assert_torn_op_panics(
            "null_site",
            &mut body,
            &[&named, ROW_FOUR, crate::live::OPERATORS_KEEP_LINKS],
            |b| null_site(b, &[seed]),
        );
    }
}

/// **The chord join's hops past a resolved face, half-edge or edge
/// panic on a torn link; a key the caller carries keeps its typed
/// answer.** Each row reads the sound fixture's answer first, then tears
/// one record and asserts the panic names the link and the premise
/// (`OPERATORS_KEEP_LINKS`: the join runs mid-operation), with the
/// body deep-unchanged.
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod torn_hop_rows {
    use geom_core::{Band, Point3, Tol, UnitVec3, Vec3};

    use super::{Datum, Departure, JoinLane, SectionCtx, SplitJoinError};
    use crate::body::Body;
    use crate::body::WALKS_CLOSE;
    use crate::entity::{EntityId, FaceKey, GeomRef, HalfEdgeKey, LoopBoundary};
    use crate::live::OPERATORS_KEEP_LINKS;
    use crate::review_d18::{ROW_FOUR, assert_torn_op_panics};

    fn band() -> Band {
        Band::linear(Tol::witness()).unwrap()
    }

    fn cyl_sheet() -> (Body<f64>, FaceKey) {
        let mut body = Body::<f64>::new();
        let face = crate::test_support_fixtures::cyl_wall_sheet(
            &mut body,
            crate::test_support_fixtures::CylFrame::canonical(1.0),
            (0.2, 1.4),
            (0.0, 1.0),
            Tol::witness(),
        );
        (body, face)
    }

    fn first_member(body: &Body<f64>, face: FaceKey) -> HalfEdgeKey {
        let outer = body.get_face(face).unwrap().outer;
        let LoopBoundary::Cycle { first } = body.get_loop(outer).unwrap().boundary else {
            panic!("the fixture's outer loop is a cycle");
        };
        first
    }

    /// Drops the curve of `he`'s edge, and names the link that dangles.
    fn drop_curve(body: &mut Body<f64>, he: HalfEdgeKey) -> String {
        let edge = body.get_half_edge(he).unwrap().edge;
        let curve = body.get_edge(edge).unwrap().curve;
        body.curves.remove(curve);
        format!(
            "{}'s curve names {}",
            EntityId::Edge(edge),
            GeomRef::Curve(curve)
        )
    }

    /// `face_azimuth_images` (`outer_cycle`, `run_azimuth_images`): a
    /// torn outer loop, loop walk, member edge and curve panic, where the
    /// loop and walk answered `Corrupt` and the edge and curve were
    /// stepped over; a stale face keeps `Corrupt`.
    #[test]
    fn the_azimuth_walk_panics_on_a_torn_loop_and_a_torn_curve() {
        let (body, face) = cyl_sheet();
        let surface = body
            .get_surface(body.get_face(face).unwrap().surface)
            .unwrap()
            .clone();
        let images = |b: &Body<f64>| {
            super::face_azimuth_images(b, &surface, face, band()).map(|i| i.map(|i| i.len()))
        };
        assert_eq!(
            images(&body).unwrap(),
            Some(4),
            "every edge of the sound sheet has an image"
        );

        let mut stale = body.clone();
        let data = stale.get_face(face).unwrap().clone();
        let gone = stale.faces.insert(data);
        stale.faces.remove(gone);
        assert!(
            matches!(
                super::face_azimuth_images(&stale, &surface, gone, band()),
                Err(SplitJoinError::Corrupt {
                    entity: EntityId::Face(f)
                }) if f == gone
            ),
            "a stale face is the caller's key, refused typed"
        );

        let mut torn = body.clone();
        let outer = torn.get_face(face).unwrap().outer;
        torn.loops.remove(outer);
        let named = format!(
            "{}'s outer names {}",
            EntityId::Face(face),
            EntityId::Loop(outer)
        );
        assert_torn_op_panics(
            "face_azimuth_images (loop)",
            &mut torn,
            &[&named, ROW_FOUR, OPERATORS_KEEP_LINKS],
            |b| images(b),
        );

        let mut torn = body.clone();
        let he = first_member(&torn, face);
        let named = drop_curve(&mut torn, he);
        assert_torn_op_panics(
            "face_azimuth_images (curve)",
            &mut torn,
            &[&named, ROW_FOUR, OPERATORS_KEEP_LINKS],
            |b| images(b),
        );

        // A member's edge: the walk steps by `next` and closes past it.
        let mut torn = body.clone();
        let he = first_member(&torn, face);
        let edge = torn.get_half_edge(he).unwrap().edge;
        torn.edges.remove(edge);
        let named = format!(
            "{}'s edge names {}",
            EntityId::HalfEdge(he),
            EntityId::Edge(edge)
        );
        assert_torn_op_panics(
            "face_azimuth_images (edge)",
            &mut torn,
            &[&named, ROW_FOUR, OPERATORS_KEEP_LINKS],
            |b| images(b),
        );

        // A member past the first: the outer loop's walk breaks.
        let mut torn = body.clone();
        let first = first_member(&torn, face);
        let second = torn.get_half_edge(first).unwrap().next;
        torn.half_edges.remove(second);
        let named = format!("the loop walk from {first:?} breaks at {first:?}");
        assert_torn_op_panics(
            "face_azimuth_images (walk)",
            &mut torn,
            &[&named, WALKS_CLOSE, OPERATORS_KEEP_LINKS],
            |b| images(b),
        );
    }

    /// `between_edge_is_section`: a torn edge panics, where it refused
    /// `Corrupt`, and a torn curve panics, where it read as null
    /// scaffolding, which lies in the plane (`Some(true)`). The sound rim
    /// lies off the plane.
    #[test]
    fn the_adjacency_skip_panics_on_a_torn_curve() {
        let (mut body, face) = cyl_sheet();
        let ctx = SectionCtx {
            origin: Point3::new(0.0, 0.0, 0.5),
            normal: UnitVec3::new(Vec3::unit_z(), "torn_hop_rows", band()).unwrap(),
            plane_key: None,
            branch: None,
        };
        let he = body
            .loop_cycle(first_member(&body, face))
            .unwrap()
            .into_iter()
            .find(|&h| {
                let e = body.get_edge(body.get_half_edge(h).unwrap().edge).unwrap();
                body.get_curve_geom(e.curve)
                    .and_then(super::CurveGeom::certified)
                    .is_some_and(|c| matches!(c.carrier(), geom::Curve3::Circle { .. }))
            })
            .unwrap();
        assert_eq!(
            super::between_edge_is_section(&body, &ctx, he, band()).unwrap(),
            Some(false),
            "a sound rim lies off the plane z = 0.5"
        );
        let mut torn = body.clone();
        let edge = torn.get_half_edge(he).unwrap().edge;
        torn.edges.remove(edge);
        let named = format!(
            "{}'s edge names {}",
            EntityId::HalfEdge(he),
            EntityId::Edge(edge)
        );
        assert_torn_op_panics(
            "between_edge_is_section (edge)",
            &mut torn,
            &[&named, ROW_FOUR, OPERATORS_KEEP_LINKS],
            |b| super::between_edge_is_section(b, &ctx, he, band()),
        );
        let named = drop_curve(&mut body, he);
        assert_torn_op_panics(
            "between_edge_is_section (curve)",
            &mut body,
            &[&named, ROW_FOUR, OPERATORS_KEEP_LINKS],
            |b| super::between_edge_is_section(b, &ctx, he, band()),
        );
    }

    /// `chord_spec`: a torn surface on the divided face panics, where it
    /// read as a curved face and refused as a curved chord outside the
    /// split's lane.
    #[test]
    fn the_chord_spec_panics_on_a_torn_surface() {
        let mut body = crate::test_support_fixtures::geometric_cube::<f64>(Tol::witness()).body;
        let face = body.faces().next().map(|(k, _)| k).unwrap();
        let he = first_member(&body, face);
        let u1 = body.get_half_edge(he).unwrap().start;
        let u2 = body.half_edge_end(he).unwrap();
        let leave = Departure {
            dir: Vec3::unit_x(),
            datum: Datum::Germ,
        };
        let spec = |b: &mut Body<f64>| {
            super::chord_spec(b, band(), JoinLane::Planar, face, u1, u2, leave).map(|s| s.is_some())
        };
        assert!(
            !spec(&mut body).unwrap(),
            "a planar face's chord is the straight one"
        );
        let surface = body.get_face(face).unwrap().surface;
        body.surfaces.remove(surface);
        let named = format!(
            "{}'s surface names {}",
            EntityId::Face(face),
            GeomRef::Surface(surface)
        );
        assert_torn_op_panics(
            "chord_spec",
            &mut body,
            &[&named, ROW_FOUR, OPERATORS_KEEP_LINKS],
            spec,
        );
    }

    /// `along_edge_spec`: a torn curve on the segment's edge, and a torn
    /// point under a chord end, panic, where they refused typed; a stale
    /// segment edge or chord end, keys the join carries, refuses typed.
    #[test]
    fn the_along_edge_spec_panics_on_a_torn_curve_and_refuses_a_stale_edge() {
        let mut body = crate::test_support_fixtures::geometric_cube::<f64>(Tol::witness()).body;
        let face = body.faces().next().map(|(k, _)| k).unwrap();
        let he = first_member(&body, face);
        let edge = body.get_half_edge(he).unwrap().edge;
        let u1 = body.get_half_edge(he).unwrap().start;
        let u2 = body.half_edge_end(he).unwrap();
        let spec = |b: &Body<f64>, segment| {
            super::along_edge_spec(b, &JoinLane::AlongEdge, Some(segment), face, u1, u2)
                .map(|s| s.is_some())
        };
        assert!(
            spec(&body, edge).unwrap(),
            "a line segment's chord is its own line"
        );
        let data = body.get_edge(edge).unwrap().clone();
        let gone = body.edges.insert(data);
        body.edges.remove(gone);
        assert!(
            matches!(
                spec(&body, gone),
                Err(SplitJoinError::SectionInvariant {
                    what: "the segment's edge no longer resolves",
                    ..
                })
            ),
            "a stale segment edge is the boolean's key, refused typed"
        );
        let mut stale = body.clone();
        stale.vertices.remove(u2);
        assert!(
            matches!(
                spec(&stale, edge),
                Err(SplitJoinError::SectionInvariant {
                    what: "a chord end along the segment's edge no longer resolves",
                    ..
                })
            ),
            "a chord end that no longer resolves is the join's key, refused typed"
        );
        let mut torn = body.clone();
        let point = torn.get_vertex(u2).unwrap().point;
        torn.points.remove(point);
        let named = format!(
            "{}'s point names {}",
            EntityId::Vertex(u2),
            GeomRef::Point(point)
        );
        assert_torn_op_panics(
            "along_edge_spec (point)",
            &mut torn,
            &[&named, ROW_FOUR, OPERATORS_KEEP_LINKS],
            |b| spec(b, edge),
        );
        let named = drop_curve(&mut body, he);
        assert_torn_op_panics(
            "along_edge_spec (curve)",
            &mut body,
            &[&named, ROW_FOUR, OPERATORS_KEEP_LINKS],
            |b| spec(b, edge),
        );
    }

    /// `along_edge_spec`'s circle arm, on a cylinder sheet's rim: the
    /// segment edge's `he_plus` and that half-edge's `next` are links.
    #[test]
    fn the_along_edge_arc_panics_on_a_torn_half_edge() {
        let (body, face) = cyl_sheet();
        let edge = body
            .loop_cycle(first_member(&body, face))
            .unwrap()
            .into_iter()
            .map(|h| body.get_half_edge(h).unwrap().edge)
            .find(|&e| {
                body.get_curve_geom(body.get_edge(e).unwrap().curve)
                    .and_then(super::CurveGeom::certified)
                    .is_some_and(|c| matches!(c.carrier(), geom::Curve3::Circle { .. }))
            })
            .unwrap();
        let he = body.get_edge(edge).unwrap().he_plus;
        let u1 = body.get_half_edge(he).unwrap().start;
        let u2 = body.half_edge_end(he).unwrap();
        let spec = |b: &Body<f64>| {
            super::along_edge_spec(b, &JoinLane::AlongEdge, Some(edge), face, u1, u2)
                .map(|s| s.is_some())
        };
        assert!(spec(&body).unwrap(), "a rim segment's chord is its own arc");

        let mut torn = body.clone();
        torn.half_edges.remove(he);
        let named = format!(
            "{}'s he_plus names {}",
            EntityId::Edge(edge),
            EntityId::HalfEdge(he)
        );
        assert_torn_op_panics(
            "along_edge_spec (he_plus)",
            &mut torn,
            &[&named, ROW_FOUR, OPERATORS_KEEP_LINKS],
            |b| spec(b),
        );

        let mut torn = body.clone();
        let next = torn.get_half_edge(he).unwrap().next;
        torn.half_edges.remove(next);
        let named = format!(
            "{}'s next names {}",
            EntityId::HalfEdge(he),
            EntityId::HalfEdge(next)
        );
        assert_torn_op_panics(
            "along_edge_spec (end)",
            &mut torn,
            &[&named, ROW_FOUR, OPERATORS_KEEP_LINKS],
            |b| spec(b),
        );
    }
}

/// [`section_reach`] on a cone wall: the window along the axis it hands
/// the plane×cone arm holds every point the face consumes.
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod cone_window_rows {
    use geom_core::{Band, Point3, Tol, Vec3};

    use super::{SectionReach, section_reach};

    /// **A cone wall's window holds its face, and no more where the face
    /// is a patch.** Patches `u ∈ [0, u1] × z ∈ [z0, z1]` of the cone of
    /// half-angle `1/2` about `z` from the origin
    /// (`boxes::tests::cone_wall`), read from a corner: the window is
    /// `[z0, z1]` to rounding under three quarters of a turn, and holds
    /// the apex past it, where a boundary may enclose it. A window that
    /// misses any of the face lets the arm serve a line pair over points
    /// nearer the apex than it reads.
    #[test]
    fn a_cone_walls_window_holds_its_face() {
        let band = Band::linear(Tol::witness()).unwrap();
        let cone = geom::Surface::Cone {
            apex: Point3::origin(),
            axis: Vec3::unit_z(),
            half_angle: 0.5,
            u_ref: Vec3::unit_x(),
        };
        for (u1, (z0, z1)) in [
            (1.0, (0.5, 1.0)),
            (1.0, (0.01, 2.0)),
            (4.0, (0.25, 0.75)),
            (5.0, (0.25, 0.75)),
        ] {
            let label = format!("u ∈ [0, {u1}], z ∈ [{z0}, {z1}]");
            let (body, face) = crate::boolean::boxes::tests::cone_wall(0.5, 0.0, u1, z0, z1);
            for (at, _) in body.vertices() {
                let SectionReach::Cone(reach) =
                    section_reach(&body, at, face, (&cone, None), band).unwrap()
                else {
                    panic!("{label}: a cone wall's face is measured as one");
                };
                let (lo, hi) = reach.range_along(Point3::origin(), Vec3::unit_z());
                assert!(
                    lo <= z0 + 1e-12 && hi >= z1 - 1e-12,
                    "{label}, from {at:?}: the window [{lo}, {hi}] misses some of the face"
                );
                if u1 < 1.5 * core::f64::consts::PI {
                    assert!(
                        lo >= z0 - 1e-12 && hi <= z1 + 1e-12,
                        "{label}, from {at:?}: the window [{lo}, {hi}] reaches past the face"
                    );
                } else {
                    assert!(
                        lo <= 0.0,
                        "{label}, from {at:?}: past three quarters of a turn the window \
                         [{lo}, {hi}] holds the apex"
                    );
                }
            }
        }
    }
}

/// A segment curve run back from its other half.
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod segment_curve_tests {
    use super::*;
    use slotmap::KeyData;

    /// **A tangent chord the must-carry rule describes conventionally
    /// runs back like an intrinsic one**: its chart image is derived
    /// from the carrier, so the reversed carrier carries it. A stated
    /// image and a declaration do not follow it, and a chord is never a
    /// wrap edge, so each of those refuses.
    #[test]
    fn a_derived_chart_chord_runs_back_and_a_stated_image_refuses() {
        let he = |n: u64| HalfEdgeKey::from(KeyData::from_ffi(n));
        let face = FaceKey::from(KeyData::from_ffi(3));
        let surface = SurfaceKey::default();
        let line = geom::Curve3::Line {
            origin: Point3::new(0.0, 0.0, 0.0),
            dir: Vec3::new(0.0, 0.0, 1.0),
        };
        let spec = |description| EdgeCurveSpec {
            description,
            carrier: line.clone(),
            param_start: 0.25,
            param_end: 1.0,
        };
        let derived = SegmentCurve::of(
            (he(1), he(2)),
            Some(spec(geom_brep::EdgeDescriptionSpec::chart(surface))),
        );
        let back = derived
            .running_from(he(2), face)
            .expect("a derived chart image runs back")
            .expect("a curved chord");
        assert!(
            matches!(
                back.description,
                geom_brep::EdgeDescriptionSpec::Chart {
                    surface: s,
                    image: None,
                    declared: None,
                    ..
                } if s == surface
            ),
            "the description travels: {:?}",
            back.description
        );
        assert_eq!(
            (back.param_start, back.param_end),
            (-1.0, -0.25),
            "the interval runs back"
        );
        assert!(
            back.carrier.eval(-1.0).distance(line.eval(1.0)) == 0.0,
            "from the far end"
        );
        let stated = SegmentCurve::of(
            (he(1), he(2)),
            Some(spec(geom_brep::EdgeDescriptionSpec::chart_image(
                surface,
                geom_brep::Pcurve::IsoLine {
                    p0: geom_core::Point2::new(0.0, 0.0),
                    pl: geom_core::Vec2::new(0.0, 1.0),
                },
            ))),
        );
        let declared = SegmentCurve::of(
            (he(1), he(2)),
            Some(spec(
                geom_brep::EdgeDescriptionSpec::chart(surface).declared_by(
                    geom_brep::MappedCurve {
                        source: geom_brep::MappedSource::ExtrudedPoint {
                            point: geom_core::Point2::new(0.0, 0.0),
                            place: geom_core::Affine3::identity(),
                            vec: Vec3::new(0.0, 0.0, 1.0),
                        },
                        range: geom_brep::SubRange::whole(),
                    },
                ),
            )),
        );
        let wrap = SegmentCurve::of(
            (he(1), he(2)),
            Some(spec(geom_brep::EdgeDescriptionSpec::wrap(surface))),
        );
        for (label, curve) in [
            ("a stated image", &stated),
            ("a declaration", &declared),
            ("a wrap", &wrap),
        ] {
            assert!(
                matches!(
                    curve.running_from(he(2), face),
                    Err(SplitJoinError::SectionInvariant { .. })
                ),
                "{label} does not run back"
            );
            assert!(
                curve.running_from(he(1), face).is_ok(),
                "{label} runs forward as computed"
            );
        }
    }
}

/// **A run's joint branch is a literal.** Over a box, an entry `k`
/// periods from the previous exit as polynomials that are not constants,
/// on a cylinder, at `k` either side of zero: the branch less `k` is the
/// zero polynomial. And on a narrow cone (`sin α ≈ 0.01`), a continuous
/// joint at a vertex decided clear of the apex but with its lever
/// `d·sin α` under the band takes a lift of its own point by the
/// near-pole read, where every mark reads Zero or in band: the image
/// itself at a zero gap, the image a period over at a whole turn's.
#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod nearest_branch_rows {
    use super::*;
    use geom_core::{Interval, Point3, Real, Sym, Vec3};

    #[test]
    fn the_nearest_branch_is_a_literal_over_a_box() {
        let band = Band::linear(Tol::witness()).unwrap();
        for k in [-7.0, -1.0, 0.0, 1.0, 2.0, 5.0] {
            let (out, counts) =
                crate::test_support_fixtures::whole_period_gap_over_a_box(k, |a, b| {
                    let f = Sym::<Interval>::from_f64;
                    let cylinder = geom::Surface::Cylinder {
                        origin: Point3::new(f(0.0), f(0.0), f(0.0)),
                        axis: Vec3::new(f(0.0), f(0.0), f(1.0)),
                        radius: f(2.0),
                        u_ref: Vec3::new(f(1.0), f(0.0), f(0.0)),
                    };
                    let vertex = Point3::new(f(2.0), f(0.0), f(0.0));
                    let got =
                        nearest_branch(&cylinder, FaceKey::default(), vertex, a - b, band).unwrap();
                    decide(
                        "nearest_branch_row",
                        Margin::of(f(f64::from(got)) - f(k)),
                        band,
                    )
                });
            assert!(
                matches!(out, Ok(Sign::Zero)),
                "k = {k}: the branch is {k} periods: {out:?}"
            );
            assert_eq!(
                counts.symbolic, 1,
                "k = {k}: the branch less k is the zero polynomial"
            );
        }
    }

    /// **A gap on a mark answers by the band at the lever, however many
    /// turns out**, as the loop walk's joint does: the branches astride
    /// the walk's mark sit `π·lever` off, so in band the split escalates
    /// and past it the gap names no common branch.
    #[test]
    fn a_half_turn_gap_answers_by_the_band_at_every_turn_count() {
        let band = Band::linear(Tol::witness()).unwrap();
        let alpha = 0.01f64;
        let cone = geom::Surface::Cone {
            apex: Point3::new(0.0, 0.0, 0.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
            half_angle: alpha,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let (pi, tau) = (core::f64::consts::PI, core::f64::consts::TAU);
        let esc = band.escalate();
        let at = |lever: f64, gap: f64| {
            let d = lever / alpha.sin();
            let vertex = Point3::new(lever, 0.0, d * alpha.cos());
            nearest_branch(&cone, FaceKey::default(), vertex, gap, band)
        };
        for f in [0.55, 0.7, 0.75, 0.85, 0.95, 0.99] {
            let lever = f * esc / pi;
            for turns in [0.5, -0.5, 1.5, -1.5, 2.5, -2.5, 3.5, -3.5] {
                let got = at(lever, turns * tau);
                assert!(
                    matches!(got, Err(SplitJoinError::Escalated { .. })),
                    "π·lever {f}·escalate, gap {turns}τ: in band: {got:?}"
                );
            }
        }
        let past = 1.01 * esc / pi;
        for turns in [0.5, -2.5, 3.5] {
            let got = at(past, turns * tau);
            assert!(
                matches!(got, Err(SplitJoinError::SectionInvariant { .. })),
                "π·lever 1.01·escalate, gap {turns}τ: decided off: {got:?}"
            );
        }
        for (turns, k) in [(2.0, 2), (-2.0, -2)] {
            let got = at(0.75 * esc / pi, turns * tau);
            assert!(
                matches!(got, Ok(b) if b == k),
                "gap {turns}τ: the branch is {k}: {got:?}"
            );
        }
    }

    #[test]
    fn a_narrow_cone_joint_clear_of_the_apex_takes_a_lift_of_its_point() {
        let band = Band::linear(Tol::witness()).unwrap();
        let alpha = 0.01f64;
        let cone = geom::Surface::Cone {
            apex: Point3::new(0.0, 0.0, 0.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
            half_angle: alpha,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        // From the band's escalate edge, where the incidence first decides
        // the vertex clear of the apex and its lever is a hundredth of
        // that, to a thousand times it, where every mark decides.
        for d in [1.0, 5.0, 10.0, 100.0, 1000.0].map(|m| m * band.escalate()) {
            let vertex = Point3::new(d * alpha.sin(), 0.0, d * alpha.cos());
            assert_eq!(
                decide("cone_row_apex", Margin::of(d), band),
                Ok(Sign::Positive),
                "d = {d:e}: the vertex is decided clear of the apex"
            );
            let lever = d * alpha.sin();
            let got = nearest_branch(&cone, FaceKey::default(), vertex, 0.0, band);
            assert!(
                matches!(got, Ok(0)),
                "d = {d:e} (lever {lever:e}): the continuous joint is its own branch: {got:?}"
            );
            // A whole turn's gap meets the image a period over; where a
            // whole turn at the lever is itself within ε, every branch
            // tried is, and the image itself is taken first.
            let tau = core::f64::consts::TAU;
            for turns in [1.0, -1.0] {
                let want = if tau * lever < band.zero() {
                    0.0
                } else {
                    turns
                };
                let got = nearest_branch(&cone, FaceKey::default(), vertex, turns * tau, band);
                assert!(
                    matches!(got, Ok(k) if f64::from(k) == want),
                    "d = {d:e} (lever {lever:e}), gap {turns}·τ: the branch is {want}: {got:?}"
                );
            }
            // Half a turn round: within ε at the lever the image is taken;
            // in band the two ends may be one point, an escalation; past
            // the band the gap names no common branch.
            let pi_lever = core::f64::consts::PI * lever;
            let got = nearest_branch(&cone, FaceKey::default(), vertex, 0.5 * tau, band);
            let read = if pi_lever < band.zero() {
                matches!(got, Ok(0))
            } else if pi_lever < band.escalate() {
                matches!(got, Err(SplitJoinError::Escalated { .. }))
            } else {
                matches!(got, Err(SplitJoinError::SectionInvariant { .. }))
            };
            assert!(read, "d = {d:e} (lever {lever:e}), gap π: {got:?}");
        }
    }
}

/// The strict branch at a sphere pole ([`pole_branch`]): its lever, the
/// same-edge whole turn, and the refusal on a mark.
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod pole_branch_rows {
    use super::*;
    use geom_core::{Interval, Point3, Real, Vec3};

    fn band() -> Band {
        Band::linear(Tol::witness()).unwrap()
    }

    /// The unit sphere about `z`, its north pole at `z = 1`.
    fn sphere<T: Real>() -> (Point3<T>, T, Vec3<T>) {
        let f = T::from_f64;
        (
            Point3::new(f(0.0), f(0.0), f(0.0)),
            f(1.0),
            Vec3::new(f(0.0), f(0.0), f(1.0)),
        )
    }

    /// The unit sphere's meridian at azimuth `phi`, as a circle whose
    /// parameter `t` is the polar angle from the north pole.
    fn meridian<T: Real>(phi: f64) -> geom::Curve3<T> {
        let f = T::from_f64;
        let (s, c) = phi.sin_cos();
        geom::Curve3::Circle {
            center: Point3::new(f(0.0), f(0.0), f(0.0)),
            // `axis × u_ref` is the azimuth direction's meridian tangent.
            axis: Vec3::new(f(-s), f(c), f(0.0)),
            radius: f(1.0),
            u_ref: Vec3::new(f(0.0), f(0.0), f(1.0)),
        }
    }

    /// The circle through the north pole cut by the plane whose normal
    /// leans `beta` from the axis toward `+x`: radius `sin β`, geodesic
    /// curvature radius `tan β`, leaving the pole along `−y` (azimuth
    /// `−π/2`). `t = 0` is the pole, `t = π` the point across the circle
    /// from it.
    fn small_circle(beta: f64) -> geom::Curve3<f64> {
        let (s, c) = beta.sin_cos();
        geom::Curve3::Circle {
            center: Point3::new(s * c, 0.0, c * c),
            axis: Vec3::new(s, 0.0, c),
            radius: s,
            u_ref: Vec3::new(-c, 0.0, s),
        }
    }

    fn joint<T: geom_core::Decide>(gap: T, same_edge: bool) -> PoleJoint<T> {
        PoleJoint {
            face: FaceKey::default(),
            surface: sphere(),
            pole_height: T::one(),
            advancing: true,
            gap,
            prev_k: 0,
            same_edge,
        }
    }

    /// **The lever is the stub's reach, not the sphere's radius**: two
    /// meridian stubs leaving the pole `θ` apart, each `s` long, with
    /// `θ·s` a thousandth of ε while `θ·R` is a hundred escalation
    /// bands. Everywhere along them the two edges are within ε, so the
    /// pole reads them as leaving along one direction and refuses; a
    /// lever at the sphere's radius would decide them apart and hand
    /// back a branch.
    #[test]
    fn a_short_stub_within_eps_everywhere_refuses_at_its_mark() {
        let band = band();
        let theta = 100.0 * band.escalate();
        let s = band.zero() / (1000.0 * theta);
        let (a, b) = (meridian::<f64>(0.0), meridian::<f64>(theta));
        let lever = pole_lever((&a, 0.0, s), sphere(), 1.0).unwrap();
        assert!(
            (lever - s.sin()).abs() <= 1e-15 * s,
            "θ = {theta:e}, s = {s:e}: the stub's lever is its reach sin s, not R = 1: {lever:e}"
        );
        let got = pole_branch(joint(-theta, false), (&a, 0.0, s), (&b, 0.0, s), band);
        assert!(
            matches!(got, Err(SplitJoinError::PoleEdgesCoincide { .. })),
            "θ = {theta:e}, s = {s:e}: the stubs are within ε everywhere: {got:?}"
        );
        // The same angle on full meridians, pole to pole, is a wedge.
        let got = pole_branch(joint(-theta, false), (&a, 0.0, PI), (&b, 0.0, PI), band);
        assert_eq!(
            got.ok(),
            Some(0),
            "θ = {theta:e}: full meridians leave the pole definitely apart, the entry on \
             its own branch just past the exit"
        );
    }

    /// **A curved edge's lever is capped at its geodesic curvature
    /// radius**: a small circle through the pole sweeping most of its
    /// circle reaches nearly its diameter from the axis, but bends away
    /// from its pole tangent over `tan β`. A meridian leaving `θ` off
    /// that tangent, `θ·tan β` under ε, is within ε of it over the bend,
    /// and the pole refuses; levered at the reach the same wedge would
    /// read in band.
    #[test]
    fn a_curved_edge_through_the_pole_is_levered_at_its_curvature_radius() {
        let band = band();
        let beta: f64 = 0.3;
        let far = 0.9 * PI;
        let curved = small_circle(beta);
        let end = curved.eval(far);
        let reach = end.x.hypot(end.y);
        let lever = pole_lever((&curved, 0.0, far), sphere(), 1.0).unwrap();
        assert!(
            (lever - beta.tan()).abs() <= 1e-12 && lever < 0.75 * reach,
            "β = {beta}: the lever is tan β = {:e}, under the reach {reach:e}: {lever:e}",
            beta.tan()
        );
        let theta = 0.9 * band.zero() / beta.tan();
        let m = meridian::<f64>(-PI / 2.0 + theta);
        let got = pole_branch(
            joint(-theta, false),
            (&m, 0.0, PI),
            (&curved, 0.0, far),
            band,
        );
        assert!(
            matches!(got, Err(SplitJoinError::PoleEdgesCoincide { .. })),
            "θ = {theta:e}: a meridian within ε of the curved edge over its bend: {got:?}"
        );
        // A meridian half a turn off is clear at the same lever.
        let m = meridian::<f64>(PI / 2.0);
        let got = pole_branch(joint(-PI, false), (&m, 0.0, PI), (&curved, 0.0, far), band);
        assert_eq!(
            got.ok(),
            Some(0),
            "a half-turn wedge against the curved edge"
        );
    }

    /// **The same edge in and out is a whole turn, as a literal**: the
    /// walk enters the pole along an edge and leaves along it, so the
    /// face wraps the pole and the entry is the previous branch one turn
    /// on (advancing) or back (returning), at `f64` and over an interval
    /// gap straddling the jump alike.
    #[test]
    fn the_same_edge_in_and_out_is_a_whole_turn_at_every_scalar() {
        let band = band();
        let a = meridian::<f64>(0.0);
        for (advancing, want) in [(true, 4), (false, 2)] {
            let at_f64 = pole_branch(
                PoleJoint {
                    advancing,
                    prev_k: 3,
                    ..joint(3.0 * TAU, true)
                },
                (&a, 0.0, PI),
                (&a, 0.0, PI),
                band,
            );
            assert_eq!(at_f64.ok(), Some(want), "f64, advancing {advancing}");
            let ai = meridian::<Interval>(0.0);
            let straddle = Interval::from_f64(3.0 * TAU) + Interval::from_bounds(-1e-15, 1e-15);
            let at_interval = pole_branch(
                PoleJoint {
                    advancing,
                    prev_k: 3,
                    ..joint(straddle, true)
                },
                (&ai, Interval::zero(), Interval::pi()),
                (&ai, Interval::zero(), Interval::pi()),
                band,
            );
            assert_eq!(
                at_interval.ok(),
                Some(want),
                "Interval over a gap straddling the jump, advancing {advancing}"
            );
        }
    }

    /// **Two distinct edges on one direction refuse, typed, naming both
    /// readings**: two whole meridians at one azimuth (an overlap),
    /// whatever the scalar.
    #[test]
    fn distinct_edges_on_a_mark_refuse_as_an_overlap_or_a_cusp() {
        let band = band();
        let a = meridian::<f64>(0.5);
        let got = pole_branch(joint(0.0, false), (&a, 0.0, PI), (&a, 0.0, PI), band);
        let Err(e @ SplitJoinError::PoleEdgesCoincide { .. }) = got else {
            panic!("an exact overlap at the pole: {got:?}");
        };
        let said = e.to_string();
        assert!(
            said.contains("overlap") && said.contains("cusp"),
            "the refusal names both readings: {said}"
        );
        let ai = meridian::<Interval>(0.5);
        let got = pole_branch(
            joint(Interval::from_f64(TAU), false),
            (&ai, Interval::zero(), Interval::pi()),
            (&ai, Interval::zero(), Interval::pi()),
            band,
        );
        assert!(
            matches!(got, Err(SplitJoinError::PoleEdgesCoincide { .. })),
            "Interval, a whole turn's gap: {got:?}"
        );
        // In band: the wedge's displacement at the lever sits between
        // the zero and escalation bands.
        let theta = 0.5 * (band.zero() + band.escalate());
        let b = meridian::<f64>(0.5 + theta);
        let got = pole_branch(joint(-theta, false), (&a, 0.0, PI), (&b, 0.0, PI), band);
        assert!(
            matches!(got, Err(SplitJoinError::Escalated { .. })),
            "θ = {theta:e} at a unit lever is in band: {got:?}"
        );
    }

    /// **An edge whose far end is back at the pole is levered at its
    /// furthest point, not its end**: a closed meridian and a closed
    /// small circle through the pole (and one a hair short of closing)
    /// reach their antipodes, so a quarter-turn wedge against a meridian
    /// decides its branch, the entry a turn on.
    #[test]
    fn an_edge_closing_back_at_the_pole_is_levered_at_its_antipode() {
        let band = band();
        let beta: f64 = 0.3;
        let curved = small_circle(beta);
        let m = meridian::<f64>(0.0);
        for far in [TAU, TAU - 1e-9] {
            let lever = pole_lever((&curved, 0.0, far), sphere(), 1.0).unwrap();
            assert!(
                (lever - beta.tan()).abs() <= 1e-12,
                "far = {far}: the closed small circle reaches sin 2β past its cap tan β: \
                 {lever:e}"
            );
            let got = pole_branch(
                joint(PI / 2.0, false),
                (&curved, 0.0, far),
                (&m, 0.0, PI),
                band,
            );
            assert_eq!(got.ok(), Some(1), "far = {far}: a quarter-turn wedge");
        }
        let closed = meridian::<f64>(0.0);
        assert_eq!(
            pole_lever((&closed, 0.0, TAU), sphere(), 1.0),
            Some(1.0),
            "a closed meridian crosses the equator"
        );
        let got = pole_branch(
            joint(PI / 2.0, false),
            (&closed, 0.0, TAU),
            (&meridian::<f64>(-PI / 2.0), 0.0, PI),
            band,
        );
        assert_eq!(
            got.ok(),
            Some(1),
            "a quarter-turn wedge against a closed meridian"
        );
    }

    /// **In band escalates at every scalar**: a wedge whose displacement
    /// at a unit lever sits between the zero and escalation bands.
    #[test]
    fn a_wedge_in_band_escalates_at_interval_and_at_a_dual() {
        fn escalates<T: geom_core::Decide>(lane: &str) {
            let band = band();
            let theta = 0.5 * (band.zero() + band.escalate());
            let (a, b) = (meridian::<T>(0.5), meridian::<T>(0.5 + theta));
            let got = pole_branch(
                joint(T::from_f64(-theta), false),
                (&a, T::zero(), T::pi()),
                (&b, T::zero(), T::pi()),
                band,
            );
            assert!(
                matches!(got, Err(SplitJoinError::Escalated { .. })),
                "[{lane}] θ = {theta:e} at a unit lever is in band: {got:?}"
            );
        }
        escalates::<Interval>("Interval");
        escalates::<geom_core::Dual64>("Dual64");
    }

    use core::f64::consts::{PI, TAU};
}

/// **A run edge between two chords that is a spiric or a spline
/// refuses typed, naming the edge and its face**: its midpoint's side
/// of the cut's plane says nothing of the rest of it. The sweep rows'
/// sheets (`boolean::reduce::planar_lane_carrier_rows`), each cut by a
/// plane through the arc's ends, which a midpoint reading would call
/// off the plane.
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod run_edge_carrier_rows {
    use super::{SectionCtx, SplitJoinError, between_edge_is_section};
    use crate::boolean::reduce::planar_lane_carrier_rows::{arc_sheet, spiric_cap};
    use geom_core::{Band, Point3, Tol, UnitVec3, Vec3};

    #[test]
    fn a_curved_run_edge_refuses_naming_itself() {
        let band = Band::linear(Tol::witness()).unwrap();
        let cuts = [
            ("spline", arc_sheet(1.0, 1.0), Vec3::new(0.0, 1.0, 0.0)),
            ("spiric", spiric_cap(), Vec3::new(0.0, 1.0, 0.0)),
        ];
        for (name, (body, edge), normal) in cuts {
            let he = body.get_edge(edge).unwrap().he_plus;
            let face = body.face_of_linked(he);
            let origin = body.half_edge_start_point(he).unwrap();
            let ctx = SectionCtx {
                origin: Point3::new(origin.x, origin.y, origin.z),
                normal: UnitVec3::new(normal, "run_edge_rows", band).unwrap(),
                plane_key: None,
                branch: None,
            };
            match between_edge_is_section(&body, &ctx, he, band) {
                Err(SplitJoinError::RunEdgeCarrierUnsupported { face: f, edge: e }) => {
                    assert_eq!((f, e), (face, edge), "{name}: the face and the edge");
                }
                other => panic!("{name}: expected the run-edge refusal, got {other:?}"),
            }
        }
    }
}

#[cfg(test)]
mod run_image_incidence {
    #![allow(clippy::unwrap_used)]
    use super::SplitJoinError;
    use geom_brep::recourse::{INCIDENCE_AT_POSE, INCIDENCE_UNSHOWN};
    use geom_brep::{OffMargin, PcurveCertifyError, PcurveCheck};
    use geom_core::{Band, Indeterminate, KERNEL_DEFECT_ENDING, MarginDiag};

    /// **A run edge's incidence refusal ends by the one map** (review on
    /// PR 5844, H3), through the run walk's own mapping
    /// (`run_image_refusal`): decided off, the kernel's defect; at a pose, the pose
    /// lever; undecided, its lever; a poisoned margin, the defect. Never
    /// the invariant break or the join's own coincidence lever it ended in
    /// before.
    #[test]
    fn a_run_edges_incidence_refusal_ends_by_its_read() {
        let off = |margin| PcurveCertifyError::CarrierOffChart {
            chart: geom::SurfaceKind::Cone,
            carrier: geom::CurveKind::Circle,
            why: "WHY",
            margin,
        };
        let undecided = |margin| PcurveCertifyError::Escalated {
            check: PcurveCheck::ChartIncidence,
            sample: 0,
            cause: Indeterminate {
                margin,
                band: Band::new(1e-9, 1e-8).unwrap(),
                predicate: Some("pcurve_cone_chart_incident"),
                terminal_sliver: false,
            },
        };
        let m = MarginDiag::value(4e-10);
        for (refusal, ending) in [
            (off(None), KERNEL_DEFECT_ENDING),
            (off(Some(OffMargin::InBand(m))), INCIDENCE_AT_POSE.lever),
            (undecided(MarginDiag::value(5e-9)), INCIDENCE_UNSHOWN.lever),
            (undecided(MarginDiag::INVALID), KERNEL_DEFECT_ENDING),
        ] {
            let mapped = super::run_image_refusal(
                crate::FaceKey::default(),
                geom::SurfaceKind::Cone,
                refusal.clone(),
            );
            assert!(
                matches!(mapped, SplitJoinError::RunImageIncidence { .. }),
                "{refusal:?}: {mapped:?}"
            );
            let text = mapped.to_string();
            assert!(text.contains(ending), "{refusal:?}: {text}");
            assert_eq!(
                text.matches("Recourse").count() + text.matches("defect").count(),
                1,
                "{text}"
            );
            assert!(
                !text.contains("invariant") && !text.contains("coincide"),
                "{text}"
            );
        }
    }
}
