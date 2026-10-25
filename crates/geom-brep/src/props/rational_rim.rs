//! **Certified vector area of a rational spline rim** — the plane cap a
//! `Curve3::Nurbs` with varying weights bounds
//! (`work/flux/a-planar-face-bounded-by-a-rational-spline-rim-has-no-vector-area.md`).
//!
//! A plane's flux is `anchor·A⃗` and its area `‖A⃗‖`, so the cap needs
//! only each edge's vector-area term `(1/2)∫(P − ref) × P′ dt`. On a
//! rational carrier `P = A/w` this is, with the homogeneous numerator
//! about the reference `B = A − ref·w`,
//!
//! ```text
//! (P − ref) × P′ = B × B′ / w²
//! ```
//!
//! (`P − ref = B/w`, `P′ = (B′w − Bw′)/w²`, and `B × B = 0`) — a
//! rational function per knot span with a positive denominator, which
//! no fixed Gauss rule integrates exactly: the closed-form lane
//! (`super::loop_vector_area`) keeps its typed refusal, and this lane
//! reads it through the spiric lane's certified rule
//! ([`super::spiric::composite_by`]: a Taylor node per piece and the
//! Lagrange remainder over it, the width falling as `h⁵`). Every
//! operation the integrand needs is a ring operation or the reciprocal
//! of `w²`, whose enclosure is positive wherever the weights are.
//!
//! # Built once, evaluated per round
//!
//! [`rational_edge_q`] extracts the carrier's Bézier spans at the
//! interval scalar (knot insertion in homogeneous coordinates, so the
//! extraction rounds into the enclosure rather than moving the curve),
//! forms each span's `B` and `w` about the reference, converts them to
//! power form in the span's own parameter `s = (t − a)/(b − a)`, and
//! multiplies out the integrand's numerator `B × dB/ds` there, once. A
//! round then evaluates it and `w` by Horner in jet arithmetic and takes
//! one reciprocal of `w²` per evaluation; nothing about
//! the carrier is rebuilt as the schedule doubles the pieces.
//!
//! Each span is integrated in `t` over its part of the edge's carrier
//! interval, whose ends are knots or the edge's own parameters, exact
//! `f64` values: the rule's endpoint-bracket pad is never paid.

use geom::NurbsCurve3;
use geom_core::Bounds;
use geom_core::Point3;
use geom_core::interval::Interval;
use geom_core::interval::certification::Certification;

use super::PropsError;
use super::bracket::pt;
use super::quad::RVec3;
use super::spiric::{Jet, composite_by};

/// One Bézier span of a rational rim, read on the part `[lo, hi]` of the
/// edge's carrier interval it covers: the integrand's numerator
/// `B × dB/ds` about the reference and the weight `w`, each in power form
/// in `s = (t − a)·inv_len`, ascending.
#[derive(Clone, Debug)]
pub struct RationalSpanQ {
    lo: f64,
    hi: f64,
    a: f64,
    inv_len: Interval,
    cross: [Vec<Interval>; 3],
    weight: Vec<Interval>,
}

/// A rational rim edge for the cap's rule: its spans over the edge's
/// carrier interval and its traversal sense.
#[derive(Clone, Debug)]
pub struct RationalEdgeQ {
    spans: Vec<RationalSpanQ>,
    forward: bool,
}

impl RationalEdgeQ {
    /// How many Bézier spans the edge integrates over, each at the
    /// round's piece count: the lane's cost per round.
    #[must_use]
    pub fn span_count(&self) -> usize {
        self.spans.len()
    }
}

fn binomial(n: usize, k: usize) -> f64 {
    #[allow(clippy::cast_precision_loss)]
    (0..k).fold(1.0, |acc, i| acc * (n - i) as f64 / (i + 1) as f64)
}

