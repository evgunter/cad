//! The vector-area boundary integral `(1/2)∮(p − ref)×dp` with exact
//! per-carrier closed forms (module docs of [`super`]).

use geom::{Curve3, NurbsCurve3};
use geom_core::spline::SpanLocate;
use geom_core::{Point3, Real, Vec3};

use super::{LoopEdge, PropsError};

/// The loop's vector area `(1/2)∮(p − ref_point)×dp`, traversed in the
/// loop's stored order — for a planar loop this is `n̂ · (signed area)`;
/// for a curved face's boundary it equals `∮_face n dA` by Stokes.
/// Translation-invariant in exact arithmetic (`∮dp = 0`); `ref_point`
/// conditions the floating evaluation (pass the face's anchor).
///
/// Per-edge closed forms (fixed association order, D9):
///
/// - line `a → b`: `(1/2)·(a − ref)×(b − ref)`;
/// - circular arc, `Δ = t1 − t0`: the triangle `ref, p0, p1` plus the
///   segment between the chord and the arc,
///   `(1/2)·[(p0 − ref)×(p1 − p0) + R²·(Δ − sin Δ)·axis]`, negated for
///   reversed traversal;
/// - ellipse arc: the same form with `R² → a·b` — `(P−C)×dP =
///   a·b·dθ·axis` exactly (the eccentric-anomaly parameterization's
///   constant areal rate), so the segment is the circle's under the
///   affine map, `(1/2)·[(p0 − ref)×(p1 − p0) + a·b·(Δ − sin Δ)·axis]`.
///
///   **An arc's lever is its start point, never its centre.** The
///   centre-referenced form `(C − ref)×(p1 − p0) + a·b·Δ·axis` is the
///   same number in ℝ, but its two terms are the sector's size and
///   cancel to the face's, so the chord's rounding reaches the sum
///   times `|C − ref|`. The chord still carries that rounding — `p0`
///   and `p1` are evaluated about `C`, to `ulp(|C|)` — but here it
///   meets only the lever `|p0 − ref|`, the face's own extent: the
///   residual is about `ulp(|C|)·|p0 − ref|` per arc, which for a
///   carrier inside D4's 1 km session box is at the scale of the
///   smallest ε the kernel runs at. That residual is what a closed
///   form's zero pad leaves unbounded.
/// - **non-rational B-spline** (D7 stage-1 promotion made this a
///   reachable at-rest state: a promoted plane keeps its parsed
///   spline boundary carriers): per knot span the integrand
///   `(P − ref)×P′` is a POLYNOMIAL of degree `2k − 1`, and the fixed
///   `k`-point Gauss–Legendre rule integrates that degree **exactly**
///   — a closed form in the same sense as the arc's, not a truncated
///   approximation (fixed nodes, fixed evaluation order, D9; the only
///   error is f64 rounding). Rational carriers keep the typed
///   refusal here — their integrand `B × B′/w²` is rational per span,
///   and no fixed rule integrates it exactly. A plane they bound takes
///   the certified lane instead ([`super::rational_rim`], through
///   topo's quadrature dispatch), which encloses that integral; this
///   arm's refusal is what a door with no quadrature lane (the
///   structural door, a `Dual` read) answers for it.
///
/// # Errors
///
/// [`PropsError::Unimplemented`] on a rational `Nurbs` carrier (the
/// certified lane, [`super::rational_rim`], reads it);
/// [`PropsError::QuadratureUnsupported`] past the Gauss table's
/// degree inventory.
pub fn loop_vector_area<T: SpanLocate>(
    edges: &[LoopEdge<T>],
    ref_point: Point3<T>,
) -> Result<Vec3<T>, PropsError> {
    let half = T::from_f64(0.5);
    let mut acc = Vec3::zero();
    for e in edges {
        let contrib = match e.carrier {
            Curve3::Line { .. } => {
                let a = e.p0() - ref_point;
                let b = e.p1() - ref_point;
                a.cross(b) * half
            }
            Curve3::Circle { axis, radius, .. } => {
                arc_vector_area(e, ref_point, axis, radius.powi(2)) * half
            }
            Curve3::Ellipse {
                axis,
                semi_u,
                semi_v,
                ..
            } => arc_vector_area(e, ref_point, axis, semi_u * semi_v) * half,
            Curve3::Nurbs(ref payload) => nurbs_vector_area(payload, e.t0, e.t1, ref_point)?,
            // The spiric's sector integral `∫ P × P′ dv` carries
            // `√((R + r cos v)² − d²)` against `sin v` and `cos v` — an
            // elliptic integral, no closed form. A face it bounds is
            // measured by the quadrature lane (`super::spiric`).
            Curve3::Spiric { .. } => return Err(PropsError::Unimplemented),
        };
        // Reversed traversal flips the line integral's sign.
        acc = if e.forward {
            acc + contrib
        } else {
            acc - contrib
        };
    }
    Ok(acc)
}

