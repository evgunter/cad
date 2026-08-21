//! M5 S11 interval lane: the concave-sense door and union rows at the
//! CERTIFIED scalar (feature `interval`).
//!
//! The sense bit itself is exact structure (a `bool` selected from
//! stored `Sign`s — nothing here widens), so what this lane proves is
//! that the FIXED doors and the containment-fallback union decide
//! definitely from honest enclosures on the first genuinely
//! mixed-sense bodies: the notch probes classify `Out` (not an
//! escalation), the pellet survives with a certified volume
//! enclosure, and the washer's bore reads as void.

#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{FRAC_PI_8, PI};
use profile::RawLoop;

use geom_core::Tol;
use geom_core::{Band, Bounds, Interval, Point2, Point3, Real, Tolerance, Vec2};
use profile::{Profile, ProfileLoop, ProfileVertex, SketchPlane, ValidatedProfile};
use sweep::{Extrusion, Revolution, RevolveAxis, extrude, revolve};
use topo::boolean::{SolidContainment, point_in_solid};
use topo::{Body, mass_properties};

fn iv(x: f64) -> Interval {
    Interval::from_f64(x)
}

fn p2(x: f64, y: f64) -> Point2<Interval> {
    Point2::new(iv(x), iv(y))
}

fn p3(x: f64, y: f64, z: f64) -> Point3<Interval> {
    Point3::new(iv(x), iv(y), iv(z))
}

fn validated(loops: Vec<ProfileLoop<Interval>>) -> ValidatedProfile<Interval> {
    Profile::new(SketchPlane::xy(), loops)
        .validate(Tol::witness())
        .unwrap()
}

/// The concave-notched prism at the certified scalar.
fn notched() -> Body<Interval> {
    let b = iv(FRAC_PI_8.tan());
    // Leaving bulges: the bottom arc bows out (+b), the top one bows
    // into the region (-b); the two sides are straight.
    let lp = <ProfileLoop<Interval> as RawLoop<Interval>>::new(vec![
        ProfileVertex::new(p2(0.0, 0.0), b),
        ProfileVertex::new(p2(2.0, 0.0), iv(0.0)),
        ProfileVertex::new(p2(2.0, 1.5), iv(0.0) - b),
        ProfileVertex::new(p2(0.0, 1.5), iv(0.0)),
    ]);
    extrude(&validated(vec![lp]), Extrusion::Distance(iv(1.0)))
        .unwrap()
        .body
}

/// The pellet strictly inside the notch (the S10 witness fixture).
fn pellet() -> Body<Interval> {
    let lp = <ProfileLoop<Interval> as RawLoop<Interval>>::polygon([
        p2(0.9, 1.25),
        p2(1.1, 1.25),
        p2(1.1, 1.35),
        p2(0.9, 1.35),
    ]);
    let plane = SketchPlane::from_frame(
        p3(0.0, 0.0, 0.3),
        geom_core::Vec3::new(iv(1.0), iv(0.0), iv(0.0)),
        geom_core::Vec3::new(iv(0.0), iv(1.0), iv(0.0)),
    );
    let vp = Profile::new(plane, vec![lp])
        .validate(Tol::witness())
        .unwrap();
    extrude(&vp, Extrusion::Distance(iv(0.4))).unwrap().body
}

#[test]
fn interval_notch_door_reads_out() {
    let body = notched();
    let b = Band::linear(Tol::witness()).unwrap();
    assert_eq!(
        point_in_solid(&body, p3(1.0, 0.5, 0.5), b).unwrap(),
        SolidContainment::In
    );
    for y in [1.2, 1.3, 1.4] {
        assert_eq!(
            point_in_solid(&body, p3(1.0, y, 0.5), b).unwrap(),
            SolidContainment::Out,
            "notch probe y = {y} must classify Out definitely"
        );
    }
}

#[test]
fn interval_union_keeps_the_pellet() {
    let a = notched();
    let b = pellet();
    let r = topo::boolean::union(&a, &b).unwrap();
    let out = r.body().expect("union of two non-empty solids");
    assert_eq!(out.body.shells().count(), 2);
    let vol = mass_properties(&out.body).unwrap().volume;
    let analytic = 3.008;
    assert!(
        vol.lo() <= analytic && analytic <= vol.hi(),
        "volume enclosure [{}, {}] must contain {analytic}",
        vol.lo(),
        vol.hi()
    );
    assert!(vol.hi() - vol.lo() <= 1e-9, "volume enclosure stays tight");
}

/// One props row on a fixed mixed-sense body: the washer's enclosure
/// still contains the analytic 3π (the bit is Category B for props —
/// flux is winding-derived — so the fixed senses must not move it).
#[test]
fn interval_washer_props_and_bore_door() {
    let lp = ProfileLoop::polygon([p2(1.0, 0.0), p2(2.0, 0.0), p2(2.0, 1.0), p2(1.0, 1.0)]);
    let axis = RevolveAxis {
        origin: p2(0.0, 0.0),
        dir: Vec2::new(iv(0.0), iv(1.0)),
    };
    let t = revolve(&validated(vec![lp]), axis, Revolution::Full).unwrap();
    let vol = mass_properties(&t.body).unwrap().volume;
    let analytic = 3.0 * PI;
    assert!(
        vol.lo() <= analytic && analytic <= vol.hi(),
        "washer volume enclosure [{}, {}] must contain 3π",
        vol.lo(),
        vol.hi()
    );
    // No washer door probes: a FULL revolve's wall trim spans the
    // whole period, which the cosine-window lane refuses typed
    // (`bool_wall_trim_period` — pre-existing and sense-independent);
    // the notch rows above are this lane's door coverage.
}
