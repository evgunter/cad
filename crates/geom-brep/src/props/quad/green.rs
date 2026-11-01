//! **The B-spline Green integral** ([`bspline_green_integral`]): the
//! cylinder lane's boundary rule ([`super::cylinder`]'s module docs: the
//! midpoint with its Taylor remainder) on B-spline pcurve channels.
//!
//! Channel hulls and derivative hulls come from the control-coefficient
//! convexity facts (`geom_core::spline::hull`). This lane has **no
//! at-rest consumer**: the NURBS-patch flux lanes beside it are separate
//! engines — `patch_flux_exact` with its composite
//! rounds for a polynomial patch, and `rational_patch_face`'s
//! quotient composite (with the `w`-uniform-in-v exact arm) for a
//! patch whose weights are not one constant — not this Green-form boundary
//! integral. The consumer that would use it is
//! the fitted-boundary Green lane, blocked on a construction that
//! mints a fitted pcurve on an analytic chart at rest — the blocker
//! `topo::props`' `QuadratureUnsupported` refusal names. That is a
//! CONDITION, not a date: the lane is a deliberate frontier whose
//! consumer lands with whichever banked construction (the marched
//! join windows, the edge×NURBS-face boolean layer) first produces
//! one, and its callers until then are tests.

use geom_core::interval::Interval;
use geom_core::interval::certification::Certification;
use geom_core::spline::{ParamRange, SplineCoeffs, SplineCoeffsBuf};

use super::net::{Loose, eval_at, range_hull, raw_eval, raw_range_hull};
use crate::props::PropsError;
use crate::props::bracket::pt;

/// One level of a [`DerivLadder`]: the derivative's coefficients with
/// the knot structure they are a proof about — a clamped vector, or the
/// [`Loose`] structure no clamped vector spells, which is the same
/// structure the patch lanes' [`Dir::Loose`](super::net::Dir::Loose) carries.
pub(super) enum Level {
    /// A clamped spline of degree ≥ 1: differentiated again through its
    /// own vector.
    Spline(SplineCoeffsBuf<Interval>),
    /// A discontinuous derivative ([`Loose::Raw`], which has a level
    /// below it) or per-span constants ([`Loose::Const`], which has
    /// none), with its coefficients.
    Loose(Loose, Vec<Interval>),
}

impl Level {
    /// The derivative of `pair`, as a level.
    fn of(pair: SplineCoeffs<'_, Interval>) -> Self {
        match Loose::from_derived(pair.derived()) {
            Ok(buf) => Self::Spline(buf),
            Err(loose) => Self::Loose(loose, pair.derivative_coeffs()),
        }
    }

    /// The level below, where this one is not per-span constant.
    fn next(&self) -> Option<Self> {
        match self {
            Self::Spline(buf) => Some(Self::of(buf.pair())),
            Self::Loose(dir, coeffs) => {
                let (below, take) = dir.step()?;
                Some(Self::Loose(below, take(coeffs)))
            }
        }
    }

    /// Hull of this derivative over `range`.
    fn hull(&self, range: ParamRange) -> Interval {
        match self {
            Self::Spline(buf) => range_hull(buf.pair(), range),
            Self::Loose(Loose::Raw(knots), coeffs) => raw_range_hull(knots, coeffs, range),
            // The whole-domain coefficient hull: sound for any range.
            Self::Loose(Loose::Const(_), q) => {
                let mut acc = Interval::refused();
                for (n, c) in q.iter().enumerate() {
                    acc = if n == 0 { *c } else { Interval::hull(acc, *c) };
                }
                acc
            }
        }
    }
}

