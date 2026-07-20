//! The vector-area boundary integral `(1/2)∮(p − ref)×dp` with exact
//! per-carrier closed forms (module docs of [`super`]).

use geom_core::{Point3, Real, Vec3};
use geom_curves::Curve3;

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
///   negated for reversed traversal.
///
/// # Errors
///
/// [`PropsError::Unimplemented`] on a `Nurbs` carrier.
pub fn loop_vector_area<T: Real>(
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
                (w.cross(chord) + axis * (radius * radius * (e.t1 - e.t0))) * half
            }
            Curve3::Nurbs => return Err(PropsError::Unimplemented),
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
        assert!((va.z - core::f64::consts::PI * r * r).abs() < 1e-12);
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
