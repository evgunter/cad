//! M3 PR 5 acceptance (consumer finding, binding addition): operands
//! built through the REAL profile → `extrude` path — whose edges carry
//! `Intersection { s1, s2 }` descriptions referencing adjacent faces'
//! surface keys — must run through the public boolean ops without any
//! caller-side description normalization (the M3 exit criterion:
//! booleans on prismatic solids end-to-end through public ops only).
//! Exercises the carve orphan-sweep description fix and the combine
//! door's description re-certification remap.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Point2, Point3, Tolerance, Vec3};
use profile::RawLoop;
use profile::{Profile, ProfileLoop, SketchPlane};
use sweep::{Extrusion, extrude};
use topo::{
    Body, BooleanResult, BooleanResultKind, mass_properties, subtract, union, validate,
    validate_closed,
};
use geom_core::Tol;

fn slab(x0: f64, y0: f64, side: f64, z0: f64, height: f64) -> Body<f64> {
    let lp = ProfileLoop::polygon([
        Point2::new(x0, y0),
        Point2::new(x0 + side, y0),
        Point2::new(x0 + side, y0 + side),
        Point2::new(x0, y0 + side),
    ]);
    let plane = SketchPlane::new(Affine3::from_parts(
        geom_core::Mat3::from_cols(Vec3::unit_x(), Vec3::unit_y(), Vec3::unit_z()),
        Point3::new(0.0, 0.0, z0) - Point3::origin(),
    ));
    let validated = Profile::new(plane, vec![lp])
        .validate(Tol::witness())
        .unwrap();
    extrude(&validated, Extrusion::Distance(height), Tol::witness())
        .unwrap()
        .body
}

/// The two-brick trace on extrude-built operands: all three ops, tier
/// gates, exact volumes — no description downgrade anywhere.
#[test]
fn extruded_two_bricks_all_ops() {
    let a = slab(0.0, 0.0, 2.0, 0.0, 2.0); // [0,2]³
    let b = slab(1.0, 1.0, 2.0, 1.0, 2.0); // [1,3]³
    for (op, volume) in [
        (topo::intersect as fn(&Body<f64>, &Body<f64>, Tol) -> _, 1.0_f64),
        (union, 15.0),
        (subtract, 7.0),
    ] {
        let r: Result<BooleanResult<f64>, topo::BooleanError> = op(&a, &b, Tol::witness());
        let r = r.unwrap();
        let body = r.body().expect("non-empty");
        assert_eq!(body.kind, BooleanResultKind::Seamed);
        assert_eq!(validate(&body.body), Ok(()), "tier 1");
        assert_eq!(validate_closed(&body.body), Ok(()), "tier 2");
        let m = mass_properties(&body.body, Tol::witness()).unwrap();
        assert_eq!(m.volume, volume, "exact volume");
    }
}

/// The cookie-cutter lane on extrude-built operands (pocket subtract).
#[test]
fn extruded_pocket_subtract() {
    let a = slab(0.0, 0.0, 2.0, 0.0, 2.0);
    let b = slab(0.75, 0.75, 0.5, 1.5, 1.0); // pillar into the top face
    let r = subtract(&a, &b, Tol::witness()).unwrap();
    let body = r.body().expect("non-empty");
    assert_eq!(body.kind, BooleanResultKind::Seamed);
    assert_eq!(validate_closed(&body.body), Ok(()), "tier 2");
    let m = mass_properties(&body.body, Tol::witness()).unwrap();
    assert_eq!(m.volume, 8.0 - 0.125, "exact volume");
}