/// Twice an arc's vector area about `ref_point`, its areal rate
/// `rate·axis` per unit parameter: the triangle `ref_point, p0, p1` and
/// the segment the arc cuts off its chord (module docs of
/// [`loop_vector_area`]).
fn arc_vector_area<T: SpanLocate>(
    e: &LoopEdge<T>,
    ref_point: Point3<T>,
    axis: Vec3<T>,
    rate: T,
) -> Vec3<T> {
    let p0 = e.p0();
    (p0 - ref_point).cross(e.p1() - p0) + arc_segment_term(axis, rate, e.t1 - e.t0)
}

/// **The bulge of an arc over its chord** — the one statement of it:
/// `Δ − sin Δ` for the signed span `Δ`. Twice the area between a unit
/// circle's arc and its chord; an arc of the conic `c + a·cos θ +
/// b·sin θ` cuts off `(a × b)·(Δ − sin Δ)` of twice-area, for any
/// signed `Δ` and any `a`, `b`.
pub fn chord_bulge<T: Real>(span: T) -> T {
    span - span.sin()
}

/// **Twice the vector area between a conic arc and its chord**:
/// `axis·rate·(Δ − sin Δ)`, `rate` the arc's areal rate per unit of its
/// parameter (`R²` for a circle, the semi-axes' signed product for an
/// ellipse) and `Δ` its signed span — the one home of the segment term
/// every reading of a conic arc's area adds to its chord's.
pub fn arc_segment_term<T: Real>(axis: Vec3<T>, rate: T, span: T) -> Vec3<T> {
    axis * (rate * chord_bulge(span))
}

/// The fixed Gauss–Legendre rules on `[−1, 1]`, by node count `n`
/// (exact through polynomial degree `2n − 1`). The inventory covers
/// the exportable spline degrees with headroom; past it the caller
/// refuses typed rather than truncating.
fn gauss_rule(n: usize) -> Option<(&'static [f64], &'static [f64])> {
    // Correctly-rounded f64-nearest values (R1 m-2: five 16-digit
    // literals sat one ulp off; re-derived at 50-digit precision).
    const X2: f64 = 0.577_350_269_189_625_7; // 1/√3
    const X3: f64 = 0.774_596_669_241_483_4; // √(3/5)
    const W3A: f64 = 8.0 / 9.0;
    const W3B: f64 = 5.0 / 9.0;
    const X4A: f64 = 0.339_981_043_584_856_26;
    const X4B: f64 = 0.861_136_311_594_052_6;
    const W4A: f64 = 0.652_145_154_862_546_1;
    const W4B: f64 = 0.347_854_845_137_453_85;
    const X5A: f64 = 0.538_469_310_105_683_1;
    const X5B: f64 = 0.906_179_845_938_664;
    const W5O: f64 = 0.568_888_888_888_888_9;
    const W5A: f64 = 0.478_628_670_499_366_47;
    const W5B: f64 = 0.236_926_885_056_189_08;
    match n {
        1 => Some((&[0.0], &[2.0])),
        2 => Some((&[-X2, X2], &[1.0, 1.0])),
        3 => Some((&[-X3, 0.0, X3], &[W3B, W3A, W3B])),
        4 => Some((&[-X4B, -X4A, X4A, X4B], &[W4B, W4A, W4A, W4B])),
        5 => Some((&[-X5B, -X5A, 0.0, X5A, X5B], &[W5B, W5A, W5O, W5A, W5B])),
        _ => None,
    }
}

/// **Whether [`loop_vector_area`] reads this carrier**, exactly as it
/// does: a line, a circle, an ellipse, and a non-rational B-spline whose
/// degree the fixed Gauss inventory covers. It refuses a spiric, a
/// rational spline and a non-rational spline past that inventory (degree
/// over 5). The one spelling of that set, which `topo`'s quadrature
/// regions join across.
#[must_use]
pub fn vector_area_reads<T: SpanLocate>(carrier: &Curve3<T>) -> bool {
    match carrier {
        Curve3::Line { .. } | Curve3::Circle { .. } | Curve3::Ellipse { .. } => true,
        Curve3::Nurbs(c) => !is_rational(c) && gauss_rule(c.degree().max(1)).is_some(),
        Curve3::Spiric { .. } => false,
    }
}

