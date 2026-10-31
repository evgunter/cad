//! **OPEN DEFECT, pinned so a change to it is seen** — not the intended
//! answer
//! (`work/flush/an-aimed-rays-start-past-a-sections-whole-ellipse-loses-its-wall-roots-and-reads-the-wrong-side.md`).
//! A unit cylinder split by a plane through a seam vertex, a hair off the
//! wall's tangent plane there, refuses one of its correctly oriented
//! sides at the at-rest gate with `NegativeVolume`: an aimed ray starts
//! far past the section's whole ellipse, loses its wall roots, and reads
//! the side inside out. When that row lands, these poses build, or
//! refuse with the ellipse's own rounding alone
//! (`split_seam_vanishing_tilt`), and this file goes.

#![allow(clippy::expect_used, clippy::unwrap_used)]

use geom_core::{Point2, Point3, Tol, Vec3};
use topo::splitting::{SplitError, SplitFinishError, split};
use topo::{AtRestBody, ValidationError};

/// The unit cylinder `0 ≤ z ≤ 1`, turned `phi` about its axis.
fn cylinder(phi: f64, tol: Tol) -> AtRestBody<f64> {
    let circle = vec![(Point2::new(1.0, 0.0), 1.0), (Point2::new(-1.0, 0.0), 1.0)];
    let body = AtRestBody::validate(sweep::test_support::prism(circle, 1.0, tol), tol).unwrap();
    let turn = geom_core::Affine3::rotation_about_axis(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
        phi,
    );
    let turned =
        topo::transform_rigid(&body, &geom_core::test_support::rigid(&turn, tol), tol).unwrap();
    AtRestBody::validate(turned, tol).unwrap()
}

/// What the split through the seam vertex at `phi`, tilted `psi` off the
/// wall's tangent plane, its origin `l` along the rim's tangent, reads:
/// `true` where a side refuses at the at-rest gate inside out.
fn inside_out(phi: f64, psi: f64, l: f64, sign: f64, tol: Tol) -> bool {
    let cyl = cylinder(phi, tol);
    let (c, s) = (phi.cos(), phi.sin());
    let v = Point3::new(c, s, 0.0);
    let n = Vec3::new(c * psi.cos(), s * psi.cos(), psi.sin()) * sign;
    let plane = topo::test_support::split_plane(v + Vec3::new(-s, c, 0.0) * l, n, tol);
    match split(&cyl, &plane, tol) {
        Err(SplitError::Finish(SplitFinishError::ResultNotAtRest { errors, .. })) => errors
            .iter()
            .any(|e| matches!(e, ValidationError::NegativeVolume { .. })),
        _ => false,
    }
}

/// A family of poses of the census below that read a side inside out at
/// the run's ε, on main and on the side reader's in-face aims alike (the
/// aim decides which poses meet the defect, both ways): at 1e-9 the seam
/// at the census's 13th azimuth `2·10⁻⁷` and `5·10⁻⁷` off tangency, at
/// both offsets and either normal, and a second reviewer's seam at
/// `φ 5.5` `10⁻⁵` off tangency, outside the census's tilts, at four
/// offsets and either normal; at 1e-12 the 8th, 9th and 15th
/// azimuths at the tilts that read so. At 1e-6 no pose of the census does,
/// and the row says so.
#[test]
fn a_seam_split_a_hair_off_tangency_reads_a_side_inside_out() {
    let tol = Tol::witness();
    let eps = tol.eps();
    let family: Vec<(f64, f64, f64, f64)> = if eps == 1e-9 {
        [2e-7, 5e-7]
            .into_iter()
            .flat_map(|psi| [0.0, 0.1].map(|l| (psi, l)))
            .flat_map(|(psi, l)| [1.0, -1.0].map(|sign| (azimuth(13), psi, l, sign)))
            .chain(
                [0.0, 0.02, 0.1, 0.5]
                    .into_iter()
                    .flat_map(|l| [1.0, -1.0].map(|sign| (5.5, 1e-5, l, sign))),
            )
            .collect()
    } else if eps == 1e-12 {
        [(8, 1e-7), (8, 2e-7), (9, 2e-7), (9, 5e-7), (15, 1e-7)]
            .into_iter()
            .map(|(i, psi)| (azimuth(i), psi, 0.0, 1.0))
            .collect()
    } else {
        #[allow(clippy::print_stdout)]
        {
            println!(
                "SKIPPED at eps {eps:e}: no pose of the census reads a side inside out here, \
                 so this row pins nothing at this ε"
            );
        }
        return;
    };
    for (phi, psi, l, sign) in family {
        assert!(
            inside_out(phi, psi, l, sign, tol),
            "φ {phi} ψ {psi:e} L {l} sign {sign}: the open defect moved (no side reads inside out)"
        );
    }
}

/// The census's `i`th seam azimuth.
fn azimuth(i: u32) -> f64 {
    -3.1 + 6.2 * f64::from(i) / 23.0
}

/// The census: 24 seam azimuths, four tilts, two offsets, both normals,
/// and how many read a side inside out. A measurement, not a gate.
#[test]
#[ignore = "measurement: the open defect's census; run with --run-ignored only --no-capture"]
fn the_seam_split_census_counts_its_inside_out_sides() {
    let tol = Tol::witness();
    let mut wrong = Vec::new();
    for i in 0..24 {
        let phi = azimuth(i);
        for psi in [1e-7_f64, 2e-7, 5e-7, 1e-6] {
            for l in [0.0, 0.1] {
                for sign in [1.0, -1.0] {
                    if inside_out(phi, psi, l, sign, tol) {
                        wrong.push((phi, psi, l, sign));
                    }
                }
            }
        }
    }
    #[allow(clippy::print_stdout)]
    {
        println!(
            "SEAM-CENSUS eps={:e}: {} of 384 inside out: {wrong:?}",
            tol.eps(),
            wrong.len()
        );
    }
}
