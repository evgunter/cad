//! Trilean **point-in-loop** containment — the ring re-homing decision
//! (ch. 14's `laringmv`, Problem 13.5) and the seed of F8's 3-D
//! containment machinery. Kept general: a planar loop plus a query
//! point in its plane → In / Out / OnBoundary, with typed escalation —
//! **no unbanded raw comparisons** (every decision is a named K
//! predicate through the Q1 funnel).
//!
//! # Method: in-plane ray parity with a deterministic retry schedule
//!
//! [`point_in_loop`] first certifies its plane: the normal is a unit
//! vector, and the loop and `q` lie in the plane through the loop's
//! first vertex with that normal (the `point_in_loop_{normal,plane,
//! query}` rows below). It then reads the loop on its edges' own
//! carriers. A loop of lines is the polygon through its vertices, and
//! its walk — the boundary pre-pass and one ray's parity count — is
//! [`crate::ray_walk`], shared with the 2-D chart-space consumer. A
//! loop bearing any other edge crosses each conic on its conic, and
//! each spiric or spline arc piece by piece (the `point_in_arc_loop_*`
//! rows). What this module owns is the *3-D* half: reading the loop out
//! of the body, and turning each member of the space-direction schedule
//! into an in-plane frame.
//!
//! Cast a ray from `q` in a direction lying in the loop's plane and
//! count proper crossings; odd ⇒ In. Grazing configurations (an
//! endpoint on the ray line, a crossing at the query point) are not
//! errors of the geometry but of the *ray choice*: retry with the next
//! direction of a fixed schedule (profile's device of a fixed ray
//! schedule, promoted to 3-D; profile steps by the golden angle, this
//! table is dyadic). The schedule directions are built from a fixed
//! table of space directions projected into the plane — no basis
//! choice by coordinate comparison anywhere (a comparison-picked basis
//! could diverge between the f64 and interval lanes; a fixed schedule
//! whose degenerate members are *detected by predicate* cannot):
//! direction k is `r_k − n(n·r_k)` for the k-th schedule vector `r_k`,
//! accepted iff the in-plane displacement it commands at the loop's
//! own scale — `(|r_k − n(n·r_k)| / |r_k|) · extent`, not the bare
//! projected length — is definitely positive (**`point_in_loop_arm`**;
//! a near-parallel `r_k` casts no ray, an in-band one is set aside like
//! any ray-level reading ([`ray_walk::walk`]), and a loop collapsed
//! onto `q` zeroes the arm for *every* member and ends in
//! `RayExhausted`).
//!
//! # Predicates (all K-tagged, meters)
//!
//! - **`point_in_loop_normal`**, **`point_in_loop_plane`**,
//!   **`point_in_loop_query`**: the plane certification — `(|n| − 1)`
//!   levered by the loop's reach; a loop feature's distance off the
//!   plane (a vertex, a conic's centre, its tilt levered by its larger
//!   semi-axis, a spiric's cutting plane the same way, a spline's
//!   control point); `q`'s distance off it.
//!   Anything but `Zero` refuses ([`PointInLoopError::OffPlane`]).
//! - **`point_in_loop_segment`**: an edge segment's own length — the
//!   degeneracy gate. Zero-length segments are legal null scaffolding
//!   in mid-join loops, and the clamped-foot division would poison on
//!   them, so they measure the point distance directly; a
//!   certified-short-but-nonzero segment is a genuine sliver and
//!   escalates. A *different question* from the row below, hence a
//!   different name.
//! - **`point_in_loop_boundary`**: distance of `q` to an edge segment
//!   (perpendicular distance when the foot lies inside the span,
//!   endpoint distance otherwise) — Zero ⇒ `OnBoundary`.
//! - **`point_in_loop_arm`**: a projected schedule direction's
//!   in-plane fraction levered by the loop's reach from `q` (skip
//!   gate, see above) — the loop's own extent is half of it.
//! - **`point_in_loop_side`**: signed offset of an edge endpoint from
//!   the ray line — anything but definitely off it ⇒ grazing ⇒ next ray.
//! - **`point_in_loop_advance`**: the crossing's advance along the
//!   ray — anything but definitely positive or negative would put a
//!   crossing at `q` itself (contradicting the boundary pre-pass) ⇒
//!   next ray.
//!
//! Both are facts about one ray, never about `q`: an in-band reading on
//! either sets the ray aside, and is the walk's refusal only if no ray
//! decides.
//!
//! [`point_in_loop`] reads the polygon rows only for a loop of lines; on a
//! loop bearing any other edge, its walk and the one boundary reading
//! of an edge on its carrier ([`LoopEdge::contact`]) carry their own
//! rows, each caller naming them through a [`BoundaryRows`] value
//! (the walk's are `WALK_ROWS`; `boolean::contain`'s pre-pass passes
//! `bool_contact_*` names):
//!
//! - **`point_in_arc_loop_{segment,boundary,side,advance}`**: the four
//!   rows above, over the loop's STRAIGHT edges.
//! - **`point_in_arc_loop_arm`**: the schedule gate, with the curved
//!   edges' balls' reach in the extent.
//! - **`point_in_arc_loop_reach`**: a ray's clearance from a spiric
//!   piece's ball, less its reach, where anything undecided halves the
//!   piece.
//! - **`point_in_arc_loop_spiric_{end,clear,on,leaf}`**: the boundary
//!   reading of a spiric edge ([`PiecedArc::contact`] on a
//!   [`SpiricArc`]) — the distance to an end, a piece's ball's clearance
//!   from the point, the distance to a point of the arc, and a piece's
//!   own radius, which ends the halving.
//! - **`point_in_arc_loop_spiric_{side,turn,advance}`**: a ray's
//!   crossing of a spiric piece ([`PiecedArc::crossings`]) — an end's
//!   offset from the ray line, the piece's monotonicity margin, and its
//!   ball's advance along the ray — where anything undecided halves the
//!   piece, and a piece still unsettled at the depth grazes the ray.
//! - **`point_in_arc_loop_spline_{end,clear,on,leaf}`**: the boundary
//!   reading of a spline edge ([`PiecedArc::contact`] on a
//!   [`SplineArc`]) — the distance to an end, a piece's control ball's
//!   clearance from the point, the distance to the piece's start point,
//!   and the control ball's own radius, which ends the halving.
//! - **`point_in_arc_loop_spline_{side,turn,advance}`**: a ray's
//!   crossing of a spline piece ([`PiecedArc::crossings`]) — a control
//!   point's offset from the ray line, a step of those offsets along the
//!   control polygon, and a control point's advance along the ray —
//!   where anything undecided halves the piece, and a piece still
//!   unsettled at the depth grazes the ray.
//! - **`point_in_arc_loop_conic_span`**: a conic arc's gap to a full
//!   period, `(τ − w)` levered by the smaller semi-axis — read only for a
//!   window wound definitely PAST a period, which is no edge. Anything
//!   else is an arc, a whole turn included (its two ends coincide).
//! - **A boundary point against a conic arc** ([`ConicArc::hit`]) is
//!   read by the two readings at the point's foot on the carrier:
//!   `geom`'s on-carrier reading (`curve_carrier_gap`, and off only on the
//!   lower bound `curve_carrier_clear`, the ellipse's Hessian-bounded
//!   root, the two straddling on `curve_carrier_straddle`), and the span's
//!   reading of its ends (`curve_pair_span_end_*`).
//! - **`point_in_arc_loop_conic_end`**: on a ray's crossing, the distance
//!   to either end of the arc ([`arc_trim`]'s step 1), the unit chord
//!   times the smaller semi-axis.
//! - **`point_in_arc_loop_conic_window`**, **`_disc`**, **`_advance`**: a
//!   ray's crossing of a conic — the distance trim's defect sum
//!   ([`arc_trim`]), the discriminant, the root's advance — each levered
//!   by the smaller semi-axis, where a `Zero` grazes the ray and an
//!   in-band margin sets it aside.
//!
//! **`point_in_arc_loop_conic_straddle`** classifies nothing: it names an
//! ellipse's two decided bounds on its span's overlap straddling the
//! whole band ([`geom_core::k_stats::straddle`]).

use geom_brep::recourse::{LeverOnly, Reading, RefusedArm, RefusedSide, SizedDecision, SizedPass};
use geom_core::period::{Exact, Extent, Period, Turn, TurnRow, TurnRows, TurnUndecided, Within};
use geom_core::{Band, Decide, Indeterminate, Margin, MarginDiag, Point3, Sign, Vec3};

use crate::body::Body;
use crate::entity::{EdgeKey, EntityId, LoopBoundary, LoopKey};
use crate::live::{linked, proven};
use crate::ray_walk::{self, ParityRows, RayFault};
use crate::splitting::pieced_arc::{ArcHit, ArcRefusal, ArcRows, PiecedArc};
use crate::splitting::spiric_arc::{Oval, SpiricArc};
use crate::splitting::spline_arc::SplineArc;
use crate::validate::decide;

/// This consumer's K rows for the shared walk, and the greppable
/// roster entry for all four (see [`ParityRows`]). The 3-D loop's
/// `point_in_loop_arm` row is not among them: it gates the *frame*
/// construction below, which is this consumer's own, and is written
/// at its own `decide` call.
const ROWS: ParityRows = ParityRows {
    segment: "point_in_loop_segment",
    boundary: "point_in_loop_boundary",
    side: "point_in_loop_side",
    advance: "point_in_loop_advance",
};

/// The trilean answer.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LoopContainment {
    /// Strictly inside the loop's plane region.
    In,
    /// Strictly outside.
    Out,
    /// On the loop's boundary (within the band) — the caller decides
    /// what that means (ring re-homing treats it as ill-conditioned).
    OnBoundary,
}

/// **The question the planar loop walk escalated on** (D4 ¶1 (i)): the
/// closed type [`PointInLoopError::Escalated`] carries, so its ending is an
/// exhaustive match over the decision rather than a lookup by predicate
/// name ([`Self::ending`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LoopDecision {
    /// Whether the point stands on the loop's boundary — a vertex, an
    /// edge, an arc or an arc's end — or clear of it. Its margins are
    /// lengths: the point's distance from each, and a straight edge's own
    /// length where the edge is short enough to read as a point. Every
    /// definite reading answers.
    Boundary,
    /// Where one ray of the walk's fixed schedule meets the boundary: a
    /// corner's offset from the ray's line, a crossing's advance along
    /// it, how far the ray's direction lies in the loop's plane. The
    /// point is already clear of the boundary here; the margin is a fact
    /// about the kernel's choice of ray, not a size the user chose.
    Ray,
    /// Whether an arc stays short of a full turn: its gap to one. On an
    /// elliptic edge a gap of zero is a whole ellipse, which passes, and
    /// the walk escalates only on an arc over-wound by less than the band,
    /// or on its two bounds straddling the band; on a scaffold conic a
    /// positive gap passes.
    ArcSpan,
    /// Whether the loop and the point lie in the plane the caller handed
    /// [`point_in_loop`] (its preconditions, certified): a residual, which
    /// passes only at zero.
    Plane,
}

impl LoopDecision {
    /// Every decision, for the rows that sample them.
    pub const ALL: [Self; 4] = [Self::Boundary, Self::Ray, Self::ArcSpan, Self::Plane];

    /// What the decision decides, as a clause.
    #[must_use]
    pub const fn subject(self) -> &'static str {
        match self {
            Self::Boundary => "whether a point lies on the loop's boundary or clear of it",
            Self::Ray => "where a test ray from a point meets the loop's boundary",
            Self::ArcSpan => "whether an arc of the loop stays short of a full turn",
            Self::Plane => "whether a point and the face lie in one plane",
        }
    }

    /// The decision's geometry lever, after "Recourse: " — the one source
    /// every rendering of it reads.
    #[must_use]
    pub const fn lever(self) -> &'static str {
        match self {
            Self::Boundary => "move the point exactly onto the boundary or clearly off it",
            Self::Ray => "nudge the point so no boundary corner lines up with it",
            Self::ArcSpan => "move the geometry so this arc stays clearly short of a full turn",
            Self::Plane => "move the geometry so the face is flat and the point lies on it",
        }
    }

    /// The one ending an escalation of this decision carries, read at
    /// `reading` (D4 ¶1 (i)), from the shared table: the point's place is
    /// a size the user may intend, and every definite reading of it
    /// answers; an arc's gap to a full turn passes at zero and above; a
    /// ray's margin is no size the user chose, and the plane is a
    /// residual, so each ends in its lever alone.
    #[must_use]
    pub fn ending(self, escalation: Escalation, diag: &Indeterminate, reading: Reading) -> String {
        let arm = escalation.arm(diag);
        let sized = |size, passes| placement_sized(self.lever(), size, passes);
        match self {
            Self::Boundary => sized("distance", SizedPass::AnySign).recourse(arm, reading),
            Self::ArcSpan => sized("arc", SizedPass::NonNegative).recourse(arm, reading),
            Self::Ray | Self::Plane => LeverOnly {
                lever: self.lever(),
            }
            .recourse(arm, reading),
        }
    }
}

/// A placement decision on a size the user may intend, read over stored
/// geometry by its lever: the one shape every sized placement decision
/// (the walk's and contfp's) takes.
pub(crate) const fn placement_sized(
    lever: &'static str,
    size: &'static str,
    passes: SizedPass,
) -> SizedDecision {
    SizedDecision {
        lever,
        size,
        passes,
        side: RefusedSide::Lever,
        no_size: None,
    }
}

/// **How an escalation's refused reading stands**, as the site that
/// raised it knows it: the one thing the carried [`Indeterminate`] cannot
/// say.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Escalation {
    /// The carried margin is the reading: in the band, or unreadable.
    Margin,
    /// Two sound bounds on the one quantity straddle the band, so no
    /// verdict of either states the reading; the carried diagnostic is
    /// their enclosure, under the site's straddle row.
    Straddle,
}

impl Escalation {
    /// The refused arm the shared table reads this escalation as.
    #[must_use]
    pub fn arm(self, diag: &Indeterminate) -> RefusedArm<'_> {
        match self {
            Self::Margin => RefusedArm::Undecided(diag),
            Self::Straddle => RefusedArm::Straddle,
        }
    }
}

/// An escalation a boundary reading raises, and how its reading stands.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct ReadEscalation {
    /// How the reading stands.
    pub(crate) escalation: Escalation,
    /// The diagnostic (named predicate inside).
    pub(crate) diag: Indeterminate,
}

impl From<Indeterminate> for ReadEscalation {
    fn from(diag: Indeterminate) -> Self {
        Self {
            escalation: Escalation::Margin,
            diag,
        }
    }
}

/// Typed failure of [`point_in_loop`].
#[derive(Clone, Debug, PartialEq)]
// The variant roster the sample-coverage row reads (test builds only).
#[cfg_attr(
    test,
    derive(strum::EnumDiscriminants),
    strum_discriminants(name(PointInLoopErrorKind), vis(pub(crate)), derive(strum::EnumIter))
)]
pub enum PointInLoopError {
    /// A predicate escalated.
    Escalated {
        /// The loop being tested.
        r#loop: LoopKey,
        /// The question it escalated on.
        decision: LoopDecision,
        /// How its refused reading stands.
        escalation: Escalation,
        /// The escalation diagnostics (named predicate inside).
        diag: Indeterminate,
    },
    /// No schedule ray settled — each grazed or gave nothing to read
    /// ([`crate::ray_walk::NoRaySettled`]; profile's
    /// `RayCastingExhausted`).
    RayExhausted {
        /// The loop being tested.
        r#loop: LoopKey,
    },
    /// The loop is not a walkable cycle of resolvable geometry — a
    /// whole-turn scaffold circle included ([`carrier_loop`]).
    CorruptLoop {
        /// The loop.
        r#loop: LoopKey,
    },
    /// The walk could not decide: a ray definitely met an edge it could
    /// not cross, and no ray settled.
    Uncrossable(Uncrossable),
    /// A precondition of [`point_in_loop`] does not hold: the walk was
    /// handed a plane the loop or the point does not lie in.
    OffPlane(OffPlane),
}

/// **Which of [`point_in_loop`]'s preconditions failed**, for `loop`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OffPlane {
    /// The loop being tested.
    pub r#loop: LoopKey,
    /// What lies off the plane.
    pub cause: OffPlaneCause,
}

/// What lies off the plane [`point_in_loop`] was handed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OffPlaneCause {
    /// The normal is not a unit vector.
    NormalNotUnit,
    /// The loop does not lie in the plane through its first vertex with
    /// that normal — it is not planar, or the normal is not its plane's.
    /// `edge` is the first, in cycle order, with a vertex or carrier off
    /// the plane.
    Loop {
        /// That edge.
        edge: EdgeKey,
    },
    /// The query point does not lie in the loop's plane.
    Query,
}

impl core::fmt::Display for OffPlane {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.cause {
            OffPlaneCause::NormalNotUnit => {
                write!(
                    f,
                    "the plane normal given for the loop is not a unit vector"
                )
            }
            OffPlaneCause::Loop { .. } => write!(
                f,
                "the loop does not lie in the plane given for it: it is not planar, or the \
                 normal is not its plane's"
            ),
            OffPlaneCause::Query => write!(f, "the point does not lie in the loop's plane"),
        }
    }
}

/// **Where the arc-aware walk could not read an edge**: `edge`, a spiric
/// or spline edge of `loop`, is crossed piece by piece, and its boundary
/// reading ran out of depth or pieces before placing the point. (A piece
/// meeting a ray that the halving's depth settles neither way grazes
/// that ray instead.)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Uncrossable {
    /// The loop the walk could not read.
    pub r#loop: LoopKey,
    /// The first edge, in loop order, whose reading ran out.
    pub edge: EdgeKey,
    /// That edge's carrier.
    pub carrier: UncrossableCarrier,
}