/// The derivative ladder of one nonrational channel, up to third
/// order. A level is missing only below a [`Loose::Const`] one, where the
/// degree is exhausted: the missing order is an **in-span polynomial
/// zero** — a degree-p span polynomial has vanishing derivatives beyond
/// order p — which is sound only on knot-free pieces, and each composite rule
/// over this ladder is responsible for ensuring that. The 1-D Green
/// lane below does it by giving a piece that straddles an interior
/// knot the first-order hull rule, where no smoothness is assumed.
/// The two PATCH lanes do it differently and should not be read
/// through this sentence: their cells are cut ON the interior knots
/// ([`knot_aligned_cuts`](super::cells::knot_aligned_cuts)), so a straddling cell does not arise.
pub(super) struct DerivLadder {
    /// The first derivative — every channel has one, its vector having
    /// at least two coefficients.
    d1: Level,
    /// Orders 2 and 3; `None` below a [`Loose::Const`] level.
    higher: [Option<Level>; 2],
}

impl DerivLadder {
    fn build(pair: SplineCoeffs<'_, Interval>) -> Self {
        let d1 = Level::of(pair);
        let d2 = d1.next();
        let d3 = d2.as_ref().and_then(Level::next);
        Self {
            d1,
            higher: [d2, d3],
        }
    }

    /// Hull of the `order`-th derivative over `range`, assuming the
    /// piece is knot-free (module docs). `order` ∈ 1..=3.
    fn hull(&self, order: usize, range: ParamRange) -> Interval {
        let level = if order == 1 {
            Some(&self.d1)
        } else {
            self.higher[order - 2].as_ref()
        };
        // `None`: an in-span polynomial zero (degree exhausted).
        level.map_or_else(Interval::zero, |l| l.hull(range))
    }
}

