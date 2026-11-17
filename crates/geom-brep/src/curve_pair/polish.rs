//! **Gauss–Newton polish** of two carriers' closest pair, each carrier
//! read through its point and first derivative at a parameter (a point's
//! foot on one carrier is [`geom::Curve3::foot`]), and **the lower bound
//! on the gap** the polished pair gives (`pose`'s module docs, "The gap's
//! lower bound"). A polished parameter is a place to read, never a
//! decision: every point it yields is read by [`super::Span::contact`],
//! and what the window around it holds is read off [`bound`], never off
//! whether the polish converged.
//!
//! **At an enclosing scalar the search is `f64` structure**
//! (`geom::projection_policy`'s "f64 structure + T payload"): a step
//! whose value is an enclosure moves the iterate by its bracket midpoint
//! ([`geom::bracket_mid`]), and the iterate enters `T` again through
//! `Real::from_computed`, a point. A step read from a whole enclosure
//! would be a step from every point of it, which grows the iterate by
//! the box's spread each step rather than closing on the pair. **A step
//! whose value is a point is taken at `T`**: at `f64`, and at `Sym` or
//! `Dual` over `f64`, the iterate keeps what it carries, its symbols or
//! its tangent, and the search is main's. At `f64` the two are the same
//! bits.
//!
//! **A selection that moves with the inputs** where it is structure, so
//! at `Dual` or `Sym` over an enclosure the polished pair carries no
//! tangent of its own, or is a literal: #874's class
//! (`geom::projection_policy`'s `mid`). At `Sym` over `f64` it is not:
//! `editor-core`'s coincidence door reads the pair's symbols through a
//! vertex's residual (`coincidence_door::two_copies_on_one_axis_glue_their_walls_on_one_axis`
//! names the recourse that residual's form gives). What a consumer reads
//! at the pair (its points, `contact`'s certified parameter) is
//! evaluated at `T` there.

use geom_core::k_stats::prove;
use geom_core::{Decide, Lever, Margin, Point3, Sign, Vec3};

use super::pose::WindowBound;

/// The steps each polish takes.
const STEPS: usize = 4;

/// The row the bound's move reads its premise on.
const MOVE: &str = "curve_pair_pose_move";

/// Gauss–Newton from `(s, t)` toward the closest pair of the carriers
/// `a` and `b`, a step taken where `keep` accepts the metres it moves the
/// pair (read at `T` at the iterate): the pair. A step whose value is a
/// point is taken at `T`; one that is an enclosure moves the iterate by
/// its middle, and the search is `f64` structure from there on (module
/// docs). A step with no finite bracket, or one `keep` refuses, ends the
/// search where it stands: the iterate has not moved, so the next step
/// would be the same one.
pub(super) fn pair<T: Decide>(
    a: impl Fn(T) -> (Point3<T>, Vec3<T>),
    b: impl Fn(T) -> (Point3<T>, Vec3<T>),
    (mut s, mut t): (T, T),
    keep: impl Fn(T) -> bool,
) -> (T, T) {
    let mut structure = false;
    for _ in 0..STEPS {
        let ((pa, da), (pb, db)) = (a(s), b(t));
        let (w, n) = (pb - pa, da.cross(db));
        let nn = n.dot(n);
        let (ds, dt) = (w.cross(db).dot(n) / nn, w.cross(da).dot(n) / nn);
        let moved = da.norm() * ds.abs() + db.norm() * dt.abs();
        let (Some(ms), Some(mt)) = (mid(ds), mid(dt)) else {
            break;
        };
        if !keep(moved) {
            break;
        }
        structure |= !(is_point(ds) && is_point(dt) && is_point(s) && is_point(t));
        if structure {
            let (Some(s0), Some(t0)) = (mid(s), mid(t)) else {
                break;
            };
            (s, t) = (T::from_computed(s0 + ms), T::from_computed(t0 + mt));
        } else {
            (s, t) = (s + ds, t + dt);
        }
    }
    (s, t)
}

