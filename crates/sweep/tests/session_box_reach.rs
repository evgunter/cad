//! **The session box reads a body's own points, never its carriers'
//! stored terms.** A cut of a large torus inside the box carries spiric
//! edges whose carrier, the whole torus, reaches past it; the box reads
//! each edge over its own span, so the cut is a body.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Point2, Tol, Vec3};
use sweep::Revolution;
use sweep::test_support::{brick, finished, revolved_about_y};

/// **A bar cutting a torus of `R = 700`, `r = 50` inside the box builds**
/// (the full review of PR 6380's probe). The torus about `y`, centred
/// at `(0, 300, 0)`, reaches about 828 m; the bar
/// `[100, 400] × [200, 400] × [400, 800]` reaches at most 980 m. Each
/// spiric edge the cut leaves on the torus rides a carrier whose stored
/// terms reach `|c| + R + r ≈ 1050` m, past the box: read so, the cut
/// was refused.
///
/// A pinned red at ε = 1e-12, `work/shell/the-torus-composite-bounds-a-thin-torus-far-above-its-carriers-rounding.md`:
/// there `intersect` refuses the cut first, limb 2's kernel limit on a
/// spiric section (a bound of 8.6e-11 m over 113 m of it, 22 times the
/// carrier's enclosure width), so this row shows the box's reading only
/// at the other ε.
#[test]
fn a_bar_cutting_a_large_torus_inside_the_box_builds() {
    let tol = Tol::witness();
    let (big_r, r) = (700.0, 50.0);
    let torus = revolved_about_y(
        vec![
            (Point2::new(big_r - r, 0.0), 1.0),
            (Point2::new(big_r + r, 0.0), 1.0),
        ],
        Revolution::Full,
        tol,
    );
    let moved = topo::transform_rigid(
        &torus,
        &geom_core::test_support::rigid(&Affine3::translation(Vec3::new(0.0, 300.0, 0.0)), tol),
        tol,
    )
    .unwrap();
    let torus = finished("the torus", moved, tol);
    let bar = finished(
        "the bar",
        brick::<f64>((100.0, 400.0), (200.0, 400.0), (400.0, 800.0), tol),
        tol,
    );
    let Some(cut) = test_utils::kernel_limit::pinned(
        topo::intersect(&torus, &bar, tol),
        tol.eps(),
        &[1e-12],
        "the in-box cut",
    ) else {
        return;
    };
    let body = cut.body().expect("the cut is a body");
    assert!(
        topo::validate_geometric(&body.body, tol).is_ok(),
        "the cut passes the finished-body gate"
    );
}

/// A cylinder of radius `r` about the origin, its circle four quarter
/// arcs (bulge `tan(π/8)`), extruded 1 m: the extrude's own result.
fn quarter_arc_cylinder(r: f64) -> Result<topo::Body<f64>, sweep::ExtrudeError> {
    let b = (core::f64::consts::PI / 8.0).tan();
    let lp = profile::test_support::bulge_loop(vec![
        (Point2::new(r, 0.0), b),
        (Point2::new(0.0, r), b),
        (Point2::new(-r, 0.0), b),
        (Point2::new(0.0, -r), b),
    ]);
    let pf = profile::Profile::new(profile::SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .expect("a valid circle");
    sweep::extrude(
        &pf,
        sweep::Extrusion::Distance {
            depth: 1.0,
            side: sweep::ExtrudeSide::Along,
        },
        Tol::witness(),
    )
    .map(|built| built.body.into_body())
}

/// **An in-box body of line and arc edges builds, however a box would
/// bound its edges.** A cylinder of radius 990 about the origin, its
/// circle four quarter arcs, reaches 990 m: each arc's own reach (its
/// cover's far distance, the crest its window holds) is exact, and a box
/// about a quarter arc would put a corner at `990·√2` ≈ 1.4 km, past the
/// box. A brick whose far corner is 990 m out reads its line edges at
/// their farther ends, exactly. Both pass the finished-body gate; the
/// cylinder at 1010 m does not. Red against a session box that read
/// either kind's edge reach above its exact value.
///
/// A pinned red at ε = 1e-12 for the 990 m cylinder,
/// `work/shell/eps-min-leaves-no-certification-headroom-at-the-box-edge.md`:
/// there limb 2 refuses its cylinder wall as the kernel limit before
/// the box reads it.
#[test]
fn line_and_arc_edges_in_the_box_read_their_exact_reach() {
    let tol = Tol::witness();
    let box_refusal = |e: &dyn std::fmt::Debug| format!("{e:?}").contains("OutsideSessionBox");
    if let Some(near) = test_utils::kernel_limit::pinned(
        quarter_arc_cylinder(990.0),
        tol.eps(),
        &[1e-12],
        "the 990 m cylinder",
    ) {
        assert!(
            topo::validate_geometric(&near, tol).is_ok(),
            "the 990 m cylinder of quarter arcs lies in the box"
        );
    }
    let corner = 990.0 / 2f64.sqrt() - 1e-6;
    let brick = finished(
        "the brick whose corner is 990 m out",
        brick::<f64>((0.0, corner), (0.0, corner), (0.0, 1e-3), tol),
        tol,
    );
    assert!(topo::validate_geometric(&brick, tol).is_ok());
    match quarter_arc_cylinder(1010.0) {
        Err(e) if box_refusal(&e) => {}
        Err(e) if tol.eps() <= 1e-12 && test_utils::kernel_limit::is_kernel_limit(&e) => {}
        other => panic!("the 1010 m cylinder is past the box: {other:?}"),
    }
}
