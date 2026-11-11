//! **The session box reads a body's own points, never its carriers'
//! stored terms**, and it bounds each coordinate (`[tolerance.units]`),
//! so it is a cube: geometry out on its diagonals, farther than 1 km from
//! the origin with every coordinate within it, is in the box.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Point2, Tol, Vec3};
use sweep::Revolution;
use sweep::test_support::{brick, finished, revolved_about_y};

/// **A bar cutting a torus of `R = 700`, `r = 50` inside the box builds**
/// (the full review of PR 6380's probe). The torus about `y`, centred
/// at `(0, 300, 0)`, has every coordinate within 750 m; the bar
/// `[100, 400] × [200, 400] × [400, 800]` within 800 m. Each spiric edge
/// the cut leaves on the torus rides the whole torus as its carrier.
/// Read as coordinates, that carrier's box is the torus operand's own,
/// inside the box, so on the cube this row separates the box's own-points
/// reading only from one that reads the stored terms as a distance from
/// the origin, `|c| + R + r ≈ 1050` m, which refused the cut. No
/// operand in the box can make a carrier's coordinate box leave it.
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

/// A cylinder of radius `r` about `(cx, cy)`, its circle four quarter
/// arcs (bulge `tan(π/8)`), extruded 1 m: the extrude's own result.
fn quarter_arc_cylinder(
    (cx, cy): (f64, f64),
    r: f64,
) -> Result<topo::Body<f64>, sweep::ExtrudeError> {
    let b = (core::f64::consts::PI / 8.0).tan();
    let lp = profile::test_support::bulge_loop(vec![
        (Point2::new(cx + r, cy), b),
        (Point2::new(cx, cy + r), b),
        (Point2::new(cx - r, cy), b),
        (Point2::new(cx, cy - r), b),
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

/// **An in-box body of line and arc edges builds out on the box's
/// diagonal.** The session box bounds each coordinate, so it is a cube:
/// a cylinder of radius 400 about `(590, 590)`, its circle four quarter
/// arcs, has every coordinate within 990 m and points 1.23 km from the
/// origin; a brick `[0, 990]² × [0, 1 mm]` has every coordinate within
/// 990 m and its far corner 1.4 km out. Each arc's coordinate range is
/// its cover's, exact, and each line's is at its ends. Both pass the
/// finished-body gate; a cylinder of radius 1010 about the origin does
/// not. Red against a session box that reads a distance from the origin
/// (a ball), at the vertices or on either edge kind, and against an arc
/// or line arm read 2% looser than exact (990 m reads 1009.8).
///
/// A pinned red at ε = 1e-12 for the in-box cylinder,
/// `work/shell/eps-min-leaves-no-certification-headroom-at-the-box-edge.md`:
/// there limb 2 refuses its cylinder wall as the kernel limit before
/// the box reads it.
#[test]
fn line_and_arc_edges_in_the_box_read_their_exact_reach() {
    let tol = Tol::witness();
    let box_refusal = |e: &dyn std::fmt::Debug| format!("{e:?}").contains("OutsideSessionBox");
    if let Some(near) = test_utils::kernel_limit::pinned(
        quarter_arc_cylinder((590.0, 590.0), 400.0),
        tol.eps(),
        &[1e-12],
        "the cylinder out on the diagonal",
    ) {
        assert!(
            topo::validate_geometric(&near, tol).is_ok(),
            "the cylinder of quarter arcs about (590, 590) lies in the box"
        );
    }
    let brick = finished(
        "the brick whose corner is (990, 990)",
        brick::<f64>((0.0, 990.0), (0.0, 990.0), (0.0, 1e-3), tol),
        tol,
    );
    assert!(topo::validate_geometric(&brick, tol).is_ok());
    match quarter_arc_cylinder((0.0, 0.0), 1010.0) {
        Err(e) if box_refusal(&e) => {}
        Err(e) if tol.eps() <= 1e-12 && test_utils::kernel_limit::is_kernel_limit(&e) => {}
        other => panic!("the 1010 m cylinder is past the box: {other:?}"),
    }
}
