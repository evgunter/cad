//! **A tiny lune on a wide sphere levers its wedges at its own extent**
//! (`topo::wedge_reach`). The faces' reach reads each face at its
//! boundary's vertices; it does not lever a curved face's edges round
//! their whole carrier, which on a sphere of radius `R` reads `2R`
//! however small the face. That upper bound serves the splitting
//! reader (`face_reach_from`); for a wedge it would read a corner more
//! definitely than the faces support.
//!
//! The fixture is a sliver of profile on the circle of radius
//! [`R_SPHERE`] about the axis, its arc `Δ` of turn with `Δ` set from
//! the ambient ε so the arc's sagitta `R·Δ²/8` is at least a thousand ε
//! (the profile reads it as an arc at every ε row), revolved a radian: its
//! sphere-zone face is a lune of a sphere of radius 2, the body a small
//! fraction of the sphere across. Every edge's reach is bounded by the
//! body's own diameter, not the sphere's. It pins the curved-carrier
//! over-statement (the first review's M1), not the along-edge run
//! (`work/twice/the-wedge-reach-counts-a-faces-run-along-its-edge.md`).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::revolve_common;

use geom_core::{Point2, Tol};
use profile::{ProfileLoop, test_support::bulge_loop};
use revolve_common::*;
use sweep::{Revolution, revolve};

/// The sphere's radius, in metres.
const R_SPHERE: f64 = 2.0;

/// The profile: an arc of the circle of radius [`R_SPHERE`] about the
/// origin from polar angle `a` to `b` off the axis, closed by two
/// segments through a point `depth` inside it.
fn sliver(a: f64, b: f64, depth: f64, bulge_sign: f64) -> ProfileLoop<f64> {
    let on = |t: f64| Point2::new(R_SPHERE * t.sin(), R_SPHERE * t.cos());
    let mid = 0.5 * (a + b);
    let inner = Point2::new(
        (R_SPHERE - depth) * mid.sin(),
        (R_SPHERE - depth) * mid.cos(),
    );
    bulge_loop(vec![
        (on(a), bulge_sign * ((b - a) / 4.0).tan()),
        (on(b), 0.0),
        (inner, 0.0),
    ])
}

#[test]
fn a_tiny_lune_on_a_wide_sphere_levers_every_edge_at_the_bodys_own_extent() {
    let tol = Tol::witness();
    // Never under 2e-3 of turn: the arc's centre, read back from its
    // bulge, drifts off the axis as `1/Δ`, into the band at 1e-12.
    let delta = (8000.0 * tol.eps() / R_SPHERE).sqrt().max(2.0e-3);
    let (a, b, depth) = (delta, 2.0 * delta, R_SPHERE * delta / 2.0);
    // The arc's bulge sign is the profile's convention: whichever sign
    // puts the arc on the circle of radius R builds a sphere face.
    let mut refused = Vec::new();
    let body = [1.0, -1.0]
        .into_iter()
        .find_map(|sign| {
            let vp = validated(vec![sliver(a, b, depth, sign)]);
            let t = revolve(&vp, axis_y(), Revolution::Partial(1.0), tol)
                .map_err(|e| refused.push(format!("{sign}: {e:?}")))
                .ok()?;
            let lune = t.body.faces().any(|(_, f)| {
                matches!(
                    t.body.get_surface(f.surface).unwrap(),
                    geom::Surface::Sphere { radius, .. } if (radius - R_SPHERE).abs() < 1e-9
                )
            });
            lune.then_some(t.body)
        })
        .unwrap_or_else(|| {
            panic!("the sliver revolves to a body with a lune of the R = 2 sphere: {refused:?}")
        });
    assert_all_tiers(&body);
    let points: Vec<_> = body
        .vertices()
        .map(|(v, _)| topo::readback::vertex_point(&body, v).unwrap())
        .collect();
    let diameter = points
        .iter()
        .flat_map(|p| points.iter().map(move |q| p.distance(*q)))
        .fold(0.0_f64, f64::max);
    assert!(
        diameter < R_SPHERE / 8.0,
        "the body is small beside its sphere: {diameter}"
    );
    for (edge, _) in body.edges() {
        let reach = topo::edge_wedge_reach(&body, edge).unwrap();
        assert!(
            reach <= 2.0 * diameter,
            "{edge:?}: the wedge's reach {reach} is the faces' (diameter {diameter}), \
             not the sphere's {}",
            2.0 * R_SPHERE
        );
    }
}
