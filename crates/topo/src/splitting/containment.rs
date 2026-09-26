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
    /// The distance from the point to either end of the arc
    /// ([`arc_trim`]).
    pub(crate) end: &'static str,
    /// The chordal-defect sum that says which side of the ends it is
    /// ([`arc_trim`]).
    pub(crate) trim: &'static str,
}

/// [`point_in_carrier_loop`]'s rows for a conic edge.
const WALK_CONIC_ROWS: ConicRows = ConicRows {
    span: "point_in_arc_loop_conic_span",
    on: "point_in_arc_loop_conic_on",
    end: "point_in_arc_loop_conic_end",
    trim: "point_in_arc_loop_conic_trim",
};

/// A conic arc of a planar loop — a circle or an ellipse — in its own
/// affine frame: the locus `center + u·a·cos θ + v·b·sin θ` over
/// `θ ∈ [t0, t1]` (for an ellipse `θ` is the eccentric anomaly, the
/// carrier's own parameter). In UNIT coordinates `((x−c)·u/a,
/// (x−c)·v/b)` the conic is the unit circle and the arc a window of
/// it, so one set of rows serves both kinds.
#[derive(Clone, Copy)]
pub(crate) struct ConicArc<T: geom_core::Real> {
    center: Point3<T>,
    /// The conic's plane normal.
    axis: Vec3<T>,
    u: Vec3<T>,
    v: Vec3<T>,
    a: T,
    b: T,
    /// The smaller semi-axis: the displacement one unit-coordinate step
    /// buys is at least this, so every unit-coordinate margin is levered
    /// by it — exact for a circle, conservative (escalating more, never
    /// less) for an ellipse.
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
pub(crate) enum ConicHit {
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
    pub(crate) fn of(
        carrier: &geom::Curve3<T>,
        (t0, t1): (T, T),
        rows: ConicRows,
        band: Band,
    ) -> Result<Option<Self>, ConicArcError> {
        let (center, axis, u, a, b) = match *carrier {
            geom::Curve3::Circle {
                center,
                axis,
                radius,
                u_ref,
            } => (center, axis, u_ref, radius, radius),
            geom::Curve3::Ellipse {
                center,
                axis,
                major,
                minor,
                u_ref,
            } => (center, axis, u_ref, major, minor),
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

    /// **Where `q` sits against this edge**, every margin a length: its
    /// distance from the conic first — the in-plane miss `(ρ − 1)`
    /// levered, folded with the out-of-plane miss `(q − c)·n̂`, which is
    /// metres already — then, on the conic, the arc's trim as distances
    /// ([`arc_trim`]).
    pub(crate) fn hit(
        &self,
        q: Point3<T>,
        rows: ConicRows,
        band: Band,
    ) -> Result<ConicHit, Indeterminate> {
        let (x, y) = self.unit(q);
        let rho = (x.powi(2) + y.powi(2)).sqrt();
        let axial = (q - self.center).dot(self.axis);
        let miss = (((rho - T::one()) * self.lever).powi(2) + axial.powi(2)).sqrt();
        match decide(rows.on, Margin::of(miss), band)? {
            Sign::Zero => {}
            Sign::Positive => return Ok(ConicHit::Off),
            // A distance is never negative.
            Sign::Negative => return Err(invalid(band, rows.on)),
        }
        let Some(span) = self.window else {
            return Ok(ConicHit::On);
        };
        Ok(
            match arc_trim((x / rho, y / rho), span, self.lever, rows, band)? {
                ArcTrim::End => ConicHit::End,
                ArcTrim::On => ConicHit::On,
                ArcTrim::Off => ConicHit::Carrier,
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
    /// reads [`arc_trim`] instead ([`Self::hit`]).
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

fn invalid(band: Band, predicate: &'static str) -> Indeterminate {
    Indeterminate {
        margin: geom_core::MarginDiag::Invalid,
        band,
        predicate: Some(predicate),
    }
}

/// Where a point on an arc's carrier sits against the arc's trim.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ArcTrim {
    /// Within the band of one of the arc's two ends.
    End,
    /// On the arc, definitely clear of both ends.
    On,
    /// Off the arc, definitely clear of both ends.
    Off,
}

/// **Whether a point on a conic's carrier lies on the arc `[t0, t1]`**,
/// decided as DISTANCES in the conic's unit coordinates, where the
/// carrier is the unit circle and `dir` is the point's direction on it.
/// `lever` is metres per unit (the radius of a circle; an ellipse's
/// smaller semi-axis, a lower bound on the metres a unit step buys, so
/// every margin escalates more for it, never less).
///
/// 1. **At an end**: `dir` within the band of either end, measured as a
///    chord, is [`ArcTrim::End`]. That is the ONLY way an endpoint
///    neighbourhood counts. The cosine window `r̂·m̂ ≥ cos(w/2)`
///    compresses arc length near an end by `sin(w/2)` (and by `sin` of
///    the complement on a near-full arc), so its `Zero` would reach
///    `ε / sin(w/2)` along the carrier — a hundred times `ε` at
///    `w = 0.02` — and its escalation ten times that.
/// 2. **Otherwise, which side of the ends**: the sum of two chordal
///    defects, `(|e − m| − |p − m|) + (|p + m| − |e + m|)`, where `e` is
///    an end, `m` the arc's mid direction and `−m` its complement's.
///    Chord length is monotone in angular distance up to a half turn,
///    so each term is positive exactly on the arc, and the sum is
///    `2(g(α) − g(w/2))` for `g(x) = cos(x/2) − sin(x/2)` and `α` the
///    angle from `m` — a function whose slope is at least `1/2` over
///    the whole half turn, so the margin is never smaller than the arc
///    length from `p` to the nearer end: never compressed, on a short
///    arc, a near-full one, or anywhere between. Its `Zero` therefore
///    lies within the band of an end, which step 1 has answered; it
///    reads as on. The sum is taken from `t0`'s end alone because the
///    two ends are symmetric about `m` — each `w/2` from it — so
///    `|e₀ ∓ m| = |e₁ ∓ m|` and either end gives the same margin.
///
/// **Its floor.** The unit coordinates carry rounding of order one ulp
/// of 1, which the lever turns into about `1e-16 · lever` metres; where
/// `ε / lever` falls to that order (`ε = 1e-12` on a conic of 10⁴ m)
/// the band is as narrow as the arithmetic's own error, and a point
/// near an end reads by rounding. The fixtures that pin this (radius
/// 10, `ε ≥ 1e-12`) stay three orders clear of it.
///
/// **A second home.** `validate::window`'s arc arm (tier 3's check 9)
/// runs the same two steps in metres on a circle; one home for both is
/// filed as `work/atrest/validate-window-arc-arm-folds-onto-arc-trim.md`.
///
/// The arc must span less than a period; a whole period has no ends and
/// is the caller's to answer.
pub(crate) fn arc_trim<T: Decide>(
    (x, y): (T, T),
    (t0, t1): (T, T),
    lever: T,
    rows: ConicRows,
    band: Band,
) -> Result<ArcTrim, Indeterminate> {
    let apart = |(ax, ay): (T, T), (bx, by): (T, T)| ((ax - bx).powi(2) + (ay - by).powi(2)).sqrt();
    let unit = |t: T| {
        let (s, c) = t.sin_cos();
        (c, s)
    };
    let p = (x, y);
    let (e0, e1) = (unit(t0), unit(t1));
    let ends = [
        decide(rows.end, Margin::levered(apart(p, e0), lever), band),
        decide(rows.end, Margin::levered(apart(p, e1), lever), band),
    ];
    if ends.iter().any(|e| matches!(e, Ok(Sign::Zero))) {
        return Ok(ArcTrim::End);
    }
    for end in ends {
        match end? {
            Sign::Positive => {}
            // A distance is never negative.
            Sign::Zero | Sign::Negative => return Err(invalid(band, rows.end)),
        }
    }
    let m = unit((t0 + t1) * T::from_f64(0.5));
    let anti = (T::zero() - m.0, T::zero() - m.1);
    let defect = (apart(e0, m) - apart(p, m)) + (apart(p, anti) - apart(e0, anti));
    Ok(
        match decide(rows.trim, Margin::levered(defect, lever), band)? {
            Sign::Positive | Sign::Zero => ArcTrim::On,
            Sign::Negative => ArcTrim::Off,
        },
    )
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

/// A loop's vertices in cycle order, and each edge (vertex `i` to `i + 1`)
/// on its own carrier.
type CarrierLoop<T> = (Vec<Point3<T>>, Vec<LoopEdge<T>>);

/// The loop's vertices and its edges on their carriers.
fn carrier_loop<T: Decide>(
    body: &Body<T>,
    r#loop: LoopKey,
    band: Band,
) -> Result<CarrierLoop<T>, PointInLoopError> {
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
        let carrier = curve.carrier();
        match ConicArc::of(carrier, (t0, t1), WALK_CONIC_ROWS, band) {
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

/// Whether the walk's own boundary pre-pass answers, or the caller's
/// already has.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Boundary {
    /// Answer `OnBoundary` where the point is on an edge.
    Verdict,
    /// The caller has decided the point is definitely off every edge of
    /// the loop, with this walk's own conic rows or the same arithmetic
    /// under its own names; a conic row is read only for whether the
    /// point is on the CARRIER, which the crossing count needs.
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
///   boundary pre-pass asks a straight edge [`ray_parity::on_segment`]
///   and an arc its own carrier and trim ([`ConicArc::hit`]) — never an
///   arc's chord, which is not boundary. A point on an arc's conic but
///   off the arc skips its own `s = 0` root. Every graze — a vertex on
///   the ray line, a ray tangent to a conic, a root at an arc's
///   endpoint, a zero advance — abandons the ray.
/// - **An edge on any other carrier** (a spiric, a spline): no crossing
///   row exists, so such an edge is held as a ball its locus lies in
///   (the arc's own, from its midpoint and its speed bound, for a
///   spiric; the control hull's, for a spline), and a ray that could
///   meet that ball is abandoned like a graze. The rest of the loop
///   answers along any scheduled ray that definitely misses every such
///   ball; `None` — the caller's refusal — only where none does, which
///   includes every point inside a ball.
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
    carrier_walk(body, r#loop, normal, q, band, Boundary::Verdict)
}

/// [`point_in_carrier_loop`] for a caller whose own boundary pre-pass
/// has already decided `q` is definitely off every edge of the loop:
/// `In` or `Out`, never `OnBoundary`. The walk's pre-pass is not run —
/// a point the caller's rows placed off the boundary is never
/// re-decided by a second set of rows — and a crossing at `q` itself,
/// which that precondition rules out, is a graze.
///
/// # Errors
///
/// As [`point_in_carrier_loop`].
pub(crate) fn carrier_loop_side<T: Decide>(
    body: &Body<T>,
    r#loop: LoopKey,
    normal: Vec3<T>,
    q: Point3<T>,
    band: Band,
) -> Result<Option<LoopContainment>, PointInLoopError> {
    carrier_walk(body, r#loop, normal, q, band, Boundary::Decided)
}

fn carrier_walk<T: Decide>(
    body: &Body<T>,
    r#loop: LoopKey,
    normal: Vec3<T>,
    q: Point3<T>,
    band: Band,
    boundary: Boundary,
) -> Result<Option<LoopContainment>, PointInLoopError> {
    let escalate = |diag| PointInLoopError::Escalated { r#loop, diag };
    let (verts, edges) = carrier_loop(body, r#loop, band)?;
    if edges.iter().all(|e| matches!(e, LoopEdge::Chord)) {
        return match boundary {
            Boundary::Verdict => point_in_loop(body, r#loop, normal, q, band).map(Some),
            Boundary::Decided => polygon_walk(r#loop, &verts, normal, q, band).map(Some),
        };
    }
    // The loop's reach from `q`: the schedule gate's lever.
    let mut extent = T::zero();
    for v in &verts {
        extent = extent.max((*v - q).norm());
    }
    let mut balls = Vec::new();
    for edge in &edges {
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
    // ---- Boundary pre-pass, and which conics carry `q`. ----
    let mut on_carrier = vec![false; n];
    for (i, edge) in edges.iter().enumerate() {
        match *edge {
            LoopEdge::Chord => {
                if boundary == Boundary::Verdict
                    && ray_parity::on_segment(verts[i], verts[(i + 1) % n], q, &ARC_LOOP_ROWS, band)
                        .map_err(escalate)?
                {
                    return Ok(Some(LoopContainment::OnBoundary));
                }
            }
            LoopEdge::Conic(k) => match k.hit(q, WALK_CONIC_ROWS, band).map_err(escalate)? {
                ConicHit::Off => {}
                ConicHit::Carrier => on_carrier[i] = true,
                // On the arc, or at one of its ends — a vertex of this
                // loop. A caller that has decided `q` off the boundary
                // with the same arithmetic never reaches here; for it
                // only the carrier matters.
                ConicHit::On | ConicHit::End => match boundary {
                    Boundary::Verdict => return Ok(Some(LoopContainment::OnBoundary)),
                    Boundary::Decided => on_carrier[i] = true,
                },
            },
            // Held as a ball the rays below steer clear of.
            LoopEdge::Unrowed { .. } => {}
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
            // the ball's centre (`w = c − q`), taken without a branch.
            for &(center, reach) in &balls {
                let w = center - q;
                let nearest = w - d * w.dot(d).max(T::zero());
                match decide(
                    "point_in_arc_loop_reach",
                    Margin::of(nearest.norm() - reach),
                    band,
                )
                .map_err(escalate)?
                {
                    Sign::Positive => {}
                    Sign::Zero | Sign::Negative => {
                        blocked = true;
                        return Ok(None);
                    }
                }
            }
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
    );
    match walked {
        Ok(side) => Ok(Some(side)),
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
