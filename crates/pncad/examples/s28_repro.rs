//! MINIMAL REPRODUCER — S28 guard false refusal.
//!
//! A plain tube (full revolve of a rectangle profile), one boundary
//! edge split with the PUBLIC `Body::split_edge`, then placed with a
//! rigid transform, then `mesh::tessellate`.
#![allow(clippy::all, clippy::pedantic)]

use geom_core::{Affine3, Point2, Point3, Tolerance, Vec2, Vec3};
use mesh::tessellate;
use mesh::validate::check_mesh;
use profile::{Profile, ProfileLoop, RawLoop, SketchPlane};
use sweep::{Revolution, RevolveAxis, revolve};
use topo::Body;

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

fn tube() -> Body<f64> {
    let lp = <ProfileLoop<f64> as RawLoop<f64>>::polygon([
        p2(1.0, 0.0),
        p2(2.0, 0.0),
        p2(2.0, 1.0),
        p2(1.0, 1.0),
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
        Revolution::Full,
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

fn report(tag: &str, body: &Body<f64>) {
    for d in [0.25, 0.1, 0.04, 0.01] {
        match tessellate(body, d) {
            Ok(m) => {
                let tris: usize = m.patches.iter().map(|p| p.triangles.len()).sum();
                let clean = match check_mesh(&m) {
                    Ok(()) => "clean".to_string(),
                    Err(e) => format!("DIRTY {e:?}"),
                };
                println!("  {tag} d={d}: OK  {tris} triangles, check_mesh {clean}");
            }
            Err(e) => println!("  {tag} d={d}: REFUSED {e:?}"),
        }
    }
}

fn main() {
    let base = tube();
    println!("[A] tube, as authored");
    report("A", &base);

    let placed = topo::transform_rigid(&base, &placement()).unwrap();
    println!("[B] tube, placed (rigid rot+translate) — NO split");
    report("B", &placed);

    // The seam line edge of the r = 2 wall. EdgeKey(1v1) in the
    // sweep's minting order; found by carrier kind + face to stay
    // robust.
    let ek = base
        .edges()
        .map(|(k, _)| k)
        .find(|&k| {
            let e = base.get_edge(k).unwrap();
            let c = base.get_curve_geom(e.curve).unwrap().certified().unwrap();
            matches!(c.carrier(), geom_curves::Curve3::Line { .. })
        })
        .unwrap();
    let mut split = base.clone();
    let (t0, t1) = {
        let e = split.get_edge(ek).unwrap();
        let c = split.get_curve_geom(e.curve).unwrap().certified().unwrap();
        c.params()
    };
    split.split_edge(ek, t0 + (t1 - t0) * 0.312_9).unwrap();
    println!("[C] tube, edge {ek:?} split at 0.3129 — NOT placed");
    report("C", &split);

    let split_placed = topo::transform_rigid(&split, &placement()).unwrap();
    println!("[D] tube, split THEN placed  <-- the reproducer");
    report("D", &split_placed);

    // and the other order, to show it is the pair and not the order
    let mut placed_split = placed.clone();
    let ek2 = placed_split
        .edges()
        .map(|(k, _)| k)
        .find(|&k| {
            let e = placed_split.get_edge(k).unwrap();
            let c = placed_split
                .get_curve_geom(e.curve)
                .unwrap()
                .certified()
                .unwrap();
            matches!(c.carrier(), geom_curves::Curve3::Line { .. })
        })
        .unwrap();
    let (t0, t1) = {
        let e = placed_split.get_edge(ek2).unwrap();
        let c = placed_split
            .get_curve_geom(e.curve)
            .unwrap()
            .certified()
            .unwrap();
        c.params()
    };
    placed_split
        .split_edge(ek2, t0 + (t1 - t0) * 0.312_9)
        .unwrap();
    println!("[E] tube, placed THEN split");
    report("E", &placed_split);
}
