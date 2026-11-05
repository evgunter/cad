//! **Gauss–Newton polish** of two carriers' closest pair, each carrier
//! read through its point and first derivative at a parameter (a point's
//! foot on one carrier is [`geom::Curve3::foot`]), and **the lower bound
//! on the gap** the polished pair gives (`pose`'s module docs, "The gap's
//! lower bound"). A polished parameter is a place to read, never a
//! decision: every point it yields is read by [`super::Span::contact`],
//! and what the window around it holds is read off [`bound`], never off
//! whether the polish converged.
//!
//! **The search is `f64` structure** (`geom::projection_policy`'s "f64
//! structure + T payload"): each step is evaluated at `T` at the
//! iterate, entered through `Real::from_computed`, and the iterate moves
//! by the step's bracket midpoint ([`geom::bracket_mid`]). A step read
//! from a whole enclosure would be a step from every point of it, which
//! grows the iterate by the box's spread each step rather than closing
//! on the pair. At `f64` the midpoint of a point is the point, so the
//! search is bit for bit the plain `f64` iteration.
//!
//! **A selection that moves with the inputs**, so at a `Dual` the
//! polished pair carries no tangent of its own, and at `Sym` it is a
//! literal: #874's class (`geom::projection_policy`'s `mid`). What a
//! consumer reads at the pair (its points, `contact`'s certified
//! parameter) is evaluated at `T` there.

use geom_core::k_stats::prove;
use geom_core::{Decide, Margin, Point3, Sign, Vec3};

use super::pose::WindowBound;

/// The steps each polish takes.
const STEPS: usize = 4;

/// The row the bound's move reads its premise on.
const MOVE: &str = "curve_pair_pose_move";

/// Gauss–Newton from `(s, t)` toward the closest pair of the carriers
/// `a` and `b`, a step taken where `keep` accepts the metres it moves the
/// pair (read at `T` at the iterate): the pair, entered at `T`. A start or
/// a step with no finite bracket ends the search where it stands.
pub(super) fn pair<T: Decide>(
    a: impl Fn(T) -> (Point3<T>, Vec3<T>),
    b: impl Fn(T) -> (Point3<T>, Vec3<T>),
    (s0, t0): (T, T),
    keep: impl Fn(T) -> bool,
) -> (T, T) {
    let (Some(mut s), Some(mut t)) = (mid(s0), mid(t0)) else {
        return (s0, t0);
    };
    for _ in 0..STEPS {
        let ((pa, da), (pb, db)) = (a(T::from_computed(s)), b(T::from_computed(t)));
        let (w, n) = (pb - pa, da.cross(db));
        let nn = n.dot(n);
        let (ds, dt) = (w.cross(db).dot(n) / nn, w.cross(da).dot(n) / nn);
        let moved = da.norm() * ds.abs() + db.norm() * dt.abs();
        let (Some(ds), Some(dt)) = (mid(ds), mid(dt)) else {
            break;
        };
        if keep(moved) {
            s += ds;
            t += dt;
        }
    }
    (T::from_computed(s), T::from_computed(t))
}

/// The middle of `x`'s bracket ([`geom::bracket_mid`]), or `None` where
/// it has no finite one: poison, an end at infinity, or a width that
/// overflows.
fn mid<T: Decide>(x: T) -> Option<f64> {
    x.structure_bracket()
        .filter(|(lo, hi)| lo.is_finite() && hi.is_finite())
        .map(geom::bracket_mid)
        .filter(|m| m.is_finite())
}

/// **The window's bound** at the polished pair, `p` with its tangent
/// `da` on the first carrier and `q` with `db` on the second; `sine` a
/// lower bound on the sine between the tangents over the window,
/// `curvature` the carriers' summed curvature bound there and `e` the
/// band's escalation (`pose`'s module docs, "The gap's lower bound"),
/// on the premise, the basin, proved by the caller: every pair of the
/// window lies within `span` metres of `(p, q)` along the carriers, and
/// `curvature · span < sine`. The floor `|w·n̂| − bend`, the reach `m₋`
/// and the basin's `span`; floor and reach undecided (`−∞`, `+∞`) where
/// the move's quadratic has no real root (`curve_pair_pose_move` not
/// proved positive).
pub(super) fn bound<T: Decide>(
    (p, da): (Point3<T>, Vec3<T>),
    (q, db): (Point3<T>, Vec3<T>),
    sine: T,
    curvature: T,
    (e, span): (T, T),
) -> WindowBound<T> {
    let n = da.cross(db);
    let n = n * (T::one() / n.norm());
    let w = q - p;
    let along = w.dot(n);
    let slack = (w - n * along).norm();
    let excess = slack + e;
    let two = T::from_f64(2.0);
    let disc = sine.powi(2) - two * curvature * excess;
    if prove(MOVE, Margin::of(disc)) != Ok(Sign::Positive) {
        return WindowBound {
            basin: Some(span),
            ..undecided()
        };
    }
    // `m₋ = (sin − √disc)/K`, written without the division by `K` so
    // that a pair of lines (`K = 0`) reads `excess/sin`.
    let reach = two * excess / (sine + disc.sqrt());
    let bend = curvature * reach.powi(2) / two;
    WindowBound {
        floor: along.abs() - bend,
        reach,
        basin: Some(span),
    }
}

/// The bound of a window nothing bounds: no floor, no reach short of the
/// whole carriers, and no basin.
pub(super) fn undecided<T: Decide>() -> WindowBound<T> {
    WindowBound {
        floor: T::from_f64(f64::NEG_INFINITY),
        reach: T::from_f64(f64::INFINITY),
        basin: None,
    }
}
