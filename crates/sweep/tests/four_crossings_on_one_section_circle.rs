//! **Four crossings on one section circle pair along the walk.** A slab
//! `x > c, z_a < z < z_b` against the unit ball: the ball's section by
//! the slab's face `x = c` is a circle of radius `ρ = √(1 − c²)`, and
//! the slab's faces `z = z_a` and `z = z_b` cross it four times, at
//! `θ_a`, `θ_b`, `π − θ_b` and `π − θ_a` (`z = ρ sin θ`). The arcs
//! between them alternate inside and outside the slab's face, so the
//! boolean join pairs each crossing with its neighbour ALONG the circle
//! — and on the poses here that neighbour is not the nearest crossing
//! by chord: at `θ = −40°, 60°, 120°, 220°` the chord from `60°` to
//! `120°` is shorter than the one from `−40°` to `60°` that the germs at
//! `−40°` and `60°` face along.
//!
//! The join's matcher rejects a facing conic pair when another site of
//! the same locus lies strictly between them along the near germ's walk
//! (`boolean::join`'s `walk_passes`: `bool_join_walk_site`, a third
//! site's distance to either end, and `bool_join_walk_order`, its sweep
//! angle about the centre against the far end's). Without it the
//! chord-nearest pair is taken and the join refuses at the ring it
//! leaves on the ball's face.
//!
//! `bool_join_walk_site` answers `Zero` on every call for the pair's own
//! two germs (each is a site of its own locus, at distance exactly
//! zero), so that branch runs in every row. Another record's germ at an
//! end's site reaches it too — a coincident copy, not between the two
//! ends, so the pair stands:
//! `run_walls_built::revolved_runs_build_one_wall_each` reaches it and
//! builds every wall.
//!
//! The ball is the canonical full revolve about `y`, and the same ball
//! with its pole turned off every axis; the turn moves no point of the
//! sphere, so both read one closed form: the slab's share of the ball,
//! `∫ seg(√(1 − z²), c) dz` over `(z_a, z_b)` with
//! `seg(r, c) = r² acos(c/r) − c√(r² − c²)` the disc segment beyond
//! `x = c` (Simpson's rule on a smooth integrand, `r > c` throughout).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use sweep::Revolution;
use sweep::test_support::{brick, revolved_about_y};
use topo::{Body, BooleanOp};

/// The unit ball at the origin, poles on world `y`, turned by `turn`
/// radians about `axis` through its centre.
fn ball(axis: Vec3<f64>, turn: f64) -> Body<f64> {
    let b = revolved_about_y(
        vec![(Point2::new(0.0, -1.0), 1.0), (Point2::new(0.0, 1.0), 0.0)],
        Revolution::Full,
        Tol::witness(),
    );
    if turn == 0.0 {
        return b;
    }
    let t = Affine3::rotation_about_axis(Point3::origin(), axis, turn);
    topo::transform_rigid(&b, &t, Tol::witness()).expect("a rigid pose")
}

/// The unit ball's share of `x > c, z_a < z < z_b`.
fn slab_share(c: f64, za: f64, zb: f64) -> f64 {
    let seg = |z: f64| {
        let r = (1.0 - z * z).sqrt();
        r * r * (c / r).acos() - c * (r * r - c * c).sqrt()
    };
    let n = 4000;
    let h = (zb - za) / f64::from(n);
    let inner: f64 = (1..n)
        .map(|i| {
            let w = if i % 2 == 1 { 4.0 } else { 2.0 };
            w * seg(za + f64::from(i) * h)
        })
        .sum();
    h / 3.0 * (seg(za) + seg(zb) + inner)
}

fn run(op: BooleanOp, a: &Body<f64>, b: &Body<f64>) -> Body<f64> {
    let out = match op {
        BooleanOp::Union => topo::boolean::union(a, b, Tol::witness()),
        BooleanOp::Intersect => topo::boolean::intersect(a, b, Tol::witness()),
        BooleanOp::Subtract => topo::boolean::subtract(a, b, Tol::witness()),
    };
    out.unwrap_or_else(|e| panic!("refused: {e:?}"))
        .body()
        .unwrap_or_else(|| panic!("came back empty"))
        .body
        .clone()
}

