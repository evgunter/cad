//! A z-poled ball's lune split by `mef` along a meridian OUTSIDE it: its
//! faces then cover the sphere twice, each holds both poles on its loop,
//! and their seam region has no boundary, so tier 3 reads no side for
//! it. The body passes tier 3 and measures twice the ball.
//!
//! This row PINS that residual as it stands
//! (`work/flux/a-seam-region-covering-the-sphere-twice-is-unread.md`):
//! when its owner lands, the body is refused and the row goes red for
//! the right reason.
#![allow(clippy::unwrap_used, clippy::panic)]

use crate::seam_region_common::{sphere_faces, split_at_meridian, v};
use geom_core::Tol;
use std::f64::consts::PI;
use sweep::test_support::{ball_poled, finished};
use topo::Body;

#[test]
fn an_exterior_meridian_covers_the_sphere_twice_and_passes() {
    let tol = Tol::witness();
    let ball = finished(
        "ball",
        ball_poled(1.0, v([0.0; 3]), v([0.0, 0.0, 1.0]), tol),
        tol,
    );
    let mut body = Body::clone(&ball);
    let lune = sphere_faces(&body)[0];
    split_at_meridian(&mut body, lune, v([0.0, 1.0, 0.0]), tol);
    assert_eq!(sphere_faces(&body).len(), 3, "the lune is split in two");
    topo::validate_geometric(&body, tol)
        .unwrap_or_else(|e| panic!("tier 3 refuses the doubled ball: {e:?}"));
    let got = topo::mass_properties(&body, tol).unwrap().volume;
    assert!(
        (got - 8.0 * PI / 3.0).abs() <= 1e-9,
        "the doubled ball measures {got}, pinned at twice the ball's 4π/3"
    );
}
