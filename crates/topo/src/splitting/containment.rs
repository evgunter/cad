//! Trilean **point-in-loop** containment — the ring re-homing decision
//! (ch. 14's `laringmv`, Problem 13.5) and the seed of F8's 3-D
//! containment machinery. Kept general: a planar loop plus a query
//! point in its plane → In / Out / OnBoundary, with typed escalation —
//! **no unbanded raw comparisons** (every decision is a named K
//! predicate through the Q1 funnel).
//!
//! # Method: in-plane ray parity with a deterministic retry schedule
//!
//! The walk itself — the boundary pre-pass and one ray's parity count
//! — is [`crate::ray_parity`], shared with the 2-D chart-space
//! consumer. What this module owns is the *3-D* half: reading the
//! loop's vertex cycle out of the body, and turning each member of
//! the space-direction schedule into an in-plane frame.
//!
//! Cast a ray from `q` in a direction lying in the loop's plane and
//! count proper crossings; odd ⇒ In. Grazing configurations (an
//! endpoint on the ray line, a crossing at the query point) are not
//! errors of the geometry but of the *ray choice*: retry with the next
//! direction of a fixed schedule (profile's golden-angle device,
//! promoted to 3-D). The schedule directions are built from a fixed
//! table of space directions projected into the plane — no basis
//! choice by coordinate comparison anywhere (a comparison-picked basis
//! could diverge between the f64 and interval lanes; a fixed schedule
//! whose degenerate members are *detected by predicate* cannot):
//! direction k is `r_k − n(n·r_k)` for the k-th schedule vector `r_k`,
//! accepted iff the in-plane displacement it commands at the loop's
//! own scale — `(|r_k − n(n·r_k)| / |r_k|) · extent`, not the bare
//! projected length — is definitely positive (**`point_in_loop_arm`**;
//! a near-parallel `r_k` is skipped, and a loop collapsed onto `q`
//! zeroes the arm for *every* member and ends in `RayExhausted`).
//!
//! # Predicates (all K-tagged, meters)
//!
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
//!   the ray line — Zero ⇒ grazing ⇒ next ray.
//! - **`point_in_loop_advance`**: the crossing's advance along the
//!   ray — Zero would mean a crossing at `q` itself (contradicting the
//!   boundary pre-pass) ⇒ next ray, escalating if persistent.
//!
//! The arc-aware walk ([`point_in_carrier_loop`]) and the one boundary
//! reading of an edge on its carrier ([`LoopEdge::contact`]) carry their
//! own rows, each caller naming them through a [`BoundaryRows`] value
//! (the walk's are `WALK_ROWS`; `boolean::contain`'s pre-pass passes
//! `bool_contact_*` names):
//!
//! - **`point_in_arc_loop_{segment,boundary,side,advance}`**: the four
//!   rows above, over the loop's STRAIGHT edges.
//! - **`point_in_arc_loop_arm`**: the schedule gate, with the conics'
//!   and balls' reach in the extent.
//! - **`point_in_arc_loop_reach`**: a ray's clearance from an
//!   uncrossable (spiric, spline) edge's ball, less its reach — anything
//!   but definitely clear abandons the ray.
//! - **`point_in_arc_loop_conic_span`**: a conic arc's gap to a full
//!   period, `(τ − w)` levered by the smaller semi-axis — read only for a
//!   window wound definitely PAST a period, which is no edge. Anything
//!   else is an arc, a whole turn included (its two ends coincide).
//! - **`point_in_arc_loop_conic_on`**: the distance from the conic. A
//!   circle: exact through the radius. An ellipse: ON on an upper bound
//!   (a point of the ellipse near the foot), OFF on a lower bound (the
//!   Hessian-bounded root) — [`ConicArc::hit`].
//! - **`point_in_arc_loop_conic_end`**: the distance to either end of the
//!   arc ([`arc_trim`]'s step 1). On the boundary, a circle's is the unit
//!   chord times the radius and an ellipse's the exact distance from the
//!   point to the end point; on a ray's crossing, the unit chord times
//!   the smaller semi-axis.
//! - **`point_in_arc_loop_conic_trim`**: the chordal-defect sum saying
//!   which side of the ends a BOUNDARY point lies, levered by the radius
//!   or, on an ellipse, the larger semi-axis.
//! - **`point_in_arc_loop_conic_window`**, **`_disc`**, **`_advance`**: a
//!   ray's crossing of a conic — the distance trim's defect sum
//!   ([`arc_trim`]), the discriminant, the root's advance — each levered
//!   by the smaller semi-axis, where a `Zero` or an in-band margin only
//!   abandons the ray.
//!
//! Two escalation names never reach the funnel
//! ([`crate::invalid_margin`]): **`point_in_arc_loop_conic_straddle`**, an
//! ellipse's two bounds on its distance straddling the whole band, and
//! **`point_in_arc_loop_boundary_disagreement`**, the walk meeting on an
//! edge a point its caller's pass placed off it.

use geom_core::{Band, Decide, Indeterminate, Margin, Point3, Sign, Vec3};

use crate::body::Body;
use crate::entity::{EdgeKey, LoopBoundary, LoopKey};
use crate::ray_parity::{self, ParityRows};
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

/// Typed failure of [`point_in_loop`].
#[derive(Debug)]
pub enum PointInLoopError {
    /// A predicate escalated (in-band margin).
    Escalated {
        /// The loop being tested.
        r#loop: LoopKey,
        /// The escalation diagnostics (named predicate inside).
        diag: Indeterminate,
    },
    /// Every schedule ray grazed — the loop/point pair is
    /// ill-conditioned at this ε (profile's `RayCastingExhausted`).
    RayExhausted {
        /// The loop being tested.
        r#loop: LoopKey,
    },
    /// The loop is not a walkable cycle of resolvable geometry.
    CorruptLoop {
        /// The loop.
        r#loop: LoopKey,
    },
}

impl core::fmt::Display for PointInLoopError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Escalated { diag, .. } => {
                write!(
                    f,
                    "whether a point lies in a loop is too close to call: {diag}"
                )
            }
            Self::RayExhausted { .. } => write!(
                f,
                "every test ray grazed the loop, so containment is ill-conditioned at \
                 this tolerance"
            ),
            Self::CorruptLoop { r#loop } => {
                write!(f, "loop {loop:?} is not walkable")
            }
        }
    }
}

impl std::error::Error for PointInLoopError {}

/// The fixed schedule of space directions (module docs). 16 entries:
/// three axes plus golden-angle-spread oblique members — for any unit
/// plane normal, most members project to a definitely-nonzero in-plane
/// direction (an all-graze outcome is the typed exhaustion error).
///
/// **The one table**, and no consumer owns it: this module's in-plane
/// ray parity (which projects each triple into the loop plane and
/// skips the near-parallel members), [`crate::splitting::order`]'s
/// point ordering, and [`crate::boolean::solid_contain`]'s
/// containment sweep (which normalizes the raw triple). They do not
/// sweep the same directions. What each needs is only that its own
/// schedule is a `const` in a fixed order every run, which holds per
/// site — so a single definition buys the absence of drift between
/// copies, not determinism, which was never at risk.
///
/// **Editing an entry is not free, and what catches you is
/// incidental.** `tests/review_m3_pr3_pil.rs` hand-copies the fifteen
/// non-degenerate `+z` in-plane projections of this table (values),
/// and `splitting::order`'s sort tests pin the iteration order. Both
/// pin it as a side effect of testing something else; nothing pins it
/// on purpose.
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