/// Bernstein coefficients `b₀..b_p` to the power form `Σ cₖ sᵏ`:
/// `cₖ = C(p, k)·Σᵢ₌₀..ₖ (−1)^(k−i)·C(k, i)·bᵢ`, in interval arithmetic.
fn power_form(bern: &[Interval]) -> Vec<Interval> {
    let p = bern.len() - 1;
    (0..=p)
        .map(|k| {
            let sum = (0..=k).fold(Interval::zero(), |acc, i| {
                let sign = if (k - i) % 2 == 0 { 1.0 } else { -1.0 };
                acc + bern[i] * pt(sign * binomial(k, i))
            });
            sum * pt(binomial(p, k))
        })
        .collect()
}

/// The power form's `s`-derivative: `(k + 1)·c_{k+1}`.
fn slope_form(power: &[Interval]) -> Vec<Interval> {
    #[allow(clippy::cast_precision_loss)]
    (1..power.len()).map(|k| power[k] * pt(k as f64)).collect()
}

/// The product of two power forms.
fn poly_mul(f: &[Interval], g: &[Interval]) -> Vec<Interval> {
    let mut out = vec![Interval::zero(); f.len() + g.len() - 1];
    for (i, &x) in f.iter().enumerate() {
        for (j, &y) in g.iter().enumerate() {
            out[i + j] = out[i + j] + x * y;
        }
    }
    out
}

/// `f·g − h·k` of four power forms.
fn poly_cross(f: &[Interval], g: &[Interval], h: &[Interval], k: &[Interval]) -> Vec<Interval> {
    let (fg, hk) = (poly_mul(f, g), poly_mul(h, k));
    fg.iter().zip(&hk).map(|(&x, &y)| x - y).collect()
}

fn horner(coeffs: &[Interval], s: Jet) -> Jet {
    coeffs
        .iter()
        .rev()
        .fold(Jet::cst(Interval::zero()), |acc, &c| {
            acc.mul(s).add(Jet::cst(c))
        })
}

/// One knot insertion (Boehm, NURBS Book A5.1) on homogeneous points:
/// the knot value is an exact `f64`, the blend ratios are enclosed, so
/// the new polygon's enclosure holds the curve's exact one.
fn insert(knots: &mut Vec<f64>, hom: &mut Vec<[Interval; 4]>, p: usize, u: f64) {
    let k = knots.partition_point(|&x| x <= u) - 1;
    let blended: Vec<[Interval; 4]> = (k + 1 - p..=k)
        .map(|i| {
            let alpha = (pt(u) - pt(knots[i])) / (pt(knots[i + p]) - pt(knots[i]));
            let beta = Interval::one() - alpha;
            core::array::from_fn(|c| hom[i][c] * alpha + hom[i - 1][c] * beta)
        })
        .collect();
    hom.splice(k + 1 - p..k, blended);
    knots.insert(k + 1, u);
}

