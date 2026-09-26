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
    /// buys. It levers the CROSSING rows only, where a `Zero` abandons a
    /// ray and an understated margin costs rays, never an answer. A
    /// boundary verdict, where a `Zero` IS the answer, never reads it
    /// for an ellipse ([`Self::hit`]).
    lever: T,
    /// The carrier parameters of the arc's two ends; `None` when the arc
    /// spans a whole period.
    window: Option<(T, T)>,
}

/// Why a conic carrier gives no [`ConicArc`].
pub(crate) enum ConicArcError {
    /// The width-against-a-period row landed in the band.
    Escalated(Indeterminate),
    /// The window winds past a period: no edge at all.
    WoundPastPeriod,
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
    /// On the conic within the band of the arc's end `0` (at `t0`) or
    /// `1` (at `t1`).
    End(usize),
}

impl<T: Decide> ConicArc<T> {
    /// The arc a certified conic carrier spans over `(t0, t1)`; `None`
    /// for a carrier that is not a circle or an ellipse.
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
        let window = match decide(
            rows.span,
            Margin::levered(T::tau() - (t1 - t0), lever),
            band,
        )
        .map_err(ConicArcError::Escalated)?
        {
            Sign::Positive => Some((t0, t1)),
            Sign::Zero => None,
            Sign::Negative => return Err(ConicArcError::WoundPastPeriod),
        };
        Ok(Some(Self {
            kind,
            center,
            axis,
            u,
            v: axis.cross(u),
            a,
            b,
            lever,
            window,
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

    /// **Where `q` sits against this edge.**
    ///
    /// **A circle**: its unit coordinates are an isometry scaled by the
    /// radius, so every reading is levered into metres exactly — the
    /// distance from the circle, `√(((ρ − 1)·r)² + axial²)`, then the
    /// arc's trim ([`arc_trim`]).
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
    /// `ON` the carrier only where the UPPER bound is within the zero
    /// band; `OFF` only where the LOWER bound clears the escalation band;
    /// between them the answer escalates on whichever bound landed in the
    /// band. The out-of-plane miss `(q − c)·n̂` folds into both. On the
    /// carrier, an end is read as the EXACT distance from `q` to the end
    /// point — never a unit chord — and the side of the ends from the
    /// foot `P` (within the band of `q`, so on the same side of an end
    /// that is more than the escalation band from `q`), its chordal
    /// defect levered by the LARGER semi-axis: a unit angle costs at most
    /// `a` metres of arc, so the margin still bounds the arc length to
    /// the nearer end from above and never compresses it.
    fn hit(&self, q: Point3<T>, rows: ConicRows, band: Band) -> Result<ConicHit, Indeterminate> {
        let (x, y) = self.unit(q);
        let axial = (q - self.center).dot(self.axis);
        if self.kind == ConicKind::Circle {
            let rho = (x.powi(2) + y.powi(2)).sqrt();
            let miss = (((rho - T::one()) * self.lever).powi(2) + axial.powi(2)).sqrt();
            match decide(rows.on, Margin::of(miss), band)? {
                Sign::Zero => {}
                Sign::Positive => return Ok(ConicHit::Off),
                Sign::Negative => return Err(ray_parity::invalid(band, rows.on)),
            }
            let Some(span) = self.window else {
                return Ok(ConicHit::On);
            };
            return Ok(
                match arc_trim((x / rho, y / rho), span, self.lever, rows, band)? {
                    ArcTrim::End(i) => ConicHit::End(i),
                    ArcTrim::On => ConicHit::On,
                    ArcTrim::Off => ConicHit::Carrier,
                },
            );
        }
        let two = T::from_f64(2.0);
        let (a, b) = (self.a, self.b);
        let f = x.powi(2) + y.powi(2) - T::one();
        let (gx, gy) = (two * x / a, two * y / b);
        let g2 = gx.powi(2) + gy.powi(2);
        let g = g2.sqrt();
        let lower = two * f.abs() / (g + (g2 + T::from_f64(4.0) * f.abs() / b.powi(2)).sqrt());
        let (px, py) = (x * a - f * gx / g2, y * b - f * gy / g2);
        let (ux, uy) = (px / a, py / b);
        let r = (ux.powi(2) + uy.powi(2)).sqrt();
        let foot = (ux / r, uy / r);
        let upper = ((x * a - foot.0 * a).powi(2) + (y * b - foot.1 * b).powi(2)).sqrt();
        let lower = (lower.powi(2) + axial.powi(2)).sqrt();
        let upper = (upper.powi(2) + axial.powi(2)).sqrt();
        let near = decide(rows.on, Margin::of(upper), band);
        if !matches!(near, Ok(Sign::Zero)) {
            return match decide(rows.on, Margin::of(lower), band)? {
                Sign::Positive => Ok(ConicHit::Off),
                Sign::Negative => Err(ray_parity::invalid(band, rows.on)),
                // The lower bound is within the band and the upper is
                // not: in-band on the upper's own margin where it has
                // one; where it is definite the two bounds straddle the
                // whole band, which only an ellipse bending tighter than
                // the band resolves allows, and no one margin says so.
                Sign::Zero => Err(match near {
                    Err(diag) => diag,
                    _ => ray_parity::invalid(band, rows.on),
                }),
            };
        }
        let Some((t0, t1)) = self.window else {
            return Ok(ConicHit::On);
        };
        let ends = [
            decide(rows.end, Margin::norm3(q - self.point(t0)), band),
            decide(rows.end, Margin::norm3(q - self.point(t1)), band),
        ];
        if let Some(i) = ends.iter().position(|e| matches!(e, Ok(Sign::Zero))) {
            return Ok(ConicHit::End(i));
        }
        for end in ends {
            match end? {
                Sign::Positive => {}
                Sign::Zero | Sign::Negative => return Err(ray_parity::invalid(band, rows.end)),
            }
        }
        Ok(
            match decide(
                rows.trim,
                Margin::levered(chordal_defect(foot, (t0, t1)), a.max(b)),
                band,
            )? {
                Sign::Positive | Sign::Zero => ConicHit::On,
                Sign::Negative => ConicHit::Carrier,
            },
        )
    }

    /// Is the unit-circle direction `(x, y)` inside the arc's window?
    /// The cosine-window construction (`solid_contain::point_on_wall_in_face`
    /// carries its argument): Positive inside, Negative outside, Zero an
    /// endpoint's neighbourhood. Only a ray's crossing reads it, where a
    /// `Zero` abandons the ray: the construction's endpoint zone is
    /// compressed by `sin(w/2)`, which costs a short arc rays and never
    /// a count. A boundary verdict, where a `Zero` would be an answer,
    /// reads [`Self::hit`] instead.
    fn in_window(&self, (x, y): (T, T), band: Band) -> Result<Sign, Indeterminate> {
        let Some((t0, t1)) = self.window else {
            return Ok(Sign::Positive);
        };
        let half = T::from_f64(0.5);
        let (ms, mc) = ((t0 + t1) * half).sin_cos();
        let (_, cos_half) = ((t1 - t0) * half).sin_cos();
        decide(
            "point_in_arc_loop_conic_window",
            Margin::levered(x * mc + y * ms - cos_half, self.lever),
            band,
        )
    }
}

/// Where a point on an arc's carrier sits against the arc's trim.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ArcTrim {
    /// Within the band of the arc's end `0` (at `t0`) or `1` (at `t1`).
    End(usize),
    /// On the arc, definitely clear of both ends.
    On,
    /// Off the arc, definitely clear of both ends.
    Off,
}

/// The distance between two unit-circle points.
fn apart<T: Decide>((ax, ay): (T, T), (bx, by): (T, T)) -> T {
    ((ax - bx).powi(2) + (ay - by).powi(2)).sqrt()
}

/// The unit-circle point at angle `t`.
fn unit_at<T: Decide>(t: T) -> (T, T) {
    let (s, c) = t.sin_cos();
    (c, s)
}

/// **Which side of an arc's ends a unit-circle point `p` lies**, as the
/// sum of two chordal defects `(|e − m| − |p − m|) + (|p + m| − |e + m|)`,
/// where `e` is an end, `m` the arc's mid direction and `−m` its
/// complement's: positive on the arc, negative off it.
///
/// Chord length is monotone in angular distance up to a half turn, so
/// each term is positive exactly on the arc, and the sum is
/// `2(g(α) − g(w/2))` for `g(x) = cos(x/2) − sin(x/2)` and `α` the angle
/// from `m` — a function whose slope is at least `1/2` over the whole
/// half turn, so the sum is never smaller than the angle from `p` to the
/// nearer end: never compressed, on a short arc, a near-full one, or
/// anywhere between. It is taken from `t0`'s end alone because the two
/// ends are symmetric about `m` — each `w/2` from it — so
/// `|e₀ ∓ m| = |e₁ ∓ m|` and either end gives the same sum.
fn chordal_defect<T: Decide>(p: (T, T), (t0, t1): (T, T)) -> T {
    let e0 = unit_at(t0);
    let m = unit_at((t0 + t1) * T::from_f64(0.5));
    let anti = (T::zero() - m.0, T::zero() - m.1);
    (apart(e0, m) - apart(p, m)) + (apart(p, anti) - apart(e0, anti))
}

/// **Whether a point on a CIRCLE lies on the arc `[t0, t1]`**, decided as
/// distances in the circle's unit coordinates, `dir` the point's
/// direction on it and `lever` the radius, which makes every reading
/// here exact in metres.
///
/// 1. **At an end**: `dir` within the band of either end, measured as a
///    chord, is [`ArcTrim::End`]. That is the ONLY way an endpoint
///    neighbourhood counts. The cosine window `r̂·m̂ ≥ cos(w/2)`
///    compresses arc length near an end by `sin(w/2)` (and by `sin` of
///    the complement on a near-full arc), so its `Zero` would reach
///    `ε / sin(w/2)` along the carrier — a hundred times `ε` at
///    `w = 0.02` — and its escalation ten times that.
/// 2. **Otherwise, which side of the ends**: [`chordal_defect`], never
///    smaller than the arc length to the nearer end. Its `Zero`
///    therefore lies within the band of an end, which step 1 has
///    answered; it reads as on.
///
/// **Its floor.** The unit coordinates carry rounding of order one ulp
/// of 1, which the lever turns into about `1e-16 · lever` metres; where
/// `ε / lever` falls to that order (`ε = 1e-12` on a circle of 10⁴ m)
/// the band is as narrow as the arithmetic's own error, and a point
/// near an end reads by rounding. The fixtures that pin this (radius
/// 10, `ε ≥ 1e-12`) stay three orders clear of it.
///
/// **A second home.** `validate::window`'s arc arm (tier 3's check 9)
/// runs the same two steps in metres on a circle; one home for both is
/// filed as `work/atrest/validate-window-arc-arm-folds-onto-arc-trim.md`.
/// An ellipse reads its ends as exact distances instead
/// ([`ConicArc::hit`]).
///
/// The arc must span less than a period; a whole period has no ends and
/// is the caller's to answer.
fn arc_trim<T: Decide>(
    p: (T, T),
    (t0, t1): (T, T),
    lever: T,
    rows: ConicRows,
    band: Band,
) -> Result<ArcTrim, Indeterminate> {
    let ends = [
        decide(
            rows.end,
            Margin::levered(apart(p, unit_at(t0)), lever),
            band,
        ),
        decide(
            rows.end,
            Margin::levered(apart(p, unit_at(t1)), lever),
            band,
        ),
    ];
    if let Some(i) = ends.iter().position(|e| matches!(e, Ok(Sign::Zero))) {
        return Ok(ArcTrim::End(i));
    }
    for end in ends {
        match end? {
            Sign::Positive => {}
            // A distance is never negative.
            Sign::Zero | Sign::Negative => return Err(ray_parity::invalid(band, rows.end)),
        }
    }
    Ok(
        match decide(
            rows.trim,
            Margin::levered(chordal_defect(p, (t0, t1)), lever),
            band,
        )? {
            Sign::Positive | Sign::Zero => ArcTrim::On,
            Sign::Negative => ArcTrim::Off,
        },
    )
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
    /// Within the band of a conic's end `0` (at the edge's start) or `1`.
    End(usize),
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
                ConicHit::End(i) => EdgeContact::End(i),
            },
            Self::Unrowed { .. } => EdgeContact::Unread,
        })
    }

    /// A conic edge's end point `i` (`0` at its start parameter).
    pub(crate) fn conic_end(&self, i: usize) -> Option<Point3<T>> {
        let Self::Conic(k) = self else {
            return None;
        };
        let (t0, t1) = k.window?;
        Some(k.point(if i == 0 { t0 } else { t1 }))
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
            Err(ConicArcError::Escalated(diag)) => {
                return Err(PointInLoopError::Escalated { r#loop, diag });
            }
            Err(ConicArcError::WoundPastPeriod) => return Err(corrupt()),
        }
        edges.push(match carrier {
            geom::Curve3::Line { .. } => LoopEdge::Chord,
            geom::Curve3::Circle { .. } | geom::Curve3::Ellipse { .. } => {
                unreachable!("a circle or an ellipse is a conic arc above")
            }
            // The arc from its midpoint: the oval's speed is at most
            // `r(R − r)/√((R − r)² − offset²)` (the carrier's own doc),
            // so no point of it lies further from `P(mid)` than that
            // times half the parameter width.
            &geom::Curve3::Spiric {
                major_radius,
                minor_radius,
                offset,
                ..
            } => {
                let inner = major_radius - minor_radius;
                let speed = minor_radius * inner / (inner.powi(2) - offset.powi(2)).sqrt();
                let half = T::from_f64(0.5);
                LoopEdge::Unrowed {
                    center: carrier.eval((t0 + t1) * half),
                    reach: speed * (t1 - t0).abs() * half,
                }
            }
            // Positive weights put a NURBS curve inside its control
            // hull, so inside any ball holding every control point: the
            // one about the control points' bounding-box centre, to the
            // farthest of them.
            geom::Curve3::Nurbs(n) => {
                let control = n.control();
                let first = *control.first().ok_or_else(corrupt)?;
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
                LoopEdge::Unrowed { center, reach }
            }
        });
    }
    Ok(CarrierLoop { verts, keys, edges })
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
    let mut extent = T::zero();
    for v in verts {
        extent = extent.max((*v - q).norm());
    }
    let mut balls = Vec::new();
    for edge in edges {
        let (center, reach) = match *edge {
            LoopEdge::Chord => continue,
            LoopEdge::Conic(k) => (k.center, k.a.max(k.b)),
            LoopEdge::Unrowed { center, reach } => {
                balls.push((center, reach));
                (center, reach)
            }
        };
        extent = extent.max((center - q).norm() + reach);
    }
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
            EdgeContact::On | EdgeContact::End(_) => match boundary {
                Boundary::Verdict => return Ok(Some(WalkSide::OnBoundary)),
                // The caller's pass reads the same arithmetic — the
                // decisions depend on the margins alone, not the rows'
                // names — and placed `q` off this edge.
                Boundary::Decided => {
                    unreachable!("the caller's boundary pass placed the point off this edge")
                }
            },
        }
    }
    let mut blocked = false;
    let walked = walk_schedule(
        r#loop,
        normal,
        extent,
        "point_in_arc_loop_arm",
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
            let Some(mut crossings) =
                ray_parity::ray_crossings(verts, q, d, side_axis, &ARC_LOOP_ROWS, band, |i| {
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
                ConicHit::End(1),
                "w = {t1}: within the band of the end"
            );
            assert_eq!(
                ask(at(t0 - 0.5 * eps / r)),
                ConicHit::End(0),
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
                assert!(
                    !matches!(got, Ok(ConicHit::On | ConicHit::End(_))),
                    "{s}ε off the major vertex ({side}) reads {got:?}"
                );
            }
        }
        // `15ε` of arc from the minor vertex: the speed there is `a`.
        for s in [11.0, 15.0, 18.0, 40.0] {
            let q = k.point(t0 + s * eps / a);
            let got = k.hit(q, WALK_ROWS.conic, band);
            assert!(
                matches!(got, Ok(ConicHit::On) | Err(_)),
                "{s}ε along the arc from its end reads {got:?}"
            );
            assert!(
                !matches!(got, Ok(ConicHit::End(_))),
                "{s}ε along the arc from its end is not the end: {got:?}"
            );
        }
    }
}
