//! A z-poled ball's lune split by `mef` along a meridian OUTSIDE it: its
//! faces then cover the sphere twice, each holds both poles on its loop,
//! and their seam region has no boundary, so no side is read for it.
//! Tier 3's cover census refuses it: the three faces claim the sphere
//! twice over (`geom_brep::props::sphere_region_cover`).
//!
//! `mass_properties` runs no tier 3, and still measures the body as
//! twice the ball. That residual is pinned here until the door reads
//! the census itself
//! (`work/flux/mass-properties-measures-a-flipped-seam-region-without-tier-3.md`),
//! and goes red for the right reason then.
#![allow(clippy::unwrap_used, clippy::panic)]

use crate::seam_region_common::{sphere_faces, split_at_meridian, v};
use geom_core::Tol;
use std::f64::consts::PI;
use sweep::test_support::{ball_poled, finished};
use topo::{Body, ValidationError};

#[test]
fn an_exterior_meridian_covers_the_sphere_twice_and_is_refused() {
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
    let errs = topo::validate_geometric(&body, tol).unwrap_err();
    assert!(
        matches!(
            errs.as_slice(),
            [ValidationError::SeamRegionOvercovered { times: 2, .. }]
        ),
        "the doubled ball is refused as covered twice: {errs:?}"
    );
    let got = topo::mass_properties(&body, tol).unwrap().volume;
    assert!(
        (got - 8.0 * PI / 3.0).abs() <= 1e-9,
        "the door alone measures {got}, pinned at twice the ball's 4π/3"
    );
}
