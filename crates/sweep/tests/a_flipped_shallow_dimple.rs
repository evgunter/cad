//! **A flipped dimple is refused by the sphere's side reader, and by
//! nothing else** — the witness for why `sphere_loop_side` reads a
//! one-loop face's side off its chart poles.
//!
//! A box less a shallow ball leaves a dimple: one sphere face bounded
//! by one circle, the section of the box's top face. The ball is
//! `y`-poled with its seam on `+x`, both in the plane `z = 1.3`, so the
//! dimple below `z = 1` holds no pole and misses the seam. Its edge
//! meets the planar top face transversally, so tier 3's check 4, which
//! reads material pairing only across `Smooth` edges, cannot see a flip
//! of the dimple's sense bit; and the sphere's flux reads that bit, so
//! a flip left unread would measure the box less the ball's COMPLEMENT,
//! a wrong positive volume. The dimple's loop holds no pole on the
//! side the face lies on, so the reader encodes its side, and both
//! tier 3's check 6 and the mass properties refuse the flip by name.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Tol, Vec3};
use sweep::test_support::{ball_poled_y, brick, finished};
use topo::AtRestBody;

use crate::common::oracles::cap_volume;

/// The ball's radius and centre: it dips `0.3` below the slab's top.
const R: f64 = 0.6;
const C: (f64, f64, f64) = (2.0, 2.0, 1.3);

fn slab() -> AtRestBody<f64> {
    let tol = Tol::witness();
    finished("the slab", brick((0.0, 4.0), (0.0, 4.0), (0.0, 1.0), tol), tol)
}

fn ball() -> AtRestBody<f64> {
    let tol = Tol::witness();
    let at = ball_poled_y(R, Vec3::new(0.0, 0.0, 0.0), tol);
    let moved =
        topo::transform_rigid(&at, &Affine3::translation(Vec3::new(C.0, C.1, C.2)), tol).unwrap();
    finished("the ball", moved, tol)
}

#[test]
fn a_flipped_dimple_is_refused_by_the_side_reader() {
    let tol = Tol::witness();
    let none = topo::BooleanDeclarations::none();
    let body = topo::subtract_with(&slab(), &ball(), &none, tol)
        .expect("the slab less the ball builds")
        .body()
        .expect("a body")
        .body
        .clone();
    topo::validate_geometric(&body, tol).expect("the dimpled slab is valid");
    let volume = 16.0 - cap_volume(R, 1.0 - (C.2 - R));
    let v = topo::mass_properties(&body, tol).unwrap().volume;
    assert!(
        (v - volume).abs() <= 1e-9 * volume,
        "volume {v} against the closed form {volume}"
    );
    let dimples: Vec<_> = body
        .faces()
        .filter(|(_, f)| {
            matches!(
                body.get_surface(f.surface),
                Some(geom::Surface::Sphere { .. })
            )
        })
        .map(|(k, _)| k)
        .collect();
    assert_eq!(dimples.len(), 1, "one dimple face");
    let dimple = dimples[0];
    let flipped = body.flipped_face_sense_for_tests(dimple).expect("the face");
    assert_eq!(
        topo::validate_geometric(&flipped, tol),
        Err(vec![topo::ValidationError::CurvedSenseInverted { face: dimple }]),
        "check 6 names the flipped dimple"
    );
    assert!(
        matches!(
            topo::mass_properties(&flipped, tol),
            Err(topo::MassPropsError::Face {
                face,
                source: geom_brep::props::PropsError::SenseContradicted,
            }) if face == dimple
        ),
        "the flipped dimple is refused, not measured as the complement"
    );
}
