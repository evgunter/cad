//! A z-poled ball's lune split by `mef` along a meridian OUTSIDE it: its
//! faces then cover the sphere twice, each holds both poles on its loop,
//! and their seam region has no boundary, so no side is read for it.
//! Tier 3's cover census refuses it: the three faces claim the sphere
//! twice (`geom_brep::props::sphere_region_cover`). The census reads
//! every seam region, whatever its members' own boundaries encode, so
//! the same cover split until each face holds its own side is refused
//! too; and so is a ball of three lunes or more turned inside out.
//!
//! `mass_properties` runs no tier 3, and reads the same census at the
//! door: it refuses the body by the same finding.
#![allow(clippy::unwrap_used, clippy::panic)]

use crate::seam_region_common::{
    flipped, sphere_faces, split_at_meridian, split_faces_at_equator, split_inside,
    split_meridians_at_equator, split_outside, v,
};
use geom_brep::props::PropsError;
use geom_core::Tol;
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
    // Raw door: the measured body is a seam region covered twice, which tier 3 refuses (SeamRegionOvercovered).
    let door = topo::test_support::measure_unvalidated(
        &body,
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
        "the door refuses the doubled ball as covered twice: {door:?}"
    );
}

/// The double cover with every meridian split at the equator, and every
/// face split along it: six faces, each touching one pole, so each reads
/// its own side and none encodes nothing alone. Its seam region is read
/// all the same, and refused as covered twice. The valid ball split so
/// passes (review of PR 5516, R2's `p2`).
#[test]
fn a_double_cover_whose_faces_each_read_their_own_side_is_refused() {
    let tol = Tol::witness();
    let ball = finished(
        "ball",
        ball_poled(1.0, v([0.0; 3]), v([0.0, 0.0, 1.0]), tol),
        tol,
    );
    let mut control = Body::clone(&ball);
    split_meridians_at_equator(&mut control, tol);
    split_faces_at_equator(&mut control, tol);
    topo::validate_geometric(&control, tol)
        .unwrap_or_else(|e| panic!("the ball split at its equator is valid: {e:?}"));
    let got = topo::test_support::pinned_volume(
        "the measured body",
        &topo::test_support::finished("the measured body", topo::Body::clone(&control), tol),
        tol,
    )
    .volume;
    assert!(
        (got - 4.0 * std::f64::consts::PI / 3.0).abs() <= 1e-9,
        "the split ball measures {got}"
    );
    let mut body = Body::clone(&ball);
    assert!(split_outside(&mut body, v([0.0, 1.0, 0.0]), tol));
    split_meridians_at_equator(&mut body, tol);
    split_faces_at_equator(&mut body, tol);
    assert_eq!(
        sphere_faces(&body).len(),
        6,
        "each lune split at the equator"
    );
    let errs = topo::validate_geometric(&body, tol).unwrap_err();
    assert!(
        matches!(
            errs.as_slice(),
            [ValidationError::SeamRegionOvercovered { times: 2, .. }]
        ),
        "the split double cover is refused as covered twice: {errs:?}"
    );
    // Raw door: the measured body is a seam region covered twice, which tier 3 refuses (SeamRegionOvercovered).
    let door = topo::test_support::measure_unvalidated(
        &body,
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
        "the door refuses it by the census too: {door:?}"
    );
}

/// A z-poled ball cut into `n` lunes by meridians inside it, turned
/// inside out by flipping every bit: each lune claims its complement, so
/// the members claim the sphere `n − 1` times (`Σ χ_f − χ_R + 1`). Two
/// lunes claim it once, an inside-out ball the volume checks refuse;
/// three or more are refused as an over-cover (review of PR 5516, R1).
#[test]
fn a_ball_of_three_lunes_or_more_turned_inside_out_is_an_over_cover() {
    let tol = Tol::witness();
    for n in [2usize, 3, 5] {
        let mut body = Body::clone(&finished(
            "ball",
            ball_poled(1.0, v([0.0; 3]), v([0.0, 0.0, 1.0]), tol),
            tol,
        ));
        let mut k = 0;
        while sphere_faces(&body).len() < n {
            let a = 0.1 + std::f64::consts::TAU * f64::from(k) / 7.0;
            split_inside(&mut body, v([a.cos(), a.sin(), 0.0]), tol);
            k += 1;
            assert!(k < 64, "the ball splits into {n} lunes");
        }
        topo::validate_geometric(&body, tol)
            .unwrap_or_else(|e| panic!("{n} lunes: the split ball is valid: {e:?}"));
        let errs =
            topo::validate_geometric(&flipped(&body, &sphere_faces(&body)), tol).unwrap_err();
        if n == 2 {
            assert!(
                matches!(errs.as_slice(), [ValidationError::NegativeVolume { .. }]),
                "two lunes inside out are a negative volume: {errs:?}"
            );
        } else {
            assert!(
                matches!(
                    errs.as_slice(),
                    [ValidationError::SeamRegionOvercovered { times, .. }] if *times == n - 1
                ),
                "{n} lunes inside out claim the sphere {} times: {errs:?}",
                n - 1
            );
        }
    }
}
