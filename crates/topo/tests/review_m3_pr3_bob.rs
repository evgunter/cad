//! PR 3 adversarial review — target 1 (BOB pinch): the reviewer's
//! independent derivation of the impossibility claim, plus the probe
//! the acceptance suite does NOT run: the SAME physical configuration
//! under the FLIPPED plane. Derivation (reviewer, independent):
//!
//! At a BOB tip vertex the ABOVE runs are {tip edge} and {wide-cap
//! bisector}; their null halves land in (slantL, slantR) and
//! (cap, cap). The join's neighbor criterion (same face + opposite
//! sense) is NOT a tunable pairing heuristic — connecting chords are
//! minted by `mef` INSIDE one face loop, so cross-face pairing is
//! structurally impossible in the half-edge calculus, and a slant
//! face touches the plane only along the tip line, so its exactly two
//! null halves MUST pair with each other: the zero-area 2-gon is
//! forced regardless of lex order. CONCUR with the refusal.
//!
//! The refusal WAS orientation-DEPENDENT: under the flipped plane the
//! tip context reads AOA, the runs become {slantL} / {slantR}, each
//! null edge bridges slant↔cap, and the sections are buildable — the
//! pinched pieces get their distinct copies because they are now the
//! ABOVE side (ch. 14 mints copies for ABOVE runs only). That
//! asymmetry is exactly what M3 PR 6a's D7 pinch lane consumes:
//! `split` reruns a `DegenerateSection` refusal under the mirrored
//! plane and swaps the sides back, so every cell of the orientation
//! table now succeeds (the table test below).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

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
    SplitPlane {
        origin: Point3::new(0.0, c, 0.0),
        normal: Vec3::new(0.0, ny, 0.0),
    }
}

fn body_of<T: geom_core::Real>(part: &SplitPart<T>) -> &Body<T> {
    part.body().expect("side has material")
}

/// Bitwise vertex lookup.
fn vertices_at(body: &Body<f64>, x: f64, y: f64, z: f64) -> usize {
    body.vertices()
        .filter(|(_, v)| {
            let p = *body.get_point(v.point).unwrap();
            p.x == x && p.y == y && p.z == z
        })
        .count()
}

/// THE MISSING PROBE: the touching-wedge fixture split by (o, −n).
/// The physical decomposition is identical to MIRRORED(+n) — slab +
/// three floor pieces pinched at the (4,1) tip line — but presented
/// so the pinched pieces are the ABOVE side, whose runs DO get vertex
/// copies. Predicted (reviewer derivation): SUCCESS, with the pieces
/// landing as three disconnected shells with coincident-but-distinct
/// tip copies — i.e., `split(S, n)` refuses while
/// `swap(split(S, −n))` returns the pieces. The refusal is NOT
/// invariant under plane orientation; only the piece-ASSIGNMENT is.
#[test]
fn mirrored_fixture_flipped_plane_succeeds() {
    let fx = prism::<f64>(MIRRORED, 1.0);
    let result = split(&fx.body, &plane(1.0, -1.0), Tol::witness()).unwrap();
    // "above" w.r.t. −n = the y < 1 material: the three floor pieces.
    let (pieces, slab) = (body_of(&result.above), body_of(&result.below));
    assert_eq!(validate_closed(pieces), Ok(()));
    assert_eq!(validate_closed(slab), Ok(()));
    assert_eq!(pieces.shells().count(), 3, "three disconnected pieces");
    assert_eq!(slab.shells().count(), 1);
    // Tip copies: two coincident-but-distinct vertices per tip
    // position on the pieces side, ONE on the slab side (the old
    // vertex, keeping the tip edge as an artifact).
    for z in [0.0, 1.0] {
        assert_eq!(vertices_at(pieces, 4.0, 1.0, z), 2);
        assert_eq!(vertices_at(slab, 4.0, 1.0, z), 1);
    }
    // Volume conservation.
    let (vp, vs, v0) = (
        mass_properties(pieces, Tol::witness()).unwrap().volume,
        mass_properties(slab, Tol::witness()).unwrap().volume,
        mass_properties(&fx.body, Tol::witness()).unwrap().volume,
    );
    assert!((vp + vs - v0).abs() <= 1e-12 * v0, "{vp} + {vs} vs {v0}");
}

/// The orientation table, CLOSED symmetric by M3 PR 6a's D7 (the
/// pinch lane): all four cells of the 2×2 now SUCCEED with identical
/// physical decompositions —
///   MIRRORED(+n) succeed | MIRRORED(−n) succeed
///   NOTCHED(−n)  succeed | NOTCHED(+n)  succeed
/// — the below-side pinch cells (left column) acquire their
/// below-vertex copies through `split`'s mirror-identity lane, so op
/// success is orientation-INVARIANT and only the piece ASSIGNMENT
/// swaps with the normal (PR 2's equivariance, now for success too).
/// Equal-volume oracle across every cell.
#[test]
fn notched_fixture_orientation_table() {
    for (profile, pinched_above_under_plus) in [(NOTCHED, true), (MIRRORED, false)] {
        let fx = prism::<f64>(profile, 1.0);
        let v0 = mass_properties(&fx.body, Tol::witness()).unwrap().volume;
        for ny in [1.0, -1.0] {
            let r = split(&fx.body, &plane(1.0, ny), Tol::witness()).unwrap();
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
                mass_properties(a, Tol::witness()).unwrap().volume,
                mass_properties(b, Tol::witness()).unwrap().volume,
            );
            assert!((va + vb - v0).abs() <= 1e-12 * v0, "{va} + {vb} vs {v0}");
        }
    }
}
