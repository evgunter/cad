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

/// The session-box refusal `errors` hold, alone: `Some(true)` past the
/// box, `Some(false)` refused as a limit, `None` with no box refusal.
fn box_verdict(errors: &[topo::ValidationError]) -> Option<bool> {
    match errors {
        [topo::ValidationError::OutsideSessionBox { .. }] => Some(true),
        [
            topo::ValidationError::SessionBoxUnshown { .. }
            | topo::ValidationError::SessionBoxUncovered { .. },
        ] => Some(false),
        _ => None,
    }
}

/// The margin of a lone past-the-box refusal, metres.
fn past_margin(errors: &[topo::ValidationError]) -> f64 {
    let [topo::ValidationError::OutsideSessionBox { margin }] = errors else {
        panic!("the past-the-box refusal, alone: {errors:?}");
    };
    let said = margin.readable_below_zero().to_string();
    said.parse()
        .unwrap_or_else(|_| panic!("a margin at f64 reads one number: {said}"))
}

/// **A ball past the box is refused as past it however its poles lie**
/// (`work/oneround/the-session-box-reach-misses-a-curved-faces-bulge.md`,
/// the measured example). A ball of `r = 50` centred at `(0, 0, 975)`
/// has its top at `z = 1025`. Poled on `z` that top is a vertex; poled on
/// `y` it is interior to a face, on no vertex or edge, and only the face's
/// inside read (its sphere's `+z` pole, placed on the face) reaches it.
/// Both refuse past the box by 25 m, at the gate and at the door that
/// places the ball there from the origin. The same ball at
/// `(0, 0, 940)`, its top at 990 m, is in the box both ways. *Breaks if*
/// the face read is dropped (poled on `y` is admitted), the inside read
/// is dropped (poled on `y` refuses as a limit) or the cover is read as
/// a point (the ball at 940 m, its ball cover within, still passes, but
/// the loft below refuses past the box).
#[test]
fn a_ball_past_the_box_is_refused_whichever_way_it_is_poled() {
    let tol = Tol::witness();
    for (what, poled) in [
        (
            "poled on y",
            sweep::test_support::ball_poled_y::<f64> as fn(_, _, _) -> _,
        ),
        ("poled on z", sweep::test_support::ball_poled_z),
    ] {
        let far = poled(50.0, Vec3::new(0.0, 0.0, 975.0), tol);
        let errors = topo::validate_geometric(&far, tol).unwrap_err();
        let margin = past_margin(&errors);
        assert!(
            (margin - 25.0).abs() < 1e-6,
            "{what}: 25 m past the box, read {margin}"
        );
        let home = poled(50.0, Vec3::new(0.0, 0.0, 0.0), tol);
        let at = geom_core::RigidMotion::translation(Vec3::new(0.0, 0.0, 975.0)).unwrap();
        match topo::session_box_at(&home, &at, tol) {
            Err(topo::MoveError::NotFinished(found)) => {
                let margin = past_margin(found.as_slice());
                assert!((margin - 25.0).abs() < 1e-6, "{what}: placed, {margin}");
            }
            other => panic!("{what}: placed past the box, got {other:?}"),
        }
        let near = poled(50.0, Vec3::new(0.0, 0.0, 940.0), tol);
        assert!(
            topo::validate_geometric(&near, tol).is_ok(),
            "{what}: the ball at 940 m is in the box"
        );
        let in_box = geom_core::RigidMotion::translation(Vec3::new(0.0, 0.0, 940.0)).unwrap();
        assert!(
            topo::session_box_at(&home, &in_box, tol).is_ok(),
            "{what}: placed at 940 m"
        );
    }
}

