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
//! [`CutOutcome`], [`SectionCtx`], [`face_azimuth_window`] and
//! [`SplitJoinError`] out of `splitting/join.rs`, while `splitting/`
//! reciprocated by hosting the [`JoinLane::BoolPlanar`] arm and
//! [`bool_planar_chord_spec`], which only the boolean reaches. The
//! three-way [`JoinLane`] threaded through [`chord_spec`] was the
//! visible cost of a shared core with no home of its own.
//!
//! This module is that home — a **top-level sibling** of `boolean/` and
//! `splitting/`, like [`crate::sector_shape`] and
//! [`crate::sector_face`], so neither half hosts the other's core.
//! `JoinLane::BoolPlanar` is NOT deleted by the move and was never the
//! defect: it is deliberate and argued (the planar side of a curved
//! germ pair has no chart of its own, so the azimuth window must
//! arrive by value). What changes is that both arms of a shared enum
//! now live in shared scope, instead of one lane hosting the other's.
//!
//! # The section-chord geometry
//!
//! [`chord_spec`] answers what curve a chord between two vertices of a
//! divided face rides: `None` for planar faces (the straight-chord
//! lane), and for a charted face the section conic of the face's
//! carrier against the section plane, with the ARC selected by
//! azimuth-window containment (`split_arc_window`, the M5 S9 rule) —
//! the window being the divided face's own ([`face_azimuth_window`]) or,
//! on the [`JoinLane::BoolPlanar`] arm, the partner wall's, arriving by
//! value because the planar side has no chart to compute one from.
//! A sphere section tilted against the chart's polar axis has no
//! monotone azimuth for a window to bound: on a divided sphere face its
//! arc is selected by the side of the run it leaves on
//! (`split_arc_run_side`), and the [`JoinLane::BoolPlanar`] arm, which
//! has only the mate's window, refuses it.

use geom_brep::{EdgeCurveSpec, Pcurve, chart_pcurve};
use geom_core::{
    Band, BandError, Decide, Indeterminate, InfSpeed, Margin, Point3, Real, Sign, UnitVec3, Vec3,
};
use slotmap::SecondaryMap;

use crate::body::Body;
use crate::entity::{EdgeKey, EntityId, FaceKey, HalfEdgeKey, LoopBoundary, LoopKey, VertexKey};
use crate::euler::{EulerOpError, FaceSurface, MefSite};
use crate::euler_ring::MekrSite;
use crate::face_normal;
use crate::geometry::SurfaceKey;
use crate::null::CurveGeom;
use crate::splitting::containment::{LoopContainment, PointInLoopError, point_in_carrier_loop};
use crate::splitting::rules::face_extent;
use crate::validate::decide;
use geom_core::Tol;

/// Which sub-case of the arc-side **azimuth-window containment** rule
/// refused (M5 S9). The rule selects the section arc whose azimuth
/// sweep lies inside the divided face's own window; when containment
/// does not name exactly one arc it refuses with the sub-case named,
/// never a guess (the PR 5 defect was a guess that could not be seen
/// downstream — #144).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArcWindowCase {
    /// The joined run carries no edge with a closed-form chart image,
    /// so the divided face has no azimuth window at all.
    ///
    /// A cross-loop chord reads the divided face's outer cycle
    /// ([`cross_loop_window_cycle`]), never a pierce ring's own null
    /// scaffolding, which `run_azimuth_window` steps over. A same-loop
    /// chord reads the run it co-bounds the new face with, so a run made
    /// of scaffolding alone, a frontier-carrier run and a corrupt one
    /// arrive here; this variant does not distinguish them.
    NoChartedRun,
    /// NEITHER candidate arc lies inside the window — the window is
    /// degenerate relative to the chord (an ill-conditioned operand, or
    /// a run that does not actually co-bound the face with this chord).
    NeitherContained,
    /// BOTH candidates lie inside the window: the window spans at least
    /// one full period, so containment does not distinguish the arcs.
    /// Ambiguous by construction — refused, never broken by convention.
    BothContained,
    /// The window would be read across a cone face's APEX, where every
    /// azimuth maps to one point and no branch pin carries: either a
    /// walk was asked to pin across the apex, or the face has no single
    /// lift that closes there (it meets its apex more than once, carries
    /// a ring, or reaches it from both nappes — [`cone_apex_closure`]'s
    /// `Open`). A window guessed there selects the complement arc
    /// without any later check seeing it.
    ApexUnlifted,
}

/// Why the run-side arc rule ([`select_arc_by_run_side`]) named no
/// arc — each a degeneracy of the chord against its run, refused typed
/// rather than tie-broken.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArcSideCase {
    /// The joined run carries no certified edge, so there is no side
    /// of it to read.
    NoCertifiedRun,
    /// The section leaves a run end along the run itself: it is
    /// tangent to the face boundary there, and neither candidate is on
    /// a definite side.
    TangentToRun,
    /// The run's two ends name different candidates: no candidate
    /// leaves both ends on the run's left, so neither arc bounds a
    /// region with the run — the run does not co-bound the divided face
    /// with this chord.
    EndsDisagree,
    /// A run end is a reflex corner or a cusp of the divided face,
    /// where leaving on the run's left does not put an arc inside the
    /// face's sector there.
    ReflexRunEnd,
    /// The divided face's loop holds no certified edge beside the run,
    /// so the corner at a run end, which decides whether that end's
    /// reading counts, has no second side to read.
    NothingBesideRun,
}

impl core::fmt::Display for ArcSideCase {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NoCertifiedRun => write!(
                f,
                "the joined run carries no certified edge, so it has no side to read"
            ),
            Self::TangentToRun => write!(
                f,
                "the section is tangent to the face boundary at an end of the joined run"
            ),
            Self::EndsDisagree => write!(
                f,
                "the joined run's two ends put different arcs on the face's side, so neither \
                 arc bounds a region with the run"
            ),
            Self::ReflexRunEnd => write!(
                f,
                "an end of the joined run is a reflex corner or a cusp of the divided face, \
                 where the run's side does not decide the arc"
            ),
            Self::NothingBesideRun => write!(
                f,
                "the divided face's loop carries no certified edge beside the joined run, so \
                 the corner at a run end cannot be read"
            ),
        }
    }
}

impl ArcWindowCase {
    /// Is this a **containment verdict** — a definite classification of
    /// `split_arc_window` against the run's band, and so one half of the
    /// two-tolerance pair whose other half is
    /// [`SplitJoinError::Escalated`] on the very same margin (S6, D4 ¶1
    /// addendum)? [`Self::NoChartedRun`] is not: nothing was classified
    /// there, so the shared recourse would be a false lead.
    fn is_containment_verdict(self) -> bool {
        match self {
            Self::NeitherContained | Self::BothContained => true,
            Self::NoChartedRun | Self::ApexUnlifted => false,
        }
    }
}

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

