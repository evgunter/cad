//! **Probe (measurement only): a boolean edge through an isolated
//! tangency point.** Ev's question was when a "mixed" edge happens: an
//! edge whose interior stations read definitely transverse at some and
//! definitely smooth at others. The simplest real candidates are
//! surface pairs whose section curve passes through a point where the
//! two surfaces touch:
//!
//! - equal-radius cylinders with crossing axes (Steinmetz): the two
//!   bisector-plane ellipses cross at `p ± r·n̂`, where the walls are
//!   mutually tangent — classic pose (seams on the pinch) and the pose
//!   with both seams turned off it;
//! - Viviani: a radius-2 ball and a radius-1 cylinder through its
//!   centre, internally tangent at `(2, 0, 0)` — classic pose (the
//!   cylinder's seam on the tangency) and turned off it.
//!
//! Each row prints what the boolean does: builds (each edge through the
//! tangency point, its description, whether the point is a vertex, and
//! tier 3) or refuses (the error). Ignored: it pins nothing, it reports.
//!
//! Measured 2026-10-13, every op and both operand orders: no row builds.
//! Steinmetz refuses `CurvedPierceUnsupported` (seams on the pinch) or
//! `GermFrameCylinderPinch` (seams off it); Viviani refuses
//! `CurvedPierceUnsupported` (a seam or pole of the ball on the wall)
//! or `GermFrameUnsupported { Sphere, Cylinder }` — all before an edge
//! through the tangency point exists to be described.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use profile::{Profile, SketchPlane};
use sweep::test_support::{ball_poled_z, finished};
use sweep::{ExtrudeSide, Extrusion, extrude};
use topo::Body;

fn cyl(r: f64, z0: f64, z1: f64) -> Body<f64> {
    let tol = Tol::witness();
    let lp = profile::circle(Point2::new(0.0, 0.0), r, tol).unwrap();
    let plane = SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, z0)));
    let profile = Profile::new(plane, vec![lp.into()]).validate(tol).unwrap();
    extrude(
        &profile,
        Extrusion::Distance {
            depth: z1 - z0,
            side: ExtrudeSide::Along,
        },
        tol,
    )
    .unwrap()
    .body
}

fn rigid(b: &Body<f64>, m: &Affine3<f64>) -> Body<f64> {
    topo::transform_rigid(b, m, Tol::witness()).unwrap()
}

fn spin(b: &Body<f64>, axis: Vec3<f64>, angle: f64) -> Body<f64> {
    rigid(
        b,
        &Affine3::rotation_about_axis(Point3::new(0.0, 0.0, 0.0), axis, angle),
    )
}

/// Reports one boolean of one row: the outcome, and for a built body
/// every edge whose carrier passes within `1e-6` of a tangency point.
fn report(name: &str, op: &str, a: &Body<f64>, b: &Body<f64>, touch: &[Point3<f64>]) {
    let tol = Tol::witness();
    let (a, b) = (
        finished("operand A", a.clone(), tol),
        finished("operand B", b.clone(), tol),
    );
    let out = match op {
        "union" => topo::union(&a, &b, tol),
        "intersect" => topo::intersect(&a, &b, tol),
        _ => topo::subtract(&a, &b, tol),
    };
    let body = match out {
        Err(e) => {
            println!("PROBE {name} {op}: REFUSES {e:?}");
            return;
        }
        Ok(topo::BooleanResult::Body(bb)) => bb.body,
        Ok(other) => {
            println!("PROBE {name} {op}: BUILDS non-body {other:?}");
            return;
        }
    };
    let tier3 = topo::validate_geometric(&body, tol);
    println!("PROBE {name} {op}: BUILDS, tier 3 = {tier3:?}");
    for &q in touch {
        let at_vertex = body
            .vertices()
            .any(|(vk, _)| topo::readback::vertex_point(&body, vk).unwrap().distance(q) < 1e-6);
        println!("PROBE   tangency {q:?}: a vertex of the result: {at_vertex}");
    }
    for (ek, e) in body.edges() {
        let Some(topo::CurveGeom::Certified(c)) = body.get_curve_geom(e.curve) else {
            continue;
        };
        let (t0, t1) = c.params();
        for &q in touch {
            let n = 4000;
            let hit = (1..n).find(|i| {
                let t = t0 + (t1 - t0) * (*i as f64) / (n as f64);
                c.carrier().eval(t).distance(q) < 1e-6
            });
            if let Some(i) = hit {
                println!(
                    "PROBE   edge {ek:?} through {q:?} at s={:.4}: carrier {:?}, description {:?}",
                    i as f64 / n as f64,
                    c.carrier().kind(),
                    c.description()
                );
            }
        }
    }
}

