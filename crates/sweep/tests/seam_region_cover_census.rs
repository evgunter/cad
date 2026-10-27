//! The cover census of a sphere seam region
//! (`geom_brep::props::sphere_region_cover`) leaves two flips unread.
//! Each is refused by another check of tier 3, on the very body, and
//! the census adds nothing to it.
//!
//! `mass_properties` runs neither check, and settles both itself: the
//! lamina off the seam region (`PropsError::SeamRegionSensesDisagree`),
//! and the ringed member's flip off that member's own boundary, as its
//! flux arm reads it.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::seam_region_common::{flipped, seam_region, sphere_faces, split_at_meridian, v};
use geom_brep::props::PropsError;
use geom_core::Tol;
use sweep::test_support::{ball_poled, brick, finished};
use topo::{Body, ValidationError};

fn census(e: &ValidationError) -> bool {
    matches!(
        e,
        ValidationError::SeamRegionOvercovered { .. }
            | ValidationError::SeamRegionCoverUnresolved { .. }
    )
}

/// Members whose bits disagree are no cover, so the census does not
/// read them: one half of a both-poles region flipped meets the other
/// across their seam with the material on one side of it, a lamina that
/// check 4 refuses there, and alone.
#[test]
fn one_flipped_sibling_is_check_4s_lamina_and_not_the_census() {
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
    split_at_meridian(&mut body, lune[0], v([1.0, 0.0, 0.0]), tol);
    let faces = sphere_faces(&body);
    assert_eq!(faces.len(), 2, "the lune is split in two");
    for f in faces {
        let errs = topo::validate_geometric(&flipped(&body, &[f]), tol).unwrap_err();
        assert!(
            matches!(errs.as_slice(), [ValidationError::LaminaWedge { .. }]),
            "{f:?} flipped alone is check 4's lamina, and only that: {errs:?}"
        );
        // Raw door: the measured body is a lamina (a flipped band), which tier 3 refuses (LaminaWedge).
        let door = topo::test_support::measure_unvalidated(
            &flipped(&body, &[f]),
            tol,
            topo::test_support::pinned_resolution(tol),
        );
        assert!(
            matches!(
                door,
                Err(topo::MassPropsError::Face {
                    source: PropsError::SeamRegionSensesDisagree,
                    ..
                })
            ),
            "{f:?} flipped alone: the door refuses the lamina off the region: {door:?}"
        );
    }
}

/// A ringed member whose bit flips claims a negative "area", and signed
/// Gauss–Bonnet area is additive, so the census reads its region once.
/// The member's own boundary encodes its side by the sign of its area,
/// and check 6 names it from that read: a bar through a ball leaves the
/// ball's lune holding both rings, and its seam sibling.
#[test]
fn a_flipped_ringed_member_is_named_by_its_own_read_and_not_the_census() {
    let tol = Tol::witness();
    let ball = finished(
        "ball",
        ball_poled(1.0, v([0.0; 3]), v([0.0, 1.0, 0.0]), tol),
        tol,
    );
    let bar = finished(
        "bar",
        brick((-2.0, 2.0), (-0.2, 0.25), (0.1, 0.4), tol),
        tol,
    );
    let body = Body::clone(
        &topo::boolean::subtract(&ball, &bar, tol)
            .unwrap()
            .body()
            .unwrap()
            .body,
    );
    let faces = sphere_faces(&body);
    let ringed: Vec<_> = faces
        .iter()
        .copied()
        .filter(|&f| {
            !body
                .faces()
                .find(|(k, _)| *k == f)
                .unwrap()
                .1
                .rings
                .is_empty()
        })
        .collect();
    assert_eq!(faces.len(), 2, "the ball is two faces: {faces:?}");
    assert_eq!(ringed.len(), 1, "one of them holds the rings");
    let (members, bounded) = seam_region(&body, faces[0]);
    assert!(
        members.len() == 2 && bounded,
        "one region of both, with a boundary"
    );
    let errs = topo::validate_geometric(&flipped(&body, &faces), tol).unwrap_err();
    assert!(
        errs.iter().any(|e| matches!(
            e,
            ValidationError::CurvedSenseInverted { face } if *face == ringed[0]
        )),
        "the ringed member is named by its own read: {errs:?}"
    );
    assert!(
        !errs.iter().any(census),
        "the census reads the region once: {errs:?}"
    );
    // Raw door: the measured body is a body with a curved face flipped, which tier 3 refuses (CurvedSenseInverted).
    let door = topo::test_support::measure_unvalidated(
        &flipped(&body, &faces),
        tol,
        topo::test_support::pinned_resolution(tol),
    );
    assert!(
        matches!(
            door,
            Err(topo::MassPropsError::Face {
                face,
                source: PropsError::SenseInverted | PropsError::SenseContradicted,
            }) if faces.contains(&face)
        ),
        "the door refuses the flip by a member's own side: {door:?}"
    );
}
