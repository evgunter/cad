//! The control net: what a NURBS curve and a NURBS surface share
//! about their `(control, weights)` pair, independent of the point's
//! dimension and of the net's tensor rank.
//!
//! Every payload in this crate — [`crate::curves::NurbsCurve2`],
//! [`crate::curves::NurbsCurve3`], [`crate::surfaces::NurbsSurface`]
//! — is a flat `Vec` of control points, a matching flat `Vec<T>` of
//! weights with their `f64` value channel beside them, and one or two
//! knot vectors held in `T`. The rank lives in the knot
//! vectors; everything below is rank-blind and dimension-blind, so it
//! is written once and used by all three.

use core::ops::{Mul, Sub};

use geom_core::spline::{Knots, SpanLocate, SplineError, Weights};
use geom_core::{CertifiedBounds, Interval, Point2, Point3, Real, Vec2, Vec3};

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
        self.to_array()
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
        self.to_array()
    }

    fn norm(offset: Vec3<T>) -> T {
        offset.norm()
    }
}

/// The count half of [`validate_counts`]: `control` points and
/// `weights` weights against the `expected` control count, in that
/// order. The one statement of the count rule.
///
/// # Errors
///
/// [`SplineError::ControlCountMismatch`], then
/// [`SplineError::WeightCountMismatch`].
pub(crate) fn check_counts(
    expected: usize,
    control: usize,
    weights: usize,
) -> Result<(), SplineError> {
    if control != expected {
        return Err(SplineError::ControlCountMismatch { control, expected });
    }
    if weights != control {
        return Err(SplineError::WeightCountMismatch { weights, control });
    }
    Ok(())
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
pub(crate) fn validate_counts(
    expected: usize,
    control: usize,
    weights: &[f64],
) -> Result<(), SplineError> {
    check_counts(expected, control, weights.len())?;
    geom_core::spline::check_weight_values(weights)
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
/// The box constructors' screen. A placeholder answers `true` here
/// (every channel of every point is poison), and so does a DESCRIBED
/// net that carries poison anywhere — the two states a box must treat
/// alike, because a box is a claim about where the locus is and a net
/// with one poisoned channel bounds its locus on no axis. That is the
/// distinction a **box** needs; a consumer that must tell the three
/// states apart asks `crate::NurbsSurface::net_state`, which reads
/// this door and [`is_placeholder`] together and answers
/// `crate::NetState`. Folding such a net gives a box that is poison on the poisoned
/// axis and finite on the others, and `Aabb::overlaps` tests each axis
/// on its own, so the finite axes witness a disjointness the geometry
/// does not support and the box PRUNES. The poison box is the loud
/// answer for a door whose return type cannot refuse.
pub(crate) fn any_poison<T: Real, P: ControlPoint<T>>(control: &[P]) -> bool {
    control
        .iter()
        .any(|p| p.channels().into_iter().any(Real::is_poison))
}

/// The net's coordinate channels as certification enclosures, in channel order
/// — `[x, y]` for a plane net, `[x, y, z]` for a space net, each in
/// the net's own flat index order.
///
/// **The bracket seam.** A control point enters certification
/// arithmetic through its own bracket, never through an evaluation.
/// (The weights and knots a payload holds in `T` enter the certificate
/// on their value channel, the `f64` structure it is stated over.) At `f64` the bracket is
/// the value (`lo` = `hi`), so this is bitwise what an `f64`-only form
/// produces; at the interval scalar each coefficient carries its
/// enclosure into the hull, which is what makes a composite bound over
/// a lifted payload honest.
pub(crate) fn certified_coords<T: CertifiedBounds, P: ControlPoint<T>>(
    control: &[P],
) -> Vec<Vec<Interval>> {
    // One lane per channel. The lane count is read off the channel
    // array of a point this impl mints itself, so it is the SAME
    // statement of the count every `channels()` below makes — one
    // array type, one length — and the zip cannot drop or pad.
    let mut lanes: Vec<Vec<Interval>> = P::splat(T::zero())
        .channels()
        .into_iter()
        .map(|_| Vec::with_capacity(control.len()))
        .collect();
    for p in control {
        for (lane, c) in lanes.iter_mut().zip(p.channels()) {
            lane.push(Interval::from_certified(c));
        }
    }
    lanes
}

/// The weights' value channel: each weight's one `f64`
/// ([`SpanLocate::point_value`]), what every structure decision on the
/// weights reads (C6's interim: the knot algebra's plans, the
/// certificates). A poison weight reads as NaN, so the positivity check
/// beside this refuses it as it refuses a NaN `f64` weight
/// ([`SplineError::NonPositiveWeight`]).
///
/// A weight wider than a point is held weights' business ([`Weights`]):
/// its sign is certified for every member there first, so a weight
/// some members take non-positive refuses as that
/// ([`SplineError::WeightSignUndecided`], with its margin), and only a
/// wide weight that is certified fine reaches the carrier's own refusal.
///
/// The order of refusals differs by regime. With every weight a point,
/// the door's count check runs first and the positivity check after it
/// (`validate_counts`). With a weight wider than a point, every weight
/// is certified here first, so a non-positive point weight elsewhere
/// refuses before a count mismatch would.
///
/// # Errors
///
/// [`Weights::new`]'s, where some weight is wider than a point; then
/// [`SplineError::CarrierWeightNotAPoint`] for the first such weight.
pub(crate) fn weight_channel<T: SpanLocate>(weights: &[T]) -> Result<Vec<f64>, SplineError> {
    let channel: Option<Vec<f64>> = weights
        .iter()
        .map(|w| {
            w.point_value()
                .or_else(|| w.is_poison().then_some(f64::NAN))
        })
        .collect();
    if let Some(channel) = channel {
        return Ok(channel);
    }
    let held = Weights::new(weights.to_vec())?;
    // Certified, so no weight is poison: the wide one is the one that
    // has no point.
    let index = held.first_wide().unwrap_or(0);
    Err(SplineError::CarrierWeightNotAPoint { index })
}

/// A carrier's knots have their `f64` vector: its evaluation, span
/// reads and knot algebra read them through it
/// ([`Knots::structure_f64_interim`]). The test is the vector itself
/// ([`Knots::is_point`]), the very thing those readers take.
///
/// # Errors
///
/// [`SplineError::CarrierKnotNotAPoint`] at the first run whose value
/// is an enclosure wider than a point.
pub(crate) fn carrier_knots<T: SpanLocate>(knots: &Knots<T>) -> Result<(), SplineError> {
    if knots.is_point() {
        return Ok(());
    }
    // Knots hold their vector exactly when every run's value is a point
    // (`Knots::new`), so a run that is not one exists.
    let index = knots.first_wide_run().unwrap_or(0);
    Err(SplineError::CarrierKnotNotAPoint { index })
}

/// One knot-removal pass's projected perturbation bound: `orig` and
/// `re` share their knot structure and net shape by construction
/// (reinsertion restores the original knot vectors, hence the original
/// control count). Every reduction is an ascending-index `Real::max` /
/// `Real::min` fold over values held in `T` — lattice value ops, so no
/// `T` comparison enters generic code, and the weights' perturbation
/// is a function of the weights the curve holds.
///
/// The derivation is on the caller's `remove_knot`; the rank does not
/// enter it, which is why one body serves the curve and the surface.
pub(crate) fn removal_pass_bound<T: Real, P: ControlPoint<T>>(
    orig: (&[P], &[T]),
    re: (&[P], &[T]),
) -> T {
    let (orig_control, orig_weights) = orig;
    let (re_control, re_weights) = re;
    let origin = P::splat(T::zero());
    let mut c_max = T::zero();
    let mut bwp = T::zero();
    let mut bw = T::zero();
    // Seeded by the first weight, not by `+∞`, which is no value at
    // `Interval` (`from_f64` poisons it) and would refuse every bound.
    let mut w_min: Option<T> = None;
    for (i, pt) in orig_control.iter().enumerate() {
        c_max = c_max.max(P::norm(*pt - origin));
        // Indexing justified: shared net shape (fn docs).
        let (rp, rw) = (re_control[i], re_weights[i]);
        bw = bw.max((orig_weights[i] - rw).abs());
        w_min = Some(w_min.map_or(rw, |m| m.min(rw)));
        let d = (*pt - origin) * orig_weights[i] - (rp - origin) * rw;
        bwp = bwp.max(P::norm(d));
    }
    // An empty net perturbs nothing.
    w_min.map_or_else(T::zero, |w_min| (c_max * bw + bwp) * (T::one() / w_min))
}
