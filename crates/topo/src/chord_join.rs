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
//!   containment ([`crate::splitting::containment`]) +
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
//! visible cost of a shared core with no home of its own (smell scan
//! S5).
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

use geom_brep::{EdgeCurveSpec, EdgeDescriptionSpec, Pcurve, chart_pcurve};
use geom_core::spline::SpanLocate;
use geom_core::{Band, BandError, Decide, Indeterminate, Margin, Point3, Real, Sign, Vec3};
use slotmap::SecondaryMap;

use crate::body::Body;
use crate::entity::{EdgeKey, EntityId, FaceKey, HalfEdgeKey, LoopBoundary, LoopKey, VertexKey};
use crate::euler::{EulerOpError, FaceSurface, MefSite};
use crate::euler_ring::MekrSite;
use crate::geometry::SurfaceKey;
use crate::null::CurveGeom;
use crate::splitting::SplitPlane;
use crate::splitting::containment::{LoopContainment, PointInLoopError, point_in_loop};
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
    /// so the divided face has no azimuth window at all. (A cylinder
    /// face's run always carries one on the shipped lane; this is the
    /// typed door for a corrupt or frontier-carrier run.)
    NoChartedRun,
    /// NEITHER candidate arc lies inside the window — the window is
    /// degenerate relative to the chord (an ill-conditioned operand, or
    /// a run that does not actually co-bound the face with this chord).
    NeitherContained,
    /// BOTH candidates lie inside the window: the window spans at least
    /// one full period, so containment does not distinguish the arcs.
    /// Ambiguous by construction — refused, never broken by convention.
    BothContained,
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
            Self::NoChartedRun => false,
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
    /// A completed section polygon bounds zero area — the zero-area
    /// residue (rule (b) adjudication record): no degenerate body is
    /// ever emitted.
    ///
    /// Per RUN this fires iff the pinched pieces lie on the NEGATIVE
    /// side of the run's plane normal (the below side, where the
    /// ch. 14 insertion mints no vertex copies). Since M3 PR 6a (D7)
    /// the public [`crate::splitting::split`] consumes this refusal as the pinch
    /// trigger and reruns under the mirrored plane — where the
    /// pinched fans are ABOVE runs and mint their copies — so op
    /// success is orientation-independent; the error still surfaces
    /// from [`crate::splitting::split`] when BOTH orientations refuse (a genuine
    /// both-sided zero-area residue) and from the join lane directly
    /// (e.g. [`crate::splitting::plane_section`], which has no sides to swap).
    DegenerateSection {
        /// The completed null face.
        face: FaceKey,
    },
    /// Ring re-homing could not decide (escalation or exhaustion).
    RingHoming(PointInLoopError),
    /// A ring's representative landed ON the divided face's outer
    /// loop — containment is ambiguous (ill-conditioned operand).
    RingHomingAmbiguous {
        /// The undecidable ring.
        ring: LoopKey,
    },
    /// Loose ends survived the sweep — the null-edge set does not
    /// close into section polygons (kernel bug or corrupt reduction).
    UnpairedLooseEnds {
        /// How many halves remained.
        count: usize,
    },
    /// A section loop mixed above copies with below-side vertices —
    /// the joining invariant (heads join heads, tails join tails)
    /// failed (kernel bug, loudly).
    SectionLoopMixed {
        /// The offending null face.
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

impl core::fmt::Display for SplitJoinError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::OrderEscalated { diag } => {
                write!(f, "split join: lexicographic order escalated: {diag}")
            }
            Self::Escalated { face, diag } => {
                write!(f, "split join: escalated at face {face:?}: {diag}")
            }
            Self::DegenerateSection { face } => write!(
                f,
                "split join: section polygon at {face:?} bounds zero area — one-sided \
                 tangency: the degenerate side has no real material (refused, never \
                 emitted); {}",
                geom_core::COINCIDENCE_RECOURSE
            ),
            Self::RingHoming(e) => write!(f, "split join: ring re-homing: {e}"),
            Self::RingHomingAmbiguous { ring } => write!(
                f,
                "split join: ring {ring:?} sits ON the divided face's outer loop — \
                 containment undecidable (ill-conditioned operand)"
            ),
            Self::UnpairedLooseEnds { count } => write!(
                f,
                "split join: {count} loose null-edge halves survived the sweep (kernel bug)"
            ),
            Self::SectionLoopMixed { face } => write!(
                f,
                "split join: null face {face:?} has a side-mixed section loop (kernel bug)"
            ),
            Self::CutInvariant { edge } => write!(
                f,
                "split join: neither face flanking null edge {edge:?} is a sliver (kernel bug)"
            ),
            Self::Corrupt { entity } => {
                write!(f, "split join: traversal failed at {entity} (corrupt body)")
            }
            Self::Band(e) => write!(f, "split join: invalid band: {e}"),
            Self::Euler(e) => write!(f, "split join: euler operation refused: {e}"),
            Self::Section { face, source } => {
                write!(f, "split join: section chord in face {face:?}: {source}")
            }
            Self::SectionArcWindow { face, case, band } => {
                write!(
                    f,
                    "split join: section chord in face {face:?}: arc-side selection \
                     refused — {case}"
                )?;
                if case.is_containment_verdict() {
                    write!(
                        f,
                        "; predicate 'split_arc_window' classified definite against the band \
                         (zero = {:e}, escalate = {:e}) — the same margin inside that band \
                         escalates instead, and it is the same ill-conditioning either way; {}",
                        band.zero(),
                        band.escalate(),
                        geom_core::COINCIDENCE_RECOURSE
                    )?;
                }
                Ok(())
            }
            Self::SectionInvariant { face, what } => {
                write!(
                    f,
                    "split join: curved-section invariant at face {face:?}: {what}"
                )
            }
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
/// sweep's rule). The boolean joining passes `None` — its operands are
/// gated all-planar and take the straight-chord lane bit-identically.
pub(crate) struct SectionCtx<T: Real> {
    /// The split plane.
    pub(crate) plane: SplitPlane<T>,
    /// The minted auxiliary plane surface, once needed.
    pub(crate) plane_key: Option<SurfaceKey>,
}

/// Which curved-section lane a [`ChordJoiner::join`] call runs
/// (M5 PR 9 generalized the M3 `Option<&mut SectionCtx>`):
///
/// - [`JoinLane::Planar`] — the M3 boolean's straight-chord lane,
///   bit-identical (`chord_spec` sees no context and returns `None`
///   for planar faces).
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
    /// Straight chords only (the M3 boolean lane).
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
        }
    }
}

