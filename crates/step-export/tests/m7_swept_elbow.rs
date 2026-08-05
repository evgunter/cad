//! **#207's export leg: the first curved-path `sweep_body` to reach
//! the wire.**
//!
//! `sweep_body` with a curved path had zero successful callers anywhere
//! in the tree until the skin fit stopped synthesizing a weight channel
//! for integral input (see `sweep/tests/m7_skin_integral.rs` for the
//! kernel-side pins and `sweep::skin::skin`'s docs for the argument).
//! This suite carries the other half of that pin: the body exports, and
//! both independent Part 21 oracles reconstruct it.
//!
//! It also pins the CONSEQUENCE of the fix on the wire, which is the
//! whole point of it. `crate::writer` emits the plain
//! `B_SPLINE_SURFACE_WITH_KNOTS` record only when every weight is
//! exactly `1.0`, and the `RATIONAL_B_SPLINE_SURFACE` complex instance
//! otherwise. Before the fix a polyline-profile elbow would have gone
//! out as a rational surface — a lie about the geometry's kind that no
//! importer could have unpicked, on a body that in fact never got that
//! far because assembly refused first.
//!
//! The elbow is NOT in the committed fixture corpus: it is a
//! corpus-adjacent construction (same helpers, same oracles) so that
//! adding it costs no new golden bytes.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use core::f64::consts::{FRAC_PI_2, PI};

use geom_core::{Affine3, Point2, Point3, Vec3};
use step_export::{StepOptions, step_string};
use sweep::{SketchSegment, sweep_body};

/// Quarter-circle path radius, and the square cross-section's
/// half-width — see `sweep/tests/m7_skin_integral.rs`, which builds the
/// identical elbow and derives its Pappus volume.
const R: f64 = 3.0;
/// The square cross-section's half-width.
const H: f64 = 0.25;
/// Stations along the path, and the v-degree they are skinned at.
const STATIONS: usize = 9;
/// The skinning degree in the section direction.
const V_DEGREE: usize = 3;

/// A quarter-torus elbow of square cross-section: a square profile
/// swept along a 90° arc in the world YZ plane that starts at the
/// origin with tangent `+z`.
fn swept_elbow() -> topo::Body<f64> {
    let seg = |a: (f64, f64), b: (f64, f64)| SketchSegment::Line {
        a: Point2::new(a.0, a.1),
        b: Point2::new(b.0, b.1),
    };
    let p = [(-H, -H), (H, -H), (H, H), (-H, H)];
    let profile = vec![vec![
        seg(p[0], p[1]),
        seg(p[1], p[2]),
        seg(p[2], p[3]),
        seg(p[3], p[0]),
    ]];
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
    sweep_body::<f64>(&profile, Affine3::identity(), &path, STATIONS, V_DEGREE)
        .expect("the curved-path sweep body builds")
        .body
}

fn export(body: &topo::Body<f64>, name: &str) -> String {
    let options = StepOptions {
        product_name: name.to_owned(),
        uncertainty_m: Some(1e-9),
        ..StepOptions::default()
    };
    step_string(body, &options).expect("the elbow exports")
}

#[test]
fn the_swept_elbow_exports_and_both_oracles_reconstruct_it() {
    let body = swept_elbow();
    // Four walls, four wall–wall seams, two planar caps; the caps
    // contribute the four rim edges each shares with its walls.
    let census = common::census(&body);
    assert_eq!(census, (6, 12, 8), "kernel census (a topological box)");
    let text = export(&body, "swept_elbow");

    let exchange = ruststep::parser::parse(&text).expect("ruststep Part 21 parse");
    assert_eq!(exchange.data.len(), 1, "one data section");

    let table = truck_stepio::r#in::Table::from_step(&text).expect("truck table parse");
    let (_, shell) = table.shell.iter().next().expect("one shell");
    let cs = table
        .to_compressed_shell(shell)
        .expect("truck shell reconstruction");
    assert_eq!(
        (cs.faces.len(), cs.edges.len(), cs.vertices.len()),
        census,
        "the independent importer sees the kernel's census"
    );
}

/// **The wire says non-rational, and says it four times.** One plain
/// `B_SPLINE_SURFACE_WITH_KNOTS` per wall, and no
/// `RATIONAL_B_SPLINE_SURFACE` anywhere — the exported KIND is the
/// input's kind. This is the byte-level statement of #207's fix.
#[test]
fn the_swept_elbows_walls_go_out_non_rational() {
    let text = export(&swept_elbow(), "swept_elbow");
    assert_eq!(
        text.matches("B_SPLINE_SURFACE_WITH_KNOTS").count(),
        4,
        "one spline-surface record per wall"
    );
    assert_eq!(
        text.matches("RATIONAL_B_SPLINE_SURFACE").count(),
        0,
        "an integral sweep must not export a rational surface"
    );
    assert_eq!(
        text.matches("RATIONAL_B_SPLINE_CURVE").count(),
        0,
        "nor a rational seam carrier"
    );
}
