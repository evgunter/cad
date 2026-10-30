//! A seam region whose boundary carries both chart poles encodes no
//! side. For a region of one face that is no defect: the flipped bit
//! describes the complement, another valid body. For a region of two
//! faces or more, all flipped, each claims its own complement, so the
//! faces overlap: tier 3's cover census refuses it, as claimed twice over
//! (`geom_brep::props::sphere_region_cover`).
//!
//! `mass_properties` runs no tier 3, and reads the same census at the
//! door: it refuses the flip by the same finding.
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
    overcovered("both halves flipped", &flipped(&body, &faces), tol);
}

/// A brick less a ball whose polar axis lies 1e-11 off the brick's face
/// plane. At ε = 1e-12 the dimple is two faces of one sphere, each with
/// both poles at its vertices, so neither encodes a side alone; their
/// region's boundary passes the poles at the band's escalation, a
/// decided distance, so a pole on the seam between them reads the
/// region's side. Both flipped are refused as covering it twice by tier
/// 3 (the door once measured 48 + 2π against 48 − 2π/3), and each
/// flipped bit against the side its region's
/// boundary encodes (`CurvedSenseInverted`), which the door reads first
/// (`SenseContradicted`). At a coarser ε the plane
/// holds the axis, the dimple is one lune, and its flip is the boss, a
/// valid body.
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
            let errs = topo::validate_geometric(&both, tol).unwrap_err();
            let mut inverted: Vec<topo::FaceKey> = errs
                .iter()
                .filter_map(|e| match e {
                    ValidationError::CurvedSenseInverted { face } => Some(*face),
                    _ => None,
                })
                .collect();
            inverted.sort();
            let mut want = faces.clone();
            want.sort();
            assert_eq!(
                inverted, want,
                "each flipped bit contradicts its region: {errs:?}"
            );
            assert!(
                errs.iter().all(|e| matches!(
                    e,
                    ValidationError::SeamRegionOvercovered { times: 2, .. }
                        | ValidationError::CurvedSenseInverted { .. }
                )) && errs
                    .iter()
                    .any(|e| matches!(e, ValidationError::SeamRegionOvercovered { times: 2, .. })),
                "refused as covered twice and inverted, and nothing else: {errs:?}"
            );
            let door = topo::test_support::measure_unvalidated(
                &both,
                tol,
                topo::test_support::pinned_resolution(tol),
            );
            // The door reads the contradicted bit before the census.
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
        n => panic!("the dimple is {n} faces"),
    }
}
