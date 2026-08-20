//! **Ray-parity point-in-region** — the one home for the trilean
//! containment walk, shared by the crate's two consumers:
//! [`crate::splitting::containment::point_in_loop`] (a planar loop in
//! 3-space) and `chart_region::point_in_polygon` (a chart-space
//! polygon in 2-D).
//!
//! The module sits beside both consumers rather than inside either:
//! the walk is the shared property, and a core hosted in one of its
//! two callers is the shape that drifts back apart.
//!
//! # Method
//!
//! Cast a ray from `q` and count proper crossings of the closed
//! boundary; odd ⇒ inside. Grazing configurations (a vertex on the
//! ray line, a crossing at `q` itself) are not errors of the geometry
//! but of the *ray choice*: the caller retries with the next member of
//! its own fixed direction schedule. No basis is ever picked by
//! coordinate comparison — a comparison-picked basis could diverge
//! between the f64 and interval lanes; a fixed schedule whose
//! degenerate members are detected *by predicate* cannot.
//!
//! Two pieces are shared, in the order a caller runs them:
//!
//! 1. [`on_boundary`] — the boundary pre-pass, run once: is `q`
//!    within the band of any closed segment?
//! 2. [`ray_verdict`] — one schedule member's parity walk, given the
//!    in-plane orthonormal frame `(d, side_axis)` the caller derives
//!    for that member.
//!
//! What is **not** shared is what genuinely differs: each caller owns
//! its own direction schedule, its own frame construction (a 3-D
//! member must be projected into the loop's plane and gated for a
//! definitely-nonzero in-plane arm; a 2-D member is in-plane by
//! construction and needs no such gate), and its own typed error.
//!
//! # Predicate rows are the CALLER's
//!
//! Every decision here funnels through [`crate::validate::decide`]
//! under a name the caller supplies in [`ParityRows`]. The K ledger
//! meters each consumer's margins separately — a 3-D loop's metres
//! and a chart polygon's metres are different populations — so the
//! shared code must not pool them under one name. Sharing the walk
//! and sharing the ledger row are independent decisions, and this
//! module makes only the first.
//!
//! Note that [`ParityRows`] carries **two** names for the boundary
//! pre-pass, because it asks two questions: `segment` decides whether
//! a segment is degenerate (the margin is the segment's own length),
//! `boundary` decides whether `q` lies on it (the margin is a
//! point-to-segment distance). One name for both would meter two
//! populations as one.

use geom_core::{Band, Decide, Indeterminate, Margin, Point2, Point3, Sign, Vec2, Vec3};

use crate::validate::decide;

/// The point/displacement algebra the walk needs, so that one body of
/// code serves the 2-D and 3-D consumers without either projecting
/// into the other's space. Every operation is the caller's own
/// arithmetic, unreassociated: a shared walk must not perturb a
/// margin.
pub(crate) trait RaySpace<T: Decide>: Copy {
    /// The displacement between two points of this space.
    type Disp: Copy;

    /// `self - from`.
    fn disp(self, from: Self) -> Self::Disp;

    /// `self + d`.
    fn offset(self, d: Self::Disp) -> Self;

    /// `d * t`.
    fn scale(d: Self::Disp, t: T) -> Self::Disp;

    /// The inner product.
    fn dot(a: Self::Disp, b: Self::Disp) -> T;

    /// The Euclidean norm.
    fn norm(d: Self::Disp) -> T;

    /// The squared norm through the `powi(2)` door: a zero-straddling
    /// enclosure squared via `Mul` gets a spurious negative lower
    /// bound, `powi` keeps the tight nonnegative one.
    fn norm_squared(d: Self::Disp) -> T;

    /// The length of `d` through this space's dimensional norm door
    /// (`Margin::norm2` / `Margin::norm3`).
    fn length_margin(d: Self::Disp) -> Margin<T>;
}

impl<T: Decide> RaySpace<T> for Point3<T> {
    type Disp = Vec3<T>;

    fn disp(self, from: Self) -> Vec3<T> {
        self - from
    }

    fn offset(self, d: Vec3<T>) -> Self {
        self + d
    }

    fn scale(d: Vec3<T>, t: T) -> Vec3<T> {
        d * t
    }

    fn dot(a: Vec3<T>, b: Vec3<T>) -> T {
        a.dot(b)
    }

    fn norm(d: Vec3<T>) -> T {
        d.norm()
    }

    fn norm_squared(d: Vec3<T>) -> T {
        d.norm_squared()
    }

    fn length_margin(d: Vec3<T>) -> Margin<T> {
        Margin::norm3(d)
    }
}

impl<T: Decide> RaySpace<T> for Point2<T> {
    type Disp = Vec2<T>;

    fn disp(self, from: Self) -> Vec2<T> {
        self - from
    }

    fn offset(self, d: Vec2<T>) -> Self {
        self + d
    }

    fn scale(d: Vec2<T>, t: T) -> Vec2<T> {
        d * t
    }

    fn dot(a: Vec2<T>, b: Vec2<T>) -> T {
        a.dot(b)
    }

