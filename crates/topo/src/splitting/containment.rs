//! Trilean **point-in-loop** containment — the ring re-homing decision
//! (ch. 14's `laringmv`, Problem 13.5) and the seed of F8's 3-D
//! containment machinery. Kept general: a planar loop plus a query
//! point in its plane → In / Out / OnBoundary, with typed escalation —
//! **no unbanded raw comparisons** (every decision is a named K
//! predicate through the Q1 funnel).
//!
//! # Method: in-plane ray parity with a deterministic retry schedule
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
//! accepted iff its length is definitely positive
//! (**`point_in_loop_arm`**; a near-parallel `r_k` is simply skipped).
//!
//! # Predicates (all K-tagged, meters)
//!
//! - **`point_in_loop_boundary`**: distance of `q` to an edge segment
//!   (perpendicular distance when the foot lies inside the span,
//!   endpoint distance otherwise) — Zero ⇒ `OnBoundary`.
//! - **`point_in_loop_arm`**: length of a projected schedule
//!   direction (skip gate, see above).
//! - **`point_in_loop_side`**: signed offset of an edge endpoint from
//!   the ray line — Zero ⇒ grazing ⇒ next ray.
//! - **`point_in_loop_advance`**: the crossing's advance along the
//!   ray — Zero would mean a crossing at `q` itself (contradicting the
//!   boundary pre-pass) ⇒ next ray, escalating if persistent.

use geom_core::{Band, Decide, Indeterminate, Point3, Sign, Vec3};

use crate::body::Body;
use crate::entity::{LoopBoundary, LoopKey};
use crate::validate::decide;

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
            Self::Escalated { r#loop, diag } => {
                write!(f, "point_in_loop: escalated at loop {loop:?}: {diag}")
            }
            Self::RayExhausted { r#loop } => write!(
                f,
                "point_in_loop: every schedule ray grazed loop {loop:?} — \
                 ill-conditioned containment query at this tolerance"
            ),
            Self::CorruptLoop { r#loop } => {
                write!(f, "point_in_loop: loop {loop:?} is not walkable")
            }
        }
    }
}

impl std::error::Error for PointInLoopError {}

/// The fixed schedule of space directions (module docs). 16 entries:
/// three axes plus golden-angle-spread oblique members — for any unit
/// plane normal, most members project to a definitely-nonzero in-plane
/// direction (an all-graze outcome is the typed exhaustion error).
pub(super) const SCHEDULE: [[f64; 3]; 16] = [
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
    let n = points.len();

    // Boundary pre-pass: distance from q to each closed segment.
    for i in 0..n {
        let (a, b) = (points[i], points[(i + 1) % n]);
        let e = b - a;
        let w = q - a;
        // norm_squared, not e·e: the interval-square-poison rule (a
        // straddling enclosure squared via Mul gets a spurious negative
        // lower bound; powi keeps the tight nonnegative one).
        let len2 = e.norm_squared();
        // Degenerate (zero-length) segments — null scaffolding is legal
        // in mid-join loops (PR 5.5 fix pass: the foot division below
        // poisons on them, `w·e/0`) — measure the point distance
        // exactly; a certified-short-but-nonzero segment is a genuine
        // sliver and escalates like any in-band comparison.
        let dist = match decide("point_in_loop_boundary", e.norm(), band).map_err(escalate)? {
            Sign::Zero => w.norm(),
            _ => {
                // Foot parameter clamped to the span — evaluation lane
                // (no comparison): t = clamp(w·e / e·e, 0, 1) via
                // min/max.
                let t = (w.dot(e) / len2).max(T::zero()).min(T::one());
                let foot = a + e * t;
                (q - foot).norm()
            }
        };
        // Zero ⇒ on boundary; Positive (Negative unreachable for a
        // distance) ⇒ strictly off this segment.
        if decide("point_in_loop_boundary", dist, band).map_err(escalate)? == Sign::Zero {
            return Ok(LoopContainment::OnBoundary);
        }
    }

    // The loop's own reach from q (evaluation-lane fold): the lever
    // arm for the probe-direction gate below. A degenerate loop
    // collapsed onto q gives a zero arm, every schedule member skips,
    // and the walk ends in the typed `RayExhausted` — fail-loud.
    let mut extent = T::zero();
    for p in &points {
        extent = extent.max((*p - q).norm());
    }

    // Ray parity with the fixed schedule.
    'ray: for r in &SCHEDULE {
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
        let arm = d_raw.norm() / r.norm() * extent;
        match decide("point_in_loop_arm", arm, band).map_err(escalate)? {
            Sign::Positive => {}
            _ => continue 'ray, // near-parallel schedule member: skip
        }
        let d = d_raw.normalize();
        let side_axis = normal.cross(d); // in-plane ⟂, unit

        // Signed frame coordinates of each vertex relative to q.
        let mut xs = Vec::with_capacity(n);
        let mut ys = Vec::with_capacity(n);
        let mut sides = Vec::with_capacity(n);
        for p in &points {
            let w = *p - q;
            xs.push(w.dot(d));
            let y = w.dot(side_axis);
            ys.push(y);
            match decide("point_in_loop_side", y, band).map_err(escalate)? {
                Sign::Zero => continue 'ray, // endpoint on the ray line
                s => sides.push(s),
            }
        }

        let mut crossings = 0usize;
        for i in 0..n {
            let j = (i + 1) % n;
            if sides[i] == sides[j] {
                continue; // no straddle, no crossing
            }
            // Straddling: the crossing's advance along the ray.
            let advance = (xs[i] * ys[j] - xs[j] * ys[i]) / (ys[j] - ys[i]);
            match decide("point_in_loop_advance", advance, band).map_err(escalate)? {
                Sign::Positive => crossings += 1,
                Sign::Negative => {}
                // A crossing at q itself contradicts the boundary
                // pre-pass — treat as a graze and retry.
                Sign::Zero => continue 'ray,
            }
        }
        return Ok(if crossings % 2 == 1 {
            LoopContainment::In
        } else {
            LoopContainment::Out
        });
    }
    Err(PointInLoopError::RayExhausted { r#loop })
}
