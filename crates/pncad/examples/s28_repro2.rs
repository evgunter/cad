//! THE COUNTEREXAMPLE — a body that meshed WATERTIGHT before PR #648
//! and is refused `UnsupportedCurvedDomain` after it.
//!
//! Frustum wedge (partial revolve of a trapezoid), one boundary edge
//! split twice with the public `Body::split_edge`, body then placed
//! with a rigid transform. `mesh::tessellate` + `check_mesh`.
#![allow(clippy::all, clippy::pedantic)]

use std::f64::consts::FRAC_PI_2;

use geom_core::{Affine3, Point2, Point3, Tolerance, Vec2, Vec3};
use mesh::tessellate;
use mesh::validate::check_mesh;
use profile::{Profile, ProfileLoop, RawLoop, SketchPlane};
use sweep::{Revolution, RevolveAxis, revolve};
use topo::Body;

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

fn frustum_wedge() -> Body<f64> {
    let lp = <ProfileLoop<f64> as RawLoop<f64>>::polygon([
        p2(0.5, 0.0),
        p2(2.0, 0.0),
        p2(1.0, 2.0),
        p2(0.5, 2.0),
    ]);
    let prof = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tolerance::get())
        .unwrap();
    revolve(
        &prof,
        RevolveAxis {
            origin: p2(0.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        },
        Revolution::Partial(FRAC_PI_2),
    )
    .unwrap()
    .body
}

fn placement() -> Affine3<f64> {
    let irr = Vec3::new(0.3178_f64, 0.9412, -0.1109);
    let irr = irr * (1.0 / irr.norm());
    Affine3::from_parts(
        Affine3::rotation_about_axis(Point3::new(0.0, 0.0, 0.0), irr, 1.0 / 3.0).linear,
        Vec3::new(0.117, -0.339_1, 5.001_7),
    )
}

fn params(b: &Body<f64>, ek: topo::EdgeKey) -> (f64, f64) {
    let e = b.get_edge(ek).unwrap();
    b.get_curve_geom(e.curve)
        .unwrap()
        .certified()
        .unwrap()
        .params()
}

fn report(tag: &str, body: &Body<f64>) {
    match tessellate(body, 0.1) {
        Ok(m) => {
            let n: usize = m.patches.iter().map(|p| p.triangles.len()).sum();
            match check_mesh(&m) {
                Ok(()) => println!("  {tag}: OK {n} triangles, check_mesh CLEAN"),
                Err(e) => println!("  {tag}: OK {n} triangles, check_mesh DIRTY {e:?}"),
            }
        }
        Err(e) => println!("  {tag}: REFUSED {e:?}"),
    }
}

fn main() {
    let base = frustum_wedge();
    let ek = base.edges().map(|(k, _)| k).next().unwrap();
    println!("first edge = {ek:?}");

    report("[1] as authored", &base);
    report(
        "[2] placed, no split",
        &topo::transform_rigid(&base, &placement()).unwrap(),
    );

    let mut split = base.clone();
    let (t0, t1) = params(&split, ek);
    split.split_edge(ek, t0 + (t1 - t0) * 0.312_9).unwrap();
    split
        .split_edge(ek, t0 + (t1 - t0) * (0.312_9 * 0.5))
        .unwrap();
    report("[3] split twice, not placed", &split);

    let placed = topo::transform_rigid(&split, &placement()).unwrap();
    report("[4] split twice THEN placed  <-- THE COUNTEREXAMPLE", &placed);
}
