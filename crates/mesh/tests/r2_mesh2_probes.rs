//! R2 reviewer probes for MESH-2 (PR #1421), issue 555.
//!
//! Not part of the unit. These attack the delivered claims through the
//! public API: the completeness of the written-zero fix on RINGED and
//! ANNULAR charts (the shape the Klein consumer actually has), and
//! spade's real `mitigate_underflow` semantics against the siting
//! argument's description of them.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Point3, Tol, Vec3};
use mesh::tessellate;
use mesh::validate::check_mesh;
use profile::RawLoop;
use profile::{Profile, ProfileLoop, SketchPlane, ValidatedProfile};
use sweep::{Extrusion, extrude};
use topo::Body;

fn lp(poly: &[(f64, f64)]) -> ProfileLoop<f64> {
    ProfileLoop::polygon(
        poly.iter()
            .map(|&(x, y)| Point2::new(x, y))
            .collect::<Vec<_>>(),
    )
}

fn validated(plane: SketchPlane<f64>, loops: Vec<ProfileLoop<f64>>) -> ValidatedProfile<f64> {
    Profile::new(plane, loops)
        .validate(Tol::witness())
        .expect("profile validation")
}

/// A prism whose sketch frame is skewed by ν (the unit's own carrier).
fn skewed(nu: f64, loops: Vec<ProfileLoop<f64>>, h: f64) -> Body<f64> {
    let plane = SketchPlane::from_frame(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(1.0, 0.0, nu),
        Vec3::new(0.0, 1.0, 0.0),
    );
    extrude(
        &validated(plane, loops),
        Extrusion::Distance(h),
        Tol::witness(),
    )
    .expect("extrude")
    .body
}

/// PROBE A — the disclaimed diagonal corner, made SYSTEMATIC rather
/// than accidental.
///
/// The module prose says a boundary point that "happens to lie on the
/// anchor→far diagonal" has an exact-zero v by geometry, which the
/// write does not reach. On a plate with a hole, that point is not an
/// accident: place a ring vertex on the outer loop's own diagonal and
/// the same sub-floor refusal returns, on a shape a modeller draws.
#[test]
fn r2_a_ring_vertex_on_the_anchor_far_diagonal_still_refuses() {
    // Outer (0,0)..(2,1): anchor (0,0), far (2,1), diagonal y = x/2.
    let outer = [(0.0, 0.0), (2.0, 0.0), (2.0, 1.0), (0.0, 1.0)];
    // A ring with a vertex exactly at (1.0, 0.5), on that diagonal.
    let ring = [(1.0, 0.5), (1.3, 0.45), (1.3, 0.62), (1.0, 0.62)];
    let mut refused = Vec::new();
    for exp in [-25i32, -30, -40, -50] {
        let nu = 10f64.powi(exp);
        let body = skewed(nu, vec![lp(&outer), lp(&ring)], 1.0);
        match tessellate(&body, 1e-2, Tol::witness()) {
            Ok(m) => {
                assert_eq!(check_mesh(&m), Ok(()), "1e{exp}: not watertight");
                println!("PROBE-A: nu 1e{exp} -> tessellates");
            }
            Err(e) => {
                println!("PROBE-A: nu 1e{exp} -> REFUSED {e:?}");
                refused.push(exp);
            }
        }
    }
    println!("PROBE-A: refused at {refused:?}");
}

/// PROBE B — the ANNULAR cap with sub-floor position noise: the actual
/// Klein shape, which the unit's own annulus row exercises only at
/// ν = 0. An annulus cap's inner ring carries points collinear with the
/// outer loop's anchor→far chord by symmetry, not by luck.
#[test]
fn r2_annular_cap_with_subfloor_noise() {
    // Square annulus: outer square, concentric square hole. The hole's
    // corners sit on the outer square's diagonals.
    let outer = [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)];
    let ring = [(-0.4, -0.4), (-0.4, 0.4), (0.4, 0.4), (0.4, -0.4)];
    for exp in [-15i32, -25, -30, -45, -60] {
        let nu = 10f64.powi(exp);
        let body = skewed(nu, vec![lp(&outer), lp(&ring)], 1.0);
        match tessellate(&body, 1e-2, Tol::witness()) {
            Ok(m) => println!(
                "PROBE-B: nu 1e{exp} -> tessellates, watertight={:?}",
                check_mesh(&m)
            ),
            Err(e) => println!("PROBE-B: nu 1e{exp} -> REFUSED {e:?}"),
        }
    }
}

