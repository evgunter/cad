//! **A sphere cut-in's cut is closed again, so each ball keeps one face
//! per piece of it** (`topo::boolean::slit`, closing a cut). Two families
//! the cut-in reaches, every op, against closed forms:
//!
//! - a plate a ball pokes through both faces, two parallel circles
//!   off every edge, at several pole axes and spins about the pole: the
//!   intersection and the plate less the ball keep the band between the
//!   circles, one face holding both;
//! - the die's pips run together: two `z`-poled balls of radius 0.09 at
//!   centre distances 0.08, 0.12 and 0.17 along three directions, whose
//!   union is the two balls less their lens.
//!
//! Every edge between two pieces of one sphere is one its operands wrote
//! (`common::maximal_spheres`): no cut the kernel made survives. A pose
//! whose seam crossed a plate face would take the crossings path, where a
//! circle off every edge meets the interior-loop guard
//! (`work/germ/interior-loop-cut-in.md`); none of these does.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Point3, Tol, Vec3};
use sweep::test_support::{ball_poled_y, ball_poled_z, brick, finished};
use topo::{AtRestBody, BooleanError, BooleanResult};

use crate::common::oracles::{ball_lens, ball_volume};

fn tol() -> Tol {
    Tol::witness()
}

/// `out` builds at `want` (relative `1e-9` over a floor of `1e-15` of
/// `scale`), every edge between two pieces of one sphere one its
/// `operands` wrote; `false` where it refuses at the interior-loop guard
/// instead.
fn built(
    label: &str,
    out: Result<BooleanResult<f64>, BooleanError>,
    (want, scale): (f64, f64),
    operands: [&topo::Body<f64>; 2],
) -> bool {
    let body = match out {
        Ok(BooleanResult::Body(b)) => b.body,
        Err(BooleanError::CurvedPairUnsupported {
            site: topo::PairRefusalSite::InteriorLoopGuard,
            ..
        }) => return false,
        other => panic!("{label}: a body or the interior-loop guard, not {other:?}"),
    };
    crate::common::maximal_spheres::assert_spheres_maximal(label, &body, &operands);
    let v = topo::test_support::pinned_volume(label, &body, tol()).volume;
    assert!(
        (v - want).abs() <= 1e-9 * want.abs() + 1e-15 * scale,
        "{label}: volume {v} against the closed form {want}"
    );
    true
}

/// The `y`-poled ball of radius `r`, its pole turned onto `axis`, spun
/// `spin` about it, centred at `c`.
fn ball_toward(r: f64, axis: Vec3<f64>, spin: f64, c: Vec3<f64>) -> AtRestBody<f64> {
    let o = Point3::origin();
    let axis = axis / axis.norm();
    let y = Vec3::new(0.0, 1.0, 0.0);
    let across = y.cross(axis);
    let tilt = if across.norm() < 1e-12 {
        Affine3::identity()
    } else {
        Affine3::rotation_about_axis(o, across / across.norm(), y.dot(axis).acos())
    };
    let turn = Affine3::translation(c) * Affine3::rotation_about_axis(o, axis, spin) * tilt;
    let ball = ball_poled_y(r, Vec3::new(0.0, 0.0, 0.0), tol());
    let ball = topo::transform_rigid(&ball, &geom_core::test_support::rigid(&turn, tol())).unwrap();
    finished("the ball", ball, tol())
}

#[test]
fn a_ball_through_a_plate_keeps_its_band_as_one_face_under_every_op() {
    let (r, t) = (0.4f64, 0.5f64);
    let c = Vec3::new(0.3, -0.2, t / 2.0);
    let plate = finished(
        "the plate",
        brick::<f64>((-2.0, 2.0), (-2.0, 2.0), (0.0, t), tol()),
        tol(),
    );
    let (vp, vb) = (4.0 * 4.0 * t, ball_volume(r));
    // The band between the two circles, `|s| ≤ t/2` about the centre.
    let h = t / 2.0;
    let band = std::f64::consts::PI * (r * r * 2.0 * h - 2.0 * h.powi(3) / 3.0);
    let mut builds = 0;
    for axis in [
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(1.0, 1.0, 1.0),
        Vec3::new(0.3, 0.8, 0.52),
    ] {
        for spin in [0.0f64, 45.0, 90.0, 135.0] {
            let ball = ball_toward(r, axis, spin.to_radians(), c);
            let pose = format!("axis {axis:?}, spin {spin}");
            for (op, out, want) in [
                ("plate ∩ ball", topo::intersect(&plate, &ball, tol()), band),
                (
                    "plate ∖ ball",
                    topo::subtract(&plate, &ball, tol()),
                    vp - band,
                ),
                (
                    "ball ∖ plate",
                    topo::subtract(&ball, &plate, tol()),
                    vb - band,
                ),
                (
                    "plate ∪ ball",
                    topo::union(&plate, &ball, tol()),
                    vp + vb - band,
                ),
            ] {
                let label = format!("{pose}, {op}");
                builds += usize::from(built(&label, out, (want, vp + vb), [&plate, &ball]));
            }
        }
    }
    // A fixed set of poses, each of which clears every seam of both
    // plate faces, so every op of each takes the cut-in and builds.
    assert_eq!(builds, 64, "every pose's every op builds");
}

#[test]
fn the_dies_pips_run_together_keep_one_face_each_under_union() {
    let r = 0.09f64;
    let v = ball_volume(r);
    let at = Vec3::new(0.5, 0.5, 1.0);
    let ball = |c: Vec3<f64>| finished("a pip ball", ball_poled_z(r, c, tol()), tol());
    let mut builds = 0;
    for d in [0.08f64, 0.12, 0.17] {
        for dir in [
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(1.0, 1.0, 0.0),
            Vec3::new(1.0, 0.0, 1.0),
        ] {
            let (a, b) = (ball(at), ball(at + dir / dir.norm() * d));
            let label = format!("d {d}, along {dir:?}");
            let want = 2.0 * v - ball_lens(r, r, d);
            builds += usize::from(built(
                &label,
                topo::union(&a, &b, tol()),
                (want, 2.0 * v),
                [&a, &b],
            ));
        }
    }
    assert_eq!(builds, 9, "every pip pair's union builds");
}
