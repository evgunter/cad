//! **A flipped dimple is refused by the sphere's side reader, and by
//! nothing else** — the witness for why `sphere_loop_side` reads a
//! one-loop face's side off its chart poles.
//!
//! A box less a shallow ball leaves a dimple. The boolean charts the
//! ball with its pole on the cut plane's normal, so the dimple is one
//! sphere face bounded by the section circle and the seam, walked out
//! to the pole and back. Flipping its sense keeps the seam consistent,
//! and the section circle meets the planar
//! top face transversally, so tier 3's check 4, which reads material
//! pairing only across `Smooth` edges, sees nothing; and the sphere's
//! flux reads the bit, so a flip left unread would measure the box less
//! the ball's COMPLEMENT, a wrong positive volume. The face has the
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
        brick::<f64>((0.0, 4.0), (0.0, 4.0), (0.0, 1.0), tol),
        tol,
    )
}

fn ball() -> AtRestBody<f64> {
    let tol = Tol::witness();
    let at = ball_poled_y(R, Vec3::new(0.0, 0.0, 0.0), tol);
    let moved = topo::transform_rigid(
        &at,
        &geom_core::RigidMotion::translation(Vec3::new(C.0, C.1, C.2)).unwrap(),
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
        .filter(|(fk, _)| matches!(body.face_surface(*fk), Ok(geom::Surface::Sphere { .. })))
        .map(|(k, _)| k)
        .collect();
    assert_eq!(dimple.len(), 1, "the dimple is one face on its seam");
    let flipped = dimple.iter().fold(topo::Body::clone(&body), |b, &k| {
        b.flipped_face_sense_for_tests(k, geom_core::Tol::witness())
            .expect("the face")
    });
    // The flipped face claims its complement once, so no seam region is
    // covered twice: its own read names it, the chart-singularity arm
    // names the pole the complement holds, and its loop's lift, read on
    // the flipped side, does not close round the pole its seam reaches.
    let [face] = dimple[..] else {
        unreachable!("one dimple face")
    };
    assert_eq!(
        topo::validate_geometric(&flipped, tol),
        Err(vec![
            topo::ValidationError::CurvedSenseInverted { face },
            topo::ValidationError::ChartSingularityNotAVertex {
                face,
                point: geom_brep::chart_singularity::Singularity::North,
                lies: topo::SingularityLies::Interior,
            },
            topo::ValidationError::Pcurve {
                finding: topo::pcurves::PcurveMintError::LoopNotClosed { face },
            },
        ]),
        "check 6 names the flipped face and the pole it holds, and the pcurve pass its open \
         lift; nothing else does"
    );
    assert!(
        matches!(
            // Raw door: `flipped` is a face whose sense contradicts its loop, which tier 3 refuses (CurvedSenseInverted).
            topo::test_support::measure_unvalidated(&flipped, tol, topo::test_support::pinned_resolution(tol)),
            Err(topo::MassPropsError::Face {
                face,
                source: geom_brep::props::PropsError::SenseContradicted,
            }) if dimple.contains(&face)
        ),
        "the flipped dimple is refused, not measured as the complement"
    );
}
