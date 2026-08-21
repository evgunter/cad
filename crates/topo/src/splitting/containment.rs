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

    // Ray parity with the fixed schedule.
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
        match decide("point_in_loop_arm", arm, band).map_err(escalate)? {
            Sign::Positive => {}
            _ => continue, // near-parallel schedule member: skip
        }
        let d = d_raw.normalize();
        let side_axis = normal.cross(d); // in-plane ⟂, unit
        if let Some(inside) =
            ray_parity::ray_verdict(&points, q, d, side_axis, &ROWS, band).map_err(escalate)?
        {
            return Ok(if inside {
                LoopContainment::In
            } else {
                LoopContainment::Out
            });
        }
    }
    Err(PointInLoopError::RayExhausted { r#loop })
}