#[test]
#[ignore = "probe: reports what booleans through an isolated tangency point do"]
fn booleans_through_an_isolated_tangency_point() {
    // Steinmetz: axes z and y (B is A turned about x), radius 1; the
    // ellipses cross at ±x̂.
    let steinmetz_touch = [Point3::new(1.0, 0.0, 0.0), Point3::new(-1.0, 0.0, 0.0)];
    let a = cyl(1.0, -2.0, 2.0);
    let b = spin(&a, Vec3::new(1.0, 0.0, 0.0), PI / 2.0);
    for op in ["union", "intersect", "subtract"] {
        report("steinmetz", op, &a, &b, &steinmetz_touch);
    }
    let (a2, b2) = (
        spin(&a, Vec3::new(0.0, 0.0, 1.0), PI / 4.0),
        spin(&b, Vec3::new(0.0, 1.0, 0.0), PI / 4.0),
    );
    for op in ["union", "intersect", "subtract"] {
        report("steinmetz-seams-off", op, &a2, &b2, &steinmetz_touch);
    }
    // Unequal radii crossing at a tangency? No: unequal crossing
    // cylinders never touch. Viviani: ball radius 2 at the origin, the
    // cylinder of radius 1 about x = 1, internally tangent at (2,0,0).
    let tol = Tol::witness();
    let ball = ball_poled_z(2.0, Vec3::new(0.0, 0.0, 0.0), tol);
    let viviani_touch = [Point3::new(2.0, 0.0, 0.0)];
    let c = rigid(
        &cyl(1.0, -3.0, 3.0),
        &Affine3::translation(Vec3::new(1.0, 0.0, 0.0)),
    );
    for op in ["union", "intersect", "subtract"] {
        report("viviani", op, &ball, &c, &viviani_touch);
    }
    let c_off = rigid(
        &spin(&cyl(1.0, -3.0, 3.0), Vec3::new(0.0, 0.0, 1.0), PI / 3.0),
        &Affine3::translation(Vec3::new(1.0, 0.0, 0.0)),
    );
    for op in ["union", "intersect", "subtract"] {
        report("viviani-seam-off", op, &ball, &c_off, &viviani_touch);
    }
    // The cylinder shorter than the ball, so its caps cut the ball and
    // the Viviani lobes end at cap rims: the section edge through the
    // tangency point has no reason to be split there.
    let c_short = rigid(
        &spin(&cyl(1.0, -1.0, 1.0), Vec3::new(0.0, 0.0, 1.0), PI / 3.0),
        &Affine3::translation(Vec3::new(1.0, 0.0, 0.0)),
    );
    for op in ["union", "intersect", "subtract"] {
        report(
            "viviani-short-seam-off",
            op,
            &ball,
            &c_short,
            &viviani_touch,
        );
    }
    // The ball's own seam meridian pierces the wall unless it sits on
    // the side the cylinder never reaches (azimuth near π): spin it.
    for k in 0..8 {
        let phi = f64::from(k) * PI / 4.0;
        let spun = spin(&ball, Vec3::new(0.0, 0.0, 1.0), phi);
        for op in ["union", "intersect", "subtract"] {
            report(
                &format!("viviani-short-ball{k}pi/4"),
                op,
                &spun,
                &c_short,
                &viviani_touch,
            );
            report(
                &format!("viviani-short-ball{k}pi/4-BA"),
                op,
                &c_short,
                &spun,
                &viviani_touch,
            );
        }
    }
}