/// PROBE C — spade's `mitigate_underflow` semantics, against the
/// siting argument's description of them. The prose says a blanket
/// filter "cannot tell the far point's structural zero from a
/// neighbouring point's small-but-real v-coordinate, so it would snap
/// the second". Check what it actually snaps.
#[test]
fn r2_what_mitigate_underflow_actually_snaps() {
    use spade::{Point2 as SP, mitigate_underflow};
    for v in [
        1.0e-30_f64, // "small but real", far above the floor
        1.0e-43,     // just under MIN_ALLOWED_VALUE = 1.7936e-43
        1.0e-48,     // the far point's residue scale in the Klein case
        2.187_945_638_770_979e-48,
        1.0e-320, // subnormal
        0.0,
    ] {
        let out = mitigate_underflow(SP::new(0.5, v));
        println!("PROBE-C: v={v:e} -> {:e}  snapped={}", out.y, out.y != v);
    }
    // The load-bearing pair, asserted.
    assert_eq!(
        mitigate_underflow(SP::new(0.5, 1.0e-30)).y,
        1.0e-30,
        "PROBE-C: mitigate_underflow moved a value ABOVE spade's floor"
    );
    assert_eq!(
        mitigate_underflow(SP::new(0.5, 1.0e-48)).y,
        0.0,
        "PROBE-C: mitigate_underflow left a sub-floor value alone"
    );
}

/// PROBE D — an independent e2e through the public API on a body the
/// merge base refuses: a non-rectangular (L-shaped, non-convex) cap at
/// sub-floor noise, meshed and checked, so the fix is exercised on a
/// shape neither the unit's rows nor the demo uses.
#[test]
fn r2_e2e_nonconvex_cap_at_subfloor_noise() {
    let l_shape = [
        (0.0, 0.0),
        (3.0, 0.0),
        (3.0, 1.0),
        (1.2, 1.0),
        (1.2, 2.5),
        (0.0, 2.5),
    ];
    for exp in [-23i32, -30, -45, -60] {
        let nu = 10f64.powi(exp);
        let body = skewed(nu, vec![lp(&l_shape)], 0.7);
        let m = tessellate(&body, 1e-2, Tol::witness())
            .unwrap_or_else(|e| panic!("PROBE-D: nu 1e{exp} refused {e:?}"));
        assert_eq!(check_mesh(&m), Ok(()), "PROBE-D: nu 1e{exp} not watertight");
        let v = mesh::validate::signed_volume(&m);
        let exact = (3.0 * 1.0 + 1.2 * 1.5) * 0.7;
        assert!(
            (v - exact).abs() < 1e-9,
            "PROBE-D: nu 1e{exp} volume {v} vs {exact}"
        );
        println!("PROBE-D: nu 1e{exp} -> tessellates, volume {v}");
    }
}

/// PROBE E — a slit annulus (full revolve, the Klein cap's own door)
/// built at several proportions, to check the seam cancellation under
/// the write beyond the single size the unit's row uses.
#[test]
fn r2_slit_annulus_at_several_proportions() {
    use geom_core::Vec2;
    use sweep::{Revolution, RevolveAxis, revolve};
    for (r_in, r_out, h) in [
        (0.225_f64, 0.275_f64, 0.4_f64),
        (0.9, 1.0, 0.2),
        (0.01, 0.02, 1.0),
        (2.0, 5.0, 0.05),
    ] {
        let profile = validated(
            SketchPlane::xy(),
            vec![lp(&[(r_in, 0.0), (r_out, 0.0), (r_out, h), (r_in, h)])],
        );
        let body = revolve(
            &profile,
            RevolveAxis {
                origin: Point2::new(0.0, 0.0),
                dir: Vec2::new(0.0, 1.0),
            },
            Revolution::Full,
            Tol::witness(),
        )
        .expect("revolve")
        .body;
        match tessellate(&body, 1e-2, Tol::witness()) {
            Ok(m) => println!(
                "PROBE-E: ({r_in},{r_out},{h}) -> ok, watertight={:?}",
                check_mesh(&m)
            ),
            Err(e) => println!("PROBE-E: ({r_in},{r_out},{h}) -> REFUSED {e:?}"),
        }
    }
}
