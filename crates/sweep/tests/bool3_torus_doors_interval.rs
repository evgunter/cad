//! Issue 1011, the torus half: the torus containment doors at the
//! CERTIFIED scalar (feature `interval`).
//!
//! The point of the lane is that the arm's margins are honest
//! enclosures rather than `f64` luck, and the ray×torus quartic asks
//! more of that than any arm before it.
//!
//! **Where the enclosure could have been lost, and is not.** Every
//! zero-straddling square in the quartic's coefficients goes through
//! `powi` (the zero-straddling-square rule that killed the cylinder
//! arm's first interval pass): `e²` in `p`, `n²` in `s`, and the four
//! powers of `p`, `q̂` and `s` inside the discriminant. The
//! discriminant's own lever is `ext¹¹`, a `powi` of a positive length,
//! so its enclosure is a point-ish interval rather than a product of
//! eleven widening multiplications.
//!
//! **The cube root is the one construction with no precedent here**, so
//! it is worth saying why the certified scalar follows it. Ferrari's
//! resolvent needs a real cube root on the branch where the quartic has
//! exactly two real roots — the common pierce — and the scalar trait has
//! none. The arm builds it from `sqrt` alone, through
//! `1/3 = Σ 4^{-k}`: a FIXED composition of 54 square roots, each
//! monotone and each an enclosure, so the interval instantiation
//! contains the true root by composition of containments. No iteration,
//! no convergence argument, nothing that could return a tight interval
//! that does not contain the answer — which is exactly what a Newton
//! step seeded from a float would have done here.
//!
//! Probes are dyadic where the geometry allows, so the enclosures are
//! points and every margin decides definitely.

#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Tol;
use geom_core::{Band, Interval, Point2, Point3, Real};
use profile::RawLoop;
use profile::{Profile, ProfileLoop, ProfileVertex, SketchPlane};
use sweep::{Revolution, RevolveAxis, revolve};
use topo::boolean::{SolidContainment, point_in_solid};

fn iv(x: f64) -> Interval {
    Interval::from_f64(x)
}

fn p3(x: f64, y: f64, z: f64) -> Point3<Interval> {
    Point3::new(iv(x), iv(y), iv(z))
}

/// The donut at the certified scalar: a circle of radius 0.25 about
/// `(1, 0)` revolved fully about the y axis — `R = 1`, `r = 0.25`, both
/// dyadic, so the torus's own data are exact intervals and every margin
/// below is the arm's arithmetic rather than the fixture's.
fn donut() -> topo::Body<Interval> {
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(Point2::new(iv(1.0), iv(-0.25)), iv(1.0)),
        ProfileVertex::new(Point2::new(iv(1.0), iv(0.25)), iv(1.0)),
    ]);
    let vp = Profile::new(SketchPlane::<Interval>::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap();
    let axis = RevolveAxis {
        origin: Point2::new(iv(0.0), iv(0.0)),
        dir: geom_core::Vec2::new(iv(0.0), iv(1.0)),
    };
    revolve(&vp, axis, Revolution::Full, Tol::witness())
        .unwrap()
        .body
}

#[test]
fn interval_torus_doors_classify_in_out_and_the_hole() {
    let body = donut();
    let b = Band::linear(Tol::witness()).unwrap();
    // Interior: the tube's spine circle, on the chart seam and a quarter
    // turn off it, and a dyadic point inside the tube off the midplane.
    for q in [p3(1.0, 0.0, 0.0), p3(0.0, 0.0, 1.0), p3(1.125, 0.125, 0.0)] {
        assert_eq!(
            point_in_solid(&body, q, b, Tol::witness()).unwrap(),
            SolidContainment::In,
            "interior at {q:?}"
        );
    }
    // The HOLE — the four-root ray, whose two nearer roots are what
    // makes this `Out`. A quartic whose enclosure had lost the near
    // pair would answer `In` here.
    assert_eq!(
        point_in_solid(&body, p3(0.0, 0.0, 0.0), b, Tol::witness()).unwrap(),
        SolidContainment::Out
    );
    // Exterior: past the outer equator, above the tube, and far away.
    for q in [p3(1.5, 0.0, 0.0), p3(1.0, 0.5, 0.0), p3(4.0, 2.5, 1.5)] {
        assert_eq!(
            point_in_solid(&body, q, b, Tol::witness()).unwrap(),
            SolidContainment::Out,
            "exterior at {q:?}"
        );
    }
    // The boundary: the outer and inner equators, both dyadic.
    for q in [p3(1.25, 0.0, 0.0), p3(0.75, 0.0, 0.0)] {
        assert_eq!(
            point_in_solid(&body, q, b, Tol::witness()).unwrap(),
            SolidContainment::OnBoundary,
            "boundary at {q:?}"
        );
    }
}
