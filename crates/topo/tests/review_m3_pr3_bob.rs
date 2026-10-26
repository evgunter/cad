//! PR 3 adversarial review — target 1 (BOB pinch): the touching-wedge
//! fixture under both plane orientations. At the Λ tip the section's
//! two corners each pair one Below wedge's germs, so the reduction keeps
//! one vertex per wedge on the wedges' side and one for the tip edge and
//! the cap between them on the other (`splitting::insert`). Every cell
//! of the orientation table succeeds with the same physical
//! decomposition; only the side assignment swaps with the normal (the
//! table test below).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::prism;
use geom_core::Tol;
use geom_core::{Point3, Vec3};
use topo::{Body, SplitPart, SplitPlane, mass_properties, split, validate_closed};

/// The acceptance suite's touching-wedge fixture, verbatim.
const MIRRORED: &[(f64, f64)] = &[
    (0.0, 0.0),
    (3.0, 0.0),
    (4.0, 1.0),
    (5.0, 0.0),
    (6.0, 1.0),
    (7.0, 1.0),
    (8.0, 0.0),
    (8.0, 2.0),
    (0.0, 2.0),
];

/// The acceptance suite's notched-block fixture, verbatim.
const NOTCHED: &[(f64, f64)] = &[
    (0.0, 0.0),
    (8.0, 0.0),
    (8.0, 2.0),
    (7.0, 1.0),
    (6.0, 1.0),
    (5.0, 2.0),
    (4.0, 1.0),
    (3.0, 2.0),
    (0.0, 2.0),
];

fn plane(c: f64, ny: f64) -> SplitPlane<f64> {
    topo::test_support::split_plane(
        Point3::new(0.0, c, 0.0),
        Vec3::new(0.0, ny, 0.0),
        geom_core::Tol::witness(),
    )
}

fn body_of<T: geom_core::Real>(part: &SplitPart<T>) -> &Body<T> {
    part.body().expect("side has material")
}

/// Bitwise vertex lookup.
fn vertices_at(body: &Body<f64>, x: f64, y: f64, z: f64) -> usize {
    body.vertex_points()
        .filter(|(_, p)| {
            let p = *p;
            p.x == x && p.y == y && p.z == z
        })
        .count()
}

/// The touching-wedge fixture split by (o, −n): the physical
/// decomposition of MIRRORED(+n) — slab + three floor pieces pinched at
/// the (4,1) tip line — presented so the pinched pieces are the ABOVE
/// side. The pieces land as three disconnected shells with
/// coincident-but-distinct tip copies.
#[test]
fn mirrored_fixture_flipped_plane_succeeds() {
    let fx = prism::<f64>(MIRRORED, 1.0, Tol::witness());
    let operand = topo::test_support::finished("the fixture", fx.body.clone(), Tol::witness());
    let result = split(&operand, &plane(1.0, -1.0), Tol::witness()).unwrap();
    // "above" w.r.t. −n = the y < 1 material: the three floor pieces.
    let (pieces, slab) = (body_of(&result.above), body_of(&result.below));
    assert_eq!(validate_closed(pieces), Ok(()));
    assert_eq!(validate_closed(slab), Ok(()));
    assert_eq!(pieces.shells().count(), 3, "three disconnected pieces");
    assert_eq!(slab.shells().count(), 1);
    // Tip copies: two coincident-but-distinct vertices per tip
    // position on the pieces side, none on the slab side, whose top at
    // y = 1 is one maximal face.
    for z in [0.0, 1.0] {
        assert_eq!(vertices_at(pieces, 4.0, 1.0, z), 2);
        assert_eq!(vertices_at(slab, 4.0, 1.0, z), 0);
    }
    // Volume conservation.
    let (vp, vs, v0) = (
        mass_properties(
            &topo::test_support::finished("the measured body", (pieces).clone(), Tol::witness()),
            Tol::witness(),
            topo::test_support::pinned_resolution(Tol::witness()),
        )
        .unwrap()
        .met()
        .expect("the pinned resolution is met")
        .volume,
        mass_properties(
            &topo::test_support::finished("the measured body", (slab).clone(), Tol::witness()),
            Tol::witness(),
            topo::test_support::pinned_resolution(Tol::witness()),
        )
        .unwrap()
        .met()
        .expect("the pinned resolution is met")
        .volume,
        mass_properties(
            &topo::test_support::finished("the measured body", fx.body.clone(), Tol::witness()),
            Tol::witness(),
            topo::test_support::pinned_resolution(Tol::witness()),
        )
        .unwrap()
        .met()
        .expect("the pinned resolution is met")
        .volume,
    );
    assert!((vp + vs - v0).abs() <= 1e-12 * v0, "{vp} + {vs} vs {v0}");
}

/// The orientation table: all four cells of the 2×2 succeed with
/// identical physical decompositions —
///   MIRRORED(+n) succeed | MIRRORED(−n) succeed
///   NOTCHED(−n)  succeed | NOTCHED(+n)  succeed
/// — the cells whose pinch lies below the normal (left column) get
/// their below copies from the reduction itself, so op success is
/// orientation-invariant and only the piece assignment swaps with the
/// normal (PR 2's equivariance). Equal-volume oracle across every cell.
#[test]
fn notched_fixture_orientation_table() {
    for (profile, pinched_above_under_plus) in [(NOTCHED, true), (MIRRORED, false)] {
        let fx = prism::<f64>(profile, 1.0, Tol::witness());
        let operand = topo::test_support::finished("the fixture", fx.body.clone(), Tol::witness());
        let v0 = mass_properties(
            &topo::test_support::finished("the measured body", fx.body.clone(), Tol::witness()),
            Tol::witness(),
            topo::test_support::pinned_resolution(Tol::witness()),
        )
        .unwrap()
        .met()
        .expect("the pinned resolution is met")
        .volume;
        for ny in [1.0, -1.0] {
            let r = split(&operand, &plane(1.0, ny), Tol::witness()).unwrap();
            let (a, b) = (body_of(&r.above), body_of(&r.below));
            assert_eq!(validate_closed(a), Ok(()));
            assert_eq!(validate_closed(b), Ok(()));
            // The pinched triple lands above exactly when the normal
            // points at it; the slab is always the other side.
            let pinched = if (ny > 0.0) == pinched_above_under_plus {
                a
            } else {
                b
            };
            assert_eq!(pinched.shells().count(), 3, "{profile:?} ny={ny}");
            let (va, vb) = (
                mass_properties(
                    &topo::test_support::finished("the measured body", (a).clone(), Tol::witness()),
                    Tol::witness(),
                    topo::test_support::pinned_resolution(Tol::witness()),
                )
                .unwrap()
                .met()
                .expect("the pinned resolution is met")
                .volume,
                mass_properties(
                    &topo::test_support::finished("the measured body", (b).clone(), Tol::witness()),
                    Tol::witness(),
                    topo::test_support::pinned_resolution(Tol::witness()),
                )
                .unwrap()
                .met()
                .expect("the pinned resolution is met")
                .volume,
            );
            assert!((va + vb - v0).abs() <= 1e-12 * v0, "{va} + {vb} vs {v0}");
        }
    }
}
