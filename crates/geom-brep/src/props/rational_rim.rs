//! **Certified vector area of a rational spline rim** — the plane cap a
//! `Curve3::Nurbs` with varying weights bounds
//! (`a-planar-face-bounded-by-a-rational-spline-rim-has-no-vector-area`, closed by PR 5689).
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
//! rational function per knot span, which no fixed Gauss rule
//! integrates exactly: the closed-form lane (`super::loop_vector_area`)
//! keeps its typed refusal, and this lane reads it through the curved
//! rims' certified rule ([`super::taylor`]: a Taylor node per piece and
//! the Lagrange remainder over it, the width falling as `h⁵`). The
//! integrand's operations are ring operations, the square of `w` and its
//! reciprocal. `w` is a convex combination of the weights on every piece,
//! so where they are positive it is too; its enclosure over a piece is
//! wider than its range, and where that enclosure reaches zero the
//! reciprocal is unbounded and the round reads no enclosure, which a finer
//! round, whose pieces are narrower, removes.
//!
//! # Built once per call, read per piece
//!
//! [`rational_edge_q`] extracts the carrier's Bézier spans over the edge's
//! carrier interval through geom-core's certified decomposition
//! ([`geom_core::spline::SplineCoeffs::bernstein_spans`], the edge's ends
//! as extra breaks): `B`'s three channels and `w`, each span a Bernstein
//! row, turned once into a power form about the span's centre. A round
//! carries each form to each piece by a Taylor shift and a scale
//! ([`piece_form`]), in `v = (t − p)/(q − p) − ½`, so Horner's argument is
//! never more than half a piece wide and its coefficients are the piece's
//! own: what a whole span's power series, read over the piece's interval,
//! loses to dependency on a high degree or a wide weight spread. Nothing
//! is rebuilt
//! as one call's schedule doubles the pieces; a sign walk that re-enters
//! the cap per round window rebuilds the edge per window, as it does a
//! spiric's.
//!
//! Each span is integrated in `t` over its part of the edge's carrier
//! interval, whose ends are knots or the edge's own parameters, exact
//! `f64` values: the rule's endpoint-bracket pad is never paid.

use geom::NurbsCurve3;
use geom_core::Bounds;
use geom_core::Point3;
use geom_core::interval::Interval;
use geom_core::interval::certification::Certification;
use geom_core::spline::KnotVector;
use geom_core::spline::pieces::binomial;

use super::PropsError;
use super::bracket::pt;
use super::quad::RVec3;
use super::taylor::{Jet, composite_by};

/// One Bézier span `[a, b]` of a rational rim, inside the edge's carrier
/// interval: `B`'s three channels and `w`, each as its power form about
/// the span's centre ([`centred_power`] of its Bernstein row).
#[derive(Clone, Debug)]
pub struct RationalSpanQ {
    a: f64,
    b: f64,
    forms: [Vec<Interval>; 4],
}

/// A rational rim edge for the cap's rule: its spans over the edge's
/// carrier interval, its degree and its traversal sense.
#[derive(Clone, Debug)]
pub struct RationalEdgeQ {
    spans: Vec<RationalSpanQ>,
    degree: usize,
    forward: bool,
}

impl RationalEdgeQ {
    /// How many Bézier spans the edge integrates over, each at the
    /// round's piece count.
    #[must_use]
    pub fn span_count(&self) -> usize {
        self.spans.len()
    }

    /// The carrier's degree, which a piece's cost grows with.
    #[must_use]
    pub fn degree(&self) -> usize {
        self.degree
    }
}

