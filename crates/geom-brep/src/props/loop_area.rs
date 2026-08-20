//! The vector-area boundary integral `(1/2)∮(p − ref)×dp` with exact
//! per-carrier closed forms (module docs of [`super`]).

use geom::{Curve3, NurbsCurve3};
use geom_core::spline::SpanLocate;
use geom_core::{Point3, Vec3};

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
/// - circular arc: `(1/2)·[(C − ref)×(p1 − p0) + R²·(t1 − t0)·axis]`,
///   negated for reversed traversal;
/// - ellipse arc (M5 PR 5): the same form with `R² → a·b` —
///   `(P−C)×dP = a·b·dθ·axis` exactly (the eccentric-anomaly
///   parameterization's constant areal rate), so
///   `(1/2)·[(C − ref)×(p1 − p0) + a·b·(t1 − t0)·axis]`;
/// - **non-rational B-spline** (D7 stage-1 promotion made this a
///   reachable at-rest state: a promoted plane keeps its parsed
///   spline boundary carriers): per knot span the integrand
///   `(P − ref)×P′` is a POLYNOMIAL of degree `2k − 1`, and the fixed
///   `k`-point Gauss–Legendre rule integrates that degree **exactly**
///   — a closed form in the same sense as the arc's, not a truncated
///   approximation (fixed nodes, fixed evaluation order, D9; the only
///   error is f64 rounding). Rational carriers keep the typed
///   refusal — their integrand is not polynomial, and no at-rest
///   analytic-surface body carries one whose face flux is answered
///   here (a rational NURBS *wall* declines tier 3 by surface kind
///   before any boundary integral runs).
///
/// # Errors
///
/// [`PropsError::Unimplemented`] on a rational `Nurbs` carrier;
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
            Curve3::Circle {
                center,
                axis,
                radius,
                ..
            } => {
                let w = center - ref_point;
                let chord = e.p1() - e.p0();
                (w.cross(chord) + axis * (radius.powi(2) * (e.t1 - e.t0))) * half
            }
            Curve3::Ellipse {
                center,
                axis,
                major,
                minor,
                ..
            } => {
                let w = center - ref_point;
                let chord = e.p1() - e.p0();
                (w.cross(chord) + axis * (major * minor * (e.t1 - e.t0))) * half
            }
            Curve3::Nurbs(ref payload) => nurbs_vector_area(payload, e.t0, e.t1, ref_point)?,
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

/// `(1/2)∫_{t0}^{t1} (P(t) − ref)×P′(t) dt` for a **non-rational**
/// B-spline carrier, exact (module docs of [`loop_vector_area`]: the
/// per-span polynomial integrand meets the fixed `k`-point Gauss rule
/// that integrates its degree exactly).
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
    if curve.weights().iter().any(|w| *w != 1.0) {
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
    let kv = curve.knots();
    let lo_spans = t0.locate_spans(kv);
    let hi_spans = t1.locate_spans(kv);
    let first = lo_spans.first.index().min(hi_spans.first.index());
    let last = lo_spans.last.index().max(hi_spans.last.index());
    let half = T::from_f64(0.5);
    let mut acc = Vec3::zero();
    for index in first..=last {
        // Emptiness check and span validation are one step.
        let Some(span) = kv.span(index) else { continue };
        let lo = t0.max(T::from_f64(kv.knots()[index]));
        let hi = t1.min(T::from_f64(kv.knots()[index + 1]));
        let half_len = ((hi - lo) * half).max(T::zero());
        let mid = (lo + hi) * half;
        for (x, w) in nodes.iter().zip(weights) {
            let t = mid + half_len * T::from_f64(*x);
            let p = curve.eval_in_span(span, t);
            let d = curve.deriv_in_span(span, t);
            acc = acc + (p - ref_point).cross(d) * (half_len * T::from_f64(*w));
        }
    }
    Ok(acc * half)
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    fn line_edge(a: Point3<f64>, b: Point3<f64>, forward: bool) -> LoopEdge<f64> {
        let d = b - a;
        let len = d.norm();
        LoopEdge {
            carrier: Curve3::Line {
                origin: a,
                dir: d * (1.0 / len),
            },
            t0: 0.0,
            t1: len,
            forward,
            start: 0,
            end: 0,
        }
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
        let e = LoopEdge {
            carrier: Curve3::Circle {
                center: Point3::new(1.0, 2.0, 3.0),
                axis: Vec3::new(0.0, 0.0, 1.0),
                radius: r,
                u_ref: Vec3::new(1.0, 0.0, 0.0),
            },
            t0: 0.0,
            t1: core::f64::consts::TAU,
            forward: true,
            start: 0,
            end: 0,
        };
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
