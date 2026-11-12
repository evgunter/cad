//! **Probe (ONEDOOR loop lift, PR 6851): the sphere twin-joint
//! divergence between `decide_joint` and `chart_window`.** A
//! measurement, not a product test.
//!
//! The unit sphere about `z`. One edge is the `x z` great circle's
//! upper half from `A = (1, 0, 0)` over the north pole to
//! `B = (−1, 0, 0)` (a meridian image that runs over a pole, no vertex
//! there); the other is the equator's half from `B` back to `A` through
//! `(0, −1, 0)`. The two faces are the quarter sphere `{y < 0, z > 0}`
//! and its complement. The over-pole image ends on the chart's twin
//! sheet, so the mint's `Orbit::of` (half-period steps on a sphere) can
//! close the joint at `B` (and `A`) through the twin, where
//! `chart_window`'s `nearest_branch` (`Orbit::Periods`) sees a gap of
//! half a period.
//!
//! A second body puts a vertex on the great circle at `N` (the half
//! split there into two quarter meridians), so the same region's loop
//! has a pole joint instead.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{FRAC_PI_2, PI};

use geom::Curve3;
use geom_brep::EdgeCurveSpec;
use geom_core::{Point3, Tol, Vec3};
use topo::Body;
use topo::construction::{Construct, FaceSurface, MefSite, MevSite};

use crate::zz_probe_looplift_tilted_pole::{dump_body, unit_sphere};

/// The `x z` great circle by latitude: `eval(t) = (cos t, 0, sin t)`;
/// `t = π/2` is `N`, `t = π` is `B`.
fn great_xz() -> Curve3<f64> {
    Curve3::circle(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, -1.0, 0.0),
        1.0,
        Vec3::new(1.0, 0.0, 0.0),
    )
}

/// The equator from `B`: `eval(t) = (−cos t, −sin t, 0)`; `t = π` is
/// `A`.
fn equator_from_b() -> Curve3<f64> {
    Curve3::circle(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
        1.0,
        Vec3::new(-1.0, 0.0, 0.0),
    )
}

fn quarter(sense: bool, vertex_at_pole: bool) -> Body<f64> {
    let tol = Tol::witness();
    let g = great_xz();
    let mut body = Body::<f64>::new();
    let start = body.mvfs(g.eval(0.0), sense).unwrap();
    body.set_face_surface(
        start.face,
        FaceSurface::New {
            surface: unit_sphere(),
            sense,
        },
    )
    .unwrap();
    let lone = MevSite::Lone {
        r#loop: start.r#loop,
    };
    let (first_plus, last_minus) = if vertex_at_pole {
        let m1 = body
            .mev(
                lone,
                g.eval(FRAC_PI_2),
                EdgeCurveSpec::arc_of_circle(g.clone(), 0.0, FRAC_PI_2).unwrap(),
                tol,
            )
            .unwrap();
        let m2 = body
            .mev(
                MevSite::Fan {
                    he1: m1.he_minus,
                    he2: m1.he_minus,
                },
                g.eval(PI),
                EdgeCurveSpec::arc_of_circle(g.clone(), FRAC_PI_2, PI).unwrap(),
                tol,
            )
            .unwrap();
        (m1.he_plus, m2.he_minus) // m2: N → B, its minus half B → N
    } else {
        let m = body
            .mev(
                lone,
                g.eval(PI),
                EdgeCurveSpec::arc_of_circle(g.clone(), 0.0, PI).unwrap(),
                tol,
            )
            .unwrap();
        (m.he_plus, m.he_minus)
    };
    // The chord from `start(he1)` = B to `start(he2)` = A.
    body.mef(
        MefSite::Chords {
            he1: last_minus,
            he2: first_plus,
        },
        EdgeCurveSpec::arc_of_circle(equator_from_b(), 0.0, PI).unwrap(),
        FaceSurface::Inherit,
        tol,
    )
    .unwrap();
    body
}

#[test]
fn probe_a_twin_joint_against_the_chart_window_walk() {
    for sense in [true, false] {
        for at_pole in [false, true] {
            let body = quarter(sense, at_pole);
            let name = format!(
                "quarter sphere, sense {sense}, {}",
                if at_pole {
                    "vertex at N"
                } else {
                    "one edge over N"
                }
            );
            dump_body(&name, &body);
            let mut minted = body.clone();
            let got = topo::mint_pcurves(&mut minted, Tol::witness());
            println!("-- after mint_pcurves: {got:?}");
            if got.is_ok() {
                dump_body(&format!("{name}, re-minted"), &minted);
            }
        }
    }
}
