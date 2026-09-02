//! ADVERSARIAL review battery for M5 PR 3, surface interval lane
//! (reviewer-authored, scratch): F1 on the tensor grid — a ruled
//! surface over a genuine C0 u-kink (interior u-knot at multiplicity
//! p) attacked with boxes degenerate at, straddling, and adjacent to
//! the kink line, in both u and v; plus the placeholder poison at
//! Interval (F6).

#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#![allow(missing_docs)]

use geom::{NurbsSurface, Surface};
use geom_core::spline::KnotVector;
use geom_core::{Bounds, Interval, Point3, Real};

/// Ruled surface: the degree-2 C0-kink curve extruded linearly in v
/// (degree 1), weights varying along u.
fn kink_sheet() -> NurbsSurface<f64> {
    let ku = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 0.5, 1.0, 1.0, 1.0], 2).unwrap();
    let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let base = [
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(1.0, 2.0, 0.0),
        Point3::new(2.0, 0.0, 1.0),
        Point3::new(3.0, -2.0, 0.0),
        Point3::new(4.0, 0.0, -1.0),
    ];
    let wu = [1.0, 0.7, 1.3, 0.9, 1.0];
    let mut control = Vec::with_capacity(10);
    let mut weights = Vec::with_capacity(10);
    for (i, b) in base.iter().enumerate() {
        control.push(*b);
        control.push(Point3::new(b.x, b.y, b.z + 2.0));
        weights.push(wu[i]);
        weights.push(wu[i]);
    }
    NurbsSurface::new(ku, kv, control, weights).unwrap()
}

fn lift(s: &NurbsSurface<f64>) -> NurbsSurface<Interval> {
    let ctrl = s
        .control()
        .iter()
        .map(|p| {
            Point3::new(
                Interval::from_f64(p.x),
                Interval::from_f64(p.y),
                Interval::from_f64(p.z),
            )
        })
        .collect();
    NurbsSurface::new(
        s.knots_u().clone(),
        s.knots_v().clone(),
        ctrl,
        s.weights().to_vec(),
    )
    .unwrap()
}

fn contains(e: Interval, v: f64) -> bool {
    e.lo() <= v && v <= e.hi()
}

fn iv(lo: f64, hi: f64) -> Interval {
    if lo == hi {
        Interval::from_f64(lo)
    } else {
        Interval::from_bounds(lo, hi)
    }
}

#[test]
fn f1_surface_kink_line_boxes_contain_f64() {
    let s = kink_sheet();
    let si = lift(&s);
    let u0 = 0.5;
    let cases = [
        (u0, u0, 0.25, 0.75),
        (u0 - 1e-9, u0 + 1e-9, 0.0, 1.0),
        (0.4, u0, 0.1, 0.9),
        (u0, 0.6, 0.0, 0.5),
        (0.3, 0.7, 0.4, 0.6),
        (0.0, 1.0, 0.0, 1.0),
    ];
    for (ulo, uhi, vlo, vhi) in cases {
        let p = si.eval(iv(ulo, uhi), iv(vlo, vhi));
        let jet = si.ders(iv(ulo, uhi), iv(vlo, vhi));
        for i in 0..=40 {
            for j in 0..=10 {
                let uf = ulo + (uhi - ulo) * i as f64 / 40.0;
                let vf = vlo + (vhi - vlo) * j as f64 / 10.0;
                let pf = s.eval(uf, vf);
                assert!(
                    contains(p.x, pf.x) && contains(p.y, pf.y) && contains(p.z, pf.z),
                    "surface F1 containment fails on [{ulo},{uhi}]x[{vlo},{vhi}] at ({uf},{vf})"
                );
                assert!(
                    contains(jet.point.x, pf.x),
                    "ders point channel loses containment at ({uf},{vf})"
                );
            }
        }
        // Exactly on the kink line, both one-sided u-derivatives must be
        // enclosed when the box straddles it.
        if ulo < u0 && uhi > u0 {
            for vf in [vlo, (vlo + vhi) / 2.0, vhi] {
                let dl = s.ders(u0 - 1e-9, vf).du;
                let dr = s.ders(u0, vf).du;
                for d in [dl, dr] {
                    assert!(
                        contains(jet.du.x, d.x)
                            && contains(jet.du.y, d.y)
                            && contains(jet.du.z, d.z),
                        "one-sided du dropped on [{ulo},{uhi}] at v={vf}"
                    );
                }
            }
        }
    }
}

/// F6 at Interval: the placeholder payload evaluates to poison through
/// the enum arm (NaI / NaN brackets), matching the old unit-variant
/// totality behavior.
#[test]
fn f6_placeholder_poison_at_interval() {
    let s: Surface<Interval> = Surface::nurbs_placeholder();
    let p = s.eval(Interval::from_f64(0.5), Interval::from_f64(0.5));
    // ALL-poison, which is what the placeholder promises: a
    // first-channel read passes on a net poisoned only in x.
    assert!(
        p.x.is_poison() && p.y.is_poison() && p.z.is_poison(),
        "placeholder not all-poison at Interval: {p:?}"
    );
    let n = s.deriv_u(Interval::from_f64(0.5), Interval::from_f64(0.5));
    assert!(n.x.is_poison() && n.y.is_poison() && n.z.is_poison());
}
