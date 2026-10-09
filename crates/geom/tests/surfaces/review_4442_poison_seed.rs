//! Review probe (PR 4442): the SURFACE evaluators' poison arm at
//! `Dual`. A poison parameter must answer poison in the derivative
//! channel too; a bare `from_f64(NaN)` would carry a zero derivative.
//! The curve evaluators have such a row; the surface ones did not.

use geom::NurbsSurface;
use geom_core::spline::KnotVector;
use geom_core::{Dual64, Point3};

#[test]
fn a_poison_surface_parameter_is_poison_in_every_dual_channel() {
    let ku = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0], 2).unwrap();
    let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let control: Vec<Point3<Dual64>> = (0..8)
        .map(|i| {
            let x = f64::from(i);
            Point3::new(
                Dual64::constant(x),
                Dual64::constant(x * x),
                Dual64::constant(1.0),
            )
        })
        .collect();
    let s = NurbsSurface::new(ku, kv, control, vec![1.0; 8]).unwrap();
    let u = Dual64::variable(f64::NAN);
    let v = Dual64::constant(0.5);
    let nan = |c: Dual64| c.value.is_nan() && c.deriv.is_nan();
    let p = s.eval(u, v);
    assert!(nan(p.x) && nan(p.y) && nan(p.z), "eval: {p:?}");
    let j = s.ders(u, v);
    for (name, w) in [
        ("du", j.du),
        ("dv", j.dv),
        ("duu", j.duu),
        ("duv", j.duv),
        ("dvv", j.dvv),
    ] {
        assert!(nan(w.x) && nan(w.y) && nan(w.z), "ders {name}: {w:?}");
    }
    let j3 = s.ders3(u, v);
    assert!(
        nan(j3.duuu.x) && nan(j3.jet.point.x),
        "ders3: {:?}",
        j3.duuu
    );
}
