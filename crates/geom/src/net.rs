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

    /// The number of coordinate channels (2 for a plane point, 3 for
    /// a space point).
    const CHANNELS: usize;

    /// The point with every coordinate equal to `v` — the origin at
    /// `T::zero()`, the all-poison point at the scalar's poison.
    fn splat(v: T) -> Self;

    /// Coordinate `d`.
    ///
    /// `d >= CHANNELS` is a caller bug, not an input: every caller in
    /// this crate drives it from `0..CHANNELS`. The implementations
    /// announce it with `unreachable!` (D9's D2 addendum: a state the
    /// code can observe in a branch is announced, never swallowed)
    /// rather than returning a plausible-looking wrong coordinate.
    ///
    /// Those two arms are therefore **unguardable by construction**:
    /// no public door reaches them, and a row could only fire them
    /// through a fake `ControlPoint` impl written to lie about
    /// `CHANNELS` — which would test the fake, not the kernel. They are
    /// unexecuted code, stated as such rather than left to be noticed.
    fn channel(self, d: usize) -> T;

    /// `‖offset‖`.
    fn norm(offset: Self::Offset) -> T;
}

impl<T: Real> ControlPoint<T> for Point2<T> {
    type Offset = Vec2<T>;
    const CHANNELS: usize = 2;

    fn splat(v: T) -> Self {
        Point2::new(v, v)
    }

    fn channel(self, d: usize) -> T {
        match d {
            0 => self.x,
            1 => self.y,
            _ => unreachable!(
                "plane control point has no channel {d}: every caller in this crate \
                 drives `channel` from `0..CHANNELS`"
            ),
        }
    }

    fn norm(offset: Vec2<T>) -> T {
        offset.norm()
    }
}

impl<T: Real> ControlPoint<T> for Point3<T> {
    type Offset = Vec3<T>;
    const CHANNELS: usize = 3;

    fn splat(v: T) -> Self {
        Point3::new(v, v, v)
    }

    fn channel(self, d: usize) -> T {
        match d {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            _ => unreachable!(
                "space control point has no channel {d}: every caller in this crate \
                 drives `channel` from `0..CHANNELS`"
            ),
        }
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
/// totality-and-poison section; this is its one implementation.
pub(crate) fn is_placeholder<T: Real, P: ControlPoint<T>>(control: &[P]) -> bool {
    control.iter().all(|p| p.channel(0).is_poison())
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
    (0..P::CHANNELS)
        .map(|d| {
            control
                .iter()
                .map(|p| RingInterval::from_certified(p.channel(d)))
                .collect()
        })
        .collect()
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
