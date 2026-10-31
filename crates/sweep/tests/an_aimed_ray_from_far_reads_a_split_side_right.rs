//! **An aimed ray started far from a wall reads a split's side right.**
//! A unit cylinder split by a plane through a seam vertex, a hair off
//! the wall's tangent plane there, has a section ellipse of semi-major
//! axis `1/sin ψ`. The side reader's aimed rays start beyond every face
//! of the side: a reach that counted the ellipse's whole carrier put
//! the start `~1/sin ψ` out, where the ray × wall quadratic, read about
//! that start, lost the wall's roots, and a correctly oriented side was
//! read inside out (`NegativeVolume`). Every pose here builds both sides,
//! or refuses one with the ellipse's own rounding alone
//! (`split_seam_vanishing_tilt`), and no side reads inside out.

#![allow(clippy::expect_used, clippy::unwrap_used, clippy::panic)]

use std::f64::consts::PI;

use geom_core::{Point2, Point3, Tol, Vec3};
use topo::splitting::{SplitError, SplitFinishError, SplitPart, split};
use topo::{AtRestBody, ValidationError, mass_properties, validate_geometric};

use crate::split_seam_vanishing_tilt::carrier_resolution;

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

/// How a split of the census reads: both sides built and holding the
/// cylinder's `π`, a side refused at its at-rest gate on the ellipse's
/// own rounding, a side whose side the reader could not read and whose
/// volume does not sign it, the split refused before it had sides, or a
/// side refused at rest inside out.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Read {
    Built,
    CarrierRefused,
    Unread,
    Refused,
    InsideOut,
}

impl Read {
    /// One letter per reading, for the census's line.
    const fn letter(self) -> char {
        match self {
            Self::Built => 'B',
            Self::CarrierRefused => 'C',
            Self::Unread => 'U',
            Self::Refused => 'R',
            Self::InsideOut => 'X',
        }
    }
}

/// What the split through the seam vertex at `phi`, tilted `psi` off the
/// wall's tangent plane, its origin `l` along the rim's tangent, reads
/// ([`Read`]). A side returned that tier 3 refuses, sides that miss `π`,
/// a side refused at rest on anything but the ellipse's rounding or
/// inside out, or a refusal the at-rest gate should have made, panics.
fn read(phi: f64, psi: f64, l: f64, sign: f64, tol: Tol) -> Read {
    let pose = format!("φ {phi} ψ {psi:e} L {l} sign {sign}");
    let cyl = cylinder(phi, tol);
    let (c, s) = (phi.cos(), phi.sin());
    let v = Point3::new(c, s, 0.0);
    let n = Vec3::new(c * psi.cos(), s * psi.cos(), psi.sin()) * sign;
    let plane = topo::test_support::split_plane(v + Vec3::new(-s, c, 0.0) * l, n, tol);
    match split(&cyl, &plane, tol) {
        Ok(r) => {
            let mut volume = 0.0;
            for (word, part) in [("above", &r.above), ("below", &r.below)] {
                let SplitPart::Body(b) = part else { continue };
                assert_eq!(
                    validate_geometric(b, tol),
                    Ok(()),
                    "{pose}: the {word} side"
                );
                volume += mass_properties(b, tol, topo::test_support::pinned_resolution(tol))
                    .unwrap_or_else(|e| panic!("{pose}: {word} measures: {e}"))
                    .met()
                    .expect("the pinned resolution is met")
                    .volume;
            }
            assert!(
                (volume - PI).abs() < 1e-6,
                "{pose}: the sides hold the cylinder's π, not {volume}"
            );
            Read::Built
        }
        Err(SplitError::Finish(SplitFinishError::ResultNotAtRest { errors, .. }))
            if errors
                .iter()
                .any(|e| matches!(e, ValidationError::NegativeVolume { .. })) =>
        {
            Read::InsideOut
        }
        Err(SplitError::Finish(SplitFinishError::ResultNotAtRest { errors, .. }))
            if errors
                .iter()
                .all(|e| matches!(e, ValidationError::VolumeSignUnresolved { .. })) =>
        {
            Read::Unread
        }
        Err(SplitError::Finish(SplitFinishError::ResultNotAtRest { errors, .. })) => {
            assert!(
                !errors.is_empty() && errors.iter().all(carrier_resolution),
                "{pose}: a side refuses on the ellipse's own rounding alone: {errors:?}"
            );
            Read::CarrierRefused
        }
        Err(
            e @ SplitError::Finish(
                SplitFinishError::ResultInvalid { .. } | SplitFinishError::ResultEscalated { .. },
            ),
        ) => panic!("{pose}: a side the at-rest gate refuses is `ResultNotAtRest`: {e:?}"),
        Err(_) => Read::Refused,
    }
}

/// The census's `i`th seam azimuth.
fn azimuth(i: u32) -> f64 {
    -3.1 + 6.2 * f64::from(i) / 23.0
}

