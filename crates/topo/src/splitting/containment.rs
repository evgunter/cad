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

use geom_core::{Band, Decide, Indeterminate, Margin, Point3, Sign, Vec3};

use crate::body::Body;
use crate::entity::{LoopBoundary, LoopKey};
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
pub(crate) const SCHEDULE: [[f64; 3]; 16] = [
    [1.0, 0.0, 0.0],
    [0.0, 1.0, 0.0],
    [0.0, 0.0, 1.0],
    [0.5, 0.25, 1.0],
    [1.0, 0.5, 0.25],
    [0.25, 1.0, 0.5],
    [-0.5, 1.0, 0.125],
    [0.125, -0.5, 1.0],
    [1.0, 0.125, -0.5],
    [0.75, -1.0, 0.375],
    [0.375, 0.75, -1.0],
    [-1.0, 0.375, 0.75],
    [0.625, 0.9375, 0.3125],
    [0.3125, -0.625, 0.9375],
    [0.9375, 0.3125, -0.625],
    [-0.75, -0.25, 1.0],
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
/// refused on, so the two signs refuse with `MarginDiag::Value(−m)`
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

    // The loop's own reach from q (evaluation-lane fold): the lever
    // arm for the probe-direction gate below. A degenerate loop
    // collapsed onto q gives a zero arm, every schedule member skips,
    // and the walk ends in the typed `RayExhausted` — fail-loud.
    let mut extent = T::zero();
    for p in &points {
        extent = extent.max((*p - q).norm());
    }

    walk_schedule(
        r#loop,
        normal,
        extent,
        "point_in_loop_arm",
        band,
        |d, side_axis| {
            ray_parity::ray_verdict(&points, q, d, side_axis, &ROWS, band).map_err(escalate)
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
    band: Band,
    mut ray: impl FnMut(Vec3<T>, Vec3<T>) -> Result<Option<bool>, PointInLoopError>,
) -> Result<LoopContainment, PointInLoopError> {
    for r in &SCHEDULE {
        let r = Vec3::new(T::from_f64(r[0]), T::from_f64(r[1]), T::from_f64(r[2]));
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
        match decide(arm_row, arm, band)
            .map_err(|diag| PointInLoopError::Escalated { r#loop, diag })?
        {
            Sign::Positive => {}
            _ => continue, // near-parallel schedule member: skip
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

/// The arc-bearing walk's K rows over a loop's STRAIGHT edges — the
/// boundary pre-pass and the straddle count, [`ParityRows`]' four —
/// kept apart from [`point_in_loop`]'s so the two walks' populations
/// stay separable. Its own rows, written at their `decide` calls:
/// `point_in_arc_loop_arm` (the schedule gate), `point_in_arc_loop_reach`
/// (the loop's bounding ball, for an edge with no crossing row), and the
/// conic arm's `point_in_arc_loop_conic_{span,on,window,disc,advance}`.
const ARC_LOOP_ROWS: ParityRows = ParityRows {
    segment: "point_in_arc_loop_segment",
    boundary: "point_in_arc_loop_boundary",
    side: "point_in_arc_loop_side",
    advance: "point_in_arc_loop_advance",
};

/// A conic arc of a planar loop — a circle or an ellipse — in its own
/// affine frame: the locus `center + u·a·cos θ + v·b·sin θ` over
/// `θ ∈ [t0, t1]` (for an ellipse `θ` is the eccentric anomaly, the
/// carrier's own parameter). In UNIT coordinates `((x−c)·u/a,
/// (x−c)·v/b)` the conic is the unit circle and the arc a window of
/// it, so one set of rows serves both kinds.
#[derive(Clone, Copy)]
struct ConicArc<T: geom_core::Real> {
    center: Point3<T>,
    u: Vec3<T>,
    v: Vec3<T>,
    a: T,
    b: T,
    /// The smaller semi-axis: the displacement one unit-coordinate step
    /// buys is at least this, so every unit-coordinate margin is levered
    /// by it — exact for a circle, conservative (escalating more, never
    /// less) for an ellipse.
    lever: T,
    /// The window's mid direction `(cos, sin)` and `cos(width / 2)`;
    /// `None` when the arc spans a whole period.
    window: Option<((T, T), T)>,
}

impl<T: Decide> ConicArc<T> {
    fn unit(&self, x: Point3<T>) -> (T, T) {
        let w = x - self.center;
        (w.dot(self.u) / self.a, w.dot(self.v) / self.b)
    }

    /// Is the unit-circle direction `(x, y)` inside the arc's window?
    /// The cosine-window construction (`solid_contain::point_on_wall_in_face`
    /// carries its argument): Positive inside, Negative outside, Zero an
    /// endpoint's neighbourhood.
    fn in_window(&self, (x, y): (T, T), band: Band) -> Result<Sign, Indeterminate> {
        let Some(((mc, ms), cos_half)) = self.window else {
            return Ok(Sign::Positive);
        };
        decide(
            "point_in_arc_loop_conic_window",
            Margin::levered(x * mc + y * ms - cos_half, self.lever),
            band,
        )
    }
}

/// One boundary edge of a planar loop, on its own carrier.
enum LoopEdge<T: geom_core::Real> {
    /// A line — which IS its chord — or null scaffolding, a zero-length
    /// coincident copy.
    Chord,
    /// A circle or ellipse arc.
    Conic(ConicArc<T>),
    /// A carrier with no crossing row (a spiric, a spline), carried as
    /// a ball its whole locus lies in: `|x − center| ≤ reach`.
    Unrowed { center: Point3<T>, reach: T },
}

/// The loop's vertices and its edges on their carriers.
fn carrier_loop<T: Decide>(
    body: &Body<T>,
    r#loop: LoopKey,
    band: Band,
) -> Result<(Vec<Point3<T>>, Vec<LoopEdge<T>>), PointInLoopError> {
    let corrupt = || PointInLoopError::CorruptLoop { r#loop };
    let LoopBoundary::Cycle { first } = body.get_loop(r#loop).ok_or_else(corrupt)?.boundary else {
        return Err(corrupt());
    };
    let mut verts = Vec::new();
    let mut edges = Vec::new();
    for he in body.loop_cycle(first).ok_or_else(corrupt)? {
        let h = body.get_half_edge(he).ok_or_else(corrupt)?;
        let point = body.get_vertex(h.start).ok_or_else(corrupt)?.point;
        verts.push(*body.get_point(point).ok_or_else(corrupt)?);
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
        let conic = |center: Point3<T>, axis: Vec3<T>, u: Vec3<T>, a: T, b: T| {
            let lever = a.min(b);
            let width = t1 - t0;
            let window = match decide(
                "point_in_arc_loop_conic_span",
                Margin::levered(T::tau() - width, lever),
                band,
            )
            .map_err(|diag| PointInLoopError::Escalated { r#loop, diag })?
            {
                Sign::Positive => {
                    let half = T::from_f64(0.5);
                    let (ms, mc) = ((t0 + t1) * half).sin_cos();
                    let (_, cos_half) = (width * half).sin_cos();
                    Some(((mc, ms), cos_half))
                }
                Sign::Zero => None,
                // A window wound past a period is no edge at all.
                Sign::Negative => return Err(corrupt()),
            };
            Ok(LoopEdge::Conic(ConicArc {
                center,
                u,
                v: axis.cross(u),
                a,
                b,
                lever,
                window,
            }))
        };
        edges.push(match curve.carrier() {
            geom::Curve3::Line { .. } => LoopEdge::Chord,
            &geom::Curve3::Circle {
                center,
                axis,
                radius,
                u_ref,
            } => conic(center, axis, u_ref, radius, radius)?,
            &geom::Curve3::Ellipse {
                center,
                axis,
                major,
                minor,
                u_ref,
            } => conic(center, axis, u_ref, major, minor)?,
            // The spiric lies on its torus, so within `R + r` of the
            // torus centre.
            &geom::Curve3::Spiric {
                center,
                major_radius,
                minor_radius,
                ..
            } => LoopEdge::Unrowed {
                center,
                reach: major_radius + minor_radius,
            },
            // Positive weights put a NURBS curve inside its control
            // hull, so within the control points' reach from the first.
            geom::Curve3::Nurbs(n) => {
                let control = n.control();
                let center = *control.first().ok_or_else(corrupt)?;
                let mut reach = T::zero();
                for p in control {
                    reach = reach.max((*p - center).norm());
                }
                LoopEdge::Unrowed { center, reach }
            }
        });
    }
    Ok((verts, edges))
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
///   boundary pre-pass asks a straight edge [`ray_parity::on_segment`]
///   and an arc its own carrier and window — never an arc's chord,
///   which is not boundary. A point on an arc's conic but off the arc
///   skips its own `s = 0` root. Every graze — a vertex on the ray line,
///   a ray tangent to a conic, a root at an arc's endpoint, a zero
///   advance — abandons the ray.
/// - **An edge on any other carrier** (a spiric, a spline): no crossing
///   row exists, so the loop is answered only where no crossing could
///   matter — `q` definitely outside a ball holding the whole loop is
///   `Out` — and `None` everywhere else: the caller's refusal, confined
///   to points the loop could actually bound.
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
    let escalate = |diag| PointInLoopError::Escalated { r#loop, diag };
    let (verts, edges) = carrier_loop(body, r#loop, band)?;
    if edges.iter().all(|e| matches!(e, LoopEdge::Chord)) {
        return point_in_loop(body, r#loop, normal, q, band).map(Some);
    }
    // The loop's reach, from its first vertex and from `q`.
    let (anchor, mut ball, mut extent) = (verts[0], T::zero(), T::zero());
    for v in &verts {
        ball = ball.max((*v - anchor).norm());
        extent = extent.max((*v - q).norm());
    }
    for edge in &edges {
        let (center, reach) = match *edge {
            LoopEdge::Chord => continue,
            LoopEdge::Conic(k) => (k.center, k.a.max(k.b)),
            LoopEdge::Unrowed { center, reach } => (center, reach),
        };
        ball = ball.max((center - anchor).norm() + reach);
        extent = extent.max((center - q).norm() + reach);
    }
    if edges.iter().any(|e| matches!(e, LoopEdge::Unrowed { .. })) {
        let gap = (q - anchor).norm() - ball;
        return Ok(
            match decide("point_in_arc_loop_reach", Margin::of(gap), band).map_err(escalate)? {
                Sign::Positive => Some(LoopContainment::Out),
                Sign::Zero | Sign::Negative => None,
            },
        );
    }
    let n = verts.len();
    // ---- Boundary pre-pass, and which conics carry `q`. ----
    let mut on_carrier = vec![false; n];
    for (i, edge) in edges.iter().enumerate() {
        match *edge {
            LoopEdge::Chord => {
                if ray_parity::on_segment(verts[i], verts[(i + 1) % n], q, &ARC_LOOP_ROWS, band)
                    .map_err(escalate)?
                {
                    return Ok(Some(LoopContainment::OnBoundary));
                }
            }
            LoopEdge::Conic(k) => {
                let (x, y) = k.unit(q);
                let rho = (x * x + y * y).sqrt();
                if decide(
                    "point_in_arc_loop_conic_on",
                    Margin::levered(rho - T::one(), k.lever),
                    band,
                )
                .map_err(escalate)?
                    != Sign::Zero
                {
                    continue;
                }
                on_carrier[i] = true;
                // On the arc, or in an endpoint's neighbourhood — a
                // vertex of this loop.
                if k.in_window((x / rho, y / rho), band).map_err(escalate)? != Sign::Negative {
                    return Ok(Some(LoopContainment::OnBoundary));
                }
            }
            // Answered by the reach test above: never walked.
            LoopEdge::Unrowed { .. } => {}
        }
    }
    walk_schedule(
        r#loop,
        normal,
        extent,
        "point_in_arc_loop_arm",
        band,
        |d, side_axis| {
            let Some(mut crossings) =
                ray_parity::ray_crossings(&verts, q, d, side_axis, &ARC_LOOP_ROWS, band, |i| {
                    matches!(edges[i], LoopEdge::Chord)
                })
                .map_err(escalate)?
            else {
                return Ok(None);
            };
            for (i, edge) in edges.iter().enumerate() {
                let LoopEdge::Conic(k) = *edge else {
                    continue;
                };
                match conic_crossings(k, on_carrier[i], q, d, band).map_err(escalate)? {
                    Some(c) => crossings += c,
                    None => return Ok(None),
                }
            }
            Ok(Some(!crossings.is_multiple_of(2)))
        },
    )
    .map(Some)
}

/// How many times the ray `q + d·t`, `t > 0`, crosses the arc `k` —
/// `None` for a graze. `on_carrier` says the pre-pass put `q` on the
/// conic and off the arc, so a root at `q` itself is no crossing.
fn conic_crossings<T: Decide>(
    k: ConicArc<T>,
    on_carrier: bool,
    q: Point3<T>,
    d: Vec3<T>,
    band: Band,
) -> Result<Option<usize>, Indeterminate> {
    let (px, py) = k.unit(q);
    let (dx, dy) = (d.dot(k.u) / k.a, d.dot(k.v) / k.b);
    // `d` is unit and in the conic's plane, so `(dx, dy)` is nonzero.
    let dn = (dx * dx + dy * dy).sqrt();
    let (ex, ey) = (dx / dn, dy / dn);
    let along = px * ex + py * ey;
    let (hx, hy) = (px - ex * along, py - ey * along);
    let disc = T::one() - (hx * hx + hy * hy);
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
        let hn = (hx * hx + hy * hy).sqrt();
        match k.in_window((hx / hn, hy / hn), band)? {
            Sign::Positive => crossings += 1,
            Sign::Negative => {}
            Sign::Zero => return Ok(None), // at an arc endpoint
        }
    }
    Ok(Some(crossings))
}