/// **A rational rim edge, ready for the rule**: the carrier's Bézier
/// spans over `[t0, t1]`, each span's `B = A − ref·w` and `w` about
/// `reference` in power form (module docs).
///
/// # Errors
///
/// [`PropsError::QuadratureUnsupported`] where the carrier interval's
/// ends or the carrier's knots are not exact parameters.
pub fn rational_edge_q(
    curve: &NurbsCurve3<Interval>,
    t0: Interval,
    t1: Interval,
    forward: bool,
    reference: Point3<Interval>,
) -> Result<RationalEdgeQ, PropsError> {
    let exact = |t: Interval| (t.is_certified() && t.lo() == t.hi()).then_some(t.lo());
    let (Some(t0), Some(t1)) = (exact(t0), exact(t1)) else {
        return Err(PropsError::QuadratureUnsupported {
            what: "a rational rim edge whose carrier interval ends are not exact parameters",
        });
    };
    let p = curve.degree();
    // The carrier in homogeneous coordinates about the reference: `B` in
    // the first three slots, `w` in the fourth.
    // The held knots, each an exact parameter: a wider one refuses.
    let Some(mut knots) = curve
        .knot_values()
        .values()
        .iter()
        .map(|&k| exact(k))
        .collect::<Option<Vec<f64>>>()
    else {
        return Err(PropsError::QuadratureUnsupported {
            what: "a rational rim whose knots are not exact parameters",
        });
    };
    let mut hom: Vec<[Interval; 4]> = curve
        .control()
        .iter()
        .zip(curve.weights())
        .map(|(c, &w)| {
            let r = reference;
            [(c.x - r.x) * w, (c.y - r.y) * w, (c.z - r.z) * w, w]
        })
        .collect();
    // Every interior knot raised to multiplicity `p`: the span windows
    // are then the Bézier control polygons.
    let (first, last) = (knots[p], knots[knots.len() - 1 - p]);
    let mut add = Vec::new();
    let mut i = 0;
    while i < knots.len() {
        let k = knots[i];
        let m = knots[i..].iter().take_while(|&&x| x == k).count();
        if k > first && k < last && m < p {
            add.extend(core::iter::repeat_n(k, p - m));
        }
        i += m;
    }
    for u in add {
        insert(&mut knots, &mut hom, p, u);
    }
    let mut spans = Vec::new();
    for index in p..knots.len() - 1 - p {
        let (a, b) = (knots[index], knots[index + 1]);
        let (lo, hi) = (a.max(t0), b.min(t1));
        if a >= b || lo >= hi {
            continue;
        }
        let window = &hom[index - p..=index];
        let coord = |c: usize| window.iter().map(|h| h[c]).collect::<Vec<_>>();
        let n: [Vec<Interval>; 3] = core::array::from_fn(|c| power_form(&coord(c)));
        let d: [Vec<Interval>; 3] = core::array::from_fn(|c| slope_form(&n[c]));
        let w = power_form(&coord(3));
        spans.push(RationalSpanQ {
            lo,
            hi,
            a,
            inv_len: Interval::one() / (pt(b) - pt(a)),
            cross: [
                poly_cross(&n[1], &d[2], &n[2], &d[1]),
                poly_cross(&n[2], &d[0], &n[0], &d[2]),
                poly_cross(&n[0], &d[1], &n[1], &d[0]),
            ],
            weight: w,
        });
    }
    Ok(RationalEdgeQ { spans, forward })
}

