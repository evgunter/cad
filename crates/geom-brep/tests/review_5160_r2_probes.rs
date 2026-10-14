//! Review probes for PR 5160 (r2): which surface a limb-2 `HullSup`
//! refusal of the plane × NURBS lane is about.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::shared::tol::{band, eps};
use geom::{NurbsCurve3, NurbsSurface, Surface};
use geom_core::spline::KnotVector;
use geom_core::{Point3, Vec3};

/// `z = k·x` over `[0, 1]²`, bilinear: meets the plane `z = 0` along `x = 0`.
fn slanted_wall(k: f64) -> NurbsSurface<f64> {
    let kv = || KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let control = vec![
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(0.0, 1.0, 0.0),
        Point3::new(1.0, 0.0, k),
        Point3::new(1.0, 1.0, k),
    ];
    NurbsSurface::new(kv(), kv(), control, vec![1.0; 4]).unwrap()
}

/// A cubic along `x = 0`, `y ∈ [0.1, 0.9]`, whose control net zigzags
/// `±δ` in `z`: the curve stays within `δ/3` of the plane, the net does
/// not, and `n` spans put its first hull past the subdivision budget.
fn zigzag(n: usize, delta: f64) -> NurbsCurve3<f64> {
    let p = 3;
    let mut kv = vec![0.0; p + 1];
    for i in 1..n {
        #[allow(clippy::cast_precision_loss)]
        kv.push(i as f64 / n as f64);
    }
    kv.extend(vec![1.0; p + 1]);
    let knots = KnotVector::clamped(kv.clone(), p).unwrap();
    let count = n + p;
    let control: Vec<Point3<f64>> = (0..count)
        .map(|i| {
            // Greville abscissa: y linear in t.
            let g: f64 = kv[i + 1..=i + p].iter().sum::<f64>() / p as f64;
            let z = if i % 2 == 0 { delta } else { -delta };
            let z = if i == 0 || i == count - 1 { 0.0 } else { z };
            Point3::new(0.0, 0.1 + 0.8 * g, z)
        })
        .collect();
    NurbsCurve3::new(knots, control, vec![1.0; count]).unwrap()
}

/// Degree 2 along `x = 0`, `y ∈ [0.1, 0.9]`, every interior knot doubled
/// (C0): on the plane `z = 0` at every knot, bulging `h` between them.
fn scallops(n: usize, h: f64) -> NurbsCurve3<f64> {
    let mut kv = vec![0.0; 3];
    for i in 1..n {
        #[allow(clippy::cast_precision_loss)]
        let t = i as f64 / n as f64;
        kv.extend([t, t]);
    }
    kv.extend([1.0; 3]);
    let knots = KnotVector::clamped(kv, 2).unwrap();
    let control: Vec<Point3<f64>> = (0..=2 * n)
        .map(|j| {
            #[allow(clippy::cast_precision_loss)]
            let t = j as f64 / (2 * n) as f64;
            let z = if j % 2 == 1 { 2.0 * h } else { 0.0 };
            Point3::new(0.0, 0.1 + 0.8 * t, z)
        })
        .collect();
    NurbsCurve3::new(knots, control, vec![1.0; 2 * n + 1]).unwrap()
}

#[test]
fn a_plane_side_scallop_refuses_at_hull_sup() {
    let plane = Surface::Plane {
        origin: Point3::new(0.0, 0.0, 0.0),
        normal: Vec3::new(0.0, 0.0, 1.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let wall = slanted_wall(10.0);
    for (n, h) in [(8192usize, 50.0), (16384, 50.0), (8192, 200.0)] {
        let carrier = scallops(n, h * eps());
        let got = geom_brep::plane_nurbs_limbs::<f64>(
            &geom::Curve3::Nurbs(std::sync::Arc::new(carrier.clone())),
            carrier.knot_values().domain(),
            &plane,
            &wall,
            1.0,
            band(),
        );
        match got {
            Ok(_) => println!("SCALLOP n {n} h {h}ε: certified"),
            Err(e) => println!("SCALLOP n {n} h {h}ε: {e:?}\n   => {e}"),
        }
    }
}

#[test]
fn a_plane_side_hull_sup_is_worded_as_the_spline_image() {
    let plane = Surface::Plane {
        origin: Point3::new(0.0, 0.0, 0.0),
        normal: Vec3::new(0.0, 0.0, 1.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let wall = slanted_wall(10.0);
    for (n, k) in [(16usize, 1.5), (4000, 1.5), (8000, 1.5), (4000, 2.5)] {
        let carrier = zigzag(n, k * eps());
        let got = geom_brep::plane_nurbs_limbs::<f64>(
            &geom::Curve3::Nurbs(std::sync::Arc::new(carrier.clone())),
            carrier.knot_values().domain(),
            &plane,
            &wall,
            1.0,
            band(),
        );
        match got {
            Ok(_) => println!("n {n} δ {k}ε: certified"),
            Err(e) => println!("n {n} δ {k}ε: {e:?}\n   => {e}"),
        }
    }
}
