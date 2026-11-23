//! **A flipped dimple is refused by the sphere's side reader, and by
//! nothing else** — the witness for why `sphere_loop_side` reads a
//! one-loop face's side off its chart poles.
//!
//! A box less a shallow ball leaves a dimple: one sphere face bounded by
//! the section circle alone, the ball's chart poles both off it (the
//! meridian the boolean cut the ball along to meet the circle is a cut
//! of the chart, not an edge, and does not survive). The section circle
//! meets the planar top face transversally, so tier 3's check 4, which
//! reads material pairing only across `Smooth` edges, sees nothing of a
//! flipped sense; and the sphere's flux reads the bit, so a flip left
//! unread would measure the box less the ball's COMPLEMENT, a wrong
//! positive volume. Flipped, the face claims the complement of the cap,
//! which holds both poles, so the reader encodes its side, and both tier
//! 3's check 6 and the mass properties refuse the flip by name.
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
        .filter(|(fk, _)| matches!(body.face_surface(*fk), Some(geom::Surface::Sphere { .. })))
        .map(|(k, _)| k)
        .collect();
    let [dimple] = dimple[..] else {
        panic!("the dimple is one face, got {dimple:?}");
    };
    let flipped = body
        .flipped_face_sense_for_tests(dimple, geom_core::Tol::witness())
        .expect("the face");
    assert_eq!(
        topo::validate_geometric(&flipped, tol),
        Err(
            core::iter::once(topo::ValidationError::CurvedSenseInverted { face: dimple })
                .chain(
                    [
                        geom_brep::chart_singularity::Singularity::North,
                        geom_brep::chart_singularity::Singularity::South,
                    ]
                    .map(|point| topo::ValidationError::ChartSingularityNotAVertex {
                        face: dimple,
                        point,
                        lies: topo::SingularityLies::Interior,
                    })
                )
                .collect()
        ),
        "check 6 names the flipped face and both poles its complement holds, and nothing \
         else does"
    );
    assert!(
        matches!(
            // Raw door: tier 3 refuses `flipped` (above).
            topo::test_support::measure_unvalidated(&flipped, tol, topo::test_support::pinned_resolution(tol)),
            Err(topo::MassPropsError::Face {
                face,
                source: geom_brep::props::PropsError::SenseContradicted,
            }) if face == dimple
        ),
        "the flipped dimple is refused, not measured as the complement"
    );
}