    fn norm(d: Vec2<T>) -> T {
        d.norm()
    }

    fn norm_squared(d: Vec2<T>) -> T {
        d.norm_squared()
    }

    fn length_margin(d: Vec2<T>) -> Margin<T> {
        Margin::norm2(d)
    }
}

/// The four K rows one consumer meters its walk through. Distinct
/// names per consumer are the point: see the module docs.
pub(crate) struct ParityRows {
    /// A segment's own length — the degeneracy gate. Zero ⇒ the
    /// segment is null scaffolding and the point distance is measured
    /// exactly (the foot division below would poison on it, `w·e/0`);
    /// a certified-short-but-nonzero segment is a genuine sliver and
    /// escalates like any in-band comparison.
    pub segment: &'static str,
    /// The distance from `q` to a closed segment — perpendicular at
    /// an interior foot, endpoint otherwise. Zero ⇒ on the boundary.
    pub boundary: &'static str,
    /// A vertex's signed offset from the ray line. Zero ⇒ grazing ⇒
    /// next ray.
    pub side: &'static str,
    /// A straddling segment's crossing advance along the ray. Zero
    /// would be a crossing at `q` itself, contradicting the boundary
    /// pre-pass ⇒ next ray.
    pub advance: &'static str,
}

/// The boundary pre-pass: is `q` within the band of any closed
/// segment of the cycle `verts`?
///
/// Run once, before any ray: a `true` here is the `OnBoundary`
/// verdict, and it is also what licenses [`ray_verdict`] to treat a
/// zero advance as a grazing retry rather than a real answer.
///
/// # Errors
///
/// The caller's `escalate` wrapper around an in-band margin on the
/// [`ParityRows::segment`] or [`ParityRows::boundary`] row.
pub(crate) fn on_boundary<T, P, E>(
    verts: &[P],
    q: P,
    rows: &ParityRows,
    band: Band,
    escalate: impl Fn(Indeterminate) -> E,
) -> Result<bool, E>
where
    T: Decide,
    P: RaySpace<T>,
{
    let n = verts.len();
    for i in 0..n {
        let (a, b) = (verts[i], verts[(i + 1) % n]);
        let e = b.disp(a);
        let w = q.disp(a);
        let len2 = P::norm_squared(e);
        let dist = match decide(rows.segment, P::length_margin(e), band).map_err(&escalate)? {
            Sign::Zero => P::norm(w),
            _ => {
                // Foot parameter clamped to the span — evaluation lane
                // (no comparison): t = clamp(w·e / e·e, 0, 1) via
                // min/max.
                let t = (P::dot(w, e) / len2).max(T::zero()).min(T::one());
                let foot = a.offset(P::scale(e, t));
                P::norm(q.disp(foot))
            }
        };
        // Zero ⇒ on boundary; Positive (Negative unreachable for a
        // distance) ⇒ strictly off this segment.
        if decide(rows.boundary, Margin::of(dist), band).map_err(&escalate)? == Sign::Zero {
            return Ok(true);
        }
    }
    Ok(false)
}

/// One schedule member's parity walk, in the in-plane orthonormal
/// frame `(d, side_axis)` the caller derived for that member: `d` is
/// the ray direction, `side_axis` its in-plane perpendicular.
///
/// `Ok(None)` is a **graze** — this ray is unusable and the caller
/// must try the next schedule member (or report its typed exhaustion
/// once the schedule runs out). `Ok(Some(inside))` is the verdict.
///
/// # Errors
///
/// The caller's `escalate` wrapper around an in-band margin on the
/// [`ParityRows::side`] or [`ParityRows::advance`] row.
pub(crate) fn ray_verdict<T, P, E>(
    verts: &[P],
    q: P,
    d: P::Disp,
    side_axis: P::Disp,
    rows: &ParityRows,
    band: Band,
    escalate: impl Fn(Indeterminate) -> E,
) -> Result<Option<bool>, E>
where
    T: Decide,
    P: RaySpace<T>,
{
    let n = verts.len();

    // Signed frame coordinates of each vertex relative to q.
    let mut xs = Vec::with_capacity(n);
    let mut ys = Vec::with_capacity(n);
    let mut sides = Vec::with_capacity(n);
    for p in verts {
        let w = p.disp(q);
        xs.push(P::dot(w, d));
        let y = P::dot(w, side_axis);
        ys.push(y);
        match decide(rows.side, Margin::of(y), band).map_err(&escalate)? {
            Sign::Zero => return Ok(None), // vertex on the ray line
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
        let advance = Margin::over_lever(xs[i] * ys[j] - xs[j] * ys[i], ys[j] - ys[i]);
        match decide(rows.advance, advance, band).map_err(&escalate)? {
            Sign::Positive => crossings += 1,
            Sign::Negative => {}
            // A crossing at q itself contradicts the boundary
            // pre-pass — treat as a graze and retry.
            Sign::Zero => return Ok(None),
        }
    }
    Ok(Some(!crossings.is_multiple_of(2)))
}
