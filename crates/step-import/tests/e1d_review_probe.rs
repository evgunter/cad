//! E1d review probe (not for merge): a STEP file whose face misses the
//! curves bounding it, imported at the real door.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::at_rest;
use geom_core::{Point2, Tol, Vec2};
use profile::test_support::bulge_loop;
use profile::{Profile, SketchPlane};
use sweep::{Revolution, RevolveAxis};

fn hemisphere_step() -> String {
    let tol = Tol::witness();
    let quarter = bulge_loop(vec![
        (Point2::new(0.0, 0.0), 0.0),
        (Point2::new(1.0, 0.0), (core::f64::consts::PI / 8.0).tan()),
        (Point2::new(0.0, 1.0), 0.0),
    ]);
    let profile = Profile::new(SketchPlane::xy(), vec![quarter])
        .validate(tol)
        .unwrap();
    let axis = RevolveAxis {
        origin: Point2::new(0.0, 0.0),
        dir: Vec2::new(0.0, 1.0),
    };
    let body = sweep::revolve(&profile, axis, Revolution::Full, tol)
        .unwrap()
        .body;
    step_export::step_string(&at_rest(&body), &step_export::StepOptions::default(), tol).unwrap()
}

fn cone_step() -> String {
    let tol = Tol::witness();
    let tri = bulge_loop(vec![
        (Point2::new(0.0, 0.0), 0.0),
        (Point2::new(1.0, 0.0), 0.0),
        (Point2::new(0.0, 1.0), 0.0),
    ]);
    let profile = Profile::new(SketchPlane::xy(), vec![tri])
        .validate(tol)
        .unwrap();
    let axis = RevolveAxis {
        origin: Point2::new(0.0, 0.0),
        dir: Vec2::new(0.0, 1.0),
    };
    let body = sweep::revolve(&profile, axis, Revolution::Full, tol)
        .unwrap()
        .body;
    step_export::step_string(&at_rest(&body), &step_export::StepOptions::default(), tol).unwrap()
}

/// Rewrite the radius of every SPHERICAL_SURFACE.
fn bump_sphere(text: &str, r: f64) -> String {
    text.lines()
        .map(|l| {
            if l.contains("SPHERICAL_SURFACE") {
                let i = l.rfind(',').unwrap();
                format!("{},{r:.17e});", &l[..i])
            } else {
                l.to_owned()
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Rewrite the half-angle of every CONICAL_SURFACE.
fn bump_cone(text: &str, d: f64) -> String {
    text.lines()
        .map(|l| {
            if l.contains("CONICAL_SURFACE") {
                let i = l.rfind(',').unwrap();
                let v: f64 = l[i + 1..].trim_end_matches(");").trim().parse().unwrap();
                format!("{},{:.17e});", &l[..i], v + d)
            } else {
                l.to_owned()
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

#[test]
fn e1d_probe_a_face_off_its_curves_at_import() {
    let eps = Tol::witness().eps();
    let text = hemisphere_step();
    for l in text.lines().filter(|l| l.contains("SPHERICAL") || l.contains("UNCERTAINTY")) {
        eprintln!("PROBE line: {l}");
    }
    for (name, src) in [("sphere", text.clone()), ("cone", cone_step())] {
        for delta in [3.0 * eps, 30.0 * eps, 300.0 * eps, 1e-3] {
            for eps_in in [None, Some(1e-2)] {
                let bumped = if name == "sphere" {
                    bump_sphere(&src, 1.0 + delta)
                } else {
                    bump_cone(&src, delta)
                };
                let options = step_import::ImportOptions {
                    eps_in,
                    ..step_import::ImportOptions::default()
                };
                let got = step_import::import_step(&bumped, &options, Tol::witness());
                match got {
                    Ok(_) => eprintln!("PROBE {name} δ={delta:e} ε_in={eps_in:?}: imports"),
                    Err(e) => {
                        let t = e.to_string();
                        eprintln!("PROBE {name} δ={delta:e} ε_in={eps_in:?}: {e:?}\n   TEXT: {t}");
                        if t.contains("not lie on") || t.contains("not shown on the face") {
                            assert!(!t.contains("ε_in") || !t.contains("does not lie"), "{t}");
                        }
                    }
                }
            }
        }
    }
}
