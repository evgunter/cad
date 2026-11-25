//! A seam region whose boundary carries both chart poles encodes no
//! side. For a region of one face that is no defect: the flipped bit
//! describes the complement, another valid body. For a region of two
//! faces or more, all flipped, each claims its own complement, so the
//! faces overlap: tier 3's cover census refuses it, as claimed twice over
//! (`geom_brep::props::sphere_region_cover`).
//!
//! `mass_properties` runs no tier 3, and reads the same census at the
//! door: it refuses the flip by the same finding, except where the
//! region's boundary encodes its side (the dimple below at ε = 1e-12),
//! when it reads the contradicted bit first (`SenseContradicted`).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::seam_region_common::{flipped, sphere_faces, split_at_meridian, v};
use geom_brep::props::PropsError;
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
    topo::test_support::pinned_volume(
        "the measured body",
        &topo::test_support::finished("the measured body", topo::Body::clone(body), tol),
        tol,
    )
    .volume
}

/// Tier 3 refuses `body` as a region claimed twice over, and nothing
/// else; so does the door, without tier 3.
fn overcovered(label: &str, body: &Body<f64>, tol: Tol) {
    let errs = topo::validate_geometric(body, tol).unwrap_err();
    assert!(
        matches!(
            errs.as_slice(),
            [ValidationError::SeamRegionOvercovered { times: 2, .. }]
        ),
        "{label}: refused as covered twice: {errs:?}"
    );
    // Raw door: the measured body is a seam region covered twice, which tier 3 refuses (SeamRegionOvercovered).
    let door = topo::test_support::measure_unvalidated(
        body,
        tol,
        topo::test_support::pinned_resolution(tol),
    );
    assert!(
        matches!(
            door,
            Err(topo::MassPropsError::Face {
                source: PropsError::SeamRegionOvercovered { times: 2 },
                ..
            })
        ),
        "{label}: the door refuses it as covered twice: {door:?}"
    );
}

/// A ball unioned with a brick whose face plane holds its polar axis
/// keeps one 180° lune; `mef` splits it along the meridian through
/// `(1, 0, 0)`. The two halves' region is bounded by the great circle
/// `x = 0`, which carries both poles. Both flipped, tier 3 and the door
/// refuse them as covering their region twice; the door once measured
/// the brick less a whole half-sphere's worth, 48 − 2π against
/// 48 + 2π/3.
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
        brick::<f64>((-3.0, 0.0), (-2.0, 2.0), (-2.0, 2.0), tol),
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
    overcovered("both halves flipped", &flipped(&body, &faces), tol);
}

/// A brick less a ball whose polar axis lies 2e-11 off the brick's face
/// plane: past the band's escalation at ε = 1e-12 (`K·ε`, `1e-11` at the
/// default K), so the
/// distance is decided there (in band, the boolean itself escalates and
/// no dimple is built), and inside the zero at any coarser ε.
///
/// Where the distance is decided, the dimple's boundary passes the
/// ball's poles 2e-11 off, so it encodes the dimple's side: flipped, the
/// bit contradicts it. The ball is one band, so the dimple is one face,
/// refused by tier 3 as `CurvedSenseInverted` and nothing else, and read
/// first by the door (`SenseContradicted`). Where the plane holds the axis, the
/// dimple's boundary runs through both poles and encodes no side: the
/// one face flipped is the boss, a valid body.
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
        brick::<f64>((-3.0, 2e-11), (-2.0, 2.0), (-2.0, 2.0), tol),
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
    // Decided past the run's escalation, K·ε (`Band::linear`).
    let decided = 2e-11 > tol.k() * tol.eps();
    match (faces.len(), decided) {
        (1, false) => {
            let got = measure("the lone lune flipped", &both, tol);
            assert!(
                (got - (48.0 + half)).abs() <= 1e-9,
                "the lone lune flipped is the boss, measures {got}"
            );
        }
        (1, true) => {
            let errs = topo::validate_geometric(&both, tol).unwrap_err();
            assert!(
                matches!(
                    errs.as_slice(),
                    [ValidationError::CurvedSenseInverted { face }] if *face == faces[0]
                ),
                "the flipped bit contradicts the side its boundary encodes, and nothing else: \
                 {errs:?}"
            );
            let door = topo::test_support::measure_unvalidated(
                &both,
                tol,
                topo::test_support::pinned_resolution(tol),
            );
            assert!(
                matches!(
                    door,
                    Err(topo::MassPropsError::Face {
                        source: PropsError::SenseContradicted,
                        ..
                    })
                ),
                "the door refuses the bit its region contradicts: {door:?}"
            );
        }
        (n, _) => panic!("the dimple is {n} faces"),
    }
}