/// One rational rim edge's vector-area term `(1/2)∫ B × B′/w² dt` at
/// `pieces` per span, traversal folded in, and per component the part of
/// its width a later round removes.
#[must_use]
pub fn rational_vector_area_split(e: &RationalEdgeQ, pieces: usize) -> (RVec3, [f64; 3]) {
    let mut va = [Interval::zero(); 3];
    let mut removable = [0.0_f64; 3];
    for span in &e.spans {
        // `dB/dt = dB/ds · ds/dt`, so the integrand is
        // `(B × dB/ds)·inv_len / (2w²)`.
        let half_rate = span.inv_len * pt(0.5);
        let (part, rem) = composite_by(pt(span.lo), pt(span.hi), pieces, |t| {
            let s = Jet::var(t).sub(Jet::cst(pt(span.a))).scale(span.inv_len);
            // `w` squared as a jet, not multiplied out: a power form of
            // `w²` read over a whole span can straddle zero where `w`'s
            // own square cannot.
            let w = horner(&span.weight, s);
            let scale = w.mul(w).recip().scale(half_rate);
            span.cross.each_ref().map(|c| horner(c, s).mul(scale))
        });
        for k in 0..3 {
            va[k] = va[k] + part[k];
            removable[k] += rem[k];
        }
    }
    (if e.forward { va } else { va.map(|x| -x) }, removable)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;
    use geom_core::spline::KnotVector;

    fn iv(x: f64) -> Interval {
        pt(x)
    }

    /// A planar rational carrier at the interval scalar, `z = 0`.
    fn carrier(
        knots: Vec<f64>,
        degree: usize,
        pts: &[[f64; 2]],
        w: &[f64],
    ) -> NurbsCurve3<Interval> {
        NurbsCurve3::new(
            geom_core::spline::Knots::from(KnotVector::clamped(knots, degree).unwrap())
                .lift(geom_core::Real::from_f64),
            pts.iter()
                .map(|p| Point3::new(iv(p[0]), iv(p[1]), iv(0.0)))
                .collect(),
            w.iter().map(|&x| iv(x)).collect(),
        )
        .unwrap()
    }

    fn domain(c: &NurbsCurve3<Interval>) -> (f64, f64) {
        let k = c.knot_values().values();
        (k[c.degree()].lo(), k[k.len() - 1 - c.degree()].lo())
    }

    fn z_area(c: &NurbsCurve3<Interval>, pieces: usize) -> (Interval, f64) {
        let r = c.control()[0];
        let (lo, hi) = domain(c);
        let e = rational_edge_q(c, iv(lo), iv(hi), true, r).unwrap();
        let (va, rem) = rational_vector_area_split(&e, pieces);
        assert!(
            va[0].mag() == 0.0 && va[1].mag() == 0.0,
            "a planar rim's vector area is along z: {va:?}"
        );
        (va[2], rem[2])
    }

    /// Composite Simpson on the definition at `f64`, 2²⁰ panels over the domain:
    /// a reference independent of the lane's algebra (it reads `P` and
    /// `P′` off the carrier's own evaluator, not `B × B′/w²`).
    fn simpson(c: &NurbsCurve3<Interval>) -> f64 {
        let f = c.map_scalar(|x: Interval| x.lo());
        let r = f.control()[0];
        let (lo, hi) = domain(c);
        let g = |t: f64| {
            let (p, d) = f.ders1(t);
            0.5 * ((p.x - r.x) * d.y - (p.y - r.y) * d.x)
        };
        let m = 1 << 20;
        #[allow(clippy::cast_precision_loss)]
        let h = (hi - lo) / m as f64;
        let mut acc = g(lo) + g(hi);
        for i in 1..m {
            #[allow(clippy::cast_precision_loss)]
            let t = lo + i as f64 * h;
            acc += if i % 2 == 1 { 4.0 } else { 2.0 } * g(t);
        }
        acc * h / 3.0
    }

    const R: f64 = 1.029_25;

    /// The vase's rim: a quarter circle of radius `R` as a rational
    /// quadratic (weights `1, √2/2, 1`), whose segment about its start is
    /// `R²·(π/2 − 1)/2` exactly.
    fn quarter() -> NurbsCurve3<Interval> {
        carrier(
            vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
            2,
            &[[R, 0.0], [R, R], [0.0, R]],
            &[1.0, core::f64::consts::FRAC_1_SQRT_2, 1.0],
        )
    }

    fn cases() -> Vec<(&'static str, NurbsCurve3<Interval>)> {
        let ab = vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0];
        vec![
            ("the vase's quarter circle", quarter()),
            (
                "an ellipse-type quadratic (w₁² < w₀w₂)",
                carrier(
                    ab.clone(),
                    2,
                    &[[1.0, 0.0], [1.5, 1.2], [-0.2, 1.0]],
                    &[1.0, 0.6, 1.3],
                ),
            ),
            (
                "a hyperbola-type quadratic (w₁² > w₀w₂)",
                carrier(
                    ab,
                    2,
                    &[[1.0, 0.0], [1.5, 1.2], [-0.2, 1.0]],
                    &[1.0, 1.8, 1.3],
                ),
            ),
            (
                "a rational cubic over two spans",
                carrier(
                    vec![0.0, 0.0, 0.0, 0.0, 0.4, 1.0, 1.0, 1.0, 1.0],
                    3,
                    &[[0.0, 0.0], [1.0, 2.0], [2.0, -1.0], [3.0, 1.0], [2.5, 2.5]],
                    &[1.0, 2.0, 0.5, 1.0, 1.5],
                ),
            ),
        ]
    }

    /// **Every rational rim the class holds measures, and its enclosure
    /// holds the truth**: the vase's quarter circle against its closed
    /// segment, and an ellipse-type and a hyperbola-type quadratic and a
    /// rational cubic across a simple interior knot against Simpson on
    /// the definition, at the piece counts a schedule visits. The
    /// enclosure tightens as `h⁵` (32× per doubling at the finest two).
    #[test]
    fn every_rational_rim_measures_and_its_enclosure_holds_the_truth() {
        let exact = 0.5 * R * R * (core::f64::consts::FRAC_PI_2 - 1.0);
        for (name, c) in cases() {
            let truth = if name.starts_with("the vase") {
                exact
            } else {
                simpson(&c)
            };
            let mut last = f64::INFINITY;
            for pieces in [16, 32, 64, 128, 256] {
                let (z, _) = z_area(&c, pieces);
                assert!(
                    z.is_certified() && z.lo() <= truth && truth <= z.hi(),
                    "{name}, {pieces} pieces: {z:?} must hold {truth:e}"
                );
                assert!(
                    z.width() < last,
                    "{name}: the enclosure tightens as the pieces double"
                );
                last = z.width();
            }
            assert!(
                last < 1e-8 * truth.abs(),
                "{name}: at 256 pieces the enclosure is {last:e} wide"
            );
        }
    }

    /// **The width the rule leaves is its remainder's**: at a resolution
    /// where the round reports what a later one removes, that removable
    /// part is all but the rounding of the enclosure — the remainder term
    /// is what holds the truth, so a rule without it would not.
    #[test]
    fn the_remainder_is_what_the_width_holds() {
        let c = quarter();
        let (z, removable) = z_area(&c, 16);
        assert!(
            removable > 0.0 && removable <= z.width(),
            "{removable:e} of {:e}",
            z.width()
        );
        assert!(
            removable >= 0.99 * z.width(),
            "{removable:e} of {:e}",
            z.width()
        );
    }

    /// Reversed traversal negates the term, to the bit.
    #[test]
    fn reversed_traversal_negates() {
        let c = quarter();
        let r = c.control()[0];
        let fwd = rational_edge_q(&c, iv(0.0), iv(1.0), true, r).unwrap();
        let rev = rational_edge_q(&c, iv(0.0), iv(1.0), false, r).unwrap();
        let (a, _) = rational_vector_area_split(&fwd, 32);
        let (b, _) = rational_vector_area_split(&rev, 32);
        assert_eq!(
            [a[2].lo(), a[2].hi()].map(f64::to_bits),
            [(-b[2]).lo(), (-b[2]).hi()].map(f64::to_bits)
        );
    }

    /// A part of the carrier interval: the span is clipped to it, and the
    /// quarter's first half holds the circular segment `R²·(θ − sin θ)/2`
    /// of the angle `θ` that `P(0.5)` subtends (the arc's parameter is not
    /// its angle, so `θ` is read off the point).
    #[test]
    fn a_part_of_the_carrier_interval_is_clipped() {
        let c = quarter();
        let r = c.control()[0];
        let e = rational_edge_q(&c, iv(0.0), iv(0.5), true, r).unwrap();
        let (va, _) = rational_vector_area_split(&e, 128);
        let f = c.map_scalar(|x: Interval| x.lo());
        let p = f.eval(0.5);
        // About the start, the region between the chord to `P(0.5)` and
        // the arc: the circular segment of the angle it subtends.
        let theta = p.y.atan2(p.x);
        let truth = 0.5 * R * R * (theta - theta.sin());
        assert!(
            va[2].lo() <= truth && truth <= va[2].hi(),
            "{:?} ∌ {truth:e}",
            va[2]
        );
    }

    /// Ends that are not exact parameters refuse typed.
    #[test]
    fn inexact_ends_refuse_typed() {
        let c = quarter();
        let r = c.control()[0];
        assert!(matches!(
            rational_edge_q(&c, Interval::from_bounds(0.0, 1e-9), iv(1.0), true, r),
            Err(PropsError::QuadratureUnsupported { .. })
        ));
    }
}
