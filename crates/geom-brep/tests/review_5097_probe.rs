//! Review 5097 probe: spline-carrier limbs, printed bit-exact (Debug of
//! f64 round-trips), for comparison between origin/main and the PR head.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::print_stdout)]

use crate::shared::fixture::quarter_cylinder_wall;
use crate::shared::tol::{band, eps};
use geom::{NurbsCurve3, NurbsSurface, Surface};
use geom_core::spline::KnotVector;
use geom_core::{Point3, Vec3};

fn bilinear(c: [Point3<f64>; 4]) -> NurbsSurface<f64> {
    let k = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    NurbsSurface::new(k.clone(), k, c.to_vec(), vec![1.0; 4]).unwrap()
}
fn flat_wall() -> NurbsSurface<f64> {
    let p = |x, y| Point3::new(x, y, 0.0);
    bilinear([p(0.0, 0.0), p(0.0, 1.0), p(1.0, 0.0), p(1.0, 1.0)])
}
fn diagonal_wall() -> NurbsSurface<f64> {
    let at = |t: f64, z| Point3::new(0.1 + 0.6 * t, 0.1 + 0.8 * t, z);
    bilinear([at(-0.1, -0.5), at(-0.1, 0.5), at(1.1, -0.5), at(1.1, 0.5)])
}
fn horizontal_wall(h: f64) -> NurbsSurface<f64> {
    let p = |x, y| Point3::new(x, y, h);
    bilinear([p(-0.1, -0.1), p(-0.1, 1.1), p(1.1, -0.1), p(1.1, 1.1)])
}
fn line(t0: f64, t1: f64) -> NurbsCurve3<f64> {
    let at = |t: f64| Point3::new(0.1 + 0.6 * t, 0.1 + 0.8 * t, 0.0);
    NurbsCurve3::new(KnotVector::clamped(vec![t0, t0, t1, t1], 1).unwrap(), vec![at(t0), at(t1)], vec![1.0, 1.0]).unwrap()
}
fn arc(z: f64, a: f64, b: f64) -> NurbsCurve3<f64> {
    let (h, m) = (0.5 * (b - a), 0.5 * (a + b));
    let at = |t: f64| Point3::new(t.cos(), t.sin(), z);
    let mid = Point3::new(m.cos() / h.cos(), m.sin() / h.cos(), z);
    NurbsCurve3::new(KnotVector::clamped(vec![a, a, a, b, b, b], 2).unwrap(), vec![at(a), mid, at(b)], vec![1.0, h.cos(), 1.0]).unwrap()
}
/// A cubic with interior knots through tilted-section points (near, not on, the wall).
fn cubic() -> NurbsCurve3<f64> {
    let k = KnotVector::clamped(vec![0.1, 0.1, 0.1, 0.1, 0.5, 0.9, 1.4, 1.4, 1.4, 1.4], 3).unwrap();
    let pts: Vec<Point3<f64>> = (0..6).map(|i| { let t = 0.1 + 1.3 * f64::from(i) / 5.0; Point3::new(t.cos(), t.sin(), 0.4 + 0.3 * t.cos()) }).collect();
    NurbsCurve3::new(k, pts, vec![1.0; 6]).unwrap()
}
fn horizontal(h: f64) -> Surface<f64> {
    Surface::Plane { origin: Point3::new(0.0, 0.0, h), normal: Vec3::new(0.0, 0.0, 1.0), u_ref: Vec3::new(1.0, 0.0, 0.0) }
}
fn diagonal_plane() -> Surface<f64> {
    Surface::Plane { origin: Point3::new(0.1, 0.1, 0.0), normal: Vec3::new(0.8, -0.6, 0.0), u_ref: Vec3::new(0.6, 0.8, 0.0) }
}
fn tilted() -> Surface<f64> {
    let k = 0.3_f64; let s = (1.0 + k * k).sqrt();
    Surface::Plane { origin: Point3::new(0.0, 0.0, 0.4), normal: Vec3::new(-k / s, 0.0, 1.0 / s), u_ref: Vec3::new(1.0 / s, 0.0, k / s) }
}

