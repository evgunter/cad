//! Issue 1011, the cone half: the cone containment doors at the
//! CERTIFIED scalar (feature `interval`).
//!
//! The point of the lane is that the arm's margins are honest
//! enclosures rather than `f64` luck. Three of them straddle zero on a
//! probe near the wall and reach `decide` through a SQUARE — the
//! quadratic's `A = (d·â)² − cos²α`, its `B²`, and the `(w·â)²` inside
//! `C` — so every one of them goes through `powi(2)` (the
//! zero-straddling-square rule). Plain multiplication would hand
//! `decide` a spurious negative lower bound and the lane would refuse
//! geometry it should accept, which is how the cylinder arm's first
//! interval pass died.
//!
//! The cone adds two enclosures the sphere lane has no analogue for:
//! the slant coordinate `v = (p − apex)·â / cos α`, a quotient by a
//! cosine that is bounded away from zero on the whole conventional
//! half-angle domain; and the apex, where `|p − apex|` is a norm whose
//! enclosure decides `Zero` and takes the graze rather than
//! normalizing a vanishing radial.
//!
//! Probes are dyadic where the geometry allows, so the enclosures are
//! points and every margin decides definitely.

#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Tol;
use geom_core::{Band, Interval, Point2, Point3, Real};
use profile::RawLoop;
use profile::{Profile, ProfileLoop, SketchPlane};
use sweep::{Revolution, RevolveAxis, revolve};
use topo::boolean::{SolidContainment, point_in_solid};

fn iv(x: f64) -> Interval {
    Interval::from_f64(x)
}

fn p3(x: f64, y: f64, z: f64) -> Point3<Interval> {
    Point3::new(iv(x), iv(y), iv(z))
}

/// The `revolve_cone` fixture at the certified scalar: the right
/// triangle (0,0), (1,0), (0,1) about the y-axis — base disc of radius
/// 1 at y = 0, apex at (0, 1, 0), half-angle π/4.
fn cone() -> topo::Body<Interval> {
    let lp = ProfileLoop::polygon([
        Point2::new(iv(0.0), iv(0.0)),
        Point2::new(iv(1.0), iv(0.0)),
        Point2::new(iv(0.0), iv(1.0)),
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
fn interval_cone_doors_classify_in_out_and_the_mirror_nappe() {
    let body = cone();
    let b = Band::linear(Tol::witness()).unwrap();
    // Interior: on the axis, and off it at a dyadic point well inside
    // the wall (the cone's radius at y = 0.25 is 0.75).
    assert_eq!(
        point_in_solid(&body, p3(0.0, 0.5, 0.0), b, Tol::witness()).unwrap(),
        SolidContainment::In
    );
    assert_eq!(
        point_in_solid(&body, p3(0.25, 0.25, 0.125), b, Tol::witness()).unwrap(),
        SolidContainment::In
    );
    // Exterior: outside the wall, below the base, and far away.
    assert_eq!(
        point_in_solid(&body, p3(0.75, 0.75, 0.25), b, Tol::witness()).unwrap(),
        SolidContainment::Out
    );
    assert_eq!(
        point_in_solid(&body, p3(0.25, -0.5, 0.125), b, Tol::witness()).unwrap(),
        SolidContainment::Out
    );
    assert_eq!(
        point_in_solid(&body, p3(4.0, 2.5, 1.5), b, Tol::witness()).unwrap(),
        SolidContainment::Out
    );
    // The MIRROR nappe, which the infinite double cone contains and the
    // solid does not — the enclosure of `(p − apex)·â` is definitely
    // signed here, so the nappe test decides rather than escalating.
    assert_eq!(
        point_in_solid(&body, p3(0.0, 1.5, 0.0), b, Tol::witness()).unwrap(),
        SolidContainment::Out
    );
    assert_eq!(
        point_in_solid(&body, p3(0.5, 1.5, 0.0), b, Tol::witness()).unwrap(),
        SolidContainment::Out
    );
}