/// A Bernstein row on `[0, 1]` as the power form about `½`:
/// `Σ dⱼ vʲ` with `v = u − ½`, in interval arithmetic. First the power
/// form about `0`, `cₖ = C(p, k)·Σᵢ (−1)^(k−i)·C(k, i)·bᵢ`, then the shift
/// `dⱼ = Σ_{k≥j} C(k, j)·2^(j−k)·cₖ`, every factor exact.
fn centred_power(row: &[Interval]) -> Vec<Interval> {
    let p = row.len() - 1;
    let c: Vec<Interval> = (0..=p)
        .map(|k| {
            let sum = (0..=k).fold(Interval::zero(), |acc, i| {
                let sign = if (k - i) % 2 == 0 { 1.0 } else { -1.0 };
                acc + row[i] * pt(sign * binomial(k, i))
            });
            sum * pt(binomial(p, k))
        })
        .collect();
    (0..=p)
        .map(|j| {
            (j..=p).fold(Interval::zero(), |acc, k| {
                #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
                let half = 0.5f64.powi((k - j) as i32);
                acc + c[k] * pt(binomial(k, j) * half)
            })
        })
        .collect()
}

/// The power form's derivative: `(k + 1)·c_{k+1}`.
fn slope_form(power: &[Interval]) -> Vec<Interval> {
    #[allow(clippy::cast_precision_loss)]
    (1..power.len()).map(|k| power[k] * pt(k as f64)).collect()
}

fn horner(coeffs: &[Interval], s: Jet) -> Jet {
    coeffs
        .iter()
        .rev()
        .fold(Jet::cst(Interval::zero()), |acc, &c| {
            acc.mul(s).add(Jet::cst(c))
        })
}

/// **A rational rim edge, ready for the rule**: the carrier's Bézier
/// spans over `[t0, t1]`, each span's `B = A − ref·w` and `w` about
/// `reference` as Bernstein rows (module docs).
///
/// # Errors
///
/// [`PropsError::QuadratureUnsupported`] where the carrier interval's
/// ends or the carrier's knots are not exact parameters, or the
/// decomposition refuses a row.
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
    let degree = curve.degree();
    // The held knots, each an exact parameter: a wider one refuses.
    let refused = PropsError::QuadratureUnsupported {
        what: "a rational rim whose knots are not exact parameters, or whose Bézier \
               decomposition refuses",
    };
    let kv = curve
        .knot_values()
        .values()
        .iter()
        .map(|&k| exact(k))
        .collect::<Option<Vec<f64>>>()
        .and_then(|knots| KnotVector::clamped(knots, degree).ok())
        .ok_or(refused.clone())?;
    // The carrier in homogeneous coordinates about the reference, one
    // channel per row: `B`'s three, then `w`.
    let r = reference;
    let channels: [Vec<Interval>; 4] = core::array::from_fn(|c| {
        curve
            .control()
            .iter()
            .zip(curve.weights())
            .map(|(p, &w)| match c {
                0 => (p.x - r.x) * w,
                1 => (p.y - r.y) * w,
                2 => (p.z - r.z) * w,
                _ => w,
            })
            .collect()
    });
    let mut decomposed = Vec::with_capacity(4);
    for channel in &channels {
        decomposed.push(
            kv.with_coeffs(channel)
                .ok_or(refused.clone())?
                .bernstein_spans(&[t0, t1]),
        );
    }
    let breaks = decomposed[0].breaks().to_vec();
    let mut spans = Vec::new();
    for j in 0..breaks.len() - 1 {
        let (a, b) = (breaks[j], breaks[j + 1]);
        if a < t0 || b > t1 || a >= b {
            continue;
        }
        let rows: [&[Interval]; 4] = core::array::from_fn(|c| &decomposed[c].spans()[j][..]);
        if rows.iter().copied().flatten().any(|x| !x.is_certified()) {
            return Err(refused);
        }
        spans.push(RationalSpanQ {
            a,
            b,
            forms: rows.map(centred_power),
        });
    }
    Ok(RationalEdgeQ {
        spans,
        degree,
        forward,
    })
}

/// What one piece `[p, q]` of a span reads: `B` and `dB/dv` and `w` as
/// power forms about the piece's centre, in `v = (t − p)/(q − p) − ½`, and
/// `1/(q − p)`.
struct PieceForm {
    p: f64,
    inv_len: Interval,
    numer: [Vec<Interval>; 3],
    slope: [Vec<Interval>; 3],
    weight: Vec<Interval>,
}

