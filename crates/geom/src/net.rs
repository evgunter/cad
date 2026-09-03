//! The control net: what a NURBS curve and a NURBS surface share
//! about their `(control, weights)` pair, independent of the point's
//! dimension and of the net's tensor rank.
//!
//! Every payload in this crate — [`crate::curves::NurbsCurve2`],
//! [`crate::curves::NurbsCurve3`], [`crate::surfaces::NurbsSurface`]
//! — is a flat `Vec` of control points, a matching flat `Vec<f64>` of
//! weights, and one or two knot vectors. The rank lives in the knot
//! vectors; everything below is rank-blind and dimension-blind, so it
//! is written once and used by all three.

use core::ops::{Mul, Sub};

use geom_core::spline::SplineError;
use geom_core::{CertifiedBounds, Point2, Point3, Real, RingInterval, Vec2, Vec3};

/// A control point as the rank-blind net helpers see it: coordinates
/// addressed by index, and the displacement algebra the perturbation
/// bound needs.
pub(crate) trait ControlPoint<T: Real>: Copy + Sub<Self, Output = Self::Offset> {
    /// The displacement type — what subtracting two control points
    /// yields, and what the bound measures.
    type Offset: Copy + Sub<Self::Offset, Output = Self::Offset> + Mul<T, Output = Self::Offset>;

    /// The coordinate channels as a fixed-size array — `[x, y]` for a
    /// plane point, `[x, y, z]` for a space point. The array's length
    /// IS the channel count: there is no separate constant to keep in
    /// agreement with it, and no channel index a caller could get
    /// wrong.
    type Channels: IntoIterator<Item = T>;

    /// The point with every coordinate equal to `v` — the origin at
    /// `T::zero()`, the all-poison point at the scalar's poison.
    fn splat(v: T) -> Self;

    /// The coordinates in channel order (see [`Self::Channels`]).
    fn channels(self) -> Self::Channels;

    /// `‖offset‖`.
    fn norm(offset: Self::Offset) -> T;
}

impl<T: Real> ControlPoint<T> for Point2<T> {
    type Offset = Vec2<T>;
    type Channels = [T; 2];

    fn splat(v: T) -> Self {
        Point2::new(v, v)
    }

    fn channels(self) -> [T; 2] {
        [self.x, self.y]
    }

    fn norm(offset: Vec2<T>) -> T {
        offset.norm()
    }
}

impl<T: Real> ControlPoint<T> for Point3<T> {
    type Offset = Vec3<T>;
    type Channels = [T; 3];

    fn splat(v: T) -> Self {
        Point3::new(v, v, v)
    }

    fn channels(self) -> [T; 3] {
        [self.x, self.y, self.z]
    }

    fn norm(offset: Vec3<T>) -> T {
        offset.norm()
    }
}

/// Constructor validation: counts and weight positivity/finiteness.
/// `expected` is the control count the knot structure demands — one
/// knot vector's `control_count` for a curve, the product of the two
/// for a tensor-product surface. (The knot vectors validate
/// themselves at their own construction.)
///
/// # Errors
///
/// [`SplineError`] naming the exact violation.
// `!(w > 0)` is deliberate (NaN-catching — NaN refuses; `w <= 0`
// would pass NaN). Same note as geom-core::spline::algebra.
#[allow(clippy::neg_cmp_op_on_partial_ord)]
pub(crate) fn validate_counts(
    expected: usize,
    control: usize,
    weights: &[f64],
) -> Result<(), SplineError> {
    if control != expected {
        return Err(SplineError::ControlCountMismatch { control, expected });
    }
    if weights.len() != control {
        return Err(SplineError::WeightCountMismatch {
            weights: weights.len(),
            control,
        });
    }
    for (index, w) in weights.iter().enumerate() {
        if !(*w > 0.0) {
            return Err(SplineError::NonPositiveWeight { index, weight: *w });
        }
        if !w.is_finite() {
            return Err(SplineError::NonFiniteWeight { index, weight: *w });
        }
    }
    Ok(())
}

/// The all-poison point (every coordinate the scalar's poison) — the
/// control point a placeholder net is filled with, and the
/// degenerate-combination result of the knot-algebra plans.
pub(crate) fn poison_point<T: Real, P: ControlPoint<T>>() -> P {
    P::splat(T::from_f64(f64::NAN))
}

