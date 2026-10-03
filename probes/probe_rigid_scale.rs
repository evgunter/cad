//! Reviewer probe (PR #3964): the row's fixture at scale `s`, under
//! rotations, translations and other δ placements. Prints, per
//! configuration, limb 1's worst distance from the closed-form field in
//! ulps of the seated R and of the image's R, and limb 2's worst drift
//! as a fraction of the floor at that scale. Asserts nothing.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{NurbsCurve3, NurbsSurface, Surface};
use geom_core::spline::KnotVector;
use geom_core::{Affine3, Band, Point3, Vec3};

const W: f64 = core::f64::consts::FRAC_1_SQRT_2;

fn kv2() -> KnotVector {
    KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap()
}
fn kv1() -> KnotVector {
    KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap()
}
fn seat() -> Affine3<f64> {
    Affine3::rotation_about_axis(Point3::origin(), Vec3::new(0.0, 0.0, 1.0), -core::f64::consts::FRAC_PI_4)
}
fn wall(s: f64) -> NurbsSurface<f64> {
    let c = vec![
        Point3::new(s, 0.0, 0.0),
        Point3::new(s, 0.0, s),
        Point3::new(s, s, 0.0),
        Point3::new(s, s, s),
        Point3::new(0.0, s, 0.0),
        Point3::new(0.0, s, s),
    ];
    let m = seat();
    NurbsSurface::new(kv2(), kv1(), c, vec![1.0, 1.0, W, W, 1.0, 1.0])
        .unwrap()
        .map_points(|p| m.transform_point(p))
}
fn plane(s: f64) -> Surface<f64> {
    Surface::Plane {
        origin: Point3::new(0.0, 0.0, 0.5 * s),
        normal: Vec3::new(0.0, 0.0, 1.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    }
}
fn arc(r: f64, s: f64) -> NurbsCurve3<f64> {
    let m = seat();
    let c = vec![Point3::new(r, 0.0, 0.5 * s), Point3::new(r, r, 0.5 * s), Point3::new(0.0, r, 0.5 * s)];
    NurbsCurve3::new(kv2(), c, vec![1.0, W, 1.0]).unwrap().map_points(|p| m.transform_point(p))
}
fn rmax(w: &NurbsSurface<f64>) -> f64 {
    w.control().iter().map(|p| p.distance(Point3::origin())).fold(0.0, f64::max)
}
fn limbs(c: &NurbsCurve3<f64>, p: &Surface<f64>, w: &NurbsSurface<f64>, s: f64, band: Band) -> geom_brep::PlaneNurbsLimbs<f64> {
    geom_brep::plane_nurbs_limbs::<f64>(c, p, w, s, band).unwrap_or_else(|e| panic!("refused: {e}"))
}
fn maps(translate: f64) -> Vec<Affine3<f64>> {
    let axes = [Vec3::new(0.0, 0.0, 1.0), Vec3::new(1.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 1.0), Vec3::new(0.3, -0.4, 0.8)];
    let mut out = Vec::new();
    for a in axes {
        for k in 1..=8 {
            let r = Affine3::rotation_about_axis(Point3::origin(), a.normalize(), f64::from(k) * 0.135);
            let t = Affine3::translation(Vec3::new(0.37, -0.81, 0.45) * translate);
            out.push(t * r);
        }
    }
    out
}

#[test]
fn probe_scale_translation_delta() {
    let s_list: Vec<f64> = std::env::var("PROBE_S").map(|v| v.split(',').map(|x| x.parse().unwrap()).collect()).unwrap_or(vec![1e-3, 1.0, 1e3]);
    for s in s_list {
        let band = Band::new(1e-4 * s, 1e-3 * s).unwrap();
        let floor = match geom_brep::plane_nurbs_limbs::<f64>(&arc(s, s), &plane(s), &wall(s), s, band) {
            Ok(l) => l.hull_sup,
            Err(e) => { println!("PROBE s={s:e}: floor at field 0 refused: {e}"); f64::NAN }
        };
        let r0 = rmax(&wall(s));
        for f_rel in [1e-12, 1e-9, 1e-6, 3.3e-11] {
            let r_arc = s * (1.0 + f_rel);
            let field = r_arc - s;
            let carrier = arc(r_arc, s);
            let seated = match geom_brep::plane_nurbs_limbs::<f64>(&carrier, &plane(s), &wall(s), s, band) {
                Ok(l) => l,
                Err(e) => { println!("PROBE s={s:e} field/s={f_rel:e}: seated refused: {e}"); continue; }
            };
            for tr in [0.0, s, 10.0 * s, 1000.0 * s] {
                let (mut l1_seat, mut l1_img, mut l2) = ((seated.on_locus_max - field).abs() / (f64::EPSILON * r0), 0.0f64, 0.0f64);
                let l1_seated = l1_seat;
                for m in maps(tr) {
                    let Surface::Plane { origin, normal, u_ref } = plane(s) else { unreachable!() };
                    let mp = Surface::Plane { origin: m.transform_point(origin), normal: m.linear * normal, u_ref: m.linear * u_ref };
                    let w = wall(s).map_points(|p| m.transform_point(p));
                    let img = match geom_brep::plane_nurbs_limbs::<f64>(&carrier.map_points(|p| m.transform_point(p)), &mp, &w, s, band) {
                        Ok(i) => i,
                        Err(e) => { println!("PROBE s={s:e} field/s={f_rel:e} translate={tr:e}: image refused: {e}"); continue; }
                    };
                    let off = (img.on_locus_max - field).abs();
                    l1_seat = l1_seat.max(off / (f64::EPSILON * r0));
                    l1_img = l1_img.max(off / (f64::EPSILON * rmax(&w)));
                    l2 = l2.max((img.hull_sup - seated.hull_sup).abs() / floor);
                }
                println!(
                    "PROBE s={s:e} field/s={f_rel:e} translate={tr:e} floor={floor:.3e} floor/s={:.3e}: seated limb1 {l1_seated:.2} ulps(R0); worst limb1 {l1_seat:.2} ulps(R0) {l1_img:.2} ulps(R_img); limb2 drift {l2:.3} floor",
                    floor / s
                );
            }
        }
    }
}
