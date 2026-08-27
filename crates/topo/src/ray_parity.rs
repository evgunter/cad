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
//! member must be projected into the loop's plane and gated on the
//! in-plane displacement that projection commands at the loop's own
//! scale — which also fires when the loop collapses onto `q`; a 2-D
//! member is in-plane by construction and needs no such gate), and
//! its own typed error.
//!
//! # The escalation seam
//!
//! Both entry points return the raw [`Indeterminate`] rather than a
//! caller-supplied wrapper. A wrapper parameter would be the one
//! joint in a module whose whole purpose is that a wrong answer
//! cannot be silent where a caller could be wrong and the compiler
//! would say nothing — `|_| RayExhausted` typechecks and relabels an
//! escalation as exhaustion. Returning the diagnostic keeps the
//! `.map_err(escalate)` idiom visible beside the call, exactly where
//! it sat when the two walks were separate.
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
///
/// The pre-pass uses all six operations; the parity walk uses two.
/// It is one trait rather than two because the two halves are one
/// procedure — the pre-pass is what licenses the walk's zero-advance
/// retry — and splitting it would let a consumer implement half the
/// space.
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

    fn norm_squared(d: Vec2<T>) -> T {
        d.norm_squared()
    }

    fn length_margin(d: Vec2<T>) -> Margin<T> {
        Margin::norm2(d)
    }
}

