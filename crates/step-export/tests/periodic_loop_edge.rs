//! **A closed section loop's edge is written as its clamped view.** An
//! edge on a periodic cubic is the loop's carrier and a span
//! (`crates/geom-brep/README.md` C11); the writer reads the edge as
//! every reader does, the clamped view over the span, and writes that
//! as one `B_SPLINE_CURVE_WITH_KNOTS` whose interior knots are simple.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use profile::{Profile, SketchPlane};
use step_export::{StepOptions, step_string};
use sweep::test_support::{finished, revolved_about_y};
use sweep::{ExtrudeSide, Extrusion, Revolution, extrude};

/// The lens of a cylinder of radius 0.5 and a ball of radius 0.3
/// centred on its wall: one cylinder face and one sphere face, bounded
/// by the one closed section loop.
#[test]
fn a_lens_between_a_cylinder_and_a_ball_writes_its_loop_as_one_clamped_spline() {
    let tol = Tol::witness();
    let lp = profile::circle(Point2::new(0.0, 0.0), 0.5, tol).unwrap();
    let plane = SketchPlane::xy_at(Point3::new(0.0, 0.0, -1.0));
    let disc = Profile::new(plane, vec![lp.into()]).validate(tol).unwrap();
    let side = ExtrudeSide::Along;
    let rod = extrude(&disc, Extrusion::Distance { depth: 2.0, side }, tol)
        .unwrap()
        .body
        .into_body();
    let ball = revolved_about_y(
        vec![(Point2::new(0.0, -0.3), 1.0), (Point2::new(0.0, 0.3), 0.0)],
        Revolution::Full,
        tol,
    );
    let ball = topo::transform_rigid(
        &ball,
        &geom_core::test_support::rigid(&Affine3::translation(Vec3::new(0.5, 0.0, 0.0)), tol),
    )
    .unwrap();
    let lens = topo::intersect(
        &finished("the rod", rod, tol),
        &finished("the ball", ball, tol),
        tol,
    )
    .expect("the lens builds");
    let lens = lens.body().expect("a body");
    let options = StepOptions {
        product_name: "lens".to_owned(),
        uncertainty_m: Some(1e-9),
        ..StepOptions::default()
    };
    let lens = finished("the lens", lens.body.clone(), tol);
    let text = step_string(&lens, &options, tol).expect("the lens exports");
    let splines: Vec<&str> = text
        .lines()
        .filter(|l| l.contains("B_SPLINE_CURVE_WITH_KNOTS("))
        .collect();
    let [spline] = splines.as_slice() else {
        panic!("one spline edge, the loop: {}", splines.len());
    };
    // `…, (multiplicities), (knots), .UNSPECIFIED.)`: the multiplicity
    // list is the second-to-last parenthesized group.
    let groups: Vec<&str> = spline.split('(').collect();
    let mults: Vec<usize> = groups[groups.len() - 2]
        .trim_end_matches([',', ' ', ')'])
        .split(',')
        .map(|m| m.trim().parse().expect("a multiplicity"))
        .collect();
    let (ends, interior) = (
        [mults[0], mults[mults.len() - 1]],
        &mults[1..mults.len() - 1],
    );
    assert_eq!(ends, [4, 4], "a clamped cubic");
    assert!(
        !interior.is_empty() && interior.iter().all(|&m| m == 1),
        "every interior knot of the written loop is simple: {interior:?}"
    );
}
