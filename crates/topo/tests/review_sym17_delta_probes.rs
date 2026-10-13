//! SYM-17 delta review probes (probe branch only, not for merge).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use geom_core::{Affine3, Band, Interval, Real, Mat3, RigidMotion, RigidMotionError, MotionPart, Tol, Vec3};

fn cube() -> topo::Body<Interval> {
    common::brick::<Interval>((-0.005, 0.005), (-0.005, 0.005), (-0.005, 0.005), Tol::witness())
}

/// R1's probe, re-run at the fix head with assertions.
#[test]
fn r1_interval_translation_finiteness() {
    let inf = Interval::from_f64(f64::INFINITY);
    let zero = Interval::from_f64(0.0);
    let minted = RigidMotion::translation(Vec3::new(inf, zero, zero));
    eprintln!("DELTA R1 Interval translation(+inf): {:?}", minted.as_ref().err());
    assert!(matches!(minted, Err(RigidMotionError::NonFinite(MotionPart::Translation))));
    let big = Interval::from_f64(f64::MAX);
    let far = RigidMotion::translation(Vec3::new(big, zero, zero)).unwrap();
    let product = far.compose(far);
    eprintln!("DELTA R1 Interval MAX∘MAX: {:?}", product.as_ref().err());
    assert!(matches!(product, Err(RigidMotionError::NonFinite(MotionPart::Product))));
}

/// The decided door at `Interval` admits an unbounded translation;
/// what does `transform_rigid` then do with it?
#[test]
fn delta_door_admitted_unbounded_translation_moves_a_body() {
    let one = Interval::from_f64(1.0);
    let zero = Interval::from_f64(0.0);
    let id = Mat3::from_cols(
        Vec3::new(one, zero, zero),
        Vec3::new(zero, one, zero),
        Vec3::new(zero, zero, one),
    );
    let t = Interval::from_bounds(f64::MAX, f64::INFINITY);
    let map = Affine3::from_parts(id, Vec3::new(t, zero, zero));
    let admitted = RigidMotion::check_rigid(&map, Band::linear(Tol::witness()).unwrap());
    eprintln!("DELTA door [MAX, inf]: {:?}", admitted.as_ref().map(|m| m.origin().x));
    let m = admitted.expect("the door admits it");
    let moved = topo::transform_rigid(&cube(), &m, Tol::witness());
    eprintln!("DELTA transform_rigid by a door-admitted [MAX, inf]: {:?}", moved.err());
}