/// The four K rows one consumer meters its walk through. Distinct
/// names per consumer are the point: see the module docs.
///
/// **These name literals are the greppable K roster entry for the
/// walk.** A name passed as a parameter is invisible to a grep for a
/// literal at the funnel site, so this type is named explicitly in
/// `K-REPORT.md`'s inventory method. It is one of several such
/// carriers, not the only one — the restated method there enumerates
/// the classes and their measured residue. A new `ParityRows` value is
/// a roster change and belongs in that document.
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
/// The raw [`Indeterminate`] of an in-band margin on the
/// [`ParityRows::segment`] or [`ParityRows::boundary`] row. It is
/// returned rather than wrapped: the caller's own
/// `.map_err(escalate)` stays visible at the call site, and this
/// module has no way to drop the diagnostic on the way out.
pub(crate) fn on_boundary<T, P>(
    verts: &[P],
    q: P,
    rows: &ParityRows,
    band: Band,
) -> Result<bool, Indeterminate>
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
        let gap = match decide(rows.segment, P::length_margin(e), band)? {
            Sign::Zero => w,
            _ => {
                // Foot parameter clamped to the span — evaluation lane
                // (no comparison): t = clamp(w·e / e·e, 0, 1) via
                // min/max.
                let t = (P::dot(w, e) / len2).max(T::zero()).min(T::one());
                let foot = a.offset(P::scale(e, t));
                q.disp(foot)
            }
        };
        // The distance stays a DISPLACEMENT until the door, so the
        // norm is taken inside `Margin::norm2`/`norm3` rather than
        // handed to `Margin::of` already rooted.
        // Zero ⇒ on boundary; Positive (Negative unreachable for a
        // distance) ⇒ strictly off this segment.
        if decide(rows.boundary, P::length_margin(gap), band)? == Sign::Zero {
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
/// The raw [`Indeterminate`] of an in-band margin on the
/// [`ParityRows::side`] or [`ParityRows::advance`] row — see
/// [`on_boundary`] on why it is not wrapped here.
pub(crate) fn ray_verdict<T, P>(
    verts: &[P],
    q: P,
    d: P::Disp,
    side_axis: P::Disp,
    rows: &ParityRows,
    band: Band,
) -> Result<Option<bool>, Indeterminate>
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
        match decide(rows.side, Margin::of(y), band)? {
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
        match decide(rows.advance, advance, band)? {
            Sign::Positive => crossings += 1,
            Sign::Negative => {}
            // A crossing at q itself contradicts the boundary
            // pre-pass — treat as a graze and retry.
            Sign::Zero => return Ok(None),
        }
    }
    Ok(Some(!crossings.is_multiple_of(2)))
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    const ROWS: ParityRows = ParityRows {
        segment: "test_ray_parity_segment",
        boundary: "test_ray_parity_boundary",
        side: "test_ray_parity_side",
        advance: "test_ray_parity_advance",
    };

    fn band() -> Band {
        Band::new(1e-9, 1e-8).unwrap()
    }

    fn lift(p: Point2<f64>) -> Point3<f64> {
        Point3::new(p.x, p.y, 0.0)
    }

    /// A square carrying a **repeated vertex**, so one segment is
    /// zero-length. Neither consumer's fixtures build one, and it is
    /// the arm where the clamped foot would divide by `|e|² = 0` — the
    /// pre-pass has to take the endpoint distance instead of poisoning.
    fn null_scaffold() -> [Point2<f64>; 5] {
        [
            Point2::new(0.0, 0.0),
            Point2::new(1.0, 0.0),
            Point2::new(1.0, 0.0), // null scaffolding
            Point2::new(1.0, 1.0),
            Point2::new(0.0, 1.0),
        ]
    }

    #[test]
    fn a_zero_length_segment_measures_the_endpoint_distance_in_both_spaces() {
        let poly = null_scaffold();
        let lifted: Vec<Point3<f64>> = poly.iter().map(|p| lift(*p)).collect();

        // Interior: the null segment must not poison the verdict.
        let inside = Point2::new(0.5, 0.5);
        assert_eq!(on_boundary(&poly, inside, &ROWS, band()), Ok(false));
        assert_eq!(on_boundary(&lifted, lift(inside), &ROWS, band()), Ok(false));

        // The null segment's own point: distance zero, on boundary —
        // reached through the degenerate arm, not the foot division.
        let at_null = Point2::new(1.0, 0.0);
        assert_eq!(on_boundary(&poly, at_null, &ROWS, band()), Ok(true));
        assert_eq!(on_boundary(&lifted, lift(at_null), &ROWS, band()), Ok(true));

        // Off the boundary and nearest to the null vertex: the
        // endpoint distance answers, and it is finite.
        let off = Point2::new(2.0, -1.0);
        assert_eq!(on_boundary(&poly, off, &ROWS, band()), Ok(false));
        assert_eq!(on_boundary(&lifted, lift(off), &ROWS, band()), Ok(false));
    }

    /// A vertex sitting exactly on the `+x` ray line from `(1, 1)`:
    /// that member must GRAZE and a later one must decide. Every
    /// consumer fixture today is decided by its FIRST schedule member,
    /// so nothing else in the crate exercises the retry.
    fn grazing_pentagon() -> [Point2<f64>; 5] {
        [
            Point2::new(0.0, 0.0),
            Point2::new(2.0, 0.0),
            Point2::new(2.0, 1.0), // on the +x ray from (1, 1)
            Point2::new(2.0, 2.0),
            Point2::new(0.0, 2.0),
        ]
    }

    #[test]
    fn the_first_ray_grazes_and_a_later_one_decides_in_2d() {
        let poly = grazing_pentagon();
        let q = Point2::new(1.0, 1.0);
        assert_eq!(on_boundary(&poly, q, &ROWS, band()), Ok(false));

        let x = Vec2::new(1.0, 0.0);
        let graze = ray_verdict(&poly, q, x, Vec2::new(0.0, 1.0), &ROWS, band());
        assert_eq!(graze, Ok(None), "+x must graze the (2, 1) vertex");

        let y = Vec2::new(0.0, 1.0);
        let sy = Vec2::new(-1.0, 0.0);
        assert_eq!(ray_verdict(&poly, q, y, sy, &ROWS, band()), Ok(Some(true)));

        let out = Point2::new(3.0, 1.5);
        assert_eq!(
            ray_verdict(&poly, out, y, sy, &ROWS, band()),
            Ok(Some(false))
        );
    }

    #[test]
    fn the_first_ray_grazes_and_a_later_one_decides_in_3d() {
        let poly: Vec<Point3<f64>> = grazing_pentagon().iter().map(|p| lift(*p)).collect();
        let q = Point3::new(1.0, 1.0, 0.0);
        assert_eq!(on_boundary(&poly, q, &ROWS, band()), Ok(false));

        // The frame the 3-D consumer builds: side_axis = normal x d.
        let x = Vec3::new(1.0, 0.0, 0.0);
        let graze = ray_verdict(&poly, q, x, Vec3::new(0.0, 1.0, 0.0), &ROWS, band());
        assert_eq!(graze, Ok(None), "+x must graze the (2, 1, 0) vertex");

        let y = Vec3::new(0.0, 1.0, 0.0);
        let sy = Vec3::new(-1.0, 0.0, 0.0);
        assert_eq!(ray_verdict(&poly, q, y, sy, &ROWS, band()), Ok(Some(true)));

        let out = Point3::new(3.0, 1.5, 0.0);
        assert_eq!(
            ray_verdict(&poly, out, y, sy, &ROWS, band()),
            Ok(Some(false))
        );
    }

    /// One procedure in two spaces, so on a lifted planar fixture they
    /// must agree not merely on the verdict but on the GRAZE — the
    /// thing a reassociation moves first.
    #[test]
    fn the_two_spaces_agree_ray_for_ray_on_the_lifted_fixture() {
        let poly = grazing_pentagon();
        let lifted: Vec<Point3<f64>> = poly.iter().map(|p| lift(*p)).collect();
        let schedule = [
            (1.0, 0.0),
            (0.0, 1.0),
            (0.5, 1.0),
            (1.0, -0.5),
            (-0.75, 1.0),
        ];
        for q in [(1.0, 1.0), (0.5, 1.5), (3.0, 1.5), (-1.0, -1.0)] {
            let q2 = Point2::new(q.0, q.1);
            let q3 = lift(q2);
            assert_eq!(
                on_boundary(&poly, q2, &ROWS, band()),
                on_boundary(&lifted, q3, &ROWS, band()),
                "pre-pass at {q:?}"
            );
            for (dx, dy) in schedule {
                let d2 = Vec2::new(dx, dy).normalize();
                let s2 = Vec2::new(-d2.y, d2.x);
                let d3 = Vec3::new(d2.x, d2.y, 0.0);
                let s3 = Vec3::new(s2.x, s2.y, 0.0);
                assert_eq!(
                    ray_verdict(&poly, q2, d2, s2, &ROWS, band()),
                    ray_verdict(&lifted, q3, d3, s3, &ROWS, band()),
                    "member ({dx}, {dy}) at {q:?}"
                );
            }
        }
    }
}
