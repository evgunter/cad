//! M5 PR 3 acceptance, surface interval lane: containment of f64
//! evaluations by interval evaluations for the NURBS cylinder patch —
//! single-cell and knot-straddling boxes — plus poison propagation.
//! (Containment legitimacy for pure ring arithmetic: the note at the
//! top of `tests/curves/nurbs_interval.rs`.)

#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#![allow(clippy::needless_range_loop)]

use geom::NurbsSurface;
use geom_core::spline::KnotVector;
use geom_core::{Bounds, Interval, Point3, Real, Vec3};

const SQRT2_2: f64 = core::f64::consts::FRAC_1_SQRT_2;
const HEIGHT: f64 = 3.0;

fn nurbs_cylinder() -> NurbsSurface<f64> {
    let c = Point3::new(-0.5, 4.0, 1.25);
    let x = Vec3::new(1.0 / 3.0, -2.0 / 3.0, 2.0 / 3.0);
    let y = Vec3::new(2.0 / 3.0, -1.0 / 3.0, -2.0 / 3.0);
    let ax = Vec3::new(2.0 / 3.0, 2.0 / 3.0, 1.0 / 3.0);
    let r = 2.5;
    let ring = [
        (1.0, 0.0),
        (1.0, 1.0),
        (0.0, 1.0),
        (-1.0, 1.0),
        (-1.0, 0.0),
        (-1.0, -1.0),
        (0.0, -1.0),
        (1.0, -1.0),
        (1.0, 0.0),
    ];
    let ws = [1.0, SQRT2_2, 1.0, SQRT2_2, 1.0, SQRT2_2, 1.0, SQRT2_2, 1.0];
    let mut control = Vec::with_capacity(18);
    let mut weights = Vec::with_capacity(18);
    for (i, (cx, cy)) in ring.iter().enumerate() {
        let base = c + x * (r * cx) + y * (r * cy);
        control.push(base);
        control.push(base + ax * HEIGHT);
        weights.push(ws[i]);
        weights.push(ws[i]);
    }
    let knots_u = KnotVector::clamped(
        vec![
            0.0, 0.0, 0.0, 0.25, 0.25, 0.5, 0.5, 0.75, 0.75, 1.0, 1.0, 1.0,
        ],
        2,
    )
    .unwrap();
    let knots_v = KnotVector::clamped(vec![0.0, 0.0, HEIGHT, HEIGHT], 1).unwrap();
    NurbsSurface::new(knots_u, knots_v, control, weights).unwrap()
}

fn contains(e: Interval, v: f64) -> bool {
    e.lo() <= v && v <= e.hi()
}

/// Containment on single-cell and knot-straddling (u) boxes for the
/// point and the full jet.
#[test]
fn interval_surface_eval_contains_f64() {
    let n = nurbs_cylinder();
    let ni = n.map_scalar(Interval::from_f64);
    for (ulo, uhi) in [(0.05, 0.2), (0.2, 0.3), (0.45, 0.8)] {
        for (vlo, vhi) in [(0.25, 1.0), (1.5, 2.75)] {
            let ui = Interval::from_bounds(ulo, uhi);
            let vi = Interval::from_bounds(vlo, vhi);
            let p = ni.eval(ui, vi);
            let jet = ni.ders(ui, vi);
            for i in 0..=24usize {
                let uf = ulo + (uhi - ulo) * i as f64 / 24.0;
                for j in 0..=8usize {
                    let vf = vlo + (vhi - vlo) * j as f64 / 8.0;
                    let pf = n.eval(uf, vf);
                    assert!(
                        contains(p.x, pf.x) && contains(p.y, pf.y) && contains(p.z, pf.z),
                        "eval containment at ({uf}, {vf}) in [{ulo},{uhi}]×[{vlo},{vhi}]"
                    );
                    let jf = n.ders(uf, vf);
                    assert!(contains(jet.du.x, jf.du.x) && contains(jet.du.y, jf.du.y));
                    assert!(contains(jet.dv.z, jf.dv.z) && contains(jet.duu.x, jf.duu.x));
                    assert!(contains(jet.duv.y, jf.duv.y) && contains(jet.dvv.z, jf.dvv.z));
                }
            }
        }
    }
}

/// Poison propagation: NaI parameters and poisoned control points
/// yield poisoned values, never a decision or a panic.
#[test]
fn surface_poison_propagates() {
    let n = nurbs_cylinder();
    let ni = n.map_scalar(Interval::from_f64);
    let p = ni.eval(Interval::from_f64(f64::NAN), Interval::from_f64(1.0));
    // Every channel: "poisoned values" is the claim above.
    assert!(p.x.is_poison() && p.y.is_poison() && p.z.is_poison());
    let mut ctrl = n.control().to_vec();
    ctrl[6] = Point3::new(f64::NAN, ctrl[6].y, ctrl[6].z);
    let np = NurbsSurface::new(
        n.knots_u().clone(),
        n.knots_v().clone(),
        ctrl,
        n.weights().to_vec(),
    )
    .unwrap();
    // Index 6 is (iu = 3, iv = 0): active near u ∈ [0.25, 0.5], v low.
    // The poisoned CHANNEL poisons and the others stay finite — the
    // partial answer named, not read on x alone.
    let near = np.eval(0.3, 0.2);
    assert!(near.x.is_nan() && near.y.is_finite() && near.z.is_finite());
    // Far cell unaffected (basis locality), on every channel.
    let far = np.eval(0.9, 2.9);
    assert!(far.x.is_finite() && far.y.is_finite() && far.z.is_finite());
}