/// The section conic's frame — the datum both chord lanes select an
/// arc of, in the form the arc-side rule reads it.
struct SectionConic<T: Real> {
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
}

/// What the C5 table made of `plane × wall` for a chord that has to
/// ride it.
///
/// `Straight` and `Tangent` are handed BACK rather than decided here:
/// the two chord lanes mean different things by them — the split lane
/// mints a tangent chord along the ruling, the boolean zip refuses a
/// tangent germ pair as a touching frontier — and that difference is
/// the whole of what the two lanes do not share.
enum SectionCase<T: Real> {
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
/// chord lanes** (smell scan S5's residue: this classification was
/// written twice in this file, once per lane, differing only in the
/// wording of its refusals and in what it did with the tangent arm).
///
/// **Pair-general, not plane-first.** The two arms wired today are
/// plane×cylinder and plane×sphere, and either order is accepted: the
/// caller hands over the pair it has, and which member is the plane is
/// this function's question rather than the caller's. A pair with no
/// arm — every curved×curved pair, today — refuses typed here, which
/// is the same discipline the germ-pair frame dispatch keeps
/// (`boolean::join::pair_section_frame`): a missing arm is never a
/// straight chord.
///
/// The sphere lane (M5 S13) classifies through `plane_sphere_section`
/// — an exact Circle, never a fitted chord — and refuses a section
/// tilted against the sphere chart's polar axis, because the
/// azimuth-anchored arc-side rule premises azimuth MONOTONE along the
/// carrier and that holds on a sphere chart only for polar sections.
/// The cylinder lane is PR 5/PR 9's `plane_cylinder_section`.
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
                 plane×cylinder and plane×sphere, and a curved×curved pair has no arc-side \
                 rule to run; refused typed rather than defaulted to a straight chord",
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
        match decide(
            "split_sphere_section_polar",
            Margin::levered(axis.cross(*sph_axis).norm(), *sph_r),
            band,
        )
        .map_err(|diag| SplitJoinError::Escalated { face, diag })?
        {
            Sign::Zero => {}
            Sign::Positive | Sign::Negative => {
                return Err(invariant(
                    "plane×sphere section tilted against the sphere chart's polar \
                     axis — the azimuth-anchored arc-side rule needs a polar \
                     section (the extent-certified re-cut re-charts the operand; a tilted \
                     residual configuration is a typed frontier)",
                ));
            }
        }
        return Ok(SectionCase::Conic(SectionConic {
            center,
            normal: axis,
            major: u_ref,
            sa: radius,
            sb: radius,
            carrier: circle,
        }));
    }
    let sec = geom_brep::plane_cylinder_section(plane_s, wall, extent, band).map_err(table)?;
    match sec {
        geom_brep::PlaneCylinderSection::TiltedEllipse(e) => {
            let geom::Curve3::Ellipse {
                center,
                axis,
                major,
                minor,
                u_ref,
            } = e
            else {
                return Err(invariant("tilted classification carried a non-ellipse"));
            };
            Ok(SectionCase::Conic(SectionConic {
                center,
                normal: axis,
                major: u_ref,
                sa: major,
                sb: minor,
                carrier: e.clone(),
            }))
        }
        geom_brep::PlaneCylinderSection::Rim(c) => {
            let geom::Curve3::Circle {
                center,
                axis,
                radius,
                u_ref,
            } = c
            else {
                return Err(invariant("rim classification carried a non-circle"));
            };
            Ok(SectionCase::Conic(SectionConic {
                center,
                normal: axis,
                major: u_ref,
                sa: radius,
                sb: radius,
                carrier: c.clone(),
            }))
        }
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

/// The wall chart an azimuth window lives in: azimuth is measured ccw
/// about `axis` from `u_ref`, and containment margins are metered at
/// `radius` (azimuth × chart radius — metres, the PR 6 convention).
struct ChartFrame<T: Real> {
    origin: Point3<T>,
    axis: Vec3<T>,
    radius: T,
    u_ref: Vec3<T>,
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
    let v_e = conic.normal.cross(conic.major);
    // Exact conic parameters of the (on-locus) endpoints.
    let theta_of = |p: Point3<T>| -> T {
        let d = p - conic.center;
        (d.dot(v_e) / conic.sb).atan2(d.dot(conic.major) / conic.sa)
    };
    let th1 = theta_of(p1);
    let th2 = theta_of(p2);
    let (w_min, w_max) = window;
    let width = w_max - w_min;
    // The chord's endpoints in the SAME chart frame the window lives
    // in (cylinder chart: azimuth ccw about the axis from `u_ref`).
    let chart_az = |p: Point3<T>| -> T {
        let w = p - chart.origin;
        let radial = w - chart.axis * w.dot(chart.axis);
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
    // Centred, the reduction's argument lies in
    // [τ/2 − width/2, τ/2 + width/2] for a start inside the window, so
    // its distance to the nearest period boundary is **(τ − width)/2** —
    // half the window's COMPLEMENT, not half the window. That distance
    // is positive only because the `width ≥ τ` arm above already
    // returned, and it is that arm's own band that makes it more than
    // infinitesimally positive: a window a hair under a full period
    // escalates there rather than reaching a knife-edge reduction here.
    // On the shipped belly/tilted cuts the complement is most of a
    // period, which is why the margin is comfortable in practice.
    let half_w = width * T::from_f64(0.5);
    let half_tau = tau * T::from_f64(0.5);
    let x1 = (a1 - (w_min + half_w) + half_tau).reduce_periodic(tau) - half_tau + half_w;
    // The azimuth gap to the chord's end. A difference, like every
    // quantity here, so a rotated `u_ref` (a moved seam) cancels.
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
    if ccw {
        // The ccw arc from p1 lies in the face.
        let span = (th2 - th1).reduce_periodic(tau);
        Ok((conic.carrier.clone(), th1, th1 + span))
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
        Ok((flipped, th1f, th1f + span))
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
#[allow(clippy::too_many_arguments)] // one internal lane, each argument a named duty
fn chord_spec<T: Decide>(
    body: &mut Body<T>,
    band: Band,
    lane: JoinLane<'_, T>,
    face: FaceKey,
    run: &[HalfEdgeKey],
    u1: VertexKey,
    u2: VertexKey,
) -> Result<Option<EdgeCurveSpec<T>>, SplitJoinError> {
    // Self-loop chords keep the scaffolding-circle convention.
    if u1 == u2 {
        return Ok(None);
    }
    let face_data = body.get_face(face).ok_or_else(|| corrupt_face(face))?;
    let wall_key = face_data.surface;
    let (o_c, a_c, r_c, u_ref_c) = match body.get_surface(wall_key) {
        Some(geom::Surface::Plane { .. }) => {
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
            };
        }
        Some(&geom::Surface::Cylinder {
            origin,
            axis,
            radius,
            u_ref,
        }) => (origin, axis, radius, u_ref),
        // The sphere wall (M5 S13): the chart frame is (center, polar
        // axis, radius, seam u_ref) — azimuth about the polar axis,
        // exactly the shape the S9 tail below meters.
        Some(&geom::Surface::Sphere {
            center,
            radius,
            axis,
            u_ref,
        }) => (center, axis, radius, u_ref),
        // Post-gate unreachable kinds — typed, never assumed.
        Some(_) | None => {
            return Err(SplitJoinError::SectionInvariant {
                face,
                what: "section chord requested on a face kind the gate refuses",
            });
        }
    };
    let JoinLane::Split(ctx) = lane else {
        return Err(SplitJoinError::SectionInvariant {
            face,
            what: "curved section chord outside the split/wall lane (no section context)",
        });
    };
    // The wall surface (cylinder OR sphere since M5 S13).
    let cyl_s = body
        .get_surface(wall_key)
        .cloned()
        .ok_or_else(|| corrupt_face(face))?;
    // A transient classification value: only origin/normal are read by
    // the table (u_ref is a placement convention the classification
    // never consumes; the STORED aux plane below gets an honest one).
    let plane_s = geom::Surface::Plane {
        origin: ctx.plane.origin,
        normal: ctx.plane.normal,
        u_ref: ctx.plane.normal,
    };
    let extent = face_extent(body, u1, face).map_err(|_| corrupt_face(face))?;
    let conic = match section_case(face, band, &plane_s, &cyl_s, extent)? {
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
                Margin::metered(t2 - t1, len),
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
                        origin: ctx.plane.origin,
                        normal: ctx.plane.normal,
                        // Honest u_ref: the ruling direction lies in
                        // the plane by the tangency classification.
                        u_ref: dir / len,
                    });
                    ctx.plane_key = Some(k);
                    k
                }
            };
            let witness = carrier.eval(s1 + (s2 - s1) * T::from_f64(0.5));
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
    // ---- The arc side, by azimuth-window containment (fn docs) ----
    //
    // The divided face's own window, from the run this chord co-bounds
    // it with. A run with no charted edge leaves the face without a
    // window: refused typed, never guessed.
    let Some(window) = run_azimuth_window(body, &cyl_s, face, run, band)? else {
        return Err(SplitJoinError::SectionArcWindow {
            face,
            case: ArcWindowCase::NoChartedRun,
            band,
        });
    };
    let chart = ChartFrame {
        origin: o_c,
        axis: a_c,
        radius: r_c,
        u_ref: u_ref_c,
    };
    let (carrier, t_start, t_end) = select_arc(face, band, &chart, &conic, window, p1, p2)?;
    // The aux plane surface (honest u_ref: the section's major
    // direction, ⊥ normal by construction), minted once per split.
    let plane_key = match ctx.plane_key {
        Some(k) => k,
        None => {
            let k = body.add_surface(geom::Surface::Plane {
                origin: ctx.plane.origin,
                normal: ctx.plane.normal,
                u_ref: conic.major,
            });
            ctx.plane_key = Some(k);
            k
        }
    };
    let witness = carrier.eval(t_start + (t_end - t_start) * T::from_f64(0.5));
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
pub(crate) fn face_azimuth_window<T: Decide>(
    body: &Body<T>,
    surface: &geom::Surface<T>,
    face: FaceKey,
    band: Band,
) -> Result<Option<(T, T)>, SplitJoinError> {
    let outer = body.get_face(face).ok_or_else(|| corrupt_face(face))?.outer;
    let crate::entity::LoopBoundary::Cycle { first } = body
        .get_loop(outer)
        .ok_or_else(|| corrupt_loop(outer))?
        .boundary
    else {
        return Ok(None);
    };
    let halves = body.loop_cycle(first).ok_or_else(|| corrupt_he(first))?;
    run_azimuth_window(body, surface, face, &halves, band)
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
    // shared rule, called with a window this lane did not derive. ----
    let chart = ChartFrame {
        origin: o_c,
        axis: a_c,
        radius: r_c,
        u_ref: u_ref_c,
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
    let witness = carrier.eval(t_start + (t_end - t_start) * T::from_f64(0.5));
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

/// The adjacency skip, for both chords of a `join`: an already-adjacent
/// pair whose between edge lies in the plane needs no chord — it IS the
/// section segment (M3). A belly conic between them is not a section
/// segment and its chord MUST be minted (M1 fix), and an escalated
/// in-plane verdict refuses typed rather than guessing either way.
///
/// One body for both guards: they were two copies reconciled by a
/// comment saying "same rule as the first guard".
fn skip_adjacent_chord<T: Decide>(
    body: &Body<T>,
    lane: &JoinLane<'_, T>,
    between: HalfEdgeKey,
    face: FaceKey,
    band: Band,
) -> Result<bool, SplitJoinError> {
    match between_edge_in_plane(body, lane, between, band)? {
        Some(in_plane) => Ok(in_plane),
        None => Err(SplitJoinError::SectionInvariant {
            face,
            what: "in-plane classification of the join-adjacent edge escalated",
        }),
    }
}

/// Whether the (real) edge under `he` lies IN the split plane over its
/// whole span — the adjacency-skip guard's question (M1 fix pass):
/// `None` = escalated. Lines and null scaffolding answer `true` with
/// NO predicate evaluation (the M3 path, bit-identical: a line whose
/// join-adjacent role puts it between two ON copies is the in-plane
/// section edge); conics ask the named trilean
/// `split_conic_inplane_mid` — margin the mid-parameter plane distance
/// (meters). With both endpoints ON and the interior sign-constant
/// (root insertion split every interior crossing out), a Zero midpoint
/// pins the whole arc in-plane; a definite midpoint is a belly arc —
/// the skipped chord MUST be minted or the section face inherits an
/// off-plane boundary edge.
fn between_edge_in_plane<T: Decide>(
    body: &Body<T>,
    lane: &JoinLane<'_, T>,
    he: HalfEdgeKey,
    band: Band,
) -> Result<Option<bool>, SplitJoinError> {
    let he_data = body.get_half_edge(he).ok_or_else(|| corrupt_he(he))?;
    let edge = body
        .get_edge(he_data.edge)
        .ok_or_else(|| corrupt_edge(he_data.edge))?;
    // Named only by the two invariant arms below, which are off the
    // hot path.
    let owning_face = || {
        body.get_loop(he_data.parent_loop)
            .map(|l| l.face)
            .ok_or_else(|| corrupt_loop(he_data.parent_loop))
    };
    let Some(CurveGeom::Certified(curve)) = body.get_curve_geom(edge.curve) else {
        return Ok(Some(true)); // null scaffolding: zero-length, ON
    };
    match curve.carrier() {
        geom::Curve3::Line { .. } | geom::Curve3::Nurbs(_) => Ok(Some(true)),
        geom::Curve3::Circle { .. } | geom::Curve3::Ellipse { .. } => {
            let (t0, t1) = curve.params();
            let mid = curve.carrier().eval(t0 + (t1 - t0) * T::from_f64(0.5));
            match lane {
                JoinLane::Planar => Err(SplitJoinError::SectionInvariant {
                    face: owning_face()?,
                    what: "the all-planar join lane reached a conic run edge (the operand \
                           gate promises every carrier planar)",
                }),
                JoinLane::Split(ctx) => {
                    let margin = Margin::of((mid - ctx.plane.origin).dot(ctx.plane.normal));
                    match decide("split_conic_inplane_mid", margin, band) {
                        Ok(Sign::Zero) => Ok(Some(true)),
                        Ok(Sign::Positive | Sign::Negative) => Ok(Some(false)),
                        Err(_) => Ok(None),
                    }
                }
                // The boolean's PLANAR side (M5 PR 9 fix pass, dev 4):
                // every edge of the divided planar face lies in the
                // germ PLANE by face containment — but on a CONIC germ
                // locus "in plane" is not "is the section segment":
                // both complementary arcs share the plane AND the
                // locus (the two-semicircle 2-gon is the witness — the
                // old structural `true` skipped the second side's mint
                // and desynced the zip). The honest test is WINDOW
                // membership: the between arc is this match's own side
                // exactly when its midpoint lies in the lane's wall
                // window (the same cone comparison the containment
                // layer decides trim with; Zero = graze = escalate).
                JoinLane::BoolPlanar { wall, window, .. } => {
                    // The wall's chart frame — cylinder (PR 9) or
                    // sphere (M5 S13); the branch-cut-free cosine
                    // window test below is chart-frame generic.
                    let (o_c, a_c, r_c, u_ref_c) = match wall {
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
                                face: owning_face()?,
                                what: "boolean planar-side germ partner is neither a cylinder \
                                       nor a sphere (arm not wired)",
                            });
                        }
                    };
                    let (w_min, w_max) = *window;
                    let half = T::from_f64(0.5);
                    let m_ang = (w_min + w_max) * half;
                    let (s_m, c_m) = m_ang.sin_cos();
                    let v_ref = a_c.cross(*u_ref_c);
                    let m_hat = *u_ref_c * c_m + v_ref * s_m;
                    let w = mid - *o_c;
                    let radial = w - *a_c * w.dot(*a_c);
                    let r_hat = radial / radial.norm();
                    let c_h = ((w_max - w_min) * half).cos();
                    // Ledger row F8 (unchanged): (cosΔ − cos h)·r is
                    // quadratic in the angular deviation for narrow
                    // windows; the margin is a length (levered) today.
                    let margin = Margin::levered(r_hat.dot(m_hat) - c_h, *r_c);
                    match decide("bool_between_arc_window", margin, band) {
                        Ok(Sign::Positive) => Ok(Some(true)),
                        Ok(Sign::Negative) => Ok(Some(false)),
                        Ok(Sign::Zero) => Ok(None),
                        Err(_) => Ok(None),
                    }
                }
            }
        }
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
/// sampled bound. The trigonometric amplitudes ride along as an
/// explicit conservative widening that is exactly zero for every
/// pcurve this lane derives (the PR 6 snap-slack idiom: the term keeps
/// the statement true if the family ever widens, and costs nothing on
/// the ship path).
///
/// **The closed-form lane only, and it says so with `None`.** The join
/// lane reads a chart image's azimuth through its harmonic amplitudes;
/// a fitted (rung-3) image has none, so it gets no answer rather than a
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
fn chart_azimuth_range<T: SpanLocate>(p: &Pcurve<T>, t0: T, t1: T) -> Option<(T, T)> {
    let Pcurve::Harmonic { pa, pb, .. } = *p else {
        return None;
    };
    let amp = pa.x.abs() + pb.x.abs();
    let u0 = p.eval(t0).x;
    let u1 = p.eval(t1).x;
    Some((u0.min(u1) - amp, u0.max(u1) + amp))
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
/// it. That is deliberate and bounded — the run's edges are already
/// certified against their own surfaces, so a chart image that does not
/// represent one is corrupt-input territory, and the *chord this rule
/// selects* is certified by the ordinary `mef` gate and then re-derived
/// and certified again by PR 6's mint pass at the end of the split. A
/// wrong window can therefore only make this rule REFUSE (or, in the
/// unreachable corrupt case, mint an arc the downstream certification
/// rejects) — never quietly widen what ships.
///
/// The branch of each run edge is pinned exactly as the PR 6 loop walk
/// pins it: the whole number of periods that lands this edge's entry
/// azimuth on the previous edge's exit. No per-sample unwrapping exists
/// here either. Null scaffolding halves are zero-length coincident
/// copies — they carry no azimuth extent and no branch information, and
/// are stepped over without breaking the chain.
fn run_azimuth_window<T: Decide>(
    body: &Body<T>,
    surface: &geom::Surface<T>,
    face: FaceKey,
    halves: &[HalfEdgeKey],
    band: Band,
) -> Result<Option<(T, T)>, SplitJoinError> {
    let tau = T::tau();
    let half = T::from_f64(0.5);
    let mut acc: Option<(T, T)> = None;
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
                let q = (prev - raw) / tau;
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
                let mut k = (q + half).floor();
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
                            k = if advancing {
                                q.floor() + T::one()
                            } else {
                                T::zero() - (T::zero() - q).floor() - T::one()
                            };
                        }
                    }
                }
                base.shift_branch(k, tau)
                    .ok_or(SplitJoinError::SectionInvariant {
                        face,
                        what: "a run edge's chart image could not be branch-shifted (the \
                               re-validated rebuild refused)",
                    })?
            }
        };
        let (lo, hi) =
            chart_azimuth_range(&pcurve, t0, t1).ok_or(SplitJoinError::SectionInvariant {
                face,
                what: "a run edge's chart image is FITTED (rung-3) — this window rule reads \
                       a closed-form azimuth, and the fitted-chord join lane is not \
                       written",
            })?;
        acc = Some(match acc {
            None => (lo, hi),
            Some((a, b)) => (a.min(lo), b.max(hi)),
        });
        prev_exit = Some(pcurve.eval(exit_t).x);
    }
    Ok(acc)
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
    pub(crate) fn join<T: Decide>(
        &mut self,
        body: &mut Body<T>,
        h1: HalfEdgeKey,
        h2: HalfEdgeKey,
        mut lane: JoinLane<'_, T>,
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
                skip_adjacent_chord(body, &lane, prev(body, h1)?, oldf, self.band)?
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
                let spec = chord_spec(
                    body,
                    self.band,
                    lane.reborrow(),
                    oldf,
                    &run_halves,
                    start_of(body, h1)?,
                    start_of(body, outside)?,
                )?;
                // The fragment INHERITS `oldf`'s orientation bit.
                // Both arms hand `mef` the parent's surface, and
                // `mint_face_surface_and_sense` returns the parent's
                // sense whenever the fragment lands on it: a piece of
                // a reversed wall is the same surface region with the
                // same material side, so stamping `true` here would
                // mint a silently inside-out fragment. Guard: sweep's
                // `m5_s12_curved_ops.rs`, the row named
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
            // Cross-loop joins have no single co-bounded run; the
            // window comes from the target's whole cycle (unreached by
            // any curved fixture in this PR — typed doors downstream,
            // and a cycle that wraps the chart refuses `BothContained`
            // rather than guessing).
            let target_cycle = body.loop_cycle(target).ok_or_else(|| corrupt_he(target))?;
            let spec = chord_spec(
                body,
                self.band,
                lane.reborrow(),
                oldf,
                &target_cycle,
                start_of(body, target)?,
                start_of(body, ring)?,
            )?;
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
            skip_adjacent_chord(body, &lane, next(body, h1)?, oldf, self.band)?
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
            let run2: Vec<HalfEdgeKey> = if adjacent2 {
                vec![next(body, h1)?]
            } else if prev_adjacent {
                vec![prev(body, h1)?]
            } else {
                run_halves.clone()
            };
            let spec = chord_spec(
                body,
                self.band,
                lane,
                owner,
                &run2,
                start_of(body, h2)?,
                start_of(body, next(body, h1)?)?,
            )?;
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
    /// by the trilean containment predicate, never a raw comparison.
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
        let normal = face_plane_normal(body, oldf)?;
        for ring in rings {
            if ring == remainder {
                continue;
            }
            let rep = ring_representative(body, ring)?;
            match point_in_loop(body, run, normal, rep, self.band)? {
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
/// to recover the loop's PLANE and whose verdict is exactly invariant
/// under `n̂ ↦ −n̂`. **That derivation lives at `point_in_loop`**,
/// under the function whose property it is rather than under the
/// five-line producer that relies on it; the consequence here is that
/// ring re-homing cannot move a ring on the sense bit, and
/// `tests/review_m3_pr3_pil.rs` pins it.
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

/// A representative point of a ring (its anchor vertex).
fn ring_representative<T: Decide>(
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
        let normal = Vec3::new(phi.sin(), 0.0, phi.cos());
        let plane = super::SplitPlane {
            origin: Point3::new(0.0, 0.0, 0.0),
            normal,
        };
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
        let seed = body.mvfs(p1).unwrap();
        body.set_face_surface(
            seed.face,
            crate::FaceSurface::New(geom::Surface::Cylinder {
                origin: Point3::origin(),
                axis: Vec3::unit_z(),
                radius: 1.0,
                u_ref: Vec3::unit_x(),
            }),
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
            plane,
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
        let seed = body.mvfs(carrier.eval(t0)).unwrap();
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
                        witness: carrier.eval((t0 + t1) * 0.5),
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
            &run,
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
        // Outside the split lane (no section context) a conic chord is
        // an invariant violation, typed.
        let band = Band::new(1e-9, 1e-8).unwrap();
        let (mut body, face, u1, u2, _) = cyl_fixture();
        let run = vec![rim_run(&mut body, -0.2, core::f64::consts::FRAC_PI_2 + 0.2)];
        let err = chord_spec(&mut body, band, JoinLane::Planar, face, &run, u1, u2).unwrap_err();
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

        for msg in [definite.to_string(), escalated.to_string()] {
            assert_eq!(
                msg.matches(geom_core::COINCIDENCE_RECOURSE).count(),
                1,
                "{msg}"
            );
            assert!(msg.contains("split_arc_window"), "{msg}");
            assert!(msg.contains("1e-9") && msg.contains("1e-8"), "{msg}");
        }
        // The sub-case that classified nothing must NOT carry the
        // recourse — there is no ill-conditioned margin behind it.
        let no_run = spec_with(&[]).unwrap_err().to_string();
        assert_eq!(
            no_run.matches(geom_core::COINCIDENCE_RECOURSE).count(),
            0,
            "{no_run}"
        );
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
    /// shared recourse fragment.
    #[test]
    fn section_area_pair_carries_the_shared_recourse() {
        let face = FaceKey::default();
        let msg = SplitJoinError::DegenerateSection { face }.to_string();
        assert_eq!(
            msg.matches(geom_core::COINCIDENCE_RECOURSE).count(),
            1,
            "{msg}"
        );

        let msg = SplitJoinError::Escalated {
            face,
            diag: Indeterminate {
                margin: geom_core::MarginDiag::Value(5e-9),
                band: Band::new(1e-9, 1e-8).unwrap(),
                predicate: Some("split_section_area"),
            },
        }
        .to_string();
        assert_eq!(
            msg.matches(geom_core::COINCIDENCE_RECOURSE).count(),
            1,
            "{msg}"
        );
    }

    /// **The anti-re-fork row for the arc-side rule.** Each of the
    /// three rungs the two chord lanes share is decided in exactly ONE
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
                     out of chord_join.rs (smell scan S5). Call `select_arc` / \
                     `section_case` instead.",
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
