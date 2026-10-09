//! Review probes for PR 4442: the surface evaluators' poison arms.
//!
//! The curve evaluators' poison is pinned at `Dual` by
//! `curves_nurbs_differential::a_poison_parameter_evaluates_to_poison_in_every_channel`.
//! Nothing pinned the surface's: replacing `poison`'s `seed × NaN` with
//! a bare `from_f64(NaN)`, a dual CONSTANT with a zero derivative,
//! left every test green.

use geom::NurbsSurface;
use geom_core::spline::KnotVector;
use geom_core::{Dual64, Point3, Vec3};

fn dual_patch() -> NurbsSurface<Dual64> {
    let ku = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0], 2).unwrap();
    let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let mut control = Vec::new();
    for i in 0..4 {
        for j in 0..2 {
            let p = Point3::new(f64::from(i), f64::from(j), f64::from(i * j) * 0.3);
            control.push(p.map(Dual64::constant));
        }
    }
    NurbsSurface::new(ku, kv, control, vec![1.0; 8]).unwrap()
}

fn all_poison(v: &[Vec3<Dual64>]) -> bool {
    v.iter().all(|c| {
        [c.x, c.y, c.z]
            .iter()
            .all(|d| d.value.is_nan() && d.deriv.is_nan())
    })
}

#[test]
fn a_poison_surface_parameter_evaluates_to_poison_in_every_dual_channel() {
    let s = dual_patch();
    for (name, u, v) in [
        ("u", Dual64::variable(f64::NAN), Dual64::constant(0.5)),
        ("v", Dual64::constant(0.5), Dual64::variable(f64::NAN)),
    ] {
        let p = s.eval(u, v);
        assert!(all_poison(&[p - Point3::origin()]), "{name}: eval {p:?}");
        let j = s.ders(u, v);
        assert!(
            all_poison(&[j.point - Point3::origin(), j.du, j.dv, j.duu, j.duv, j.dvv]),
            "{name}: ders {j:?}"
        );
        let j3 = s.ders3(u, v);
        assert!(
            all_poison(&[j3.duuu, j3.duuv, j3.duvv, j3.dvvv]),
            "{name}: ders3 {j3:?}"
        );
    }
}