/// `∫_a^b u(t)·v\'(t) dt` for a nonrational B-spline pcurve `(u, v)`,
/// each channel minted against its own knot vector — the pcurve's one
/// vector, twice, in every caller today — the general hull-bounded integrand lane
/// (module docs). Pieces free of interior knots use the composite rule
/// `h·f(m) + F₂·h³/24` (channel values via interval de Boor, `F₂` from
/// the derivative ladder); pieces straddling an interior knot use the
/// smoothness-free first-order hull rule `h·hull(f)` — both sound, so
/// the total is an enclosure at every resolution.
///
/// **No at-rest construction mints a stored B-spline pcurve**, so this
/// lane's callers today are tests (module docs). What M6-3 added on
/// loft walls is `Pcurve::IsoLine` — an exact straight line in UV, not
/// a spline — which this integrand does not take; a spline chart image
/// is the SSI trace's `Pcurve::Fitted`, and the construction that first
/// stores one at rest brings this lane's consumer with it. A
/// [`SplineCoeffs`] carries no weights, so a rational pcurve has no
/// spelling here — a rational derivative is not a control-coefficient
/// convexity fact.
///
/// # Errors
///
/// [`PropsError::QuadratureUnsupported`] on an empty or non-finite
/// parameter interval.
///
/// # The pairing rows
///
/// Each channel arrives minted against its vector; the loose
/// `(kv, u, v, weights)` spelling does not compile (the error code was
/// read off `rustc` 1.97.0; stable rustdoc checks only that it fails):
///
/// ```compile_fail,E0061
/// use geom_core::interval::Interval;
/// use geom_core::interval::certification::Certification;
/// use geom_core::spline::KnotVector;
/// let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
/// let c = [Interval::point(0.0); 3];
/// let _ = geom_brep::props::quad::bspline_green_integral(&kv, &c, &c, &[1.0; 3], 0.0, 1.0, 8);
/// ```
///
/// The twin differs in that each channel is minted first:
///
/// ```
/// use geom_core::interval::Interval;
/// use geom_core::interval::certification::Certification;
/// use geom_core::spline::KnotVector;
/// let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
/// let c = [Interval::point(0.0); 3];
/// let pair = kv.with_coeffs(&c).unwrap();
/// assert!(geom_brep::props::quad::bspline_green_integral(pair, pair, 0.0, 1.0, 8).is_ok());
/// ```
pub fn bspline_green_integral(
    u: SplineCoeffs<'_, Interval>,
    v: SplineCoeffs<'_, Interval>,
    a: f64,
    b: f64,
    pieces: usize,
) -> Result<Interval, PropsError> {
    let span = b - a;
    if !(span.is_finite() && span >= 0.0) || pieces == 0 {
        return Err(PropsError::QuadratureUnsupported {
            what: "empty or non-finite parameter interval",
        });
    }
    let u_ladder = DerivLadder::build(u);
    let v_ladder = DerivLadder::build(v);
    // A piece is knot-free only if it is free of BOTH channels' knots.
    let interior: Vec<f64> = u
        .knots()
        .interior_knots()
        .chain(v.knots().interior_knots())
        .map(|(k, _)| k)
        .collect();
    let mut total = Interval::zero();
    #[allow(clippy::cast_precision_loss)]
    let h = span / pieces as f64;
    let h3_24 = pt(h) * pt(h).sqr() / pt(24.0);
    for i in 0..pieces {
        #[allow(clippy::cast_precision_loss)]
        let p_lo = a + span * (i as f64 / pieces as f64);
        let p_hi = p_lo + h;
        let Some(piece) = ParamRange::new(p_lo, p_hi) else {
            return Err(PropsError::QuadratureUnsupported {
                what: "empty or non-finite parameter interval",
            });
        };
        let straddles = interior.iter().any(|k| *k > p_lo && *k < p_hi);
        let uh = range_hull(u, piece);
        let v1h = v_ladder.d1.hull(piece);
        if straddles {
            // Smoothness-free rule across the knot.
            total = total + pt(h) * (uh * v1h);
            continue;
        }
        let m = p_lo + h * 0.5;
        let fm = eval_at(u, m)
            * match &v_ladder.d1 {
                Level::Spline(v1) => eval_at(v1.pair(), m),
                Level::Loose(Loose::Raw(knots), coeffs) => raw_eval(knots, coeffs, m),
                Level::Loose(Loose::Const(_), _) => v1h,
            };
        // f\'\' = u\'\'·v\' + 2·u\'·v\'\' + u·v\'\'\' over the knot-free piece.
        let f2 = u_ladder.hull(2, piece) * v1h
            + pt(2.0) * u_ladder.hull(1, piece) * v_ladder.hull(2, piece)
            + uh * v_ladder.hull(3, piece);
        total = total + pt(h) * fm + f2 * h3_24;
    }
    Ok(total)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use geom_core::Bounds;
    use geom_core::spline::KnotVector;

    /// The general B-spline lane on a known integral: u = t (deg 2),
    /// v = t² (deg 2) on [0, 1]; ∫ u·v' dt = ∫ 2t² dt = 2/3.
    #[test]
    fn bspline_green_contains_the_polynomial_truth() {
        let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
        let u = [pt(0.0), pt(0.5), pt(1.0)];
        let v = [pt(0.0), pt(0.0), pt(1.0)];
        let (u, v) = (kv.with_coeffs(&u).unwrap(), kv.with_coeffs(&v).unwrap());
        let out = bspline_green_integral(u, v, 0.0, 1.0, 64).unwrap();
        let exact = 2.0 / 3.0;
        assert!(
            out.lo() <= exact && exact <= out.hi(),
            "{out:?} must bracket {exact}"
        );
        assert!(out.width() < 1e-6, "width {}", out.width());
    }

    /// **A piece is knot-free only if it is free of both channels'
    /// knots** — a kink in EITHER one. With 63 pieces the middle one
    /// straddles `t = 1/2`, where the integrand's slope jumps; the
    /// composite rule there misses `O(h²)`, far beyond its `O(h³)` pad,
    /// so each bracket holds only if that piece takes the
    /// smoothness-free hull rule. Each case reddens the mutation that
    /// straddle-checks only the OTHER channel's knots.
    ///
    /// - `u = |1 − 2t|` (degree 1, kink at ½) beside `v = t²`:
    ///   `∫ |1 − 2t|·2t dt = 1/2`.
    /// - `u = t` beside `v` of degree 2 with a simple knot at ½ (`v''`
    ///   jumps there), coefficients `[0, 0, 1, 0]`: `∫ t·v' dt =
    ///   v(1) − ∫ v dt = 0 − 1/3`.
    #[test]
    fn a_kink_in_either_channel_takes_the_hull_rule() {
        let lin = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
        let kinked_lin = KnotVector::clamped(vec![0.0, 0.0, 0.5, 1.0, 1.0], 1).unwrap();
        let quad = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
        let kinked_quad = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0], 2).unwrap();
        let (u1, v1) = ([pt(1.0), pt(0.0), pt(1.0)], [pt(0.0), pt(0.0), pt(1.0)]);
        let (u2, v2) = ([pt(0.0), pt(1.0)], [pt(0.0), pt(0.0), pt(1.0), pt(0.0)]);
        for (name, u, v, truth) in [
            (
                "kink in u",
                kinked_lin.with_coeffs(&u1).unwrap(),
                quad.with_coeffs(&v1).unwrap(),
                0.5,
            ),
            (
                "kink in v",
                lin.with_coeffs(&u2).unwrap(),
                kinked_quad.with_coeffs(&v2).unwrap(),
                -1.0 / 3.0,
            ),
        ] {
            let out = bspline_green_integral(u, v, 0.0, 1.0, 63).unwrap();
            assert!(
                out.lo() <= truth && truth <= out.hi(),
                "{name}: {out:?} must bracket {truth}"
            );
        }
    }

    /// **A discontinuous SECOND derivative keeps the order below it.**
    /// `v` cubic with a double knot at ½ (multiplicity `p − 1`: `v'` is
    /// continuous, `v''` jumps, so `v'`'s derivative vector is not
    /// clamped) and coefficients `[0, 0, 0, 1, 0, 0]`, beside `u ≡ 1`:
    /// `∫ u·v' dt = v(1) − v(0) = 0`. Reading the third derivative as
    /// an in-span zero answered a thin bracket that excludes it.
    #[test]
    fn a_discontinuous_second_derivative_does_not_zero_the_third() {
        let ku = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
        let kv =
            KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 0.5, 0.5, 1.0, 1.0, 1.0, 1.0], 3).unwrap();
        let u = [1.0, 1.0].map(pt);
        let v = [0.0, 0.0, 0.0, 1.0, 0.0, 0.0].map(pt);
        let (u, v) = (ku.with_coeffs(&u).unwrap(), kv.with_coeffs(&v).unwrap());
        let out = bspline_green_integral(u, v, 0.0, 1.0, 64).unwrap();
        assert!(
            out.lo() <= 0.0 && 0.0 <= out.hi(),
            "{:?} must bracket 0",
            (out.lo(), out.hi())
        );
    }

    /// **A discontinuous first derivative keeps the orders below it.**
    /// `u` of degree 2 with a double knot at ½ — `u'` jumps there, so
    /// its derivative vector is not a clamped one — and coefficients
    /// `[0, 0, 0, 0, 1]`: `u = 4(t − ½)²` on the second span, so `u'' = 8`
    /// there. Reading orders 2 and 3 as in-span zeros, as a degree-1
    /// parent's would be, drops `u''·v'` from the composite pad and
    /// answers a thin bracket that excludes the truth
    /// `∫ u·v' dt = ∫_{½}^{1} 4(t − ½)² dt = 1/6` (`v = t`).
    #[test]
    fn a_discontinuous_first_derivative_does_not_zero_the_second() {
        let ku = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 0.5, 1.0, 1.0, 1.0], 2).unwrap();
        let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
        let u = [0.0, 0.0, 0.0, 0.0, 1.0].map(pt);
        let v = [0.0, 1.0].map(pt);
        let (u, v) = (ku.with_coeffs(&u).unwrap(), kv.with_coeffs(&v).unwrap());
        let out = bspline_green_integral(u, v, 0.0, 1.0, 64).unwrap();
        let truth = 1.0 / 6.0;
        assert!(
            out.lo() <= truth && truth <= out.hi(),
            "{:?} must bracket 1/6",
            (out.lo(), out.hi())
        );
    }
}