/// Trilean containment of `q` in the region bounded by `r#loop`, which
/// must be a planar polygon (line carriers — the F5 regime) with unit
/// plane normal `normal`; `q` is assumed to lie in the loop's plane
/// (the ring re-homing contract: ring and outer share one face plane).
///
/// # The sign of `normal`
///
/// `normal` is read only to recover the loop's PLANE, so a caller may
/// hand over a raw CHART normal without folding in the face's sense —
/// `chord_join::face_plane_normal` does exactly that. The
/// invariance is derived here because it is this walk's property, not
/// that producer's.
///
/// Each schedule member is projected as `r − n̂(n̂·r)`, which is
/// invariant under `n̂ ↦ −n̂`, and the parity walk then runs in the
/// in-plane frame `(d, n̂ × d)` — whose second axis is the only thing
/// a sign flip moves. Negating that axis negates every vertex ordinate
/// `y`, so the straddle test `sign(yᵢ) ≠ sign(yⱼ)` is unchanged, the
/// vertex-on-the-ray `Zero` graze is unchanged, and the crossing's
/// advance `(xᵢyⱼ − xⱼyᵢ)/(yⱼ − yᵢ)` has numerator and denominator
/// both negated. Negation is exact and both `sign_within` classifiers
/// are symmetric about zero, so **the verdict is bit-identical either
/// way**, and a refusal is identical in variant, predicate and band.
///
/// One thing is NOT identical, and saying so is what keeps the
/// sentence above true: an escalation carries the **signed** margin it
/// refused on, so the two signs refuse with `MarginKind::Value(−m)`
/// against `Value(m)`. That is diagnostic payload —
/// [`geom_core::Indeterminate`]'s own docs call its fields *"honest
/// diagnostic data … for actionable error messages and later margin
/// telemetry"*, and nothing in this walk reads a margin back — but a
/// differential test comparing whole `Debug` renderings would see
/// it.
/// `topo/tests/review_m3_pr3_pil.rs`'s
/// `the_verdict_is_blind_to_the_normals_sign` pins all of this, and
/// compares variant, predicate and band rather than the rendering.
///
/// # Errors
///
/// [`PointInLoopError`] — escalation, ray exhaustion, or an
/// unwalkable loop.
pub fn point_in_loop<T: Decide>(
    body: &Body<T>,
    r#loop: LoopKey,
    normal: Vec3<T>,
    q: Point3<T>,
    band: Band,
) -> Result<LoopContainment, PointInLoopError> {
    let escalate = |diag| PointInLoopError::Escalated { r#loop, diag };
    let points = loop_points(body, r#loop)?;

    if ray_parity::on_boundary(&points, q, &ROWS, band).map_err(escalate)? {
        return Ok(LoopContainment::OnBoundary);
    }
    polygon_walk(r#loop, &points, normal, q, band)
}

/// [`point_in_loop`]'s ray walk alone, for a point its pre-pass (or a
/// caller's) has placed off the boundary: `In` or `Out`.
fn polygon_walk<T: Decide>(
    r#loop: LoopKey,
    points: &[Point3<T>],
    normal: Vec3<T>,
    q: Point3<T>,
    band: Band,
) -> Result<LoopContainment, PointInLoopError> {
    let escalate = |diag| PointInLoopError::Escalated { r#loop, diag };
    // The loop's own reach from q (evaluation-lane fold): the lever
    // arm for the probe-direction gate below. A degenerate loop
    // collapsed onto q gives a zero arm, every schedule member skips,
    // and the walk ends in the typed `RayExhausted` — fail-loud.
    let mut extent = T::zero();
    for p in points {
        extent = extent.max((*p - q).norm());
    }

    walk_schedule(
        r#loop,
        normal,
        extent,
        "point_in_loop_arm",
        ArmBand::Escalate,
        band,
        |d, side_axis| {
            ray_parity::ray_verdict(points, q, d, side_axis, &ROWS, band).map_err(escalate)
        },
    )
}

/// **Ray parity over the fixed schedule, in a loop's plane** — the
/// driver both walks here share: each schedule member projected into
/// the plane, gated on the in-plane displacement it commands at the
/// loop's own `extent` (`arm_row`), and handed to `ray` as the in-plane
/// frame `(d, n̂ × d)`. `ray` answers `Some(inside)` or `None` for a
/// graze; exhaustion is the loop's typed `RayExhausted`.
fn walk_schedule<T: Decide>(
    r#loop: LoopKey,
    normal: Vec3<T>,
    extent: T,
    arm_row: &'static str,
    arm_band: ArmBand,
    band: Band,
    mut ray: impl FnMut(Vec3<T>, Vec3<T>) -> Result<Option<bool>, PointInLoopError>,
) -> Result<LoopContainment, PointInLoopError> {
    for r in &SCHEDULE {
        let r = r.map(T::from_f64);
        let n_dot_r = normal.dot(r);
        let d_raw = r - normal * n_dot_r;
        // sin(schedule member, plane NORMAL) × loop extent — the
        // member's in-plane fraction |d_raw|/|r|: the SCHEDULE triples
        // are bare numbers, so the raw projected norm was a
        // dimensionless comparand against the length band
        // (rim-dimensional audit, class (c)); the honest margin is
        // the in-plane displacement the probe direction commands at
        // the loop's own scale.
        let arm = Margin::levered(d_raw.norm() / r.norm(), extent);
        match decide(arm_row, arm, band) {
            Ok(Sign::Positive) => {}
            Ok(_) => continue, // near-parallel schedule member: skip
            Err(_) if arm_band == ArmBand::Retry => continue,
            Err(diag) => return Err(PointInLoopError::Escalated { r#loop, diag }),
        }
        let d = d_raw.normalize();
        let side_axis = normal.cross(d); // in-plane ⟂, unit
        if let Some(inside) = ray(d, side_axis)? {
            return Ok(if inside {
                LoopContainment::In
            } else {
                LoopContainment::Out
            });
        }
    }
    Err(PointInLoopError::RayExhausted { r#loop })
}

/// What [`walk_schedule`] does with an in-band margin on its arm row,
/// which is about one schedule MEMBER and not about the point.
#[derive(Clone, Copy, PartialEq, Eq)]
enum ArmBand {
    /// Escalate — [`point_in_loop`]'s posture, which its consumers read.
    Escalate,
    /// Skip that member, as a near-parallel one is skipped.
    Retry,
}

/// The arc-bearing walk's K rows over a loop's STRAIGHT edges — the
/// boundary pre-pass and the straddle count, [`ParityRows`]' four —
/// kept apart from [`point_in_loop`]'s so the two walks' populations
/// stay separable. Its own rows, written at their `decide` calls:
/// `point_in_arc_loop_arm` (the schedule gate), `point_in_arc_loop_reach`
/// (the loop's bounding ball, for an edge with no crossing row), and the
/// conic arm's `point_in_arc_loop_conic_{span,on,window,disc,advance}`
/// and its pre-pass trim's `point_in_arc_loop_conic_{end,trim}`.
const ARC_LOOP_ROWS: ParityRows = ParityRows {
    segment: "point_in_arc_loop_segment",
    boundary: "point_in_arc_loop_boundary",
    side: "point_in_arc_loop_side",
    advance: "point_in_arc_loop_advance",
};

/// The rows a caller reads a conic edge under — its own, so each
/// caller's population stays separable in the telemetry.
#[derive(Clone, Copy)]
pub(crate) struct ConicRows {
    /// The arc's width against a period (`τ − w`, levered).
    pub(crate) span: &'static str,
    /// The point's distance from the conic.
    pub(crate) on: &'static str,
    /// The distance from the point to either end of the arc.
    pub(crate) end: &'static str,
    /// The chordal-defect sum that says which side of the ends it is.
    pub(crate) trim: &'static str,
    /// The name an ellipse's escalation carries where a lower and an
    /// upper bound on one quantity straddle the whole band — no margin
    /// of `on` or `span` says that, so it is not charged to them
    /// ([`crate::invalid_margin`]). It never reaches the funnel.
    pub(crate) straddle: &'static str,
}

/// The rows a caller reads a loop's boundary under: a straight edge's
/// ([`ray_parity::on_segment`]'s two) and a conic edge's.
#[derive(Clone, Copy)]
pub(crate) struct BoundaryRows {
    /// A straight edge's rows.
    pub(crate) line: &'static ParityRows,
    /// A conic edge's rows.
    pub(crate) conic: ConicRows,
}

/// [`point_in_carrier_loop`]'s rows for a loop's boundary.
const WALK_ROWS: BoundaryRows = BoundaryRows {
    line: &ARC_LOOP_ROWS,
    conic: ConicRows {
        span: "point_in_arc_loop_conic_span",
        on: "point_in_arc_loop_conic_on",
        end: "point_in_arc_loop_conic_end",
        trim: "point_in_arc_loop_conic_trim",
        straddle: "point_in_arc_loop_conic_straddle",
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
    /// buys. It levers the CROSSING rows, where a `Zero` abandons a ray
    /// and an understated margin costs rays, never an answer. A verdict
    /// on an ellipse reads it only where a LOWER bound is the sound side
    /// — the span rule's wound-past-period ([`Self::of`]); the boundary
    /// reading bounds its distances on both sides ([`Self::hit`]), and
    /// an arc's span needs the larger semi-axis too.
    lever: T,
    /// The carrier parameters of the arc's two ends.
    span: (T, T),
    /// The arc's two ends and its apex (the window's mid direction),
    /// on the unit circle — the trim [`arc_trim`] decides by distance.
    trim: [(T, T); 3],
}

/// Why a conic carrier gives no [`ConicArc`].
pub(crate) enum ConicArcError {
    /// The window winds definitely past a period: no edge at all.
    WoundPastPeriod,
    /// An ellipse's overlap past a period is neither definitely within
    /// the band nor definitely past it ([`ConicArc::of`]'s span rule).
    Escalated(Indeterminate),
}

/// Where a point sits against one conic edge.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ConicHit {
    /// Definitely off the conic.
    Off,
    /// On the conic, definitely off the arc and clear of both its ends.
    Carrier,
    /// On the arc, definitely clear of both ends.
    On,
    /// On the conic within the band of one of the arc's ends.
    End,
}

impl<T: Decide> ConicArc<T> {
    /// The arc a certified conic carrier spans over `(t0, t1)`; `None`
    /// for a carrier that is not a circle or an ellipse.
    ///
    /// **The span rule, two-sided.** Let `Δ = τ − w`: a gap to a whole
    /// turn where positive, a doubled stretch of carrier (an OVERLAP)
    /// where negative. Its length along the carrier is `|Δ|` times a
    /// speed between the smaller semi-axis and the larger — exact on a
    /// circle, where the two are one. The band is not symmetric about a
    /// verdict: `Zero` is `|m| ≤ ε`, an escalation `ε < |m| < 10ε`, and a
    /// sign only beyond `10ε`. So each verdict reads the lever that makes
    /// it sound:
    ///
    /// - **Wound past a period** — no edge at all — only where `Δ·min`,
    ///   a LOWER bound on the overlap's length, is definitely negative.
    /// - **An arc** only where `Δ` times an UPPER bound on the edge's
    ///   speed over the overlap (at most the larger semi-axis; the bound
    ///   is at the site) is not definitely negative: any overlap is then within
    ///   the escalation band along the edge, which is the slack a circle
    ///   has always had (a `Zero` or an in-band `Δ·r` is an arc), and
    ///   points on it lie within that band of the edge's vertex. A gap,
    ///   of any length, is an arc like any other: the distance trim reads
    ///   a whole circle, a near-whole one and a short one alike — on a
    ///   whole turn its two ends coincide at the edge's one vertex — so
    ///   no gap is read as closed.
    /// - **Between the two** — an ellipse whose overlap is definitely
    ///   longer than the band by its upper bound but not by its lower —
    ///   the span escalates: on the smaller lever's own margin
    ///   where it has one, on `rows.straddle` where the two bounds
    ///   straddle the band. Reading it as an arc would put a stretch of
    ///   the edge on both sides of its own trim, and a point ON it would
    ///   read off the edge.
    fn of(
        carrier: &geom::Curve3<T>,
        (t0, t1): (T, T),
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
                major,
                minor,
                u_ref,
            } => (ConicKind::Ellipse, center, axis, u_ref, major, minor),
            _ => return Ok(None),
        };
        let lever = a.min(b);
        let gap = T::tau() - (t1 - t0);
        let low = decide(rows.span, Margin::levered(gap, lever), band);
        if let Ok(Sign::Negative) = low {
            return Err(ConicArcError::WoundPastPeriod);
        }
        // A circle's two levers are one, and its lower bound has spoken.
        // An ellipse's upper lever is the edge's speed over the overlap
        // itself: `|P′(θ)| = √((a·sin θ)² + (b·cos θ)²)` at its middle
        // `m = t0 − Δ/2`, plus `a·|Δ|/2` for the stretch either side
        // (the speed changes at most as fast as `|P″| ≤ a`) — never less
        // than the speed anywhere on the overlap, and tight as it
        // shrinks, so a certified window (its ends pinned within the
        // band of its vertex) is never read as over-wound.
        let (sm, cm) = (t0 - gap * T::from_f64(0.5)).sin_cos();
        let speed = ((a * sm).powi(2) + (b * cm).powi(2)).sqrt() + a * gap.abs() * T::from_f64(0.5);
        if kind == ConicKind::Ellipse
            && let Ok(Sign::Negative) = decide(rows.span, Margin::levered(gap, speed), band)
        {
            return Err(ConicArcError::Escalated(match low {
                Err(diag) => diag,
                _ => crate::invalid_margin::invalid(band, rows.straddle),
            }));
        }
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
            span: (t0, t1),
            trim: [(c0, s0), (c1, s1), (cm, sm)],
        }))
    }

    fn unit(&self, x: Point3<T>) -> (T, T) {
        let w = x - self.center;
        (w.dot(self.u) / self.a, w.dot(self.v) / self.b)
    }

    /// The carrier's point at parameter `t`.
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

    /// **Where `q` sits against this edge.**
    ///
    /// **A circle**: its unit coordinates are an isometry scaled by the
    /// radius, so every reading is levered into metres exactly — the
    /// distance from the circle, `√(((ρ − 1)·r)² + axial²)`, then the
    /// arc's trim ([`arc_trim`], its `Zero` the band of an end).
    ///
    /// **An ellipse**: no single lever is exact. The smaller semi-axis
    /// understates a distance by up to `b/a` and the larger overstates it
    /// by up to `a/b`, so an `ON` verdict taken through the one and an
    /// `OFF` verdict through the other would each be wrong somewhere.
    /// The distance `d` from `q`'s in-plane image `Q` to the ellipse is
    /// bounded on both sides instead, and each verdict reads the bound
    /// that makes it sound:
    ///
    /// - **Lower**, `d ≥ 2|F| / (g + √(g² + 4|F|/b²))`, for
    ///   `F = X²/a² + Y²/b² − 1` (zero on the ellipse) and `g = |∇F(Q)|`:
    ///   along the segment from `Q` to its foot, `F` falls to zero while
    ///   `|∇F|` grows at most at the Hessian's norm `2/b²`, so
    ///   `|F| ≤ g·d + d²/b²`, whose positive root is the bound.
    /// - **Upper**, `d ≤ |Q − P|` for any point `P` of the ellipse; `P` is
    ///   one Newton step of `F` from `Q` (along `∇F`) snapped radially
    ///   onto the ellipse in unit coordinates. Both bounds approach `d`
    ///   as `d → 0`; they part by more than the band's own ratio only
    ///   where the ellipse bends tighter than the band resolves (its
    ///   smallest radius of curvature `b²/a` within a few `ε`).
    ///
    /// `OFF` only where the LOWER bound clears the escalation band; `ON`
    /// the carrier only where the UPPER bound is within the zero band;
    /// between them the answer escalates. The out-of-plane miss
    /// `(q − c)·n̂` folds into both. On the carrier, an end is read as the
    /// EXACT distance from `q` to the end point — never a unit chord — and
    /// the side of the ends from the foot `P` (within the band of `q`, so
    /// on the same side of an end that is more than the escalation band
    /// from `q`), by [`arc_trim_margin`] levered by the LARGER semi-axis:
    /// a unit angle costs at most `a` metres of arc, so the margin still
    /// bounds the arc length to the nearer end from above.
    fn hit(&self, q: Point3<T>, rows: ConicRows, band: Band) -> Result<ConicHit, Indeterminate> {
        let (x, y) = self.unit(q);
        let axial = (q - self.center).dot(self.axis);
        let (ends, apex, anti) = self.trim_points();
        if self.kind == ConicKind::Circle {
            let rho = (x.powi(2) + y.powi(2)).sqrt();
            let miss = (((rho - T::one()) * self.lever).powi(2) + axial.powi(2)).sqrt();
            match decide(rows.on, Margin::of(miss), band)? {
                Sign::Zero => {}
                Sign::Positive => return Ok(ConicHit::Off),
                Sign::Negative => return Err(crate::invalid_margin::invalid(band, rows.on)),
            }
            let trim = ArcTrimRows {
                end: rows.end,
                trim: rows.trim,
            };
            return Ok(
                match arc_trim(
                    Self::lift((x / rho, y / rho)),
                    ends,
                    apex,
                    anti,
                    self.lever,
                    &trim,
                    band,
                )? {
                    Sign::Zero => ConicHit::End,
                    Sign::Positive => ConicHit::On,
                    Sign::Negative => ConicHit::Carrier,
                },
            );
        }
        let two = T::from_f64(2.0);
        let (a, b) = (self.a, self.b);
        let f = x.powi(2) + y.powi(2) - T::one();
        let (gx, gy) = (two * x / a, two * y / b);
        let g2 = gx.powi(2) + gy.powi(2);
        let g = g2.sqrt();
        // The LOWER bound first: it is finite everywhere — at the centre,
        // where `∇F` vanishes, it is `b` exactly, the true distance — and
        // a definite OFF needs nothing else.
        let lower = two * f.abs() / (g + (g2 + T::from_f64(4.0) * f.abs() / b.powi(2)).sqrt());
        let lower = (lower.powi(2) + axial.powi(2)).sqrt();
        let far = decide(rows.on, Margin::of(lower), band);
        match far {
            Ok(Sign::Positive) => return Ok(ConicHit::Off),
            Ok(Sign::Negative) => return Err(crate::invalid_margin::invalid(band, rows.on)),
            Ok(Sign::Zero) | Err(_) => {}
        }
        // Within the escalation band of the conic by the lower bound, so
        // `∇F` is nonzero here and the Newton foot is defined.
        let (px, py) = (x * a - f * gx / g2, y * b - f * gy / g2);
        let (ux, uy) = (px / a, py / b);
        let r = (ux.powi(2) + uy.powi(2)).sqrt();
        let foot = (ux / r, uy / r);
        let upper = ((x * a - foot.0 * a).powi(2) + (y * b - foot.1 * b).powi(2)).sqrt();
        let upper = (upper.powi(2) + axial.powi(2)).sqrt();
        // ON only on the upper bound. Using the lower bound here instead
        // cannot be told apart on a body whose ellipse bends looser than
        // the band — the two bounds agree to within the band's own ratio
        // there — and is wrong on one that bends tighter
        // (`tests::an_ellipse_tighter_than_the_band_straddles_it`).
        match (decide(rows.on, Margin::of(upper), band), far) {
            (Ok(Sign::Zero), _) => {}
            (Ok(Sign::Negative), _) => return Err(crate::invalid_margin::invalid(band, rows.on)),
            (_, Err(diag)) | (Err(diag), _) => return Err(diag),
            // The lower bound within the zero band, the upper definitely
            // beyond it: the two straddle the whole band, which only an
            // ellipse bending tighter than the band resolves (`b²/a`
            // within a few `ε`) allows.
            (Ok(Sign::Positive), _) => {
                return Err(crate::invalid_margin::invalid(band, rows.straddle));
            }
        }
        let (t0, t1) = self.span;
        let at = [
            decide(rows.end, Margin::norm3(q - self.point(t0)), band),
            decide(rows.end, Margin::norm3(q - self.point(t1)), band),
        ];
        if at.iter().any(|e| matches!(e, Ok(Sign::Zero))) {
            return Ok(ConicHit::End);
        }
        for end in at {
            match end? {
                Sign::Positive => {}
                Sign::Zero | Sign::Negative => {
                    return Err(crate::invalid_margin::invalid(band, rows.end));
                }
            }
        }
        let side = arc_trim_margin(Self::lift(foot), ends[0], apex, anti);
        Ok(
            match decide(rows.trim, Margin::levered(side, a.max(b)), band)? {
                // `Zero` is unreachable: both ends are definitely more than
                // `10ε` from `q` and the foot is within `ε` of `q`, so the
                // foot is more than `9ε` of arc from either end, and the
                // margin, levered by the larger semi-axis, is at least that
                // arc length. It reads as on, as a circle's does, should
                // rounding reach it.
                Sign::Positive | Sign::Zero => ConicHit::On,
                Sign::Negative => ConicHit::Carrier,
            },
        )
    }

    /// Is the unit-circle point `(x, y)` inside the arc's window?
    /// [`arc_trim`] in the arc's unit coordinates, levered by the
    /// smaller semi-axis: Positive inside, Negative outside, Zero an
    /// endpoint's neighbourhood. Only a ray's crossing reads it, where a
    /// `Zero` or an in-band margin abandons the ray, so the lever's
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
/// where a `Zero` abandons the ray.
const ARC_LOOP_TRIM: ArcTrimRows = ArcTrimRows {
    end: "point_in_arc_loop_conic_end",
    trim: "point_in_arc_loop_conic_window",
};

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
/// the one place that says why, for this trim and for the chart
/// windows' cosine construction (`boolean::solid_contain`'s
/// `chart_azimuth_margin`) alike. That door answers "may hold" wherever
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
fn arc_trim_margin<T: Decide>(p: Point3<T>, a: Point3<T>, apex: Point3<T>, anti: Point3<T>) -> T {
    ((a - apex).norm() - (p - apex).norm()) + ((p - anti).norm() - (a - anti).norm())
}

/// One boundary edge of a planar loop, on its own carrier.
pub(crate) enum LoopEdge<T: geom_core::Real> {
    /// A line — which IS its chord — or null scaffolding, a zero-length
    /// coincident copy.
    Chord,
    /// A circle or ellipse arc.
    Conic(ConicArc<T>),
    /// A carrier with no crossing row (a spiric, a spline), carried as
    /// a ball its whole locus lies in: `|x − center| ≤ reach`.
    Unrowed { center: Point3<T>, reach: T },
}

/// Where a point sits against one boundary edge — [`LoopEdge::contact`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum EdgeContact {
    /// Definitely off the edge.
    Off,
    /// On a conic's carrier, definitely off the arc and clear of its
    /// ends — off the edge, and a fact the crossing count needs.
    Carrier,
    /// On the edge, clear of a conic's ends.
    On,
    /// Within the band of one of a conic's two ends.
    End,
    /// An edge on a carrier with no row (a spiric, a spline): this pass
    /// cannot say, and the region walk holds it as a ball.
    Unread,
}

impl<T: Decide> LoopEdge<T> {
    /// **The one boundary reading of an edge** `a → b`, shared by every
    /// point-in-loop door: a straight edge by the distance to its closed
    /// segment ([`ray_parity::on_segment`]), a conic by
    /// [`ConicArc::hit`], anything else unread.
    pub(crate) fn contact(
        &self,
        (a, b): (Point3<T>, Point3<T>),
        q: Point3<T>,
        rows: BoundaryRows,
        band: Band,
    ) -> Result<EdgeContact, Indeterminate> {
        Ok(match self {
            Self::Chord => {
                if ray_parity::on_segment(a, b, q, rows.line, band)? {
                    EdgeContact::On
                } else {
                    EdgeContact::Off
                }
            }
            Self::Conic(k) => match k.hit(q, rows.conic, band)? {
                ConicHit::Off => EdgeContact::Off,
                ConicHit::Carrier => EdgeContact::Carrier,
                ConicHit::On => EdgeContact::On,
                ConicHit::End => EdgeContact::End,
            },
            Self::Unrowed { .. } => EdgeContact::Unread,
        })
    }

    /// A conic edge's two end points (at its start and end parameters).
    pub(crate) fn conic_ends(&self) -> Option<[Point3<T>; 2]> {
        let Self::Conic(k) = self else {
            return None;
        };
        let (t0, t1) = k.span;
        Some([k.point(t0), k.point(t1)])
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
}

/// The loop's vertices and its edges on their carriers, a conic's span
/// read on `rows.conic`.
pub(crate) fn carrier_loop<T: Decide>(
    body: &Body<T>,
    r#loop: LoopKey,
    rows: BoundaryRows,
    band: Band,
) -> Result<CarrierLoop<T>, PointInLoopError> {
    let corrupt = || PointInLoopError::CorruptLoop { r#loop };
    let LoopBoundary::Cycle { first } = body.get_loop(r#loop).ok_or_else(corrupt)?.boundary else {
        return Err(corrupt());
    };
    let mut verts = Vec::new();
    let mut keys = Vec::new();
    let mut edges = Vec::new();
    for he in body.loop_cycle(first).ok_or_else(corrupt)? {
        let h = body.get_half_edge(he).ok_or_else(corrupt)?;
        let point = body.get_vertex(h.start).ok_or_else(corrupt)?.point;
        verts.push(*body.get_point(point).ok_or_else(corrupt)?);
        keys.push(h.edge);
        let edge = body.get_edge(h.edge).ok_or_else(corrupt)?;
        // A curve key that does not resolve is corruption; a resolved
        // entry that is null scaffolding is a zero-length chord.
        let Some(curve) = body
            .get_curve_geom(edge.curve)
            .ok_or_else(corrupt)?
            .certified()
        else {
            edges.push(LoopEdge::Chord);
            continue;
        };
        let (t0, t1) = curve.params();
        let carrier = curve.carrier();
        match ConicArc::of(carrier, (t0, t1), rows.conic, band) {
            Ok(Some(k)) => {
                edges.push(LoopEdge::Conic(k));
                continue;
            }
            Ok(None) => {}
            Err(ConicArcError::WoundPastPeriod) => return Err(corrupt()),
            Err(ConicArcError::Escalated(diag)) => {
                return Err(PointInLoopError::Escalated { r#loop, diag });
            }
        }
        edges.push(match carrier {
            geom::Curve3::Line { .. } => LoopEdge::Chord,
            geom::Curve3::Circle { .. } | geom::Curve3::Ellipse { .. } => {
                unreachable!("a circle or an ellipse is a conic arc above")
            }
            _ => {
                let (center, reach) = carrier_ball(carrier, (t0, t1)).ok_or_else(corrupt)?;
                LoopEdge::Unrowed { center, reach }
            }
        });
    }
    Ok(CarrierLoop { verts, keys, edges })
}

/// **A ball holding the arc `span` of `carrier`**, `(center, radius)`,
/// read off the carrier's own data with no decision: a conic within its
/// larger semi-axis of its centre, a spiric oval and a spline as
/// [`LoopEdge::Unrowed`] carries them. `None` for a line, whose segment
/// its two end vertices hold, and for a spline with no control points.
fn carrier_ball<T: Decide>(
    carrier: &geom::Curve3<T>,
    (t0, t1): (T, T),
) -> Option<(Point3<T>, T)> {
    match *carrier {
        geom::Curve3::Line { .. } => None,
        geom::Curve3::Circle { center, radius, .. } => Some((center, radius)),
        geom::Curve3::Ellipse {
            center,
            major,
            minor,
            ..
        } => Some((center, major.max(minor))),
        // The arc from its midpoint: the oval's speed is at most
        // `r(R − r)/√((R − r)² − offset²)` (the carrier's own doc),
        // so no point of it lies further from `P(mid)` than that
        // times half the parameter width.
        geom::Curve3::Spiric {
            major_radius,
            minor_radius,
            offset,
            ..
        } => {
            let inner = major_radius - minor_radius;
            let speed = minor_radius * inner / (inner.powi(2) - offset.powi(2)).sqrt();
            Some((
                carrier.mid_point(t0, t1),
                speed * (t1 - t0).abs() * T::from_f64(0.5),
            ))
        }
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

/// **How far from `q` the loop reaches**: the radius of a ball about
/// `q` holding every vertex of `loop` and every edge's whole carrier
/// arc ([`carrier_ball`]). It asks no decision, so it cannot refuse on
/// geometry; it over-estimates (a conic's ball is its carrier's, not its
/// arc's), which is the direction its callers need.
///
/// # Errors
///
/// [`PointInLoopError::CorruptLoop`] for a loop that does not walk.
pub(crate) fn loop_extent_from<T: Decide>(
    body: &Body<T>,
    r#loop: LoopKey,
    q: Point3<T>,
) -> Result<T, PointInLoopError> {
    let corrupt = || PointInLoopError::CorruptLoop { r#loop };
    let first = match body.get_loop(r#loop).ok_or_else(corrupt)?.boundary {
        LoopBoundary::Cycle { first } => first,
        LoopBoundary::Empty { vertex } => {
            let point = body.get_vertex(vertex).ok_or_else(corrupt)?.point;
            return Ok((*body.get_point(point).ok_or_else(corrupt)? - q).norm());
        }
    };
    let mut extent = T::zero();
    for he in body.loop_cycle(first).ok_or_else(corrupt)? {
        let h = body.get_half_edge(he).ok_or_else(corrupt)?;
        let point = body.get_vertex(h.start).ok_or_else(corrupt)?.point;
        extent = extent.max((*body.get_point(point).ok_or_else(corrupt)? - q).norm());
        let edge = body.get_edge(h.edge).ok_or_else(corrupt)?;
        let Some(curve) = body
            .get_curve_geom(edge.curve)
            .ok_or_else(corrupt)?
            .certified()
        else {
            continue;
        };
        if let Some((center, reach)) = carrier_ball(curve.carrier(), curve.params()) {
            extent = extent.max((center - q).norm() + reach);
        }
    }
    Ok(extent)
}

/// Whether the walk reads the boundary itself, or its caller has.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Boundary {
    /// Answer `OnBoundary` where the point is on an edge.
    Verdict,
    /// The caller has decided the point is definitely off every edge of
    /// the loop through [`LoopEdge::contact`]; the walk reads a conic
    /// only for whether the point is on its CARRIER, which the crossing
    /// count needs.
    Decided,
}

/// **A planar loop's region, read on its edges' own carriers** — the
/// arc-aware sibling of [`point_in_loop`], whose polygon through the
/// vertices is the region only when every edge is a line. An arc moves
/// region across its chord: a revolved cap is a half-disc whose three
/// vertices are COLLINEAR, an extruded disc's cap has two, and in both
/// the polygon has no area; an arc bowing into a polygon puts region
/// the loop does not bound inside it, and one bowing out leaves region
/// outside it.
///
/// - **Every edge a line**: [`point_in_loop`], unchanged.
/// - **Circle and ellipse arcs** (with lines): ray parity over the same
///   schedule ([`walk_schedule`]), the straight edges counted by
///   [`ray_parity::ray_crossings`] and each arc crossed on its conic —
///   in the arc's unit coordinates the conic is the unit circle, the
///   ray a line, and a crossing a root of `|P + D·s|² = 1` inside the
///   arc's window. The discriminant is taken in its perpendicular-offset
///   form `1 − h²` (`h` the unit-coordinate line's distance from the
///   centre), which does not cancel far from a small conic. The
///   boundary pre-pass is [`LoopEdge::contact`] — never an arc's chord,
///   which is not boundary. A point on an arc's conic but off the arc
///   skips its own `s = 0` root. Every graze — a vertex on the ray line,
///   a ray tangent to a conic, a root at an arc's endpoint, a zero
///   advance — abandons the ray.
/// - **An edge on any other carrier** (a spiric, a spline): no crossing
///   row exists, so such an edge is held as a ball its locus lies in
///   (the arc's own, from its midpoint and its speed bound, for a
///   spiric; the control hull's, for a spline), and a ray that could
///   meet that ball — or whose clearance from it lands in the band — is
///   abandoned like a graze. The rest of the loop answers along any
///   scheduled ray that definitely misses every such ball; `None` — the
///   caller's refusal — only where none does, which includes every point
///   inside a ball.
///
/// `q` must lie in the loop's plane, whose unit normal is `normal`.
///
/// # Errors
///
/// [`PointInLoopError`] — an escalation, exhaustion, or an unwalkable
/// loop.
pub(crate) fn point_in_carrier_loop<T: Decide>(
    body: &Body<T>,
    r#loop: LoopKey,
    normal: Vec3<T>,
    q: Point3<T>,
    band: Band,
) -> Result<Option<LoopContainment>, PointInLoopError> {
    let lp = carrier_loop(body, r#loop, WALK_ROWS, band)?;
    if lp.edges.iter().all(|e| matches!(e, LoopEdge::Chord)) {
        return point_in_loop(body, r#loop, normal, q, band).map(Some);
    }
    carrier_walk(r#loop, &lp, normal, q, band, Boundary::Verdict).map(|side| {
        side.map(|s| match s {
            WalkSide::In => LoopContainment::In,
            WalkSide::Out => LoopContainment::Out,
            WalkSide::OnBoundary => LoopContainment::OnBoundary,
        })
    })
}

/// [`point_in_carrier_loop`] for a caller whose own boundary pass —
/// [`LoopEdge::contact`] over every edge of `lp` — has already decided
/// `q` is definitely off every edge: `Some(true)` inside, `Some(false)`
/// outside, `None` where an uncrossable edge stands in the way of every
/// ray. The walk's boundary pass is not run, and a crossing at `q`
/// itself, which that precondition rules out, is a graze.
///
/// # Errors
///
/// As [`point_in_carrier_loop`].
pub(crate) fn carrier_loop_side<T: Decide>(
    r#loop: LoopKey,
    lp: &CarrierLoop<T>,
    normal: Vec3<T>,
    q: Point3<T>,
    band: Band,
) -> Result<Option<bool>, PointInLoopError> {
    if lp.edges.iter().all(|e| matches!(e, LoopEdge::Chord)) {
        return Ok(Some(
            polygon_walk(r#loop, &lp.verts, normal, q, band)? == LoopContainment::In,
        ));
    }
    Ok(carrier_walk(r#loop, lp, normal, q, band, Boundary::Decided)?.map(|s| s == WalkSide::In))
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
    boundary: Boundary,
) -> Result<Option<WalkSide>, PointInLoopError> {
    let escalate = |diag| PointInLoopError::Escalated { r#loop, diag };
    let (verts, edges) = (&lp.verts, &lp.edges);
    // The loop's reach from `q`: the schedule gate's lever.
    let extent = reach_from(verts, edges, q);
    let balls: Vec<_> = edges
        .iter()
        .filter_map(|e| match *e {
            LoopEdge::Unrowed { center, reach } => Some((center, reach)),
            _ => None,
        })
        .collect();
    let n = verts.len();
    // ---- Boundary pass, and which conics carry `q`. ----
    let mut on_carrier = vec![false; n];
    for (i, edge) in edges.iter().enumerate() {
        if let (Boundary::Decided, LoopEdge::Chord | LoopEdge::Unrowed { .. }) = (boundary, edge) {
            continue;
        }
        match edge
            .contact((verts[i], verts[(i + 1) % n]), q, WALK_ROWS, band)
            .map_err(escalate)?
        {
            EdgeContact::Off | EdgeContact::Unread => {}
            EdgeContact::Carrier => on_carrier[i] = true,
            EdgeContact::On | EdgeContact::End => match boundary {
                Boundary::Verdict => return Ok(Some(WalkSide::OnBoundary)),
                // The caller's pass ran the same arithmetic and placed `q`
                // off this edge. The two agree whenever every decision is
                // the band's own; they can part only where a decision was
                // taken on something else — the test-only identity pass,
                // which reads an in-band margin as `Zero`, is one such —
                // and then the point is in the band of this edge, which
                // is an escalation, never a panic.
                Boundary::Decided => {
                    return Err(escalate(crate::invalid_margin::invalid(
                        band,
                        "point_in_arc_loop_boundary_disagreement",
                    )));
                }
            },
        }
    }
    // ---- The rays. ----
    // WHY A RAY-LEVEL MARGIN RETRIES (the one home of this argument).
    // Past the boundary pass, every row is a fact about ONE RAY — which
    // schedule member, where it meets a vertex's line, a conic, an arc's
    // end, an uncrossable edge's ball — and not about `q`: the boundary
    // pass (this walk's own, or its caller's) has decided `q` off every
    // edge and arc by more than the band, which bounds any crossing's
    // advance `t` away from zero, so no in-band margin on a ray can be the
    // question "is `q` on the boundary". And a ray's parity is used only
    // when EVERY row on it is decisive, so abandoning one — exactly as a
    // graze is abandoned — can only turn an escalation into an answer or
    // into `RayExhausted`, never into a wrong verdict. Only the boundary
    // pass's rows, which ask where `q` itself stands, escalate.
    let mut blocked = false;
    let walked = walk_schedule(
        r#loop,
        normal,
        extent,
        "point_in_arc_loop_arm",
        ArmBand::Retry,
        band,
        |d, side_axis| {
            // A ray that could meet an uncrossable edge's ball answers
            // nothing: `|w − d·max(w·d, 0)|` is the ray's distance from
            // the ball's centre (`w = c − q`), taken without a branch. A
            // clearance in the band is a ray that COULD meet it, and is
            // abandoned like any graze rather than escalating the walk.
            for &(center, reach) in &balls {
                let w = center - q;
                let nearest = w - d * w.dot(d).max(T::zero());
                if !matches!(
                    decide(
                        "point_in_arc_loop_reach",
                        Margin::of(nearest.norm() - reach),
                        band
                    ),
                    Ok(Sign::Positive)
                ) {
                    blocked = true;
                    return Ok(None);
                }
            }
            let Ok(Some(mut crossings)) =
                ray_parity::ray_crossings(verts, q, d, side_axis, &ARC_LOOP_ROWS, band, |i| {
                    matches!(edges[i], LoopEdge::Chord)
                })
            else {
                return Ok(None);
            };
            for (i, edge) in edges.iter().enumerate() {
                let LoopEdge::Conic(k) = *edge else {
                    continue;
                };
                match conic_crossings(k, on_carrier[i], q, d, band) {
                    Some(c) => crossings += c,
                    None => return Ok(None),
                }
            }
            Ok(Some(!crossings.is_multiple_of(2)))
        },
    );
    match walked {
        Ok(LoopContainment::In) => Ok(Some(WalkSide::In)),
        Ok(_) => Ok(Some(WalkSide::Out)),
        // An uncrossable edge stood in the way of some ray: the loop
        // could not be read there, which is the caller's refusal rather
        // than an exhausted schedule.
        Err(PointInLoopError::RayExhausted { .. }) if blocked => Ok(None),
        Err(e) => Err(e),
    }
}

/// The radius of a ball about `from` that holds the whole loop: every
/// vertex, and every edge's carrier locus through the bound its
/// [`LoopEdge`] carries (a conic within its larger semi-axis of its
/// centre, an unrowed carrier within its own reach).
fn reach_from<T: Decide>(verts: &[Point3<T>], edges: &[LoopEdge<T>], from: Point3<T>) -> T {
    let mut ball = T::zero();
    for v in verts {
        ball = ball.max((*v - from).norm());
    }
    for edge in edges {
        let (center, reach) = match *edge {
            LoopEdge::Chord => continue,
            LoopEdge::Conic(k) => (k.center, k.a.max(k.b)),
            LoopEdge::Unrowed { center, reach } => (center, reach),
        };
        ball = ball.max((center - from).norm() + reach);
    }
    ball
}

/// **A ball holding the whole loop**, `(center, radius)`: the loop's
/// first vertex and the reach [`point_in_carrier_loop`] confines its
/// refusal with. Nothing here needs the loop to be planar — each edge
/// is bounded on its own carrier — so a curved face's outer loop is
/// served the same way.
///
/// # Errors
///
/// [`PointInLoopError`] — an unwalkable loop, or a conic span that
/// escalates.
pub(crate) fn loop_reach<T: Decide>(
    body: &Body<T>,
    r#loop: LoopKey,
    band: Band,
) -> Result<(Point3<T>, T), PointInLoopError> {
    let lp = carrier_loop(body, r#loop, WALK_ROWS, band)?;
    let anchor = *lp
        .verts
        .first()
        .ok_or(PointInLoopError::CorruptLoop { r#loop })?;
    Ok((anchor, reach_from(&lp.verts, &lp.edges, anchor)))
}

/// How many times the ray `q + d·t`, `t > 0`, crosses the arc `k` —
/// `None` for a graze, and for an in-band margin on any of its rows
/// (why that is sound: the ray loop in [`carrier_walk`]). `on_carrier` says the pre-pass put `q` on the
/// conic and off the arc, so a root at `q` itself is no crossing.
fn conic_crossings<T: Decide>(
    k: ConicArc<T>,
    on_carrier: bool,
    q: Point3<T>,
    d: Vec3<T>,
    band: Band,
) -> Option<usize> {
    let (px, py) = k.unit(q);
    let (dx, dy) = (d.dot(k.u) / k.a, d.dot(k.v) / k.b);
    // `d` is unit and in the conic's plane, so `(dx, dy)` is nonzero.
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
    )
    .ok()?
    {
        Sign::Positive => {}
        Sign::Zero => return None, // tangent to the conic
        Sign::Negative => return Some(0),
    }
    let root = disc.max(T::zero()).sqrt();
    let mut crossings = 0;
    for s in [T::zero() - along - root, T::zero() - along + root] {
        // Back to metres along the unit ray.
        let t = s / dn;
        match decide("point_in_arc_loop_conic_advance", Margin::of(t), band).ok()? {
            Sign::Positive => {}
            Sign::Negative => continue,
            Sign::Zero if on_carrier => continue,
            // A crossing at `q` the pre-pass did not see.
            Sign::Zero => return None,
        }
        let (hx, hy) = (px + dx * t, py + dy * t);
        let hn = (hx.powi(2) + hy.powi(2)).sqrt();
        match k.in_window((hx / hn, hy / hn), band).ok()? {
            Sign::Positive => crossings += 1,
            Sign::Negative => {}
            // An endpoint's neighbourhood. The endpoint is a vertex of
            // the loop, and a ray through a vertex has already grazed on
            // its side row, so this arm answers only where that row and
            // this window disagree inside the band — a graze all the
            // same, never a count.
            Sign::Zero => return None,
        }
    }
    Some(crossings)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
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
            let Ok(Some(k)) = ConicArc::of(&carrier, (t0, t1), WALK_ROWS.conic, band) else {
                panic!("a circle arc under a period");
            };
            let ask = |q| k.hit(q, WALK_ROWS.conic, band).expect("decided");
            for s in [20.0 * eps, 50.0 * eps, 500.0 * eps, 5000.0 * eps] {
                let d = s / r;
                assert_eq!(
                    ask(at(t1 - d)),
                    ConicHit::On,
                    "w = {t1}: {s} m inside the end"
                );
                assert_eq!(
                    ask(at(t1 + d)),
                    ConicHit::Carrier,
                    "w = {t1}: {s} m past the end"
                );
                assert_eq!(
                    ask(at(t0 + d)),
                    ConicHit::On,
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
            major: a,
            minor: b,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        // The arc from the minor vertex (π/2) round through the major
        // vertex at π to 3π/2.
        let (t0, t1) = (core::f64::consts::FRAC_PI_2, 1.5 * core::f64::consts::PI);
        let Ok(Some(k)) = ConicArc::of(&carrier, (t0, t1), WALK_ROWS.conic, band) else {
            panic!("an ellipse arc under a period");
        };
        for s in [12.0, 15.0, 40.0] {
            for side in [1.0, -1.0] {
                let q = Point3::new(-a - side * s * eps, 0.0, 0.0);
                let got = k.hit(q, WALK_ROWS.conic, band);
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
            let got = k.hit(q, WALK_ROWS.conic, band);
            assert_eq!(
                got,
                Ok(ConicHit::On),
                "{s}ε along the arc from its end: on the arc, neither its end nor an escalation"
            );
        }
    }

    /// The steep ellipse of [`a_steep_ellipse_reads_the_band_in_metres`].
    fn steep(band: Band, (t0, t1): (f64, f64)) -> Result<Option<ConicArc<f64>>, ConicArcError> {
        let carrier = geom::Curve3::Ellipse {
            center: Point3::new(0.0, 0.0, 0.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
            major: 20.0,
            minor: 1.0,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        ConicArc::of(&carrier, (t0, t1), WALK_ROWS.conic, band)
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
        let mid_gap = k.hit(k.point(core::f64::consts::FRAC_PI_2), WALK_ROWS.conic, band);
        assert!(
            !matches!(mid_gap, Ok(ConicHit::On)),
            "the gap is not the arc: {mid_gap:?}"
        );
        for s in [5.0, 11.0] {
            // `s·ε` of arc into the gap from the arc's end at `t1`.
            let past = k.hit(k.point(t1 + s * eps / 20.0), WALK_ROWS.conic, band);
            assert!(
                !matches!(past, Ok(ConicHit::On)),
                "{s}ε past the end, in the gap: {past:?}"
            );
        }
        assert_eq!(k.hit(k.point(1.0), WALK_ROWS.conic, band), Ok(ConicHit::On));
        let Ok(Some(full)) = steep(band, (0.0, core::f64::consts::TAU)) else {
            panic!("the full ellipse is read");
        };
        for t in [0.3, core::f64::consts::FRAC_PI_2, 3.0, 5.0] {
            assert_eq!(
                full.hit(full.point(t), WALK_ROWS.conic, band),
                Ok(ConicHit::On),
                "a full period is on the arc at {t}"
            );
        }
        assert_eq!(
            full.hit(full.point(0.0), WALK_ROWS.conic, band),
            Ok(ConicHit::End)
        );
    }

    /// **An over-wound ellipse window is not read as an arc.** The review's
    /// repro: `a = 14, b = 1`, a window of a full period plus a `30ε`
    /// overlap at the minor vertex (where the edge runs at speed `a`).
    /// Levered by `b` alone the overlap read `2.1ε`, in band and not
    /// negative, so the window read as an arc and a point ON the doubled
    /// stretch — `15ε` from each end — read `Carrier`, off the edge. The
    /// span now needs the LARGER lever for an arc: the overlap is `30ε`
    /// there, definitely past the band, and the span escalates.
    #[test]
    fn an_over_wound_ellipse_window_is_not_an_arc() {
        let band = Band::linear(Tol::witness()).unwrap();
        let eps = band.zero();
        let ellipse = |a: f64| geom::Curve3::Ellipse {
            center: Point3::new(0.0, 0.0, 0.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
            major: a,
            minor: 1.0,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        // The repro, as the review spelled it at ε = 1e-9.
        if eps == 1e-9 {
            let got = ConicArc::of(
                &ellipse(14.0),
                (1.570796325723468, 7.853981635045912),
                WALK_ROWS.conic,
                band,
            );
            assert!(
                matches!(got, Err(ConicArcError::Escalated(_))),
                "the repro reads as an arc"
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
                    let got = ConicArc::of(&ellipse(a), span, WALK_ROWS.conic, band);
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
        // A window certification can pin to one vertex — its overlap within
        // `2ε` of arc — is an arc, at the MAJOR vertex too, where the
        // larger semi-axis would overstate that overlap twentyfold.
        for a in [20.0f64, 100.0] {
            let dt = 2.0 * eps;
            let got = ConicArc::of(
                &ellipse(a),
                (-0.5 * dt, core::f64::consts::TAU + 0.5 * dt),
                WALK_ROWS.conic,
                band,
            );
            assert!(
                matches!(got, Ok(Some(_))),
                "a/b {a}: a 2ε overlap at the major vertex is an arc"
            );
        }
        // A circle is unchanged: an overlap in its band is an arc, one past
        // it wound.
        let circle = geom::Curve3::Circle {
            center: Point3::new(0.0, 0.0, 0.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
            radius: 1.0,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let tau = core::f64::consts::TAU;
        assert!(matches!(
            ConicArc::of(&circle, (0.0, tau + 5.0 * eps), WALK_ROWS.conic, band),
            Ok(Some(_))
        ));
        assert!(matches!(
            ConicArc::of(&circle, (0.0, tau + 30.0 * eps), WALK_ROWS.conic, band),
            Err(ConicArcError::WoundPastPeriod)
        ));
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
        assert_eq!(
            k.hit(Point3::new(0.0, 0.0, 0.0), WALK_ROWS.conic, band),
            Ok(ConicHit::Off)
        );
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
            major: p(20.0),
            minor: p(1.0),
            u_ref: Vec3::new(p(1.0), p(0.0), p(0.0)),
        };
        let Ok(Some(k)) = ConicArc::of(
            &carrier,
            (p(0.0), p(core::f64::consts::PI)),
            WALK_ROWS.conic,
            band,
        ) else {
            panic!("the arc is read");
        };
        k_stats::start_recording();
        let got = k.hit(Point3::new(p(0.0), p(0.0), p(0.0)), WALK_ROWS.conic, band);
        let samples = k_stats::take_samples();
        assert_eq!(got, Ok(ConicHit::Off));
        assert!(
            samples.iter().all(|s| s.outcome != SampleOutcome::Invalid),
            "{samples:?}"
        );
    }

    /// **An ellipse bending tighter than the band** (`b²/a = ε/100`): at
    /// its major vertex the lower bound on a point `20ε` out reads within
    /// the zero band while the upper bound — here the distance itself —
    /// reads definitely beyond it. The answer escalates on its own
    /// straddle name; it is never `On`, which reading ON off the lower
    /// bound would give.
    #[test]
    fn an_ellipse_tighter_than_the_band_straddles_it() {
        let band = Band::linear(Tol::witness()).unwrap();
        let eps = band.zero();
        let (a, b) = (1.0, (eps / 100.0).sqrt());
        let carrier = geom::Curve3::Ellipse {
            center: Point3::new(0.0, 0.0, 0.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
            major: a,
            minor: b,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let Ok(Some(k)) = ConicArc::of(
            &carrier,
            (0.0, core::f64::consts::TAU),
            WALK_ROWS.conic,
            band,
        ) else {
            panic!("the full ellipse is read");
        };
        let got = k.hit(Point3::new(a + 20.0 * eps, 0.0, 0.0), WALK_ROWS.conic, band);
        assert!(
            matches!(&got, Err(d) if d.predicate == Some("point_in_arc_loop_conic_straddle")),
            "{got:?}"
        );
    }
}