/// The carrier of an edge the in-plane walk could not cross.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UncrossableCarrier {
    /// A plane's section of a torus.
    Spiric,
    /// A NURBS curve.
    Spline,
}

impl UncrossableCarrier {
    /// The carrier in a user's words.
    pub fn word(self) -> &'static str {
        match self {
            Self::Spiric => "torus-section",
            Self::Spline => "spline",
        }
    }
}

impl core::fmt::Display for Uncrossable {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "the test could not place the point against a {} edge of the outline: \
             cutting the edge into pieces ran out before the point was clear of every one",
            self.carrier.word()
        )
    }
}

impl PointInLoopError {
    /// This refusal as a consumer's own: `doing` names the walk's use
    /// ("re-homing a hole loop"), `subject` the question it answers
    /// ("which piece holds a hole loop"), and `recourse` the consumer's
    /// lever for a loop off the plane it was handed. One rendering for
    /// every consumer, so the endings cannot drift:
    ///
    /// - an escalation ends in its decision's lever;
    /// - an exhausted ray schedule names the walk's own levers;
    /// - an uncrossable spiric or spline edge is a limit of the walk, which
    ///   no move of the geometry lifts: `NOT_YET_ENDING`;
    /// - a loop the walk cannot walk is a torn body: `KERNEL_DEFECT_ENDING`;
    /// - a loop or point off the plane is the consumer's geometry, so
    ///   `recourse`.
    pub(crate) fn render_as(
        &self,
        f: &mut core::fmt::Formatter<'_>,
        doing: &str,
        subject: &str,
        recourse: &str,
    ) -> core::fmt::Result {
        match self {
            Self::Escalated {
                decision,
                escalation,
                diag,
                ..
            } => write!(
                f,
                "{doing}: {}",
                diag.undecided(
                    decision.subject(),
                    crate::boolean::ContainDecision::Loop(*decision)
                        .lever_ending(*escalation, diag),
                )
            ),
            Self::RayExhausted { .. } => write!(
                f,
                "{subject}{}: {}",
                geom_core::undecided!(),
                ray_walk::NoRaySettled
            ),
            Self::Uncrossable(u) => write!(
                f,
                "{subject} cannot be read: {u}. {}",
                geom_core::NOT_YET_ENDING
            ),
            Self::CorruptLoop { .. } => write!(
                f,
                "{subject} cannot be read: {self}. {}",
                geom_core::KERNEL_DEFECT_ENDING
            ),
            Self::OffPlane(o) => write!(f, "{subject} cannot be read: {o}. Recourse: {recourse}"),
        }
    }
}

impl core::fmt::Display for PointInLoopError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Escalated {
                decision,
                escalation,
                diag,
                ..
            } => diag
                .undecided(
                    decision.subject(),
                    decision.ending(*escalation, diag, Reading::Build),
                )
                .fmt(f),
            Self::RayExhausted { .. } => write!(
                f,
                geom_core::undecided!("whether a point lies in a loop", payload),
                ray_walk::NoRaySettled
            ),
            Self::CorruptLoop { r#loop } => {
                write!(f, "loop {loop:?} is not walkable")
            }
            Self::Uncrossable(u) => write!(
                f,
                geom_core::undecided!("whether a point lies in a loop", payload),
                u
            ),
            Self::OffPlane(o) => write!(f, "{o}"),
        }
    }
}

impl std::error::Error for PointInLoopError {}

impl PointInLoopError {
    /// An escalation of `decision` on `loop`, for a `map_err`.
    fn on(r#loop: LoopKey, decision: LoopDecision) -> impl Fn(ReadEscalation) -> Self + Copy {
        move |ReadEscalation { escalation, diag }| Self::Escalated {
            r#loop,
            decision,
            escalation,
            diag,
        }
    }

