//! **A rigid turn keeps a section-ellipse sliver's volume.** The unit
//! cylinder cut by a plane `ψ` off the wall's tangent plane at azimuth
//! `φ` leaves, on the plane's far side, a sliver about `tan ψ` deep at
//! the top, bounded by an arc of an ellipse of semi-major axis
//! `1/sin ψ` whose centre stands that far from the face. A rigid motion
//! preserves volume, so every turn of the sliver validates and measures
//! what the sliver does.
//!
//! The arc's vector area is where that distance enters: summed about
//! the ellipse's centre, its chord's rounding at the centre's magnitude
//! reaches the face's flux times the same distance, and a turn that
//! moves the rounding off the centre's direction read these poses
//! inside out (`NegativeVolume`) or a wrong-signed volume with no pad.
//! The poses are the probe's
//! (`crates/sweep/examples/turned_sliver_probe.rs`) that did, at 1e-9
//! (`ψ` = 1e-5 and 3e-6) and at 1e-12 (`ψ` = 1e-3), and one at
//! `ψ` = 1e-4 whose enclosure missed at 1e-9 without flipping a sign.
//!
//! The certificate is held to its own pad: the turned sliver's
//! `volume ± volume_pad` must hold the true volume. Only the
//! comparison with the unturned sliver adds the ε-scale slack, since the
//! turn may move each stored point by its own rounding.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use test_utils::vacuity::Exposure;
use topo::AtRestBody;
use topo::splitting::split;

use crate::common::oracles::volume_slack;

/// The sliver's volume `∫₀¹ S(z·tan ψ) dz` by `ψ`, with `S(h) =
/// acos(1 − h) − (1 − h)·√(2h − h²)` the unit disc's segment of depth
/// `h`: integrated at 40 digits (mpmath) for the `f64` values of `ψ`.
const SLIVER_VOLUME: [(f64, f64); 4] = [
    (1e-3, 2.384_884_799_981_565_3e-5),
    (1e-4, 7.542_391_557_603_427e-7),
    (1e-5, 2.385_136_620_610_998e-8),
    (3e-6, 3.919_182_328_732_877e-9),
];

fn sliver_volume(psi: f64) -> f64 {
    SLIVER_VOLUME
        .iter()
        .find(|(p, _)| p.to_bits() == psi.to_bits())
        .map(|&(_, v)| v)
        .unwrap_or_else(|| panic!("no oracle for ψ = {psi:e}"))
}

/// `(ψ, φ, L, axis, angle)`: the cutting plane through
/// `(cos φ, sin φ, 0) + L·t`, `t` the rim's tangent there, and the turn
/// about `axis` through `(0.3, −0.2, 0.5)`.
const POSES: [(f64, f64, f64, [f64; 3], f64); 8] = [
    (1e-5, 0.0, 0.0, [1.0, 0.0, 0.0], 0.7),
    (1e-5, 0.4, 0.3, [1.0, 2.0, 3.0], 3.0),
    (1e-5, 2.2, 0.1, [1.0, 0.0, 0.0], 1.3),
    (3e-6, 0.4, 0.3, [1.0, 2.0, 3.0], -1.1),
    (3e-6, 2.2, 0.1, [1.0, 0.0, 0.0], -1.1),
    (1e-4, 0.0, 0.0, [1.0, 0.0, 0.0], 0.7),
    (1e-3, 0.0, 0.0, [-0.3, 0.7, 0.2], 2.2),
    (1e-3, 0.4, 0.3, [1.0, 0.0, 0.0], 2.2),
];

#[test]
fn a_turned_sliver_validates_and_measures_its_own_volume() {
    let tol = Tol::witness();
    let circle = vec![(Point2::new(1.0, 0.0), 1.0), (Point2::new(-1.0, 0.0), 1.0)];
    let cylinder = AtRestBody::validate(sweep::test_support::prism(circle, 1.0, tol), tol).unwrap();
    let mut seen = Exposure::new("turned slivers");
    for (psi, phi, l, axis, angle) in POSES {
        let pose = format!("ψ {psi:e}, φ {phi}, L {l}, about {axis:?} by {angle}");
        let (c, s) = (phi.cos(), phi.sin());
        let normal = Vec3::new(c * psi.cos(), s * psi.cos(), psi.sin());
        let at = Point3::new(c, s, 0.0) + Vec3::new(-s, c, 0.0) * l;
        let plane = topo::test_support::split_plane(at, normal, tol);
        // A refused split or an invalid side is not a wrong answer, and
        // the floor below counts the poses that reach the turn.
        let Ok(parts) = split(&cylinder, &plane, tol) else {
            seen.note("split refused");
            continue;
        };
        let Some(sliver) = parts.above.body() else {
            seen.note("no sliver side");
            continue;
        };
        // The split's side is finished: its gate kept the verdict.
        let sliver = sliver.clone();
        seen.note("turned");
        let [x, y, z] = axis;
        let axis = Vec3::new(x, y, z);
        let turn =
            Affine3::rotation_about_axis(Point3::new(0.3, -0.2, 0.5), axis / axis.norm(), angle);
        let turned = topo::test_support::finished(
            "turned",
            topo::transform_rigid(&sliver, &geom_core::test_support::rigid(&turn, tol), tol)
                .unwrap(),
            tol,
        );
        assert_eq!(
            topo::validate_geometric(&turned, tol),
            Ok(()),
            "{pose}: the turned sliver validates"
        );
        let truth = sliver_volume(psi);
        let before =
            topo::mass_properties(&sliver, tol, topo::test_support::pinned_resolution(tol))
                .unwrap();
        let after = topo::mass_properties(&turned, tol, topo::test_support::pinned_resolution(tol))
            .unwrap();
        assert!(
            (after.enclosure.volume - truth).abs() <= after.enclosure.volume_pad,
            "{pose}: the turned sliver's enclosure {:e} ± {:e} misses its volume {truth:e}",
            after.enclosure.volume,
            after.enclosure.volume_pad
        );
        let (slack_before, slack_after) = (
            volume_slack(&before.enclosure, truth),
            volume_slack(&after.enclosure, truth),
        );
        assert!(
            (after.enclosure.volume - before.enclosure.volume).abs() <= slack_after + slack_before,
            "{pose}: the turn moved the volume from {:e} to {:e}, past both pads ({:e}, {:e})",
            before.enclosure.volume,
            after.enclosure.volume,
            slack_before,
            slack_after
        );
    }
    seen.report();
    seen.require(
        "turned",
        1,
        "every ε builds a sliver to turn: ψ = 1e-3 is far outside each band",
    );
}
