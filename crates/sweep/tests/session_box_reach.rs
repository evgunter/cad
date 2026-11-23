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
    let Some(body) = bar_cut_torus(tol) else {
        return;
    };
    assert!(
        topo::validate_geometric(&body, tol).is_ok(),
        "the cut passes the finished-body gate"
    );
}

/// The cut [`a_bar_cutting_a_large_torus_inside_the_box_builds`] makes,
/// unfinished, or `None` at the ε its pinned red stands at.
fn bar_cut_torus(tol: Tol) -> Option<topo::Body<f64>> {
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
    let cut = test_utils::kernel_limit::pinned(
        topo::intersect(&torus, &bar, tol),
        tol.eps(),
        &[1e-12],
        "the in-box cut",
    )?;
    Some(
        cut.body()
            .expect("the cut is a body")
            .body
            .clone()
            .into_body(),
    )
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

/// The box's verdict on `errors`, by class and margin: `("Outside", m)`,
/// `("Unshown", m)`, `("Uncovered", NaN)`, or `("Other", NaN)`.
fn box_class(errors: &[topo::ValidationError]) -> (&'static str, f64) {
    let read = |m: &geom_core::MarginDiag| {
        m.readable_below_zero()
            .to_string()
            .parse::<f64>()
            .unwrap_or(f64::NAN)
    };
    match errors {
        [topo::ValidationError::OutsideSessionBox { margin }] => ("Outside", read(margin)),
        [topo::ValidationError::SessionBoxUnshown { margin }] => ("Unshown", read(margin)),
        [topo::ValidationError::SessionBoxUncovered { .. }] => ("Uncovered", f64::NAN),
        _ => ("Other", f64::NAN),
    }
}

/// The gate's box verdict on `body`: `("Ok", 0)` where it is finished.
fn gate_class(body: &topo::Body<f64>, tol: Tol) -> (&'static str, f64) {
    match topo::validate_geometric(body, tol) {
        Ok(_) => ("Ok", 0.0),
        Err(errors) => box_class(&errors),
    }
}

/// The placing door's box verdict on `home` placed by `at`.
fn door_class(
    home: &topo::Body<f64>,
    at: &geom_core::RigidMotion<f64>,
    tol: Tol,
) -> (&'static str, f64) {
    match topo::session_box_at(home, at, tol) {
        Ok(()) => ("Ok", 0.0),
        Err(topo::MoveError::NotFinished(found)) => box_class(found.as_slice()),
        Err(other) => panic!("the door maps the body: {other:?}"),
    }
}

/// A unit direction.
fn unit(v: Vec3<f64>, tol: Tol) -> geom_core::UnitVec3<f64> {
    geom_core::UnitVec3::new(v, "a row's axis", geom_core::Band::linear(tol).unwrap()).unwrap()
}

/// **A torus tilted however little off a world axis is refused past the
/// box by its face's extreme** (reviewer r1's probe). A full torus,
/// `R = 300`, `r = 100`, its seams at the inner and outer equators, its
/// axis turned `θ` off `+y` about `x`, stood so its true largest `y`,
/// `r + R·sin θ`, is past the box by half of `R·sin θ` while every vertex
/// and edge (the seam's top at `r·cos θ` and below) is inside it. Only the
/// face's extreme reaches past, and it is read off the torus's direction
/// off its axis, which cancels to `1 − cos²θ` if spelled `e − a(a·e)`.
/// Refused past the box by that half, to 1e-6 of it, for `θ` from 0.3
/// down to 1e-7; exactly axial, the top circle is the extreme and its
/// height is the seam's, so the edges read it. *Breaks if* the direction
/// off the axis loses its digits near the axis (then `θ ≤ 1e-6` refuses
/// as a limit), or the inside read is dropped.
#[test]
fn a_torus_tilted_off_an_axis_is_refused_past_the_box_by_its_extreme() {
    let tol = Tol::witness();
    let (big, r) = (300.0_f64, 100.0_f64);
    let home = revolved_about_y(
        vec![
            (Point2::new(big - r, 0.0), 1.0),
            (Point2::new(big + r, 0.0), 1.0),
        ],
        Revolution::Full,
        tol,
    );
    for theta in [0.3, 1e-1, 1e-2, 1e-3, 1e-4, 1e-5, 1e-6, 1e-7, 0.0] {
        let turn =
            geom_core::RigidMotion::rotation(unit(Vec3::new(1.0, 0.0, 0.0), tol), theta).unwrap();
        let top = r + big * theta.sin();
        let past = if theta == 0.0 {
            5.0
        } else {
            0.5 * big * theta.sin()
        };
        let at = geom_core::RigidMotion::translation(Vec3::new(0.0, 1000.0 + past - top, 0.0))
            .unwrap()
            .compose(turn)
            .unwrap();
        let image = topo::transform_rigid(&home, &at).unwrap();
        for (door, (class, margin)) in [
            ("gate", gate_class(&image, tol)),
            ("placing door", door_class(&home, &at, tol)),
        ] {
            assert_eq!(class, "Outside", "θ = {theta:e}, {door}: past by {past:e}");
            assert!(
                (margin - past).abs() <= 1e-6 * past,
                "θ = {theta:e}, {door}: past by {past:e}, read {margin:e}"
            );
        }
    }
}

/// **A sphere zone whose carrier's pole alone is past the box is
/// admitted** (reviewer r1). The barrel, a sphere zone `|y| ≤ 30` of a
/// radius-50 sphere capped by discs, centred at `y = 965`, reaches
/// `y = 995` (its top disc); its ball reaches 1015 m, and its `+y` pole is
/// not on the face. So stage 1 (the ball) does not admit it, stage 2 (the
/// zone's own cover) does, and the inside read drops the pole. At
/// `y = 975` its disc is past by 5 m and it is refused as past the box.
/// *Breaks if* stage 2 never admits, or a candidate the face does not hold
/// is kept.
#[test]
fn a_zone_whose_pole_alone_is_past_the_box_is_admitted() {
    let tol = Tol::witness();
    let home = revolved_about_y(
        vec![
            (Point2::new(0.0, -30.0), 0.0),
            (Point2::new(40.0, -30.0), 1.0 / 3.0),
            (Point2::new(40.0, 30.0), 0.0),
            (Point2::new(0.0, 30.0), 0.0),
        ],
        Revolution::Full,
        tol,
    );
    let at = geom_core::RigidMotion::translation(Vec3::new(0.0, 965.0, 0.0)).unwrap();
    let image = topo::transform_rigid(&home, &at).unwrap();
    assert_eq!(gate_class(&image, tol).0, "Ok", "the barrel at 965 m");
    assert_eq!(door_class(&home, &at, tol).0, "Ok", "placed at 965 m");
    let far = geom_core::RigidMotion::translation(Vec3::new(0.0, 975.0, 0.0)).unwrap();
    let image = topo::transform_rigid(&home, &far).unwrap();
    for (door, (class, margin)) in [
        ("gate", gate_class(&image, tol)),
        ("placing door", door_class(&home, &far, tol)),
    ] {
        assert!(
            class == "Outside" && (margin - 5.0).abs() < 1e-6,
            "{door}: the barrel at 975 m is 5 m past, read {class} {margin}"
        );
    }
}

/// **An arc edge whose interior alone is past the box refuses past it**
/// (reviewer r1). A cylinder of radius 100 about `(cx, 0)`, its circle
/// four quarter arcs whose ends sit on the diagonals, extruded 1 m: at
/// `cx = 905` its `+x` arc reaches 1005 m at its middle, on no vertex (the
/// vertices reach 975.7 m) and on no face extreme (a cylinder has none).
/// Refused past the box by 5 m; at `cx = 895` it is admitted. *Breaks if*
/// stage 3 drops the edges' own points.
#[test]
fn an_arc_whose_middle_alone_is_past_the_box_refuses_past_it() {
    let tol = Tol::witness();
    let b = (core::f64::consts::PI / 8.0).tan();
    let at = |cx: f64| {
        let d = 100.0 / 2.0_f64.sqrt();
        let lp = profile::test_support::bulge_loop(vec![
            (Point2::new(cx + d, -d), b),
            (Point2::new(cx + d, d), b),
            (Point2::new(cx - d, d), b),
            (Point2::new(cx - d, -d), b),
        ]);
        let pf = profile::Profile::new(profile::SketchPlane::xy(), vec![lp])
            .validate(tol)
            .expect("a valid circle");
        sweep::extrude(
            &pf,
            sweep::Extrusion::Distance {
                depth: 1.0,
                side: sweep::ExtrudeSide::Along,
            },
            tol,
        )
    };
    match at(905.0) {
        Err(e) => {
            let said = format!("{e:?}");
            assert!(
                said.contains("OutsideSessionBox { margin: MarginDiag(Value(-5"),
                "{said}"
            );
        }
        Ok(_) => panic!("the arc's middle is 5 m past the box"),
    }
    assert!(at(895.0).is_ok(), "inside the box");
}

/// **A sphere face through both poles reads its whole ball, so a wedge in
/// the box whose ball pokes out of it refuses as a limit, never as past
/// it** (reviewer r2). A ball of radius 30 revolved 1.1 rad, turned a
/// half turn about `y` to face `−x`, its axis at `x = 990`: every point of
/// it is within 990 m, and its sphere's ball reaches 1020 m. The face
/// runs pole to pole and reads no window, so its cover is the ball; the
/// ball's `+x` pole, at 1020 m, is a candidate the face does not hold.
/// Refused as the limit 20 m past, at the gate and at the placing door.
/// (The radius is 30 so the wedge certifies at every ε the suite runs.)
/// *Breaks if* the inside read keeps a candidate the face does not hold
/// (then the wedge, which is in the box, refuses as past it by 20 m).
#[test]
fn a_ball_wedge_in_the_box_whose_ball_pokes_out_refuses_as_a_limit() {
    let tol = Tol::witness();
    let wedge = sweep::test_support::try_revolved_about_y_at::<f64>(
        vec![
            (Point2::new(0.0, -30.0), 1.0),
            (Point2::new(0.0, 30.0), 0.0),
        ],
        Revolution::Partial(1.1),
        tol,
    )
    .expect("the wedge revolves");
    let turn = geom_core::RigidMotion::rotation(
        unit(Vec3::new(0.0, 1.0, 0.0), tol),
        core::f64::consts::PI,
    )
    .unwrap();
    let at = geom_core::RigidMotion::translation(Vec3::new(990.0, 0.0, 0.0))
        .unwrap()
        .compose(turn)
        .unwrap();
    let image = topo::transform_rigid(&wedge, &at).unwrap();
    for (door, (class, margin)) in [
        ("gate", gate_class(&image, tol)),
        ("placing door", door_class(&wedge, &at, tol)),
    ] {
        assert!(
            class == "Unshown" && (margin - 20.0).abs() < 1e-6,
            "{door}: the wedge's ball reaches 20 m past, read {class} {margin}"
        );
    }
}

/// **A body that reaches the box's edge exactly is in it**: the box
/// decides on a margin, and only a definitely negative one refuses. A
/// brick `[999, 1000] × [0, 1] × [0, 1]` reaches 1000 m, a margin of
/// zero, and is in the box at the gate and at the placing door; the
/// same brick `1000·ε` farther out, a margin past the band, is not.
/// *Breaks if* a margin within the band is read as past the box.
#[test]
fn a_body_at_the_boxs_edge_is_in_it() {
    let tol = Tol::witness();
    let home = brick::<f64>((-0.5, 0.5), (0.0, 1.0), (0.0, 1.0), tol);
    for (x, inside) in [(999.5, true), (999.5 + 1e3 * tol.eps(), false)] {
        let at = geom_core::RigidMotion::translation(Vec3::new(x, 0.0, 0.0)).unwrap();
        let image = topo::transform_rigid(&home, &at).unwrap();
        for (door, (class, _)) in [
            ("gate", gate_class(&image, tol)),
            ("placing door", door_class(&home, &at, tol)),
        ] {
            assert_eq!(
                class == "Ok",
                inside,
                "{door}: the brick reaching {} m read {class}",
                x + 0.5
            );
        }
    }
}

/// **A ring-torus face a cut leaves reads its whole tube, so the piece,
/// in the box, refuses as a limit by the tube** (reviewers r1 and r2 of
/// PR 7225's fix pass). The bar's cut of the `R = 700`, `r = 50` torus
/// ([`bar_cut_torus`]) is bounded by spiric edges, whose chart images
/// are not harmonic, so its torus faces read no chart window and cover
/// the torus whole (`topo::cover`'s torus arm). Moved 300 m along `x` the
/// piece reaches 743.3 m and the tube 1050 m: refused as the limit 50 m
/// past, at the gate and at the placing door; moved 200 m, the tube
/// reaches 950 m and it is admitted. The cut's plane faces, bounded by
/// the same spiric edges, read within the bar: a spiric edge's cover is
/// its arc's rectangle in its section plane met with the tube, and only
/// the torus face reads the tube whole. *Breaks if* a torus face comes
/// to read a window from such edges (then the row's 300 m case admits,
/// and the row is the measure of it), or a plane face bounded by them
/// reads the tube.
#[test]
fn a_ring_torus_face_a_cut_leaves_reads_its_whole_tube() {
    let tol = Tol::witness();
    let Some(home) = bar_cut_torus(tol) else {
        return;
    };
    let band = geom_core::Band::linear(tol).unwrap();
    for (dx, want) in [(300.0, ("Unshown", 50.0)), (200.0, ("Ok", 0.0))] {
        let at = geom_core::RigidMotion::translation(Vec3::new(dx, 0.0, 0.0)).unwrap();
        let image = topo::transform_rigid(&home, &at).unwrap();
        for (door, (class, margin)) in [
            ("gate", gate_class(&image, tol)),
            ("placing door", door_class(&home, &at, tol)),
        ] {
            assert!(
                class == want.0 && (margin - want.1).abs() < 1e-6,
                "{door}, moved {dx} m: read {class} {margin}, want {want:?}"
            );
        }
    }
    let x = Vec3::new(1.0, 0.0, 0.0);
    for (face, _) in home.faces() {
        let kind = topo::readback::face_carrier_kind(&home, face).unwrap();
        let (lo, hi) = topo::face_cover(&home, face, band).unwrap().range_along(x);
        match kind {
            geom::SurfaceKind::Torus => assert!(
                (lo + 750.0).abs() < 1e-6 && (hi - 750.0).abs() < 1e-6,
                "the torus face reads the whole tube: [{lo}, {hi}]"
            ),
            _ => assert!(
                lo >= 100.0 - 1e-6 && hi <= 400.0 + 1e-6,
                "a {kind:?} face of the cut reads within the bar: [{lo}, {hi}]"
            ),
        }
    }
}

/// **A spindle face reads its whole tube, so a lemon in the box refuses
/// as a limit by it.** A lemon revolved from the arc through `(0, ±30)`
/// of bulge 0.5 (radius 37.5, its centre 22.5 m behind the axis, so the
/// sheet's `R = −22.5`) reaches 15 m off its axis; its tube reaches
/// `|R| + r = 60` m. Moved 980 m along `x` the lemon reaches 995 m and
/// the tube 1040 m: refused as the limit 40 m past, at the gate and at
/// the placing door, as it is admitted at the origin. *Breaks if* a
/// spindle face comes to read less than its tube (then the row is the
/// measure of it), or the limit is refused as past.
#[test]
fn a_lemon_in_the_box_reads_its_whole_tube() {
    let tol = Tol::witness();
    let lemon = sweep::test_support::try_revolved_about_y_at::<f64>(
        vec![
            (Point2::new(0.0, -30.0), 0.5),
            (Point2::new(0.0, 30.0), 0.0),
        ],
        Revolution::Full,
        tol,
    )
    .expect("the lemon revolves");
    for (dx, want) in [(0.0, ("Ok", 0.0)), (980.0, ("Unshown", 40.0))] {
        let at = geom_core::RigidMotion::translation(Vec3::new(dx, 0.0, 0.0)).unwrap();
        let image = topo::transform_rigid(&lemon, &at).unwrap();
        for (door, (class, margin)) in [
            ("gate", gate_class(&image, tol)),
            ("placing door", door_class(&lemon, &at, tol)),
        ] {
            assert!(
                class == want.0 && (margin - want.1).abs() < 1e-6,
                "{door}, moved {dx} m: read {class} {margin}, want {want:?}"
            );
        }
    }
}

/// **A slender cone face reads no apex where its lift is decided, cut
/// or not**: a ring revolved from `(0.09, 0)`, `(0.1, 0)`, `(0.1001,
/// 0.1)`, `(0.09, 0.1)`, whose outer wall tapers 0.1 mm over 0.1 m (its
/// apex 100 m off), and the same ring cut by a slab tilted 1.2 rad about
/// `x`, which leaves the wall bounded by plane sections. Each is turned
/// so its axis runs along `x`, its apex toward `+x`, and moved to
/// `x = 980`: it lies within `[979.9, 980]` along `x`, its apex at 1080 m. Each is admitted, at
/// the gate and at the placing door: the cone face's cover is its
/// boundary's hull, its apex out (`topo::cover`'s cone arm). *Breaks if*
/// the cone arm keeps a decided face's apex (then both refuse as the
/// limit 80 m past).
#[test]
fn a_slender_cone_face_reads_no_apex_cut_or_not() {
    let tol = Tol::witness();
    let ring = sweep::test_support::try_revolved_about_y_at::<f64>(
        vec![
            (Point2::new(0.09, 0.0), 0.0),
            (Point2::new(0.1, 0.0), 0.0),
            (Point2::new(0.1001, 0.1), 0.0),
            (Point2::new(0.09, 0.1), 0.0),
        ],
        Revolution::Full,
        tol,
    )
    .expect("the ring revolves");
    let slab = brick::<f64>((-0.2, 0.2), (-0.2, 0.2), (-0.01, 0.01), tol);
    let tilt = geom_core::RigidMotion::translation(Vec3::new(0.0, 0.05, 0.0))
        .unwrap()
        .compose(
            geom_core::RigidMotion::rotation(unit(Vec3::new(1.0, 0.0, 0.0), tol), 1.2).unwrap(),
        )
        .unwrap();
    let slab = finished(
        "the slab",
        topo::transform_rigid(&slab, &tilt).unwrap(),
        tol,
    );
    let cut = topo::intersect(&finished("the ring", ring.clone(), tol), &slab, tol)
        .expect("the slab cuts the ring");
    let cut = cut
        .body()
        .expect("the cut is a body")
        .body
        .clone()
        .into_body();
    // The apex lies at `(0, −100, 0)`; a quarter turn about `z` takes it
    // to `(100, 0, 0)`, then the move to `x = 980`.
    let at = geom_core::RigidMotion::translation(Vec3::new(980.0, 0.0, 0.0))
        .unwrap()
        .compose(
            geom_core::RigidMotion::rotation(
                unit(Vec3::new(0.0, 0.0, 1.0), tol),
                core::f64::consts::FRAC_PI_2,
            )
            .unwrap(),
        )
        .unwrap();
    for (what, home) in [("the ring", &ring), ("the cut ring", &cut)] {
        let image = topo::transform_rigid(home, &at).unwrap();
        for (door, (class, _)) in [
            ("gate", gate_class(&image, tol)),
            ("placing door", door_class(home, &at, tol)),
        ] {
            assert_eq!(class, "Ok", "{what}, {door}: in the box");
        }
    }
}

/// **An approximated surface's face reads its fit's net**: the 2 × 2 × 1
/// block whose top cap wears the certified `Approx` of its plane
/// (`common::approx::box_with_approx_cap`, the only `Approx` face this
/// tree builds; no door mints one) covers the cap by the fit's net,
/// which here spans the cap exactly. So the body stands at the box's
/// wall: moved to reach 999.5 m along `x` it is admitted, at the gate
/// and at the placing door, and the cap's cover along `x` is the net's
/// `[0, 2]`. *Breaks if* the cap comes to read anything but its fit's
/// net (a whole-carrier refusal reads as uncovered here).
#[test]
fn an_approximated_cap_reads_its_fits_net() {
    let tol = Tol::witness();
    let (home, cap) = crate::common::approx::box_with_approx_cap(0.1, 1e-9);
    let band = geom_core::Band::linear(tol).unwrap();
    let (lo, hi) = topo::face_cover(&home, cap, band)
        .unwrap()
        .range_along(Vec3::new(1.0, 0.0, 0.0));
    assert!(
        lo.abs() < 1e-6 && (hi - 2.0).abs() < 1e-6,
        "the fit's net: [{lo}, {hi}]"
    );
    let at = geom_core::RigidMotion::translation(Vec3::new(997.5, 0.0, 0.0)).unwrap();
    let image = topo::transform_rigid(&home, &at).unwrap();
    for (door, (class, _)) in [
        ("gate", gate_class(&image, tol)),
        ("placing door", door_class(&home, &at, tol)),
    ] {
        assert_eq!(class, "Ok", "{door}: at the wall, in the box");
    }
}

/// A ball of radius `r` about the origin, poled on `y`: a semicircle
/// revolved whole.
fn ball(r: f64, tol: Tol) -> topo::Body<f64> {
    sweep::test_support::try_revolved_about_y_at::<f64>(
        vec![(Point2::new(0.0, -r), 1.0), (Point2::new(0.0, r), 0.0)],
        Revolution::Full,
        tol,
    )
    .expect("the ball revolves")
}

/// The gate's and the placing door's verdicts on `home` moved by `by`,
/// each checked against `want` (its class and margin).
fn verdicts_at(home: &topo::Body<f64>, by: Vec3<f64>, want: (&str, f64), tol: Tol) {
    let at = geom_core::RigidMotion::translation(by).unwrap();
    let image = topo::transform_rigid(home, &at).unwrap();
    for (door, (class, margin)) in [
        ("gate", gate_class(&image, tol)),
        ("placing door", door_class(home, &at, tol)),
    ] {
        assert!(
            class == want.0 && (margin - want.1).abs() < 1e-6,
            "{door}, moved by {by:?}: read {class} {margin}, want {want:?}"
        );
    }
}

/// **A ball cut by a box reads the whole ball, so the piece, in the box,
/// refuses as a limit by it** (both reviewers of PR 7225's second delta).
/// A ball of radius 1.5 poled on `z`, intersected with the brick
/// `[−1, 1]² × [0, 3]`: its sphere faces are bounded partly by the
/// walls' sections, small circles that are neither latitude rims (circles
/// about the polar axis) nor meridian great circles, so the sphere trim's
/// class (`solid_contain::sphere_chart_trim`) holds no such face,
/// `topo::cover`'s sphere arm reads no window, and they cover the ball. Moved 998.7 m along `x`, the
/// piece reaches 999.7 m and the ball 1000.2 m: refused as the limit
/// 0.2 m past, at the gate and at the placing door; moved 998 m, admitted.
/// *Breaks if* a sphere face a box cuts comes to read a window (then the
/// row's 998.7 m case admits, and the row is the measure of it), or the
/// limit is refused as past.
#[test]
fn a_ball_a_box_cuts_reads_its_whole_ball() {
    let tol = Tol::witness();
    let to_z = geom_core::RigidMotion::rotation(
        unit(Vec3::new(1.0, 0.0, 0.0), tol),
        core::f64::consts::FRAC_PI_2,
    )
    .unwrap();
    let ball = finished(
        "the ball",
        topo::transform_rigid(&ball(1.5, tol), &to_z).unwrap(),
        tol,
    );
    let bar = finished(
        "the brick",
        brick::<f64>((-1.0, 1.0), (-1.0, 1.0), (0.0, 3.0), tol),
        tol,
    );
    let cut = topo::intersect(&ball, &bar, tol).expect("the brick cuts the ball");
    let cut = cut
        .body()
        .expect("the cut is a body")
        .body
        .clone()
        .into_body();
    verdicts_at(&cut, Vec3::new(998.7, 0.0, 0.0), ("Unshown", 0.2), tol);
    verdicts_at(&cut, Vec3::new(998.0, 0.0, 0.0), ("Ok", 0.0), tol);
}

/// **A ball cut by a plane tilted to its poles reads the whole ball; cut
/// square to them, its window** (both reviewers of PR 7225's second
/// delta). A ball of radius 50 poled on `y`, intersected with the half
/// space below a plane 10 m above its centre, turned 0.4 rad about `x`:
/// the rim is a circle tilted to the polar axis, neither a latitude rim
/// nor a meridian great circle, so the sphere trim's class holds no such
/// face and it covers the ball. The kept part's top stands
/// 28.29 m above the centre. Lifted 960 m it reaches 988.3 m and the ball
/// 1010 m: refused as the limit 10 m past, at the gate and at the placing
/// door; lifted 940 m, admitted; lifted 985 m, past by its true 13.29 m.
/// The same ball cut by the plane square to its poles (its rim a latitude
/// rim) reads its window,
/// and is admitted lifted 960 m and 985 m. *Breaks if* the tilted rim's
/// face comes to read a window (the 960 m case then admits), or the
/// square rim's stops reading one.
#[test]
fn a_ball_cut_by_a_plane_tilted_to_its_poles_reads_its_whole_ball() {
    let tol = Tol::witness();
    let cut_at = |tilt: f64| {
        let slab = brick::<f64>((-100.0, 100.0), (-100.0, 10.0), (-100.0, 100.0), tol);
        let turn =
            geom_core::RigidMotion::rotation(unit(Vec3::new(1.0, 0.0, 0.0), tol), tilt).unwrap();
        let slab = finished(
            "the half space",
            topo::transform_rigid(&slab, &turn).unwrap(),
            tol,
        );
        let cut = topo::intersect(&finished("the ball", ball(50.0, tol), tol), &slab, tol)
            .expect("the plane cuts the ball");
        cut.body()
            .expect("the cut is a body")
            .body
            .clone()
            .into_body()
    };
    let tilted = cut_at(0.4);
    verdicts_at(&tilted, Vec3::new(0.0, 960.0, 0.0), ("Unshown", 10.0), tol);
    verdicts_at(&tilted, Vec3::new(0.0, 940.0, 0.0), ("Ok", 0.0), tol);
    let at = geom_core::RigidMotion::translation(Vec3::new(0.0, 985.0, 0.0)).unwrap();
    let (class, margin) = gate_class(&topo::transform_rigid(&tilted, &at).unwrap(), tol);
    assert!(
        class == "Outside" && (margin - 13.288).abs() < 1e-3,
        "lifted 985 m the tilted cut is past by its true reach: {class} {margin}"
    );
    let square = cut_at(0.0);
    for dy in [960.0, 985.0] {
        verdicts_at(&square, Vec3::new(0.0, dy, 0.0), ("Ok", 0.0), tol);
    }
}