/// Is this carrier rational: are its weights not all one plain constant
/// ([`NurbsCurve3::constant_weight`])? The one spelling of the question
/// the closed form refuses on ([`nurbs_vector_area`]) and the certified
/// lane answers ([`super::rational_rim`]).
#[must_use]
pub fn is_rational<T: SpanLocate>(curve: &NurbsCurve3<T>) -> bool {
    curve.constant_weight().is_none()
}

/// `(1/2)∫_{t0}^{t1} (P(t) − ref)×P′(t) dt` for a **non-rational**
/// B-spline carrier ([`NurbsCurve3::constant_weight`]), exact (module
/// docs of [`loop_vector_area`]: the per-span polynomial integrand meets the fixed `k`-point Gauss rule
/// that integrates its degree exactly). A rational carrier refuses
/// [`PropsError::Unimplemented`]; [`super::rational_rim`] encloses it.
///
/// Span walking is generic-scalar sound: the covered span range comes
/// from [`SpanLocate::locate_spans`] at the interval's ends, each
/// span's sub-interval is clipped with the scalar's own total
/// `min`/`max` (Q1: no raw comparisons), and a span the interval does
/// not reach clips to length ≤ 0, whose `max(0)` half-length zeroes
/// its contribution identically — no value branching anywhere.
fn nurbs_vector_area<T: SpanLocate>(
    curve: &NurbsCurve3<T>,
    t0: T,
    t1: T,
    ref_point: Point3<T>,
) -> Result<Vec3<T>, PropsError> {
    if is_rational(curve) {
        return Err(PropsError::Unimplemented);
    }
    let degree = curve.degree().max(1);
    let Some((nodes, weights)) = gauss_rule(degree) else {
        return Err(PropsError::QuadratureUnsupported {
            what: "a non-rational B-spline boundary carrier of degree > 5 — the exact \
                   Gauss inventory covers the exportable degrees; extend the table \
                   before integrating higher",
        });
    };
    let kv = curve.knots_f64_interim();
    let (Some(lo_spans), Some(hi_spans)) = (t0.locate_spans(kv), t1.locate_spans(kv)) else {
        // A poison end locates no span, and the area is poison in every
        // channel the ends carry.
        return Ok(Vec3::<T>::zero().map(|_| geom_core::spline::poison_from(t0 + t1)));
    };
    let first = lo_spans.first.index().min(hi_spans.first.index());
    let last = lo_spans.last.index().max(hi_spans.last.index());
    let half = T::from_f64(0.5);
    let mut acc = Vec3::zero();
    for index in first..=last {
        // Emptiness check and span validation are one step.
        let Some(span) = curve.span(index) else {
            continue;
        };
        let held = curve.knot_values().values();
        let lo = t0.max(held[index]);
        let hi = t1.min(held[index + 1]);
        let half_len = ((hi - lo) * half).max(T::zero());
        let mid = (lo + hi) * half;
        for (x, w) in nodes.iter().zip(weights) {
            let t = mid + half_len * T::from_f64(*x);
            // `P × P′` at one parameter: one order-1 basis pass
            // answers both halves of the integrand.
            let (p, d) = span.ders1_in_span(t);
            acc = acc + (p - ref_point).cross(d) * (half_len * T::from_f64(*w));
        }
    }
    Ok(acc * half)
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    /// A poison end locates no span, so the NURBS area is poison in every
    /// channel the ends carry — at `Dual` the derivative channel too,
    /// where a bare `from_f64(NaN)` would be a dual constant whose
    /// derivative reads as a zero.
    #[test]
    fn a_poison_end_integrates_to_poison_in_every_dual_channel() {
        use geom_core::Dual64;
        use geom_core::spline::KnotVector;
        let c = |x: f64, y: f64| {
            Point3::new(
                Dual64::constant(x),
                Dual64::constant(y),
                Dual64::constant(0.0),
            )
        };
        let curve = NurbsCurve3::new(
            geom_core::spline::Knots::from(
                KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0], 2).unwrap(),
            )
            .lift(geom_core::Real::from_f64),
            vec![c(0.0, 0.0), c(1.0, 2.0), c(2.0, 0.0), c(3.0, 1.0)],
            vec![geom_core::Real::one(); 4],
        )
        .unwrap();
        let a = nurbs_vector_area(
            &curve,
            Dual64::variable(f64::NAN),
            Dual64::constant(1.0),
            c(0.0, 0.0),
        )
        .unwrap();
        for v in [a.x, a.y, a.z] {
            assert!(v.value.is_nan() && v.deriv.is_nan(), "{a:?}");
        }
    }

    /// **A constant weight is the polynomial arm's**: the cubic on one
    /// net with every weight `2` integrates to the unit-weight curve's
    /// area to the bit (a power of two scales the homogeneous
    /// evaluation exactly). At `3` and `0.1` it agrees to a few ulps
    /// and no closer: the arm evaluates through the rational
    /// evaluator, whose `Σ Nᵢ·c·Pᵢ / Σ Nᵢ·c` rounds once more than the
    /// unit net's. A net whose weights vary refuses as the rational
    /// carrier this arm does not integrate, and the certified lane
    /// ([`super::super::rational_rim`]) encloses it: its bracket holds a
    /// composite Simpson reference on the definition.
    #[test]
    fn a_constant_weight_carrier_integrates_as_its_polynomial() {
        use geom_core::spline::KnotVector;
        let p = |x: f64, y: f64| Point3::new(x, y, 0.0);
        let area = |weights: Vec<f64>| {
            let curve = NurbsCurve3::new(
                KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0], 2).unwrap(),
                vec![p(0.0, 0.0), p(1.0, 2.0), p(2.0, 0.0), p(3.0, 1.0)],
                weights,
            )
            .unwrap();
            nurbs_vector_area(&curve, 0.1, 0.9, p(0.5, -1.0))
        };
        let unit = area(vec![1.0; 4]).unwrap();
        let twos = area(vec![2.0; 4]).unwrap();
        assert_eq!(
            [twos.x, twos.y, twos.z].map(f64::to_bits),
            [unit.x, unit.y, unit.z].map(f64::to_bits),
            "weights 2: {twos:?}, unit: {unit:?}"
        );
        for c in [3.0, 0.1] {
            let got = area(vec![c; 4]).unwrap();
            assert!(
                got.x == 0.0
                    && got.y == 0.0
                    && (got.z - unit.z).abs() <= 16.0 * f64::EPSILON * unit.z.abs(),
                "weights {c}: {got:?}, unit: {unit:?}"
            );
        }
        assert!(
            matches!(
                area(vec![1.0, 2.0, 2.0, 1.0]),
                Err(PropsError::Unimplemented)
            ),
            "a varying weight vector is rational"
        );
        let rational = NurbsCurve3::new(
            KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0], 2).unwrap(),
            vec![p(0.0, 0.0), p(1.0, 2.0), p(2.0, 0.0), p(3.0, 1.0)],
            vec![1.0, 2.0, 2.0, 1.0],
        )
        .unwrap();
        let reference = p(0.5, -1.0);
        let simpson = {
            let g = |t: f64| {
                let (q, d) = rational.ders1(t);
                0.5 * ((q - reference).cross(d)).z
            };
            let (m, lo, hi) = (1 << 16, 0.1, 0.9);
            let h = (hi - lo) / f64::from(m);
            let inner: f64 = (1..m)
                .map(|i| if i % 2 == 1 { 4.0 } else { 2.0 } * g(lo + f64::from(i) * h))
                .sum();
            (g(lo) + g(hi) + inner) * h / 3.0
        };
        use geom_core::Bounds;
        use geom_core::interval::certification::Certification;
        let iv = super::super::bracket::pt;
        let lane = super::super::rational_rim::rational_edge_q(
            &rational.map_scalar(iv),
            iv(0.1),
            iv(0.9),
            true,
            Point3::new(iv(0.5), iv(-1.0), iv(0.0)),
        )
        .unwrap();
        let (z, _) = super::super::rational_rim::rational_vector_area_split(&lane, 128);
        assert!(
            z[2].lo() <= simpson && simpson <= z[2].hi() && z[2].width() < 1e-9,
            "the certified lane holds the rational carrier's area: {:?} ∌ {simpson}",
            z[2]
        );
    }

    /// **[`vector_area_reads`] is the set [`loop_vector_area`] reads**:
    /// on a line, a circle, an ellipse, a polynomial spline of degree 2
    /// and of degree 5, a rational one, and a polynomial one of degree 6,
    /// the predicate answers exactly where the integral does.
    #[test]
    fn the_predicate_answers_where_the_integral_reads() {
        use geom_core::spline::KnotVector;
        let p = |x: f64, y: f64| Point3::new(x, y, 0.0);
        let spline = |degree: usize, weights: Vec<f64>| {
            let n = weights.len();
            let mut knots = vec![0.0; degree + 1];
            knots.extend(vec![1.0; degree + 1]);
            assert_eq!(n, degree + 1);
            Curve3::Nurbs(std::sync::Arc::new(
                NurbsCurve3::new(
                    KnotVector::clamped(knots, degree).unwrap(),
                    (0..n).map(|i| p(i as f64, (i % 2) as f64)).collect(),
                    weights,
                )
                .unwrap(),
            ))
        };
        let carriers = [
            (
                Curve3::Line {
                    origin: p(0.0, 0.0),
                    dir: Vec3::new(1.0, 0.0, 0.0),
                },
                true,
            ),
            (
                Curve3::Circle {
                    center: p(0.0, 0.0),
                    axis: Vec3::new(0.0, 0.0, 1.0),
                    radius: 1.0,
                    u_ref: Vec3::new(1.0, 0.0, 0.0),
                },
                true,
            ),
            (
                Curve3::Ellipse {
                    center: p(0.0, 0.0),
                    axis: Vec3::new(0.0, 0.0, 1.0),
                    semi_u: 2.0,
                    semi_v: 1.0,
                    u_ref: Vec3::new(1.0, 0.0, 0.0),
                },
                true,
            ),
            (spline(2, vec![1.0; 3]), true),
            (spline(5, vec![1.0; 6]), true),
            (spline(2, vec![1.0, 2.0, 1.0]), false),
            (spline(6, vec![1.0; 7]), false),
        ];
        for (carrier, reads) in carriers {
            let edge = LoopEdge::hand_built(carrier.clone(), 0.0, 1.0, true, 0, 0);
            let integral = loop_vector_area(core::slice::from_ref(&edge), p(0.5, -1.0));
            assert_eq!(vector_area_reads(&carrier), reads, "{carrier:?}");
            assert_eq!(integral.is_ok(), reads, "{carrier:?}: {integral:?}");
        }
    }

    fn line_edge(a: Point3<f64>, b: Point3<f64>, forward: bool) -> LoopEdge<f64> {
        let d = b - a;
        let len = d.norm();
        LoopEdge::hand_built(
            Curve3::Line {
                origin: a,
                dir: d * (1.0 / len),
            },
            0.0,
            len,
            forward,
            0,
            0,
        )
    }

    #[test]
    fn unit_square_vector_area_is_z() {
        let p = |x: f64, y: f64| Point3::new(x, y, 0.0);
        let edges = vec![
            line_edge(p(0.0, 0.0), p(1.0, 0.0), true),
            line_edge(p(1.0, 0.0), p(1.0, 1.0), true),
            line_edge(p(1.0, 1.0), p(0.0, 1.0), true),
            line_edge(p(0.0, 1.0), p(0.0, 0.0), true),
        ];
        let va = loop_vector_area(&edges, Point3::new(5.0, -3.0, 2.0)).unwrap();
        assert!((va.x).abs() < 1e-12 && (va.y).abs() < 1e-12);
        assert!((va.z - 1.0).abs() < 1e-12);
    }

    #[test]
    fn full_circle_vector_area_is_pi_r_squared_axis() {
        let r = 2.0;
        let e = LoopEdge::hand_built(
            Curve3::Circle {
                center: Point3::new(1.0, 2.0, 3.0),
                axis: Vec3::new(0.0, 0.0, 1.0),
                radius: r,
                u_ref: Vec3::new(1.0, 0.0, 0.0),
            },
            0.0,
            core::f64::consts::TAU,
            true,
            0,
            0,
        );
        let va = loop_vector_area(core::slice::from_ref(&e), Point3::origin()).unwrap();
        assert!((va.z - core::f64::consts::PI * r.powi(2)).abs() < 1e-12);
        assert!(va.x.abs() < 1e-12 && va.y.abs() < 1e-12);
    }

    #[test]
    fn reversed_traversal_negates() {
        let a = Point3::new(0.0, 0.0, 0.0);
        let b = Point3::new(1.0, 0.0, 0.0);
        let f = loop_vector_area(&[line_edge(a, b, true)], Point3::new(0.0, 1.0, 0.0)).unwrap();
        let r = loop_vector_area(&[line_edge(a, b, false)], Point3::new(0.0, 1.0, 0.0)).unwrap();
        assert_eq!(f.z.to_bits(), (-r.z).to_bits());
    }
}