/// Is this the "no description yet" placeholder net rather than a
/// described one? The contract is stated once, in the crate docs'
/// totality-and-poison section; this is its one implementation, and
/// it reads **every channel of every control point** — the width that
/// contract states, as one expression because
/// [`ControlPoint::Channels`] carries the count.
///
/// The width is load-bearing in one direction. A net whose every point
/// carries poison in SOME channel and finite data in the others is
/// corrupt *described* geometry: it must reach each consumer's
/// described arm and fail there. The placeholder arm is a benign
/// "mid-surgery, nothing to answer here" at every consumer that tells
/// the states apart, and it is the one answer such a net must never
/// get.
pub(crate) fn is_placeholder<T: Real, P: ControlPoint<T>>(control: &[P]) -> bool {
    control
        .iter()
        .all(|p| p.channels().into_iter().all(Real::is_poison))
}

/// Does any control point carry poison in any channel?
///
/// The box constructors' screen, and the complement of the question
/// [`is_placeholder`] asks: a placeholder answers `true` here (every
/// channel of every point is poison), and so does a DESCRIBED net that
/// carries poison anywhere. That is the distinction a **box** needs and
/// the state discriminator does not — a box is a claim about where the
/// locus is, and a net with one poisoned bracket bounds its locus on no
/// axis. Folding such a net gives a box that is poison on the poisoned
/// axis and finite on the others, and `Aabb::overlaps` tests each axis
/// on its own, so the finite axes witness a disjointness the geometry
/// does not support and the box PRUNES. The poison box is the loud
/// answer for a door whose return type cannot refuse.
pub(crate) fn any_poison<T: Real, P: ControlPoint<T>>(control: &[P]) -> bool {
    control
        .iter()
        .any(|p| p.channels().into_iter().any(Real::is_poison))
}

/// The net's coordinate channels as ring enclosures, in channel order
/// — `[x, y]` for a plane net, `[x, y, z]` for a space net, each in
/// the net's own flat index order.
///
/// **The bracket seam.** Knots, weights and degree are `f64`
/// structure, so the only scalar-typed data in a payload is the
/// control net — and a control point enters the C9 ring through its
/// own bracket, never through an evaluation. At `f64` the bracket is
/// the value (`lo` = `hi`), so this is bitwise what an `f64`-only form
/// produces; at the interval scalar each coefficient carries its
/// enclosure into the hull, which is what makes a composite bound over
/// a lifted payload honest.
pub(crate) fn ring_coords<T: CertifiedBounds, P: ControlPoint<T>>(
    control: &[P],
) -> Vec<Vec<RingInterval>> {
    // One lane per channel. The lane count is read off the channel
    // array of a point this impl mints itself, so it is the SAME
    // statement of the count every `channels()` below makes — one
    // array type, one length — and the zip cannot drop or pad.
    let mut lanes: Vec<Vec<RingInterval>> = P::splat(T::zero())
        .channels()
        .into_iter()
        .map(|_| Vec::with_capacity(control.len()))
        .collect();
    for p in control {
        for (lane, c) in lanes.iter_mut().zip(p.channels()) {
            lane.push(RingInterval::from_certified(c));
        }
    }
    lanes
}

/// One knot-removal pass's projected perturbation bound: `orig` and
/// `re` share their knot structure and net shape by construction
/// (reinsertion restores the original knot vectors, hence the original
/// control count). The two scalar-valued reductions (`c_max`, `bwp`)
/// are ascending-index `Real::max` folds — lattice value ops, so no
/// `T` comparison enters generic code. The other two (`bw`, `w_min`)
/// reduce `f64` **structure** — weights are `f64` by the data model —
/// and are written as ordinary comparisons, which is the C6 lane's
/// posture, not a decision.
///
/// The derivation is on the caller's `remove_knot`; the rank does not
/// enter it, which is why one body serves the curve and the surface.
pub(crate) fn removal_pass_bound<T: Real, P: ControlPoint<T>>(
    orig: (&[P], &[f64]),
    re: (&[P], &[f64]),
) -> T {
    let (orig_control, orig_weights) = orig;
    let (re_control, re_weights) = re;
    let origin = P::splat(T::zero());
    let mut c_max = T::zero();
    let mut bwp = T::zero();
    let mut bw = 0.0f64;
    let mut w_min = f64::INFINITY;
    for (i, pt) in orig_control.iter().enumerate() {
        c_max = c_max.max(P::norm(*pt - origin));
        // Indexing justified: shared net shape (fn docs).
        let (rp, rw) = (re_control[i], re_weights[i]);
        let dw = (orig_weights[i] - rw).abs();
        if dw > bw {
            bw = dw;
        }
        if rw < w_min {
            w_min = rw;
        }
        let d = (*pt - origin) * T::from_f64(orig_weights[i]) - (rp - origin) * T::from_f64(rw);
        bwp = bwp.max(P::norm(d));
    }
    (c_max * T::from_f64(bw) + bwp) * T::from_f64(1.0 / w_min)
}
