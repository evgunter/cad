//! **A flipped dimple is refused by the sphere's side reader, and by
//! nothing else** — the witness for why `sphere_loop_side` reads a
//! one-loop face's side off its chart poles.
//!
//! A box less a shallow ball leaves a dimple. The boolean charts the
//! ball with its pole on the cut plane's normal, so the dimple is two
//! sphere faces meeting along a seam at the pole, each bounded by half
//! the section circle and the seam. Flipping the sense of BOTH faces
//! keeps the seam consistent, and the section circle meets the planar
//! top face transversally, so tier 3's check 4, which reads material
//! pairing only across `Smooth` edges, sees nothing; and the sphere's
//! flux reads the bit, so a flip left unread would measure the box less
//! the ball's COMPLEMENT, a wrong positive volume. Each face has the
//! pole on its loop and the far pole off it, on the side the face does
//! not lie on, so the reader encodes its side, and both tier 3's check
//! 6 and the mass properties refuse the flip by name.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Tol, Vec3};
use sweep::test_support::{ball_poled_y, brick, finished};
use topo::AtRestBody;

use crate::common::oracles::cap_volume;

/// The ball's radius and centre: it dips `0.3` below the slab's top.
const R: f64 = 0.6;
const C: (f64, f64, f64) = (2.0, 2.0, 1.3);

fn slab() -> AtRestBody<f64> {
    let tol = Tol::witness();
    finished(
        "the slab",
        brick((0.0, 4.0), (0.0, 4.0), (0.0, 1.0), tol),
        tol,
    )
}

fn ball() -> AtRestBody<f64> {
    let tol = Tol::witness();
    let at = ball_poled_y(R, Vec3::new(0.0, 0.0, 0.0), tol);
    let moved = topo::transform_rigid(
        &at,
        &geom_core::RigidMotion::translation(Vec3::new(C.0, C.1, C.2)).unwrap(),
        tol,
    )
    .unwrap();
    finished("the ball", moved, tol)
}

#[test]
fn a_flipped_dimple_is_refused_by_the_side_reader() {
    let tol = Tol::witness();
    let body = topo::subtract(&slab(), &ball(), tol)
        .expect("the slab less the ball builds")
        .body()
        .expect("a body")
        .body
        .clone();
    topo::validate_geometric(&body, tol).expect("the dimpled slab is valid");
    let volume = 16.0 - cap_volume(R, 1.0 - (C.2 - R));
    let v = topo::test_support::pinned_volume("the measured body", &body, tol).volume;
    assert!(
        (v - volume).abs() <= 1e-9 * volume,
        "volume {v} against the closed form {volume}"
    );
    let dimple: Vec<_> = body
        .faces()
        .filter(|(_, f)| {
            matches!(
                body.get_surface(f.surface),
                Some(geom::Surface::Sphere { .. })
            )
        })
        .map(|(k, _)| k)
        .collect();
    assert_eq!(dimple.len(), 2, "the dimple is two half-faces on one seam");
    let flipped = dimple.iter().fold(topo::Body::clone(&body), |b, &k| {
        b.flipped_face_sense_for_tests(k).expect("the face")
    });
    // Each flipped half claims its complement, which holds the other, so
    // the cover census refuses their region as claimed twice, named by
    // its first member, before the face reads name each flipped half;
    // that complement holds the north pole too, which the
    // chart-singularity arm names after them.
    assert_eq!(
        topo::validate_geometric(&flipped, tol),
        Err(
            core::iter::once(topo::ValidationError::SeamRegionOvercovered {
                face: dimple[0],
                times: 2,
            })
            .chain(
                dimple
                    .iter()
                    .map(|&face| topo::ValidationError::CurvedSenseInverted { face })
            )
            .chain(dimple.iter().map(|&face| {
                topo::ValidationError::ChartSingularityNotAVertex {
                    face,
                    point: geom_brep::chart_singularity::Singularity::North,
                    lies: topo::SingularityLies::Interior,
                }
            }))
            .collect()
        ),
        "check 6 names the overlap, both flipped faces and the pole each holds, and nothing \
         else does"
    );
    assert!(
        matches!(
            // Raw door: `flipped` is a seam region covered twice, which tier 3 refuses (SeamRegionOvercovered).
            topo::test_support::measure_unvalidated(&flipped, tol, topo::test_support::pinned_resolution(tol)),
            Err(topo::MassPropsError::Face {
                face,
                source: geom_brep::props::PropsError::SenseContradicted,
            }) if dimple.contains(&face)
        ),
        "the flipped dimple is refused, not measured as the complement"
    );
}
