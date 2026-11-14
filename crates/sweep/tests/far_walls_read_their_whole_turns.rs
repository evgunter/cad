//! **A far, generic-pose cylinder or torus wall reads its loops' whole
//! turns at every ε.** Tier 3's certificate walk re-derives its round at
//! the interval scalar, where each edge's whole-turn miss
//! (`props_chart_loop_whole`) is stated where it is exact: a ruling's
//! short turn misses its ends by nothing, and a rim's or a parallel's
//! carried turn is read about its own centre. Read back about the axis,
//! the miss carried the width of the edge's ends there, times a lever of
//! hundreds of metres, past the loop's `3ε` at ε = 1e-12.
//!
//! - **An arc-topped block on a tilted sketch**: a `1 × 1` loop whose top
//!   side is an arc of radius [`R`], extruded `0.7`, measures
//!   `0.7·(1 + segment)`.
//! - **A tilted hollow tube** (axis `(1, 1, 1)/√3`, outer radius `2.5`,
//!   wall `1 mm`, a full period) measures its shell at majors 300 and
//!   600. At majors 990 and 1100 and ε = 1e-12 it still escalates
//!   `props_chart_loop_whole`, on the two whole-turn parallels' carried
//!   misses: the width of each parallel's two ends, evaluated apart, at a
//!   lever of about 1 km
//!   (`work/spiricarm/a-whole-turn-parallels-ends-are-read-apart-at-the-interval-scalar.md`).
//!   That arm is pinned, and turns red when that row lands.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom_core::{Point2, Point3, Tol, Vec3};
use profile::{Profile, test_support::bulge_loop};
use sweep::test_support::{sketch_from_axes, tube_frame};
use sweep::{ExtrudeSide, Extrusion, RevolveError, TubeError, TubeWindow, tube_along_arc_hollow};

/// The block's arc radius.
const R: f64 = 213.083_862_394_847_5;

#[test]
fn an_arc_topped_block_on_a_tilted_sketch_measures() {
    let tol = Tol::witness();
    let plane = sketch_from_axes(
        Point3::new(
            0.071_515_309_154_077_44,
            0.567_366_954_668_013_6,
            -0.365_901_202_832_783_2,
        ),
        Vec3::new(
            0.170_103_632_992_922_47,
            -0.269_062_789_066_274_4,
            0.947_982_051_297_642_8,
        ),
        Vec3::new(
            0.483_912_835_974_898_4,
            -0.815_211_650_898_433_3,
            -0.318_211_142_825_297_67,
        ),
        tol,
    );
    let theta = 2.0 * (0.5 / R).asin();
    let lp = bulge_loop(vec![
        (Point2::new(0.0, 0.0), 0.0),
        (Point2::new(1.0, 0.0), 0.0),
        (Point2::new(1.0, 1.0), (theta / 4.0).tan()),
        (Point2::new(0.0, 1.0), 0.0),
    ]);
    let vp = Profile::new(plane, vec![lp])
        .validate(tol)
        .expect("the profile");
    let e = sweep::extrude(
        &vp,
        Extrusion::Distance {
            depth: 0.7,
            side: ExtrudeSide::Along,
        },
        tol,
    )
    .unwrap_or_else(|e| panic!("the block refused: {e:?}"));
    let got = topo::test_support::pinned_volume("the block", &e.body, tol).volume;
    let want = 0.7 * (1.0 + 0.5 * R * R * (theta - theta.sin()));
    assert!(
        (got - want).abs() <= 1e-9 * want,
        "the block's volume: {got} against {want}"
    );
}

#[test]
fn a_tilted_hollow_tube_measures() {
    let tol = Tol::witness();
    let (s3, s2) = (3.0_f64.sqrt().recip(), 2.0_f64.sqrt().recip());
    let (outer, wall) = (2.5, 0.001);
    let inner: f64 = outer - wall;
    for major in [300.0, 600.0, 990.0, 1100.0] {
        let frame = tube_frame(
            Point3::new(0.0, 0.0, 0.0),
            Vec3::new(s3, s3, s3),
            Vec3::new(s2, -s2, 0.0),
            tol,
        );
        let got = tube_along_arc_hollow::<f64>(frame, major, TubeWindow::Full, outer, wall, tol);
        if major > 900.0 && tol.eps() <= 1e-12 {
            let pinned = matches!(
                &got,
                Err(TubeError::Revolve(RevolveError::NotFinished { findings }))
                    if findings.iter().all(|f| matches!(
                        f,
                        topo::ValidationError::VolumeUncomputable { .. }
                    )) && format!("{findings:?}").contains("props_chart_loop_whole")
            );
            assert!(
                pinned,
                "major {major}: pinned to escalate props_chart_loop_whole at ε 1e-12; \
                 if it built, the whole-turn parallel row landed and this arm lifts: {got:?}"
            );
            continue;
        }
        let t = got.unwrap_or_else(|e| panic!("major {major}: {e:?}"));
        let got = topo::test_support::pinned_volume("the tube", &t.body, tol).volume;
        let want = 2.0 * PI * PI * major * (outer * outer - inner * inner);
        assert!(
            (got - want).abs() <= 1e-9 * want,
            "major {major}: the shell's volume {got} against {want}"
        );
    }
}