    /// An escalation of `decision` on `loop` on an in-band or
    /// unreadable margin, for a `map_err`.
    fn on_margin(r#loop: LoopKey, decision: LoopDecision) -> impl Fn(Indeterminate) -> Self + Copy {
        move |diag| Self::on(r#loop, decision)(diag.into())
    }
}

/// The fixed schedule of space directions (module docs). 16 entries:
/// three axes plus oblique members of dyadic components — for any unit
/// plane normal, most members project to a definitely-nonzero in-plane
/// direction (an all-graze outcome is the typed exhaustion error).
///
/// **The one table**, and no consumer owns it: this module's in-plane
/// ray parity (which projects each triple into the loop plane and
/// skips the near-parallel members), [`crate::splitting::order`]'s
/// point ordering (whose frame takes the three axes for an axis plane
/// and the members after them for any other, so the axes stay first),
/// [`crate::boolean::solid_contain`]'s containment sweep (which
/// normalizes the raw triple), and [`crate::boolean::sphere_region`]'s
/// ray directions. They do not
/// sweep the same directions. What each needs is only that its own
/// schedule is a `const` in a fixed order every run, which holds per
/// site — so a single definition buys the absence of drift between
/// copies, not determinism, which was never at risk.
///
/// **Editing an entry is not free, and what catches you is
/// incidental.** `tests/review_m3_pr3_pil.rs` hand-copies the fifteen
/// non-degenerate `+z` in-plane projections of this table (values),
/// and `splitting::order`'s sort tests pin the iteration order. Both
/// pin it as a side effect of testing something else. What is pinned
/// on purpose is the split `splitting::order` reads: the first three
/// entries are the axes and no later one is
/// (`the_schedule_opens_with_the_three_axes_and_only_them`), and no two
/// later ones lie near one line.
///
/// `pub(crate)` is the minimum visibility that reaches `boolean`, a
/// sibling of `splitting` rather than a descendant. `chart_region`'s
/// `SCHEDULE_2D` is a different table by dimension, not a fourth
/// reader of this one.
pub(crate) const SCHEDULE: [Vec3<f64>; 16] = [
    Vec3::new(1.0, 0.0, 0.0),
    Vec3::new(0.0, 1.0, 0.0),
    Vec3::new(0.0, 0.0, 1.0),
    Vec3::new(0.5, 0.25, 1.0),
    Vec3::new(1.0, 0.5, 0.25),
    Vec3::new(0.25, 1.0, 0.5),
    Vec3::new(-0.5, 1.0, 0.125),
    Vec3::new(0.125, -0.5, 1.0),
    Vec3::new(1.0, 0.125, -0.5),
    Vec3::new(0.75, -1.0, 0.375),
    Vec3::new(0.375, 0.75, -1.0),
    Vec3::new(-1.0, 0.375, 0.75),
    Vec3::new(0.625, 0.9375, 0.3125),
    Vec3::new(0.3125, -0.625, 0.9375),
    Vec3::new(0.9375, 0.3125, -0.625),
    Vec3::new(-0.75, -0.25, 1.0),
];

/// Collects the loop's vertex points in cycle order.
fn loop_points<T: Decide>(
    body: &Body<T>,
    r#loop: LoopKey,
) -> Result<Vec<Point3<T>>, PointInLoopError> {
    let corrupt = || PointInLoopError::CorruptLoop { r#loop };
    let loop_data = body.get_loop(r#loop).ok_or_else(corrupt)?;
    let LoopBoundary::Cycle { first } = loop_data.boundary else {
        return Err(corrupt()); // an empty loop bounds no region
    };
    let mut points = Vec::new();
    for he in body.loop_cycle(first).ok_or_else(corrupt)? {
        let start = body.get_half_edge(he).ok_or_else(corrupt)?.start;
        let p = *body
            .get_point(body.get_vertex(start).ok_or_else(corrupt)?.point)
            .ok_or_else(corrupt)?;
        points.push(p);
    }
    if points.len() < 2 {
        return Err(corrupt());
    }
    Ok(points)
}

/// Trilean containment of `q` in the POLYGON through `r#loop`'s
/// vertices, with unit plane normal `normal`; `q` is assumed to lie in
/// the loop's plane. That polygon is the loop's region only when every
/// edge is a line, so the walk's one caller is [`point_in_loop`] (and
/// its projected read), once every edge's carrier reads as a line;
/// check 9's tests call it only to show a fixture's polygon is not its
/// region.
///
/// Blind to the sign of `normal`, as [`point_in_loop`] states.
///
/// # Errors
///
/// [`PointInLoopError`] — escalation, ray exhaustion, or an
/// unwalkable loop.
pub(crate) fn point_in_vertex_polygon<T: Decide>(
    body: &Body<T>,
    r#loop: LoopKey,
    normal: Vec3<T>,
    q: Point3<T>,
    band: Band,
) -> Result<LoopContainment, PointInLoopError> {
    let escalate = PointInLoopError::on_margin(r#loop, LoopDecision::Boundary);
    let points = loop_points(body, r#loop)?;

    if ray_walk::on_boundary(&points, q, &ROWS, band).map_err(escalate)? {
        return Ok(LoopContainment::OnBoundary);
    }
    polygon_walk(r#loop, &points, normal, q, band)
}

/// [`point_in_vertex_polygon`]'s ray walk alone, for a point its
/// pre-pass (or a caller's) has placed off the boundary: `In` or `Out`.
fn polygon_walk<T: Decide>(
    r#loop: LoopKey,
    points: &[Point3<T>],
    normal: Vec3<T>,
    q: Point3<T>,
    band: Band,
) -> Result<LoopContainment, PointInLoopError> {
    let escalate = PointInLoopError::on_margin(r#loop, LoopDecision::Ray);
    // The loop's own reach from q (evaluation-lane fold): the lever
    // arm for the probe-direction gate below. A degenerate loop
    // collapsed onto q gives a zero arm, every schedule member skips,
    // and the walk ends in the typed `RayExhausted` — fail-loud.
    let mut extent = T::zero();
    for p in points {
        extent = extent.max((*p - q).norm());
    }
    let inside = walk_schedule(
        r#loop,
        normal,
        extent,
        "point_in_loop_arm",
        band,
        |d, side_axis| {
            RayFault::of(
                ray_walk::ray_verdict(points, q, d, side_axis, &ROWS, band),
                escalate,
            )
        },
    )?;
    Ok(if inside {
        LoopContainment::In
    } else {
        LoopContainment::Out
    })
}

/// **Ray parity over the fixed schedule, in a loop's plane** — the
/// frame both walks here share, driven by [`ray_walk::walk`]: each
/// schedule member projected into the plane, gated on the in-plane
/// displacement it commands at the loop's own `extent` (`arm_row`), and
/// handed to `ray` as the in-plane frame `(d, n̂ × d)`. A near-parallel
/// member casts no ray, and an in-band arm is a reading about that
/// member, not about `q`.
fn walk_schedule<T: Decide>(
    r#loop: LoopKey,
    normal: Vec3<T>,
    extent: T,
    arm_row: &'static str,
    band: Band,
    mut ray: impl FnMut(Vec3<T>, Vec3<T>) -> Result<bool, RayFault<PointInLoopError>>,
) -> Result<bool, PointInLoopError> {
    ray_walk::walk(
        &SCHEDULE,
        |r| {
            let r = r.map(T::from_f64);
            let d_raw = r - normal * normal.dot(r);
            // sin(schedule member, plane NORMAL) × loop extent: the
            // SCHEDULE triples are bare numbers, so the honest margin is
            // the in-plane displacement the probe direction commands at
            // the loop's own scale.
            let arm = Margin::levered(d_raw.norm() / r.norm(), extent);
            match decide(arm_row, arm, band) {
                Ok(Sign::Positive) => {}
                // Within the band of the normal: no ray in the plane.
                Ok(_) => return Err(RayFault::Unread),
                Err(diag) => {
                    return Err(RayFault::InBand(PointInLoopError::on_margin(
                        r#loop,
                        LoopDecision::Ray,
                    )(diag)));
                }
            }
            let d = d_raw.normalize();
            ray(d, normal.cross(d))
        },
        || PointInLoopError::RayExhausted { r#loop },
    )
}

/// The arc-bearing walk's K rows over a loop's STRAIGHT edges — the
/// boundary pre-pass and the straddle count, [`ParityRows`]' four —
/// kept apart from [`point_in_vertex_polygon`]'s so the two walks'
/// populations stay separable. Its own rows, written at their `decide` calls:
/// `point_in_arc_loop_arm` (the schedule gate), `point_in_arc_loop_reach`
/// (a spiric piece's ball's clearance from a ray), and the
/// conic arm's `point_in_arc_loop_conic_{span,on,window,disc,advance}`
/// and its pre-pass trim's `point_in_arc_loop_conic_{end,trim}`.
const ARC_LOOP_ROWS: ParityRows = ParityRows {
    segment: "point_in_arc_loop_segment",
    boundary: "point_in_arc_loop_boundary",
    side: "point_in_arc_loop_side",
    advance: "point_in_arc_loop_advance",
};

/// The rows a caller reads a conic edge under — its own, so each
/// caller's population stays separable in the telemetry: the arc's width
/// against a period (`turn`, [`ConicArc::of`]'s turn rule) and the name
/// an ellipse's span escalation carries where its two readings straddle
/// the band ([`geom_core::k_stats::straddle`]), which no margin of `turn`
/// says, so it is not charged to it.
pub(crate) type ConicRows = TurnRows<&'static str>;

/// The rows a caller reads a loop's boundary under: a straight edge's
/// ([`ray_walk::on_segment`]'s two), a conic edge's, a spiric edge's and
/// a spline edge's.
#[derive(Clone, Copy)]
pub(crate) struct BoundaryRows {
    /// A straight edge's rows.
    pub(crate) line: &'static ParityRows,
    /// A conic edge's rows.
    pub(crate) conic: ConicRows,
    /// A spiric edge's rows.
    pub(crate) spiric: ArcRows,
    /// A spline edge's rows.
    pub(crate) spline: ArcRows,
}

/// [`point_in_loop`]'s rows for a loop's boundary.
const WALK_ROWS: BoundaryRows = BoundaryRows {
    line: &ARC_LOOP_ROWS,
    conic: ConicRows::bounded(
        "point_in_arc_loop_conic_span",
        "point_in_arc_loop_conic_straddle",
    ),
    spiric: ArcRows {
        end: "point_in_arc_loop_spiric_end",
        clear: "point_in_arc_loop_spiric_clear",
        on: "point_in_arc_loop_spiric_on",
        leaf: "point_in_arc_loop_spiric_leaf",
    },
    spline: ArcRows {
        end: "point_in_arc_loop_spline_end",
        clear: "point_in_arc_loop_spline_clear",
        on: "point_in_arc_loop_spline_on",
        leaf: "point_in_arc_loop_spline_leaf",
    },
};

/// Which conic a [`ConicArc`] is. A circle's unit coordinates are an
/// isometry scaled by its radius, so one lever turns every
/// unit-coordinate reading into metres EXACTLY; an ellipse's are not,
/// and its boundary rows bound the metric on both sides instead.
#[derive(Clone, Copy, PartialEq, Eq)]
enum ConicKind {
    Circle,
    Ellipse,
}

/// A conic arc of a planar loop — a circle or an ellipse — in its own
/// affine frame: the locus `center + u·a·cos θ + v·b·sin θ` over
/// `θ ∈ [t0, t1]` (for an ellipse `θ` is the eccentric anomaly, the
/// carrier's own parameter). In UNIT coordinates `((x−c)·u/a,
/// (x−c)·v/b)` the conic is the unit circle and the arc a window of
/// it.
#[derive(Clone, Copy)]
pub(crate) struct ConicArc<T: geom_core::Real> {
    kind: ConicKind,
    center: Point3<T>,
    /// The conic's plane normal.
    axis: Vec3<T>,
    u: Vec3<T>,
    v: Vec3<T>,
    a: T,
    b: T,
    /// The smaller semi-axis, the fewest metres one unit-coordinate step
    /// buys. It levers the CROSSING rows, where a `Zero` grazes a ray
    /// and an understated margin costs rays, never an answer. A verdict
    /// on an ellipse reads it only where a LOWER bound is the sound side
    /// — the span rule's wound-past-period ([`Self::of`]); the boundary
    /// reading bounds its distances on both sides ([`Self::hit`]), and
    /// an arc's span needs the larger semi-axis too.
    lever: T,
    /// The arc's two end points, evaluated as certification evaluates
    /// them (`Curve3::eval`), so every reading of an end reads the point
    /// whose pin to its vertex was certified.
    ends: [Point3<T>; 2],
    /// The arc's two ends and its apex (the window's mid direction),
    /// on the unit circle — the trim [`arc_trim`] decides by distance.
    trim: [(T, T); 3],
    /// The span's turn ([`Self::of`]): short of a whole turn, a whole
    /// turn (over one never builds a `ConicArc`), or `Err` where it is
    /// unsettled with any overlap within the escalation band.
    span_turn: Result<Within, Indeterminate>,
    /// The arc's parameter interval on its carrier ([`Self::carrier`]).
    span: (T, T),
}

/// Why a conic carrier gives no [`ConicArc`].
pub(crate) enum ConicArcError {
    /// The window winds definitely past a period: no edge at all.
    WoundPastPeriod,
    /// An ellipse's overlap past a period is neither definitely within
    /// the band nor definitely past it ([`ConicArc::of`]'s span rule): in
    /// band on the smaller lever, or its two levers' readings straddling
    /// the band.
    Escalated(ReadEscalation),
}

/// Where a point sits against one conic edge.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum ConicHit {
    /// Definitely off the conic.
    Off,
    /// On the conic, definitely off the arc and clear of both its ends.
    Carrier,
    /// On the arc, definitely clear of both ends, by the margin that
    /// decided the point on the conic.
    On(MarginDiag),
    /// On the conic within the band of one of the arc's ends.
    End,
    /// Not placed: an ellipse's `Past` stands on the span walk
    /// ([`geom::Curve3::clear_of`]), which ran past its budget, or the
    /// carrier bounds no speed to walk by.
    Unsettled,
}

impl<T: Decide> ConicArc<T> {
    /// The arc a certified conic carrier spans over `(t0, t1)`, whose
    /// certified extent is `extent`; `None` for a carrier that is not a
    /// circle or an ellipse.
    ///
    /// **The span is read for "not over"**, through the turn rule
    /// ([`Period::turn`]): exact on a circle, between the smaller
    /// semi-axis and the speed over the turn's defect on an ellipse.
    ///
    /// - **Wound past a period** — no edge at all — where the rule reads
    ///   [`Turn::Over`].
    /// - **An arc** wherever the rule settles `Short` or `Whole`, and
    ///   wherever the sup reads the headroom definitely positive
    ///   ([`TurnUndecided::not_over_at_sup`], the one home of that
    ///   admission): the edge is then short of a turn in its parameter. A
    ///   gap, of any length, is an arc like any other: the distance trim
    ///   reads a whole circle, a near-whole one and a short one alike —
    ///   on a whole turn its two ends coincide at the edge's one vertex —
    ///   so no gap is read as closed.
    /// - **Any other undecided reading** — a circle's in-band `Δ·r`, an
    ///   ellipse whose sup reads the headroom in the band or definitely
    ///   negative — escalates, on the rule's row ([`TurnUndecided`]).
    ///   Reading it as an arc could put a stretch of the edge on both
    ///   sides of its own trim, and a point ON it would read off the
    ///   edge. A certified edge read at its certification's band reaches
    ///   neither: check 2 settled its turn on these readings.
    pub(crate) fn of(
        carrier: &geom::Curve3<T>,
        turn: Option<&geom_brep::CertifiedTurn<T>>,
        rows: ConicRows,
        band: Band,
    ) -> Result<Option<Self>, ConicArcError> {
        if !matches!(
            carrier,
            geom::Curve3::Circle { .. } | geom::Curve3::Ellipse { .. }
        ) {
            return Ok(None);
        }
        let Some(turn) = turn else {
            unreachable!("a certified circle or ellipse carries its turn (check 2)")
        };
        Self::read(carrier, turn.params(), turn.extent(), rows, band)
    }

    /// [`Self::of`] over an interval and its extent.
    fn read(
        carrier: &geom::Curve3<T>,
        (t0, t1): (T, T),
        extent: Extent<T>,
        rows: ConicRows,
        band: Band,
    ) -> Result<Option<Self>, ConicArcError> {
        let (kind, center, axis, u, a, b) = match *carrier {
            geom::Curve3::Circle {
                center,
                axis,
                radius,
                u_ref,
            } => (ConicKind::Circle, center, axis, u_ref, radius, radius),
            geom::Curve3::Ellipse {
                center,
                axis,
                semi_u,
                semi_v,
                u_ref,
            } => (ConicKind::Ellipse, center, axis, u_ref, semi_u, semi_v),
            _ => return Ok(None),
        };
        let lever = a.min(b);
        let one_turn = Period::new(T::tau());
        let Some(floor) = geom::carrier_speed::speed_floor(carrier) else {
            unreachable!("`speed_floor` answers a circle and an ellipse")
        };
        // A circle's two levers are one, its radius. An ellipse's are
        // certification's: the inf and the sup of the speed over the
        // turn's defect itself ([`geom::carrier_speed::turn_defect_rate`]),
        // bounds on the
        // speed anywhere on it and tight as it shrinks, so a certified
        // window (its ends pinned within the band of its vertex) is never
        // read as over-wound.
        let (turn, recording) = geom_core::k_stats::detached(|| match kind {
            ConicKind::Circle => one_turn.turn(
                TurnRows::exact(rows.turn),
                extent,
                Exact::new(floor.get()),
                band,
            ),
            ConicKind::Ellipse => {
                let rate = geom::carrier_speed::turn_defect_rate(carrier, t0, extent);
                one_turn.turn(rows, extent, rate, band)
            }
        });
        // An arc kept on an unsettled turn rests on no escalation: its
        // readings are superseded by the arc (`k_stats::splice_superseded`),
        // and a scaffold that does read the turn records it itself
        // (`carrier_loop`).
        match turn {
            Err(undecided) if undecided.not_over_at_sup() => {
                geom_core::k_stats::splice_superseded(recording);
            }
            _ => geom_core::k_stats::splice(recording),
        }
        let span_turn = match turn {
            Ok(Turn::Over(_)) => return Err(ConicArcError::WoundPastPeriod),
            Ok(Turn::Short(_)) => Ok(Within::Short),
            Ok(Turn::Whole(_)) => Ok(Within::Whole),
            Err(undecided @ TurnUndecided { diag, on, .. }) if !undecided.not_over_at_sup() => {
                return Err(ConicArcError::Escalated(ReadEscalation {
                    escalation: match on {
                        TurnRow::Turn => Escalation::Margin,
                        TurnRow::Straddle => Escalation::Straddle,
                    },
                    diag,
                }));
            }
            Err(TurnUndecided { diag, .. }) => Err(diag),
        };
        let (s0, c0) = t0.sin_cos();
        let (s1, c1) = t1.sin_cos();
        let (sm, cm) = geom::mid_param(t0, t1).sin_cos();
        Ok(Some(Self {
            kind,
            center,
            axis,
            u,
            v: axis.cross(u),
            a,
            b,
            lever,
            ends: [carrier.eval(t0), carrier.eval(t1)],
            trim: [(c0, s0), (c1, s1), (cm, sm)],
            span_turn,
            span: (t0, t1),
        }))
    }

    /// The carrier, rebuilt from the arc's frame as [`Self::of`] read it.
    fn carrier(&self) -> geom::Curve3<T> {
        match self.kind {
            ConicKind::Circle => geom::Curve3::Circle {
                center: self.center,
                axis: self.axis,
                radius: self.a,
                u_ref: self.u,
            },
            ConicKind::Ellipse => geom::Curve3::Ellipse {
                center: self.center,
                axis: self.axis,
                semi_u: self.a,
                semi_v: self.b,
                u_ref: self.u,
            },
        }
    }

    fn unit(&self, x: Point3<T>) -> (T, T) {
        let w = x - self.center;
        (w.dot(self.u) / self.a, w.dot(self.v) / self.b)
    }

    /// The carrier's point at parameter `t`, by the conic's own frame:
    /// the probes' placement (the ends are [`Self::ends`]).
    #[cfg(test)]
    fn point(&self, t: T) -> Point3<T> {
        let (s, c) = t.sin_cos();
        self.center + self.u * (self.a * c) + self.v * (self.b * s)
    }

    /// A unit-circle point as a [`Point3`] for [`arc_trim`], which reads
    /// points; the third coordinate is zero, so every distance is the
    /// plane's own.
    fn lift((x, y): (T, T)) -> Point3<T> {
        Point3::new(x, y, T::zero())
    }

    /// The arc's ends, apex and the apex's antipode, lifted.
    fn trim_points(&self) -> ([Point3<T>; 2], Point3<T>, Point3<T>) {
        let [e0, e1, m] = self.trim;
        let z = T::zero();
        (
            [Self::lift(e0), Self::lift(e1)],
            Self::lift(m),
            Self::lift((z - m.0, z - m.1)),
        )
    }

    /// **Where `q` sits against this edge**: the arc's span read by
    /// [`geom_brep::curve_pair::Span::read`], the one composition of the
    /// carrier's reading at `q`'s foot ([`geom::Foot::on_carrier`], off
    /// only on a lower bound on its distance from the whole conic, the
    /// ellipse's Hessian-bounded root) with the span's reading of its ends
    /// ([`geom_brep::curve_pair::Span::place`]). A point past the arc is
    /// off the edge, on its carrier ([`ConicHit::Carrier`]) or not; the
    /// ellipse's straddle stays a straddle, and a span walk that cannot
    /// certify its `Past` is [`ConicHit::Unsettled`].
    pub(crate) fn hit(&self, q: Point3<T>, band: Band) -> Result<ConicHit, ReadEscalation> {
        use geom_brep::curve_pair::{Reading, Span};
        let carrier = self.carrier();
        let span = Span::new(&carrier, self.span);
        let near = geom::mid_param(self.span.0, self.span.1);
        Ok(match span.read(q, near, band) {
            Ok(Reading::Off) => ConicHit::Off,
            Ok(Reading::Carrier) => ConicHit::Carrier,
            Ok(Reading::On { margin, .. }) => ConicHit::On(margin),
            Ok(Reading::End(_)) => ConicHit::End,
            Err(geom::CarrierRefusal::Escalated(diag)) => return Err(diag.into()),
            // The ellipse's two bounds on its distance, which no margin
            // says: a straddle, typed.
            Err(geom::CarrierRefusal::Straddle(diag)) => {
                return Err(ReadEscalation {
                    escalation: Escalation::Straddle,
                    diag,
                });
            }
            // An ellipse's `Past` stands on the span walk, which can run
            // past its budget: not placed, typed.
            Err(geom::CarrierRefusal::Exhausted | geom::CarrierRefusal::Unbounded) => {
                ConicHit::Unsettled
            }
        })
    }

    /// Is the unit-circle point `(x, y)` inside the arc's window?
    /// [`arc_trim`] in the arc's unit coordinates, levered by the
    /// smaller semi-axis: Positive inside, Negative outside, Zero an
    /// endpoint's neighbourhood. Only a ray's crossing reads it, where a
    /// `Zero` grazes the ray and an in-band margin sets it aside, so the lever's
    /// understatement on an ellipse costs rays, never a count.
    fn in_window(&self, (x, y): (T, T), band: Band) -> Result<Sign, Indeterminate> {
        let (ends, apex, anti) = self.trim_points();
        arc_trim(
            Self::lift((x, y)),
            ends,
            apex,
            anti,
            self.lever,
            &ARC_LOOP_TRIM,
            band,
        )
    }
}

/// The K rows one [`arc_trim`] consumer decides on.
pub(crate) struct ArcTrimRows {
    /// A point's distance to either end of the arc.
    pub(crate) end: &'static str,
    /// The two-term chordal margin: which side of the ends.
    pub(crate) trim: &'static str,
}

/// The arc-bearing walk's trim rows for a ray's CROSSING of an arc,
/// where a `Zero` grazes the ray.
const ARC_LOOP_TRIM: ArcTrimRows = ArcTrimRows {
    end: "point_in_arc_loop_conic_end",
    trim: "point_in_arc_loop_conic_window",
};

/// The distance from `p` to the circle (`center`, unit `axis`,
/// `radius`): `√(h² + (ρ − r)²)`, `h` its height over the circle's plane
/// and `ρ` its distance from the axis. The one spelling of a point's
/// miss from a circle, for the loop walk here and the boolean's contact
/// and on-carrier rows.
pub(crate) fn circle_miss<T: Decide>(
    p: Point3<T>,
    center: Point3<T>,
    axis: Vec3<T>,
    radius: T,
) -> T {
    let off = p - center;
    let h = off.dot(axis);
    let rho = (off - axis * h).norm();
    (h.powi(2) + (rho - radius).powi(2)).sqrt()
}

/// **Whether `p`, a point on a circle, lies inside the arc of it whose
/// ends are `ends` and whose apex is `apex`** (`anti` the apex's
/// antipode) — decided as DISTANCES, never as an angle, and the one
/// home of that rule: tier 3's check 9 trims an edge with it and the
/// arc-bearing walk here trims a crossing with it. Every length is
/// multiplied by `lever` on its way to a margin: `1` for points given
/// in metres, the smaller semi-axis for points given in a conic's unit
/// coordinates (exact for a circle, conservative for an ellipse).
///
/// 1. **At an end**: `p` within the band of either end, measured as
///    `|p − end|`, answers `Zero`. That is the ONLY way an endpoint
///    neighbourhood counts: an angular window compresses arc length
///    near an end by `sin(w/2)`, so on a short arc (and by `sin` of the
///    complement on a near-full one) its `Zero` reaches `ε / sin(w/2)`
///    along the carrier, a hundred times `ε` at `w = 0.02`.
/// 2. **Otherwise, which side of the ends**: the sum of two chordal
///    defects, `(|a − m| − |p − m|) + (|p − m′| − |a − m′|)`, where `a`
///    is an end, `m` the apex and `m′` its antipode. Chord length is
///    monotone in angular distance up to a half turn, so each term is
///    positive exactly on the arc; near an end they move as `cos(w/4)`
///    and `sin(w/4)` times the arc length, whose sum is at least 1, so
///    the margin is never compressed below the distance it measures —
///    on a short arc, a near-full one, or a whole circle (where `m′` is
///    the end and every point is inside). Its `Zero` is therefore within
///    the band of an end, which step 1 has already answered.
///
/// `Positive` inside, `Negative` past an end, `Zero` at an end or in
/// the trim's band.
///
/// **A DECIDED membership, and so not
/// [`geom::periodic_window_may_hold`]'s question** — this paragraph is
/// the one place that says why, for this trim. That door answers "may hold" wherever
/// exclusion is unproved, which is the sound answer for a caller
/// selecting between two bounds that each hold either way. A decided
/// membership has no such free answer: an uncertain one escalates, and
/// its band is a length — measured along the carrier here, levered by
/// the radius there — never a bare angle.
///
/// # Errors
///
/// An in-band end distance with neither end definitely `Zero`, or an
/// in-band trim margin.
pub(crate) fn arc_trim<T: Decide>(
    p: Point3<T>,
    ends: [Point3<T>; 2],
    apex: Point3<T>,
    anti: Point3<T>,
    lever: T,
    rows: &ArcTrimRows,
    band: Band,
) -> Result<Sign, Indeterminate> {
    let [a, b] = ends;
    let at = [
        decide(rows.end, Margin::levered((p - a).norm(), lever), band),
        decide(rows.end, Margin::levered((p - b).norm(), lever), band),
    ];
    if at.iter().any(|e| matches!(e, Ok(Sign::Zero))) {
        return Ok(Sign::Zero);
    }
    if let Some(diag) = at.into_iter().find_map(Result::err) {
        return Err(diag);
    }
    decide(
        rows.trim,
        Margin::levered(arc_trim_margin(p, a, apex, anti), lever),
        band,
    )
}

/// [`arc_trim`]'s step 2 alone — the sum of two chordal defects,
/// `(|a − m| − |p − m|) + (|p − m′| − |a − m′|)`, positive on the arc and
/// never compressed below the angle to the nearer end — for a caller that
/// has decided the ends by another metric first (an ellipse's exact end
/// distances, [`ConicArc::hit`]). The one body of the construction.
pub(crate) fn arc_trim_margin<T: Decide>(
    p: Point3<T>,
    a: Point3<T>,
    apex: Point3<T>,
    anti: Point3<T>,
) -> T {
    ((a - apex).norm() - (p - apex).norm()) + ((p - anti).norm() - (a - anti).norm())
}

/// One boundary edge of a planar loop, on its own carrier.
pub(crate) enum LoopEdge<T: geom_core::Real> {
    /// A line — which IS its chord — or null scaffolding, a zero-length
    /// coincident copy.
    Chord,
    /// A circle or ellipse arc.
    Conic(ConicArc<T>),
    /// An arc of a spiric oval.
    Spiric(SpiricArc<T>),
    /// An arc of a spline.
    Spline(SplineArc<T>),
}

/// An [`EdgeContact`] that places the point off the edge: what a caller's
/// boundary pass hands [`carrier_loop_side`] for each edge.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum OffEdge {
    /// [`EdgeContact::Off`].
    Off,
    /// [`EdgeContact::Carrier`].
    Carrier,
    /// [`EdgeContact::Unread`].
    Unread,
}

/// An [`EdgeContact`] that places the point on the edge or at one of its
/// ends: what stops a boundary pass.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum OnEdge {
    /// [`EdgeContact::On`].
    On(MarginDiag),
    /// [`EdgeContact::End`].
    End,
}

impl EdgeContact {
    /// This reading split by whether it places the point off the edge:
    /// [`OffEdge`] is what a walk behind a boundary pass still needs of
    /// it, [`OnEdge`] what ends that pass.
    pub(crate) fn off(self) -> Result<OffEdge, OnEdge> {
        match self {
            Self::Off => Ok(OffEdge::Off),
            Self::Carrier => Ok(OffEdge::Carrier),
            Self::Unread => Ok(OffEdge::Unread),
            Self::On(margin) => Err(OnEdge::On(margin)),
            Self::End => Err(OnEdge::End),
        }
    }
}

/// Where a point sits against one boundary edge — [`LoopEdge::contact`].
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum EdgeContact {
    /// Definitely off the edge.
    Off,
    /// On a conic's carrier, definitely off the arc and clear of its
    /// ends — off the edge, and a fact the crossing count needs.
    Carrier,
    /// On the edge, clear of a conic's ends, by the margin that decided
    /// the point on it.
    On(MarginDiag),
    /// Within the band of one of a curved edge's two carrier ends.
    End,
    /// The reading could not place the point: a spiric's or a spline's
    /// whose pieces ran out of depth or budget, which the region walk
    /// refuses on ([`Uncrossable`]); or a conic's that did not settle,
    /// which it does not, since a conic's crossing count reads its own
    /// carrier.
    Unread,
}

impl<T: Decide> LoopEdge<T> {
    /// **The one boundary reading of an edge** `a → b`, shared by every
    /// point-in-loop door: a straight edge by the distance to its closed
    /// segment ([`ray_walk::on_segment`]), a conic by
    /// [`ConicArc::hit`], a spiric or a spline by
    /// [`PiecedArc::contact`].
    pub(crate) fn contact(
        &self,
        (a, b): (Point3<T>, Point3<T>),
        q: Point3<T>,
        rows: BoundaryRows,
        band: Band,
    ) -> Result<EdgeContact, ReadEscalation> {
        Ok(match self {
            Self::Chord => {
                match ray_walk::on_segment_margin(a, b, q, &rows.line.segment_rows(), band)? {
                    Some(on) => EdgeContact::On(on),
                    None => EdgeContact::Off,
                }
            }
            Self::Conic(k) => match k.hit(q, band)? {
                ConicHit::Off => EdgeContact::Off,
                ConicHit::Carrier => EdgeContact::Carrier,
                ConicHit::On(on) => EdgeContact::On(on),
                ConicHit::End => EdgeContact::End,
                ConicHit::Unsettled => EdgeContact::Unread,
            },
            Self::Spiric(k) => pieced_contact(k.contact(q, rows.spiric, band))?,
            Self::Spline(k) => pieced_contact(k.contact(q, rows.spline, band))?,
        })
    }

    /// A curved edge's two carrier end points (at its start and end
    /// parameters).
    pub(crate) fn carrier_ends(&self) -> Option<[Point3<T>; 2]> {
        match self {
            Self::Conic(k) => Some(k.ends),
            Self::Spiric(k) => Some(k.ends()),
            Self::Spline(k) => Some(k.ends()),
            Self::Chord => None,
        }
    }
}

/// A pieced arc's boundary reading as an edge's. A reading that ran out
/// cannot place the point, which the region walk refuses on.
fn pieced_contact(hit: Result<ArcHit, ArcRefusal>) -> Result<EdgeContact, ReadEscalation> {
    match hit {
        Ok(ArcHit::Off) => Ok(EdgeContact::Off),
        Ok(ArcHit::On(on)) => Ok(EdgeContact::On(on)),
        Ok(ArcHit::End) => Ok(EdgeContact::End),
        Err(ArcRefusal::Unsettled) => Ok(EdgeContact::Unread),
        Err(ArcRefusal::Escalated(diag)) => Err(diag.into()),
    }
}

/// A loop whose every edge [`LoopEdge::contact`] placed one point off,
/// with those readings in edge order: built only by
/// [`CarrierLoop::place_off`], so each edge has exactly one.
pub(crate) struct OffLoop<T: geom_core::Real> {
    lp: CarrierLoop<T>,
    off: Vec<OffEdge>,
}

/// The first edge [`CarrierLoop::place_off`] found the point on, and the
/// loop it read.
pub(crate) struct OnLoop<T: geom_core::Real> {
    /// The loop.
    pub(crate) lp: CarrierLoop<T>,
    /// The edge's index in `lp`.
    pub(crate) edge: usize,
    /// Where on it the point lies.
    pub(crate) on: OnEdge,
}

impl<T: Decide> CarrierLoop<T> {
    /// **The boundary pass**: [`LoopEdge::contact`] of `q` against each
    /// edge in turn, under `rows`, to the first that places it on the
    /// edge or at an end.
    ///
    /// # Errors
    ///
    /// The first edge's escalation.
    pub(crate) fn place_off(
        self,
        q: Point3<T>,
        rows: BoundaryRows,
        band: Band,
    ) -> Result<Result<OffLoop<T>, OnLoop<T>>, ReadEscalation> {
        let n = self.verts.len();
        let mut off = Vec::with_capacity(n);
        let mut hit = None;
        for (i, edge) in self.edges.iter().enumerate() {
            match edge
                .contact((self.verts[i], self.verts[(i + 1) % n]), q, rows, band)?
                .off()
            {
                Ok(reading) => off.push(reading),
                Err(on) => {
                    hit = Some((i, on));
                    break;
                }
            }
        }
        Ok(match hit {
            None => Ok(OffLoop { lp: self, off }),
            Some((edge, on)) => Err(OnLoop { lp: self, edge, on }),
        })
    }
}

/// A loop in cycle order: each vertex, and each edge (vertex `i` to
/// `i + 1`) with its key and on its own carrier.
pub(crate) struct CarrierLoop<T: geom_core::Real> {
    /// The vertices' points.
    pub(crate) verts: Vec<Point3<T>>,
    /// The edges' keys.
    pub(crate) keys: Vec<EdgeKey>,
    /// The edges on their carriers.
    pub(crate) edges: Vec<LoopEdge<T>>,
    /// The balls holding each curved edge's carrier arc ([`step_ball`]),
    /// which with `verts` hold the whole loop ([`extent_from`]).
    balls: Vec<ArcBalls<T>>,
}

/// One step of a loop's cycle, read with no decision: its start
/// vertex's point, its edge, and the edge's certified curve (`None` for
/// null scaffolding, a zero-length chord its vertex holds).
struct LoopStep<'b, T: geom_core::Real> {
    point: Point3<T>,
    edge: EdgeKey,
    curve: Option<&'b geom_brep::EdgeCurve<T>>,
}

/// The steps of a loop's cycle from its half-edge `first`, which the
/// caller read out of the loop: every hop is a link, and a miss panics.
fn cycle_steps<T: Decide>(
    body: &Body<T>,
    first: crate::entity::HalfEdgeKey,
) -> Vec<LoopStep<'_, T>> {
    let mut steps = Vec::new();
    for he in body.loop_walk(first).closed("loop", first) {
        let h = proven(&body.half_edges, he, EntityId::HalfEdge);
        let edge = linked(
            &body.edges,
            h.edge,
            EntityId::Edge,
            EntityId::HalfEdge(he),
            "edge",
        );
        // A null-scaffolding curve is a zero-length chord.
        steps.push(LoopStep {
            point: body.linked_vertex_point(h.start, EntityId::HalfEdge(he), "start"),
            edge: h.edge,
            curve: body.edge_curve_linked(h.edge, edge).certified(),
        });
    }
    steps
}

/// **Two balls, each holding one curved edge's arc**: a reader of the
/// edge's extent takes the nearer of them, and a reader of its side of
/// a plane the clearer. A conic's are its whole carrier's
/// ([`carrier_ball`]) and its arc's own ([`conic_arc_ball`]): neither
/// holds the other, since a short arc at the end of a long ellipse's
/// major axis lies far outside the ball of its own size about the
/// centre, and the carrier's ball reaches the far end of the ellipse.
/// Any other carrier's are its one ball, twice.
pub(crate) type ArcBalls<T> = [(Point3<T>, T); 2];

/// The balls holding a step's edge beyond its end vertices
/// ([`ArcBalls`]): `None` for a chord.
///
/// # Errors
///
/// [`PointInLoopError::CorruptLoop`] for a spline edge with no control
/// points.
fn step_ball<T: Decide>(
    r#loop: LoopKey,
    step: &LoopStep<'_, T>,
) -> Result<Option<ArcBalls<T>>, PointInLoopError> {
    let Some(curve) = step.curve else {
        return Ok(None);
    };
    match carrier_ball(curve.carrier(), curve.params()) {
        Some(ball) => Ok(Some([
            ball,
            conic_arc_ball(curve.carrier(), curve.params()).unwrap_or(ball),
        ])),
        None if matches!(curve.carrier(), geom::Curve3::Line { .. }) => Ok(None),
        None => Err(PointInLoopError::CorruptLoop { r#loop }),
    }
}

/// The loop's vertices and its edges on their carriers, a conic's span
/// read on `conic`, the only rows building the loop reads.
///
/// # Errors
///
/// [`PointInLoopError::CorruptLoop`] for a loop that does not walk, a
/// conic wound past a period, or a whole-turn SCAFFOLD conic;
/// [`PointInLoopError::Escalated`] for an ellipse's span the band cannot
/// place, or a scaffold conic's span in the band.
pub(crate) fn carrier_loop<T: Decide>(
    body: &Body<T>,
    r#loop: LoopKey,
    conic: ConicRows,
    band: Band,
) -> Result<CarrierLoop<T>, PointInLoopError> {
    let corrupt = || PointInLoopError::CorruptLoop { r#loop };
    let LoopBoundary::Cycle { first } = body.get_loop(r#loop).ok_or_else(corrupt)?.boundary else {
        return Err(corrupt());
    };
    let steps = cycle_steps(body, first);
    let mut verts = Vec::new();
    let mut keys = Vec::new();
    let mut edges = Vec::new();
    let mut balls = Vec::new();
    for step in &steps {
        verts.push(step.point);
        keys.push(step.edge);
        balls.extend(step_ball(r#loop, step)?);
        let Some(curve) = step.curve else {
            edges.push(LoopEdge::Chord);
            continue;
        };
        match ConicArc::of(curve.carrier(), curve.turn(), conic, band) {
            // A whole-turn scaffold circle is how
            // `EdgeCurveSpec::self_loop_circle_at` certifies a NULL edge:
            // a deliberately arbitrary unit circle, which the conic row
            // would cross as a real one. Nothing in the edge tells that
            // placeholder from an honest whole-turn scaffold, so the loop
            // is not read; and a scaffold whose span is in the band is
            // not known to be either, so it escalates on that span.
            Ok(Some(k)) if curve.description().is_scaffold() => match k.span_turn {
                Ok(Within::Short) => edges.push(LoopEdge::Conic(k)),
                Ok(Within::Whole) => return Err(corrupt()),
                Err(diag) => {
                    // The arc set this escalation aside (`ConicArc::read`);
                    // the scaffold's refusal rests on it.
                    geom_core::k_stats::relog(&geom_core::k_stats::Recorded {
                        verdicts: Vec::new(),
                        escalations: vec![geom_core::k_stats::Escalation { source: diag }],
                    });
                    return Err(PointInLoopError::on_margin(r#loop, LoopDecision::ArcSpan)(
                        diag,
                    ));
                }
            },
            Ok(Some(k)) => edges.push(LoopEdge::Conic(k)),
            Ok(None) => edges.push(match *curve.carrier() {
                geom::Curve3::Circle { .. } | geom::Curve3::Ellipse { .. } => {
                    unreachable!("a circle or an ellipse is a conic arc above")
                }
                // A line: its segment is its two end vertices'.
                geom::Curve3::Line { .. } => LoopEdge::Chord,
                geom::Curve3::Spiric {
                    center,
                    axis,
                    u_ref,
                    major_radius,
                    minor_radius,
                    offset,
                } => LoopEdge::Spiric(SpiricArc::new(
                    Oval {
                        center,
                        axis,
                        u_ref,
                        major: major_radius,
                        minor: minor_radius,
                        offset,
                    },
                    curve.params(),
                )),
                // A certified edge's span is forward, between point
                // parameters.
                geom::Curve3::Nurbs(ref n) => {
                    LoopEdge::Spline(SplineArc::new(n, curve.params()).ok_or_else(corrupt)?)
                }
            }),
            Err(ConicArcError::WoundPastPeriod) => return Err(corrupt()),
            Err(ConicArcError::Escalated(e)) => {
                return Err(PointInLoopError::on(r#loop, LoopDecision::ArcSpan)(e));
            }
        }
    }
    Ok(CarrierLoop {
        verts,
        keys,
        edges,
        balls,
    })
}

/// **A ball holding the arc `span` of `carrier`**, `(center, radius)`,
/// read off the carrier's own data with no decision: a conic within its
/// larger semi-axis of its centre, a spiric arc within its speed bound
/// times its half-width of its midpoint, a spline within its control
/// hull's ball. `None` for a line, whose segment
/// its two end vertices hold, and for a spline with no control points.
pub(crate) fn carrier_ball<T: Decide>(
    carrier: &geom::Curve3<T>,
    (t0, t1): (T, T),
) -> Option<(Point3<T>, T)> {
    match *carrier {
        geom::Curve3::Line { .. } => None,
        geom::Curve3::Circle { center, radius, .. } => Some((center, radius)),
        geom::Curve3::Ellipse {
            center,
            semi_u,
            semi_v,
            ..
        } => Some((center, geom::Curve3::semi_axes_by_size(semi_u, semi_v).0)),
        // The arc from its midpoint: no point of it lies further from
        // `P(mid)` than the oval's speed bound times half the parameter
        // width.
        geom::Curve3::Spiric { .. } => Some((
            carrier.mid_point(t0, t1),
            geom_core::bisect::reach(geom::carrier_speed::speed_range(carrier).1, (t0, t1)),
        )),
        // Positive weights put a NURBS curve inside its control
        // hull, so inside any ball holding every control point: the
        // one about the control points' bounding-box centre, to the
        // farthest of them.
        geom::Curve3::Nurbs(ref n) => {
            let control = n.control();
            let first = *control.first()?;
            let (mut lo, mut hi) = (first, first);
            for p in control {
                lo = Point3::new(lo.x.min(p.x), lo.y.min(p.y), lo.z.min(p.z));
                hi = Point3::new(hi.x.max(p.x), hi.y.max(p.y), hi.z.max(p.z));
            }
            let center = lo + (hi - lo) * T::from_f64(0.5);
            let mut reach = T::zero();
            for p in control {
                reach = reach.max((*p - center).norm());
            }
            Some((center, reach))
        }
    }
}

/// **A ball holding the arc `span` of a conic `carrier`, of the arc's
/// own size**: about the arc's middle point, to half the span's width
/// times a bound on the speed over it ([`geom::carrier_speed::SemiAxes::speed_over`]),
/// widened by the rounding of that middle point at the coordinates the
/// carrier is placed at. `None` for any carrier but a circle or an
/// ellipse.
pub(crate) fn conic_arc_ball<T: Decide>(
    carrier: &geom::Curve3<T>,
    (t0, t1): (T, T),
) -> Option<(Point3<T>, T)> {
    let (center, axes) = match *carrier {
        geom::Curve3::Circle { center, radius, .. } => {
            (center, geom::carrier_speed::SemiAxes::new(radius, radius))
        }
        geom::Curve3::Ellipse {
            center,
            semi_u,
            semi_v,
            ..
        } => (center, geom::carrier_speed::SemiAxes::new(semi_u, semi_v)),
        _ => return None,
    };
    let speed = geom_core::SupSpeed::new(axes.speed_over((t0, t1)));
    let placed = geom_core::rounding_charge((center - Point3::origin()).norm() + axes.speed_hi());
    Some((
        carrier.mid_point(t0, t1),
        geom_core::bisect::reach(speed, (t0, t1)) + placed,
    ))
}

/// **A lower bound on the distance from `q` to the arc `span` of
/// `carrier`**, read on the carrier with no decision, and the reading
/// that gave it ([`ArcFloor`]): a line's own segment; a circle's or an
/// ellipse's own arc (`ring_path::Conic::distance_floor`, the arc of
/// the circle over the same span less the conic's distance from it);
/// any other carrier's ball ([`carrier_ball`]), and its arc's pieces
/// past it ([`ArcPieces`]). A conic carries its whole annulus's floor
/// beside its arc's. `None` for a spline with no control points. The one home of the question "is this edge
/// clear of that point by more than a radius", which the clearance
/// cell door and the boolean's touch gate both ask.
pub(crate) fn arc_distance_floor<T: Decide>(
    carrier: &geom::Curve3<T>,
    (t0, t1): (T, T),
    q: Point3<T>,
) -> Option<ArcFloor<'_, T>> {
    Some(match *carrier {
        geom::Curve3::Line { .. } => ArcFloor::Segment(
            crate::sector_shape::point_segment_distance(carrier.eval(t0), carrier.eval(t1), q),
        ),
        geom::Curve3::Circle { center, axis, .. } | geom::Curve3::Ellipse { center, axis, .. } => {
            let Some(crate::ring_path::LoopArc::Conic(conic)) =
                crate::ring_path::LoopArc::of(carrier, (t0, t1))
            else {
                unreachable!("a circle or an ellipse is a conic arc")
            };
            // The whole conic lies in the annulus of its plane between
            // its semi-axes. That floor is exact where the arc's is not:
            // near the centre, where `q`'s direction is undecided.
            let (major, minor) = match *carrier {
                geom::Curve3::Ellipse { semi_u, semi_v, .. } => {
                    geom::Curve3::semi_axes_by_size(semi_u, semi_v)
                }
                geom::Curve3::Circle { radius, .. } => (radius, radius),
                _ => unreachable!("a circle or an ellipse"),
            };
            let n = axis / axis.norm();
            let w = q - center;
            let height = w.dot(n);
            let across = (w - n * height).norm();
            let off = (minor - across).max(across - major).max(T::zero());
            ArcFloor::Conic {
                annulus: (height.powi(2) + off.powi(2)).sqrt(),
                arc: conic.distance_floor(q),
            }
        }
        geom::Curve3::Spiric { .. } | geom::Curve3::Nurbs(_) => {
            let (center, reach) = carrier_ball(carrier, (t0, t1))?;
            ArcFloor::Ball {
                ball: ((q - center).norm() - reach).max(T::zero()),
                pieces: ArcPieces {
                    carrier,
                    span: (t0, t1),
                    q,
                },
            }
        }
    })
}

/// Which reading of a carrier [`arc_distance_floor`] took, with its
/// bounds: each is its own question, so a caller deciding on them keeps
/// them apart.
#[derive(Clone, Copy, Debug)]
pub(crate) enum ArcFloor<'c, T: geom_core::Real> {
    /// A line's distance to its segment.
    Segment(T),
    /// A conic's two floors: its whole annulus's, exact near its centre
    /// and far off it, and its arc's, which reads the trim.
    Conic {
        /// The annulus between the semi-axes, in the conic's plane.
        annulus: T,
        /// The arc's own floor.
        arc: T,
    },
    /// A spiric's or a spline's: the distance to a ball holding the
    /// arc, and the arc's pieces, read when the ball is not clear.
    Ball {
        /// The distance to the ball.
        ball: T,
        /// The arc, to read piece by piece.
        pieces: ArcPieces<'c, T>,
    },
}

impl<T: Decide> ArcFloor<'_, T> {
    /// Whether `clear` decides any of the floors clear: either of a
    /// conic's two, the annulus first; a ball's, or else every piece of
    /// its arc's.
    pub(crate) fn clears(self, clear: impl Fn(T) -> bool) -> bool {
        match self {
            Self::Segment(d) => clear(d),
            Self::Conic { annulus, arc } => clear(annulus) || clear(arc),
            Self::Ball { ball, pieces } => clear(ball) || pieces.clear(clear),
        }
    }
}

/// **An arc read piece by piece against a point `q`.** A piece `(a, b)`
/// lies within [`geom_core::bisect::reach`] `ℓ` of its middle point `m`
/// at the carrier's speed bound ([`geom::carrier_speed::speed_range`]),
/// so `|q − m| − ℓ` is a floor on its distance from `q`. A piece whose
/// floor is not clear is halved; one whose middle point is not clear
/// answers that the arc is not, since no piece holding that point
/// clears.
#[derive(Clone, Copy, Debug)]
pub(crate) struct ArcPieces<'c, T: geom_core::Real> {
    carrier: &'c geom::Curve3<T>,
    span: (T, T),
    q: Point3<T>,
}

impl<T: Decide> ArcPieces<'_, T> {
    /// Whether `clear` decides every piece's floor clear on the
    /// carrier's point walk ([`geom::Curve3::point_walk`]), within
    /// [`geom_core::bisect::PIECE_BUDGET`].
    fn clear(self, clear: impl Fn(T) -> bool) -> bool {
        let hi = geom::carrier_speed::speed_range(self.carrier).1;
        let walk = self.carrier.point_walk(self.q, self.span, hi, |p| {
            if !clear(p.gap) {
                return geom::PieceStep::Stop(());
            }
            if clear(p.gap - p.reach) {
                return geom::PieceStep::Settled;
            }
            geom::PieceStep::Halve
        });
        matches!(walk, Ok(None))
    }
}

/// **How far from `q` the loop reaches**: the radius of a ball about
/// `q` holding every vertex of `loop` and every edge's whole carrier
/// arc ([`carrier_ball`]). It asks no decision, so it cannot refuse on
/// geometry; it over-estimates (a conic's ball is its carrier's, not its
/// arc's), which is the direction its callers need.
///
/// # Errors
///
/// [`PointInLoopError::CorruptLoop`] for a loop that does not walk, a
/// spline edge with no control points, or a poisoned extent.
pub(crate) fn loop_extent_from<T: Decide>(
    body: &Body<T>,
    r#loop: LoopKey,
    q: Point3<T>,
) -> Result<T, PointInLoopError> {
    let (verts, balls) = loop_hull(body, r#loop)?;
    extent_from(r#loop, q, &verts, &balls)
}

/// The points and balls [`extent_from`] folds for `loop`, read with no
/// decision: every vertex, and each curved edge's [`step_ball`].
#[allow(clippy::type_complexity)] // two lists, each named at its reader
pub(crate) fn loop_hull<T: Decide>(
    body: &Body<T>,
    r#loop: LoopKey,
) -> Result<(Vec<Point3<T>>, Vec<ArcBalls<T>>), PointInLoopError> {
    let corrupt = || PointInLoopError::CorruptLoop { r#loop };
    let first = match body.get_loop(r#loop).ok_or_else(corrupt)?.boundary {
        LoopBoundary::Cycle { first } => first,
        LoopBoundary::Empty { vertex } => {
            let point = body.linked_vertex_point(vertex, EntityId::Loop(r#loop), "boundary");
            return Ok((vec![point], Vec::new()));
        }
    };
    let steps = cycle_steps(body, first);
    let mut balls = Vec::new();
    for step in &steps {
        balls.extend(step_ball(r#loop, step)?);
    }
    Ok((steps.iter().map(|s| s.point).collect(), balls))
}

/// **The radius of a ball about `from` holding a loop**, given its
/// vertices and the balls holding each curved edge ([`ArcBalls`]):
/// `|v − from|`, and for each edge the nearer of its balls'
/// `|center − from| + radius`, folded by their maximum. A poisoned term
/// is refused rather than folded, so no lever built on the extent
/// carries it on.
///
/// # Errors
///
/// [`PointInLoopError::CorruptLoop`] for a poisoned term.
fn extent_from<T: Decide>(
    r#loop: LoopKey,
    from: Point3<T>,
    verts: &[Point3<T>],
    balls: &[ArcBalls<T>],
) -> Result<T, PointInLoopError> {
    let poisoned = |term: T| match term.is_poison() {
        true => Err(PointInLoopError::CorruptLoop { r#loop }),
        false => Ok(term),
    };
    let mut extent = T::zero();
    for v in verts {
        extent = extent.max(poisoned((*v - from).norm())?);
    }
    for pair in balls {
        let [near, far] = pair.map(|(center, radius)| (center - from).norm() + radius);
        extent = extent.max(poisoned(near)?.min(poisoned(far)?));
    }
    Ok(extent)
}

/// Whether the walk reads the boundary itself, or its caller has.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Boundary<'a> {
    /// Answer `OnBoundary` where the point is on an edge.
    Verdict,
    /// The caller's boundary pass ([`CarrierLoop::place_off`]) placed the
    /// point off every edge of the loop, one reading per edge in edge
    /// order; the walk takes those readings rather than deciding them
    /// again.
    Decided(&'a [OffEdge]),
}

/// **Trilean containment of `q` in a planar loop's region, read on its
/// edges' own carriers.** The polygon through the vertices is the
/// region only when every edge is a line. An arc moves region across
/// its chord: a revolved cap is a half-disc whose three vertices are
/// COLLINEAR, an extruded disc's cap has two, and in both the polygon
/// has no area; an arc bowing into a polygon puts region the loop does
/// not bound inside it, and one bowing out leaves region outside it.
///
/// - **Every edge a line**: ray parity over the polygon through the
///   vertices (`point_in_vertex_polygon`).
/// - **Circle and ellipse arcs** (with lines): ray parity over the same
///   schedule (`walk_schedule`), the straight edges counted by
///   `ray_walk::ray_crossings` and each arc crossed on its conic —
///   in the arc's unit coordinates the conic is the unit circle, the
///   ray a line, and a crossing a root of `|P + D·s|² = 1` inside the
///   arc's window. The discriminant is taken in its perpendicular-offset
///   form `1 − h²` (`h` the unit-coordinate line's distance from the
///   centre), which does not cancel far from a small conic. The
///   boundary pre-pass is `LoopEdge::contact` — never an arc's chord,
///   which is not boundary. A point on an arc's conic but off the arc
///   skips its own `s = 0` root. Every graze — a vertex on the ray line,
///   a ray tangent to a conic, a root at an arc's endpoint, a zero
///   advance — passes to the next ray.
/// - **Spiric and spline arcs**: the boundary pass reads the arc by
///   halving it until every piece's ball is definitely clear of the
///   point, or a point of the arc is within the band of it
///   ([`PiecedArc::contact`]), and each ray counts its crossings piece by
///   piece ([`PiecedArc::crossings`]). A spiric piece is settled when its
///   ball misses the ray or when its offset from the ray line is
///   definitely monotone across it ([`super::spiric_arc`]); a spline
///   piece when its control points all lie definitely on one side of the
///   ray line or behind the point, or when their offsets from the ray
///   line are definitely monotone ([`super::spline_arc`]). A ray with a
///   piece still unsettled at the depth grazes; a boundary reading that
///   runs out of depth or pieces is that edge's
///   [`PointInLoopError::Uncrossable`] at once — it is no reading in the
///   band. The crossing count presumes the point off the arc, which only
///   that boundary reading says: behind a caller's pass the walk takes
///   the caller's.
///
/// `normal` must be a unit normal of the loop's plane, and `q` must lie
/// in that plane; each is certified before the walk, and a loop, normal
/// or point that does not hold refuses rather than being read in some
/// other plane.
///
/// # The sign of `normal`
///
/// `normal` is read only to recover the loop's PLANE, so a caller may
/// hand over a raw CHART normal without folding in the face's sense —
/// `chord_join::face_plane_normal` does exactly that. The invariance
/// is derived here because it is this walk's property, not that
/// producer's, and it holds over lines and conics alike.
///
/// - **The preconditions.** The unit row reads `|n|`; a conic's tilt
///   reads `|axis × n|` — both unchanged. Every plane offset
///   `(p − o)·n` is negated, and the band's classifier is symmetric
///   about zero, so each decides `Zero` or not the same way.
/// - **The schedule.** Each member is projected as `r − n̂(n̂·r)`, which
///   is invariant under `n̂ ↦ −n̂`, and so is the arm row on its norm;
///   the walk then runs in the in-plane frame `(d, n̂ × d)`, whose
///   second axis is the only thing a sign flip moves.
/// - **Straight edges.** Negating that axis negates every vertex
///   ordinate `y`, so the straddle test `sign(yᵢ) ≠ sign(yⱼ)` is
///   unchanged, the vertex-on-the-ray `Zero` graze is unchanged, and the
///   crossing's advance `(xᵢyⱼ − xⱼyᵢ)/(yⱼ − yᵢ)` has numerator and
///   denominator both negated.
/// - **Conics.** A conic's crossing reads the ray `q + d·t` in the
///   conic's own frame, which does not read `n̂ × d`. The boundary
///   pre-pass reads no normal at all.
/// - **Spiric and spline pieces.** Every offset from the ray line along
///   `n̂ × d` is negated: the side rows and a spiric's distance from the
///   ray are symmetric in it; a spiric's turn row reads `|s_b − s_a|`;
///   a spline's control offsets all of one sign, or all steps of one
///   sign, swap that sign; and two ends on one side stay on one side.
///
/// Negation is exact, so **the verdict is bit-identical either way**,
/// and a refusal is identical in variant, predicate and band. One thing
/// is NOT identical, and saying so is what keeps that sentence true: a
/// refusal on a signed margin — a plane offset, or a ray's in-band
/// ordinate (the first ray's, when no ray decides) — carries it signed,
/// so the two signs refuse with `MarginKind::Value(−m)` against
/// `Value(m)`. That is
/// diagnostic payload — [`geom_core::Indeterminate`]'s own docs call its
/// fields *"honest diagnostic data … for actionable error messages and
/// later margin telemetry"*, and nothing here reads a margin back — but
/// a differential test comparing whole `Debug` renderings would see it.
/// `topo/tests/review_m3_pr3_pil.rs`'s
/// `the_verdict_is_blind_to_the_normals_sign` pins the straight-edge
/// verdicts, and `validate.rs`'s
/// `point_in_loop_is_blind_to_the_normals_sign_on_an_arc_bearing_loop`
/// the conic half; both compare variant, predicate and band rather than
/// the rendering.
///
/// # Errors
///
/// [`PointInLoopError`] — an escalation, exhaustion, an unwalkable
/// loop, a spiric or spline edge whose reading ran out, or a broken
/// precondition ([`PointInLoopError::OffPlane`]).
pub fn point_in_loop<T: Decide>(
    body: &Body<T>,
    r#loop: LoopKey,
    normal: Vec3<T>,
    q: Point3<T>,
    band: Band,
) -> Result<LoopContainment, PointInLoopError> {
    certify_plane(body, r#loop, normal, q, band)?;
    point_in_loop_projected(body, r#loop, normal, q, band)
}

/// [`point_in_loop`] with no precondition certified: the loop is read
/// as its projection along `normal`. For `boolean::solid_contain`'s
/// in-face test alone, which reads a face's trim at the face's OWN
/// surface plane, `q` a ray's hit on that plane: an operand the
/// boolean's gates admit may hold a vertex off that plane by more than
/// a tightened run's band (a re-described face), and the face's region
/// there is its trim's projection, not a refusal.
///
/// # Errors
///
/// As [`point_in_loop`], but never [`PointInLoopError::OffPlane`].
pub(crate) fn point_in_loop_projected<T: Decide>(
    body: &Body<T>,
    r#loop: LoopKey,
    normal: Vec3<T>,
    q: Point3<T>,
    band: Band,
) -> Result<LoopContainment, PointInLoopError> {
    let lp = carrier_loop(body, r#loop, WALK_ROWS.conic, band)?;
    if lp.edges.iter().all(|e| matches!(e, LoopEdge::Chord)) {
        return point_in_vertex_polygon(body, r#loop, normal, q, band);
    }
    Ok(
        match carrier_walk(r#loop, &lp, normal, q, band, Boundary::Verdict)? {
            WalkSide::In => LoopContainment::In,
            WalkSide::Out => LoopContainment::Out,
            WalkSide::OnBoundary => LoopContainment::OnBoundary,
        },
    )
}

/// **[`point_in_loop`]'s preconditions, certified**: `normal` is a unit
/// vector, the whole loop lies in the plane through its first vertex
/// with that normal, and so does `q` (the module docs' three rows). A
/// wrong plane is a different region, so anything but a definite
/// `Zero` refuses. A spline's control points stand for its locus:
/// positive weights keep the curve in their hull.
fn certify_plane<T: Decide>(
    body: &Body<T>,
    r#loop: LoopKey,
    normal: Vec3<T>,
    q: Point3<T>,
    band: Band,
) -> Result<(), PointInLoopError> {
    let corrupt = || PointInLoopError::CorruptLoop { r#loop };
    let escalate = PointInLoopError::on_margin(r#loop, LoopDecision::Plane);
    let off_plane = |cause| PointInLoopError::OffPlane(OffPlane { r#loop, cause });
    let LoopBoundary::Cycle { first } = body.get_loop(r#loop).ok_or_else(corrupt)?.boundary else {
        return Err(corrupt());
    };
    let steps = cycle_steps(body, first);
    let origin = steps.first().ok_or_else(corrupt)?.point;
    let (_, reach) = loop_reach(body, r#loop)?;
    let unit = Margin::levered(normal.norm() - T::one(), reach);
    if decide("point_in_loop_normal", unit, band).map_err(escalate)? != Sign::Zero {
        return Err(off_plane(OffPlaneCause::NormalNotUnit));
    }
    let off = |p: Point3<T>| Margin::of((p - origin).dot(normal));
    let tilt = |axis: Vec3<T>, arm: T| Margin::levered(axis.cross(normal).norm(), arm);
    for step in &steps {
        let mut margins = vec![off(step.point)];
        match step.curve.map(|c| c.carrier()) {
            None | Some(geom::Curve3::Line { .. }) => {}
            Some(&geom::Curve3::Circle {
                center,
                axis,
                radius,
                ..
            }) => margins.extend([off(center), tilt(axis, radius)]),
            Some(&geom::Curve3::Ellipse {
                center,
                axis,
                semi_u,
                semi_v,
                ..
            }) => margins.extend([
                off(center),
                tilt(axis, geom::Curve3::semi_axes_by_size(semi_u, semi_v).0),
            ]),
            Some(&geom::Curve3::Spiric {
                center,
                u_ref,
                major_radius,
                minor_radius,
                offset,
                ..
            }) => margins.extend([
                off(center + u_ref * offset),
                tilt(u_ref, major_radius + minor_radius + offset.abs()),
            ]),
            Some(geom::Curve3::Nurbs(n)) => margins.extend(n.control().iter().map(|&p| off(p))),
        }
        for m in margins {
            if decide("point_in_loop_plane", m, band).map_err(escalate)? != Sign::Zero {
                return Err(off_plane(OffPlaneCause::Loop { edge: step.edge }));
            }
        }
    }
    if decide("point_in_loop_query", off(q), band).map_err(escalate)? != Sign::Zero {
        return Err(off_plane(OffPlaneCause::Query));
    }
    Ok(())
}

/// [`point_in_loop`] for a caller whose own boundary pass
/// ([`CarrierLoop::place_off`]) has already placed `q` off every edge
/// of the loop it read: `true` inside, `false` outside. The walk reads no edge's boundary again: a crossing at `q`
/// itself, which that pass rules out, is a graze, a conic's `Carrier`
/// reading is what its crossing count needs, and a spiric that pass
/// could not read refuses as uncrossable.
///
/// # Errors
///
/// As [`point_in_loop`].
pub(crate) fn carrier_loop_side<T: Decide>(
    r#loop: LoopKey,
    read: &OffLoop<T>,
    normal: Vec3<T>,
    q: Point3<T>,
    band: Band,
) -> Result<bool, PointInLoopError> {
    let OffLoop { lp, off } = read;
    if lp.edges.iter().all(|e| matches!(e, LoopEdge::Chord)) {
        return Ok(polygon_walk(r#loop, &lp.verts, normal, q, band)? == LoopContainment::In);
    }
    Ok(carrier_walk(r#loop, lp, normal, q, band, Boundary::Decided(off))? == WalkSide::In)
}

/// What [`carrier_walk`] says of a point.
#[derive(Clone, Copy, PartialEq, Eq)]
enum WalkSide {
    In,
    Out,
    /// Only under [`Boundary::Verdict`].
    OnBoundary,
}

fn carrier_walk<T: Decide>(
    r#loop: LoopKey,
    lp: &CarrierLoop<T>,
    normal: Vec3<T>,
    q: Point3<T>,
    band: Band,
    boundary: Boundary<'_>,
) -> Result<WalkSide, PointInLoopError> {
    let on_boundary = PointInLoopError::on(r#loop, LoopDecision::Boundary);
    let escalate = PointInLoopError::on_margin(r#loop, LoopDecision::Ray);
    let (verts, edges) = (&lp.verts, &lp.edges);
    // The loop's reach from `q`: the schedule gate's lever.
    let extent = extent_from(r#loop, q, verts, &lp.balls)?;
    let n = verts.len();
    // ---- Boundary pass, and which conics carry `q`. ----
    let mut on_carrier = vec![false; n];
    for (i, edge) in edges.iter().enumerate() {
        let off = match boundary {
            Boundary::Decided(off) => off[i],
            Boundary::Verdict => match edge
                .contact((verts[i], verts[(i + 1) % n]), q, WALK_ROWS, band)
                .map_err(on_boundary)?
                .off()
            {
                Ok(off) => off,
                Err(_) => return Ok(WalkSide::OnBoundary),
            },
        };
        // A pieced arc's crossing count presumes `q` definitely off the
        // arc, which only its own reading says.
        let pieced = match edge {
            LoopEdge::Spiric(_) => Some(UncrossableCarrier::Spiric),
            LoopEdge::Spline(_) => Some(UncrossableCarrier::Spline),
            LoopEdge::Chord | LoopEdge::Conic(_) => None,
        };
        match (off, pieced) {
            (OffEdge::Unread, Some(carrier)) => {
                return Err(PointInLoopError::Uncrossable(Uncrossable {
                    r#loop,
                    edge: lp.keys[i],
                    carrier,
                }));
            }
            (OffEdge::Off | OffEdge::Unread, _) => {}
            (OffEdge::Carrier, _) => on_carrier[i] = true,
        }
    }
    // ---- The rays. ----
    // The boundary pass (this walk's own, or its caller's) has decided
    // `q` off every edge and arc by more than the band, so each reading
    // below is about one ray ([`ray_walk`]).
    let inside = walk_schedule(
        r#loop,
        normal,
        extent,
        "point_in_arc_loop_arm",
        band,
        |d, side_axis| {
            let mut crossings = RayFault::of(
                ray_walk::ray_crossings(verts, q, d, side_axis, &ARC_LOOP_ROWS, band, |i| {
                    matches!(edges[i], LoopEdge::Chord)
                }),
                escalate,
            )?;
            for (i, edge) in edges.iter().enumerate() {
                crossings += match edge {
                    LoopEdge::Conic(k) => {
                        RayFault::of(conic_crossings(*k, on_carrier[i], q, d, band), escalate)?
                    }
                    // A piece still unsettled at the depth is a graze, or a
                    // crossing at a piece's end ([`PiecedArc::crossings`]).
                    LoopEdge::Spiric(k) => {
                        k.crossings(q, d, side_axis, band).ok_or(RayFault::Graze)?
                    }
                    LoopEdge::Spline(k) => {
                        k.crossings(q, d, side_axis, band).ok_or(RayFault::Graze)?
                    }
                    LoopEdge::Chord => 0,
                };
            }
            Ok(!crossings.is_multiple_of(2))
        },
    )?;
    Ok(if inside { WalkSide::In } else { WalkSide::Out })
}

/// **A ball holding the whole loop**, `(center, radius)`: the loop's
/// first vertex and the reach [`point_in_loop`] confines its
/// refusal with ([`loop_extent_from`]). Nothing here needs the loop to
/// be planar — each edge is bounded on its own carrier — so a curved
/// face's outer loop is served the same way.
///
/// # Errors
///
/// [`PointInLoopError::CorruptLoop`] — a loop [`loop_extent_from`]
/// refuses.
pub(crate) fn loop_reach<T: Decide>(
    body: &Body<T>,
    r#loop: LoopKey,
) -> Result<(Point3<T>, T), PointInLoopError> {
    let (verts, balls) = loop_hull(body, r#loop)?;
    let anchor = *verts
        .first()
        .ok_or(PointInLoopError::CorruptLoop { r#loop })?;
    Ok((anchor, extent_from(r#loop, anchor, &verts, &balls)?))
}

/// How many times the ray `q + d·t`, `t > 0`, crosses the arc `k` —
/// `None` for a graze. `on_carrier` says the pre-pass put `q` on the
/// conic and off the arc, so a root at `q` itself is no crossing.
///
/// # Errors
///
/// The raw [`Indeterminate`] of an in-band margin on any of its rows,
/// which sets the ray aside ([`ray_walk`]).
fn conic_crossings<T: Decide>(
    k: ConicArc<T>,
    on_carrier: bool,
    q: Point3<T>,
    d: Vec3<T>,
    band: Band,
) -> Result<Option<usize>, Indeterminate> {
    let (px, py) = k.unit(q);
    let (dx, dy) = (d.dot(k.u) / k.a, d.dot(k.v) / k.b);
    // On `point_in_loop`, `certify_plane` has put the conic in the plane
    // `d` lies in, so `(dx, dy)` is nonzero. On the projected read a
    // conic tilted off that plane shrinks it, and a zero poisons every
    // margin below, which sets the ray aside.
    let dn = (dx.powi(2) + dy.powi(2)).sqrt();
    let (ex, ey) = (dx / dn, dy / dn);
    let along = px * ex + py * ey;
    let (hx, hy) = (px - ex * along, py - ey * along);
    let disc = T::one() - (hx.powi(2) + hy.powi(2));
    // `(1 − h²)/2` is the unit circle's `(r² − h²)/2r`: levered by the
    // conic's smaller semi-axis it is a length (exact for a circle).
    match decide(
        "point_in_arc_loop_conic_disc",
        Margin::levered(disc * T::from_f64(0.5), k.lever),
        band,
    )? {
        Sign::Positive => {}
        Sign::Zero => return Ok(None), // tangent to the conic
        Sign::Negative => return Ok(Some(0)),
    }
    let root = disc.max(T::zero()).sqrt();
    let mut crossings = 0;
    for s in [T::zero() - along - root, T::zero() - along + root] {
        // Back to metres along the unit ray.
        let t = s / dn;
        match decide("point_in_arc_loop_conic_advance", Margin::of(t), band)? {
            Sign::Positive => {}
            Sign::Negative => continue,
            Sign::Zero if on_carrier => continue,
            // A crossing at `q` the pre-pass did not see.
            Sign::Zero => return Ok(None),
        }
        let (hx, hy) = (px + dx * t, py + dy * t);
        let hn = (hx.powi(2) + hy.powi(2)).sqrt();
        match k.in_window((hx / hn, hy / hn), band)? {
            Sign::Positive => crossings += 1,
            Sign::Negative => {}
            // An endpoint's neighbourhood. The endpoint is a vertex of
            // the loop, and a ray through a vertex has already grazed on
            // its side row, so this arm answers only where that row and
            // this window disagree inside the band — a graze all the
            // same, never a count.
            Sign::Zero => return Ok(None),
        }
    }
    Ok(Some(crossings))
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;

    /// [`ConicArc::of`] over a fixture span, its extent decided forward
    /// as certification decides it.
    fn conic_of<T: Decide>(
        carrier: &geom::Curve3<T>,
        (t0, t1): (T, T),
        rows: ConicRows,
        band: Band,
    ) -> Result<Option<ConicArc<T>>, ConicArcError> {
        let extent = Extent::forward(
            "test_conic_extent",
            t0,
            t1,
            geom_core::InfSpeed::new(T::one()),
            band,
        )
        .expect("a forward fixture span");
        ConicArc::read(carrier, (t0, t1), extent, rows, band)
    }
    use geom_core::Tol;

    /// An arc's end neighbourhood is the band's own width along the
    /// carrier, however short or long the arc: on a `w = 0.02` arc of
    /// radius 10, and on its complement, a point `s` along the carrier
    /// from an end is on or off the arc for every `s` past the band —
    /// including the `ε < s < 10ε / sin(w/2)` stretch a cosine window
    /// compresses into its zero or escalation zone — and only within
    /// the band is it the end's.
    #[test]
    fn an_arcs_end_zone_is_the_bands_own_width() {
        let band = Band::linear(Tol::witness()).unwrap();
        let eps = band.zero();
        let r = 10.0;
        let carrier = geom::Curve3::Circle {
            center: Point3::new(0.0, 0.0, 0.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
            radius: r,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let at = |theta: f64| Point3::new(r * theta.cos(), r * theta.sin(), 0.0);
        for (t0, t1) in [(0.0, 0.02), (0.0, core::f64::consts::TAU - 0.02)] {
            let Ok(Some(k)) = conic_of(&carrier, (t0, t1), WALK_ROWS.conic, band) else {
                panic!("a circle arc under a period");
            };
            let ask = |q| k.hit(q, band).expect("decided");
            for s in [20.0 * eps, 50.0 * eps, 500.0 * eps, 5000.0 * eps] {
                let d = s / r;
                assert!(
                    matches!(ask(at(t1 - d)), ConicHit::On(_)),
                    "w = {t1}: {s} m inside the end"
                );
                assert_eq!(
                    ask(at(t1 + d)),
                    ConicHit::Carrier,
                    "w = {t1}: {s} m past the end"
                );
                assert!(
                    matches!(ask(at(t0 + d)), ConicHit::On(_)),
                    "w = {t1}: {s} m inside the start"
                );
                assert_eq!(
                    ask(at(t0 - d)),
                    ConicHit::Carrier,
                    "w = {t1}: {s} m before the start"
                );
            }
            assert_eq!(
                ask(at(t1 + 0.5 * eps / r)),
                ConicHit::End,
                "w = {t1}: within the band of the end"
            );
            assert_eq!(
                ask(at(t0 - 0.5 * eps / r)),
                ConicHit::End,
                "w = {t1}: within the band of the start"
            );
        }
    }

    /// A steep ellipse (`a/b = 20`) read at the band's own width in
    /// metres, not in unit coordinates: a point `12ε` off the major
    /// vertex, either side, is not on the conic — the smaller semi-axis
    /// would have levered its unit-coordinate miss down to `0.6ε` — and a
    /// point on the conic `15ε` of arc from the end at the minor vertex
    /// (where the speed is `a`, so a unit chord levered by `b` would read
    /// it `0.75ε` away) is on the arc, not at its end.
    #[test]
    fn a_steep_ellipse_reads_the_band_in_metres() {
        let band = Band::linear(Tol::witness()).unwrap();
        let eps = band.zero();
        let (a, b) = (20.0, 1.0);
        let carrier = geom::Curve3::Ellipse {
            center: Point3::new(0.0, 0.0, 0.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
            semi_u: a,
            semi_v: b,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        // The arc from the minor vertex (π/2) round through the major
        // vertex at π to 3π/2.
        let (t0, t1) = (core::f64::consts::FRAC_PI_2, 1.5 * core::f64::consts::PI);
        let Ok(Some(k)) = conic_of(&carrier, (t0, t1), WALK_ROWS.conic, band) else {
            panic!("an ellipse arc under a period");
        };
        for s in [12.0, 15.0, 40.0] {
            for side in [1.0, -1.0] {
                let q = Point3::new(-a - side * s * eps, 0.0, 0.0);
                let got = k.hit(q, band);
                assert_eq!(
                    got,
                    Ok(ConicHit::Off),
                    "{s}ε off the major vertex ({side}): a definite answer, neither on nor an escalation"
                );
            }
        }
        // `15ε` of arc from the minor vertex: the speed there is `a`.
        for s in [11.0, 15.0, 18.0, 40.0] {
            let q = k.point(t0 + s * eps / a);
            let got = k.hit(q, band);
            assert!(
                matches!(got, Ok(ConicHit::On(_))),
                "{s}ε along the arc from its end: on the arc, neither its end nor an escalation"
            );
        }
    }

    /// The steep ellipse of [`a_steep_ellipse_reads_the_band_in_metres`].
    fn steep(band: Band, (t0, t1): (f64, f64)) -> Result<Option<ConicArc<f64>>, ConicArcError> {
        let carrier = geom::Curve3::Ellipse {
            center: Point3::new(0.0, 0.0, 0.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
            semi_u: 20.0,
            semi_v: 1.0,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        conic_of(&carrier, (t0, t1), WALK_ROWS.conic, band)
    }

    /// **A nearly full ellipse arc keeps its gap; a full one has none.**
    /// A full period less a gap of `15ε` at the MINOR vertex (`a/b = 20`,
    /// where the gap runs at speed `a`): the span is read only for a
    /// window wound past a period, so the arc keeps both ends, and the
    /// point in the middle of the gap — on the carrier, `7.5ε` from each
    /// end — is never ON the arc. Levered by `b` as a CLOSED verdict, that
    /// gap read `0.75ε` and the arc read closed. A true full period keeps
    /// its two coincident ends at the edge's one vertex, and every other
    /// point of the carrier is on it.
    #[test]
    fn a_nearly_full_ellipse_arc_is_not_closed() {
        let band = Band::linear(Tol::witness()).unwrap();
        let eps = band.zero();
        let gap = 15.0 * eps / 20.0;
        let t0 = core::f64::consts::FRAC_PI_2 + 0.5 * gap;
        let t1 = t0 + core::f64::consts::TAU - gap;
        let Ok(Some(k)) = steep(band, (t0, t1)) else {
            panic!("the arc is read");
        };
        let mid_gap = k.hit(k.point(core::f64::consts::FRAC_PI_2), band);
        assert!(
            !matches!(mid_gap, Ok(ConicHit::On(_))),
            "the gap is not the arc: {mid_gap:?}"
        );
        for s in [5.0, 11.0] {
            // `s·ε` of arc into the gap from the arc's end at `t1`.
            let past = k.hit(k.point(t1 + s * eps / 20.0), band);
            assert!(
                !matches!(past, Ok(ConicHit::On(_))),
                "{s}ε past the end, in the gap: {past:?}"
            );
        }
        assert!(matches!(k.hit(k.point(1.0), band), Ok(ConicHit::On(_))));
        let Ok(Some(full)) = steep(band, (0.0, core::f64::consts::TAU)) else {
            panic!("the full ellipse is read");
        };
        for t in [0.3, core::f64::consts::FRAC_PI_2, 3.0, 5.0] {
            assert!(
                matches!(full.hit(full.point(t), band), Ok(ConicHit::On(_))),
                "a full period is on the arc at {t}"
            );
        }
        assert_eq!(full.hit(full.point(0.0), band), Ok(ConicHit::End));
    }

    /// **An over-wound ellipse window is not read as an arc.** The review's
    /// repro: `a = 14, b = 1`, a window of a full period plus a `30ε`
    /// overlap at the minor vertex (where the edge runs at speed `a`).
    /// Levered by `b` alone the overlap read `2.1ε`, in band and not
    /// negative, so the window read as an arc and a point ON the doubled
    /// stretch — `15ε` from each end — read `Carrier`, off the edge. The
    /// span is now read at the inf over the overlap itself, which reads
    /// its `30ε` definitely past the band: wound past a period.
    #[test]
    fn an_over_wound_ellipse_window_is_not_an_arc() {
        let band = Band::linear(Tol::witness()).unwrap();
        let eps = band.zero();
        let ellipse = |a: f64| geom::Curve3::Ellipse {
            center: Point3::new(0.0, 0.0, 0.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
            semi_u: a,
            semi_v: 1.0,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        // The repro, as the review spelled it at ε = 1e-9.
        if eps == 1e-9 {
            let got = conic_of(
                &ellipse(14.0),
                (1.570796325723468, 7.853981635045912),
                WALK_ROWS.conic,
                band,
            );
            assert!(
                matches!(got, Err(ConicArcError::WoundPastPeriod)),
                "the repro is wound past a period"
            );
        }
        // The population: overlaps of 30ε and 100ε of arc, at a/b 14, 20 and
        // 100, centred at the minor vertex, the major vertex and between.
        // Never an arc: escalated, or wound past a period.
        for a in [14.0f64, 20.0, 100.0] {
            for overlap in [30.0 * eps, 100.0 * eps] {
                for at in [
                    core::f64::consts::FRAC_PI_2,
                    0.0,
                    core::f64::consts::FRAC_PI_4,
                ] {
                    let speed = (a.powi(2) * at.sin().powi(2) + at.cos().powi(2)).sqrt();
                    let dt = overlap / speed;
                    let span = (at - 0.5 * dt, at - 0.5 * dt + core::f64::consts::TAU + dt);
                    let got = conic_of(&ellipse(a), span, WALK_ROWS.conic, band);
                    assert!(
                        matches!(
                            got,
                            Err(ConicArcError::Escalated(_) | ConicArcError::WoundPastPeriod)
                        ),
                        "a/b {a}, overlap {overlap} at {at}: read as an arc"
                    );
                }
            }
        }
        // A window whose overlap reads in the band at both rates (`2ε` at
        // the MAJOR vertex, where the larger semi-axis would overstate it
        // twentyfold) escalates: no reading settles a turn, and the sup
        // does not read it short. Check 2 escalates on the same readings,
        // so no edge certified at this band is such a window.
        for a in [20.0f64, 100.0] {
            let dt = 2.0 * eps;
            let got = conic_of(
                &ellipse(a),
                (-0.5 * dt, core::f64::consts::TAU + 0.5 * dt),
                WALK_ROWS.conic,
                band,
            );
            assert!(
                matches!(got, Err(ConicArcError::Escalated(_))),
                "a/b {a}: a 2ε overlap at the major vertex escalates"
            );
        }
        // A circle: an overlap in its band escalates, one past it is wound.
        let circle = geom::Curve3::Circle {
            center: Point3::new(0.0, 0.0, 0.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
            radius: 1.0,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let tau = core::f64::consts::TAU;
        assert!(matches!(
            conic_of(&circle, (0.0, tau + 5.0 * eps), WALK_ROWS.conic, band),
            Err(ConicArcError::Escalated(_))
        ));
        assert!(matches!(
            conic_of(&circle, (0.0, tau + 30.0 * eps), WALK_ROWS.conic, band),
            Err(ConicArcError::WoundPastPeriod)
        ));
    }

    /// **The span rule tags what it can raise**: an elliptic window
    /// over-wound by an overlap in the band at both bounds over it
    /// escalates on its in-band margin — always negative, the refused side,
    /// so its ending is the lever alone; one whose inf over the overlap
    /// reads zero while the sup reads it definitely negative is a straddle
    /// of the two bounds, which also ends in the lever alone and carries no
    /// unreadable-margin note. The bounds over a short overlap are within
    /// the larger semi-axis times its width of each other, so a straddle
    /// needs an overlap of `≈ √(Kε/a)` at a vertex slow enough that the
    /// floor reads it zero: a needle ellipse at its slow vertex. Driven
    /// through `ConicArc::of`; the walk's `carrier_loop` site maps both to
    /// `LoopDecision::ArcSpan` unchanged.
    #[test]
    fn the_span_rule_tags_an_over_wound_margin_and_a_straddle() {
        let band = Band::linear(Tol::witness()).unwrap();
        let eps = band.zero();
        let ellipse = |a: f64, b: f64| geom::Curve3::Ellipse {
            center: Point3::new(0.0, 0.0, 0.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
            semi_u: a,
            semi_v: b,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let over = |at: f64, dt: f64| (at - 0.5 * dt, at - 0.5 * dt + core::f64::consts::TAU + dt);
        // Centred at the minor vertex, where the edge's speed is `a`, by an
        // overlap whose length there is mid-band.
        let m = 0.5 * (band.zero() + band.escalate());
        let lever = format!("Recourse: {}", LoopDecision::ArcSpan.lever());
        let fast = over(core::f64::consts::FRAC_PI_2, m / 100.0);
        match conic_of(&ellipse(100.0, 1.0), fast, WALK_ROWS.conic, band) {
            Err(ConicArcError::Escalated(ReadEscalation {
                escalation: Escalation::Margin,
                diag,
            })) => {
                let v = diag
                    .margin
                    .diagnostic_f64_for_error_text()
                    .value()
                    .expect("an in-band margin");
                assert!(v < 0.0, "over-wound: {v:e}");
                assert_eq!(
                    LoopDecision::ArcSpan.ending(Escalation::Margin, &diag, Reading::AtRest),
                    lever
                );
            }
            _ => panic!("expected an in-band span"),
        }
        // A needle `a = 1`, `b = ε/(2h)` at its slow vertex `θ = 0`, by an
        // overlap `h`: the inf there is the floor `b`, reading `ε/2`, and
        // the sup `b + h/2` reads past `10ε` once `h² ≥ 20ε`.
        for k in [40.0, 80.0] {
            let h = (k * eps).sqrt();
            let carrier = ellipse(1.0, eps / (2.0 * h));
            let span = over(0.0, h);
            let bracket = geom_core::k_stats::Bracket::open();
            let got = conic_of(&carrier, span, WALK_ROWS.conic, band);
            let recorded = bracket.finish();
            let Err(ConicArcError::Escalated(got)) = got else {
                panic!("overlap √({k}ε): expected a straddle");
            };
            // The window's signed gap to a period, at the sup and at the
            // inf over it, as `ConicArc::of` reads them.
            let extent = Extent::between(span.0, span.1);
            let gap = core::f64::consts::TAU - extent.get();
            let sup = geom::carrier_speed::turn_defect_speed_hi(&carrier, span.0, extent).get();
            let inf = geom::carrier_speed::turn_defect_speed_lo(&carrier, span.0, extent).get();
            assert_straddle(
                got,
                &recorded,
                (LoopDecision::ArcSpan, WALK_ROWS.conic.straddle),
                (gap * sup, gap * inf),
                &format!("overlap √({k}ε)"),
            );
        }
    }

    /// **A straddle is one escalation on the frame, carrying the
    /// enclosure of its two bounds** (`k_stats::straddle`): under the
    /// walk's straddle row, the enclosure `bounds` the site's own lower
    /// and upper readings give, reaching from within the zero band to past
    /// the escalation band, never a poisoned margin. Its ending is the
    /// decision's lever alone, as before the enclosure was carried: the
    /// straddle's own arm gives it, whatever the margin.
    fn assert_straddle(
        got: ReadEscalation,
        recorded: &geom_core::k_stats::Recorded,
        (decision, row): (LoopDecision, &str),
        bounds: (f64, f64),
        label: &str,
    ) {
        let ReadEscalation {
            escalation: Escalation::Straddle,
            diag,
        } = got
        else {
            panic!("{label}: expected a straddle, got {got:?}");
        };
        assert_eq!(diag.predicate, Some(row), "{label}");
        let geom_core::ErrorTextReading::Enclosure { lo, hi } =
            diag.margin.diagnostic_f64_for_error_text()
        else {
            panic!("{label}: expected an enclosure, got {:?}", diag.margin);
        };
        let close = |x: f64, y: f64| (x - y).abs() <= 1e-6 * y.abs();
        assert!(
            close(lo, bounds.0) && close(hi, bounds.1),
            "{label}: [{lo:e}, {hi:e}] is the hull of the site's bounds {bounds:?}"
        );
        let band = diag.band;
        assert!(
            lo.abs().min(hi.abs()) <= band.zero() && lo.abs().max(hi.abs()) >= band.escalate(),
            "{label}: [{lo:e}, {hi:e}] straddles the band"
        );
        assert!(!diag.terminal_sliver, "{label}");
        assert_eq!(
            recorded.escalations.last().map(|e| e.source),
            Some(diag),
            "{label}: on the frame"
        );
        assert_eq!(
            decision.ending(Escalation::Straddle, &diag, Reading::AtRest),
            format!("Recourse: {}", decision.lever()),
            "{label}: the lever alone"
        );
    }

    /// **The centre of an ellipse** is `b` from it, exactly the lower
    /// bound there, so it reads `Off` on that bound alone — the Newton
    /// foot, undefined where `∇F` vanishes, is never formed.
    #[test]
    fn an_ellipses_centre_reads_off_on_the_lower_bound() {
        let band = Band::linear(Tol::witness()).unwrap();
        let Ok(Some(k)) = steep(band, (0.0, core::f64::consts::PI)) else {
            panic!("the arc is read");
        };
        assert_eq!(k.hit(Point3::new(0.0, 0.0, 0.0), band), Ok(ConicHit::Off));
    }

    /// The same, in the telemetry: the centre records no `Invalid`
    /// sample, which a `0/0` Newton foot would.
    #[cfg(feature = "probe")]
    #[test]
    fn an_ellipses_centre_records_no_invalid_margin() {
        use geom_core::k_stats::{self, Probe, SampleOutcome};
        let band = Band::linear(Tol::witness()).unwrap();
        let p = |x: f64| <Probe as geom_core::Real>::from_f64(x);
        let carrier = geom::Curve3::Ellipse {
            center: Point3::new(p(0.0), p(0.0), p(0.0)),
            axis: Vec3::new(p(0.0), p(0.0), p(1.0)),
            semi_u: p(20.0),
            semi_v: p(1.0),
            u_ref: Vec3::new(p(1.0), p(0.0), p(0.0)),
        };
        let Ok(Some(k)) = conic_of(
            &carrier,
            (p(0.0), p(core::f64::consts::PI)),
            WALK_ROWS.conic,
            band,
        ) else {
            panic!("the arc is read");
        };
        k_stats::start_recording();
        let got = k.hit(Point3::new(p(0.0), p(0.0), p(0.0)), band);
        let samples = k_stats::take_samples();
        assert_eq!(got, Ok(ConicHit::Off));
        assert!(
            samples.iter().all(|s| s.outcome != SampleOutcome::Invalid),
            "{samples:?}"
        );
    }

    /// The lower bound and the upper bound `ConicArc::hit` reads on the
    /// distance from the ellipse of semi-axes `a` (along x) and `b` to the
    /// point `x` out along its major axis: the Hessian-bounded root of the
    /// implicit form, and the distance to the Newton foot, the vertex.
    fn major_vertex_bounds(a: f64, b: f64, x: f64) -> (f64, f64) {
        let u = x / a;
        let f = u.powi(2) - 1.0;
        let g = 2.0 * u / a;
        let small = a.min(b);
        let lower = 2.0 * f.abs() / (g + (g.powi(2) + 4.0 * f.abs() / small.powi(2)).sqrt());
        (lower, x - a)
    }

    /// **An ellipse bending tighter than the band** (`b²/a = ε/100`): at
    /// its major vertex the lower bound on a point `kε` out reads within
    /// the zero band while the upper bound — here the distance itself —
    /// reads definitely beyond it. The answer escalates as a straddle of
    /// the two ([`assert_straddle`]); it is never `On`, which reading ON
    /// off the lower bound would give.
    #[test]
    fn an_ellipse_tighter_than_the_band_straddles_it() {
        let band = Band::linear(Tol::witness()).unwrap();
        let eps = band.zero();
        for a in [1.0, 4.0] {
            let b = (a * eps / 100.0).sqrt();
            let carrier = geom::Curve3::Ellipse {
                center: Point3::new(0.0, 0.0, 0.0),
                axis: Vec3::new(0.0, 0.0, 1.0),
                semi_u: a,
                semi_v: b,
                u_ref: Vec3::new(1.0, 0.0, 0.0),
            };
            let Ok(Some(k)) = conic_of(
                &carrier,
                (0.0, core::f64::consts::TAU),
                WALK_ROWS.conic,
                band,
            ) else {
                panic!("a {a}: the full ellipse is read");
            };
            for out in [15.0, 20.0, 40.0] {
                let bracket = geom_core::k_stats::Bracket::open();
                let got = k.hit(Point3::new(a + out * eps, 0.0, 0.0), band);
                let recorded = bracket.finish();
                let Err(got) = got else {
                    panic!("a {a}, {out}ε out: expected a straddle, got {got:?}");
                };
                assert_straddle(
                    got,
                    &recorded,
                    (LoopDecision::Boundary, "curve_carrier_straddle"),
                    major_vertex_bounds(a, b, a + out * eps),
                    &format!("a {a}, {out}ε"),
                );
            }
        }
    }

    /// The ellipse of semi-axes `a` along x and `b` along y about the
    /// origin in `z = 0`, stored with `b` in `major` (`u_ref` along y),
    /// as STEP import may store it: `geom::Curve3::Ellipse` certifies the
    /// semi-axes positive, not ordered. Its parameter runs a quarter
    /// turn behind the `a`-in-`major` storage's.
    fn stored_minor_first(a: f64, b: f64) -> geom::Curve3<f64> {
        geom::Curve3::Ellipse {
            center: Point3::new(0.0, 0.0, 0.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
            semi_u: b,
            semi_v: a,
            u_ref: Vec3::new(0.0, 1.0, 0.0),
        }
    }

    /// **[`an_ellipse_tighter_than_the_band_straddles_it`], stored minor
    /// first.** The lower bound's Hessian term is the SMALLER semi-axis
    /// whichever field holds it; read off `b` (here the larger), it
    /// over-states the distance and reads the point `Off`.
    #[test]
    fn an_ellipse_stored_minor_first_tighter_than_the_band_straddles_it() {
        let band = Band::linear(Tol::witness()).unwrap();
        let eps = band.zero();
        let (a, b) = (1.0, (eps / 100.0).sqrt());
        let Ok(Some(k)) = conic_of(
            &stored_minor_first(a, b),
            (0.0, core::f64::consts::TAU),
            WALK_ROWS.conic,
            band,
        ) else {
            panic!("the full ellipse is read");
        };
        let bracket = geom_core::k_stats::Bracket::open();
        let got = k.hit(Point3::new(a + 20.0 * eps, 0.0, 0.0), band);
        let recorded = bracket.finish();
        let Err(got) = got else {
            panic!("expected a straddle, got {got:?}");
        };
        assert_straddle(
            got,
            &recorded,
            (LoopDecision::Boundary, "curve_carrier_straddle"),
            major_vertex_bounds(a, b, a + 20.0 * eps),
            "stored minor first",
        );
    }

    /// **[`an_over_wound_ellipse_window_is_not_an_arc`]'s population,
    /// stored minor first**: never an arc.
    #[test]
    fn an_over_wound_ellipse_window_stored_minor_first_is_not_an_arc() {
        let band = Band::linear(Tol::witness()).unwrap();
        let eps = band.zero();
        for a in [14.0f64, 20.0, 100.0] {
            for overlap in [30.0 * eps, 100.0 * eps] {
                for at in [
                    core::f64::consts::FRAC_PI_2,
                    0.0,
                    core::f64::consts::FRAC_PI_4,
                ] {
                    let speed = (a.powi(2) * at.sin().powi(2) + at.cos().powi(2)).sqrt();
                    let dt = overlap / speed;
                    let start = at - 0.5 * dt - core::f64::consts::FRAC_PI_2;
                    let span = (start, start + core::f64::consts::TAU + dt);
                    let got = conic_of(&stored_minor_first(a, 1.0), span, WALK_ROWS.conic, band);
                    assert!(
                        matches!(
                            got,
                            Err(ConicArcError::Escalated(_) | ConicArcError::WoundPastPeriod)
                        ),
                        "a/b {a}, overlap {overlap} at {at}: read as an arc"
                    );
                }
            }
        }
    }

    /// **A null self-loop is not its placeholder circle.** `mef`'s lone
    /// site certifies the new edge as `self_loop_circle_at(p)`, the unit
    /// circle about `p + x̂`. Read as an arc it would put that circle's
    /// centre `In` a region the null edge does not bound; the walk
    /// refuses each loop holding it instead.
    #[test]
    fn a_scaffold_circle_is_refused_not_read_as_an_arc() {
        let tol = Tol::witness();
        let band = Band::linear(tol).unwrap();
        let mut body = Body::<f64>::new();
        let seed = body.mvfs(Point3::new(0.0, 0.0, 0.0), true).unwrap();
        let circ = body
            .mef_chord(
                crate::euler::MefSite::Lone {
                    r#loop: seed.r#loop,
                },
                tol,
            )
            .unwrap();
        let centre = Point3::new(1.0, 0.0, 0.0);
        let up = Vec3::new(0.0, 0.0, 1.0);
        for lp in [seed.r#loop, circ.r#loop] {
            let got = point_in_loop(&body, lp, up, centre, band);
            assert!(
                matches!(got, Err(PointInLoopError::CorruptLoop { r#loop }) if r#loop == lp),
                "the placeholder's centre is refused on {lp:?}, got {got:?}"
            );
        }
        // The same placeholder wound ε/2 past a period. Certification
        // reads that overlap Zero at the build's band — a whole turn — and
        // the walk is asked at a band ten times finer, where ε/2 is IN
        // the band: whether it is a whole turn is not known there, so the
        // walk escalates on the span row rather than calling the loop
        // corrupt. (Certification's winding row meters the same `τ − w`
        // at the same lever, so at one band an in-band span never
        // certifies; a finer walk band is the only way in.)
        let eps = band.zero();
        let mut spec = geom_brep::EdgeCurveSpec::self_loop_circle_at(Point3::new(0.0, 0.0, 0.0));
        let over = spec.param_end + 0.5 * eps;
        spec.param_end = over;
        if let geom_brep::EdgeDescriptionSpec::Scaffold(geom_brep::MappedCurve {
            source: geom_brep::MappedSource::RevolvedPoint { ref mut angle, .. },
            ..
        }) = spec.description
        {
            *angle = over;
        }
        body.set_edge_curve(circ.edge, spec, tol)
            .expect("an overlap inside the build's band certifies");
        let fine = Band::new(0.1 * eps, eps).expect("a finer band");
        let got = point_in_loop(&body, circ.r#loop, up, centre, fine);
        assert!(
            matches!(
                &got,
                Err(PointInLoopError::Escalated { r#loop, decision: LoopDecision::ArcSpan, diag, .. })
                    if *r#loop == circ.r#loop
                        && diag.predicate == Some("point_in_arc_loop_conic_span")
            ),
            "an in-band scaffold span escalates on its span row, got {got:?}"
        );
    }

    /// What a conic span reads as, the question `ConicArc::of` asks
    /// ("not over"): wound past a period, escalated (on a margin or a
    /// straddle), or an arc with its turn — short (`Positive`), whole
    /// (`Zero`) or unsettled (`None`).
    #[derive(Clone, Copy, Debug, PartialEq)]
    enum SpanRead {
        Wound,
        Escalated(Escalation),
        Arc(Option<Sign>),
    }

    /// **The oracle**: `ConicArc::of`'s span rule as it was spelled before
    /// the turn rule (`geom_core::period`), bit for bit — generic over the
    /// scalar as it was, so its trigonometry and arithmetic are `Real`'s —
    /// but for the arc it builds.
    fn old_span_rule<T: Decide>(
        carrier: &geom::Curve3<T>,
        (t0, t1): (T, T),
        band: Band,
    ) -> SpanRead {
        use geom_core::Decided;
        use geom_core::k_stats::decide_reported;
        let rows = WALK_ROWS.conic;
        let (ellipse, a, b) = match *carrier {
            geom::Curve3::Circle { radius, .. } => (false, radius, radius),
            geom::Curve3::Ellipse { semi_u, semi_v, .. } => (true, semi_u, semi_v),
            _ => unreachable!("a conic fixture"),
        };
        let lever = a.min(b);
        let gap = T::tau() - (t1 - t0);
        let low = decide_reported(rows.turn, Margin::levered(gap, lever), band);
        if let Ok(Decided {
            sign: Sign::Negative,
            ..
        }) = low
        {
            return SpanRead::Wound;
        }
        let (sm, cm) = (t0 - gap * T::from_f64(0.5)).sin_cos();
        let speed = ((a * sm).powi(2) + (b * cm).powi(2)).sqrt()
            + a.abs().max(b.abs()) * gap.abs() * T::from_f64(0.5);
        if ellipse
            && let Ok(Decided {
                sign: Sign::Negative,
                ..
            }) = decide_reported(rows.turn, Margin::levered(gap, speed), band)
        {
            return SpanRead::Escalated(match low {
                Err(_) => Escalation::Margin,
                Ok(_) => Escalation::Straddle,
            });
        }
        SpanRead::Arc(low.map(|d| d.sign).ok())
    }

    /// The span as `ConicArc::of` now reads it.
    fn new_span_rule(carrier: &geom::Curve3<f64>, span: (f64, f64), band: Band) -> SpanRead {
        match conic_of(carrier, span, WALK_ROWS.conic, band) {
            Err(ConicArcError::WoundPastPeriod) => SpanRead::Wound,
            Err(ConicArcError::Escalated(e)) => SpanRead::Escalated(e.escalation),
            Ok(Some(k)) => SpanRead::Arc(k.span_turn.ok().map(|turn| match turn {
                Within::Short => Sign::Positive,
                Within::Whole => Sign::Zero,
            })),
            Ok(None) => unreachable!("a conic fixture"),
        }
    }

    /// **The turn rule agrees with the span rule it replaced** (the
    /// charter's agreement test), on the cases the rule was written for:
    /// a full turn, and a period over-wound (`+`) or short (`−`) by
    /// `{0.5, 0.9, 3, 20}·ε` of the inf rate's and of the sup rate's
    /// length, on a circle of radius 1 and 0.01 and on an `a = 16, b = 1`
    /// ellipse with the defect centred at its fast (`π/2`) and its slow
    /// (`0`) side. (At `a/b = 16` no case lands on a band edge.)
    ///
    /// **The expected deltas**, each listed below and nowhere hidden:
    /// - on the ellipse's fast side, where the floor `1` reads the defect
    ///   sixteen times short of its length and the inf over the defect
    ///   reads it at about its length: an overlap of `0.9` and `3` of the
    ///   floor's length and `20` of the sup's, which escalated, is wound
    ///   past a period; a short span by as much, which read as a whole
    ///   turn or an unsettled one, reads as an arc short of a turn;
    /// - every span the sup does not read definitely positive while no
    ///   reading settles a turn — a headroom in the band on a circle
    ///   (`3` of either rate, either side) and on the ellipse's slow side,
    ///   or the ellipse's fast side with the inf `Zero` and the sup in the
    ///   band (`0.5` of the inf's, `3` of the sup's) — was an arc and now
    ///   escalates on its margin: "not over" admits an undecided reading
    ///   only where the sup reads the headroom definitely positive
    ///   ([`TurnUndecided::not_over_at_sup`]). A certified edge read at
    ///   its certification's band reaches none of these: check 2 reads
    ///   the same turn and would have escalated it.
    ///
    /// A reversed span no longer reaches the rule: the extent door
    /// refuses it, where the old rule read headroom `τ + |Δ|` and an arc.
    #[test]
    fn the_turn_rule_agrees_with_the_old_conic_span_rule() {
        let band = Band::linear(Tol::witness()).unwrap();
        let eps = band.zero();
        let tau = core::f64::consts::TAU;
        let half_pi = core::f64::consts::FRAC_PI_2;
        let circle = |radius: f64| geom::Curve3::Circle {
            center: Point3::new(0.0, 0.0, 0.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
            radius,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let ellipse = geom::Curve3::Ellipse {
            center: Point3::new(0.0, 0.0, 0.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
            semi_u: 16.0,
            semi_v: 1.0,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        // (name, carrier, defect centre, inf rate, sup rate at the centre)
        let fixtures = [
            ("circle r=1", circle(1.0), 0.0, 1.0, 1.0),
            ("circle r=0.01", circle(0.01), 0.0, 0.01, 0.01),
            ("ellipse fast", ellipse.clone(), half_pi, 1.0, 16.0),
            ("ellipse slow", ellipse, 0.0, 1.0, 1.0),
        ];
        let arc_zero = SpanRead::Arc(Some(Sign::Zero));
        let arc_short = SpanRead::Arc(Some(Sign::Positive));
        let unsettled = SpanRead::Arc(None);
        let wound = SpanRead::Wound;
        // (fixture, rate, k, over) → (old, new)
        let escalated = SpanRead::Escalated(Escalation::Margin);
        let straddle = SpanRead::Escalated(Escalation::Straddle);
        let mut deltas = vec![
            (("ellipse fast", "inf", 0.5, true), (arc_zero, escalated)),
            (("ellipse fast", "inf", 0.5, false), (arc_zero, escalated)),
            (("ellipse fast", "inf", 0.9, true), (straddle, wound)),
            (("ellipse fast", "inf", 0.9, false), (arc_zero, arc_short)),
            (("ellipse fast", "inf", 3.0, true), (escalated, wound)),
            (("ellipse fast", "inf", 3.0, false), (unsettled, arc_short)),
            (("ellipse fast", "sup", 3.0, true), (arc_zero, escalated)),
            (("ellipse fast", "sup", 3.0, false), (arc_zero, escalated)),
            (("ellipse fast", "sup", 20.0, true), (escalated, wound)),
            (("ellipse fast", "sup", 20.0, false), (unsettled, arc_short)),
        ];
        for name in ["circle r=1", "circle r=0.01", "ellipse slow"] {
            for rate in ["inf", "sup"] {
                for over in [true, false] {
                    deltas.push(((name, rate, 3.0, over), (unsettled, escalated)));
                }
            }
        }
        let mut seen = 0;
        for (name, carrier, centre, inf, sup) in &fixtures {
            let full = (*centre, centre + tau);
            assert_eq!(
                new_span_rule(carrier, full, band),
                old_span_rule(carrier, full, band),
                "{name}: a full turn"
            );
            for (rate_name, rate) in [("inf", *inf), ("sup", *sup)] {
                for k in [0.5, 0.9, 3.0, 20.0] {
                    for over in [true, false] {
                        let defect = if over {
                            k * eps / rate
                        } else {
                            -k * eps / rate
                        };
                        let t0 = centre - 0.5 * defect;
                        let span = (t0, t0 + tau + defect);
                        let (old, new) = (
                            old_span_rule(carrier, span, band),
                            new_span_rule(carrier, span, band),
                        );
                        let case = (*name, rate_name, k, over);
                        match deltas.iter().find(|(c, _)| *c == case) {
                            Some((_, want)) => {
                                seen += 1;
                                assert_eq!((old, new), *want, "{case:?}: the listed delta");
                            }
                            None => assert_eq!(new, old, "{case:?}: unlisted"),
                        }
                    }
                }
            }
        }
        assert_eq!(seen, deltas.len(), "every listed delta is a case");
        let reversed = (1.0, 0.0);
        assert_eq!(
            old_span_rule(&circle(1.0), reversed, band),
            SpanRead::Arc(Some(Sign::Positive)),
            "the old rule read a reversed span as an arc"
        );
        assert!(
            matches!(
                Extent::forward(
                    "t_reversed",
                    reversed.0,
                    reversed.1,
                    geom_core::InfSpeed::new(1.0),
                    band
                ),
                Err(geom_core::period::NotForward::Reversed(_))
            ),
            "the extent door refuses it"
        );
    }

    /// **A span moves class over a dense band-edge sweep only from an arc
    /// to a margin escalation** (review R1's row, against the real
    /// `ConicArc`): wound, escalated on a margin, escalated on a straddle,
    /// or an arc — the class the old rule gave every forward span, at three
    /// bands, on circles and on ellipses from `a/b = 1.5` to `1000` stored
    /// either way round, the defect centred around the carrier and sized
    /// in steps of `0.1ε` of three rates. The moves are the listed deltas'
    /// classes in `the_turn_rule_agrees_with_the_old_conic_span_rule`: an
    /// undecided turn the sup does not read definitely short, which the
    /// old rule kept as an arc, escalates on its margin; an overlap the
    /// inf over the defect reads definitely past, which the floor read in
    /// the band or `Zero`, is wound past a period rather than escalated;
    /// and an overlap the floor read `Zero` and the inf over the defect
    /// reads in the band escalates on its margin rather than a straddle.
    /// Each move occurs.
    #[test]
    fn no_conic_span_moves_class_over_a_band_edge_sweep() {
        let class = |read: SpanRead| match read {
            SpanRead::Arc(_) => SpanRead::Arc(None),
            other => other,
        };
        let tau = core::f64::consts::TAU;
        let mut moved = Vec::new();
        for eps in [1e-6, 1e-9, 1e-12] {
            let band = Band::new(eps, 10.0 * eps).unwrap();
            for (a, b, ellipse) in [
                (1.0, 1.0, false),
                (0.01, 0.01, false),
                (250.0, 250.0, false),
                (1.5, 1.0, true),
                (16.0, 1.0, true),
                (20.0, 1.0, true),
                (1.0, 20.0, true),
                (100.0, 1.0, true),
                (1000.0, 1.0, true),
            ] {
                let carrier = if ellipse {
                    geom::Curve3::Ellipse {
                        center: Point3::new(0.0, 0.0, 0.0),
                        axis: Vec3::new(0.0, 0.0, 1.0),
                        semi_u: a,
                        semi_v: b,
                        u_ref: Vec3::new(1.0, 0.0, 0.0),
                    }
                } else {
                    geom::Curve3::Circle {
                        center: Point3::new(0.0, 0.0, 0.0),
                        axis: Vec3::new(0.0, 0.0, 1.0),
                        radius: a,
                        u_ref: Vec3::new(1.0, 0.0, 0.0),
                    }
                };
                for centre in [0.0f64, 0.3, core::f64::consts::FRAC_PI_2, 2.5, -1.2] {
                    let at = ((a * centre.sin()).powi(2) + (b * centre.cos()).powi(2)).sqrt();
                    for rate in [a.min(b), at, a.max(b)] {
                        for i in 0..=240 {
                            for k in [f64::from(i) * 0.1, f64::from(i) * 0.1 + 0.0137] {
                                for sign in [1.0, -1.0] {
                                    let defect = sign * k * eps / rate;
                                    let t0 = centre - 0.5 * defect;
                                    let span = (t0, t0 + tau + defect);
                                    let old = class(old_span_rule(&carrier, span, band));
                                    let new = class(new_span_rule(&carrier, span, band));
                                    if old != new {
                                        moved.push((eps, a, b, centre, k, sign, old, new));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        let (margin, straddle) = (
            SpanRead::Escalated(Escalation::Margin),
            SpanRead::Escalated(Escalation::Straddle),
        );
        let listed = [
            (SpanRead::Arc(None), margin),
            (margin, SpanRead::Wound),
            (straddle, SpanRead::Wound),
            (straddle, margin),
        ];
        let other: Vec<_> = moved
            .iter()
            .filter(|m| !listed.contains(&(m.6, m.7)))
            .collect();
        assert!(other.is_empty(), "{:#?}", &other[..other.len().min(20)]);
        for pair in listed {
            assert!(
                moved.iter().any(|m| (m.6, m.7) == pair),
                "the listed move {pair:?} occurs"
            );
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod spline_cap_rows {
    use super::*;
    use crate::boolean::FaceContainment;
    use crate::entity::FaceKey;
    use crate::euler::{MefSite, MevSite};
    use geom_brep::{EdgeCurveSpec, EdgeDescriptionSpec};
    use geom_core::Tol;

    /// The cubic over `(0, 0), (1, 3), (3, 3), (4, 0)` in the `z = 0`
    /// plane — its top `(2, 2.25)` at `t = ½` — and the cap it closes
    /// with its chord, on a flat spline sheet that holds the cubic as
    /// an iso line. The control net's ball, about `(2, 1.5)` with
    /// radius `2.5`, holds every point the rows below ask.
    fn spline_cap() -> (Body<f64>, FaceKey, LoopKey, EdgeKey, geom::Curve3<f64>) {
        let tol = Tol::witness();
        let p = |x, y| Point3::new(x, y, 0.0);
        let ctrl = [p(0.0, 0.0), p(1.0, 3.0), p(3.0, 3.0), p(4.0, 0.0)];
        let cubic = || {
            geom_core::spline::KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0], 3)
                .unwrap()
        };
        let spline = geom::Curve3::Nurbs(std::sync::Arc::new(
            geom::NurbsCurve3::new(cubic(), ctrl.to_vec(), vec![1.0; 4]).unwrap(),
        ));
        let mut body = Body::<f64>::new();
        let seed = body.mvfs(ctrl[0], true).expect("a seed");
        let sheet = body.add_surface(geom::Surface::Nurbs(std::sync::Arc::new(
            geom::NurbsSurface::new(
                geom_core::spline::KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap(),
                cubic(),
                ctrl.iter()
                    .copied()
                    .chain(ctrl.iter().map(|c| *c - Vec3::unit_y() * 5.0))
                    .collect(),
                vec![1.0; 8],
            )
            .unwrap(),
        )));
        let arc = body
            .mev(
                MevSite::Lone {
                    r#loop: seed.r#loop,
                },
                ctrl[3],
                EdgeCurveSpec {
                    description: EdgeDescriptionSpec::iso(sheet, 0.0, 0.0, 1.0, 0.0, 1.0),
                    carrier: spline.clone(),
                    param_start: 0.0,
                    param_end: 1.0,
                },
                tol,
            )
            .expect("the spline arc");
        let cap = body
            .mef(
                MefSite::Chords {
                    he1: arc.he_minus,
                    he2: arc.he_plus,
                },
                EdgeCurveSpec::line_between(ctrl[3], ctrl[0]),
                crate::euler::FaceSurface::Shared {
                    key: sheet,
                    sense: true,
                },
                tol,
            )
            .expect("the chord closes the cap");
        let outer = body.get_face(cap.face).expect("the cap").outer;
        (body, cap.face, outer, arc.edge, spline)
    }

    /// **A planar face bounded by a spline edge is read inside its
    /// control net's ball**, by the walk and by the boolean's face
    /// containment (whose boundary pre-pass reads the edge through
    /// [`LoopEdge::contact`]): under the top and over the chord is in,
    /// over the top and under the chord out, a point of the arc on it.
    /// Each refused `Uncrossable { carrier: Spline }` before the spline
    /// had a row.
    #[test]
    fn a_spline_cap_is_read_inside_its_control_ball() {
        let band = Band::linear(Tol::witness()).unwrap();
        let (body, face, outer, edge, spline) = spline_cap();
        let z = Vec3::unit_z();
        let on = spline.eval(0.3);
        for (name, q, walk, face_wise) in [
            (
                "under the top",
                Point3::new(2.0, 2.2, 0.0),
                LoopContainment::In,
                FaceContainment::In,
            ),
            (
                "near the chord",
                Point3::new(2.0, 0.1, 0.0),
                LoopContainment::In,
                FaceContainment::In,
            ),
            (
                "near an end",
                Point3::new(0.1, 0.1, 0.0),
                LoopContainment::In,
                FaceContainment::In,
            ),
            (
                "over the top",
                Point3::new(2.0, 2.3, 0.0),
                LoopContainment::Out,
                FaceContainment::Out,
            ),
            (
                "under the chord",
                Point3::new(2.0, -0.5, 0.0),
                LoopContainment::Out,
                FaceContainment::Out,
            ),
            (
                "beside the arc",
                Point3::new(0.2, 1.0, 0.0),
                LoopContainment::Out,
                FaceContainment::Out,
            ),
            (
                "on the arc",
                on,
                LoopContainment::OnBoundary,
                FaceContainment::OnEdge(edge),
            ),
        ] {
            assert_eq!(
                point_in_loop(&body, outer, z, q, band),
                Ok(walk),
                "{name}: the walk"
            );
            assert_eq!(
                crate::boolean::contfp(&body, face, z, q, band),
                Ok(face_wise),
                "{name}: the face's containment"
            );
        }
    }
}

#[cfg(test)]
#[allow(clippy::panic)]
mod arc_ball_rows {
    use super::*;

    /// **A short arc at the end of a long ellipse's major axis has a ball
    /// of its own size.** A seam split `10⁻⁷` off a unit wall's tangent
    /// plane cuts an ellipse of semi-major `10⁷`, of which the side keeps
    /// the arc about `v` up to height 1. Its carrier's ball is `10⁷`
    /// across; its arc's holds each of its points and is under 3 m.
    #[test]
    fn a_short_arc_of_a_long_ellipse_has_a_ball_of_its_own_size() {
        let (a, b) = (1e7, 1.0);
        let center = Point3::new(1.0, 0.0, 0.0) + Vec3::new(0.0, 0.0, a);
        let carrier = geom::Curve3::Ellipse {
            center,
            axis: Vec3::new(1.0, 0.0, 0.0),
            semi_u: a,
            semi_v: b,
            u_ref: Vec3::new(0.0, 0.0, -1.0),
        };
        let span = (-4.5e-4, 4.5e-4);
        let Some((mid, radius)) = conic_arc_ball(&carrier, span) else {
            panic!("an ellipse's arc has a ball")
        };
        assert!(radius < 3.0, "the arc's ball is of its size: {radius}");
        for k in 0..=64 {
            let t = span.0 + (span.1 - span.0) * f64::from(k) / 64.0;
            let gap = (carrier.eval(t) - mid).norm();
            assert!(
                gap <= radius,
                "the arc's point at {t} lies {gap} out, past {radius}"
            );
        }
    }
}