/// A span's centred forms carried to the piece `[p, q]`: in the span's
/// own `s = (t − a)/(b − a)`, the piece is `s = ½ + δ + h·v` with
/// `δ = (s₀ + s₁)/2 − ½` and `h = s₁ − s₀`, so each form `Σ dₖ (s − ½)ᵏ`
/// becomes `Σ eⱼ vʲ`, `eⱼ = hʲ·Σ_{k≥j} C(k, j)·δ^(k−j)·dₖ` (the Taylor
/// shift by `δ`, then the scale by `h`). `δ` and `h` are enclosed, so
/// each `eⱼ` encloses the exact one.
fn piece_form(span: &RationalSpanQ, p: f64, q: f64) -> PieceForm {
    let (a, len) = (pt(span.a), pt(span.b) - pt(span.a));
    let (s0, s1) = ((pt(p) - a) / len, (pt(q) - a) / len);
    let delta = (s0 + s1) * pt(0.5) - pt(0.5);
    let h = s1 - s0;
    let local = |d: &[Interval]| -> Vec<Interval> {
        let n = d.len();
        let mut powers = Vec::with_capacity(n);
        let mut x = Interval::one();
        for _ in 0..n {
            powers.push(x);
            x = x * delta;
        }
        let mut scale = Interval::one();
        (0..n)
            .map(|j| {
                let e = (j..n).fold(Interval::zero(), |acc, k| {
                    acc + d[k] * powers[k - j] * pt(binomial(k, j))
                });
                let out = e * scale;
                scale = scale * h;
                out
            })
            .collect()
    };
    let numer: [Vec<Interval>; 3] = core::array::from_fn(|c| local(&span.forms[c]));
    PieceForm {
        p,
        inv_len: Interval::one() / (pt(q) - pt(p)),
        slope: core::array::from_fn(|c| slope_form(&numer[c])),
        numer,
        weight: local(&span.forms[3]),
    }
}

/// One rational rim edge's vector-area term `(1/2)∫ B × B′/w² dt` at
/// `pieces` per span, traversal folded in, and per component the part of
/// its width a later round removes.
#[must_use]
pub fn rational_vector_area_split(e: &RationalEdgeQ, pieces: usize) -> (RVec3, [f64; 3]) {
    let mut va = [Interval::zero(); 3];
    let mut removable = [0.0_f64; 3];
    for span in &e.spans {
        let (part, rem) = composite_by(
            pt(span.a),
            pt(span.b),
            pieces,
            |p, q| piece_form(span, p, q),
            |f, t| {
                // `v = (t − p)/(q − p) − ½`, enclosed: the centre the forms
                // are about, whatever `f64` the rule's node rounds to.
                let v = Jet::var(t)
                    .sub(Jet::cst(pt(f.p)))
                    .scale(f.inv_len)
                    .sub(Jet::cst(pt(0.5)));
                let b: [Jet; 3] = core::array::from_fn(|c| horner(&f.numer[c], v));
                // `dB/dt = dB/dv · dv/dt`.
                let db: [Jet; 3] =
                    core::array::from_fn(|c| horner(&f.slope[c], v).scale(f.inv_len));
                let scale = horner(&f.weight, v).sqr().recip().scale(pt(0.5));
                [
                    b[1].mul(db[2]).sub(b[2].mul(db[1])),
                    b[2].mul(db[0]).sub(b[0].mul(db[2])),
                    b[0].mul(db[1]).sub(b[1].mul(db[0])),
                ]
                .map(|c| c.mul(scale))
            },
        );
        for k in 0..3 {
            va[k] = va[k] + part[k];
            removable[k] += rem[k];
        }
    }
    (if e.forward { va } else { va.map(|x| -x) }, removable)
}