/// The poses that read a side inside out while the reach counted the
/// ellipse's whole carrier, at every ε row: at 1e-9 the 13th azimuth
/// `2·10⁻⁷` and `5·10⁻⁷` off tangency, at both offsets and either
/// normal, and a seam at `φ 5.5` `10⁻⁵` off tangency at four offsets and
/// either normal; at 1e-12 the 8th, 9th and 15th azimuths.
#[test]
fn a_seam_split_a_hair_off_tangency_reads_no_side_inside_out() {
    let tol = Tol::witness();
    let family = [2e-7, 5e-7]
        .into_iter()
        .flat_map(|psi| [0.0, 0.1].map(|l| (psi, l)))
        .flat_map(|(psi, l)| [1.0, -1.0].map(|sign| (azimuth(13), psi, l, sign)))
        .chain(
            [0.0, 0.02, 0.1, 0.5]
                .into_iter()
                .flat_map(|l| [1.0, -1.0].map(|sign| (5.5, 1e-5, l, sign))),
        )
        .chain(
            [(8, 1e-7), (8, 2e-7), (9, 2e-7), (9, 5e-7), (15, 1e-7)]
                .into_iter()
                .map(|(i, psi)| (azimuth(i), psi, 0.0, 1.0)),
        );
    let reads: Vec<Read> = family
        .map(|(phi, psi, l, sign)| read(phi, psi, l, sign, tol))
        .collect();
    #[allow(clippy::print_stdout)]
    {
        println!("SEAM-FAMILY eps={:e}: {reads:?}", tol.eps());
    }
    assert!(
        !reads.contains(&Read::InsideOut),
        "no pose reads a side inside out: {reads:?}"
    );
}

/// The census: 24 seam azimuths, four tilts, two offsets, both normals.
/// No side reads inside out at any ε row.
#[test]
fn the_seam_split_census_reads_no_side_inside_out() {
    let tol = Tol::witness();
    let mut reads = [0; 5];
    let mut line = String::new();
    let mut inside_out = Vec::new();
    for i in 0..24 {
        for psi in [1e-7_f64, 2e-7, 5e-7, 1e-6] {
            for l in [0.0, 0.1] {
                for sign in [1.0, -1.0] {
                    let got = read(azimuth(i), psi, l, sign, tol);
                    reads[got as usize] += 1;
                    line.push(got.letter());
                    if got == Read::InsideOut {
                        inside_out.push((azimuth(i), psi, l, sign));
                    }
                }
            }
        }
    }
    #[allow(clippy::print_stdout)]
    {
        println!(
            "SEAM-CENSUS eps={:e}: of 384, {} build, {} refuse a side on the ellipse's \
             rounding, {} refuse a side unread, {} refuse the split, {} read a side inside \
             out: {inside_out:?}\nSEAM-LINE eps={:e}: {line}",
            tol.eps(),
            reads[0],
            reads[1],
            reads[2],
            reads[3],
            reads[4],
            tol.eps()
        );
    }
    assert!(inside_out.is_empty(), "no side reads inside out");
}

/// **A rod 1 km long and 1 mm across reads its side.** Its aimed rays
/// start about 2 km out, where a wall quadratic read about the start
/// loses `u·(2 km)²` against the radius's `10⁻⁶ m²` and moves a root by
/// about `10⁻⁷` m, past the band (`solid_contain`'s
/// `a_ray_from_far_keeps_a_walls_roots` holds the roots themselves). The
/// rod finishes at every ε row, as does each side of a plane cut across
/// it a hair off square.
#[test]
fn a_kilometre_rod_a_millimetre_across_reads_its_side() {
    let tol = Tol::witness();
    let r = 1e-3;
    let circle = vec![(Point2::new(r, 0.0), 1.0), (Point2::new(-r, 0.0), 1.0)];
    let rod = AtRestBody::validate(sweep::test_support::prism(circle, 1000.0, tol), tol)
        .expect("the rod finishes");
    let n = Vec3::new(1e-4, 0.0, 1.0);
    let plane = topo::test_support::split_plane(Point3::new(0.0, 0.0, 999.0), n / n.norm(), tol);
    let parts = split(&rod, &plane, tol).expect("the cut builds both sides");
    let mut volume = 0.0;
    for part in [&parts.above, &parts.below] {
        let SplitPart::Body(b) = part else {
            panic!("the cut leaves a body on each side")
        };
        volume += mass_properties(b, tol, topo::test_support::pinned_resolution(tol))
            .unwrap()
            .met()
            .expect("the pinned resolution is met")
            .volume;
    }
    let truth = PI * r * r * 1000.0;
    assert!(
        (volume - truth).abs() < 1e-6 * truth,
        "the sides hold the rod's {truth:e}, not {volume:e}"
    );
}