/// **A torus whose bulge no edge rides is refused past the box**
/// (`work/oneround/the-session-box-reach-misses-a-curved-faces-bulge.md`,
/// the first example). The torus `R = 450`, `r = 400` about `y`, its
/// meridian circle's seams at the top and bottom (`(R, ±r)`), turned a
/// half turn about `y` so its meridian edge faces away from `+x`, and
/// centred at `(250, 0, 0)`: its edges reach 700 m and its outer equator
/// 1100 m, on no edge. Refused past the box by 100 m, at the gate and at
/// the placing door. *Breaks if* the face read or the inside read is
/// dropped (admitted, or a limit).
#[test]
fn a_torus_whose_bulge_no_edge_rides_is_refused_past_the_box() {
    let tol = Tol::witness();
    let (big_r, r) = (450.0, 400.0);
    let torus = revolved_about_y(
        vec![(Point2::new(big_r, -r), 1.0), (Point2::new(big_r, r), 1.0)],
        Revolution::Full,
        tol,
    );
    let turn = geom_core::RigidMotion::rotation(
        geom_core::UnitVec3::new(
            Vec3::new(0.0, 1.0, 0.0),
            "y",
            geom_core::Band::linear(tol).unwrap(),
        )
        .unwrap(),
        core::f64::consts::PI,
    )
    .unwrap();
    let at = geom_core::RigidMotion::translation(Vec3::new(250.0, 0.0, 0.0))
        .unwrap()
        .compose(turn)
        .unwrap();
    let placed = topo::transform_rigid(&torus, &at).unwrap();
    let errors = topo::validate_geometric(&placed, tol).unwrap_err();
    let margin = past_margin(&errors);
    assert!((margin - 100.0).abs() < 1e-6, "100 m past, read {margin}");
    match topo::session_box_at(&torus, &at, tol) {
        Err(topo::MoveError::NotFinished(found)) => {
            let margin = past_margin(found.as_slice());
            assert!((margin - 100.0).abs() < 1e-6, "placed, {margin}");
        }
        other => panic!("placed past the box, got {other:?}"),
    }
}

/// **A body whose cover alone reaches past the box is refused as a
/// limit, not as past it.** The loft prism's `+x` wall is a spline face
/// that flares to `x = 1.375` while its control net, its cover, reaches
/// `x = 1.75`. Moved `998.5` m along `x`, the wall stands at `999.875` m,
/// inside the box, and its cover at `1000.25` m, past it: refused as the
/// limit `SessionBoxUnshown`, at the gate and at the placing door. Moved
/// `998` m, its cover is within and it is admitted. *Breaks if* the limit
/// is refused as past the box, or the cover is read as a point (admitted).
#[test]
fn a_body_whose_cover_alone_is_past_the_box_is_refused_as_a_limit() {
    let tol = Tol::witness();
    let prism = sweep::test_support::loft_prism(tol);
    let at = geom_core::RigidMotion::translation(Vec3::new(998.5, 0.0, 0.0)).unwrap();
    let placed = topo::transform_rigid(&prism, &at).unwrap();
    let errors = topo::validate_geometric(&placed, tol).unwrap_err();
    assert_eq!(box_verdict(&errors), Some(false), "a limit: {errors:?}");
    let [topo::ValidationError::SessionBoxUnshown { margin }] = &errors[..] else {
        panic!("the cover's limit: {errors:?}");
    };
    let said = errors[0].to_string();
    assert!(
        said.contains("not built yet") && said.contains("nearer the origin"),
        "{said}"
    );
    assert!(
        (margin
            .readable_below_zero()
            .to_string()
            .parse::<f64>()
            .unwrap()
            - 0.25)
            .abs()
            < 1e-3,
        "its net reaches 0.25 m past: {said}"
    );
    match topo::session_box_at(&prism, &at, tol) {
        Err(topo::MoveError::NotFinished(found)) => {
            assert_eq!(
                box_verdict(found.as_slice()),
                Some(false),
                "placed: {found:?}"
            );
        }
        other => panic!("placed as a limit, got {other:?}"),
    }
    let within = geom_core::RigidMotion::translation(Vec3::new(998.0, 0.0, 0.0)).unwrap();
    let near = topo::transform_rigid(&prism, &within).unwrap();
    assert!(
        topo::validate_geometric(&near, tol).is_ok(),
        "its net within the box"
    );
    assert!(topo::session_box_at(&prism, &within, tol).is_ok());
}
