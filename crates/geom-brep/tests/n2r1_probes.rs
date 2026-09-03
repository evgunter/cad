//! CERT-N2 R1 reviewer probe — the consumer class the PR body's
//! blast-radius table omits: `topo::props::nurbs_face`'s quadrature
//! lane, whose only guard against a poisoned control net is
//! `is_placeholder`. Its integrand door is public here.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::shared::tol::band;
use geom_brep::props::quad::nurbs_patch_face;
use geom_core::spline::KnotVector;
use geom_core::{RingInterval, Tol};

/// The masquerade driven through the mass-properties patch door: a
/// bilinear net whose every control point has a poisoned `x` and
/// finite `y`/`z`. Before this PR the caller's `is_placeholder` guard
/// caught it; now it reaches here. Report what LEAVES the door.
#[test]
fn probe_masquerading_net_through_the_props_quadrature_door() {
    let kv = KnotVector::unit_segment(1);
    let p = |x: f64, y: f64, z: f64| {
        [
            RingInterval::point(x),
            RingInterval::point(y),
            RingInterval::point(z),
        ]
    };
    let nan = f64::NAN;
    let net = [
        p(nan, 0.0, 1.0),
        p(nan, 1.0, 1.0),
        p(nan, 0.0, 1.0),
        p(nan, 1.0, 1.0),
    ];
    let out = nurbs_patch_face::<f64>(
        &kv,
        &kv,
        &net,
        &[1.0; 4],
        (0.0, 1.0, 0.0, 1.0),
        4.0,
        0.0,
        Tol::witness().get().eps,
        band(),
    );
    match out {
        Ok(b) => panic!(
            "PROBE RESULT: the quadrature door returned Ok on a poisoned net — \
             flux {:?} area {:?}",
            b.flux, b.area
        ),
        Err(e) => println!("PROBE RESULT: typed refusal {e:?}"),
    }
}

/// The same net poisoned in `z` instead of `x` — the channel the flux
/// integrand weights differently — so the answer is not an artifact of
/// which channel carries the poison.
#[test]
fn probe_masquerading_net_poisoned_in_z_through_the_same_door() {
    let kv = KnotVector::unit_segment(1);
    let p = |x: f64, y: f64, z: f64| {
        [
            RingInterval::point(x),
            RingInterval::point(y),
            RingInterval::point(z),
        ]
    };
    let nan = f64::NAN;
    let net = [
        p(0.0, 0.0, nan),
        p(0.0, 1.0, nan),
        p(1.0, 0.0, nan),
        p(1.0, 1.0, nan),
    ];
    let out = nurbs_patch_face::<f64>(
        &kv,
        &kv,
        &net,
        &[1.0; 4],
        (0.0, 1.0, 0.0, 1.0),
        4.0,
        0.0,
        Tol::witness().get().eps,
        band(),
    );
    match out {
        Ok(b) => panic!(
            "PROBE RESULT (z): Ok on a poisoned net — flux {:?} area {:?}",
            b.flux, b.area
        ),
        Err(e) => println!("PROBE RESULT (z): typed refusal {e:?}"),
    }
}
