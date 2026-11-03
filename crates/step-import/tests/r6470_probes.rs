//! PR 6470 review probes (E2): an in-band ruling through public verbs.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Tol};
use profile::{Profile, SketchPlane, test_support::bulge_loop};
use sweep::{ExtrudeSide, Extrusion, extrude};

fn cylinder_text() -> String {
    cylinder_text_h(2.0)
}

fn cylinder_text_h(depth: f64) -> String {
    let lp = bulge_loop(vec![
        (Point2::new(-1.0, 0.0), 1.0),
        (Point2::new(1.0, 0.0), 1.0),
    ]);
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap();
    let body = extrude(
        &profile,
        Extrusion::Distance {
            depth,
            side: ExtrudeSide::Along,
        },
        Tol::witness(),
    )
    .unwrap()
    .body;
    let body = topo::test_support::finished("the cylinder", body, Tol::witness());
    let options = step_export::StepOptions {
        product_name: "cyl".to_owned(),
        ..step_export::StepOptions::default()
    };
    step_export::step_string(&crate::common::at_rest(&body), &options, Tol::witness()).unwrap()
}

#[test]
fn r6470_print_cylinder() {
    println!("{}", cylinder_text());
}

fn tilted(delta: f64) -> String {
    tilted_h(delta, 2.0)
}

fn tilted_h(delta: f64, h: f64) -> String {
    let (s, c) = delta.sin_cos();
    let d = geom_core::Vec3::new(c - 1.0, s, h).normalize();
    cylinder_text_h(h)
        .lines()
        .map(|l| {
            if l.starts_with("#6 = CARTESIAN_POINT") {
                format!("#6 = CARTESIAN_POINT('', ({c:.17e}, {s:.17e}, {h:.17e}));")
            } else if l.starts_with("#67 = DIRECTION") {
                format!("#67 = DIRECTION('', ({:.17e}, {:.17e}, {:.17e}));", d.x, d.y, d.z)
            } else {
                l.to_owned()
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn short<E: core::fmt::Debug>(e: &E) -> String {
    format!("{e:?}").chars().take(160).collect()
}

#[test]
fn r6470_tilted_seam_through_public_verbs() {
    use step_import::{ImportOptions, StepImport, import_step};
    use topo::splitting::{SplitPart, split};
    let tol = Tol::witness();
    let band = geom_core::Band::linear(tol).unwrap();
    println!("band zero {:e} escalate {:e}", band.zero(), band.escalate());
    for k in 0..=40 {
        let delta = 10f64.powf(-13.0 + 0.25 * f64::from(k));
        let text = tilted(delta);
        let imported = import_step(&text, &ImportOptions::default(), tol);
        let body = match imported {
            Ok(StepImport::Solid { body, .. }) => body,
            Ok(other) => {
                println!("δ {delta:.3e}: import not a solid: {}", short(&other).chars().take(60).collect::<String>());
                continue;
            }
            Err(e) => {
                println!("δ {delta:.3e}: import refused: {}", short(&e));
                continue;
            }
        };
        let tier3 = topo::validate_geometric(body.body(), tol);
        let mut line = format!("δ {delta:.3e}: import ok, tier3 {}", if tier3.is_ok() { "ok".into() } else { short(&tier3) });
        for (name, n) in [
            ("flat", geom_core::Vec3::new(0.0, 0.0, 1.0)),
            ("tilted", geom_core::Vec3::new(0.3, 0.1, 1.0).normalize()),
            ("vertical", geom_core::Vec3::new(0.0, 1.0, 0.0)),
        ] {
            let plane = topo::test_support::split_plane(
                geom_core::Point3::new(0.0, 0.0, 1.0),
                n,
                tol,
            );
            let r = split(&crate::common::at_rest(body.body()), &plane, tol);
            line.push_str(&match r {
                Ok(res) => format!(
                    " | split {name} ok ({})",
                    matches!(res.above, SplitPart::Body(_))
                ),
                Err(e) => format!(" | split {name} ERR {}", short(&e)),
            });
        }
        println!("{line}");
    }
}

#[test]
fn r6470_tilted_seam_full_errors() {
    use step_import::{ImportOptions, import_step};
    for delta in [3.162e-9, 1e-8, 1e-6] {
        match import_step(&tilted(delta), &ImportOptions::default(), Tol::witness()) {
            Ok(_) => println!("δ {delta:e}: ok"),
            Err(e) => println!("δ {delta:e}: DEBUG {e:?}\n  DISPLAY {e}\n"),
        }
    }
}

#[test]
fn r6470_short_tilted_seam_through_public_verbs() {
    use step_import::{ImportOptions, StepImport, import_step};
    use topo::splitting::split;
    let tol = Tol::witness();
    for h in [0.05, 0.2] {
        for k in 0..=24 {
            let delta = 10f64.powf(-12.0 + 0.25 * f64::from(k));
            let text = tilted_h(delta, h);
            let line = match import_step(&text, &ImportOptions::default(), tol) {
                Ok(StepImport::Solid { body, .. }) => {
                    let mut line = format!("h {h} δ {delta:.3e} (tilt/unit {:.2e}): import ok", delta / h);
                    for (name, o, n) in [
                        ("flat", geom_core::Point3::new(0.0, 0.0, h / 2.0), geom_core::Vec3::new(0.0, 0.0, 1.0)),
                        ("vertical", geom_core::Point3::new(0.0, 0.0, h / 2.0), geom_core::Vec3::new(0.0, 1.0, 0.0)),
                        ("oblique", geom_core::Point3::new(0.0, 0.0, h / 2.0), geom_core::Vec3::new(0.3, 0.2, 1.0).normalize()),
                    ] {
                        let plane = topo::test_support::split_plane(o, n, tol);
                        match split(&crate::common::at_rest(body.body()), &plane, tol) {
                            Ok(_) => line.push_str(&format!(" | split {name} ok")),
                            Err(e) => line.push_str(&format!(" | split {name} ERR {}", short(&e))),
                        }
                    }
                    line
                }
                Ok(_) => format!("h {h} δ {delta:.3e}: not a solid"),
                Err(e) => format!("h {h} δ {delta:.3e} (tilt/unit {:.2e}): import refused: {}", delta / h, short(&e)),
            };
            println!("{line}");
        }
    }
}