/// **A slab across four crossings that straddle the circle's lowest
/// point builds under every boolean, at the slab's closed form**, on the
/// `y`-poled ball and on one turned off every axis. Disable either rung
/// of the walk check and the row refuses at the ring the chord-nearest
/// pairing leaves.
#[test]
fn a_slab_crossing_one_section_circle_four_times_builds_under_every_boolean() {
    let ball_volume = 4.0 / 3.0 * PI;
    for (c, deg_a, deg_b) in [(0.3f64, -40.0f64, 60.0f64), (0.0, -10.0, 80.0)] {
        let rho = (1.0 - c * c).sqrt();
        let (za, zb) = (
            rho * deg_a.to_radians().sin(),
            rho * deg_b.to_radians().sin(),
        );
        let slab: Body<f64> = brick((c, 3.0), (-3.0, 3.0), (za, zb), Tol::witness());
        let slab_volume = (3.0 - c) * 6.0 * (zb - za);
        let shared = slab_share(c, za, zb);
        for (pole, b) in [
            ("y-poled", ball(Vec3::unit_z(), 0.0)),
            ("turned", ball(Vec3::new(0.4, 0.1, 0.9), 1.3)),
        ] {
            for (name, op, x, y, expected) in [
                (
                    "slab ∪ ball",
                    BooleanOp::Union,
                    &slab,
                    &b,
                    slab_volume + ball_volume - shared,
                ),
                ("slab ∩ ball", BooleanOp::Intersect, &slab, &b, shared),
                (
                    "slab ∖ ball",
                    BooleanOp::Subtract,
                    &slab,
                    &b,
                    slab_volume - shared,
                ),
                (
                    "ball ∖ slab",
                    BooleanOp::Subtract,
                    &b,
                    &slab,
                    ball_volume - shared,
                ),
            ] {
                let label = format!("x > {c}, θ = {deg_a}°/{deg_b}°, {pole} ball, {name}");
                let body = run(op, x, y);
                assert_eq!(topo::validate_closed(&body), Ok(()), "{label}: tier 2");
                assert_eq!(
                    topo::validate_geometric(&body, Tol::witness()),
                    Ok(()),
                    "{label}: tier 3"
                );
                let p = topo::mass_properties(&body, Tol::witness())
                    .unwrap_or_else(|e| panic!("{label}: mass properties, {e:?}"));
                assert_eq!(p.volume_pad, 0.0, "{label}: closed-form faces only");
                assert!(
                    (p.volume - expected).abs() <= 1e-9 * expected,
                    "{label}: volume {} against the closed form {expected}",
                    p.volume
                );
            }
        }
    }
}

/// **Four crossings all above the circle's centre stop typed at the
/// sphere ring.** At `x > 0.5, θ = 10°, 70°, 110°, 170°` the walk pairs
/// the crossings as above, and the section's loop on the ball's face is
/// a ring clear of every edge of that face: the ring lane has no island
/// winding on a sphere (`work/tang/a-ring-on-a-sphere-face-has-no-island-winding.md`).
#[test]
fn four_crossings_above_the_centre_stop_at_the_sphere_ring() {
    let (c, rho) = (0.5f64, 0.75f64.sqrt());
    let (za, zb) = (
        rho * 10f64.to_radians().sin(),
        rho * 70f64.to_radians().sin(),
    );
    let slab: Body<f64> = brick((c, 3.0), (-3.0, 3.0), (za, zb), Tol::witness());
    for (pole, b) in [
        ("y-poled", ball(Vec3::unit_z(), 0.0)),
        ("turned", ball(Vec3::new(0.4, 0.1, 0.9), 1.3)),
    ] {
        for (name, out) in [
            (
                "slab ∪ ball",
                topo::boolean::union(&slab, &b, Tol::witness()),
            ),
            (
                "slab ∩ ball",
                topo::boolean::intersect(&slab, &b, Tol::witness()),
            ),
            (
                "slab ∖ ball",
                topo::boolean::subtract(&slab, &b, Tol::witness()),
            ),
            (
                "ball ∖ slab",
                topo::boolean::subtract(&b, &slab, Tol::witness()),
            ),
        ] {
            match out {
                Err(topo::BooleanError::Join(topo::SplitJoinError::RingOffCylinderChart {
                    kind: geom::SurfaceKind::Sphere,
                    ..
                })) => {}
                other => panic!(
                    "{pole} ball, {name}: expected the sphere ring's door, got {:?}",
                    other.map(|_| "a body")
                ),
            }
        }
    }
}
