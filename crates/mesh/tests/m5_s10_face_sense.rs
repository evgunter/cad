//! M5 S10 acceptance row: the tessellator honours `Face::sense`.
//!
//! S10 gave `topo::Face` an explicit orientation bit — the face's
//! outward normal is `sense_sign · chart_normal`. Most of the mesh
//! crate is deliberately **sense-invariant by derivation**: the emitted
//! triangle order comes from the loop's stored winding, which the
//! interior-left rule already ties to the outward normal, and which
//! `revert` flips in the same step as the bit. Multiplying there would
//! double-count.
//!
//! One site is not like that. A pole-to-pole sphere band's meridian
//! walk needs to know **which azimuth half** the band occupies — a
//! direction in the surface's CHART frame, recovered from the loop's
//! 3-D vector area, which is stated in the OUTWARD frame. Those two
//! frames differ by exactly `sense_sign`, so `walk.rs` multiplies. This
//! row is what proves it: flip one band's bit and the branch selection
//! moves by π, meshing the complementary half of the sphere.
//!
//! **Tolerance shape**: structural, per the `PartialSphereFace`
//! precedent. `sense_sign` is a `±1` selected by a `bool`, never a
//! decided quantity, so there is no ε-relative margin to sweep — the
//! discrimination is an exact branch change, and the assertions are
//! against the analytic ball or against bitwise equality.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::*;
use geom_core::Tol;
use mesh::validate::signed_volume;

/// **Acceptance row (tessellation).** The unit ball is two rimless
/// pole-to-pole bands on one sphere surface — the exact face
/// population whose azimuth branch the boundary winding cannot supply
/// on its own. Flipping one band's `sense` must change the emitted
/// mesh; leaving both alone must change nothing at all.
///
/// The negative half is the bit-identity claim for the whole crate:
/// with every constructor minting `sense: true` the new multiply is by
/// `+1`, so the ball meshes to the identical triangle list it did
/// before S10 (compared bitwise, not to a tolerance).
#[test]
fn tessellation_follows_the_band_sense() {
    let body = ball();
    let delta = 0.05;

    // The clean body still meshes watertight and correctly oriented,
    // converging on the analytic ball — the bit-identity half, run
    // through the crate's own acceptance helper so it means exactly
    // what every other revolve row means.
    let (v, a) = (
        4.0 * core::f64::consts::PI / 3.0,
        4.0 * core::f64::consts::PI,
    );
    for d in [0.1, 0.01, 0.002] {
        check_mesh_acceptance(&body, d, Some((v, a)));
    }
    let honest = mesh::tessellate(&body, delta, Tol::witness()).unwrap();
    assert!(
        signed_volume(&honest) > 0.0,
        "the unflipped ball meshes outward"
    );

    // Flip ONE band's sense bit and nothing else.
    let (face, _) = body.faces().next().unwrap();
    let flipped = body.flipped_face_sense_for_tests(face).unwrap();
    let lied = mesh::tessellate(&flipped, delta, Tol::witness()).unwrap();

    // The band's meridian branch moved by π: the emitted geometry is
    // NOT the same mesh. (Bitwise: if `walk.rs` ignored the bit these
    // would be identical, since every other input is untouched.)
    let same = honest.positions.len() == lied.positions.len()
        && honest
            .positions
            .iter()
            .zip(&lied.positions)
            .all(|(a, b)| a.x.to_bits() == b.x.to_bits() && a.y.to_bits() == b.y.to_bits());
    assert!(
        !same,
        "flipping a rimless band's sense must move its azimuth branch — \
         the tessellator is still ignoring Face::sense"
    );
}