#[test]
fn review_5097_spline_bits() {
    let short = 0.7 + 0.002 * (eps() / 1e-9).sqrt().min(20.0);
    let qc = quarter_cylinder_wall();
    println!("BITS eps={:e}", eps());
    println!("BITS pn line {:?}", pnl(&line(0.0, 1.0), &diagonal_plane(), &flat_wall()));
    println!("BITS pn arc13 {:?}", pnl(&arc(0.5, 0.1, 1.4), &horizontal(0.5), &qc));
    println!("BITS pn arcshort {:?}", pnl(&arc(0.5, 0.7, short), &horizontal(0.5), &qc));
    println!("BITS pn cubic {:?}", pnl(&cubic(), &tilted(), &qc));
    println!("BITS nn line {:?}", pair(&line(0.0, 1.0), &flat_wall(), &diagonal_wall()));
    println!("BITS nn arcshort {:?}", pair(&arc(0.5, 0.7, short), &qc, &horizontal_wall(0.5)));
    println!("BITS nn arc13 {:?}", pair(&arc(0.5, 0.1, 1.4), &qc, &horizontal_wall(0.5)));
    let _ = band();
}

fn pnl(c: &NurbsCurve3<f64>, p: &Surface<f64>, w: &NurbsSurface<f64>) -> String {
    let cc = geom::Curve3::Nurbs(std::sync::Arc::new(c.clone()));
    format!("{:?}", geom_brep::plane_nurbs_limbs(&cc, c.knot_values().domain(), p, w, 1.0, band()))
}
fn pair(c: &NurbsCurve3<f64>, a: &NurbsSurface<f64>, b: &NurbsSurface<f64>) -> String {
    let cc = geom::Curve3::Nurbs(std::sync::Arc::new(c.clone()));
    format!("{:?}", geom_brep::nurbs_pair_limbs(&cc, c.knot_values().domain(), a, b, 1.0, band()))
}

/// Closed-form carriers at the Interval scalar: the verdict and bound
/// beside the f64 lane's.
#[test]
fn review_5097_closed_forms_at_interval() {
    use geom::Curve3;
    use geom_core::interval::Interval;
    use geom_core::interval::certification::Certification;
    let iv = Interval::point;
    let circle = Curve3::Circle { center: Point3::new(0.0, 0.0, 0.5), axis: Vec3::new(0.0, 0.0, 1.0), radius: 1.0, u_ref: Vec3::new(1.0, 0.0, 0.0) };
    let k = 0.3_f64; let s = (1.0 + k * k).sqrt();
    let ellipse = Curve3::Ellipse { center: Point3::new(0.0, 0.0, 0.4), axis: Vec3::new(-k / s, 0.0, 1.0 / s), major: s, minor: 1.0, u_ref: Vec3::new(1.0 / s, 0.0, k / s) };
    let diag = Curve3::Line { origin: Point3::new(0.1, 0.1, 0.0), dir: Vec3::new(0.6, 0.8, 0.0) };
    let short = 0.7 + 0.002 * (eps() / 1e-9).sqrt().min(20.0);
    let qc = quarter_cylinder_wall();
    let rows: Vec<(&str, Curve3<f64>, (f64, f64), Surface<f64>, NurbsSurface<f64>)> = vec![
        ("line", diag, (0.0, 1.0), diagonal_plane(), flat_wall()),
        ("circle short", circle.clone(), (0.7, short), horizontal(0.5), qc.clone()),
        ("circle 1.3", circle, (0.1, 1.4), horizontal(0.5), qc.clone()),
        ("ellipse 1.3", ellipse, (0.1, 1.4), tilted(), qc.clone()),
    ];
    for (name, c, (t0, t1), p, w) in rows {
        let f = geom_brep::plane_nurbs_limbs(&c, (t0, t1), &p, &w, 1.0, band());
        let ci = c.map_scalar(iv);
        let pi = p.map_scalar(iv);
        let wi = w.map_scalar(iv);
        let i = geom_brep::plane_nurbs_limbs(&ci, (iv(t0), iv(t1)), &pi, &wi, iv(1.0), band());
        let short = |r: &Result<geom_brep::PlaneNurbsLimbs<_>, geom_brep::PlaneNurbsRefusal>| match r { Ok(l) => format!("OK hull {:?} locus {:?}", l.hull_sup, l.on_locus_max), Err(e) => format!("ERR {e}") };
        println!("IV {name}: f64 {} | interval {}", short(&f), match &i { Ok(l) => format!("OK hull {:?} locus {:?}", l.hull_sup, l.on_locus_max), Err(e) => format!("ERR {e}") });
    }
}