/// Whether `x`'s bracket is one point.
fn is_point<T: Decide>(x: T) -> bool {
    x.structure_bracket().is_some_and(|(lo, hi)| lo == hi)
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

/// **What a window reads of its polished pair along and across the
/// carriers** (`pose`'s module docs, "The gap's lower bound" and "The
/// one-sided window"): the sine `g` the along term reads, `|uα − vβ| ≥
/// g·m`; the slack, the part of the gap a move along the carriers could
/// still take up; and the height across, `|w·n̂|`, where a direction
/// across both tangents is read.
#[derive(Clone, Copy, Debug)]
pub(super) struct Along<T> {
    g: T,
    slack: T,
    across: Option<T>,
}

impl<T: Decide> Along<T> {
    /// **The two-sided reading** at `p` with tangent `da` and `q` with
    /// `db`: `g = sin φ` between the tangents, `n̂ = da×db/|da×db|`, the
    /// slack `σ_w = |w − n̂(n̂·w)|` and the height `|w·n̂|`.
    pub(super) fn two_sided((p, da): (Point3<T>, Vec3<T>), (q, db): (Point3<T>, Vec3<T>)) -> Self {
        let n = da.cross(db).normalize();
        let (along, slack) = geom::Line { origin: p, dir: n }.height_and_distance(q);
        Self {
            g: da.normalize().cross(db.normalize()).norm(),
            slack,
            across: Some(along.abs()),
        }
    }

    /// **The one-sided reading** at a pair held at an end of each span:
    /// `u` at `p` and `v` at `q` are the half-tangents pointing into the
    /// spans, every pair of the window at arc `α, β ≥ 0` along them.
    /// `g = √(1 − max(u·v, 0)²)`: `sin φ` while they part by less than a
    /// right angle, `1` past it. The slack is the whole gap `|q − p|`, and
    /// no direction across is read, so the window has no floor.
    pub(super) fn one_sided((p, u): (Point3<T>, Vec3<T>), (q, v): (Point3<T>, Vec3<T>)) -> Self {
        let cos = u.normalize().dot(v.normalize()).max(T::zero());
        Self {
            g: (T::one() - cos.powi(2)).max(T::zero()).sqrt(),
            slack: (q - p).norm(),
            across: None,
        }
    }
}

/// **The basin, and the window's bound** of a polished pair read
/// `along` ([`Along::two_sided`] or [`Along::one_sided`]; `pose`'s module
/// docs, "The gap's lower bound"): every pair of the window lies within
/// `span` metres of the pair along the carriers, `curvature` bounds the
/// carriers' summed curvature there, and `e` is the band's escalation.
/// The one spelling of the basin, the move, the reach and the bend: the
/// sine `g` less the turn over `span` is the sine's lower bound `s`, and
/// the basin is `s − curvature · span > 0`, proved on `row` (levered by
/// `lever` where the caller has one), which puts the window below the
/// move's larger root. `None` where the basin is not proved. Otherwise
/// the floor `|w·n̂| − bend` (`−∞` where no direction across is read),
/// the reach `m₋` and the bend; all three undecided (`−∞`, `+∞`, `+∞`)
/// where the move's quadratic has no real root (`curve_pair_pose_move`
/// not proved positive).
pub(super) fn bound<T: Decide>(
    (row, lever): (&'static str, Option<Lever<T>>),
    along: Along<T>,
    curvature: T,
    (e, span): (T, T),
) -> Option<WindowBound<T>> {
    let sine = along.g - curvature * span;
    let basin = sine - curvature * span;
    let basin = lever.map_or_else(|| Margin::of(basin), |lever| Margin::levered(basin, lever));
    if prove(row, basin) != Ok(Sign::Positive) {
        return None;
    }
    let excess = along.slack + e;
    let two = T::from_f64(2.0);
    let disc = sine.powi(2) - two * curvature * excess;
    if prove(MOVE, Margin::of(disc)) != Ok(Sign::Positive) {
        return Some(undecided());
    }
    // `m₋ = (sin − √disc)/K`, written without the division by `K` so
    // that a pair of lines (`K = 0`) reads `excess/sin`.
    let reach = two * excess / (sine + disc.sqrt());
    let bend = curvature * reach.powi(2) / two;
    Some(WindowBound {
        floor: along
            .across
            .map_or_else(|| T::from_f64(f64::NEG_INFINITY), |h| h - bend),
        reach,
        bend,
    })
}

/// The bound of a window nothing bounds: no floor, and no reach or bend
/// short of the whole carriers.
pub(super) fn undecided<T: Decide>() -> WindowBound<T> {
    WindowBound {
        floor: T::from_f64(f64::NEG_INFINITY),
        reach: T::from_f64(f64::INFINITY),
        bend: T::from_f64(f64::INFINITY),
    }
}