#[cfg(test)]
// The truths are quoted at the reference's 40 digits, rounded at the parse.
#[allow(clippy::unwrap_used, clippy::panic, clippy::excessive_precision)]
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

    /// Composite Simpson on the definition at `f64`, 2¹⁶ panels over the domain:
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
        let m = 1 << 16;
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

    /// A rational carrier in space at the interval scalar.
    fn carrier3(
        knots: &[f64],
        degree: usize,
        pts: &[[f64; 3]],
        w: &[f64],
    ) -> NurbsCurve3<Interval> {
        NurbsCurve3::new(
            geom_core::spline::Knots::from(KnotVector::clamped(knots.to_vec(), degree).unwrap())
                .lift(geom_core::Real::from_f64),
            pts.iter()
                .map(|p| Point3::new(iv(p[0]), iv(p[1]), iv(p[2])))
                .collect(),
            w.iter().map(|&x| iv(x)).collect(),
        )
        .unwrap()
    }

    #[allow(clippy::type_complexity)]
    fn hard_cases() -> Vec<(
        &'static str,
        NurbsCurve3<Interval>,
        [f64; 3],
        (f64, f64),
        [f64; 3],
    )> {
        let q = [0.0, 0.0, 0.0, 1.0, 1.0, 1.0];
        let base = [[1.0, 0.0, 0.0], [1.5, 1.2, 0.0], [-0.2, 1.0, 0.0]];
        let deg7: Vec<f64> = [0.0; 8].into_iter().chain([2.0; 8]).collect();
        vec![
            (
                "degree 7, weights 0.1 to 9",
                carrier3(
                    &deg7,
                    7,
                    &[
                        [0.0, 0.0, 0.0],
                        [1.0, 2.0, 0.0],
                        [2.0, -1.0, 0.0],
                        [3.0, 1.0, 0.0],
                        [2.5, 2.5, 0.0],
                        [1.0, 3.0, 0.0],
                        [0.0, 2.0, 0.0],
                        [-1.0, 1.0, 0.0],
                    ],
                    &[1.0, 9.0, 0.1, 4.0, 0.3, 6.0, 0.2, 1.0],
                ),
                [0.0, 0.0, 0.0],
                (0.0, 2.0),
                [0.0, 0.0, 2.992_793_367_736_956_826],
            ),
            (
                "a quintic over a double knot, weights 0.05 to 50",
                carrier3(
                    &[
                        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.3, 0.3, 0.7, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
                    ],
                    5,
                    &[
                        [0.0, 0.0, 0.0],
                        [1.0, 2.0, 0.1],
                        [2.0, -1.0, 0.0],
                        [3.0, 1.0, -0.2],
                        [2.5, 2.5, 0.0],
                        [1.0, 3.0, 0.0],
                        [0.0, 2.0, 0.3],
                        [-1.0, 1.0, 0.0],
                        [-0.5, 0.0, 0.0],
                    ],
                    &[1.0, 50.0, 0.05, 3.0, 0.2, 10.0, 0.5, 7.0, 1.0],
                ),
                [0.2, 0.3, 0.0],
                (0.0, 1.0),
                [
                    0.053_010_271_318_522_123_5,
                    0.119_777_451_817_015_245_7,
                    3.552_025_280_598_975_693,
                ],
            ),
            (
                "an end weight of 1e-6",
                carrier3(&q, 2, &base, &[1e-6, 1.0, 1.0]),
                [1.0, 0.0, 0.0],
                (0.0, 1.0),
                [0.0, 0.0, 0.969_993_597_114_767_398_7],
            ),
            (
                "a middle weight of 1e3",
                carrier3(&q, 2, &base, &[1.0, 1e3, 1.3]),
                [1.0, 0.0, 0.0],
                (0.0, 1.0),
                [0.0, 0.0, 0.969_991_841_666_348_673_7],
            ),
        ]
    }

    /// The edge's term about `r` over `[t0, t1]` at `pieces`.
    fn term(c: &NurbsCurve3<Interval>, r: [f64; 3], (t0, t1): (f64, f64), pieces: usize) -> RVec3 {
        let r = Point3::new(iv(r[0]), iv(r[1]), iv(r[2]));
        let e = rational_edge_q(c, iv(t0), iv(t1), true, r).unwrap();
        rational_vector_area_split(&e, pieces).0
    }

    /// **A rim of degree past 3, or of weights spread over decades, holds
    /// its 40-digit truth at every resolution, and narrows** (the truths:
    /// mpmath at 40 digits on the carrier's own Cox–de Boor evaluation,
    /// the quadrature split at its knots). A degree-7 span with weights
    /// from 0.1 to 9 is under `1e-3` wide by 256 pieces (`6.6e-8` by 1024,
    /// measured); a quintic over a double knot with weights from 0.05 to 50
    /// under `1e-3` by 1024 (`5.9e-8` by 4096); a quadratic of middle
    /// weight `1e3` under `0.02` by 4096 (`4.5e-6` by 16384). Read as one
    /// power series over the whole span, the degree-7 span had no finite
    /// enclosure below 1024 pieces. An end weight of `1e-6` holds its truth
    /// but is not live within the schedule's reach: `1/w²` peaks near
    /// `1e12` over about `1e-6` of the parameter, which equal pieces do not
    /// resolve
    /// (`work/spanfold/a-rational-rim-with-a-near-zero-weight-is-not-live-on-equal-pieces.md`).
    #[test]
    fn a_high_degree_or_wide_weight_rim_holds_its_truth_and_narrows() {
        for (name, c, r, ends, truth) in hard_cases() {
            let (finest, width) = match name {
                n if n.starts_with("degree 7") => (256, 1e-3),
                n if n.starts_with("a quintic") => (1024, 1e-3),
                n if n.starts_with("a middle weight") => (4096, 0.02),
                _ => (64, f64::INFINITY),
            };
            let mut pieces = 16;
            while pieces <= finest {
                let va = term(&c, r, ends, pieces);
                for k in 0..3 {
                    assert!(
                        va[k].is_certified() && va[k].lo() <= truth[k] && truth[k] <= va[k].hi(),
                        "{name}, {pieces} pieces, component {k}: {:?} ∌ {}",
                        va[k],
                        truth[k]
                    );
                }
                if pieces == finest {
                    assert!(
                        va[2].width() < width,
                        "{name}: {:e} wide at {pieces} pieces",
                        va[2].width()
                    );
                }
                pieces *= 4;
            }
        }
    }

    /// **A carrier with two distinct interior knots holds its 40-digit
    /// truth** in every component, read about a reference 2e4 away over
    /// a part of its interval that ends inside a span at each end: the
    /// spans' extraction, their breaks at the knots and at the edge's ends,
    /// and the restriction to each piece all have to land where the knots
    /// are. The truth agrees with a reviewer's independent mpmath reading
    /// to every digit printed.
    #[test]
    fn two_interior_knots_hold_the_40_digit_truth() {
        let c = carrier3(
            &[0.0, 0.0, 0.0, 0.0, 0.3, 0.6, 1.0, 1.0, 1.0, 1.0],
            3,
            &[
                [0.0, 0.0, 0.0],
                [1.0, 2.0, 0.5],
                [2.0, -1.0, 0.0],
                [3.0, 1.0, 1.0],
                [2.5, 2.5, 0.0],
                [1.0, 3.0, -1.0],
            ],
            &[1.0, 2.0, 0.5, 1.0, 1.5, 0.8],
        );
        let truth = [
            -6_941.638_973_648_976_952,
            -5_715.356_377_599_417_599,
            -14_975.945_515_598_630_74,
        ];
        let va = term(&c, [1e4, -2e4, 3e3], (0.1, 0.95), 64);
        for k in 0..3 {
            assert!(
                va[k].is_certified() && va[k].lo() <= truth[k] && truth[k] <= va[k].hi(),
                "component {k}: {:?} ∌ {}",
                va[k],
                truth[k]
            );
        }
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