impl core::fmt::Display for ArcWindowCase {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NoChartedRun => write!(
                f,
                "the joined run carries no edge with a closed-form chart image, so the \
                 divided face has no azimuth window"
            ),
            Self::NeitherContained => write!(
                f,
                "neither arc of the section conic lies inside the divided face's azimuth \
                 window (a degenerate window)"
            ),
            Self::BothContained => write!(
                f,
                "both arcs of the section conic lie inside the divided face's azimuth \
                 window (the window spans at least one full period — an ambiguous chord)"
            ),
            Self::ApexUnlifted => write!(
                f,
                "the divided cone face's azimuth window would be read across its apex, \
                 where the chart has no azimuth: the walk reached the apex, or the face \
                 meets it in a way no single chart lift closes (twice, from both nappes, \
                 or around a ring)"
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
    /// The arc-side containment rule did not name exactly one arc
    /// (M5 S9): the sub-case says which way it failed.
    ///
    /// The two containment sub-cases are the **definite** half of a
    /// two-tolerance pair (D4 ¶1 addendum, the S6 sweep): the very same
    /// `split_arc_window` margin one band-width away escalates as
    /// [`Self::Escalated`] instead, and a user cannot tell the two
    /// situations apart from the geometry. So both halves name the
    /// predicate, both quote the band that decided, and both end on the
    /// one shared recourse carrier — composed exactly once.
    SectionArcWindow {
        /// The face being divided.
        face: FaceKey,
        /// Which containment verdict refused.
        case: ArcWindowCase,
        /// The band the containment margins were classified against —
        /// the two tolerances, so the definite verdict and its in-band
        /// neighbour read as one situation.
        band: Band,
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
    /// The boolean's PLANAR side of a plane×sphere germ pair met a
    /// section tilted against the sphere's chart polar axis
    /// (`split_sphere_section_polar`). That side selects its arc by the
    /// mate wall face's azimuth window, handed over by value, and a
    /// tilted section's azimuth is not monotone, so there is no window
    /// to select by; the wall side of the same pair, and both sides of
    /// a sphere pair, take the run-side rule instead. A deliberate
    /// typed frontier (`work/reach/planar-side-of-a-tilted-plane-sphere-cut-has-no-arc-cue.md`).
    SectionNotPolar {
        /// The planar face being divided.
        face: FaceKey,
        /// The band the tilt was decided against.
        band: Band,
    },
    /// The run-side arc rule — the one a section whose chart azimuth
    /// is not monotone takes (`split_arc_run_side`) — named no arc.
    SectionArcSide {
        /// The face being divided.
        face: FaceKey,
        /// Which degeneracy.
        case: ArcSideCase,
        /// The band the sides were decided against.
        band: Band,
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
            Self::SectionArcWindow { case, band, .. } => {
                write!(
                    f,
                    "the section through a curved face has no arc to take: {case}"
                )?;
                if case.is_containment_verdict() {
                    write!(
                        f,
                        " ('split_arc_window', band ({:e}, {:e})). Recourse: {recourse}",
                        band.zero(),
                        band.escalate(),
                    )?;
                }
                Ok(())
            }
            Self::SectionInvariant { face, what } => {
                write!(f, "curved-section invariant at face {face:?}: {what}")
            }
            Self::SectionNotPolar { band, .. } => write!(
                f,
                "the planar side of a plane×sphere cut is tilted against the sphere's polar \
                 axis, and that side takes its arc from the sphere face's azimuth window, \
                 which a tilted section has none of ('split_sphere_section_polar', band \
                 ({:e}, {:e})). Recourse: revolve the ball about the face's normal",
                band.zero(),
                band.escalate(),
            ),
            // Only the tangency is a verdict of the band; a run with no
            // certified edge or two disagreeing ends is the geometry's.
            Self::SectionArcSide { case, band, .. } => write!(
                f,
                "the section through a curved face has no arc to take: {case} \
                 ('split_arc_run_side', band ({:e}, {:e})). Recourse: {}",
                band.zero(),
                band.escalate(),
                match case {
                    ArcSideCase::TangentToRun => recourse,
                    ArcSideCase::NoCertifiedRun => {
                        "the section pierces this face as a ring, which has no join arm yet; \
                         move the geometry so the section crosses the face's boundary"
                    }
                    ArcSideCase::EndsDisagree
                    | ArcSideCase::ReflexRunEnd
                    | ArcSideCase::NothingBesideRun => "move the geometry",
                },
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
///   transient context; the divided face's own cylinder chart drives
///   the S9 azimuth-window arc selection unchanged).
/// - [`JoinLane::BoolPlanar`] — the boolean's PLANAR-side chord of a
///   curved germ pair: the divided face is the plane, the partner
///   wall arrives by value with its face's azimuth window (computed
///   by the caller from the OTHER operand), and the selected arc is
///   the one contained in that window — the same S9 statement, asked
///   of the mate's chart.
pub(crate) enum JoinLane<'a, T: Real> {
    /// Straight chords only.
    Planar,
    /// The split lane / boolean wall-side conic lane.
    Split(&'a mut SectionCtx<T>),
    /// The boolean planar-side chord of a curved germ pair.
    BoolPlanar {
        /// The partner wall surface (value; from the other operand).
        wall: geom::Surface<T>,
        /// The wall FACE's azimuth window on the wall chart.
        window: (T, T),
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
            JoinLane::BoolPlanar {
                wall,
                window,
                partner_key,
            } => JoinLane::BoolPlanar {
                wall: wall.clone(),
                window: *window,
                partner_key,
            },
            JoinLane::AlongEdge => JoinLane::AlongEdge,
        }
    }
}

/// The section conic's frame — the datum both chord lanes select an
/// arc of, in the form the arc-side rule reads it.
pub(crate) struct SectionConic<T: Real> {
    /// The conic's centre.
    center: Point3<T>,
    /// The section plane's normal — the conic's own axis, whose sign
    /// against the wall chart's axis says which candidate is ccw.
    normal: Vec3<T>,
    /// The major direction.
    major: Vec3<T>,
    /// The semi-axis along `major` (the radius, for a circle).
    sa: T,
    /// The semi-axis along `normal × major`.
    sb: T,
    /// The carrier itself.
    carrier: geom::Curve3<T>,
    /// Whether chart azimuth is monotone along the carrier — the
    /// premise of the azimuth-window rule ([`select_arc`]). A cylinder or
    /// cone conic the table admits and a polar sphere section have it; a
    /// sphere section tilted against the chart's polar axis does not,
    /// and its arc is selected by the run's side instead
    /// ([`select_arc_by_run_side`]).
    azimuth_monotone: bool,
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

/// The section of the surface PAIR `(s1, s2)` under THE C5 table, in
/// the frame the arc-side rule reads — **one implementation for both
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
/// — an exact Circle, never a fitted chord — and decides whether the
/// section is polar for the sphere's chart (`split_sphere_section_polar`):
/// the azimuth-window rule premises azimuth MONOTONE along the carrier,
/// which a sphere chart gives for a polar section, and a tilted one is
/// handed on marked for the run-side rule ([`SectionConic::azimuth_monotone`]).
/// The cylinder lane is PR 5/PR 9's `plane_cylinder_section`; the cone
/// lane is `plane_cone_section`, whose ellipse meets every generator
/// once, so its azimuth is monotone along the carrier.
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
                 plane×cylinder, plane×cone and plane×sphere, and a curved×curved pair has no arc-side \
                 rule to run; refused typed rather than defaulted to a straight chord. A \
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
    if let geom::Surface::Sphere {
        axis: sph_axis,
        radius: sph_r,
        ..
    } = wall
    {
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
        let azimuth_monotone = match decide(
            "split_sphere_section_polar",
            Margin::levered(axis.cross(*sph_axis).norm(), *sph_r),
            band,
        )
        .map_err(|diag| SplitJoinError::Escalated { face, diag })?
        {
            Sign::Zero => true,
            Sign::Positive | Sign::Negative => false,
        };
        return Ok(SectionCase::Conic(SectionConic {
            center,
            normal: axis,
            major: u_ref,
            sa: radius,
            sb: radius,
            carrier: circle,
            azimuth_monotone,
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

/// A cone wall's chart lever and nappe for the arc-side rule.
///
/// The lever is the section's farthest reach from the axis, bounded by
/// its centre's radial offset plus its semi-major axis — an honest
/// over-arm (the sphere frame takes the sphere's radius the same way),
/// and exact for the axis-normal circle. The nappe is the chord start's
/// side of the apex: the chart azimuth of a mirror-nappe point is the
/// spatial one plus `π`, and the window this chord is read against was
/// built in the chart's azimuth.
fn cone_chart_lever<T: Decide>(
    face: FaceKey,
    band: Band,
    apex: Point3<T>,
    axis: Vec3<T>,
    conic: &SectionConic<T>,
    p1: Point3<T>,
) -> Result<(T, T), SplitJoinError> {
    let offset = conic.center - apex;
    let radius = (offset - axis * offset.dot(axis)).norm() + conic.sa;
    let nappe = match decide(
        "split_cone_chord_nappe",
        Margin::of((p1 - apex).dot(axis)),
        band,
    )
    .map_err(|diag| SplitJoinError::Escalated { face, diag })?
    {
        Sign::Positive => T::one(),
        Sign::Negative => T::zero() - T::one(),
        Sign::Zero => {
            return Err(SplitJoinError::SectionInvariant {
                face,
                what: "a conic section chord starts at its cone's apex level — no conic arc \
                       of a plane off the apex reaches it",
            });
        }
    };
    Ok((radius, nappe))
}

/// The arc-side frame of a section carrier: an ellipse's own axes, a
/// circle's radius twice, its azimuth monotone (the cylinder and cone
/// conics the table admits). `None` for any other kind.
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
        azimuth_monotone: true,
    })
}

/// The wall chart an azimuth window lives in: azimuth is measured ccw
/// about `axis` from `u_ref`, and containment margins are metered at
/// `radius` (azimuth × chart radius — metres, the PR 6 convention).
struct ChartFrame<T: Real> {
    origin: Point3<T>,
    axis: Vec3<T>,
    radius: T,
    u_ref: Vec3<T>,
    /// `+1`, or `−1` on a cone's mirror nappe, whose chart azimuth is
    /// the spatial one plus `π` (the cone chart's own convention).
    nappe: T,
}

/// **The arc-side rule (M5 S9), once, for both chord lanes.** Given
/// the section conic, the chord's endpoints and the azimuth WINDOW the
/// arc has to lie inside, returns the selected arc as an oriented
/// `(carrier, t_start, t_end)` running `p1 → p2`.
///
/// Where the window comes from is the only thing the two lanes disagree
/// about, and it is a parameter: the split lane derives it from the
/// divided face's own run ([`run_azimuth_window`]), the boolean's
/// planar side is handed the MATE wall face's window by value, because
/// a plane has no chart of its own to derive one from. The margins,
/// the predicate names, the short-circuit order and the refusal cases
/// are therefore the same by construction rather than by hand-syncing
/// two copies — which is what they were.
///
/// # Errors
///
/// [`SplitJoinError::SectionArcWindow`] when containment names no
/// single arc, [`SplitJoinError::Escalated`] on an in-band boundary,
/// [`SplitJoinError::SectionInvariant`] for a section frame with no
/// chart orientation.
fn select_arc<T: Decide>(
    face: FaceKey,
    band: Band,
    chart: &ChartFrame<T>,
    conic: &SectionConic<T>,
    window: (T, T),
    p1: Point3<T>,
    p2: Point3<T>,
) -> Result<(geom::Curve3<T>, T, T), SplitJoinError> {
    // Exact conic parameters of the (on-locus) endpoints.
    let th1 = conic.param(p1);
    let th2 = conic.param(p2);
    let (w_min, w_max) = window;
    let width = w_max - w_min;
    // The chord's endpoints in the SAME chart frame the window lives
    // in (cylinder chart: azimuth ccw about the axis from `u_ref`).
    let chart_az = |p: Point3<T>| -> T {
        let w = p - chart.origin;
        let radial = (w - chart.axis * w.dot(chart.axis)) * chart.nappe;
        stable_azimuth(
            radial.dot(chart.axis.cross(chart.u_ref)),
            radial.dot(chart.u_ref),
            band,
        )
    };
    let tau = T::tau();
    let a1 = chart_az(p1);
    // Containment margins are metered as azimuth × chart radius
    // (metres — the PR 6 convention); an in-band window boundary
    // escalates F6 with that lever arm. Fixed evaluation order, no
    // data-dependent iteration (D9).
    let contains = |margin: T| -> Result<bool, SplitJoinError> {
        match decide(
            "split_arc_window",
            Margin::levered(margin, chart.radius),
            band,
        )
        .map_err(|diag| SplitJoinError::Escalated { face, diag })?
        {
            Sign::Positive | Sign::Zero => Ok(true),
            Sign::Negative => Ok(false),
        }
    };
    // A window spanning a full period contains BOTH candidates (at one
    // branch or another), so containment names no arc — and it is
    // exactly the condition under which the chord's own branch inside
    // the window is undetermined. Decided FIRST, before anything
    // depends on that branch.
    if contains(width - tau)? {
        return Err(SplitJoinError::SectionArcWindow {
            face,
            case: ArcWindowCase::BothContained,
            band,
        });
    }
    // The chord's start, window-relative: the unique branch of its
    // azimuth lying in the (now certainly sub-period) window. Found by
    // reducing against the window's CENTRE, never its edge — the
    // chord's start sits ON a window edge generically (it is one of the
    // run's own ends), and a periodic reduction taken there straddles a
    // period boundary, which at interval type widens to a full period
    // by containment honesty and would escalate every curved cut.
    //
    // `reduce_periodic_centred` is that reduction under its own name:
    // it folds the offset-from-centre once, into the window whose jump
    // is at ±τ/2, so the argument's distance to the nearest jump is
    // **(τ − width)/2** for a start inside the window —
    // half the window's COMPLEMENT, not half the window. That distance
    // is positive only because the `width ≥ τ` arm above already
    // returned, and it is that arm's own band that makes it more than
    // infinitesimally positive: a window a hair under a full period
    // escalates there rather than reaching a knife-edge reduction here.
    // On the shipped belly/tilted cuts the complement is most of a
    // period, which is why the margin is comfortable in practice.
    //
    // **"For a start inside the window" is a PREMISE, not a gate.** No
    // check on this path establishes it, and the bound is false without
    // it: a start box at the window's ANTIPODE sits on the centred
    // fold's own jump and comes back a full period wide at every window
    // width, including widths where `(τ − width)/2` is a comfortable
    // 1.57 rad. That is measured — `cert4r1_the_centred_anchoring_
    // widens_at_its_own_jump_for_a_near_whole_window` in this file's
    // tests drives it. Whether a run can present an antipodal start is
    // NOT established either way here; it is recorded as an open
    // premise rather than gated, because gating an unreached case costs
    // a decision on every cut.
    let half_w = width * T::from_f64(0.5);
    let x1 = (a1 - (w_min + half_w)).reduce_periodic_centred(tau) + half_w;
    // The azimuth gap to the chord's end. A difference, like every
    // quantity here, so a rotated `u_ref` (a moved seam) cancels.
    //
    // A FORWARD gap, so the `[0, τ)` window and not the centred one: a
    // chord may legitimately span more than half a period and its gap
    // must read as that and not as its negative complement. The
    // window's jump is therefore at a gap of zero — the two ends
    // coincident, where `0` and `τ` are genuinely both consistent with
    // an enclosure of them.
    //
    // Reaching it needs a chord whose two ends share an azimuth. That
    // a real run does not produce one is an UNENFORCED PREMISE, stated
    // as such: nothing on this path gates the gap away from zero, and
    // the two ends being distinct points does not by itself make their
    // azimuths distinct (a chord parallel to the axis has one azimuth
    // at both ends). Moving the jump would relocate it onto a
    // non-degenerate gap rather than remove it, so the window stays;
    // what is not claimed is that the degenerate case is impossible.
    let g = (chart_az(p2) - a1).reduce_periodic(tau);
    // Both ends of both candidates are checked against both ends of the
    // window — `up` = [x₁, x₁ + g] (ccw in the chart) and
    // `dn` = [x₁ − (τ − g), x₁] against [0, width]. The chord's start
    // lying in the window is a consequence of the run's own geometry,
    // not an assumption: a run that does not actually end where this
    // chord starts fails the x₁ rows and lands in `NeitherContained`.
    //
    // `&&` short-circuits, and the semantics of that are stated rather
    // than left to be discovered: an in-band boundary escalates on the
    // rows that are EVALUATED; a row skipped because an earlier
    // containment on the same candidate was definitely FALSE never gets
    // metered. That is deterministic (the order is fixed, D9) and
    // refusal-safe (the candidate is already excluded, so a skipped row
    // could only have excluded it again or escalated — never admitted
    // it), and it keeps a definite non-containment from being masked by
    // an unrelated ill-conditioned boundary on the same candidate.
    let up_in = contains(x1)? && contains(width - x1 - g)?;
    let dn_in = contains(width - x1)? && contains(x1 + g - tau)?;
    // Which candidate is the conic parameter's CCW arc: θ runs ccw
    // about the section normal n̂ₑ, chart azimuth ccw about the axis
    // âc, so they agree exactly when n̂ₑ · âc > 0. Definite for every
    // TiltedEllipse/Rim the table admits (|n̂ₑ · âc| · sₐ is the chart
    // radius); an axis-orthogonal section frame is a ruling case the
    // classification routes elsewhere, refused typed if it arrives.
    let ccw_is_up = match decide(
        "split_arc_chart_orientation",
        Margin::levered(conic.normal.dot(chart.axis), conic.sa),
        band,
    )
    .map_err(|diag| SplitJoinError::Escalated { face, diag })?
    {
        Sign::Positive => true,
        Sign::Negative => false,
        Sign::Zero => {
            return Err(SplitJoinError::SectionInvariant {
                face,
                what: "the section conic's frame is orthogonal to the cylinder axis \
                       (no chart orientation for the arc-side rule)",
            });
        }
    };
    let (ccw_in, cw_in) = if ccw_is_up {
        (up_in, dn_in)
    } else {
        (dn_in, up_in)
    };
    let ccw = match (ccw_in, cw_in) {
        (true, false) => true,
        (false, true) => false,
        (false, false) => {
            return Err(SplitJoinError::SectionArcWindow {
                face,
                case: ArcWindowCase::NeitherContained,
                band,
            });
        }
        // Unreachable in exact arithmetic once the window is under a
        // period, but reachable in the ε-shell where both containment
        // rows classify Zero on a window a hair under τ — the same
        // verdict, refused the same way rather than tie-broken.
        (true, true) => {
            return Err(SplitJoinError::SectionArcWindow {
                face,
                case: ArcWindowCase::BothContained,
                band,
            });
        }
    };
    Ok(oriented_arc(conic, th1, th2, ccw))
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
    // The arc's own span, forward from `th1` — the `[0, τ)` window for
    // the same reason [`select_arc`]'s azimuth gap takes it, and with
    // its jump in the same place: a span of zero, which is a
    // zero-length or a whole-circle arc.
    //
    // Only ONE of those two ends is guarded, and by the arm that
    // actually guards it: `BothContained` fires on `width ≥ τ`, so it
    // keeps a whole-period WINDOW off this reduction — the τ end. The
    // zero end is not its business and is not gated here; a
    // zero-length arc reaching this reduction would come back a period
    // wide, honestly, and the selection rules' classifications are what
    // make that configuration not arise rather than what forbids it.
    // On the run-side rule the window arm does not exist: there the τ
    // end is two chord ends sharing a conic parameter, which the
    // `split_arc_run_end` match refuses as a degenerate chord.
    if ccw {
        // The ccw arc from p1 lies in the face.
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

/// A point on a half-edge and its unit tangent there, in the half-edge's
/// own direction of travel.
type PointTangent<T> = (Point3<T>, Vec3<T>);

/// A half-edge's point and unit tangent, in its own direction of
/// travel, at its start (`at_start`) or its end; `None` for null
/// scaffolding, which is zero-length and has no tangent.
fn half_end<T: Decide>(
    body: &Body<T>,
    he: HalfEdgeKey,
    at_start: bool,
) -> Result<Option<PointTangent<T>>, SplitJoinError> {
    let he_data = body.get_half_edge(he).ok_or_else(|| corrupt_he(he))?;
    let edge = body
        .get_edge(he_data.edge)
        .ok_or_else(|| corrupt_edge(he_data.edge))?;
    let Some(CurveGeom::Certified(curve)) = body.get_curve_geom(edge.curve) else {
        return Ok(None);
    };
    let (t0, t1) = curve.params();
    // A minus half runs its carrier backwards: it starts at `t1`.
    let (t, sign) = match (edge.he_plus == he, at_start) {
        (true, true) => (t0, T::one()),
        (true, false) => (t1, T::one()),
        (false, true) => (t1, -T::one()),
        (false, false) => (t0, -T::one()),
    };
    let d = curve.carrier().deriv(t) * sign;
    Ok(Some((curve.carrier().eval(t), d / d.norm())))
}

/// The run's certified half-edges in order — the one walk of the run
/// the run-side rule reads its ends, its corners and its coincidence
/// from. Null scaffolding is stepped over.
fn certified_run<T: Decide>(
    body: &Body<T>,
    run: &[HalfEdgeKey],
) -> Result<Vec<HalfEdgeKey>, SplitJoinError> {
    let mut real = Vec::with_capacity(run.len());
    for &he in run {
        let he_data = body.get_half_edge(he).ok_or_else(|| corrupt_he(he))?;
        let edge = body
            .get_edge(he_data.edge)
            .ok_or_else(|| corrupt_edge(he_data.edge))?;
        if let Some(CurveGeom::Certified(_)) = body.get_curve_geom(edge.curve) {
            real.push(he);
        }
    }
    Ok(real)
}

/// The tangent of the divided face's boundary just beside a run end,
/// off the run: the first certified half-edge reached from `he` by
/// walking the loop backwards (`at_start`, the half arriving at the
/// run's start) or forwards (the half leaving its end). `None` when the
/// loop holds nothing certified but the run's own halves' scaffolding.
fn beside_run<T: Decide>(
    body: &Body<T>,
    he: HalfEdgeKey,
    at_start: bool,
) -> Result<Option<Vec3<T>>, SplitJoinError> {
    let cycle_len = body.loop_cycle(he).ok_or_else(|| corrupt_he(he))?.len();
    let mut at = he;
    for _ in 0..cycle_len {
        let data = body.get_half_edge(at).ok_or_else(|| corrupt_he(at))?;
        at = if at_start { data.prev } else { data.next };
        if let Some((_, tangent)) = half_end(body, at, !at_start)? {
            return Ok(Some(tangent));
        }
    }
    Ok(None)
}

/// Whether the run is one certified edge lying on the section conic:
/// an arc of the section the join has already minted. Decided at the
/// edge's mid-parameter — its offset from the section plane and its
/// conic residual — as `split_conic_inplane_mid` decides a between
/// edge: a conic edge on the face's carrier meets the section in at
/// most two points unless it lies on it, and its two ends are the
/// chord's.
fn run_is_section_arc<T: Decide>(
    body: &Body<T>,
    band: Band,
    face: FaceKey,
    conic: &SectionConic<T>,
    real: &[HalfEdgeKey],
) -> Result<bool, SplitJoinError> {
    let [he] = real else {
        return Ok(false);
    };
    let edge = body
        .get_half_edge(*he)
        .and_then(|h| body.get_edge(h.edge))
        .ok_or_else(|| corrupt_he(*he))?;
    let Some(CurveGeom::Certified(curve)) = body.get_curve_geom(edge.curve) else {
        return Ok(false);
    };
    if !matches!(
        curve.carrier(),
        geom::Curve3::Circle { .. } | geom::Curve3::Ellipse { .. }
    ) {
        return Ok(false);
    }
    let d = curve.mid_point() - conic.center;
    let x = d.dot(conic.major) / conic.sa;
    let y = d.dot(conic.normal.cross(conic.major)) / conic.sb;
    let escalated = |diag| SplitJoinError::Escalated { face, diag };
    let in_plane = decide(
        "split_arc_run_on_section_plane",
        Margin::of(d.dot(conic.normal)),
        band,
    )
    .map_err(escalated)?;
    if in_plane != Sign::Zero {
        return Ok(false);
    }
    let on_conic = decide(
        "split_arc_run_on_section_conic",
        Margin::levered((x.powi(2) + y.powi(2)).sqrt() - T::one(), conic.sa),
        band,
    )
    .map_err(escalated)?;
    Ok(on_conic == Sign::Zero)
}

/// Whether a corner of a face's boundary — arriving along `arrive`,
/// leaving along `depart`, both unit, the face to the left under the
/// outward `normal` — is smooth or convex: the face's sector there is
/// at most a half-turn. A left turn about the normal is convex
/// (`split_arc_run_corner` positive), none is smooth unless the boundary
/// reverses (a cusp, `split_arc_run_cusp`), and a right turn is reflex.
/// Metered as angle × `lever`.
fn run_corner_opens<T: Decide>(
    normal: Vec3<T>,
    arrive: Vec3<T>,
    depart: Vec3<T>,
    lever: T,
    band: Band,
) -> Result<bool, geom_core::Indeterminate> {
    match decide(
        "split_arc_run_corner",
        Margin::levered(normal.dot(arrive.cross(depart)), lever),
        band,
    )? {
        Sign::Positive => Ok(true),
        Sign::Negative => Ok(false),
        Sign::Zero => Ok(decide(
            "split_arc_run_cusp",
            Margin::levered(arrive.dot(depart), lever),
            band,
        )? == Sign::Positive),
    }
}

/// **The arc-side rule where azimuth is not monotone along the
/// section** — a sphere section tilted against the face's chart polar
/// axis, whose chart image doubles back, so no azimuth window says
/// which arc a face holds. Chart-free: the selected arc and the run
/// bound the divided face together, and the face lies to the LEFT of
/// the run (the interior-left rule, viewed down the face's outward
/// normal), so at each run end the arc leaves on the run's left.
///
/// Each run end is matched to the chord end it sits on (a named
/// trilean on the two distances), the ccw candidate's direction of
/// departure there is read off the conic's tangent, and its side of the
/// run is the sign of that direction against `n̂ × t̂` (outward normal
/// cross the run's travel tangent), metered as angle × section radius.
/// Exactly one candidate leaves on the left at a run end that is a
/// smooth boundary point or a convex corner of the divided face, since
/// there the face's sector at that end is at most a half-turn and the
/// two candidates leave in opposite directions — so that is checked at
/// each end before its reading counts: the turn from the boundary
/// arriving at the end to the boundary leaving it, about the outward
/// normal, must be a left turn or none (`split_arc_run_corner`; a turn
/// of none that reverses is a cusp, `split_arc_run_cusp`). A reflex
/// corner refuses typed. Both ends must name the same candidate;
/// anything else refuses typed, never broken by convention.
///
/// # Errors
///
/// [`SplitJoinError::SectionArcSide`] when the run has no certified
/// edge, when the section is tangent to the run at an end, when a run
/// end is a reflex corner or a cusp of the divided face, when the loop
/// holds no certified edge beside the run to read that corner from, or
/// when the two ends name different candidates; [`SplitJoinError::Escalated`] on
/// an in-band side, corner or end match.
#[allow(clippy::too_many_arguments)] // one internal rule, each argument a named duty
fn select_arc_by_run_side<T: Decide>(
    body: &Body<T>,
    band: Band,
    face: FaceKey,
    conic: &SectionConic<T>,
    run: &[HalfEdgeKey],
    p1: Point3<T>,
    p2: Point3<T>,
) -> Result<(geom::Curve3<T>, T, T), SplitJoinError> {
    let refuse = |case| SplitJoinError::SectionArcSide { face, case, band };
    let escalated = |diag| SplitJoinError::Escalated { face, diag };
    let real = certified_run(body, run)?;
    let (Some(&first), Some(&last)) = (real.first(), real.last()) else {
        return Err(refuse(ArcSideCase::NoCertifiedRun));
    };
    let (th1, th2) = (conic.param(p1), conic.param(p2));
    let mut ccw: Option<bool> = None;
    for (he, is_start) in [(first, true), (last, false)] {
        let Some((at, travel)) = half_end(body, he, is_start)? else {
            return Err(refuse(ArcSideCase::NoCertifiedRun));
        };
        let at_p1 = match decide(
            "split_arc_run_end",
            Margin::of((at - p2).norm() - (at - p1).norm()),
            band,
        )
        .map_err(escalated)?
        {
            Sign::Positive => true,
            Sign::Negative => false,
            Sign::Zero => {
                return Err(SplitJoinError::SectionInvariant {
                    face,
                    what: "a run end is equidistant from the chord's two ends — the chord \
                           is degenerate",
                });
            }
        };
        // The ccw candidate runs p1 → p2 with θ increasing: it leaves
        // p1 along +C′(θ₁) and leaves p2, walked back, along −C′(θ₂).
        let leave = if at_p1 {
            conic.tangent(th1)
        } else {
            -conic.tangent(th2)
        };
        let normal = match face_normal::face_outward_normal_at(body, face, at, band) {
            Ok(Some(n)) => n.vec(),
            Ok(None) => return Err(corrupt_face(face)),
            Err(face_normal::NormalAtError::Escalated { diag, .. }) => {
                return Err(escalated(diag));
            }
            Err(_) => {
                return Err(SplitJoinError::SectionInvariant {
                    face,
                    what: "the divided face has no outward normal at a run end",
                });
            }
        };
        // Sign-blind extraction is sound here: the side is read against
        // the STORED loop traversal, which `revert` reverses with the
        // sense bit (`OutwardNormal::vec`).
        let left = normal.cross(travel);
        let side = left.dot(leave) / (left.norm() * leave.norm());
        let reading = decide("split_arc_run_side", Margin::levered(side, conic.sa), band)
            .map_err(escalated)?;
        let here = match reading {
            Sign::Positive | Sign::Negative => {
                // The reading counts only at a smooth or convex corner
                // of the divided face (fn docs).
                let beside =
                    beside_run(body, he, is_start)?.ok_or(refuse(ArcSideCase::NothingBesideRun))?;
                let (arrive, depart) = if is_start {
                    (beside, travel)
                } else {
                    (travel, beside)
                };
                if !run_corner_opens(normal, arrive, depart, conic.sa, band).map_err(escalated)? {
                    return Err(refuse(ArcSideCase::ReflexRunEnd));
                }
                reading == Sign::Positive
            }
            // The divided face's corner at this end has no opening: the
            // chord would run along the run. That is the sliver the
            // join's second chord bounds against its first — its run IS
            // the first chord, an arc of this very section — and then
            // the chord takes that arc, leaving along the run at the
            // run's start and back along it at the run's end. Any other
            // tangency is refused.
            Sign::Zero => {
                if !run_is_section_arc(body, band, face, conic, &real)? {
                    return Err(refuse(ArcSideCase::TangentToRun));
                }
                let along = travel.dot(leave) / leave.norm();
                let forward = match decide(
                    "split_arc_run_along",
                    Margin::levered(along, conic.sa),
                    band,
                )
                .map_err(escalated)?
                {
                    Sign::Positive => true,
                    Sign::Negative => false,
                    Sign::Zero => return Err(refuse(ArcSideCase::TangentToRun)),
                };
                forward == is_start
            }
        };
        match ccw {
            Some(prev) if prev != here => return Err(refuse(ArcSideCase::EndsDisagree)),
            _ => ccw = Some(here),
        }
    }
    let ccw = ccw.ok_or(refuse(ArcSideCase::NoCertifiedRun))?;
    Ok(oriented_arc(conic, th1, th2, ccw))
}

/// The chord spec for dividing `face` between vertices `u1 → u2`
/// (the mef/mekr `he_plus` direction): `None` for planar faces and
/// for ruling sections (the straight chord IS the honest carrier —
/// the M3 lane, bit-identical), `Some(spec)` with the C5 conic arc
/// for curved faces.
///
/// The conic lane (M5 PR 5):
///
/// 1. Classify (plane × wall surface) through THE table's
///    [`geom_brep::plane_cylinder_section`] — trileans before any
///    rung; `TiltedEllipse`/`Rim` proceed, `ParallelLines` falls back
///    to the ruling chord, `TangentLine` refuses typed (C7),
///    escalations pass through whole.
/// 2. Select WHICH arc of the section conic lies in `face` by
///    **azimuth-window containment** (M5 S9, repairing the PR 5
///    RUN-sample rule — see the selection note below).
/// 3. Describe as `Intersection { wall, aux plane, witness }` with the
///    witness minted at the carrier's mid-parameter (the witness
///    contract) — certification then pins endpoints, residuals, and
///    transversality through the ordinary gate.
///
/// # The arc-side rule: azimuth-window containment (M5 S9)
///
/// The stored arc must lie inside the **divided face's own azimuth
/// window** — the same statement M5 PR 6 certifies for pcurves
/// ([`crate::pcurves`]), evaluated here at selection time.
///
/// - The window comes from the RUN this chord co-bounds the face with
///   (`run`, the real halves between the two null halves being joined):
///   each run edge's chart image is derived in closed form through
///   [`geom_brep::chart_pcurve`], branch-pinned to its predecessor's
///   exit exactly as PR 6's loop walk does, and the hull of their
///   **exact** azimuth extents is the window. Nothing is sampled.
/// - The chord's two complementary candidates are closed azimuth
///   intervals anchored at its start, `[x₁, x₁ + g]` (ccw in the chart)
///   and `[x₁ − (τ − g), x₁]`, with `g` the endpoints' azimuth gap;
///   the selected arc is the one **contained** in the window. Exactly
///   one is contained whenever the window is narrower than a period
///   (the two candidates' unions cover the circle and only one can fit
///   inside a sub-period window), so neither/both are genuine
///   degeneracies and refuse typed with the sub-case named
///   ([`ArcWindowCase`]) — never a guess.
/// - Every containment margin is metered as **azimuth × chart radius**
///   (metres, the PR 6 convention) through the named trilean
///   `split_arc_window`; an in-band window boundary escalates F6.
///   Which candidate is the conic parameter's ccw arc is itself a named
///   trilean, `split_arc_chart_orientation` (θ runs ccw about the
///   section normal, azimuth ccw about the cylinder axis: they agree
///   iff `n̂ₑ · âc > 0`). The cw arc takes the axis-flipped frame so the
///   carrier still runs forward `u1 → u2`.
///
/// **Why the PR 5 rule was wrong** (#144, and the history note in
/// `sweep/tests/m5_pr5_tilted_cut.rs`): it decided the side from a
/// single azimuth *sample* on the run, premised on that sample lying
/// inside the chord's own interval. That premise fails whenever the
/// divided face spans more azimuth than the chord — the tilted belly
/// cut, where a 91° rim run bounds a face closed by 17.5° and 44.4°
/// section arcs — and the rule then selected the complement arc, a body
/// no tier-3 check could reject. Containment asks about the face, not
/// about a point.
///
/// Seam placement does not enter: rotating the chart's `u_ref` shifts
/// the window and the chord's endpoint azimuths by the same constant,
/// and every quantity below is a difference.
///
/// A sphere section tilted against the chart's polar axis doubles back
/// in azimuth, so no window bounds its arc; that section takes
/// [`select_arc_by_run_side`] on the same run instead.
#[allow(clippy::too_many_arguments)] // one internal lane, each argument a named duty
fn chord_spec<T: Decide>(
    body: &mut Body<T>,
    band: Band,
    lane: JoinLane<'_, T>,
    face: FaceKey,
    run: ChordRun<'_>,
    u1: VertexKey,
    u2: VertexKey,
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
            JoinLane::BoolPlanar {
                wall,
                window,
                partner_key,
            } => bool_planar_chord_spec(
                body,
                band,
                face,
                wall_key,
                &wall,
                window,
                partner_key,
                u1,
                u2,
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
    let Some(WallSection { wall: cyl_s, case }) =
        wall_section(body, band, ctx.origin, ctx.normal, face, u1)?
    else {
        return Err(SplitJoinError::SectionInvariant {
            face,
            what: "a face read planar by its key reads curved by its section",
        });
    };
    // The chart frame: (center, axis, radius, seam u_ref) — azimuth
    // about the axis, the shape the S9 tail below meters. A sphere's
    // is about its polar axis (M5 S13). A cone's radius varies along
    // the slant, so its lever is the section's own reach, taken once
    // the conic is known (below).
    let (o_c, a_c, r_c, u_ref_c) = match cyl_s {
        geom::Surface::Cylinder {
            origin,
            axis,
            radius,
            u_ref,
        } => (origin, axis, Some(radius), u_ref),
        geom::Surface::Sphere {
            center,
            radius,
            axis,
            u_ref,
        } => (center, axis, Some(radius), u_ref),
        geom::Surface::Cone {
            apex, axis, u_ref, ..
        } => (apex, axis, None, u_ref),
        _ => {
            return Err(SplitJoinError::SectionInvariant {
                face,
                what: "section chord requested on a face kind the gate refuses",
            });
        }
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
    // ---- The arc side (fn docs) ----
    //
    // By azimuth-window containment where the section's azimuth is
    // monotone: the divided face's own window, from the run this chord
    // co-bounds it with. A run with no charted edge leaves the face
    // without a window: refused typed, never guessed.
    let (carrier, t_start, t_end) = if conic.azimuth_monotone {
        let Some(window) = run_azimuth_window(body, &cyl_s, face, run.halves(), band)? else {
            return Err(SplitJoinError::SectionArcWindow {
                face,
                case: ArcWindowCase::NoChartedRun,
                band,
            });
        };
        let (radius, nappe) = match r_c {
            Some(r) => (r, T::one()),
            None => cone_chart_lever(face, band, o_c, a_c, &conic, p1)?,
        };
        let chart = ChartFrame {
            origin: o_c,
            axis: a_c,
            radius,
            u_ref: u_ref_c,
            nappe,
        };
        select_arc(face, band, &chart, &conic, window, p1, p2)?
    } else {
        // The run-side rule reads the run the chord CLOSES; a face
        // window co-bounds nothing, so it hands the rule no run and the
        // rule refuses `NoCertifiedRun`.
        let co_bounded = match run {
            ChordRun::CoBounded(halves) => halves,
            ChordRun::FaceWindow(_) => &[],
        };
        select_arc_by_run_side(body, band, face, &conic, co_bounded, p1, p2)?
    };
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
/// divided face IS the germ plane, the section conic comes from the C5
/// table against the partner wall (by value, from the other operand),
/// and the arc side is selected by containment in the WALL FACE's
/// azimuth window — the S9 statement asked of the mate's chart. Both
/// operands' chords of one polygon side therefore select the same
/// geometric arc, which is what keeps the zip's seams
/// antiparallel-congruent. The arc selection itself is [`select_arc`],
/// the one body this lane SHARES with [`chord_spec`]'s S9 block — same
/// margins, same predicate names, same refusal cases because it is the
/// same code; what differs is only that the window arrives from the
/// mate's face instead of being derived here.
#[allow(clippy::too_many_arguments)]
fn bool_planar_chord_spec<T: Decide>(
    body: &mut Body<T>,
    band: Band,
    face: FaceKey,
    plane_key: SurfaceKey,
    wall: &geom::Surface<T>,
    window: (T, T),
    partner_key: &mut Option<SurfaceKey>,
    u1: VertexKey,
    u2: VertexKey,
) -> Result<Option<EdgeCurveSpec<T>>, SplitJoinError> {
    // The wall's chart frame — cylinder (PR 9, untouched) or sphere
    // (M5 S13: center, polar axis, radius, seam u_ref).
    let (o_c, a_c, r_c, u_ref_c) = match *wall {
        geom::Surface::Cylinder {
            origin,
            axis,
            radius,
            u_ref,
        } => (origin, axis, radius, u_ref),
        geom::Surface::Sphere {
            center,
            radius,
            axis,
            u_ref,
        } => (center, axis, radius, u_ref),
        _ => {
            return Err(SplitJoinError::SectionInvariant {
                face,
                what: "boolean planar-side germ partner is neither a cylinder nor a sphere \
                       (arm not wired)",
            });
        }
    };
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
    // ---- The arc side against the SUPPLIED (mate-face) window: the
    // shared rule, called with a window this lane did not derive. A
    // section with no monotone azimuth has no window to be handed. ----
    if !conic.azimuth_monotone {
        return Err(SplitJoinError::SectionNotPolar { face, band });
    }
    let chart = ChartFrame {
        origin: o_c,
        axis: a_c,
        radius: r_c,
        u_ref: u_ref_c,
        nappe: T::one(),
    };
    let (carrier, t_start, t_end) = select_arc(face, band, &chart, &conic, window, p1, p2)?;
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
/// **The closed-form lane only, and it says so with `None`.** The join
/// lane reads a chart image's azimuth through its closed form; a
/// fitted (rung-3) image has none, so it gets no answer rather than a
/// sentinel. A sentinel would be actively wrong here: the caller hulls
/// with `(a.min(lo), b.max(hi))`, which ABSORBS an inverted range
/// silently instead of propagating it, so "the empty range makes the
/// window refuse" would have been false. `None` propagates through the
/// caller as a typed `SectionInvariant` refusal.
///
/// The arm is unreachable today — `chart_pcurve` refuses a `Nurbs`
/// carrier before a fitted image can reach this function — and it is
/// written anyway because the cyl×sphere join window (banked past M6,
/// M6-PLAN: "chase the lift") is exactly what would make it live.
fn chart_azimuth_range<T: Real>(p: &Pcurve<T>, t0: T, t1: T) -> Option<(T, T)> {
    p.closed_form_span_box(t0, t1).map(|b| (b.u_min, b.u_max))
}

/// The divided face's **azimuth window** on its own chart: the hull of
/// the boundary run's exact chart-azimuth extents, unwrapped
/// branch-continuously along the run — PR 6's per-face window
/// derivation ([`crate::pcurves`]), transplanted to selection time.
/// `None` when the run carries no charted edge.
///
/// **The chart images here are consumed UNCERTIFIED.** This is a
/// selection-time read of [`geom_brep::chart_pcurve`]'s closed form, not
/// a minted cache: no residual, envelope, winding or trim check runs on
/// it. The run's edges are certified against their own surfaces, so a
/// chart image that does not represent one is corrupt input.
///
/// **Nothing downstream re-asks which arc this window selects.** The
/// complement arc lies on the same wall and the same plane, so the `mef`
/// gate certifies it; the mint pass hulls its window out of the face's
/// own images, which then contain it; and no tier-3 check compares a
/// trim against the face it bounds. A wrong window ships a wrong body,
/// so the window is right by construction or it refuses:
///
/// - The branch of each run edge is pinned exactly as the PR 6 loop
///   walk pins it — the whole number of periods that lands this edge's
///   entry azimuth on the previous edge's exit — which is exact at
///   every junction that is a chart point.
/// - A cone APEX is not one. A cone face whose boundary visits its apex
///   takes the run's images from the face's apex-closed lift
///   ([`cone_apex_closure`]), which never pins across the apex and
///   supplies the jump there from the closed boundary; a face no single
///   lift describes refuses [`ArcWindowCase::ApexUnlifted`], and so does
///   the walk itself if it is ever asked to pin across an apex.
///
/// Null scaffolding halves are zero-length coincident copies — they
/// carry no azimuth extent and no branch information, and are stepped
/// over without breaking the chain.
fn run_azimuth_window<T: Decide>(
    body: &Body<T>,
    surface: &geom::Surface<T>,
    face: FaceKey,
    halves: &[HalfEdgeKey],
    band: Band,
) -> Result<Option<(T, T)>, SplitJoinError> {
    if let geom::Surface::Cone { .. } = surface {
        match cone_apex_closure(body, surface, face, band)? {
            ApexClosure::Clear => {}
            ApexClosure::Closed { images, .. } => {
                let run: Vec<AzimuthImage<T>> = images
                    .into_iter()
                    .filter(|image| halves.contains(&image.he))
                    .collect();
                return Ok(azimuth_hull(&run));
            }
            ApexClosure::Open => {
                return Err(SplitJoinError::SectionArcWindow {
                    face,
                    case: ArcWindowCase::ApexUnlifted,
                    band,
                });
            }
        }
    }
    Ok(azimuth_hull(&run_azimuth_images(
        body, surface, face, halves, band,
    )?))
}

/// One boundary half-edge's chart azimuth image, on the branch the
/// run walk pinned: the azimuth where the walk ENTERS it and where it
/// EXITS it (the half-edge's own start and end), and the hull
/// [`run_azimuth_window`] folds.
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

/// The run walk behind [`run_azimuth_window`] and
/// [`face_azimuth_images`]: each charted half-edge's image, its branch
/// pinned to the previous edge's exit.
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
                        return Err(SplitJoinError::SectionArcWindow {
                            face,
                            case: ArcWindowCase::ApexUnlifted,
                            band,
                        });
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
                what: "a run edge's chart image is FITTED — this window rule reads \
                       a closed-form azimuth, and the fitted-chord join lane is not \
                       written",
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

/// What a curved chord's arc side is read from ([`chord_spec`]).
#[derive(Clone, Copy)]
enum ChordRun<'a> {
    /// The run the chord co-bounds the divided face with (a same-loop
    /// join): both arc rules read it.
    CoBounded(&'a [HalfEdgeKey]),
    /// A cross-loop join co-bounds no run, so it hands the divided
    /// face's outer cycle ([`cross_loop_window_cycle`]): a window for the
    /// containment rule only. The run-side rule reads the run a chord
    /// closes, and gets none.
    FaceWindow(&'a [HalfEdgeKey]),
}

impl<'a> ChordRun<'a> {
    /// The halves, for the containment rule's window.
    fn halves(self) -> &'a [HalfEdgeKey] {
        match self {
            Self::CoBounded(h) | Self::FaceWindow(h) => h,
        }
    }
}

/// The cycle a **cross-loop** chord reads its azimuth window from:
/// `face`'s outer cycle.
///
/// A cross-loop join co-bounds no run — `mekr` divides nothing — so the
/// chord's arc is selected by containment in the divided FACE's window,
/// the statement [`bool_planar_chord_spec`] asks of the mate's face. A
/// ring's own cycle is no window: a pierce ring carries only null
/// scaffolding (no window at all) or the section edges already joined
/// into it — on a wall, a ruling and its mate, one azimuth wide. So a
/// face whose outer loop is not a cycle hands the window reader NO run,
/// and a curved face refuses there typed
/// ([`ArcWindowCase::NoChartedRun`]); a plane reads no window.
fn cross_loop_window_cycle<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
) -> Result<Vec<HalfEdgeKey>, SplitJoinError> {
    Ok(outer_cycle(body, face)?.unwrap_or_default())
}

/// `∫ v du` of a harmonic chart image from `t0` to `t1`, and an upper
/// bound on `∫ |dv|` there; `None` for an image that is not harmonic.
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
    let v_int = |t: T| p0.y * t + pa.y * t.sin() - pb.y * t.cos() + pl.y * t.powi(2) * half;
    Ok(Some((pl.x * (v_int(t1) - v_int(t0)), v_var)))
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
/// cylinder wall: a sphere island is valid input whose chart winding is
/// not yet written (no reachable pose built one when the wall reading
/// was), so it stops here typed rather than run unexercised arithmetic.
fn no_wall_chart(face: FaceKey) -> SplitJoinError {
    SplitJoinError::SectionInvariant {
        face,
        what: "a ring-lane island is wound, and a ring re-homed, on a cylinder wall's chart; \
               the sphere and the other curved kinds have no such reading yet",
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
///   form, or a ruling (`n·â = 0`, no `du`). A straight chart
///   segment would differ by the lens between the sinusoid and its
///   chord. Measured over forty tilted thin bars through a pipe (480
///   islands, both tilts, both sides of the axis, down to 1 mm thick)
///   that lens enlarged `|A|` by up to 4.8× and never reversed its
///   sign, and no row distinguishes the two closures; but nothing here
///   bounds the lens by the island, so the exact form stays.
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
    } = wall_chart(&surface).ok_or_else(|| no_wall_chart(face))?;
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
    let mut area = T::zero();
    let mut length = T::zero();
    for (i, image) in images.iter().enumerate() {
        let harmonic = match image.harmonic.as_ref() {
            Some(p) => chart_v_du(face, p, image.t, radius, band)?,
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
            area = area - (image.v.1 + next.v.0) * half * du;
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
            let k = n.dot(origin - centre) / n_a;
            let lever = radius / n_a;
            let g = |u: T| k * u - lever * (n_u * u.sin() - n_v * u.cos());
            let du = to.0 - from.0;
            area = area - (g(to.0) - g(from.0));
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

/// The halves of the loop cycle strictly between `from` (exclusive)
/// and `to` (exclusive), walking `next`.
fn run_between<T: Decide>(
    body: &Body<T>,
    from: HalfEdgeKey,
    to: HalfEdgeKey,
) -> Result<Vec<HalfEdgeKey>, SplitJoinError> {
    let cycle = body.loop_cycle(from).ok_or_else(|| corrupt_he(from))?;
    let mut out = Vec::new();
    for he in cycle.into_iter().skip(1) {
        if he == to {
            return Ok(out);
        }
        out.push(he);
    }
    Err(corrupt_he(to))
}

impl ChordJoiner {
    /// `join` (module docs): connect the old loose end `h1` and the
    /// new half `h2` with up to two chord edges; the minted chord
    /// edges come back (the boolean joining records their germ — M3
    /// PR 5).
    pub(crate) fn join<T: Decide + crate::props::AtRestPolicy>(
        &mut self,
        body: &mut Body<T>,
        h1: HalfEdgeKey,
        h2: HalfEdgeKey,
        mut lane: JoinLane<'_, T>,
        segment: SegmentEdge,
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
        // The RUN the section chords co-bound (real halves between h1
        // and h2 in next order) — the divided face's other boundary,
        // and so the source of its azimuth window (see `chord_spec`).
        let run_halves: Vec<HalfEdgeKey> = if l1 == l2 {
            run_between(body, h1, h2)?
        } else {
            Vec::new()
        };
        // Adjacency of the two null halves on the prev side of h1
        // (h2 → between → h1) — consulted by both chord guards' sample
        // routing below.
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
                // In BOTH configurations the first chord co-bounds
                // the run [h1 .. h2] (in the prev-adjacent belly mint
                // the mef run walks the long way to the between edge,
                // which is exactly cycle[h1..h2]) — one window.
                let (u1, u2) = (start_of(body, h1)?, start_of(body, outside)?);
                let spec = match along_edge_spec(body, &lane, segment, oldf, u1, u2)? {
                    Some(spec) => Some(spec),
                    None => chord_spec(
                        body,
                        self.band,
                        lane.reborrow(),
                        oldf,
                        ChordRun::CoBounded(&run_halves),
                        u1,
                        u2,
                    )?,
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
            let (target, ring) = if l2 == outer {
                (next(body, h2)?, h1)
            } else {
                (h1, next(body, h2)?)
            };
            let site = MekrSite::Cycles { target, ring };
            let face_cycle = cross_loop_window_cycle(body, oldf)?;
            let (u1, u2) = (start_of(body, target)?, start_of(body, ring)?);
            let spec = match along_edge_spec(body, &lane, segment, oldf, u1, u2)? {
                Some(spec) => Some(spec),
                None => chord_spec(
                    body,
                    self.band,
                    lane.reborrow(),
                    oldf,
                    ChordRun::FaceWindow(&face_cycle),
                    u1,
                    u2,
                )?,
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
            // The second chord co-bounds [h2, between, h1] in either
            // adjacent configuration (its mef run walks h2 → between →
            // h1), so the between edge is its run there; otherwise it
            // spans the same interval as the first chord (the two null
            // edges are zero-length, so it is that chord reversed) and
            // takes the same run.
            let (run2, co_bounded): (Vec<HalfEdgeKey>, bool) = if adjacent2 {
                (vec![next(body, h1)?], true)
            } else if prev_adjacent {
                (vec![prev(body, h1)?], true)
            } else if l1 == l2 {
                (run_halves.clone(), true)
            } else {
                (cross_loop_window_cycle(body, owner)?, false)
            };
            let (u1, u2) = (start_of(body, h2)?, start_of(body, next(body, h1)?)?);
            let run2 = if co_bounded {
                ChordRun::CoBounded(&run2)
            } else {
                ChordRun::FaceWindow(&run2)
            };
            let spec = match along_edge_spec(body, &lane, segment, owner, u1, u2)? {
                Some(spec) => Some(spec),
                None => chord_spec(body, self.band, lane, owner, run2, u1, u2)?,
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
    /// on the run's own edge carriers ([`point_in_carrier_loop`]),
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
/// Its one consumer is [`point_in_carrier_loop`], which reads the
/// normal only to recover the loop's PLANE and the in-plane side axis
/// `n̂ × d` of each ray; only the straight edges' crossing rows read
/// that axis, and their verdict is exactly invariant under `n̂ ↦ −n̂`.
/// **That derivation lives at
/// [`point_in_loop`](crate::splitting::containment::point_in_loop)**,
/// under the function whose property it is rather than under the
/// five-line producer that relies on it; the consequence here is that
/// ring re-homing cannot move a ring on the sense bit, and
/// `tests/review_m3_pr3_pil.rs` pins it for the straight rows.
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
        match point_in_carrier_loop(body, run, normal, p, band)? {
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
    } = wall_chart(surface).ok_or_else(|| no_wall_chart(newf))?;
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

    // -----------------------------------------------------------------
    // DIRECT rows for the arc-side selector. Since M5 S9 the selector is
    // azimuth-window CONTAINMENT (`split_arc_window`): definite ccw /
    // definite cw / in-band boundary, plus the three named
    // `ArcWindowCase` refusals — driven straight through `chord_spec` on
    // a hand-built cylinder-face body whose RUN is a rim arc (or a pair
    // of chained rim arcs) placed to put the window where each row wants
    // it. The pre-S9 rows drove the same three verdicts from a single
    // azimuth SAMPLE; that premise is the repaired defect (#144).
    // -----------------------------------------------------------------

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

    /// `chord_spec` on the fixture with a run of rim arcs.
    fn spec_with(runs: &[(f64, f64)]) -> Result<Option<EdgeCurveSpec<f64>>, SplitJoinError> {
        let band = Band::new(1e-9, 1e-8).unwrap();
        let (mut body, face, u1, u2, mut ctx) = cyl_fixture();
        let run: Vec<_> = runs
            .iter()
            .map(|&(a, b)| rim_run(&mut body, a, b))
            .collect();
        chord_spec(
            &mut body,
            band,
            JoinLane::Split(&mut ctx),
            face,
            ChordRun::CoBounded(&run),
            u1,
            u2,
        )
    }

    #[test]
    fn arc_side_definite_ccw_and_cw() {
        // The chord's endpoints sit at chart azimuths 0 and π/2 (the
        // fixture's θ = 0 and θ = π/2 ellipse points), so g = π/2.
        //
        // ccw: a window [−0.2, π/2 + 0.2] strictly contains [0, π/2] and
        // cannot contain the complement (it is under a period wide).
        let spec = spec_with(&[(-0.2, core::f64::consts::FRAC_PI_2 + 0.2)])
            .unwrap()
            .expect("cylinder face mints a conic chord");
        let geom::Curve3::Ellipse { axis, .. } = spec.carrier else {
            panic!("tilted section is an ellipse");
        };
        // ccw keeps the classification frame (axis ≈ the plane normal)
        // and spans θ: 0 → π/2.
        assert!(axis.dot(Vec3::new(0.5f64.sin(), 0.0, 0.5f64.cos())) > 0.9);
        assert!(spec.param_start.abs() < 1e-12);
        assert!((spec.param_end - core::f64::consts::FRAC_PI_2).abs() < 1e-12);

        // cw: the window is the OTHER way round the chart —
        // [π/2 − 0.2, τ + 0.2] contains the long arc, so the carrier's
        // frame flips to keep it running forward u1 → u2.
        let spec = spec_with(&[(
            core::f64::consts::FRAC_PI_2 - 0.2,
            core::f64::consts::TAU + 0.2,
        )])
        .unwrap()
        .expect("cylinder face mints a conic chord");
        let geom::Curve3::Ellipse { axis, .. } = spec.carrier else {
            panic!("tilted section is an ellipse");
        };
        assert!(
            axis.dot(Vec3::new(0.5f64.sin(), 0.0, 0.5f64.cos())) < -0.9,
            "the cw arc takes the flipped frame"
        );
        assert!(
            (spec.param_end - spec.param_start - 1.5 * core::f64::consts::PI).abs() < 1e-12,
            "the cw arc spans the complement"
        );
    }

    #[test]
    fn arc_side_in_band_escalates() {
        // A window whose upper boundary sits 5e-9 rad past the chord's
        // end azimuth: the containment margin (metered at the unit chart
        // radius) lands in the band — F6, typed, predicate named.
        let err = spec_with(&[(-0.2, core::f64::consts::FRAC_PI_2 + 5e-9)]).unwrap_err();
        let SplitJoinError::Escalated { diag, .. } = err else {
            panic!("expected the arc-side escalation, got {err:?}");
        };
        assert_eq!(diag.predicate, Some("split_arc_window"));
    }

    #[test]
    fn arc_side_refusal_arms_are_typed() {
        // No charted edge in the joined run: the divided face has no
        // azimuth window at all.
        let err = spec_with(&[]).unwrap_err();
        assert!(
            matches!(
                err,
                SplitJoinError::SectionArcWindow {
                    case: ArcWindowCase::NoChartedRun,
                    ..
                }
            ),
            "{err:?}"
        );
        // A window narrower than either candidate: NEITHER is contained
        // — a degenerate window, refused, never guessed.
        let err = spec_with(&[(-0.2, 0.2)]).unwrap_err();
        assert!(
            matches!(
                err,
                SplitJoinError::SectionArcWindow {
                    case: ArcWindowCase::NeitherContained,
                    ..
                }
            ),
            "{err:?}"
        );
        // A face that wraps the chart: two chained rim arcs whose hull
        // spans τ + 0.2 (from −3π/2 − 0.1 to π/2 + 0.1) contain BOTH
        // candidates — containment does not name an arc, so it refuses
        // rather than falling back to a convention.
        let pi = core::f64::consts::PI;
        let err = spec_with(&[(-1.5 * pi - 0.1, -pi), (-pi, pi / 2.0 + 0.1)]).unwrap_err();
        assert!(
            matches!(
                err,
                SplitJoinError::SectionArcWindow {
                    case: ArcWindowCase::BothContained,
                    ..
                }
            ),
            "{err:?}"
        );
        // The plane×plane lane divides only planar faces, so a conic
        // chord asked of it on a wall is an invariant violation, typed.
        let band = Band::new(1e-9, 1e-8).unwrap();
        let (mut body, face, u1, u2, _) = cyl_fixture();
        let run = vec![rim_run(&mut body, -0.2, core::f64::consts::FRAC_PI_2 + 0.2)];
        let lane = JoinLane::Planar;
        let err = chord_spec(
            &mut body,
            band,
            lane,
            face,
            ChordRun::CoBounded(&run),
            u1,
            u2,
        )
        .unwrap_err();
        assert!(
            matches!(err, SplitJoinError::SectionInvariant { .. }),
            "{err:?}"
        );
    }

    /// The S9 adversarial review's disagreement probe, **adopted as a
    /// committed row**: the one direction in which the old sample rule
    /// and the new window rule disagree outside the belly class, and the
    /// proof that the disagreement is refusal-vs-guess.
    ///
    /// The run is chained arcs whose FIRST edge's midpoint azimuth
    /// (0.25) lies inside the chord's interval [0, π/2] — so the OLD
    /// rule's premise holds and it would have selected ccw — while the
    /// run's hull [0.1, 5.0] contains NEITHER candidate (the chord's
    /// start, azimuth 0, sits outside the window: this run does not
    /// actually end where the chord starts). The window rule refuses
    /// `NeitherContained` rather than selecting anything. Every
    /// constructible disagreement has this shape: the sample rule
    /// answers from one point and the window rule declines from the
    /// face, so a disagreement costs a refusal, never wrong geometry.
    #[test]
    fn s9_review_probe_old_premise_holds_new_refuses() {
        let err = spec_with(&[(0.1, 0.4), (0.4, 5.0)]).unwrap_err();
        assert!(
            matches!(
                err,
                SplitJoinError::SectionArcWindow {
                    case: ArcWindowCase::NeitherContained,
                    ..
                }
            ),
            "{err:?}"
        );
    }

    /// The two-tolerance pair of the containment rule (S6, D4 ¶1
    /// addendum): a window of exactly one period is a DEFINITE
    /// `BothContained` refusal, and the same window 5e-9 rad narrower
    /// is the in-band `Escalated` — one user situation, so both
    /// messages name `split_arc_window`, quote the same two tolerances,
    /// and carry the shared recourse carrier exactly once.
    #[test]
    fn arc_window_two_tolerance_pair_shares_the_carrier() {
        let pi = core::f64::consts::PI;
        // width = τ exactly (to rounding): definite.
        let definite = spec_with(&[(-1.5 * pi, -pi), (-pi, pi / 2.0)]).unwrap_err();
        assert!(
            matches!(
                definite,
                SplitJoinError::SectionArcWindow {
                    case: ArcWindowCase::BothContained,
                    ..
                }
            ),
            "{definite:?}"
        );
        // width = τ − 5e-9: inside the band.
        let escalated = spec_with(&[(-1.5 * pi, -pi), (-pi, pi / 2.0 - 5e-9)]).unwrap_err();
        let SplitJoinError::Escalated { diag, .. } = &escalated else {
            panic!("expected the in-band neighbour, got {escalated:?}");
        };
        assert_eq!(diag.predicate, Some("split_arc_window"));
        // The sentence says what was too close to call, in words.
        assert!(
            escalated
                .to_string()
                .contains("where a section runs across a face is too close to call"),
            "{escalated}"
        );

        for msg in [definite.to_string(), escalated.to_string()] {
            assert_eq!(msg.matches(JOIN_RECOURSE).count(), 1, "{msg}");
            assert!(!msg.contains("declare"), "{msg}");
            assert!(msg.contains("1e-9") && msg.contains("1e-8"), "{msg}");
        }
        // The sub-case that classified nothing must NOT carry the
        // recourse — there is no ill-conditioned margin behind it.
        let no_run = spec_with(&[]).unwrap_err().to_string();
        assert_eq!(no_run.matches(JOIN_RECOURSE).count(), 0, "{no_run}");
    }

    /// Seam-placement independence, at the unit: the whole construction
    /// is rotated about the cylinder axis — chart `u_ref` included, so
    /// the chart seam moves with it — and the selected arc's parameter
    /// span is bit-identical. Every quantity the rule compares is a
    /// difference of azimuths, so a moved seam cancels.
    #[test]
    fn window_rule_is_seam_placement_independent() {
        let base = spec_with(&[(-0.2, core::f64::consts::FRAC_PI_2 + 0.2)])
            .unwrap()
            .expect("cylinder face mints a conic chord");
        // The same rule with the run's window shifted by a whole period:
        // the chart branch is different, the containment verdict is not.
        let tau = core::f64::consts::TAU;
        let shifted = spec_with(&[(-0.2 - tau, core::f64::consts::FRAC_PI_2 + 0.2 - tau)])
            .unwrap()
            .expect("cylinder face mints a conic chord");
        assert_eq!(base.param_start, shifted.param_start);
        assert_eq!(base.param_end, shifted.param_end);
    }

    /// S6 (two-tolerance, D4 ¶1 addendum): the split-join pair —
    /// exactly-zero section area (`DegenerateSection`) and in-band
    /// (`Escalated`) — is one user situation; both arms carry the
    /// join's one recourse ([`JOIN_RECOURSE`]), which offers no
    /// declaration because the join takes none.
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

    /// **The run-side rule reads a run end only where the divided face's
    /// corner opens at most a half-turn.** Walking a boundary with the
    /// face on the left under `+z`: a left turn is convex, straight on
    /// is smooth, a right turn is reflex, and a reversal is a cusp — the
    /// last two refused, since there the run's left side does not put
    /// an arc inside the face.
    #[test]
    fn a_run_end_reads_only_at_a_smooth_or_convex_corner() {
        let band = Band::new(1e-9, 1e-8).unwrap();
        let z = Vec3::new(0.0, 0.0, 1.0);
        let x = Vec3::new(1.0, 0.0, 0.0);
        let y = Vec3::new(0.0, 1.0, 0.0);
        let opens = |arrive, depart| run_corner_opens(z, arrive, depart, 1.0, band).unwrap();
        assert!(opens(x, y), "a left turn is a convex corner");
        assert!(opens(x, x), "straight on is a smooth boundary point");
        assert!(!opens(x, -y), "a right turn is a reflex corner");
        assert!(!opens(x, -x), "a reversal is a cusp");
    }

    /// **A run end at a reflex corner refuses by name, at the rule's own
    /// call.** The top face of a chevron prism, `(0,0) → (2,1) → (0,2) →
    /// (0.5,1)`, ccw seen from `+z`, has one reflex corner, at
    /// `(0.5, 1)`. The run is the edge arriving there from the convex
    /// corner `(0, 2)`, and the chord is a circle arc between the run's
    /// two ends, in the face's plane. The start end reads its side at a
    /// convex corner. The end end's corner turns right, so its reading
    /// does not count, and the rule refuses `ReflexRunEnd` rather than
    /// letting that end vote (or disagree).
    #[test]
    fn a_reflex_run_end_refuses_at_the_rule() {
        let tol = Tol::witness();
        let band = Band::new(1e-9, 1e-8).unwrap();
        let profile = [(0.0, 0.0), (2.0, 1.0), (0.0, 2.0), (0.5, 1.0)];
        let prism = crate::test_support_fixtures::prism_z::<f64>(&profile, 0.0, 1.0, tol);
        let body = &prism.body;
        let (c, d) = (Point3::new(0.0, 2.0, 1.0), Point3::new(0.5, 1.0, 1.0));
        let at = |he: HalfEdgeKey| {
            let v = body.get_half_edge(he).unwrap().start;
            *body.get_point(body.get_vertex(v).unwrap().point).unwrap()
        };
        let near = |p: Point3<f64>, q: Point3<f64>| (p - q).norm() < 1e-12;
        // The face whose outward normal is `+z`, and its half-edge
        // running from `c` into the reflex corner `d`.
        let (face, run) = body
            .faces()
            .filter_map(|(k, f)| {
                let crate::LoopBoundary::Cycle { first } = body.get_loop(f.outer)?.boundary else {
                    return None;
                };
                let he = body.loop_cycle(first)?.into_iter().find(|&he| {
                    let next = body.get_half_edge(he).unwrap().next;
                    near(at(he), c) && near(at(next), d)
                })?;
                Some((k, he))
            })
            .find(|&(k, _)| {
                face_normal::face_outward_normal_at(body, k, c, band)
                    .ok()
                    .flatten()
                    .is_some_and(|n| n.vec().z > 0.5)
            })
            .expect("the chevron's top face carries the run c → d");
        // A circle through the run's two ends, in the face's plane.
        let normal = Vec3::new(0.0, 0.0, 1.0);
        let mid = Point3::new(0.25, 1.5, 1.0);
        let off = Vec3::new(1.0, 0.5, 0.0) / Vec3::new(1.0, 0.5, 0.0).norm();
        let center = mid + off * 2.0;
        let radius = (c - center).norm();
        let major = (c - center) / radius;
        let conic = SectionConic {
            center,
            normal,
            major,
            sa: radius,
            sb: radius,
            carrier: geom::Curve3::Circle {
                center,
                axis: normal,
                radius,
                u_ref: major,
            },
            azimuth_monotone: false,
        };
        let got = select_arc_by_run_side(body, band, face, &conic, &[run], c, d);
        assert!(
            matches!(
                got,
                Err(SplitJoinError::SectionArcSide {
                    case: ArcSideCase::ReflexRunEnd,
                    ..
                })
            ),
            "{got:?}"
        );
    }

    /// **The anti-re-fork row for the arc-side rule.** Each of the
    /// rungs the two chord lanes share is decided in exactly ONE
    /// place in this crate — counted, not merely located, because the
    /// duplication this row exists against was INSIDE one file: for
    /// most of this module's life `chord_spec` and
    /// `bool_planar_chord_spec` sat 500 lines apart carrying
    /// line-identical copies of the S9 block, with a doc comment at the
    /// copy site declaring them the same ("same margins, same predicate
    /// names, same refusal cases"). A cross-file guard would have been
    /// green throughout.
    ///
    /// **What it cannot match** — three shapes:
    ///
    /// 1. **A second copy under a FRESH predicate name.** It surfaces
    ///    as new rows in `docs/K-REPORT.md`'s census, which is the
    ///    mechanism that already exists for that.
    /// 2. **A copy that re-derives the verdict from `select_arc`'s
    ///    RESULT** — recomputing which arc was chosen from the returned
    ///    carrier's axis, say. The rungs still fire once each, the K
    ///    stream is unchanged, and no string search can see it.
    /// 3. **A copy in another crate.** The count is scoped to
    ///    `topo/src`; `crate::validate::decide` is `pub(crate)`, so a
    ///    foreign crate would have to call `geom_core`'s directly.
    #[test]
    fn the_arc_side_rungs_are_decided_in_one_place() {
        // Assembled rather than spelled, so this file is subject to
        // the count like any other — writing the three names out here
        // would make the guard its own second site.
        let rungs = [
            "arc_window",
            "arc_chart_orientation",
            "sphere_section_polar",
            "arc_run_end",
            "arc_run_side",
            "arc_run_along",
            "arc_run_on_section_plane",
            "arc_run_on_section_conic",
            "arc_run_corner",
            "arc_run_cusp",
        ]
        .map(|rung| format!("\"split_{rung}\""));
        let home = crate::source_walk::src_root().join("chord_join.rs");
        let files = crate::source_walk::crate_sources();
        assert!(files.contains(&home), "the walk did not find chord_join.rs");
        for rung in &rungs {
            let mut sites = 0;
            for path in &files {
                let text = std::fs::read_to_string(path).expect("a readable source file");
                // DECIDE sites, counted on a whitespace-stripped copy so
                // a call broken across lines counts the same as an
                // inline one. Test rows asserting the name (they read
                // `predicate`, they do not decide) are not sites.
                let stripped: String = text.chars().filter(|c| !c.is_whitespace()).collect();
                sites += stripped.matches(&format!("decide({rung}")).count();
                let here = text.matches(rung.as_str()).count();
                assert!(
                    path == &home || here == 0,
                    "{} names the arc-side rung {rung} — the rule has been re-forked \
                     out of chord_join.rs, which must hold the only one. Call \
                     `select_arc` / `section_case` instead.",
                    path.display()
                );
            }
            assert_eq!(
                sites, 1,
                "{rung} is spelled at {sites} site(s); the arc-side rule is supposed \
                 to be written once — a second site is the S9 block copied again"
            );
        }
    }

    // -----------------------------------------------------------------
    // Straddle rows at the interval scalar (issue 1191). Nothing in the
    // shipped suites drives these reductions into a straddle, so the
    // rows below do it directly, on the site's own numbers.
    // -----------------------------------------------------------------

    /// **The chord start ON the window edge, at `Interval`.**
    ///
    /// The generic configuration — the chord's start IS one of the run's
    /// own ends, so its azimuth sits exactly on `w_min` — driven with an
    /// enclosure that genuinely straddles that edge rather than a
    /// degenerate box that would not exercise the reduction at all. The
    /// arc is still selected: the window-relative coordinate comes back
    /// at the width of its input.
    ///
    /// The row also computes the reduction the naive way — anchored at
    /// the window's EDGE instead of its centre — and pins that it is
    /// period-wide on the same input. That comparison is the claim: the
    /// site is correct because of where its window's jump is, and the
    /// row would still pass if it were merely lucky, so the alternative
    /// is measured beside it rather than described.
    ///
    /// **Consults no tolerance.** The widths are widths; the band below
    /// is the ordinary one the site's own margins need in order to run
    /// at all, and no assertion here reads it.
    #[test]
    fn the_window_relative_start_keeps_its_width_when_the_start_straddles_the_edge() {
        use geom_core::{Bounds, Interval, Real};

        let iv = |lo: f64, hi: f64| Interval::from_bounds(lo, hi);
        let ex = Interval::from_f64;
        let band = Band::new(1e-9, 1e-8).unwrap();

        // A hairline box about azimuth 0 — the shape an enclosure of a
        // run end takes once its coordinates have been rounded apart.
        let w = 1e-15;
        let p1 = Point3::new(iv(1.0 - w, 1.0), iv(-w, w), ex(0.0));
        let p2 = Point3::new(ex(0.0), ex(1.0), ex(0.0));

        let chart = ChartFrame {
            origin: Point3::new(ex(0.0), ex(0.0), ex(0.0)),
            axis: Vec3::new(ex(0.0), ex(0.0), ex(1.0)),
            radius: ex(1.0),
            u_ref: Vec3::new(ex(1.0), ex(0.0), ex(0.0)),
            nappe: ex(1.0),
        };
        let carrier = geom::Curve3::Circle {
            center: Point3::new(ex(0.0), ex(0.0), ex(0.0)),
            axis: Vec3::new(ex(0.0), ex(0.0), ex(1.0)),
            radius: ex(1.0),
            u_ref: Vec3::new(ex(1.0), ex(0.0), ex(0.0)),
        };
        let conic = SectionConic {
            center: Point3::new(ex(0.0), ex(0.0), ex(0.0)),
            normal: Vec3::new(ex(0.0), ex(0.0), ex(1.0)),
            major: Vec3::new(ex(1.0), ex(0.0), ex(0.0)),
            sa: ex(1.0),
            sb: ex(1.0),
            carrier,
            azimuth_monotone: true,
        };
        // The window runs ccw from the straddled edge to half a period
        // on: the start is on `w_min`, which is the whole point.
        let window = (ex(0.0), ex(core::f64::consts::PI));

        let (_, t0, t1) = select_arc(FaceKey::default(), band, &chart, &conic, window, p1, p2)
            .expect("the arc is selected: the window-relative start is not period-wide");
        for (what, p) in [("t0", t0), ("t1", t1)] {
            let width = p.hi() - p.lo();
            assert!(
                width <= 1e-9,
                "the selected arc's {what} enclosure is {width:e} wide — a period-width \
                 answer, not an input-width one"
            );
        }

        // The measurement the row rests on, taken on the same numbers.
        //
        // DISPOSITION: this half re-derives both anchorings inline, so
        // it pins the two WINDOWS against each other, not the site — a
        // site that stopped calling either one would leave it green.
        // That is deliberate and it is not the site's pin: the
        // `select_arc` call above is, and it reds if the site changes
        // window. What this half adds is the reason the site's choice
        // is the right one, which an assertion on the site's output
        // alone cannot show.
        let tau = Interval::tau();
        let a1 = iv(-w, w);
        let half_w = window.1 * ex(0.5);
        let centred = (a1 - (window.0 + half_w)).reduce_periodic_centred(tau) + half_w;
        let at_edge = (a1 - window.0).reduce_periodic(tau);
        assert!(
            centred.hi() - centred.lo() <= 1e-9,
            "the centred offset widened: [{}, {}]",
            centred.lo(),
            centred.hi()
        );
        assert!(
            at_edge.hi() - at_edge.lo() >= core::f64::consts::TAU,
            "the edge-anchored offset is supposed to be the period-wide one; it gave \
             [{}, {}]",
            at_edge.lo(),
            at_edge.hi()
        );
    }

    /// CERT-4 R2 probe (local-only): the same straddle authored on MY
    /// numbers — the window's straddled edge at azimuth pi/2 rather
    /// than 0, an off-axis chord end, a 2.5-radian window. The unit's
    /// committed row uses the zero azimuth, where several quantities
    /// are exactly representable; this one is not so friendly.
    #[test]
    fn cert4r2_the_window_edge_straddle_off_axis() {
        use geom_core::{Bounds, Interval, Real};

        let iv = |lo: f64, hi: f64| Interval::from_bounds(lo, hi);
        let ex = Interval::from_f64;
        let band = Band::new(1e-9, 1e-8).unwrap();
        let w = 1e-15;
        // p1 straddles azimuth pi/2: x in [-w, w], y hairline about 1.
        let p1 = Point3::new(iv(-w, w), iv(1.0 - w, 1.0), ex(0.0));
        // p2 at azimuth pi/2 + 1.2, nothing exact about it.
        let a2 = core::f64::consts::FRAC_PI_2 + 1.2;
        let p2 = Point3::new(ex(a2.cos()), ex(a2.sin()), ex(0.0));

        let chart = ChartFrame {
            origin: Point3::new(ex(0.0), ex(0.0), ex(0.0)),
            axis: Vec3::new(ex(0.0), ex(0.0), ex(1.0)),
            radius: ex(1.0),
            u_ref: Vec3::new(ex(1.0), ex(0.0), ex(0.0)),
            nappe: ex(1.0),
        };
        let carrier = geom::Curve3::Circle {
            center: Point3::new(ex(0.0), ex(0.0), ex(0.0)),
            axis: Vec3::new(ex(0.0), ex(0.0), ex(1.0)),
            radius: ex(1.0),
            u_ref: Vec3::new(ex(1.0), ex(0.0), ex(0.0)),
        };
        let conic = SectionConic {
            center: Point3::new(ex(0.0), ex(0.0), ex(0.0)),
            normal: Vec3::new(ex(0.0), ex(0.0), ex(1.0)),
            major: Vec3::new(ex(1.0), ex(0.0), ex(0.0)),
            sa: ex(1.0),
            sb: ex(1.0),
            carrier,
            azimuth_monotone: true,
        };
        let window = (
            ex(core::f64::consts::FRAC_PI_2),
            ex(core::f64::consts::FRAC_PI_2 + 2.5),
        );

        let (_, t0, t1) = select_arc(FaceKey::default(), band, &chart, &conic, window, p1, p2)
            .expect("the off-axis window-edge straddle still selects the arc");
        for (what, p) in [("t0", t0), ("t1", t1)] {
            let width = p.hi() - p.lo();
            assert!(
                width <= 1e-9,
                "{what} enclosure {width:e} wide — period-width, not input-width"
            );
        }
    }
}

/// **R1 review probe (CERT-4): the NEW anchoring's own jump.**
///
/// The unit's row above measures the alternative (edge) anchoring on
/// a window of width π and finds it period-wide, which is the right
/// comparison. It does not probe the centred anchoring's OWN jump.
/// The site's argument is that the distance from the reduction's
/// argument to that jump is `(τ − width)/2`, positive only because
/// the `width ≥ τ` arm returned earlier — so the margin VANISHES as
/// the window's width approaches a period. This row drives that
/// limit: a nearly-whole-period window with the chord start near the
/// window's antipode.
///
/// Consults no tolerance: the widths asserted are widths.
#[test]
fn cert4r1_the_centred_anchoring_widens_at_its_own_jump_for_a_near_whole_window() {
    use geom_core::{Bounds, Interval, Real};

    let iv = |lo: f64, hi: f64| Interval::from_bounds(lo, hi);
    let ex = Interval::from_f64;
    let tau = Interval::tau();
    let w = 1e-15;

    // A window just under a whole period, and a start box sitting at
    // its antipode -- i.e. on the centred window's own jump.
    // Written relative to TAU rather than as decimal literals that
    // approximate it: what the row varies is the window's MARGIN to a
    // full period, and the last two are that margin made small.
    let tau_f = core::f64::consts::TAU;
    for width in [core::f64::consts::PI, 6.0, tau_f - 0.0032, tau_f - 1e-13] {
        let w_min = 0.7_f64;
        let half_w = ex(width) * ex(0.5);
        let centre = w_min + width / 2.0;
        let antipode = centre + core::f64::consts::PI;
        let a1 = iv(antipode - w, antipode + w);
        let x1 = (a1 - (ex(w_min) + half_w)).reduce_periodic_centred(tau) + half_w;
        let got = x1.hi() - x1.lo();
        println!(
            "cert4r1 topo: window width {width:e}, margin to jump {:e}, \
                 window-relative start width {got:e}",
            (core::f64::consts::TAU - width) / 2.0
        );
        // MEASURED: period-wide at EVERY width tested, including the
        // unit's own width of pi where the margin to the jump is a
        // comfortable 1.57 rad. The site's argument is sound only for
        // a start INSIDE the window; a start at the window's antipode
        // is on the centred fold's own jump and comes back a period
        // wide. That premise is prose at the site, not a gate.
        assert!(
            got >= core::f64::consts::TAU,
            "expected the antipodal start to sit on the centred fold's jump"
        );
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
