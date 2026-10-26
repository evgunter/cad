//! **A cone whose stored seam leans along its axis, through the
//! booleans.** Nothing margins a cone's frame at rest
//! (`Surface::representability_margins`), so a cone face may carry a
//! `u_ref` tilted `τ` off square to its axis. The section arms square
//! the seam (`geom::square_frame`), so their circles and lines lie on
//! the implicit cone and its partner; the face's own chart
//! (`Surface::eval`) reads the seam as stored and stands up to `R·τ`
//! off both. Each pose either builds the true body (tier 3 and the
//! closed-form volume) or refuses typed, at every ε row. Past `ε/R` the
//! at-rest gate refuses the cone itself (its edges' chart residual);
//! at `ε/4` it admits it, and the axis-normal circle builds in all six
//! ops.
//!
//! The cone is the triangle `(0,0) (1,0) (0,1)` revolved fully about
//! `y` and merged to one cone face (apex `(0, 1, 0)`, `R = 1`, half-angle
//! 45°), its seam then restated tilted. Its partners: a half-space
//! holding the axis (the apex line pair), the half-space `y ≥ 0.5` (the
//! axis-normal circle), and a coaxial pin of radius 0.4 (the coaxial
//! circles).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::operands::bar;
use crate::germ_plane_torus_chord::volume_tol;
use crate::germ_plane_torus_frame::{SIX, six};
use crate::revolve_common::{axis_y, validated};
use core::f64::consts::PI;
use geom::Surface;
use geom_core::{Point2, Tol};
use profile::{ProfileLoop, RawLoop};
use sweep::{Revolution, revolve};
use topo::{AtRestBody, Body, BooleanOp};

/// `polygon` (radius, height) revolved fully about `y`, merged.
fn revolved(polygon: &[(f64, f64)]) -> Body<f64> {
    let loop_ = ProfileLoop::polygon(polygon.iter().map(|&(x, y)| Point2::new(x, y)));
    let mut body = revolve(
        &validated(vec![loop_]),
        axis_y(),
        Revolution::Full,
        Tol::witness(),
    )
    .unwrap()
    .body;
    body.merge_coplanar_faces(Tol::witness()).unwrap();
    body
}

/// The cone with its seam restated `tilt` along its axis: at rest, or
/// the at-rest gate's refusal.
fn tilted_cone(tilt: f64) -> Result<AtRestBody<f64>, String> {
    let mut body = revolved(&[(0.0, 0.0), (1.0, 0.0), (0.0, 1.0)]);
    let mut cones: Vec<_> = body
        .faces()
        .map(|(_, f)| f.surface)
        .filter(|&k| matches!(body.get_surface(k), Some(Surface::Cone { .. })))
        .collect();
    cones.dedup();
    assert_eq!(cones.len(), 1, "one cone carrier: {cones:?}");
    let Some(&Surface::Cone {
        apex,
        axis,
        half_angle,
        u_ref,
    }) = body.get_surface(cones[0])
    else {
        unreachable!()
    };
    let leaning = u_ref * (1.0 - tilt * tilt).sqrt() + axis * tilt;
    assert!(topo::test_support::restate_surface(
        &mut body,
        cones[0],
        Surface::Cone {
            apex,
            axis,
            half_angle,
            u_ref: leaning,
        },
    ));
    AtRestBody::validate(body, Tol::witness()).map_err(|e| format!("{e:?}"))
}

/// The coaxial pin of radius `r` over `y ∈ [−1, 2]`.
fn pin(r: f64) -> AtRestBody<f64> {
    AtRestBody::validate(
        revolved(&[(0.0, -1.0), (r, -1.0), (r, 2.0), (0.0, 2.0)]),
        Tol::witness(),
    )
    .unwrap()
}

#[test]
fn a_cone_with_a_tilted_seam_builds_its_true_body_or_refuses_typed() {
    let cone_volume = PI / 3.0;
    let partners: [(&str, AtRestBody<f64>, f64, f64); 3] = [
        (
            "x ≥ 0 (the apex pair)",
            bar((0.0, 3.0), (-1.0, 3.0), (-3.0, 3.0)),
            36.0,
            cone_volume / 2.0,
        ),
        (
            "y ≥ 0.5 (the axis-normal circle)",
            bar((-3.0, 3.0), (0.5, 3.5), (-3.0, 3.0)),
            108.0,
            PI * 0.25 * 0.5 / 3.0,
        ),
        (
            "pin 0.4 (the coaxial circles)",
            pin(0.4),
            PI * 0.16 * 3.0,
            PI * 0.16 * 0.6 + PI * 0.16 * 0.4 / 3.0,
        ),
    ];
    let quarter = 0.25 * crate::revolve_common::eps();
    for tilt in [0.0, 1e-9, 1e-6, quarter] {
        let mut built = 0;
        let cone = match tilted_cone(tilt) {
            Ok(c) => c,
            Err(e) => {
                assert!(
                    tilt > quarter,
                    "tilt {tilt:e}: the cone refused at rest: {e}"
                );
                eprintln!("tilt {tilt:e}: the cone refused at rest: {e}");
                continue;
            }
        };
        for (pose, partner, partner_volume, meet) in &partners {
            for ((label, r), &(_, op, swap)) in six(&cone, partner).into_iter().zip(&SIX) {
                let (x, y) = if swap {
                    (*partner_volume, cone_volume)
                } else {
                    (cone_volume, *partner_volume)
                };
                let want = match op {
                    BooleanOp::Union => x + y - meet,
                    BooleanOp::Intersect => *meet,
                    BooleanOp::Subtract => x - meet,
                };
                let what = format!("tilt {tilt:e}, {pose}, {label}");
                let r = match r {
                    Ok(r) => r,
                    Err(e) => {
                        eprintln!("{what}: refused {e:?}");
                        continue;
                    }
                };
                let b = &r.body().unwrap_or_else(|| panic!("{what}: empty")).body;
                assert_eq!(
                    topo::validate_geometric(b, Tol::witness()),
                    Ok(()),
                    "{what}: tier 3"
                );
                let v = topo::test_support::pinned_volume("the measured body", b, Tol::witness())
                    .volume;
                assert!(
                    (v - want).abs() <= volume_tol(want),
                    "{what}: volume {v} against the truth {want}"
                );
                eprintln!("{what}: built, volume {v}");
                built += 1;
            }
        }
        if tilt <= quarter {
            assert!(built >= 6, "tilt {tilt:e}: {built} bodies built");
        }
    }
}
