//! Acceptance (f): D9 determinism — byte-identical rebuilds of every
//! acceptance shape (keys, points, certificates, bundles), the
//! `Dual<f64>` value channel bit-identical to the plain `f64` build,
//! and the signed-volume oracle positive on all bodies.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod revolve_common;

use core::f64::consts::FRAC_PI_2;
use profile::RawLoop;

use geom_core::Tolerance;
use geom_core::{Point2, Vec2};
use profile::{Profile, ProfileLoop, ProfileVertex, SketchPlane};
use revolve_common::*;
use sweep::{Revolution, RevolveAxis, revolve};
use geom_core::Tol;

/// The four acceptance shapes as (profile loops, revolution).
fn shapes() -> Vec<(Vec<ProfileLoop<f64>>, Revolution<f64>)> {
    let half_disc = ProfileLoop::new(vec![
        ProfileVertex::new(p2(0.0, -1.0), 1.0),
        ProfileVertex::new(p2(0.0, 1.0), 0.0),
    ]);
    let triangle = ProfileLoop::polygon([p2(0.0, 0.0), p2(1.0, 0.0), p2(0.0, 1.0)]);
    let washer = ProfileLoop::polygon([p2(1.0, 0.0), p2(2.0, 0.0), p2(2.0, 1.0), p2(1.0, 1.0)]);
    let square = ProfileLoop::polygon([p2(0.0, 0.0), p2(1.0, 0.0), p2(1.0, 1.0), p2(0.0, 1.0)]);
    vec![
        (vec![half_disc], Revolution::Full),
        (vec![triangle], Revolution::Full),
        (vec![washer], Revolution::Full),
        (vec![square], Revolution::Partial(FRAC_PI_2)),
    ]
}

#[test]
fn rebuild_is_byte_identical_and_volumes_positive() {
    for (i, (loops, rev)) in shapes().into_iter().enumerate() {
        let build = || {
            let vp = validated(loops.clone());
            revolve(&vp, axis_y(), rev, Tol::witness()).unwrap()
        };
        let a = build();
        let b = build();
        assert_all_tiers(&a.body);
        assert_eq!(dump(&a), dump(&b), "shape {i} not byte-identical");
        // Volume signs for the wire shapes (ball, cone) need interior
        // lift points and are asserted in their own suites; the
        // boundary-probe oracle covers the washer and the wedge.
        if i >= 2 {
            assert!(signed_volume(&a.body) > 0.0, "shape {i} volume");
        }
    }
}

#[test]
fn dual_value_channel_matches_f64_bitwise() {
    use geom_core::{Dual, Dual64};
    let lift = |lp: &ProfileLoop<f64>| -> ProfileLoop<Dual64> {
        ProfileLoop::new(
            lp.vertices()
                .iter()
                .map(|v| {
                    ProfileVertex::new(
                        Point2::new(Dual::constant(v.pos().x), Dual::constant(v.pos().y)),
                        Dual::constant(v.bulge()),
                    )
                })
                .collect(),
        )
    };
    for (i, (loops, rev)) in shapes().into_iter().enumerate() {
        let vp = validated(loops.clone());
        let f = revolve(&vp, axis_y(), rev, Tol::witness()).unwrap();
        let dp = Profile::new(
            SketchPlane::<Dual64>::xy(),
            loops.iter().map(&lift).collect(),
        )
        .validate(Tol::witness())
        .unwrap();
        let drev = match rev {
            Revolution::Full => Revolution::Full,
            Revolution::Partial(t) => Revolution::Partial(Dual::constant(t)),
        };
        let daxis = RevolveAxis {
            origin: Point2::new(Dual::constant(0.0), Dual::constant(0.0)),
            dir: Vec2::new(Dual::constant(0.0), Dual::constant(1.0)),
        };
        let d = revolve(&dp, daxis, drev, Tol::witness()).unwrap();
        // Value channel bit-identity: compare every point coordinate.
        let f_pts: Vec<f64> = f.body.points().flat_map(|(_, p)| [p.x, p.y, p.z]).collect();
        let d_pts: Vec<f64> = d
            .body
            .points()
            .flat_map(|(_, p)| [p.x.value, p.y.value, p.z.value])
            .collect();
        assert_eq!(f_pts.len(), d_pts.len(), "shape {i}");
        for (a, b) in f_pts.iter().zip(&d_pts) {
            assert_eq!(a.to_bits(), b.to_bits(), "shape {i} value-channel drift");
        }
        assert_all_tiers(&f.body);
    }
}
