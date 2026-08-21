//! REVIEW PROBE (not part of the tree's acceptance set): rebuilds the
//! elbow exactly as the DELETED duplicate in `tests/m7_swept_elbow.rs`
//! did at `main`, exports it with the fixture-writing options, and
//! compares the bytes with the committed `swept_elbow.step`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{FRAC_PI_2, PI};
use profile::RawLoop;

use geom_core::{Affine3, Point2, Point3, Vec3};
use step_export::{StepOptions, step_string};
use sweep::{ProfileLoop, SketchSegment, sweep_body};
use geom_core::Tol;

const R: f64 = 3.0;
const H: f64 = 0.25;
const STATIONS: usize = 9;
const V_DEGREE: usize = 3;

fn duplicate_elbow(tol: Tol) -> topo::Body<f64> {
    // LIB-U3: the profile vocabulary — same square, loop form.
    let profile = vec![ProfileLoop::polygon(
        [(-H, -H), (H, -H), (H, H), (-H, H)].map(|(x, y)| Point2::new(x, y)),
    )];
    let path = sweep::skin::segment_curve(
        0,
        SketchSegment::Arc {
            a: Point2::new(0.0, 0.0),
            b: Point2::new(R, R),
            bulge: (PI / 8.0).tan(),
        },
        Affine3::rotation_about_axis(
            Point3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            -FRAC_PI_2,
        ),
    )
    .expect("the elbow path is a well-formed quarter arc");
    sweep_body::<f64>(&profile, Affine3::identity(), &path, STATIONS, V_DEGREE, tol)
        .expect("the curved-path sweep body builds")
        .body
}

fn main() {
    let tol = Tol::witness();
    let options = StepOptions {
        product_name: "swept_elbow".to_owned(),
        uncertainty_m: Some(1e-9),
        ..StepOptions::default()
    };
    let text = step_string(&duplicate_elbow(tol), &options, tol).expect("exports");
    let committed = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/swept_elbow.step"),
    )
    .expect("committed fixture");
    println!(
        "duplicate-construction export vs committed bytes: {}",
        if text == committed {
            "IDENTICAL".to_owned()
        } else {
            format!("DIFFER ({} vs {} bytes)", text.len(), committed.len())
        }
    );
}
