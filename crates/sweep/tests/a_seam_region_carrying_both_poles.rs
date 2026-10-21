//! A seam region whose boundary carries both chart poles encodes no
//! side. For a region of one face that is no defect: the flipped bit
//! describes the complement, another valid body. For a region of two
//! faces or more, all flipped, each claims its own complement, so the
//! faces overlap: tier 3's cover census refuses it, as claimed twice over
//! (`geom_brep::props::sphere_region_cover`).
//!
//! `mass_properties` runs no tier 3, and still measures the flip. That
//! residual is pinned here until the door reads the census itself
//! (`work/flux/mass-properties-measures-a-flipped-seam-region-without-tier-3.md`),
//! and goes red for the right reason then.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::seam_region_common::{flipped, sphere_faces, split_at_meridian, v};
use geom_core::Tol;
use std::f64::consts::PI;
use sweep::test_support::{ball_poled, brick, finished};
use topo::{Body, ValidationError};

fn close(got: f64, want: f64) -> bool {
    (got - want).abs() <= 1e-9 * want
}

fn measure(label: &str, body: &Body<f64>, tol: Tol) -> f64 {
    topo::validate_geometric(body, tol)
        .unwrap_or_else(|e| panic!("{label}: tier 3 refuses: {e:?}"));
    topo::mass_properties(body, tol).unwrap().volume
}

/// Tier 3 refuses `body` as a region claimed twice over, and nothing
/// else; the door alone measures it.
fn overcovered(label: &str, body: &Body<f64>, tol: Tol) -> f64 {
    let errs = topo::validate_geometric(body, tol).unwrap_err();
    assert!(
        matches!(
            errs.as_slice(),
            [ValidationError::SeamRegionOvercovered { times: 2, .. }]
        ),
        "{label}: refused as covered twice: {errs:?}"
    );
    topo::mass_properties(body, tol).unwrap().volume
}

/// A ball unioned with a brick whose face plane holds its polar axis
/// keeps one 180° lune; `mef` splits it along the meridian through
/// `(1, 0, 0)`. The two halves' region is bounded by the great circle
/// `x = 0`, which carries both poles. Both flipped, tier 3 refuses them
/// as covering their region twice; the door alone measures the brick
/// less a whole half-sphere's worth: 48 − 2π against 48 + 2π/3.
#[test]
fn two_faces_whose_region_carries_both_poles_flip_into_a_double_cover() {
    let tol = Tol::witness();
    let ball = finished(
        "ball",
        ball_poled(1.0, v([0.0; 3]), v([0.0, 1.0, 0.0]), tol),
        tol,
    );
    let block = finished(
        "block",
        brick((-3.0, 0.0), (-2.0, 2.0), (-2.0, 2.0), tol),
        tol,
    );
    let mut body = Body::clone(
        &topo::boolean::union(&ball, &block, tol)
            .unwrap()
            .body()
            .unwrap()
            .body,
    );
    let lune = sphere_faces(&body);
    assert_eq!(lune.len(), 1, "one lune is left of the ball");
    split_at_meridian(&mut body, lune[0], v([1.0, 0.0, 0.0]), tol);
    let faces = sphere_faces(&body);
    assert_eq!(faces.len(), 2, "the lune is split in two");
    let boss = measure("the split boss", &body, tol);
    assert!(
        close(boss, 48.0 + 2.0 * PI / 3.0),
        "the split boss measures {boss}"
    );
    let door = overcovered("both halves flipped", &flipped(&body, &faces), tol);
    assert!(
        close(door, 48.0 - 2.0 * PI),
        "both halves flipped measure {door} at the door, pinned at 48 − 2π"
    );
}

/// A brick less a ball whose polar axis lies 1e-11 off the brick's face
/// plane. At ε = 1e-12 the dimple is two faces of one sphere, its
/// boundary carries both poles within the band, and both flipped are
/// refused as covering it twice; the door alone measures 48 + 2π against
/// 48 − 2π/3. At a coarser ε the
/// plane holds the axis, the dimple is one lune, and its flip is the
/// boss, a valid body.
#[test]
fn a_dimple_beside_its_polar_axis_flips_by_the_run_band() {
    let tol = Tol::witness();
    let ball = finished(
        "ball",
        ball_poled(1.0, v([0.0; 3]), v([0.0, 0.6, 0.8]), tol),
        tol,
    );
    let block = finished(
        "block",
        brick((-3.0, 1e-11), (-2.0, 2.0), (-2.0, 2.0), tol),
        tol,
    );
    let body = Body::clone(
        &topo::boolean::subtract(&block, &ball, tol)
            .unwrap()
            .body()
            .unwrap()
            .body,
    );
    let half = 2.0 * PI / 3.0;
    let dimple = measure("the dimple", &body, tol);
    assert!(
        (dimple - (48.0 - half)).abs() <= 1e-9,
        "the dimple measures {dimple}"
    );
    let faces = sphere_faces(&body);
    let both = flipped(&body, &faces);
    match faces.len() {
        1 => {
            let got = measure("the lone lune flipped", &both, tol);
            assert!(
                (got - (48.0 + half)).abs() <= 1e-9,
                "the lone lune flipped is the boss, measures {got}"
            );
        }
        2 => {
            let got = overcovered("both halves flipped", &both, tol);
            assert!(
                (got - (48.0 + 2.0 * PI)).abs() <= 1e-9,
                "both halves flipped measure {got} at the door, pinned at 48 + 2π"
            );
        }
        n => panic!("the dimple is {n} faces"),
    }
}
